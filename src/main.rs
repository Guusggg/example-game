extern crate client_sys;

use std::ffi::{CStr};
use std::os::raw::c_char;

use client_sys::wasm_rt::WasmRtMemory;


fn main() {
    unsafe { 
        client_sys::Z_envZ_clear_screen = Some(|| {

        });
    }

    println!("Hello world!");

    unsafe {
        client_sys::wasm_rt::init();

        println!("Runtime initialised");

        (client_sys::emscripten_rt::wasm_call_ctors.unwrap())();
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