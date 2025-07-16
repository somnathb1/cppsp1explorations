fn main() {
    // Make Cargo pass “-L native=.” so rustc can find libfib.{a|so}
    println!("cargo:rustc-link-search=native={}", std::env::var("CARGO_MANIFEST_DIR").unwrap());
    // inside build.rs
    cc::Build::new()
        .cpp(true)
        .file("cpp/fib.cpp")   // or src/cpp/fib.cpp
        .include("cpp")
        .flag("-O3")
        .compile("fib");           // produces libfib.a automatically
}
