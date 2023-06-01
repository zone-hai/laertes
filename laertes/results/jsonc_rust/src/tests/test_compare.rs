use ::libc;
extern "C" {
    
    fn printf(_: * const i8, _: ...) -> i32;
    
    
    
    
    
    
    
    
    
    
}
pub use crate::src::json_object::json_object_array_add;
pub use crate::src::json_object::json_object_equal;
pub use crate::src::json_object::json_object_new_array;
pub use crate::src::json_object::json_object_new_double;
pub use crate::src::json_object::json_object_new_int;
pub use crate::src::json_object::json_object_new_object;
pub use crate::src::json_object::json_object_new_string;
pub use crate::src::json_object::json_object_new_uint64;
pub use crate::src::json_object::json_object_object_add;
pub use crate::src::json_object::json_object_put;
pub use crate::src::json_object::json_object;
pub type __int32_t = i32;
pub type __uint64_t = u64;
pub type int32_t = i32;
pub type uint64_t = u64;
unsafe fn main_0(
    mut argc: i32,
    mut argv: * mut * mut i8,
) -> i32 {
    let mut int1: * mut crate::src::json_object::json_object = json_object_new_int(0 as i32);
    let mut int2: * mut crate::src::json_object::json_object = json_object_new_int(1 as i32);
    let mut int3: * mut crate::src::json_object::json_object = json_object_new_int(1 as i32);
    let mut int4: * mut crate::src::json_object::json_object = json_object_new_int(-(1 as i32));
    let mut uint1: * mut crate::src::json_object::json_object = json_object_new_uint64(
        0 as i32 as uint64_t,
    );
    let mut uint2: * mut crate::src::json_object::json_object = json_object_new_uint64(
        1 as i32 as uint64_t,
    );
    let mut uint3: * mut crate::src::json_object::json_object = json_object_new_uint64(
        1 as i32 as uint64_t,
    );
    let mut uint4: * mut crate::src::json_object::json_object = json_object_new_uint64(
        (9223372036854775807 as i64 as uint64_t)
            .wrapping_add(100 as i32 as u64),
    );
    if json_object_equal(int1, int2) == 0 {
        printf(
            b"JSON integer comparison is correct\n\0" as *const u8 as *const i8,
        );
    } else {
        printf(
            b"JSON integer comparison failed\n\0" as *const u8 as *const i8,
        );
    }
    if json_object_equal(int1, int1) != 0 {
        printf(
            b"JSON same object comparison is correct\n\0" as *const u8
                as *const i8,
        );
    } else {
        printf(
            b"JSON same object comparison failed\n\0" as *const u8 as *const i8,
        );
    }
    if json_object_equal(int2, int3) != 0 {
        printf(
            b"JSON same integer comparison is correct\n\0" as *const u8
                as *const i8,
        );
    } else {
        printf(
            b"JSON same integer comparison failed\n\0" as *const u8
                as *const i8,
        );
    }
    if json_object_equal(uint1, uint2) == 0 {
        printf(
            b"JSON usigned integer comparison is correct\n\0" as *const u8
                as *const i8,
        );
    } else {
        printf(
            b"JSON usigned integer comparison failed\n\0" as *const u8
                as *const i8,
        );
    }
    if json_object_equal(uint1, uint1) != 0 {
        printf(
            b"JSON same usigned object comparison is correct\n\0" as *const u8
                as *const i8,
        );
    } else {
        printf(
            b"JSON same usigned object comparison failed\n\0" as *const u8
                as *const i8,
        );
    }
    if json_object_equal(uint2, uint3) != 0 {
        printf(
            b"JSON same usigned integer comparison is correct\n\0" as *const u8
                as *const i8,
        );
    } else {
        printf(
            b"JSON same usigned integer comparison failed\n\0" as *const u8
                as *const i8,
        );
    }
    if json_object_equal(int2, uint2) != 0 {
        printf(
            b"JSON integer & usigned integer comparison is correct\n\0" as *const u8
                as *const i8,
        );
    } else {
        printf(
            b"JSON integer & usigned integer comparison failed\n\0" as *const u8
                as *const i8,
        );
    }
    if json_object_equal(int2, uint4) == 0 {
        printf(
            b"JSON integer & usigned integer comparison is correct\n\0" as *const u8
                as *const i8,
        );
    } else {
        printf(
            b"JSON integer & usigned integer comparison failed\n\0" as *const u8
                as *const i8,
        );
    }
    if json_object_equal(int4, uint2) == 0 {
        printf(
            b"JSON integer & usigned integer comparison is correct\n\0" as *const u8
                as *const i8,
        );
    } else {
        printf(
            b"JSON integer & usigned integer comparison failed\n\0" as *const u8
                as *const i8,
        );
    }
    if json_object_equal(int4, uint4) == 0 {
        printf(
            b"JSON integer & usigned integer comparison is correct\n\0" as *const u8
                as *const i8,
        );
    } else {
        printf(
            b"JSON integer & usigned integer comparison failed\n\0" as *const u8
                as *const i8,
        );
    }
    if json_object_equal(uint2, int2) != 0 {
        printf(
            b"JSON usigned integer & integer comparison is correct\n\0" as *const u8
                as *const i8,
        );
    } else {
        printf(
            b"JSON usigned integer & integer comparison failed\n\0" as *const u8
                as *const i8,
        );
    }
    if json_object_equal(uint2, int4) == 0 {
        printf(
            b"JSON usigned integer & integer comparison is correct\n\0" as *const u8
                as *const i8,
        );
    } else {
        printf(
            b"JSON usigned integer & integer comparison failed\n\0" as *const u8
                as *const i8,
        );
    }
    if json_object_equal(uint4, int2) == 0 {
        printf(
            b"JSON usigned integer & integer comparison is correct\n\0" as *const u8
                as *const i8,
        );
    } else {
        printf(
            b"JSON usigned integer & integer comparison failed\n\0" as *const u8
                as *const i8,
        );
    }
    if json_object_equal(uint4, int4) == 0 {
        printf(
            b"JSON usigned integer & integer comparison is correct\n\0" as *const u8
                as *const i8,
        );
    } else {
        printf(
            b"JSON usigned integer & integer comparison failed\n\0" as *const u8
                as *const i8,
        );
    }
    json_object_put(int1);
    json_object_put(int2);
    json_object_put(int3);
    json_object_put(int4);
    json_object_put(uint1);
    json_object_put(uint2);
    json_object_put(uint3);
    json_object_put(uint4);
    let mut str1: * mut crate::src::json_object::json_object = json_object_new_string(
        b"TESTSTRING\0" as *const u8 as *const i8,
    );
    let mut str2: * mut crate::src::json_object::json_object = json_object_new_string(
        b"TESTSTRING\0" as *const u8 as *const i8,
    );
    let mut str3: * mut crate::src::json_object::json_object = json_object_new_string(
        b"DIFFERENT\0" as *const u8 as *const i8,
    );
    if json_object_equal(str1, str2) != 0 {
        printf(
            b"Comparing equal strings is correct\n\0" as *const u8 as *const i8,
        );
    } else {
        printf(
            b"Comparing equal strings failed\n\0" as *const u8 as *const i8,
        );
    }
    if json_object_equal(str1, str3) == 0 {
        printf(
            b"Comparing different strings is correct\n\0" as *const u8
                as *const i8,
        );
    } else {
        printf(
            b"Comparing different strings failed\n\0" as *const u8 as *const i8,
        );
    }
    json_object_put(str1);
    json_object_put(str2);
    json_object_put(str3);
    let mut dbl1: * mut crate::src::json_object::json_object = json_object_new_double(3.14159f64);
    let mut dbl2: * mut crate::src::json_object::json_object = json_object_new_double(3.14159f64);
    let mut dbl3: * mut crate::src::json_object::json_object = json_object_new_double(3.0f64);
    if json_object_equal(dbl1, dbl2) != 0 {
        printf(
            b"Comparing equal doubles is correct\n\0" as *const u8 as *const i8,
        );
    } else {
        printf(
            b"Comparing equal doubles failed\n\0" as *const u8 as *const i8,
        );
    }
    if json_object_equal(dbl1, dbl3) == 0 {
        printf(
            b"Comparing different doubles is correct\n\0" as *const u8
                as *const i8,
        );
    } else {
        printf(
            b"Comparing different doubles failed\n\0" as *const u8 as *const i8,
        );
    }
    json_object_put(dbl1);
    json_object_put(dbl2);
    json_object_put(dbl3);
    let mut ar1: * mut crate::src::json_object::json_object = json_object_new_array();
    let mut ar2: * mut crate::src::json_object::json_object = json_object_new_array();
    let mut ar3: * mut crate::src::json_object::json_object = json_object_new_array();
    let mut ar4: * mut crate::src::json_object::json_object = json_object_new_array();
    json_object_array_add(ar1, json_object_new_int(1 as i32));
    json_object_array_add(ar1, json_object_new_int(2 as i32));
    json_object_array_add(ar2, json_object_new_int(1 as i32));
    json_object_array_add(ar2, json_object_new_int(2 as i32));
    json_object_array_add(ar3, json_object_new_int(1 as i32));
    json_object_array_add(ar3, json_object_new_int(1 as i32));
    if json_object_equal(ar1, ar2) != 0 {
        printf(
            b"Comparing equal arrays is correct\n\0" as *const u8 as *const i8,
        );
    } else {
        printf(b"Comparing equal arrays failed\n\0" as *const u8 as *const i8);
    }
    json_object_array_add(ar2, json_object_new_int(1 as i32));
    if json_object_equal(ar1, ar2) == 0 {
        printf(
            b"Comparing arrays of different len is correct\n\0" as *const u8
                as *const i8,
        );
    } else {
        printf(
            b"Comparing arrays of different len failed\n\0" as *const u8
                as *const i8,
        );
    }
    if json_object_equal(ar1, ar3) == 0 {
        printf(
            b"Comparing different arrays is correct\n\0" as *const u8
                as *const i8,
        );
    } else {
        printf(
            b"Comparing different arrays failed\n\0" as *const u8 as *const i8,
        );
    }
    if json_object_equal(ar1, ar4) == 0 {
        printf(
            b"Comparing different arrays (one empty) is correct\n\0" as *const u8
                as *const i8,
        );
    } else {
        printf(
            b"Comparing different arrays (one empty) failed\n\0" as *const u8
                as *const i8,
        );
    }
    json_object_put(ar1);
    json_object_put(ar2);
    json_object_put(ar3);
    json_object_put(ar4);
    let mut obj1: * mut crate::src::json_object::json_object = json_object_new_object();
    let mut obj2: * mut crate::src::json_object::json_object = json_object_new_object();
    json_object_object_add(
        obj1,
        b"test1\0" as *const u8 as *const i8,
        json_object_new_int(123 as i32),
    );
    json_object_object_add(
        obj1,
        b"test2\0" as *const u8 as *const i8,
        json_object_new_int(321 as i32),
    );
    json_object_object_add(
        obj1,
        b"test3\0" as *const u8 as *const i8,
        json_object_new_int(320 as i32),
    );
    json_object_object_add(
        obj1,
        b"test4\0" as *const u8 as *const i8,
        json_object_new_int(319 as i32),
    );
    json_object_object_add(
        obj1,
        b"test5\0" as *const u8 as *const i8,
        json_object_new_int(318 as i32),
    );
    json_object_object_add(
        obj2,
        b"test5\0" as *const u8 as *const i8,
        json_object_new_int(318 as i32),
    );
    json_object_object_add(
        obj2,
        b"test4\0" as *const u8 as *const i8,
        json_object_new_int(319 as i32),
    );
    json_object_object_add(
        obj2,
        b"test3\0" as *const u8 as *const i8,
        json_object_new_int(320 as i32),
    );
    json_object_object_add(
        obj2,
        b"test2\0" as *const u8 as *const i8,
        json_object_new_int(321 as i32),
    );
    json_object_object_add(
        obj2,
        b"test1\0" as *const u8 as *const i8,
        json_object_new_int(123 as i32),
    );
    if json_object_equal(obj1, obj2) != 0 {
        printf(
            b"Comparing JSON object with different key order is correct\n\0" as *const u8
                as *const i8,
        );
    } else {
        printf(
            b"Comparing JSON object with different key order is incorrect\n\0"
                as *const u8 as *const i8,
        );
    }
    json_object_object_add(
        obj2,
        b"test3\0" as *const u8 as *const i8,
        json_object_new_int(234 as i32),
    );
    if json_object_equal(obj1, obj2) == 0 {
        printf(
            b"Comparing different objects is correct\n\0" as *const u8
                as *const i8,
        );
    } else {
        printf(
            b"Comparing different objects is incorrect\n\0" as *const u8
                as *const i8,
        );
    }
    json_object_object_add(
        obj2,
        b"test3\0" as *const u8 as *const i8,
        json_object_new_int(320 as i32),
    );
    json_object_object_add(
        obj2,
        b"test6\0" as *const u8 as *const i8,
        json_object_new_int(321 as i32),
    );
    if json_object_equal(obj1, obj2) == 0 {
        printf(
            b"Comparing different objects is correct\n\0" as *const u8
                as *const i8,
        );
    } else {
        printf(
            b"Comparing different objects is incorrect\n\0" as *const u8
                as *const i8,
        );
    }
    json_object_object_add(
        obj1,
        b"test6\0" as *const u8 as *const i8,
        json_object_new_int(321 as i32),
    );
    if json_object_equal(obj1, obj2) != 0 {
        printf(
            b"Comparing different objects is correct\n\0" as *const u8
                as *const i8,
        );
    } else {
        printf(
            b"Comparing different objects is incorrect\n\0" as *const u8
                as *const i8,
        );
    }
    json_object_object_add(
        obj1,
        b"test7\0" as *const u8 as *const i8,
        json_object_new_int(322 as i32),
    );
    if json_object_equal(obj1, obj2) == 0 {
        printf(
            b"Comparing different objects is correct\n\0" as *const u8
                as *const i8,
        );
    } else {
        printf(
            b"Comparing different objects is incorrect\n\0" as *const u8
                as *const i8,
        );
    }
    json_object_put(obj1);
    json_object_put(obj2);
    let mut int5: * mut crate::src::json_object::json_object = json_object_new_int(0 as i32);
    let mut dbl5: * mut crate::src::json_object::json_object = json_object_new_double(3.14159f64);
    if json_object_equal(int5, 0 as *mut json_object) == 0 {
        printf(
            b"JSON integer and NULL comparison is correct\n\0" as *const u8
                as *const i8,
        );
    } else {
        printf(
            b"JSON integer and NULL comparison failed\n\0" as *const u8
                as *const i8,
        );
    }
    if json_object_equal(0 as *mut json_object, dbl5) == 0 {
        printf(
            b"JSON NULL and double comparison is correct\n\0" as *const u8
                as *const i8,
        );
    } else {
        printf(
            b"JSON NULL and double comparison failed\n\0" as *const u8
                as *const i8,
        );
    }
    if json_object_equal(int5, dbl5) == 0 {
        printf(
            b"JSON integer and double comparison is correct\n\0" as *const u8
                as *const i8,
        );
    } else {
        printf(
            b"JSON integer and double comparison failed\n\0" as *const u8
                as *const i8,
        );
    }
    json_object_put(int5);
    json_object_put(dbl5);
    return 0 as i32;
}
pub fn main() {
    let mut args: Vec::<*mut i8> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as i32,
                args.as_mut_ptr() as *mut *mut i8,
            ) as i32,
        )
    }
}
use crate::laertes_rt::*;
