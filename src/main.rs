use std::io::{BufReader, Read, Write};

fn main() -> std::io::Result<()> {
    // Constants (should eventually be commandline arguments or something)
    let input_dir = std::path::Path::new("/Users/christopher/git/compressed-loading/input_files/");
    let working_dir = std::path::Path::new("/Users/christopher/git/compressed-loading/working_files/");

    // Copy inputs to the working directory
    // std::fs::copy(input_dir.join("random.dat"), working_dir.join("random.dat"))?;
    // std::fs::copy(input_dir.join("wikipedia.bz2"), working_dir.join("wikipedia.bz2"))?;

    // Compress input files as needed
    if !working_dir.join("random_compressed.dat").try_exists()? {
        zstd_compress_file(
            &input_dir.join("random.dat"),
            &working_dir.join("random_compressed.dat"),
            0,
        )?;
    }

    if !working_dir.join("wikipedia").try_exists()? {
        let source_file = std::fs::File::open(&input_dir.join("wikipedia.bz2"))?;
        let source_bufread = BufReader::new(source_file);
        let mut decoder = bzip2::bufread::BzDecoder::new(source_bufread);
        let mut destination_file = std::fs::File::create(&working_dir.join("wikipedia"))?;
        std::io::copy(&mut decoder, &mut destination_file)?;
        destination_file.flush()?;
    }

    if !working_dir.join("wikipedia.zst").try_exists()? {
        let source_file = std::fs::File::open(&working_dir.join("wikipedia"))?;
        let mut destination_file = std::fs::File::create(&working_dir.join("wikipedia.zst"))?;
        zstd::stream::copy_encode(source_file, &mut destination_file, 0)?;
        destination_file.flush()?;
    }

    if !working_dir.join("wikipedia.xz").try_exists()? {
        let source_file = std::fs::File::open(&working_dir.join("wikipedia"))?;
        let source_bufread = BufReader::new(source_file);
        let mut destination_file: std::fs::File = std::fs::File::create(&working_dir.join("wikipedia.xz"))?;
        let mut encoder = xz2::bufread::XzEncoder::new(source_bufread, 6);
        std::io::copy(&mut encoder, &mut destination_file)?;
        destination_file.flush()?;
    }


    if !working_dir.join("wikipedia_small").try_exists()? {
        let source_file = std::fs::File::open(&working_dir.join("wikipedia"))?;
        let mut destination_file = std::fs::File::create(&working_dir.join("wikipedia_small"))?;
        std::io::copy(&mut source_file.take(1_000_000_000), &mut destination_file)?;
        destination_file.flush()?;
    }

    if !working_dir.join("wikipedia_small.zst").try_exists()? {
        let source_file = std::fs::File::open(&working_dir.join("wikipedia_small"))?;
        let mut destination_file = std::fs::File::create(&working_dir.join("wikipedia_small.zst"))?;
        zstd::stream::copy_encode(source_file, &mut destination_file, 0)?;
        destination_file.flush()?;
    }

    if !working_dir.join("wikipedia_small.xz").try_exists()? {
        let source_file = std::fs::File::open(&working_dir.join("wikipedia_small"))?;
        let source_bufread = BufReader::new(source_file);
        let mut destination_file: std::fs::File = std::fs::File::create(&working_dir.join("wikipedia_small.xz"))?;
        let mut encoder = xz2::bufread::XzEncoder::new(source_bufread, 6);
        std::io::copy(&mut encoder, &mut destination_file)?;
        destination_file.flush()?;
    }

    if !working_dir.join("wikipedia_small_high.xz").try_exists()? {
        let source_file = std::fs::File::open(&working_dir.join("wikipedia_small"))?;
        let source_bufread = BufReader::new(source_file);
        let mut destination_file: std::fs::File = std::fs::File::create(&working_dir.join("wikipedia_small_high.xz"))?;
        let mut encoder = xz2::bufread::XzEncoder::new(source_bufread, 9);
        std::io::copy(&mut encoder, &mut destination_file)?;
        destination_file.flush()?;
    }

    // if !working_dir.join("wikipedia_small.xz").try_exists()? {
    //     let mut source_file = std::fs::File::open(&working_dir.join("wikipedia_small"))?;
    //     // let source_bufread = BufReader::new(source_file);
    //     let destination_file: std::fs::File = std::fs::File::create(&working_dir.join("wikipedia_small.xz"))?;
    //     let mut encoder = xz2::write::XzEncoder::new(destination_file, 6);
    //     std::io::copy(&mut source_file, &mut encoder)?;
    //     encoder.flush()?;
    // }


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
    // let start = std::time::Instant::now();
    // let random_file = std::fs::File::open(input_dir.join("random.dat"))?;
    // let random_file_bufreader = BufReader::new(random_file);
    // let last_byte = random_file_bufreader.bytes().last().unwrap()?;
    // println!("Uncompressed:\tElapsed: {:?}\tLast byte: {:?}", start.elapsed(), last_byte);

    // let start = std::time::Instant::now();
    // let random_compressed_file = std::fs::File::open(working_dir.join("random_compressed.dat"))?;
    // let random_decoder = zstd::Decoder::new(random_compressed_file)?;
    // let random_compressed_bufreader = BufReader::new(random_decoder);
    // let last_byte = random_compressed_bufreader.bytes().last().unwrap()?;
    // println!("Compressed:\tElapsed: {:?}\tLast byte: {:?}", start.elapsed(), last_byte);

    let start = std::time::Instant::now();
    let file = std::fs::File::open(working_dir.join("wikipedia_small"))?;
    let file_bufread = BufReader::new(file);
    let checksum = reader_checksum(file_bufread);
    println!("Wikipedia uncompressed:\tElapsed: {:?}\tChecksum: {:?}", start.elapsed(), checksum);

    let start = std::time::Instant::now();
    let compressed_file = std::fs::File::open(working_dir.join("wikipedia_small.xz"))?;
    let decoder = xz2::read::XzDecoder::new(compressed_file);
    let checksum = reader_checksum(decoder);
    println!("Wikipedia compressed (xz):\tElapsed: {:?}\tChecksum: {:?}", start.elapsed(), checksum);

    let start = std::time::Instant::now();
    let compressed_file = std::fs::File::open(working_dir.join("wikipedia_small_high.xz"))?;
    let decoder = xz2::read::XzDecoder::new(compressed_file);
    let checksum = reader_checksum(decoder);
    println!("Wikipedia compressed (xz high):\tElapsed: {:?}\tChecksum: {:?}", start.elapsed(), checksum);

    let start = std::time::Instant::now();
    let compressed_file = std::fs::File::open(working_dir.join("wikipedia_small.zst"))?;
    let compressed_file_bufread = BufReader::new(compressed_file);
    let decoder = zstd::Decoder::new(compressed_file_bufread)?;
    let checksum = reader_checksum(decoder);
    println!("Wikipedia compressed (zstd):\tElapsed: {:?}\tChecksum: {:?}", start.elapsed(), checksum);

    let start = std::time::Instant::now();
    let compressed_file = std::fs::File::open(input_dir.join("wikipedia_small.bz2"))?;
    let compressed_file_bufread = BufReader::new(compressed_file);
    let decoder = bzip2::bufread::BzDecoder::new(compressed_file_bufread);
    let checksum = reader_checksum(decoder);
    println!("Wikipedia compressed (bzip2):\tElapsed: {:?}\tChecksum: {:?}", start.elapsed(), checksum);

    return Ok(());
}

fn reader_checksum<R : Read>(reader : R) -> u64 {
    let mut out : u64 = 0;
    for b in reader.bytes() {
        let v = b.unwrap() as u64;
        out = (out + v) % 10000000000;
    }

    return out;
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
