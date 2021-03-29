// Copyright (c) 2018-2021, Firas Khalil Khana
// Distributed under the terms of the ISC License

mod constants;
mod functions;
mod options;

fn main() {
    //options::radula_options();
    functions::radula_behave_teeth_environment();
    println!("{:?}", std::env::vars());
}
