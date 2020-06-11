#[cfg(all(unix, not(target_os = "macos")))]
fn main() {

    // add unix dependencies below
    println!("cargo:rustc-flags=-l X11 -l Xext");
}

#[cfg(target_os = "macos")]
fn main() {

    // add macos dependencies below
    // println!("cargo:rustc-flags=-l edit");
}
