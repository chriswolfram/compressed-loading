use std::io::{BufReader, BufWriter, Read, Write};

macro_rules! log_result {
    ($fmt:literal, $($result:expr),*$(,)?) => {
        println!(concat!("{}:{}: ", $fmt), file!(), line!(), $($result,)*);
    };
}
const SMALL_FILE_SIZE: u64 = 1 << 33;

fn main() -> std::io::Result<()> {
    // Constants (should eventually be commandline arguments or something)
    let input_dir = std::path::Path::new("input_files/");
    let working_dir = std::path::Path::new("working_files/");

    // Populate the working directory as needed
    setup_files(input_dir, working_dir)?;

    // Purge experiments
    purge_filesystem_caches();
    let start = std::time::Instant::now();
    let mut file = std::fs::File::open(working_dir.join("constant"))?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    let checksum = iterator_checksum(buf.into_iter());
    let duration = start.elapsed();
    log_result!(
        "With purge:\tElapsed: {:?}\tChecksum: {:?}",
        duration,
        checksum
    );

    let start = std::time::Instant::now();
    let mut file = std::fs::File::open(working_dir.join("constant"))?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    let checksum = iterator_checksum(buf.into_iter());
    let duration = start.elapsed();
    log_result!(
        "Without purge:\tElapsed: {:?}\tChecksum: {:?}",
        duration,
        checksum
    );

    let start = std::time::Instant::now();
    let mut file = std::fs::File::open(working_dir.join("constant"))?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    let checksum = iterator_checksum(buf.into_iter());
    let duration = start.elapsed();
    log_result!(
        "Without purge:\tElapsed: {:?}\tChecksum: {:?}",
        duration,
        checksum
    );

    purge_filesystem_caches();
    let start = std::time::Instant::now();
    let mut file = std::fs::File::open(working_dir.join("constant"))?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    let checksum = iterator_checksum(buf.into_iter());
    let duration = start.elapsed();
    log_result!(
        "With purge:\tElapsed: {:?}\tChecksum: {:?}",
        duration,
        checksum
    );

    // Benchmarking experiments

    println!("Reccomended output buffer for zstd: {:?}", zstd::Decoder::<BufReader<std::fs::File>>::recommended_output_size());

    purge_filesystem_caches();
    let start = std::time::Instant::now();
    let mut file = std::fs::File::open(working_dir.join("constant"))?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    let checksum = reader_checksum(std::io::Cursor::new(buf));
    let duration = start.elapsed();
    log_result!(
        "Constant uncompressed with read_to_end and reader_checksum:\tElapsed: {:?}\tChecksum: {:?}",
        duration,
        checksum
    );

    purge_filesystem_caches();
    let start = std::time::Instant::now();
    let mut file = std::fs::File::open(working_dir.join("constant"))?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    let checksum = iterator_checksum(buf.into_iter());
    let duration = start.elapsed();
    log_result!(
        "Constant uncompressed with read_to_end and iterator_checksum:\tElapsed: {:?}\tChecksum: {:?}",
        duration,
        checksum
    );

    // let mut file = std::fs::File::open(working_dir.join("constant"))?;
    // let mut buf = Vec::new();
    // file.read_to_end(&mut buf)?;
    // purge_filesystem_caches();
    // let start = std::time::Instant::now();
    // let checksum = iterator_checksum(buf.into_iter());
    // let duration = start.elapsed();
    // log_result!(
    //     "Constant iterator checksum alone:\tElapsed: {:?}\tChecksum: {:?}",
    //     duration,
    //     checksum
    // );

    purge_filesystem_caches();
    let start = std::time::Instant::now();
    let file = std::fs::File::open(working_dir.join("constant"))?;
    let file_bufread = BufReader::new(file);
    let checksum = reader_checksum(file_bufread);
    let duration = start.elapsed();
    log_result!(
        "Constant uncompressed with bufreader:\tElapsed: {:?}\tChecksum: {:?}",
        duration,
        checksum
    );

    purge_filesystem_caches();
    let start = std::time::Instant::now();
    let compressed_file = std::fs::File::open(working_dir.join("constant.zst"))?;
    let mut decoder = zstd::Decoder::new(compressed_file)?;
    let mut buf = Vec::new();
    decoder.read_to_end(&mut buf)?;
    let checksum = iterator_checksum(buf.into_iter());
    let duration = start.elapsed();
    log_result!(
        "Constant compressed with read_to_end (zstd):\tElapsed: {:?}\tChecksum: {:?}",
        duration,
        checksum
    );

    purge_filesystem_caches();
    let start = std::time::Instant::now();
    let compressed_file = std::fs::File::open(working_dir.join("constant.zst"))?;
    let decoder = zstd::Decoder::new(compressed_file)?;
    let decoder_bufreader = BufReader::with_capacity(1 << 30, decoder);
    let checksum = reader_checksum(decoder_bufreader);
    let duration = start.elapsed();
    log_result!(
        "Constant compressed with bufreaders on decoder (zstd):\tElapsed: {:?}\tChecksum: {:?}",
        duration,
        checksum
    );

    // purge_filesystem_caches();
    // let start = std::time::Instant::now();
    // let compressed_file = std::fs::File::open(working_dir.join("constant.zst"))?;
    // let compressed_file_bufread = BufReader::with_capacity(1 << 30, compressed_file);
    // let decoder = zstd::Decoder::new(compressed_file_bufread)?;
    // let checksum = reader_checksum(decoder);
    // let duration = start.elapsed();
    // log_result!(
    //     "Constant compressed with big bufreader (zstd):\tElapsed: {:?}\tChecksum: {:?}",
    //     duration,
    //     checksum
    // );

    purge_filesystem_caches();
    let start = std::time::Instant::now();
    let compressed_file = std::fs::File::open(working_dir.join("constant.zst"))?;
    let compressed_file_bufread = BufReader::new(compressed_file);
    let decoder = zstd::Decoder::new(compressed_file_bufread)?;
    let checksum = reader_checksum(decoder);
    let duration = start.elapsed();
    log_result!(
        "Constant compressed with bufreader (zstd):\tElapsed: {:?}\tChecksum: {:?}",
        duration,
        checksum
    );

    purge_filesystem_caches();
    let start = std::time::Instant::now();
    let compressed_file = std::fs::File::open(working_dir.join("constant.zst"))?;
    let decoder = zstd::Decoder::new(compressed_file)?;
    let checksum = reader_checksum(decoder);
    let duration = start.elapsed();
    log_result!(
        "Constant compressed (zstd):\tElapsed: {:?}\tChecksum: {:?}",
        duration,
        checksum
    );

    purge_filesystem_caches();
    let start = std::time::Instant::now();
    let compressed_file = std::fs::File::open(working_dir.join("constant.xz"))?;
    let mut decoder = xz2::read::XzDecoder::new(compressed_file);
    let mut buf = Vec::new();
    decoder.read_to_end(&mut buf)?;
    let checksum = iterator_checksum(buf.into_iter());
    log_result!(
        "Constant compressed buffer (xz):\tElapsed: {:?}\tChecksum: {:?}",
        start.elapsed(),
        checksum
    );

    purge_filesystem_caches();
    let start = std::time::Instant::now();
    let mut compressed_file = std::fs::File::open(working_dir.join("constant.xz"))?;
    let mut compressed_buf = Vec::new();
    compressed_file.read_to_end(&mut compressed_buf)?;
    let mut decoder = xz2::read::XzDecoder::new(std::io::Cursor::new(compressed_buf));
    let mut buf = Vec::new();
    decoder.read_to_end(&mut buf)?;
    let checksum = iterator_checksum(buf.into_iter());
    log_result!(
        "Constant compressed buffer 2 (xz):\tElapsed: {:?}\tChecksum: {:?}",
        start.elapsed(),
        checksum
    );

    purge_filesystem_caches();
    let start = std::time::Instant::now();
    let compressed_file = std::fs::File::open(working_dir.join("constant.xz"))?;
    let decoder = xz2::read::XzDecoder::new(compressed_file);
    let checksum = reader_checksum(decoder);
    log_result!(
        "Constant compressed (xz):\tElapsed: {:?}\tChecksum: {:?}",
        start.elapsed(),
        checksum
    );

    purge_filesystem_caches();
    let start = std::time::Instant::now();
    let compressed_file = std::fs::File::open(working_dir.join("constant_high.xz"))?;
    let decoder = xz2::read::XzDecoder::new(compressed_file);
    let checksum = reader_checksum(decoder);
    log_result!(
        "Constant compressed (xz high):\tElapsed: {:?}\tChecksum: {:?}",
        start.elapsed(),
        checksum
    );

    purge_filesystem_caches();
    let start = std::time::Instant::now();
    let file = std::fs::File::open(working_dir.join("wikipedia_small"))?;
    let file_bufread = BufReader::new(file);
    let checksum = reader_checksum(file_bufread);
    log_result!(
        "Wikipedia uncompressed:\tElapsed: {:?}\tChecksum: {:?}",
        start.elapsed(),
        checksum
    );

    purge_filesystem_caches();
    let start = std::time::Instant::now();
    let compressed_file = std::fs::File::open(working_dir.join("wikipedia_small.xz"))?;
    let decoder = xz2::read::XzDecoder::new(compressed_file);
    let checksum = reader_checksum(decoder);
    log_result!(
        "Wikipedia compressed (xz):\tElapsed: {:?}\tChecksum: {:?}",
        start.elapsed(),
        checksum
    );

    purge_filesystem_caches();
    let start = std::time::Instant::now();
    let compressed_file = std::fs::File::open(working_dir.join("wikipedia_small_high.xz"))?;
    let decoder = xz2::read::XzDecoder::new(compressed_file);
    let checksum = reader_checksum(decoder);
    log_result!(
        "Wikipedia compressed (xz high):\tElapsed: {:?}\tChecksum: {:?}",
        start.elapsed(),
        checksum
    );

    purge_filesystem_caches();
    let start = std::time::Instant::now();
    let compressed_file = std::fs::File::open(working_dir.join("wikipedia_small.zst"))?;
    let compressed_file_bufread = BufReader::new(compressed_file);
    let decoder = zstd::Decoder::new(compressed_file_bufread)?;
    let checksum = reader_checksum(decoder);
    log_result!(
        "Wikipedia compressed (zstd):\tElapsed: {:?}\tChecksum: {:?}",
        start.elapsed(),
        checksum
    );

    // purge_caches();
    // let start = std::time::Instant::now();
    // let compressed_file = std::fs::File::open(input_dir.join("wikipedia_small.bz2"))?;
    // let compressed_file_bufread = BufReader::new(compressed_file);
    // let decoder = bzip2::bufread::BzDecoder::new(compressed_file_bufread);
    // let checksum = reader_checksum(decoder);
    // log_result!(
    //     "Wikipedia compressed (bzip2):\tElapsed: {:?}\tChecksum: {:?}",
    //     start.elapsed(),
    //     checksum
    // );

    return Ok(());
}

