// Copyright (c) 2018-2023, Firas Khalil Khana
// Distributed under the terms of the ISC License

use std::error::Error;

mod architecture;
mod bootstrap;
mod ccache;
mod ceras;
mod clean;
mod compress;
mod constants;
mod construct;
mod cross;
mod flags;
mod help;
mod img;
mod iso;
mod options;
mod pkg_config;
mod rsync;
mod swallow;
mod teeth;
mod toolchain;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    options::radula_options().await?;

    Ok(())
}
