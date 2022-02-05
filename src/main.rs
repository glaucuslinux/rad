// Copyright (c) 2018-2022, Firas Khalil Khana
// Distributed under the terms of the ISC License

use std::error::Error;

mod architecture;
mod bootstrap;
mod ccache;
mod ceras;
mod clean;
mod constants;
mod construct;
mod cross;
mod flags;
mod help;
mod pkg_config;
mod rsync;
mod teeth;
mod toolchain;
// mod image;
mod options;
mod swallow;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    options::radula_options().await?;

    Ok(())
}
