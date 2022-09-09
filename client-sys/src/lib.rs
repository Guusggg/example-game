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
        pub static wasm_call_ctors: Option<unsafe extern "C" fn() -> ()>;
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


#[no_mangle]
#[used]
pub static mut Z_envZ_console_log: Option<fn(u32) -> ()> = Some(|val| {
    unsafe {
        // let val = (*client_sys::wasm_rt::memory).data.add(val.try_into().unwrap());

        // let str = CStr::from_ptr(val as *mut c_char);
        println!("console_log called: {:?}", "");
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