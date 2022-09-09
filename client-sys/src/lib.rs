use std::ffi::{CStr};
use std::os::raw::c_char;

pub mod wasm_rt { 
    #[repr(C)]
    pub struct WasmRtMemory {
        pub data: *mut u8,
        pub pages: u32,
        pub max_pages: u32,
        pub size: u32
    }
    
    #[link(name = "client")]
    extern "C" {
        #[link_name = "Z_my_game_init"]
        pub fn init();
        
        #[link_name = "Z_my_game_free"]
        pub fn free();

        #[link_name = "Z_my_gameZ_memory"]
        pub static memory: *mut WasmRtMemory;
    }
}

pub mod emscripten_rt {
    #[link(name = "client")]
    extern "C" {
        #[link_name = "Z_my_gameZ___wasm_call_ctors"]
        pub static __wasm_call_ctors: Option<unsafe extern "C" fn() -> ()>;
    }
}
    
#[link(name = "client")]
extern "C" {
    #[link_name = "Z_my_gameZ_game_init"]
    pub static game_init: Option<unsafe extern "C" fn() -> u32>;
    #[link_name = "Z_my_gameZ_game_loop"]
    pub static game_loop: Option<unsafe extern "C" fn() -> ()>;    
    #[link_name = "Z_my_gameZ_game_get_score"]
    pub static game_get_score: Option<unsafe extern "C" fn() -> u32>;
}

#[link(name = "client")]
extern "C" {
    pub static mut Z_envZ_console_log: Option<extern fn(u32) -> ()>;
}