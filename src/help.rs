pub const RADULA_BEHAVE_BINARY_HELP: &'static str = "
Usage:
\tradula [ b | -b | --behave ] [ i | binary ] [ Options ] [ Cerata ]

Options:
\td, decyst    \tRemove binary cerata without preserving their cysts

\th, -h, --help\tDisplay this help message

\ti, install   \tInstall binary cerata (default)
\tr, remove    \tRemove binary cerata while preserving their cyst(s)
\ts, search    \tSearch for binary cerata within the remote repositories
\tu, upgrade   \tUpgrade binary cerata";

pub const RADULA_BEHAVE_BOOTSTRAP_HELP: &'static str = "
Usage:
\tradula [ b | -b | --behave ] [ b | bootstrap ] [ Options ]

Options:
\tc, clean     \tClean up while preserving sources and backups
\td, distclean \tClean up everything

\th, -h, --help\tDisplay this help message

\ti, image     \tCreate a .img file of the glaucus system
\tl, list      \tList supported genomes and species
\tr, require   \tCheck if host has all required packages
\ts, release   \tRelease a compressed tarball of the toolchain
\tt, toolchain \tBootstrap a cross compiler toolchain
\tx, cross     \tBootstrap a cross compiled glaucus system";

pub const RADULA_BEHAVE_ENVENOMATE_HELP: &'static str = "
Usage:
\tradula [ b | -b | --behave ] [ e | envenomate ] [ Options ] [ Cerata ]

Options:
\td, decyst    \tRemove cerata without preserving their cyst(s)

\th, -h, --help\tDisplay this help message

\ti, install   \tInstall cerata from source (default)
\tr, remove    \tRemove cerata while preserving their cyst(s)
\ts, search    \tSearch for cerata within the cerata directory
\tu, upgrade   \tUpgrade cerata";

pub const RADULA_BEHAVE_HELP: &'static str = "
Usage:
\tradula [ b | -b | --behave ] [ Options ]

Options:
\tb, bootstrap \tPerform bootstrap behavior
\te, envenomate\tPerform envenomate behavior

\th, -h, --help\tDisplay this help message

\ti, binary    \tPerform binary behavior";

pub const RADULA_CERAS_HELP: &'static str = "
Usage:
\tradula [ c | -c | --ceras ] [ Options ] [ Cerata ]

Options:
\tc, cnt, concentrate, concentrates\tDisplay cerata concentrate(s)

\th, -h, --help                    \tDisplay this help message

\tl, lic, license, licenses        \tDisplay cerata license(s)
\tn, nom, name                     \tDisplay cerata name(s)
\ts, sum, checksum, sha512sum      \tDisplay cerata sha512sum(s)
\tu, url, source                   \tDisplay cerata source(s)
\tv, ver, version                  \tDisplay cerata version(s)
\ty, cys, cyst, cysts              \tDisplay cerata cyst(s)";

pub const RADULA_HELP: &'static str = "
Usage:
\tradula [ Options ]

Options:
\tb, -b, --behave \tPerform any of the following behaviors:
\t                \tbinary, bootstrap, envenomate

\tc, -c, --ceras  \tDisplay ceras information
\tg, -g, --genome \tDisplay current genome

\th, -h, --help   \tDisplay this help message

\ts, -s, --species\tDisplay current species
\tv, -v, --version\tDisplay current version number";

pub const RADULA_VERSION: &'static str = "
Copyright (c) 2018-2021, Firas Khalil Khana
Distributed under the terms of the ISC License

radula version 3.3.0 (genome species)";

// This function will probably be changed in the future to something that makes more sense...
pub fn radula_open(x: &str) {
    println!("{}\n{}", RADULA_VERSION, x);
}
