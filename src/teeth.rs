// Copyright (c) 2018-2022, Firas Khalil Khana
// Distributed under the terms of the ISC License

use std::env;

use super::constants;

extern crate num_cpus;

//
// Teeth Function
//

pub fn radula_behave_teeth_environment() {
    env::set_var(
        constants::RADULA_ENVIRONMENT_TOOTH_AUTORECONF,
        [
            constants::RADULA_TOOTH_AUTORECONF,
            " ",
            constants::RADULA_TOOTH_AUTORECONF_FLAGS,
        ]
        .concat(),
    );

    // Use `mawk` as the default AWK implementation
    env::set_var(
        constants::RADULA_ENVIRONMENT_TOOTH_AWK,
        constants::RADULA_TOOTH_MAWK,
    );

    // Use `byacc` as the default YACC implementation
    env::set_var(
        constants::RADULA_ENVIRONMENT_TOOTH_BISON,
        constants::RADULA_TOOTH_BYACC,
    );

    env::set_var(
        constants::RADULA_ENVIRONMENT_TOOTH_CHMOD,
        [
            constants::RADULA_TOOTH_CHMOD,
            " ",
            constants::RADULA_TOOTH_CHMOD_CHOWN_FLAGS,
        ]
        .concat(),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_TOOTH_CHOWN,
        [
            constants::RADULA_TOOTH_CHOWN,
            " ",
            constants::RADULA_TOOTH_CHMOD_CHOWN_FLAGS,
        ]
        .concat(),
    );

    // Use `flex` as the default LEX implementation (will be replaced by `reflex` in the future)
    env::set_var(
        constants::RADULA_ENVIRONMENT_TOOTH_FLEX,
        constants::RADULA_TOOTH_FLEX,
    );

    // Use `mawk` as the default AWK implementation
    env::set_var(
        constants::RADULA_ENVIRONMENT_TOOTH_GAWK,
        constants::RADULA_TOOTH_MAWK,
    );

    // Use `flex` as the default LEX implementation (will be replaced by `reflex` in the future)
    env::set_var(
        constants::RADULA_ENVIRONMENT_TOOTH_LEX,
        constants::RADULA_TOOTH_FLEX,
    );

    env::set_var(
        constants::RADULA_ENVIRONMENT_TOOTH_LN,
        [
            constants::RADULA_TOOTH_LN,
            " ",
            constants::RADULA_TOOTH_LN_FLAGS,
        ]
        .concat(),
    );

    // `make` and its flags
    env::set_var(
        constants::RADULA_ENVIRONMENT_TOOTH_MAKE,
        constants::RADULA_TOOTH_MAKE,
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_TOOTH_MAKEFLAGS,
        [
            "-j",
            &(num_cpus::get() as f32 * 1.5).to_string(),
            " ",
            constants::RADULA_TOOTH_MAKEFLAGS,
        ]
        .concat(),
    );

    env::set_var(
        constants::RADULA_ENVIRONMENT_TOOTH_MKDIR,
        [
            constants::RADULA_TOOTH_MKDIR,
            " ",
            constants::RADULA_TOOTH_MKDIR_FLAGS,
        ]
        .concat(),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_TOOTH_MKDIR_P,
        [
            constants::RADULA_TOOTH_MKDIR,
            " ",
            constants::RADULA_TOOTH_MKDIR_FLAGS,
        ]
        .concat(),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_TOOTH_MV,
        [
            constants::RADULA_TOOTH_MV,
            " ",
            constants::RADULA_TOOTH_MV_FLAGS,
        ]
        .concat(),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_TOOTH_PATCH,
        [
            constants::RADULA_TOOTH_PATCH,
            " ",
            constants::RADULA_TOOTH_PATCH_FLAGS,
        ]
        .concat(),
    );

    // Use `pkgconf` as the default PKG_CONFIG implementation
    env::set_var(
        constants::RADULA_ENVIRONMENT_TOOTH_PKG_CONFIG,
        constants::RADULA_TOOTH_PKGCONF,
    );

    env::set_var(
        constants::RADULA_ENVIRONMENT_TOOTH_RM,
        [
            constants::RADULA_TOOTH_RM,
            " ",
            constants::RADULA_TOOTH_RM_FLAGS,
        ]
        .concat(),
    );
    env::set_var(
        constants::RADULA_ENVIRONMENT_TOOTH_RSYNC,
        [
            constants::RADULA_TOOTH_RSYNC,
            " ",
            constants::RADULA_TOOTH_RSYNC_FLAGS,
        ]
        .concat(),
    );

    // Use `byacc` as the default YACC implementation
    env::set_var(
        constants::RADULA_ENVIRONMENT_TOOTH_YACC,
        constants::RADULA_TOOTH_BYACC,
    );
}
