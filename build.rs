use std::env;

fn main() {
    println!("The current directory is {}", env::current_dir().unwrap().display());

    println!("cargo:rustc-link-search=native={}", "./c");
    println!("cargo:rustc-link-search=native={}", "./go");

    println!("cargo:rustc-link-lib=static={}", "helloc");
    println!("cargo:rustc-link-lib=static={}", "hellogo");
}