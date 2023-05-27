// Example with ref-mut, in this case connect the rhs with the pts-to set of the lhs

static mut foo : [* mut i32; 1] = [ core::ptr::null_mut() ];

unsafe fn bar(baz: * mut * mut i32) {
    let ref mut fresh = *baz.offset(0);
    *fresh = foo[0];
}
