use std::ffi::{CStr};
use std::os::raw::c_char;

mod wasm;
mod emscripten;
mod client;

const MEMORY: wasm::Memory = wasm::Memory::from_raw();
const TABLE: wasm::FunctionTable = wasm::FunctionTable::from_raw();

pub type Pointer<'a, T> = *const T;
pub type PointerMut<'a, T> = *mut T;

pub type Ref<'a, T> = &'a T;
pub type RefMut<'a, T> = &'a mut T;

pub type Global<T> = &'static T;
pub type GlobalMut<T> = &'static mut T;

// fn call_import<F>(method: Option<F>) {
//     match method {

//     }
// }
   
// Imports
#[no_mangle]
pub static mut Z_envZ_console_log: Option<fn(u32) -> ()> = Some(|val| unsafe {
    let val = MEMORY.ptr::<c_char>(val);

    let str = CStr::from_ptr(val).to_str().unwrap();
    println!("console_log called: {}", str);
});

#[no_mangle]
pub static mut Z_envZ_clear_screen: Option<fn() -> ()> = Some(|| {
    println!("clear_screen called!");
});

#[no_mangle]
pub static mut Z_envZ_glActiveTexture: Option<fn(u32) -> ()> = Some(|_val| {
    println!("glActiveTexture called!");
});

#[no_mangle]
pub static mut Z_envZ_draw_rect: Option<fn(u32, u32, u32, u32) -> ()> = Some(|_x, _y, _width, _height| {
    println!("draw_rect called!");
});