use std::env;

mod argparse;
mod help;
mod paths;
mod variables;

fn main() {
    // Disable unicode (supposedly speeds up running shell scripts)
    env::set_var("LANG", "C");
    env::set_var("LC_ALL", "C");

    // Define default variables (radula-rs won't use a config file, but will
    // rely on flags instead)
    let radula_ccache = true;
    // Prevents running the basic check function as it's not ready yet...
    let radula_check = false;

    #[cfg(target_arch = "aarch64")]
    let radula_genome = "aarch64";

    #[cfg(target_arch = "arm")]
    let radula_genome = "armv6zk";

    #[cfg(target_arch = "x86")]
    let radula_genome = "i686";

    #[cfg(target_arch = "x86_64")]
    let radula_genome = "x86-64";

    let radula_parallel = true;

    // Required when working with the following cerata `radula`, `hydroskeleton`
    env::set_var("genome", radula_genome);

    if radula_ccache {
        variables::radula_behave_ccache_variables();
    }

    //argparse::radula_argparse(radula_genome);
    variables::radula_behave_toolchain_variables();
}
