
fn main() {
    // Make Cargo pass “-L native=.” so rustc can find libfib.{a|so}
    println!("cargo:rustc-link-search=native={}", std::env::var("CARGO_MANIFEST_DIR").unwrap());
    // inside build.rs
    cc::Build::new()
        .file("cpp/fib.c")   // or src/cpp/fib.cpp
        .include("cpp")
        .compiler("riscv-none-elf-gcc")
        .archiver("riscv-none-elf-ar")
        .include("/home/som/.local/xPacks/@xpack-dev-tools/riscv-none-elf-gcc/14.2.0-3.1/.content/riscv-none-elf/include/c++/14.2.0/")
        .compile("fib");
}
