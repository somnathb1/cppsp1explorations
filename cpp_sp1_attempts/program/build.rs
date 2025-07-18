fn main() {
    println!("cargo:rustc-link-search=native={}", std::env::var("CARGO_MANIFEST_DIR").unwrap());

    // println!("cargo:rustc-link-search=native=/home/som/.sp1/riscv/riscv32im-linux-x86_64/riscv32-unknown-elf/lib");
    // println!("cargo:rustc-link-lib=static=supc++");
    println!("cargo:rustc-link-lib=static=stdc++");
    println!("cargo:rustc-link-lib=static=c");
    println!("cargo:rustc-link-lib=static=nosys");
    println!("cargo:rustc-link-lib=static=gcc");

    let dst = cmake::Config::new("../fib_cpp_project")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("CMAKE_SYSTEM_NAME", "Generic")
        .build();
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=fibcpp");
    println!("cargo:rustc-link-search=native=../../cppsp1explorations/templibs");

    let include_dir = dst.join("include");

    // Compile the C++ code and generate bindings
    cxx_build::bridge("src/main.rs") // path to the file with #[cxx::bridge]
        .include(include_dir)// where fib.h lives
        // .file("src/fib.cpp") // additional C++ source files
        // .compiler("riscv-none-elf-g++")
        // .archiver("riscv-none-elf-ar")
        // .target("riscv32-im")
        // .flag("-march=rv32im")
        .flag_if_supported("-nostdlib++")   // g++: don’t pull libstdc++ while linking the static lib
        .include("/home/som/.sp1/riscv/riscv32im-linux-x86_64/riscv32-unknown-elf/include/c++/13.2.0")
        .compile("fib"); // static lib name: libfib.a

}
