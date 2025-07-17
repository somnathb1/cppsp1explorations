fn main() {
    // Make Cargo pass “-L native=.” so rustc can find libfib.{a|so}
    // println!("cargo:rustc-link-search=native={}", std::env::var("CARGO_MANIFEST_DIR").unwrap());


    println!("cargo:rustc-link-lib=static=stdc++");
    println!("cargo:rustc-link-search=native=/home/som/.local/xPacks/@xpack-dev-tools/riscv-none-elf-gcc/14.2.0-3.1/.content/riscv-none-elf/lib/rv32im/ilp32");
    // inside build.rs
    cc::Build::new()
        .cpp(true)
        .file("src/fib.cpp")   // or src/cpp/fib.cpp
        .include("src/include")
        // .compiler("riscv-none-elf-g++")
        .compiler("riscv-none-elf-c++")
        .cpp_link_stdlib("stdc++")
        // .compiler("/home/som/.sp1/riscv/riscv32im-linux-x86_64/bin/riscv32-unknown-elf-g++")
        .archiver("riscv-none-elf-ar")
        .include("/home/som/.local/xPacks/@xpack-dev-tools/riscv-none-elf-gcc/14.2.0-3.1/.content/riscv-none-elf/include/c++/14.2.0/")
        // .archiver("/home/som/.sp1/riscv/riscv32im-linux-x86_64/bin/riscv32-unknown-elf-ar")
        .flag("-L/home/som/.local/xPacks/@xpack-dev-tools/riscv-none-elf-gcc/14.2.0-3.1/.content/riscv-none-elf/lib") 
        .compile("fib");

    // println!("cargo:rustc-link-lib=static=stdc++");
    // println!("cargo:rustc-link-search=native=/home/som/.local/xPacks/@xpack-dev-tools/riscv-none-elf-gcc/14.2.0-3.1/.content/riscv-none-elf/lib/rv32im/ilp32");
    // // println!("Hi Emily is in Paris");
    // // Compile the C++ code and generate bindings
    // cxx_build::bridge("src/main.rs") // path to the file with #[cxx::bridge]
    //     .file("src/fib.cpp") // additional C++ source files
    //     .include("src/include") // where fib.h lives
    //     // .compiler("riscv-none-elf-g++")
    //     // .archiver("riscv-none-elf-ar")
    //     // .target("riscv32-im")
    //     // .flag("-march=rv32im")
    //     .include("/home/som/.local/xPacks/@xpack-dev-tools/riscv-none-elf-gcc/14.2.0-3.1/.content/riscv-none-elf/include/c++/14.2.0/")
    //     .flag("-L/home/som/.local/xPacks/@xpack-dev-tools/riscv-none-elf-gcc/14.2.0-3.1/.content/riscv-none-elf/lib/rv32im/ilp32") 
    //     .compile("fib"); // static lib name: libfib.a
}

// $ which c++
// /usr/bin/c++