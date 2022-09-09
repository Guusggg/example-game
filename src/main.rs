extern crate client_sys;

use std::ffi::{CStr};
use std::os::raw::c_char;

extern "C" {
    fn emscripten_builtin_malloc(size: usize) -> *mut u8;
    fn emscripten_builtin_free(ptr: *mut u8);  
}

#[no_mangle]
pub unsafe extern "C" fn malloc(size: usize) -> *mut u8 {
    println!("My malloc");

    emscripten_builtin_malloc(size)
}


fn main() {
    println!("Hello world!");

    unsafe {
        client_sys::wasm_rt::init();

        println!("Runtime initialised");

        (client_sys::emscripten_rt::__wasm_call_ctors.unwrap())();
        println!("Wasm ctors called!");

        println!("Game has {} pages of memory!", (*client_sys::wasm_rt::memory).pages);
        println!("console_log = {:?}", client_sys::Z_envZ_console_log);

        let a = (client_sys::game_init.unwrap())();
        println!("Game init result: {}", a);


        for _ in 0..5 {
            println!("Calling loop!");
            (client_sys::game_loop.unwrap())();
        }
    }
}