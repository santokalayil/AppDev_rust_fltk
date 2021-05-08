// build.rs
// -------- this file is used to set icon (binary_icon.ico) for binary exe ---

#[cfg(windows)]
extern crate winres;

#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("assets/binary_icon.ico");
    res.compile().unwrap();
}

#[cfg(unix)]
fn main() {
}