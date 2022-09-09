mod client;

use std::ffi::{CStr};
use std::os::raw::c_char;

// extern "C" {
//     fn emscripten_builtin_malloc(size: usize) -> *mut u8;
//     fn emscripten_builtin_free(ptr: *mut u8);  
// }

// #[no_mangle]
// pub unsafe extern "C" fn malloc(size: usize) -> *mut u8 {
//     println!("My malloc");

//     emscripten_builtin_malloc(size)
// }


fn main() {
    println!("Hello world!");

    unsafe {
        client::wasm_rt::init();

        println!("Runtime initialised");

        (client::emscripten_rt::__wasm_call_ctors.unwrap())();
        println!("Wasm ctors called!");

        println!("Game has {} pages of memory!", (*client::wasm_rt::memory).pages);
        println!("Z_envZ_console_log = {:?}", client::Z_envZ_console_log);
        println!("Z_envZ_clear_screen = {:?}", client::Z_envZ_clear_screen);
        println!("Z_envZ_glActiveTexture = {:?}", client::Z_envZ_glActiveTexture);
        println!("Z_envZ_draw_rect = {:?}", client::Z_envZ_draw_rect);

        let a = (client::game_init.unwrap())();
        println!("Game init result: {}", a);


        for _ in 0..5 {
            println!("Calling loop!");
            (client::game_loop.unwrap())();
        }
    }
}