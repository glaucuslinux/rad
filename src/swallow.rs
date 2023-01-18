// Copyright (c) 2018-2023, Firas Khalil Khana
// Distributed under the terms of the ISC License

use std::env;
use std::error::Error;
use std::ffi::OsStr;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use super::ceras;
use super::constants;

use bzip2::read::BzDecoder;
use colored::Colorize;
use flate2::read::GzDecoder;
use futures_util::{stream, StreamExt};
use git2::Repository;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use reqwest::{Client, Url};
use tar::Archive;
use tokio::{fs, fs::File, io::AsyncReadExt, io::AsyncWriteExt, task};
use xz2::read::XzDecoder;
use zstd::stream::read::Decoder;

// Asynchronously clone source repos
pub async fn radula_behave_clone(
    commit: String,
    url: Url,
    path: PathBuf,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Clone ceras's source `git` repository and checkout the freshly cloned `git`
    // repository at the specified commit number
    let repo = Repository::clone(url.as_str(), path)?;
    repo.checkout_tree(&repo.revparse_ext(&commit)?.0, None)?;

    Ok(())
}

// Asynchronously download source tarballs
pub async fn radula_behave_download(
    url: Url,
    path: PathBuf,
    m: Arc<MultiProgress>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let client = Client::new();

    let mut res = client.get(url.as_str()).send().await?;
    let size = res.content_length().unwrap_or(0);

    let file = String::from(
        url.path_segments()
            .and_then(|segments| segments.last())
            .unwrap_or_default(),
    );

    let pb = m.add(ProgressBar::new(size));
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                " {wide_msg} {bytes:>} {bytes_per_sec:>12} {elapsed_precise} [{bar:40}] {percent:>3}%",
            )?
            .progress_chars("##-"),
    );
    pb.set_message(file.clone());

    let mut out = File::create(path.join(file)).await?;

    // Use `write_all` instead of `write`:
    // https://rust-lang.github.io/rust-clippy/master/index.html#unused_io_amount
    while let Some(chunk) = res.chunk().await? {
        pb.inc(chunk.len() as u64);
        out.write_all(&chunk).await?;
    }

    pb.finish();

    // Must flush manually
    out.flush().await?;

    Ok(())
}

// Extract source tarballs
pub async fn radula_behave_extract(file: PathBuf, path: PathBuf) -> Result<(), Box<dyn Error>> {
    let decoder: Box<dyn Read> = match file.extension().and_then(OsStr::to_str).unwrap_or_default()
    {
        "bz2" => Box::new(BzDecoder::new(std::fs::File::open(file)?)),
        "gz" | "tgz" => Box::new(GzDecoder::new(std::fs::File::open(file)?)),
        "xz" => Box::new(XzDecoder::new_multi_decoder(std::fs::File::open(file)?)),
        "zst" => Box::new(Decoder::new(std::fs::File::open(file)?)?),
        _ => unreachable!(),
    };
    Archive::new(decoder).unpack(path)?;

    Ok(())
}

// Fetch, verify and extract ceras's source
pub async fn radula_behave_swallow(name: &'static str) -> Result<(), Box<dyn Error>> {
    // Receive the variables from the `ceras` file
    let ceras = ceras::radula_behave_ceras_parse(name).await?;

    let version = ceras.ver.unwrap_or_default();
    let commit = ceras.cmt.unwrap_or_default();
    let url = Url::parse(&ceras.url.unwrap())?;
    let checksum = ceras.sum.unwrap_or_default();

    println!("{} swallow", "::".bold());

    // Get the path of the source directory
    let path = Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_SOURCES)?).join(name);

    let file = path.join(
        url.path_segments()
            .and_then(|segments| segments.last())
            .unwrap_or_default(),
    );

    if !Path::is_dir(Path::new(&path)) {
        if version == constants::RADULA_TOOTH_GIT {
            task::spawn(async move { radula_behave_clone(commit, url, path).await.unwrap() })
                .await?;
        } else {
            fs::create_dir_all(&path).await?;

            // Currently we're only pushing one url at a time...
            let mut urls = Vec::new();
            urls.push(url.as_str());

            let m = Arc::new(MultiProgress::new());

            // Total progress bar
            let pb = Arc::new(m.add(ProgressBar::new(urls.len() as u64)));
            pb.set_style(ProgressStyle::default_bar().template("{msg} ({pos}/{len})")?);
            pb.set_message(" Total");
            pb.tick();

            stream::iter(urls)
                .enumerate()
                .for_each_concurrent(Some(8), |(_, url)| {
                    let path = path.clone();
                    let m = m.clone();
                    let pb = pb.clone();

                    async move {
                        task::spawn(radula_behave_download(Url::parse(url).unwrap(), path, m))
                            .await
                            .unwrap()
                            .unwrap();
                        pb.inc(1);
                    }
                })
                .await;

            pb.finish();

            radula_behave_verify(name, file.clone(), checksum.to_string()).await?;

            // Extract ceras's source tarball
            radula_behave_extract(file, path).await?;
        }
    } else if version != constants::RADULA_TOOTH_GIT {
        // Only verify existing ceras's source tarballs without extracting them again
        radula_behave_verify(name, file.clone(), checksum.to_string()).await?;
    }

    Ok(())
}

// Verify `XXH3_128bits` checksum of source tarballs
pub async fn radula_behave_verify(
    name: &'static str,
    file: PathBuf,
    checksum: String,
) -> Result<bool, Box<dyn Error>> {
    let mut file = File::open(ceras::radula_behave_ceras_source(&name).await?.join(file)).await?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer).await?;

    let hash = format!("{:x}", xxhash_rust::xxh3::xxh3_128(&buffer));

    let verify = checksum == hash;

    Ok(verify)
}
