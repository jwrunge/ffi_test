use std::env;

fn main() {
    let path = "./go";
    let lib = "main";

    println!("The current directory is {}", env::current_dir().unwrap().display());

    println!("cargo:rustc-link-search=native={}", path);
    println!("cargo:rustc-link-lib=static={}", lib);
}