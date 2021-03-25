mod constants;
mod functions;
mod options;

fn main() {
    // Disable Unicode
    functions::radula_behave_unicode_variables();

    options::radula_options();
}
