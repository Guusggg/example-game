use std::ffi::{CStr};
use std::os::raw::c_char;

use client_sys::wasm_rt::WasmRtMemory;

#[no_mangle]
#[used]
pub static mut Z_envZ_console_log: Option<fn(u32) -> ()> = Some(|val| {
    unsafe {
        let val = (*client_sys::wasm_rt::memory).data.add(val.try_into().unwrap());

        let str = CStr::from_ptr(val as *mut c_char);
        println!("console_log called: {:?}", str);
    }
});

#[no_mangle]
#[used]
pub static mut Z_envZ_clear_screen: Option<fn() -> ()> = Some(|| {
    println!("clear_screen called!");
});

#[no_mangle]
#[used]
#[export_name = "Z_envZ_glActiveTexture"]
pub static mut glActiveTexture: Option<fn(u32) -> ()> = Some(|_val| {
    println!("glActiveTexture called!");
});

#[no_mangle]
#[used]
#[export_name = "Z_envZ_draw_rect"]
pub static mut draw_rect: Option<fn(u32, u32, u32, u32) -> ()> = Some(|_x, _y, _width, _height| {
    println!("draw_rect called!");
});


fn main() {
    println!("Hello world!");

    unsafe {
        client_sys::wasm_rt::init();

        println!("Runtime initialised");

        (client_sys::emscripten_rt::wasm_call_ctors.unwrap())();
        println!("Wasm ctors called!");

        println!("Game has {} pages of memory!", (*client_sys::wasm_rt::memory).pages);
        println!("console_log = {:?}", Z_envZ_console_log);

        let a = (Z_my_gameZ_game_init.unwrap())();
        println!("Game init result: {}", a);

        for _ in 0..5 {
            println!("Calling loop!");
            (Z_my_gameZ_game_loop.unwrap())();
        }
    }
}