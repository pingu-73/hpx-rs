fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/hpx_rs_main.cc")
        .std("c++17")
        .compile("hpx-rs-demo");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/hpx_rs_main.cc");
    println!("cargo:rerun-if-changed=include/hpx_rs_defs.h");
    println!("cargo:rerun-if-changed=include/stencil.h");
    println!("cargo:rustc-flags={}", std::env::var("LD_FLAGS").unwrap());
}
