use std::{
    io::{BufReader, BufWriter, Read, Write},
    usize,
};

const SMALL_FILE_SIZE: usize = 1 << 33;

fn random_range_size(size_index: usize) -> usize {
    size_index + 1
}

fn experiment(
    name: &str,
    working_dir: &std::path::Path,
    test_case: &str,
    algorithm: &str,
    level: i32,
) -> std::io::Result<()> {
    purge_filesystem_caches();
    let start = std::time::Instant::now();
    let file_name = format!("{}.{}.{}", test_case, algorithm, level);
    let file = std::fs::File::open(working_dir.join(file_name))?;
    let checksum = match algorithm {
        "none" => reader_checksum(BufReader::new(file)),
        "zstd" => reader_checksum(zstd::Decoder::new(file)?),
        "xz" => reader_checksum(xz2::read::XzDecoder::new(file)),
        _ => panic!("Unknown algorithm: {}", algorithm),
    };
    let duration = start.elapsed();
    println!(
        "{}, {}, {}, {}, {}, {}",
        name,
        test_case,
        algorithm,
        level,
        duration.as_secs_f64(),
        checksum
    );
    return Ok(());
}

fn experiment_name(test_case: &str, algorithm: &str, level: i32) -> String {
    match algorithm {
        "none" => format!("{test_case} (uncompressed)"),
        _ => format!("{test_case} ({algorithm} {level})"),
    }
}

fn experiment_test_case(
    working_dir: &std::path::Path,
    test_case: &str,
    variants: &[(&str, i32)],
) -> std::io::Result<()> {
    for &(algorithm, level) in variants {
        experiment(
            &experiment_name(test_case, algorithm, level),
            working_dir,
            test_case,
            algorithm,
            level,
        )?;
    }
    return Ok(());
}

fn main() -> std::io::Result<()> {
    // Constants (should eventually be commandline arguments or something)
    let input_dir = std::path::Path::new("input_files/");
    let working_dir = std::path::Path::new("working_files/");

    // Populate the working directory as needed
    setup_files(input_dir, working_dir)?;

    println!("name, test_case, algorithm,level, duration, checksum");

    experiment_test_case(working_dir, "constant", &[("none", 0), ("zstd", 0)])?;

    experiment_test_case(
        working_dir,
        "wikipedia_small",
        &[("none", 0), ("zstd", 0), ("xz", 0)],
    )?;

    experiment_test_case(working_dir, "logs", &[("none", 0), ("zstd", 0)])?;

    for size_index in 0..20 {
        experiment_test_case(
            &working_dir.join("random_range"),
            &random_range_size(size_index).to_string(),
            &[("none", 0), ("zstd", 0)],
        )?;
    }

    return Ok(());
}

fn setup_files(input_dir: &std::path::Path, working_dir: &std::path::Path) -> std::io::Result<()> {
    setup_files_constant(input_dir, working_dir)?;
    setup_files_wikipedia(input_dir, working_dir)?;
    setup_files_logs(input_dir, working_dir)?;
    setup_files_random_range(input_dir, working_dir, 1_000_000_000)?;

    return Ok(());
}

fn setup_files_constant(
    _input_dir: &std::path::Path,
    working_dir: &std::path::Path,
) -> std::io::Result<()> {
    if !working_dir.join("constant.none.0").try_exists()? {
        let bytes = "A".as_bytes();
        let destination_file = std::fs::File::create(&working_dir.join("constant.none.0"))?;
        let mut destination_file = BufWriter::new(destination_file);
        for _ in 1..(SMALL_FILE_SIZE) {
            destination_file.write_all(bytes)?;
        }
        destination_file.flush()?;
    }

    zstd_compress_file_if_needed(working_dir, "constant", 0)?;

    xz_compress_file_if_needed(working_dir, "constant", 6)?;

    return Ok(());
}

fn setup_files_random_range(
    input_dir: &std::path::Path,
    working_dir: &std::path::Path,
    size: usize,
) -> std::io::Result<()> {
    let out_dir = &working_dir.join("random_range");
    if !out_dir.is_dir() {
        std::fs::DirBuilder::new().create(out_dir)?;
    }

    // let mut rng = rand::thread_rng();

    // for size in (1..255) {
    //     let out_file = std::fs::File::create(out_dir.join(size.to_string()))?;
    //     let mut out_file = BufWriter::new(out_file);
    //     for b in (0..SMALL_FILE_SIZE).map(|_| rng.gen::<u8>()) {
    //         out_file.write(&[b % size])?;
    //     }
    // }

    let mut random_input = std::fs::File::open(&input_dir.join("random.dat"))?;
    let mut random_bytes = Vec::new();
    random_input.read_to_end(&mut random_bytes)?;

    for size_index in 0..20 {
        let block_size = random_range_size(size_index);
        let path = out_dir.join(format!("{}.none.0", block_size));
        if path.exists() {
            continue;
        }

        let mut out_file = std::fs::File::create(&path)?;
        let mut out_data: Vec<u8> = std::iter::repeat(b'a').take(size).collect();
        for i in 0..(size / block_size) {
            out_data[i * block_size] = random_bytes[i];
        }
        out_file.write_all(&out_data)?;

        zstd_compress_file_if_needed(&out_dir, &block_size.to_string(), 0)?;
    }

    return Ok(());
}

