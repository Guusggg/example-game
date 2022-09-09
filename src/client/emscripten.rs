mod raw {
    #[link(name = "client")]
    extern "C" {
        #[link_name = "Z_my_gameZ___wasm_call_ctors"]
        pub static __wasm_call_ctors: Option<unsafe extern "C" fn() -> ()>;
    }
}

pub fn __wasm_call_ctors() {
    unsafe { raw::__wasm_call_ctors() }
}