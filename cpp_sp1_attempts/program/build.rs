fn main() {
    println!("cargo:rustc-link-search=native={}", std::env::var("CARGO_MANIFEST_DIR").unwrap());

    // println!("cargo:rustc-link-lib=static=supc++");
    println!("cargo:rustc-link-lib=static=stdc++");
    println!("cargo:rustc-link-lib=static=c");
    println!("cargo:rustc-link-lib=static=nosys");
    println!("cargo:rustc-link-lib=static=gcc");
    // println!("cargo:rustc-link-search=native=/home/som/.sp1/riscv/riscv32im-linux-x86_64/riscv32-unknown-elf/lib");
    println!("cargo:rustc-link-search=native=../../cppsp1explorations/templibs");

    // Compile the C++ code and generate bindings
    cxx_build::bridge("src/main.rs") // path to the file with #[cxx::bridge]
        .file("src/fib.cpp") // additional C++ source files
        .include("src/include") // where fib.h lives
        // .compiler("riscv-none-elf-g++")
        // .archiver("riscv-none-elf-ar")
        // .target("riscv32-im")
        // .flag("-march=rv32im")
        .flag_if_supported("-nostdlib++")   // g++: don’t pull libstdc++ while linking the static lib
        .include("/home/som/.sp1/riscv/riscv32im-linux-x86_64/riscv32-unknown-elf/include/c++/13.2.0")
        .compile("fib"); // static lib name: libfib.a

}