fn setup_files(input_dir: &std::path::Path, working_dir: &std::path::Path) -> std::io::Result<()> {
    setup_files_constant(input_dir, working_dir)?;
    setup_files_random(input_dir, working_dir)?;
    setup_files_wikipedia(input_dir, working_dir)?;

    return Ok(());
}

fn setup_files_constant(
    _input_dir: &std::path::Path,
    working_dir: &std::path::Path,
) -> std::io::Result<()> {
    if !working_dir.join("constant").try_exists()? {
        let bytes = "A".as_bytes();
        let destination_file = std::fs::File::create(&working_dir.join("constant"))?;
        let mut destination_file = BufWriter::new(destination_file);
        for _ in 1..(SMALL_FILE_SIZE) {
            destination_file.write_all(bytes)?;
        }
        destination_file.flush()?;
    }

    zstd_compress_file_if_needed(
        &working_dir.join("constant"),
        &working_dir.join("constant.zst"),
        0,
    )?;

    xz_compress_file_if_needed(
        &working_dir.join("constant"),
        &working_dir.join("constant.xz"),
        6,
    )?;

    xz_compress_file_if_needed(
        &working_dir.join("constant"),
        &working_dir.join("constant_high.xz"),
        9,
    )?;

    return Ok(());
}

fn setup_files_random(
    input_dir: &std::path::Path,
    working_dir: &std::path::Path,
) -> std::io::Result<()> {
    zstd_compress_file_if_needed(
        &input_dir.join("random.dat"),
        &working_dir.join("random_compressed.dat"),
        0,
    )?;

    return Ok(());
}

fn setup_files_wikipedia(
    input_dir: &std::path::Path,
    working_dir: &std::path::Path,
) -> std::io::Result<()> {
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
        std::io::copy(
            &mut source_file.take(SMALL_FILE_SIZE),
            &mut destination_file,
        )?;
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

fn reader_checksum<R: Read>(mut reader: R) -> u64 {
    let mut buf = [0u8; 1_000_000];
    while let Ok(n) = reader.read(&mut buf) {
        if n == 0 {
            break;
        }
    }
    return buf.last().unwrap().clone() as u64;
}

fn iterator_checksum<I: Iterator<Item = u8>>(iter: I) -> u64 {
    return iter.last().unwrap() as u64;
}

fn purge_filesystem_caches() {
    if cfg!(target_os = "linux") {
        std::process::Command::new("sh")
            .arg("-c")
            .arg("sync && echo 3 > /proc/sys/vm/drop_caches")
            .output()
            .expect("Failed to purge");
    } else if cfg!(target_os = "macos") {
        std::process::Command::new("sh")
            .arg("-c")
            .arg("sync && sudo purge")
            .output()
            .expect("Failed to purge");
    }
}
