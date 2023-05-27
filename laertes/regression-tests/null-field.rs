pub struct Foo {
    // create a pointer field
    bar: * const i32,
}

pub unsafe fn baz(mut foo: Foo) {
    // use pointer arithmetic to taint the field
    foo.bar = foo.bar.offset(1);
}

pub fn quux() -> Foo {
    // initialize the pointer field to `NULL` from C
    return Foo {
        bar: 0 as * const i32,
    };
}
