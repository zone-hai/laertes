extern "C" {
    fn malloc(_: usize) -> *mut std::os::raw::c_void;
}

unsafe fn my_malloc() -> * mut i32 {
    return malloc(std::mem::size_of::<i32>()) as * mut std::os::raw::c_void as * mut i32
}

unsafe fn foo(mut p: * mut * mut i32) {
    *p = my_malloc();
}

unsafe fn main_0() {
    let mut x = 0 as * mut i32;
    foo(&mut x);
}

pub fn main() {
    unsafe {
        main_0();
    }
}
