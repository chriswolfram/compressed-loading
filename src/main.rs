use std::io::{BufReader, Read, Write};

fn main() -> std::io::Result<()> {
    // Constants (should eventually be commandline arguments or something)
    let input_dir = std::path::Path::new("/Users/christopher/git/compressed-loading/input_files/");
    let working_dir = std::path::Path::new("/Users/christopher/git/compressed-loading/working_files/");

    setup_files(input_dir, working_dir)?;

    let start = std::time::Instant::now();
    let file = std::fs::File::open(working_dir.join("wikipedia_small"))?;
    let file_bufread = BufReader::new(file);
    let checksum = reader_checksum(file_bufread);
    println!(
        "Wikipedia uncompressed:\tElapsed: {:?}\tChecksum: {:?}",
        start.elapsed(),
        checksum
    );

    let start = std::time::Instant::now();
    let compressed_file = std::fs::File::open(working_dir.join("wikipedia_small.xz"))?;
    let decoder = xz2::read::XzDecoder::new(compressed_file);
    let checksum = reader_checksum(decoder);
    println!(
        "Wikipedia compressed (xz):\tElapsed: {:?}\tChecksum: {:?}",
        start.elapsed(),
        checksum
    );

    let start = std::time::Instant::now();
    let compressed_file = std::fs::File::open(working_dir.join("wikipedia_small_high.xz"))?;
    let decoder = xz2::read::XzDecoder::new(compressed_file);
    let checksum = reader_checksum(decoder);
    println!(
        "Wikipedia compressed (xz high):\tElapsed: {:?}\tChecksum: {:?}",
        start.elapsed(),
        checksum
    );

    let start = std::time::Instant::now();
    let compressed_file = std::fs::File::open(working_dir.join("wikipedia_small.zst"))?;
    let compressed_file_bufread = BufReader::new(compressed_file);
    let decoder = zstd::Decoder::new(compressed_file_bufread)?;
    let checksum = reader_checksum(decoder);
    println!(
        "Wikipedia compressed (zstd):\tElapsed: {:?}\tChecksum: {:?}",
        start.elapsed(),
        checksum
    );

    let start = std::time::Instant::now();
    let compressed_file = std::fs::File::open(input_dir.join("wikipedia_small.bz2"))?;
    let compressed_file_bufread = BufReader::new(compressed_file);
    let decoder = bzip2::bufread::BzDecoder::new(compressed_file_bufread);
    let checksum = reader_checksum(decoder);
    println!(
        "Wikipedia compressed (bzip2):\tElapsed: {:?}\tChecksum: {:?}",
        start.elapsed(),
        checksum
    );

    return Ok(());
}

fn setup_files(input_dir: &std::path::Path, working_dir: &std::path::Path) -> std::io::Result<()> {
    setup_files_random(input_dir, working_dir)?;
    setup_files_wikipedia(input_dir, working_dir)?;

    return Ok(());
}

fn setup_files_random(input_dir: &std::path::Path, working_dir: &std::path::Path) -> std::io::Result<()> {
    zstd_compress_file_if_needed(
        &input_dir.join("random.dat"),
        &working_dir.join("random_compressed.dat"),
        0,
    )?;

    return Ok(());
}

fn setup_files_wikipedia(input_dir: &std::path::Path, working_dir: &std::path::Path) -> std::io::Result<()> {
    if !working_dir.join("wikipedia").try_exists()? {
        let source_file = std::fs::File::open(&input_dir.join("wikipedia.bz2"))?;
        let source_bufread = BufReader::new(source_file);
        let mut decoder = bzip2::bufread::BzDecoder::new(source_bufread);
        let mut destination_file = std::fs::File::create(&working_dir.join("wikipedia"))?;
        std::io::copy(&mut decoder, &mut destination_file)?;
        destination_file.flush()?;
    }

    if !working_dir.join("wikipedia_small").try_exists()? {
        let source_file = std::fs::File::open(&working_dir.join("wikipedia"))?;
        let mut destination_file = std::fs::File::create(&working_dir.join("wikipedia_small"))?;
        std::io::copy(&mut source_file.take(1_000_000_000), &mut destination_file)?;
        destination_file.flush()?;
    }

    zstd_compress_file_if_needed(
        &working_dir.join("wikipedia"),
        &working_dir.join("wikipedia.zst"),
        0,
    )?;

    zstd_compress_file_if_needed(
        &working_dir.join("wikipedia_small"),
        &working_dir.join("wikipedia_small.zst"),
        0,
    )?;

    xz_compress_file_if_needed(
        &working_dir.join("wikipedia"),
        &working_dir.join("wikipedia.xz"),
        6,
    )?;

    xz_compress_file_if_needed(
        &working_dir.join("wikipedia_small"),
        &working_dir.join("wikipedia_small.xz"),
        6,
    )?;

    xz_compress_file_if_needed(
        &working_dir.join("wikipedia_small"),
        &working_dir.join("wikipedia_small_high.xz"),
        9,
    )?;

    return Ok(());
}

// Compression utilities

/// Convenience function for compressing files with zstd.
fn zstd_compress_file_if_needed(
    source_path: &std::path::Path,
    destination_path: &std::path::Path,
    level: i32,
) -> std::io::Result<()> {
    if destination_path.try_exists()? {
        println!("File already exists:\t{:?}", destination_path);
        return Ok(());
    }

    println!("Compressing file with zstd to:\t{:?}", destination_path);
    let source_file = std::fs::File::open(source_path)?;
    let mut destination_file = std::fs::File::create(destination_path)?;
    zstd::stream::copy_encode(source_file, &mut destination_file, level)?;
    destination_file.flush()?;

    return Ok(());
}

/// Convenience function for compressing files with xz.
fn xz_compress_file_if_needed(
    source_path: &std::path::Path,
    destination_path: &std::path::Path,
    level: u32,
) -> std::io::Result<()> {
    if destination_path.try_exists()? {
        println!("File already exists:\t{:?}", destination_path);
        return Ok(());
    }

    println!("Compressing file with XZ to:\t{:?}", destination_path);
    let source_file = std::fs::File::open(source_path)?;
    let source_bufread = BufReader::new(source_file);
    let mut destination_file: std::fs::File = std::fs::File::create(destination_path)?;
    let mut encoder = xz2::bufread::XzEncoder::new(source_bufread, level);
    std::io::copy(&mut encoder, &mut destination_file)?;
    destination_file.flush()?;

    return Ok(());
}


// Experiment utilities

/// Generate a super cheap hash of a reader.
fn reader_checksum<R: Read>(reader: R) -> u64 {
    let mut out: u64 = 0;
    for b in reader.bytes() {
        let v = b.unwrap() as u64;
        out = (out + v) % 10000000000;
    }

    return out;
}