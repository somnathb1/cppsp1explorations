
fn main() {
    // Make Cargo pass “-L native=.” so rustc can find libfib.{a|so}
    println!("cargo:rustc-link-search=native={}", std::env::var("CARGO_MANIFEST_DIR").unwrap());
    // inside build.rs
    cc::Build::new()
        // .cpp(true)
        .file("cpp/fib.c")   // or src/cpp/fib.cpp
        .include("cpp")
        // .compiler("riscv-none-elf-g++")
        .compiler("riscv-none-elf-gcc")
        // .compiler("/home/som/.sp1/riscv/riscv32im-linux-x86_64/bin/riscv32-unknown-elf-g++")
        .archiver("riscv-none-elf-ar")
        .include("/home/som/.local/xPacks/@xpack-dev-tools/riscv-none-elf-gcc/14.2.0-3.1/.content/riscv-none-elf/include/c++/14.2.0/")
        // .archiver("/home/som/.sp1/riscv/riscv32im-linux-x86_64/bin/riscv32-unknown-elf-ar")
        // .flag("-L/home/som/.local/xPacks/@xpack-dev-tools/riscv-none-elf-gcc/14.2.0-3.1/.content/bin/../lib/gcc/riscv-none-elf/14.2.0/../../../../riscv-none-elf/lib") // <-- Add this line
        .compile("fib");
}
