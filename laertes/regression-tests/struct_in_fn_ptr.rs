// This struct should not be rewritten because it occurs in a function
// pointer in another struct.
struct Poison1 {
    p: * mut i32,
}

struct Bar {
    f: Option<unsafe extern "C" fn () -> * mut * mut Poison1>,
}

// This struct should not be rewritten because it occurs in a fn
// pointer in another function's signature.
struct Poison2 {
    p: * mut i32,
}

fn test_fn(f: Option<unsafe extern "C" fn () -> * mut * mut Poison2>) {
}

// This type can be rewritten to insert lifetimes for Safe
type SafeP = * const Safe;

struct Safe {
    p: * mut i32,
}

// This type cannot be rewritten to insert lifetimes because it
// contains a function pointer.
type UnsafeFP = Option<unsafe extern "C"  fn() -> * const UnsafeP>;

type UnsafeP = * const crate::Unsafe;

struct Unsafe {
    p: * mut i32,
}

// Here, we have a struct used in an external API, it mentions another
// type in an fn pointer, so that other struct cannot be rewritten either.
struct UsedInExtern {
    x: i32,
    f: IntermediaryP,
}

type IntermediaryP = * const Intermediary;

struct Intermediary {
    p: IndirectlyUsedP,
}

type IndirectlyUsedP = * const IndirectlyUsedInExtern;

struct IndirectlyUsedInExtern {
    should_be_raw: * const i32,
}

type UsedInExternP = * const UsedInExtern;

extern "C" {
    fn some_extern_fn() -> UsedInExternP;
}
