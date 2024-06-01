#![allow(dead_code)]

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {

        include!("hpx_rs_defs.h");

        fn fibonacci(u: u64) -> u64;
        fn fibonacci_hpx(u: u64) -> u64;

        unsafe fn copy_hpx(src: *const i32, src_end: *const i32, dest: *mut i32);
        
        fn hpx_init() -> i32;
        fn start() -> i32;
        fn stop() -> i32;
    }
}

fn main() {
    let fib = ffi::fibonacci(10);
    println!("fib (non-hpx) (10) = {:?}", fib);

    ffi::hpx_init();
}

#[no_mangle]
pub extern "C" fn hpx_main_rust(_argc: i32, _argv: *mut *mut i8) -> i32 {
    let fib_hpx = ffi::fibonacci_hpx(10);
    println!("fib hpx(10) = {:#?}", fib_hpx);

    let src = vec![1, 6, 3, 4, 9, 5, 11];
    let mut dest = vec![0; src.len()];

    unsafe {
        ffi::copy_hpx(src.as_ptr(), src.as_ptr().add(src.len()), dest.as_mut_ptr());
    }
    println! {"Destination: {:?}", dest};

    0
}
