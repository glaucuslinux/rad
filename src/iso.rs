// Copyright (c) 2018-2022, Firas Khalil Khana
// Distributed under the terms of the ISC License

use std::error::Error;

#[cfg(test)]
use super::bootstrap;

pub async fn radula_behave_bootstrap_cross_iso() -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[tokio::test]
async fn test_radula_behave_bootstrap_cross_iso() -> Result<(), Box<dyn Error>> {
    bootstrap::radula_behave_bootstrap_environment().await?;

    Ok(())
}
