fn main() {
    for lib in ["X11", "xkbfile"] {
        println!("cargo:rustc-link-lib={}", lib);
    }
}
