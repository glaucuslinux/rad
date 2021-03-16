use super::paths;
use std::env;
use std::path::Path;

pub fn radula_behave_ccache_variables() {
    env::set_var(
        "PATH",
        Path::new(paths::RADULA_CCACHE_PATH)
            .join(":")
            .join(env::var("PATH").unwrap().strip_prefix("/").unwrap()),
    );
}
