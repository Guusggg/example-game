use super::{Global, GlobalMut, Pointer, PointerMut, Ref, RefMut};
use std::mem;

#[allow(non_camel_case_types)]
mod raw {
    /** A Memory object. */
    #[repr(C)]
    pub struct wasm_rt_memory_t {
        /** The linear memory data, with a byte length of `size`. */
        pub data: *mut u8,
        /** The current and maximum page count for this Memory object. If there is no
         * maximum, `max_pages` is 0xffffffffu (i.e. UINT32_MAX). */
        pub pages: u32,
        /** The current size of the linear memory, in bytes. */
        pub max_pages: u32,
        pub size: u32,
    }
    /**
     * A function type for all `funcref` functions in a Table. All functions are
     * stored in this canonical form, but must be cast to their proper signature to
     * call.
     */
    pub type wasm_rt_funcref_t = extern "C" fn() -> ();

    /** A single element of a Table. */
    #[repr(C)]
    pub struct wasm_rt_elem_t {
        /** The index as returned from `wasm_rt_register_func_type`. */
        func_type: u32,
        /** The function. The embedder must know the actual C signature of the
         * function and cast to it before calling. */
        func: wasm_rt_funcref_t,
    }

    /** A Table object. */
    pub struct wasm_rt_table_t {
        /** The table element data, with an element count of `size`. */
        data: *mut wasm_rt_elem_t,
        /** The maximum element count of this Table object. If there is no maximum,
         * `max_size` is 0xffffffffu (i.e. UINT32_MAX). */
        max_size: u32,
        /** The current element count of the table. */
        size: u32,
    }

    #[link(name = "client")]
    extern "C" {
        #[link_name = "Z_my_game_init"]
        pub fn init();

        #[link_name = "Z_my_game_free"]
        pub fn free();

        #[link_name = "Z_my_gameZ_memory"]
        pub static memory: *mut wasm_rt_memory_t;
        #[link_name = "Z_my_gameZ___indirect_function_table"]
        pub static function_table: *mut wasm_rt_table_t;
    }
}

pub struct Address(u32);

impl From<u32> for Address {
    fn from(val: u32) -> Self {
        Self(val)
    }
}

pub struct Memory(*mut raw::wasm_rt_memory_t);

impl Memory {
    pub fn from_raw() -> Self {
        Self(raw::memory)
    }

    fn get(&self, addr: impl Into<Address>) -> *mut u8 {
        let addr = addr.into();

        unsafe { (*self.0).data.add(addr.0 as _) }
    }

    pub fn ptr<T>(&self, addr: impl Into<Address>) -> Pointer<T> {
        unsafe { mem::transmute(self.get(addr)) }
    }

    pub fn ptr_mut<T>(&self, addr: impl Into<Address>) -> PointerMut<T> {
        unsafe { mem::transmute(self.get(addr)) }
    }

    pub fn r#ref<T>(&self, addr: impl Into<Address>) -> Ref<T> {
        unsafe { mem::transmute(self.get(addr)) }
    }

    pub fn ref_mut<T>(&self, addr: impl Into<Address>) -> RefMut<T> {
        unsafe { mem::transmute(self.get(addr)) }
    }

    pub fn global<T>(&self, addr: impl Into<Address>) -> Global<T> {
        unsafe { mem::transmute(self.get(addr)) }
    }

    pub fn global_mut<T>(&self, addr: impl Into<Address>) -> GlobalMut<T> {
        unsafe { mem::transmute(self.get(addr)) }
    }
}

pub struct FunctionTable(*mut raw::wasm_rt_table_t);

impl FunctionTable {
    pub fn from_raw() -> Self {
        Self(raw::function_table)
    }
}
