// Copyright (c) 2018-2022, Firas Khalil Khana
// Distributed under the terms of the ISC License

use std::error::Error;
use std::process::Stdio;

use super::constants;
use tokio::process::Command;

//
// rsync Function
//

pub async fn radula_behave_rsync(source: &str, destination: &str) -> Result<(), Box<dyn Error>> {
    Command::new(constants::RADULA_TOOTH_RSYNC)
        .args(&[
            constants::RADULA_TOOTH_RSYNC_FLAGS,
            source,
            destination,
            "--delete",
        ])
        .stdout(Stdio::null())
        .spawn()?
        .wait()
        .await?;

    Ok(())
}
