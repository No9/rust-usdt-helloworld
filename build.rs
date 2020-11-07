use std::process::Command;
use std::path::Path;

fn main() {
    Command::new("make")
        .args(&["clean"])
        .current_dir(&Path::new("./native"))
        .status()
        .unwrap();

    Command::new("make")
        .args(&["static"])
        .current_dir(&Path::new("./native"))
        .status()
        .unwrap();

    println!("cargo:rustc-link-search=native={}", "./native");
    println!("cargo:rustc-link-lib=static=hello");
}