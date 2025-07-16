// Tell Rust where to find and how to link the C++ library.
#[link(name = "fib", kind = "static")]      // change to "dylib" for .so/.dylib
extern "C" {
    fn fib_c(n: u32) -> u64;
}

fn fibonacci(n: u32) -> u64 {
    // Safety: we trust fib_c’s contract (pure, no side effects).
    unsafe { fib_c(n) }
}

fn main() {
    for n in 0..=10 {
        println!("fib({}) = {}", n, fibonacci(n));
    }
}