fn setup_files_wikipedia(
    input_dir: &std::path::Path,
    working_dir: &std::path::Path,
) -> std::io::Result<()> {
    if !working_dir.join("wikipedia.none.0").try_exists()? {
        let source_file = std::fs::File::open(&input_dir.join("wikipedia.bz2"))?;
        let source_bufread = BufReader::new(source_file);
        let mut decoder = bzip2::bufread::BzDecoder::new(source_bufread);
        let mut destination_file = std::fs::File::create(&working_dir.join("wikipedia.none.0"))?;
        std::io::copy(&mut decoder, &mut destination_file)?;
        destination_file.flush()?;
    }

    if !working_dir.join("wikipedia_small.none.0").try_exists()? {
        let source_file = std::fs::File::open(&working_dir.join("wikipedia.none.0"))?;
        let mut destination_file =
            std::fs::File::create(&working_dir.join("wikipedia_small.none.0"))?;
        std::io::copy(
            &mut source_file.take(SMALL_FILE_SIZE as u64),
            &mut destination_file,
        )?;
        destination_file.flush()?;
    }

    zstd_compress_file_if_needed(working_dir, "wikipedia", 0)?;

    zstd_compress_file_if_needed(working_dir, "wikipedia_small", 0)?;

    xz_compress_file_if_needed(working_dir, "wikipedia", 6)?;

    xz_compress_file_if_needed(working_dir, "wikipedia_small", 6)?;
    xz_compress_file_if_needed(working_dir, "wikipedia_small", 0)?;

    return Ok(());
}

fn setup_files_logs(
    input_dir: &std::path::Path,
    working_dir: &std::path::Path,
) -> std::io::Result<()> {
    if !working_dir.join("logs.none.0").try_exists()? {
        let mut source_file = std::fs::File::open(&input_dir.join("access.log"))?;
        let mut destination_file = std::fs::File::create(&working_dir.join("logs.none.0"))?;
        std::io::copy(&mut source_file, &mut destination_file)?;
        destination_file.flush()?;
    }

    zstd_compress_file_if_needed(working_dir, "logs", 0)?;

    xz_compress_file_if_needed(working_dir, "logs", 6)?;

    return Ok(());
}

// Compression utilities

/// Convenience function for compressing files with zstd.
fn zstd_compress_file_if_needed(
    working_dir: &std::path::Path,
    test_case: &str,
    level: i32,
) -> std::io::Result<()> {
    let dest_file = format!("{}.zstd.{}", test_case, level);
    let destination_path = working_dir.join(dest_file);
    let source_path = working_dir.join(format!("{}.none.0", test_case));
    if destination_path.try_exists()? {
        eprintln!("File already exists:\t{:?}", destination_path);
        return Ok(());
    }

    eprintln!("Compressing file with zstd to:\t{:?}", destination_path);
    let source_file = std::fs::File::open(source_path).unwrap();
    let mut destination_file = std::fs::File::create(destination_path).unwrap();
    zstd::stream::copy_encode(source_file, &mut destination_file, level).unwrap();
    destination_file.flush().unwrap();

    return Ok(());
}

/// Convenience function for compressing files with xz.
fn xz_compress_file_if_needed(
    working_dir: &std::path::Path,
    test_case: &str,
    level: u32,
) -> std::io::Result<()> {
    let dest_file = format!("{}.xz.{}", test_case, level);
    let destination_path = working_dir.join(dest_file);
    let source_path = working_dir.join(format!("{}.none.0", test_case));
    if destination_path.try_exists()? {
        eprintln!("File already exists:\t{:?}", destination_path);
        return Ok(());
    }

    eprintln!("Compressing file with XZ to:\t{:?}", destination_path);
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
    let mut last = 0;
    while let Ok(n) = reader.read(&mut buf) {
        if n == 0 {
            break;
        }
        last = buf[n - 1];
    }
    return last as u64;
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
