// Copyright (c) 2018-2022, Firas Khalil Khana
// Distributed under the terms of the ISC License

use std::error::Error;

mod architecture;
mod ccache;
mod ceras;
mod clean;
mod constants;
mod construct;
mod flags;
mod functions;
mod help;
// mod image;
mod options;
mod swallow;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    options::radula_options().await?;

    Ok(())
}
