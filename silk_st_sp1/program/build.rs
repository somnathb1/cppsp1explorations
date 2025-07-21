fn main() {
    println!("cargo:rustc-link-search=native={}", std::env::var("CARGO_MANIFEST_DIR").unwrap());

    // println!("cargo:rustc-link-search=native=/home/som/.sp1/riscv/riscv32im-linux-x86_64/riscv32-unknown-elf/lib");
    // println!("cargo:rustc-link-lib=static=supc++");
    println!("cargo:rustc-link-lib=static=stdc++");
    println!("cargo:rustc-link-lib=static=c");
    println!("cargo:rustc-link-lib=static=cstd");
    println!("cargo:rustc-link-lib=static=nosys");
    println!("cargo:rustc-link-lib=static=gcc");
    println!("cargo:rustc-link-lib=static=gmp");
    // println!("cargo:rustc-link-lib=static=fibcpp");
    println!("cargo:rustc-link-search=native=/home/som/Documents/code/cppsp1explorations/templibs");

    let dst = cmake::Config::new("../silkworm")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("CMAKE_SYSTEM_NAME", "Generic")
        .define("CMAKE_SYSTEM_PROCESSOR", "riscv32")
        .define("CMAKE_CXX_STANDARD", "17")
        .define("CMAKE_CXX_STANDARD_REQUIRED", "ON")
        .define("GMP_LIBRARY", "/home/som/Documents/code/cppsp1explorations/templibs/gmp")
        .define("GMP_INCLUDE_DIR", "/home/som/Documents/code/cppsp1explorations/templibs/gmp")
        .define("CATCH_BUILD_TESTING", "OFF")
        .profile("Debug")
        .build_arg("LIBFF_WITH_GMP=OFF")
        // .define("CMAKE_CXX_FLAGS", "-Wno-dev")
        // .target("silkworm_core")
        // .build_target("silkworm_core")
        .build();
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-search=native=/home/som/Documents/code/cppsp1explorations/templibs");

    let include_dir = dst.join("include");

    // // Compile the C++ code and generate bindings
    // cxx_build::bridge("src/main.rs") // path to the file with #[cxx::bridge]
    //     .include(include_dir)// where fib.h lives
    //     // .file("src/fib.cpp") // additional C++ source files
    //     // .compiler("riscv-none-elf-g++")
    //     // .archiver("riscv-none-elf-ar")
    //     // .target("riscv32-im")
    //     // .flag("-march=rv32im")
    //     .flag_if_supported("-nostdlib++")   // g++: don’t pull libstdc++ while linking the static lib
    //     .include("/home/som/.sp1/riscv/riscv32im-linux-x86_64/riscv32-unknown-elf/include/c++/13.2.0")
    //     .compile("fib"); // static lib name: libfib.a

}
