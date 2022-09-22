use std::ffi::{CStr};
use std::os::raw::c_char;

mod wasm;
mod emscripten;
mod my_game;

lazy_static! {
    pub static ref MEMORY: wasm::Memory = wasm::Memory::from_raw();
    pub static ref TABLE: wasm::FunctionTable = wasm::FunctionTable::from_raw();
}

pub type Pointer<'a, T> = *const T;
pub type PointerMut<'a, T> = *mut T;

pub type Ref<'a, T> = &'a T;
pub type RefMut<'a, T> = &'a mut T;

pub type Global<T> = &'static T;
pub type GlobalMut<T> = &'static mut T;

pub fn init() {
    wasm::init();
    println!("Runtime initialised");

    lazy_static::initialize(&MEMORY);
    lazy_static::initialize(&TABLE);

    emscripten::__wasm_call_ctors();
    println!("Wasm ctors called!");

    println!("Game has {} pages of memory!", MEMORY.pages());

    unsafe {
        // This is needed for now, otherwise they will be optimized away.
        println!("Z_envZ_console_log = {:?}", Z_envZ_console_log);
        println!("Z_envZ_clear_screen = {:?}", Z_envZ_clear_screen);
        println!("Z_envZ_glActiveTexture = {:?}", Z_envZ_glActiveTexture);
        println!("Z_envZ_draw_rect = {:?}", Z_envZ_draw_rect);
    }

    let a = my_game::game_init();
    println!("Game init result: {}", a);
}

pub fn step() {
    my_game::game_loop()
}
   
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