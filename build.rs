fn main() {
    println!("cargo:rustc-link-lib=user32");
    println!("cargo:rerun-if-changed=icon.rc");
}