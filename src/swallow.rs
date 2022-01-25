// Copyright (c) 2018-2022, Firas Khalil Khana
// Distributed under the terms of the ISC License

use std::env;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use super::ceras;
use super::constants;

use colored::Colorize;
use futures_util::{stream, StreamExt};
use git2::Repository;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use reqwest::{Client, Url};
use tokio::{fs, fs::File, io::AsyncReadExt, io::AsyncWriteExt, task};

// Asynchronously clone source repos
pub async fn radula_behave_clone(
    commit: String,
    url: Url,
    path: PathBuf,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Clone ceras's source `git` repository and checkout the freshly cloned `git`
    // repository at the specified commit number
    let repo = Repository::clone(url.as_str(), path).unwrap();
    repo.checkout_tree(&repo.revparse_ext(&commit).unwrap().0, None)
        .unwrap();

    Ok(())
}

// Asynchronously download source tarballs
pub async fn radula_behave_download(
    url: Url,
    m: Arc<MultiProgress>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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
                " {wide_msg} {bytes:>} {bytes_per_sec:>12} {elapsed_precise} [{bar:40}]
                {percent:>3}%",
            )
            .progress_chars("##-"),
    );
    pb.set_message(file.clone());

    let mut out = File::create(file.clone()).await?;

    while let Some(chunk) = res.chunk().await? {
        pb.inc(chunk.len() as u64);
        out.write(&chunk).await?;
    }

    pb.finish();

    // Must flush manually
    out.flush().await?;

    Ok(())
}

// Fetch, verify and extract ceras's source
pub async fn radula_behave_swallow(name: &'static str) -> Result<(), Box<dyn std::error::Error>> {
    // Receive the variables from the `ceras` file
    let ceras = ceras::radula_behave_ceras_parse(name).await?;

    let version = ceras.ver.unwrap();
    let commit = ceras.cmt.unwrap();
    let url = Url::parse(&ceras.url.unwrap())?;
    let checksum = ceras.sum.unwrap();

    println!("{} swallow", "::".bold());

    // Get the path of the source directory
    let path = Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_SOURCES)?).join(name);

    let file = String::from(
        url.path_segments()
            .and_then(|segments| segments.last())
            .unwrap_or_default(),
    );

    if !Path::is_dir(Path::new(&path)) {
        if version == constants::RADULA_TOOTH_GIT {
            task::spawn(async move { radula_behave_clone(commit, url, path).await.unwrap() })
                .await?;
        } else {
            let mut urls = Vec::new();
            urls.push("https://musl.libc.org/releases/musl-1.2.2.tar.gz");
            urls.push("https://github.com/landley/toybox/archive/0.8.6.tar.gz");

            let m = Arc::new(MultiProgress::new());

            // Total progress bar
            let pb = Arc::new(m.add(ProgressBar::new(urls.len() as u64)));
            pb.set_style(ProgressStyle::default_bar().template("{msg} ({pos}/{len})"));
            pb.set_message(" Total");
            pb.tick();

            stream::iter(urls)
                .enumerate()
                .for_each_concurrent(Some(8), |(_, url)| {
                    let m = m.clone();
                    let pb = pb.clone();

                    async move {
                        task::spawn(radula_behave_download(Url::parse(url).unwrap(), m))
                            .await
                            .unwrap();
                        pb.inc(1);
                    }
                })
                .await;

            pb.finish();

            fs::create_dir(&path).await?;

            radula_behave_verify(name, file.to_string(), checksum.to_string()).await?;

            // Extract ceras's source tarball
            // Command::new(constants::RADULA_TOOTH_TAR)
            //     .args(&["xpvf", &file, "-C", &path])
            //     .stdout(Stdio::null())
            //     .spawn()?
            //     .wait()?;
        }
    } else if version != constants::RADULA_TOOTH_GIT {
        // Only verify existing ceras's source tarballs without extracting them again
        // radula_behave_verify();
    }

    Ok(())
}

// Verify `XXH3_128bits` checksum of source tarballs
pub async fn radula_behave_verify(
    name: &'static str,
    file: String,
    checksum: String,
) -> Result<bool, Box<dyn std::error::Error>> {
    let mut file =
        fs::File::open(ceras::radula_behave_ceras_source(&name).await?.join(file)).await?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer).await?;

    let hash = format!("{:x}", xxhash_rust::xxh3::xxh3_128(&buffer));

    let verify = checksum == hash;

    Ok(verify)
}
