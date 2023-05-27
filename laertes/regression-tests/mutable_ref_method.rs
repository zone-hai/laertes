struct Bar {
    arr: [i32; 10],
}

struct Foo {
    bar: Bar,
}

unsafe fn test_foo(mut foo: * mut Foo) -> * mut i32 {
    return (*foo).bar.arr.as_mut_ptr().offset(1);
}
