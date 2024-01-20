fn main() {
    println!("cargo:rustc-link-lib=dylib=leapC");
    println!(r"cargo:rustc-link-search=native=C:\Program Files\Ultraleap\LeapSDK\lib\x64");
}