// Copyright (c) 2018-2022, Firas Khalil Khana
// Distributed under the terms of the ISC License

use super::constants;

//
// Help Functions
//

pub fn radula_help(message: &'static str) {
    println!("{}\n\n{}", constants::RADULA_HELP_VERSION, message);
}
