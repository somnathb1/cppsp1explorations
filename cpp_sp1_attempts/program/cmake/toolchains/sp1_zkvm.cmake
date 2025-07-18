# Tell CMake this is a bare-metal RISC-V target
set(CMAKE_SYSTEM_NAME      Generic)
set(CMAKE_SYSTEM_PROCESSOR riscv32)

# Force the right triple and strip host CRT/stl
set(CMAKE_C_FLAGS_INIT     "--target=riscv32im-succinct-zkvm-elf -nostdlib")
set(CMAKE_CXX_FLAGS_INIT   "--target=riscv32im-succinct-zkvm-elf -nostdlib -fno-exceptions -fno-rtti")

# Disable executable test-links during configure (no sysroot yet)
set(CMAKE_TRY_COMPILE_TARGET_TYPE STATIC_LIBRARY)

# Optional: where to look for pre-built RV32 libraries
# set(CMAKE_SYSROOT /opt/sp1/sysroot)
