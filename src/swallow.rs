// Copyright (c) 2018-2022, Firas Khalil Khana
// Distributed under the terms of the ISC License

use std::env;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::Arc;

use super::ceras;
use super::constants;

use colored::Colorize;
use futures_util::{stream, StreamExt};
use git2::Repository;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use reqwest::{Client, Url};
use tokio::{fs, fs::File, io::AsyncWriteExt, task};

// Asynchronously clone repos
pub async fn radula_behave_clone(commit: String, url: String, path: String) {
    // Clone ceras's source `git` repository and checkout the freshly cloned `git`
    // repository at the specified commit number
    let repo = Repository::clone(&url, path).unwrap();
    repo.checkout_tree(&repo.revparse_ext(&commit).unwrap().0, None)
        .unwrap();
}

// Asynchronously download tarballs
pub async fn radula_behave_download(url: &'static str, m: Arc<MultiProgress>) {
    let client = Client::new();

    let mut res = client.get(url).send().await.unwrap();
    let size = res.content_length().unwrap_or(0);

    let url = Url::parse(url).unwrap();

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

    let mut out = File::create(file.clone()).await.unwrap();

    while let Some(chunk) = res.chunk().await.unwrap() {
        pb.inc(chunk.len() as u64);
        out.write(&chunk).await.unwrap();
    }

    pb.finish();

    // Must flush manually
    out.flush().await.unwrap();
}

// Fetch, verify and extract ceras's source
pub async fn radula_behave_swallow(name: &'static str) {
    // Receive the variables from the `ceras` file
    let ceras = ceras::radula_behave_ceras_parse(name);

    let version = ceras.ver.unwrap();
    let commit = ceras.cmt.unwrap();
    let url = Url::parse(&ceras.url.unwrap()).unwrap();
    let sum = ceras.sum.unwrap();

    println!("{} swallow", "::".bold());

    // Get the path of the source directory
    let path = String::from(
        Path::new(&env::var(constants::RADULA_ENVIRONMENT_DIRECTORY_SOURCES).unwrap())
            .join(name)
            .to_str()
            .unwrap(),
    );

    let file = String::from(
        url.path_segments()
            .and_then(|segments| segments.last())
            .unwrap_or_default(),
    );

    // Verify ceras's source tarball
    let radula_behave_verify = || {
        write!(
            Command::new(constants::RADULA_TOOTH_CHECKSUM)
                .arg(constants::RADULA_TOOTH_CHECKSUM_FLAGS)
                .stdin(Stdio::piped())
                .stdout(Stdio::null())
                .spawn()
                .unwrap()
                .stdin
                .unwrap(),
            "{}",
            [sum, " ".to_string(), file.clone()].concat()
        )
        .unwrap();
    };

    if !Path::is_dir(Path::new(&path)) {
        if version == constants::RADULA_TOOTH_GIT {
            async move {
                task::spawn(radula_behave_clone(commit, url.to_string(), path))
                    .await
                    .unwrap();
            };
        } else {
            let urls = vec![
                "https://musl.libc.org/releases/musl-1.2.2.tar.gz",
                "https://github.com/landley/toybox/archive/0.8.6.tar.gz",
            ];

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
                        task::spawn(radula_behave_download(url, m)).await.unwrap();
                        pb.inc(1);
                    }
                })
                .await;

            pb.finish();

            fs::create_dir(&path).await.unwrap();

            radula_behave_verify();

            // Extract ceras's source tarball
            Command::new(constants::RADULA_TOOTH_TAR)
                .args(&["xpvf", &file, "-C", &path])
                .stdout(Stdio::null())
                .spawn()
                .unwrap()
                .wait()
                .unwrap();
        }
    } else if version != constants::RADULA_TOOTH_GIT {
        // Only verify existing ceras's source tarballs without extracting them again
        radula_behave_verify();
    }
}
