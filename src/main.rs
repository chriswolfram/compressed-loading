// use std::io::Write;
// use rand::{Rng, RngCore, SeedableRng};

// fn main() -> std::io::Result<()> {
//     // Constants
//     let random_seed = 1234;
//     let data_size = 100_000_000;
//     let input_files_dir = std::path::Path::new("/Users/christopher/git/compressed-loading/input_files/");
//     let working_files_dir = std::path::Path::new("/Users/christopher/git/compressed-loading/working_files/");

//     // Generate random data
//     let mut rng = rand::rngs::StdRng::seed_from_u64(random_seed);
//     let mut rand_data = vec![0; data_size];
//     rng.fill_bytes(&mut rand_data);

//     std::fs::write(working_files_dir.join("rand_file.dat"), &rand_data)?;

//     let rand_file_compressed = std::fs::File::create(working_files_dir.join("rand_file_compressed.dat"))?;
//     let mut compressor = bzip2::write::BzEncoder::new(rand_file_compressed, bzip2::Compression::best());
//     compressor.write_all(&rand_data)?;
//     compressor.finish()?;

//     println!("{:?}", rand_data.len());

//     return Ok(());
// }

// fn export_compressed<R : std::io::Read>(reader : &R, dir : &std::path::Path) {
//     // Use std::io::Cursor to convert an iterator or data to a reader
// }


use std::io::Write;
use rand::{Rng, RngCore, SeedableRng};

fn main() -> std::io::Result<()> {
    // Constants
    let random_seed = 1234;
    let data_size = 100_000_000;
    let input_files_dir = std::path::Path::new("/Users/christopher/git/compressed-loading/input_files/");
    let working_files_dir = std::path::Path::new("/Users/christopher/git/compressed-loading/working_files/");

    // Generate random data
    let mut rng = rand::rngs::StdRng::seed_from_u64(random_seed);
    let rand_iter  = (0..data_size).map(|_| rng.gen::<u8>());
    let mut rand_reader = std::io::Cursor::new(rand_iter);

    let mut rand_file = std::fs::File::create(working_files_dir.join("rand_file.dat"))?;
    std::io::copy(&mut rand_reader, &mut rand_file);

    // let rand_file_compressed = std::fs::File::create(working_files_dir.join("rand_file_compressed.dat"))?;
    // let mut compressor = bzip2::write::BzEncoder::new(rand_file_compressed, bzip2::Compression::best());
    // compressor.write_all(&rand_data)?;
    // compressor.finish()?;

    // println!("{:?}", rand_data.len());

    return Ok(());
}

fn export_compressed<R : std::io::Read>(reader : &R, dir : &std::path::Path) {
    // Use std::io::Cursor to convert an iterator or data to a reader
}
