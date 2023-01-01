// Copyright (c) 2018-2023, Firas Khalil Khana
// Distributed under the terms of the ISC License

use std::error::Error;

use super::constants;

//
// Help Functions
//

pub fn radula_help(message: &'static str) -> Result<(), Box<dyn Error>> {
    println!("{}\n\n{}", constants::RADULA_HELP_VERSION, message);

    Ok(())
}
