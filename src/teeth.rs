// Copyright (c) 2018-2022, Firas Khalil Khana
// Distributed under the terms of the ISC License

use std::env;
use std::error::Error;

use super::constants;

//
// Teeth Function
//

pub fn radula_behave_teeth_environment() -> Result<(), Box<dyn Error>> {
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
        constants::RADULA_TOOTH_MAKEFLAGS,
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

    Ok(())
}

#[test]
fn test_radula_behave_teeth_environment() -> Result<(), Box<dyn Error>> {
    radula_behave_teeth_environment()?;

    println!(
        "\nAUTORECONF :: {}",
        env::var(constants::RADULA_ENVIRONMENT_TOOTH_AUTORECONF)?
    );
    println!(
        "AWK        :: {}",
        env::var(constants::RADULA_ENVIRONMENT_TOOTH_AWK)?
    );
    println!(
        "BISON      :: {}",
        env::var(constants::RADULA_ENVIRONMENT_TOOTH_BISON)?
    );
    println!(
        "CHMOD      :: {}",
        env::var(constants::RADULA_ENVIRONMENT_TOOTH_CHMOD)?
    );
    println!(
        "CHOWN      :: {}",
        env::var(constants::RADULA_ENVIRONMENT_TOOTH_CHOWN)?
    );
    println!(
        "FLEX       :: {}",
        env::var(constants::RADULA_ENVIRONMENT_TOOTH_FLEX)?
    );
    println!(
        "GAWK       :: {}",
        env::var(constants::RADULA_ENVIRONMENT_TOOTH_GAWK)?
    );
    println!(
        "LEX        :: {}",
        env::var(constants::RADULA_ENVIRONMENT_TOOTH_LEX)?
    );
    println!(
        "LN         :: {}",
        env::var(constants::RADULA_ENVIRONMENT_TOOTH_LN)?
    );
    println!(
        "MAKE       :: {}",
        env::var(constants::RADULA_ENVIRONMENT_TOOTH_MAKE)?
    );
    println!(
        "MAKEFLAGS  :: {}",
        env::var(constants::RADULA_ENVIRONMENT_TOOTH_MAKEFLAGS)?
    );
    println!(
        "MKDIR      :: {}",
        env::var(constants::RADULA_ENVIRONMENT_TOOTH_MKDIR)?
    );
    println!(
        "MKDIR_P    :: {}",
        env::var(constants::RADULA_ENVIRONMENT_TOOTH_MKDIR_P)?
    );
    println!(
        "MV         :: {}",
        env::var(constants::RADULA_ENVIRONMENT_TOOTH_MV)?
    );
    println!(
        "PATCH      :: {}",
        env::var(constants::RADULA_ENVIRONMENT_TOOTH_PATCH)?
    );
    println!(
        "PKG_CONFIG :: {}",
        env::var(constants::RADULA_ENVIRONMENT_TOOTH_PKG_CONFIG)?
    );
    println!(
        "RM         :: {}",
        env::var(constants::RADULA_ENVIRONMENT_TOOTH_RM)?
    );
    println!(
        "RSYNC      :: {}",
        env::var(constants::RADULA_ENVIRONMENT_TOOTH_RSYNC)?
    );
    println!(
        "YACC       :: {}\n",
        env::var(constants::RADULA_ENVIRONMENT_TOOTH_YACC)?
    );

    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_TOOTH_AUTORECONF)?,
        "autoreconf -vfis"
    );
    assert_eq!(env::var(constants::RADULA_ENVIRONMENT_TOOTH_AWK)?, "mawk");
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_TOOTH_BISON)?,
        "byacc"
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_TOOTH_CHMOD)?,
        "chmod -Rv"
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_TOOTH_CHOWN)?,
        "chown -Rv"
    );
    assert_eq!(env::var(constants::RADULA_ENVIRONMENT_TOOTH_FLEX)?, "flex");
    assert_eq!(env::var(constants::RADULA_ENVIRONMENT_TOOTH_GAWK)?, "mawk");
    assert_eq!(env::var(constants::RADULA_ENVIRONMENT_TOOTH_LEX)?, "flex");
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_TOOTH_LN)?,
        "ln -fnsv"
    );
    assert_eq!(env::var(constants::RADULA_ENVIRONMENT_TOOTH_MAKE)?, "make");
    assert_eq!(env::var(constants::RADULA_ENVIRONMENT_TOOTH_MAKEFLAGS)?, "-j4 -O");
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_TOOTH_MKDIR)?,
        "/usr/bin/install -dv"
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_TOOTH_MKDIR_P)?,
        "/usr/bin/install -dv"
    );
    assert_eq!(env::var(constants::RADULA_ENVIRONMENT_TOOTH_MV)?, "mv -v");
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_TOOTH_PATCH)?,
        "patch --verbose"
    );
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_TOOTH_PKG_CONFIG)?,
        "pkgconf"
    );
    assert_eq!(env::var(constants::RADULA_ENVIRONMENT_TOOTH_RM)?, "rm -frv");
    assert_eq!(
        env::var(constants::RADULA_ENVIRONMENT_TOOTH_RSYNC)?,
        "rsync -vaHAXSx"
    );
    assert_eq!(env::var(constants::RADULA_ENVIRONMENT_TOOTH_YACC)?, "byacc");

    Ok(())
}
