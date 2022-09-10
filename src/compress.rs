// Copyright (c) 2018-2022, Firas Khalil Khana
// Distributed under the terms of the ISC License

use std::error::Error;
use std::fs::File;
use std::io::BufWriter;

use tar::{Archive, Builder};
use zstd::stream::{read::Decoder, write::Encoder};

pub fn radula_behave_compress(name: &str, path: &str) -> Result<(), Box<dyn Error>> {
    let mut encoder = Encoder::new(
        BufWriter::new(File::create(&format!("{}.tar.zst", name))?),
        22,
    )?;
    encoder.multithread((num_cpus::get() as f32 * 1.5) as u32)?;
    encoder.long_distance_matching(true)?;
    encoder.window_log(31)?;

    let mut archive = Builder::new(encoder);
    archive.follow_symlinks(false);
    archive.append_dir_all(".", path)?;

    let encoder = archive.into_inner()?;
    encoder.finish()?;

    Ok(())
}

pub fn radula_behave_decompress(name: &str, path: &str) -> Result<(), Box<dyn Error>> {
    let mut decoder = Decoder::new(File::open(&format!("{}.tar.zst", name))?)?;
    decoder.window_log_max(31)?;

    Archive::new(decoder).unpack(path)?;

    Ok(())
}
