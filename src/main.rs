use std::io::{BufReader, BufWriter, Read, Write};

macro_rules! log_result {
    ($fmt:literal, $($result:expr),*$(,)?) => {
        println!(concat!("{}:{}: ", $fmt), file!(), line!(), $($result,)*);
    };
}
const SMALL_FILE_SIZE: u64 = 1 << 33;

fn experiment<F, R>(
    name: &str,
    working_dir: &std::path::Path,
    file: &str,
    reader_wrapper: F,
) -> std::io::Result<()>
where
    F: FnOnce(std::fs::File) -> R,
    R: Read,
{
    purge_filesystem_caches();
    let start = std::time::Instant::now();
    let file = std::fs::File::open(working_dir.join(file))?;
    let reader = reader_wrapper(file);
    let checksum = reader_checksum(reader);
    let duration = start.elapsed();
    log_result!(
        "{}:\tElapsed: {:?}\tChecksum: {:?}",
        name,
        duration,
        checksum
    );
    return Ok(());
}

fn main() -> std::io::Result<()> {
    // Constants (should eventually be commandline arguments or something)
    let input_dir = std::path::Path::new("input_files/");
    let working_dir = std::path::Path::new("working_files/");

    // Populate the working directory as needed
    setup_files(input_dir, working_dir)?;

    experiment(
        "Constant uncompressed with bufreader",
        working_dir,
        "constant",
        BufReader::new,
    )?;

    experiment(
        "Constant compressed (zstd)",
        working_dir,
        "constant.zst",
        |file| zstd::Decoder::new(file).expect("Failed to create zstd decoder"),
    )?;

    experiment(
        "Constant compressed (xz)",
        working_dir,
        "constant.xz",
        xz2::read::XzDecoder::new,
    )?;

    experiment(
        "Constant compressed (xz high)",
        working_dir,
        "constant_high.xz",
        xz2::read::XzDecoder::new,
    )?;

    experiment(
        "Wikipedia small uncompressed",
        working_dir,
        "wikipedia_small",
        BufReader::new,
    )?;

    experiment(
        "Wikipedia small compressed (zstd)",
        working_dir,
        "wikipedia_small.zst",
        |file| zstd::Decoder::new(file).expect("Failed to create zstd decoder"),
    )?;

    experiment(
        "Wikipedia small compressed (xz)",
        working_dir,
        "wikipedia_small.xz",
        xz2::read::XzDecoder::new,
    )?;

    experiment(
        "Wikipedia small compressed (xz high)",
        working_dir,
        "wikipedia_small_high.xz",
        xz2::read::XzDecoder::new,
    )?;

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
