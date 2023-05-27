fn foo(p: * mut * const i32) {
}

unsafe fn bar(q: * mut i32) {
    foo(&mut q.offset(1) as * mut * mut i32 as * mut * const i32);
}
