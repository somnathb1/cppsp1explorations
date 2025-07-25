use pkg_config;
use std::{env, path::Path};

fn main() {
    println!(
        "cargo:rustc-link-search=native={}",
        std::env::var("CARGO_MANIFEST_DIR").unwrap()
    );

    // println!("cargo:rustc-link-search=native=/home/som/.sp1/riscv/riscv32im-linux-x86_64/riscv32-unknown-elf/lib");
    // println!("cargo:rustc-link-lib=static=supc++");

    // println!("cargo:rustc-link-lib=static=fibcpp");
    // println!("cargo:rustc-link-search=native=/home/som/Documents/code/cppsp1explorations/templibs");
    let conan_dir = Path::new("build/conan");
    let dst = cmake::Config::new("../silkworm")
        .build_arg("-j16") // Use 4 parallel jobs, adjust as needed
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("CMAKE_SYSTEM_NAME", "Generic")
        .define("CMAKE_SYSTEM_PROCESSOR", "riscv32")
        .define("CMAKE_CXX_STANDARD", "17")
        .define("CMAKE_CXX_STANDARD_REQUIRED", "ON")
        .define(
            "GMP_LIBRARY",
            "/home/som/Documents/code/cppsp1explorations/templibs/gmp",
        )
        .define(
            "GMP_INCLUDE_DIR",
            "/home/som/Documents/code/cppsp1explorations/templibs/gmp",
        )
        .define("CATCH_BUILD_TESTING", "OFF")
        .define("CONAN_HOST_PROFILE", "riscv32-baremetal")
        // .define("SILKWORM_CORE_ONLY", "ON")
        .define("SILKWORM_CORE_USE_ABSEIL", "OFF")
        .profile("Debug")
        .build_arg("LIBFF_WITH_GMP=OFF")
        .define("CMAKE_PREFIX_PATH", conan_dir)
        // .define("CMAKE_CXX_FLAGS", "-Wno-dev")
        // .target("silkworm_core")
        // .build_target("silkworm_core")
        .build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!(
        "cargo:rustc-link-search=native={}/build/silkworm/core",
        dst.display()
    );
    println!("cargo:rustc-link-search=native={}/build/silkworm/dev", dst.display());
    println!("cargo:rustc-link-search=native={}/build/third_party/secp256k1", dst.display());
    println!("cargo:rustc-link-search=native={}/build/third_party/evmone", dst.display());
    println!("cargo:rustc-link-search=native={}/build/deps/src/blst", dst.display());

    // println!(
    //     "cargo:rustc-link-search=native={}/build/third_party/libff/libff/libff",
    //     dst.display()
    // );

    println!("cargo:rustc-link-search=native=/home/som/Documents/code/cppsp1explorations/templibs");
    println!("cargo:rustc-link-search=native=/home/som/Documents/code/cppsp1explorations/templibs/gmp");


    println!("cargo:rustc-link-lib=static=c");
    println!("cargo:rustc-link-lib=static=gcc");
    println!("cargo:rustc-link-lib=static=nosys");
    println!("cargo:rustc-link-lib=static=stdc++");
    // println!("cargo:rustc-link-lib=static=cstd");
    println!("cargo:rustc-link-lib=static=gmp");
    println!("cargo:rustc-link-lib=static=ff");
    println!("cargo:rustc-link-lib=static=silkworm_dev");
    println!("cargo:rustc-link-lib=static=silkworm_core");
    println!("cargo:rustc-link-lib=static=evmone");
    println!("cargo:rustc-link-lib=static=blst");
    println!("cargo:rustc-link-lib=static=secp256k1");
    // println!("cargo:rustc-link-lib=static=silkworm_sentry");
    println!("cargo:rustc-link-lib=static=ethash");
    println!("cargo:rustc-link-lib=static=keccak");
    println!("cargo:rustc-link-lib=static=tooling");
    println!("cargo:rustc-link-lib=static=evmc-loader");
    println!("cargo:rustc-link-lib=static=evmc-loader");
    // println!("cargo:rustc-link-lib=static=intx");


    // println!("cargo:rustc-link-lib=static=nlohmann");
    // println!("cargo:rustc-link-lib=static=gsl");
    // println!("cargo:rustc-link-lib=static=nlohmann_json");

    let include_dir = dst.join("include");

    // Compile the C++ code and generate bindings
    let mut binding = cxx_build::bridge("src/main.rs");
    let mut builder = binding
        .include(include_dir)
        .cpp(true)
        .std("c++20")
        .file("src/wrapper.cpp")
        .include("src/include")
        // .include("../external/json/include")
        .flag_if_supported("-nostdlib++") // g++: don’t pull libstdc++ while linking the static lib
        .include(
            "/home/som/.sp1/riscv/riscv32im-linux-x86_64/riscv32-unknown-elf/include/c++/13.2.0",
        );

    for (key, val) in env::vars() {
        if key.starts_with("CONAN_INCLUDE_DIRS_") {
            for dir in val.split(';') {
                builder = builder.include(dir);
            }
        }
    }

        // ── 1. point pkg-config at the .pc files Conan generated ──────────────
    let conan_pc_dir = dst.join("build/conan2");       // <-- adjust if needed
    std::env::set_var("PKG_CONFIG_PATH", &conan_pc_dir);
    std::env::set_var("PKG_CONFIG_ALLOW_CROSS", "1");
    std::env::set_var("PKG_CONFIG_ALLOW_CROSS_riscv32im-succinct-zkvm-elf", "1");
    println!("cargo:warning=PKG_CONFIG_PATH set to: {}", conan_pc_dir.display());
    // ── 3. pull cflags (include dirs) from the .pc files we care about ────
    for pkg in ["ms-gsl", "nlohmann_json", "magic_enum", "tl-expected"] {
        if let Ok(meta) = pkg_config::Config::new()
            // .statik(true) // ensure −static libs if present
            .probe(pkg)
        {

            println!("cargo:warning=bazooka pkg-config found pkg {}", pkg);

            // add every -I path to the wrapper build
            for mut dir in meta.include_paths {
                builder.include(dir);
                // println!("cargo:warning=bazooka pkg-config include path: {}", dir.display());
            }
            // optional: also link the libs that pkg-config lists
            for lib_path in meta.link_paths {
                println!("cargo:rustc-link-search=native={}", lib_path.display());
            }
            for lib in meta.libs {
                println!("cargo:rustc-link-lib=static={}", lib);
            }
        } else {
            println!("cargo:warning=pkg-config couldn’t find {}", pkg);
        }
    }
    // builder.
    builder.compile("silk_bazooka");
}
