use std::io::{BufReader, Read, Write};

fn main() -> std::io::Result<()> {
    // Constants (should eventually be commandline arguments or something)
    let input_dir = std::path::Path::new("/Users/christopher/git/compressed-loading/input_files/");
    let working_dir = std::path::Path::new("/Users/christopher/git/compressed-loading/working_files/");

    // Copy inputs to the working directory
    std::fs::copy(input_dir.join("random.dat"), working_dir.join("random.dat"))?;
    std::fs::copy(input_dir.join("wikipedia.bz2"), working_dir.join("wikipedia.bz2"))?;

    // Compress input files as needed
    zstd_compress_file(
        &working_dir.join("random.dat"),
        &working_dir.join("random_compressed.dat"),
        0,
    )?;

    // Read the compressed random data as a test
    // let random_compressed_file = std::fs::File::open(working_dir.join("random_compressed.dat"))?;
    // let mut random_decoder = zstd::Decoder::new(random_compressed_file)?;

    // let mut out = [0; 10];
    // random_decoder.read_exact(&mut out)?;
    // println!("Uncompressed: {:?}", out);

    // let mut random_file = std::fs::File::open(working_dir.join("random.dat"))?;
    // random_file.read_exact(&mut out)?;
    // println!("Uncompressed: {:?}", out);

    // Simple benchmarking play
    let start = std::time::Instant::now();
    let random_file = std::fs::File::open(working_dir.join("random.dat"))?;
    let random_file_bufreader = BufReader::new(random_file);
    let last_byte = random_file_bufreader.bytes().last().unwrap()?;
    println!("Uncompressed:\tElapsed: {:?}\tLast byte: {:?}", start.elapsed(), last_byte);

    let start = std::time::Instant::now();
    let random_compressed_file = std::fs::File::open(working_dir.join("random_compressed.dat"))?;
    let random_decoder = zstd::Decoder::new(random_compressed_file)?;
    let random_compressed_bufreader = BufReader::new(random_decoder);
    let last_byte = random_compressed_bufreader.bytes().last().unwrap()?;
    println!("Compressed:\tElapsed: {:?}\tLast byte: {:?}", start.elapsed(), last_byte);

    let start = std::time::Instant::now();
    let compressed_file = std::fs::File::open(working_dir.join("wikipedia.bz2"))?;
    let compressed_file_bufread = BufReader::new(compressed_file);
    let decoder = bzip2::bufread::BzDecoder::new(compressed_file_bufread);
    let last_byte = decoder.bytes().last().unwrap()?;
    println!("Wikipedia compressed:\tElapsed: {:?}\tLast byte: {:?}", start.elapsed(), last_byte);

    return Ok(());
}

/// Convenience function for compressing files with zstd.
fn zstd_compress_file(
    source_path: &std::path::Path,
    destination_path: &std::path::Path,
    level: i32,
) -> std::io::Result<()> {
    let source_file = std::fs::File::open(source_path)?;
    let mut destination_file = std::fs::File::create(destination_path)?;
    zstd::stream::copy_encode(source_file, &mut destination_file, level)?;
    destination_file.flush()?;

    return Ok(());
}
