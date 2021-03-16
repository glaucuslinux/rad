use std::io::{self, Write};
use std::process;
use std::{
    env::{self, current_dir},
    path::Path,
    path::PathBuf,
};

mod help;
mod paths;
mod variables;

fn main() {
    let mut args = env::args();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "b" => match args.next().unwrap().as_str() {
                "c" => {
                    println!("Do something");
                }
                _ => {
                    println!("Do nothing");
                }
            },
            _ => println!("Do nothing"),
        }
    }

    // Disable unicode (supposedly speeds up running shell scripts)
    env::set_var("LANG", "C");
    env::set_var("LC_ALL", "C");

    // Define default variables (radula-rs won't use a config file, but will
    // rely on flags instead)
    let radula_ccache = true;
    // Prevents running the basic check function as it's not ready yet...
    let radula_check = false;
    let radula_genome = "x86-64";
    let radula_parallel = true;

    // Required when working with the following cerata `radula`, `hydroskeleton`, `pcre2`
    env::set_var("genome", "x86-64");

    variables::radula_behave_ccache_variables();

    let radula_glaucus_directory = current_dir().unwrap();

    if args.len() < 2 {
        help::radula_open(help::RADULA_HELP);
        process::exit(1);
    }

    //match args[1].as_str() {
    //"b" | "-b" | "--behave" => match args[2].as_str() {
    //"b" | "bootstrap" => match args[3].as_str() {
    //"c" | "clean" => println!("Do nothing"),
    //"d" | "distclean" => println!("Do nothing"),

    //"h" | "-h" | "--help" => help::radula_open(help::RADULA_BEHAVE_BOOTSTRAP_HELP),

    //"i" | "image" => println!("Do nothing"),
    //"l" | "list" => println!("Do nothing"),
    //"r" | "require" => println!("Do nothing"),
    //"s" | "release" => println!("Do nothing"),
    //"t" | "toolchain" => println!("Do nothing"),
    //"x" | "cross" => println!("Do nothing"),
    //_ => {
    //help::radula_open(help::RADULA_BEHAVE_BOOTSTRAP_HELP);
    //process::exit(1);
    //}
    //},
    //"e" | "envenomate" => match args[3].as_str() {
    //"h" | "-h" | "--help" => help::radula_open(help::RADULA_BEHAVE_ENVENOMATE_HELP),

    //_ => {
    //help::radula_open(help::RADULA_BEHAVE_ENVENOMATE_HELP);
    //process::exit(1);
    //}
    //},

    //"h" | "-h" | "--help" => help::radula_open(help::RADULA_BEHAVE_HELP),

    //"i" | "binary" => match args[3].as_str() {
    //"h" | "-h" | "--help" => help::radula_open(help::RADULA_BEHAVE_BINARY_HELP),

    //_ => {
    //help::radula_open(help::RADULA_BEHAVE_BINARY_HELP);
    //process::exit(1);
    //}
    //},
    //_ => {
    //help::radula_open(help::RADULA_BEHAVE_HELP);
    //process::exit(1);
    //}
    //},
    //"c" | "-c" | "--ceras" => match args[2].as_str() {
    //"n" | "nom" | "name" => println!("Do nothing"),

    //"h" | "-h" | "--help" => help::radula_open(help::RADULA_CERAS_HELP),

    //"v" | "ver" | "version" => println!("Do nothing"),
    //"u" | "url" | "source" => println!("Do nothing"),
    //"s" | "sum" | "checksum" | "sha512sum" => println!("Do nothing"),
    //"y" | "cys" | "cyst" | "cysts" => println!("Do nothing"),
    //"c" | "cnt" | "concentrate" | "concentrates" => println!("Do nothing"),
    //"l" | "lic" | "license" | "licenses" => println!("Do nothing"),
    //_ => {
    //help::radula_open(help::RADULA_CERAS_HELP);
    //process::exit(1);
    //}
    //},
    //"g" | "-g" | "--genome" => println!("{}", radula_genome),

    //"h" | "-h" | "--help" => help::radula_open(help::RADULA_HELP),

    //"v" | "-v" | "--version" => println!("{}", help::RADULA_VERSION),
    //_ => {
    //help::radula_open(help::RADULA_BEHAVE_HELP);
    //process::exit(1);
    //}
    //}
}
