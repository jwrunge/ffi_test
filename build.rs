use std::env;

fn main() {
    println!("The current directory is {}", env::current_dir().unwrap().display());

    println!("-lstdc++"); // Link to C++ standard library

    println!("cargo:rustc-link-search=native={}", "./c");
    println!("cargo:rustc-link-search=native={}", "./cpp");
    // println!("cargo:rustc-link-search=native={}", "./go");

    println!("cargo:rustc-link-lib=static={}", "helloc");
    println!("cargo:rustc-link-lib=static={}", "hellocpp");
    // println!("cargo:rustc-link-lib=static={}", "hellogo");
}