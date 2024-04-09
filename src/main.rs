use std::{
    ffi::c_double,
    time::{Duration, SystemTime},
};

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {

        include!("hpx_rs_defs.h");
        include!("stencil.h");

        fn fibonacci(u: u64) -> u64;
        fn fibonacci_hpx(u: u64) -> u64;

        fn start(func: unsafe fn(i32, *mut *mut c_char) -> i32) -> i32;
        fn stop() -> i32;

        type stepper;

        fn new_stepper() -> UniquePtr<stepper>;

        // type partition_data
        type amoolspace;
        fn do_work(self: &stepper, a: usize, b: usize, c: usize) -> UniquePtr<amoolspace>;

        fn amool();

    }
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn your_main() -> i32 {
    let fib_hpx = ffi::fibonacci_hpx(10);
    println!("fib hpx(10) = {:?}", fib_hpx);
    return 0;
}

fn rust_main(a: i32, b: *mut *mut i8) -> i32 {
    ffi::amool();
    return ffi::stop();
}

fn stencil_main() -> i32 {
    let now = SystemTime::now();
    let step = ffi::new_stepper();
    let res = (step).as_ref().unwrap().do_work(10, 10, 45);
    println!("res = \nelapsed = {:?}", now.elapsed().unwrap());
    return 0;
}

fn main() {
    let fib = ffi::fibonacci(10);
    println!("fib (non-hpx) (10) = {:?}", fib);

    let _a = ffi::start(rust_main as fn(i32, *mut *mut i8) -> i32);
    // stencil_main()
    // let _c = your_main();
    // let _b = ffi::stop();
}
