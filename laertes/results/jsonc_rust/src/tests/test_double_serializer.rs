use ::libc;
extern "C" {
    fn printf(_: * const i8, _: ...) -> i32;
    
    
    
    
    
    
    
}
pub use crate::src::json_object::json_c_set_serialization_double_format;
pub use crate::src::json_object::json_object_double_to_json_string;
pub use crate::src::json_object::json_object_new_double;
pub use crate::src::json_object::json_object_put;
pub use crate::src::json_object::json_object_set_serializer;
pub use crate::src::json_object::json_object_to_json_string;
pub use crate::src::json_object::json_object_to_json_string_ext;
pub type __uint32_t = u32;
pub type uint32_t = u32;
// #[derive(Copy, Clone)]

pub type printbuf = crate::src::apps::json_parse::printbuf;
// #[derive(Copy, Clone)]

pub type json_object = crate::src::json_object::json_object;
pub type json_object_delete_fn<'a1, 'a2> = unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::json_object::json_object>,_: Option<&'a2 mut core::ffi::c_void>,) -> ();
pub type json_object_to_json_string_fn<'a1, 'a2> = unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::json_object::json_object>,_: Option<&'a2 mut crate::src::apps::json_parse::printbuf>,_: i32,_: i32,) -> i32;
pub type json_type = u32;
pub const json_type_string: json_type = 6;
pub const json_type_array: json_type = 5;
pub const json_type_object: json_type = 4;
pub const json_type_int: json_type = 3;
pub const json_type_double: json_type = 2;
pub const json_type_boolean: json_type = 1;
pub const json_type_null: json_type = 0;
#[no_mangle]
pub static mut zero_dot_zero: f64 = 0.0f64;
unsafe fn main_0(
    mut argc: i32,
    mut argv: * mut * mut i8,
) -> i32 {
    let mut obj: * mut crate::src::json_object::json_object = json_object_new_double(0.5f64);
    let mut udata: [i8; 5] = *core::intrinsics::transmute::<&'_ [u8; 5], &'_ mut [i8; 5]>(b"test\0");
    printf(b"Test default serializer:\n\0" as *const u8 as *const i8);
    printf(
        b"obj.to_string(standard)=%s\n\0" as *const u8 as *const i8,
        json_object_to_json_string(obj),
    );
    printf(
        b"Test default serializer with custom userdata:\n\0" as *const u8
            as *const i8,
    );
    let mut fresh0 = &mut ((*obj)._userdata);
    *fresh0 = udata.as_mut_ptr() as *mut libc::c_void;
    printf(
        b"obj.to_string(userdata)=%s\n\0" as *const u8 as *const i8,
        json_object_to_json_string(obj),
    );
    printf(
        b"Test explicit serializer with custom userdata:\n\0" as *const u8
            as *const i8,
    );
    json_object_set_serializer(
        obj,
        Some(
            json_object_double_to_json_string,
        ),
        udata.as_mut_ptr() as *mut libc::c_void,
        None,
    );
    printf(
        b"obj.to_string(custom)=%s\n\0" as *const u8 as *const i8,
        json_object_to_json_string(obj),
    );
    printf(b"Test reset serializer:\n\0" as *const u8 as *const i8);
    json_object_set_serializer(obj, None, 0 as *mut libc::c_void, None);
    printf(
        b"obj.to_string(reset)=%s\n\0" as *const u8 as *const i8,
        json_object_to_json_string(obj),
    );
    json_object_put(obj);
    printf(b"Test no zero reset serializer:\n\0" as *const u8 as *const i8);
    obj = json_object_new_double(3.1415000f64);
    let mut data: [i8; 6] = *core::intrinsics::transmute::<&'_ [u8; 6], &'_ mut [i8; 6]>(b"%.17g\0");
    json_object_set_serializer(
        obj,
        Some(
            json_object_double_to_json_string,
        ),
        data.as_mut_ptr() as *mut libc::c_void,
        None,
    );
    printf(
        b"obj.to_string(reset)=%s\n\0" as *const u8 as *const i8,
        json_object_to_json_string_ext(obj, 4 as i32),
    );
    json_object_put(obj);
    obj = json_object_new_double(0.52381f64);
    printf(
        b"obj.to_string(default format)=%s\n\0" as *const u8 as *const i8,
        json_object_to_json_string(obj),
    );
    if json_c_set_serialization_double_format(
        b"x%0.3fy\0" as *const u8 as *const i8,
        0 as i32,
    ) < 0 as i32
    {
        printf(
            b"ERROR: json_c_set_serialization_double_format() failed\0" as *const u8
                as *const i8,
        );
    }
    printf(
        b"obj.to_string(with global format)=%s\n\0" as *const u8 as *const i8,
        json_object_to_json_string(obj),
    );
    if json_c_set_serialization_double_format(
        b"T%0.2fX\0" as *const u8 as *const i8,
        1 as i32,
    ) < 0 as i32
    {
        printf(
            b"ERROR: json_c_set_serialization_double_format() failed\0" as *const u8
                as *const i8,
        );
    }
    printf(
        b"obj.to_string(with thread format)=%s\n\0" as *const u8 as *const i8,
        json_object_to_json_string(obj),
    );
    if json_c_set_serialization_double_format(
        b"Ttttttttttttt%0.2fxxxxxxxxxxxxxxxxxxX\0" as *const u8 as *const i8,
        1 as i32,
    ) < 0 as i32
    {
        printf(
            b"ERROR: json_c_set_serialization_double_format() failed\0" as *const u8
                as *const i8,
        );
    }
    printf(
        b"obj.to_string(long thread format)=%s\n\0" as *const u8 as *const i8,
        json_object_to_json_string(obj),
    );
    if json_c_set_serialization_double_format(0 as *const i8, 1 as i32)
        < 0 as i32
    {
        printf(
            b"ERROR: json_c_set_serialization_double_format() failed\0" as *const u8
                as *const i8,
        );
    }
    printf(
        b"obj.to_string(back to global format)=%s\n\0" as *const u8
            as *const i8,
        json_object_to_json_string(obj),
    );
    if json_c_set_serialization_double_format(0 as *const i8, 0 as i32)
        < 0 as i32
    {
        printf(
            b"ERROR: json_c_set_serialization_double_format() failed\0" as *const u8
                as *const i8,
        );
    }
    printf(
        b"obj.to_string(back to default format)=%s\n\0" as *const u8
            as *const i8,
        json_object_to_json_string(obj),
    );
    json_object_put(obj);
    obj = json_object_new_double(12.0f64);
    printf(
        b"obj(12.0).to_string(default format)=%s\n\0" as *const u8
            as *const i8,
        json_object_to_json_string(obj),
    );
    if json_c_set_serialization_double_format(
        b"%.0f\0" as *const u8 as *const i8,
        0 as i32,
    ) < 0 as i32
    {
        printf(
            b"ERROR: json_c_set_serialization_double_format() failed\0" as *const u8
                as *const i8,
        );
    }
    printf(
        b"obj(12.0).to_string(%%.0f)=%s\n\0" as *const u8 as *const i8,
        json_object_to_json_string(obj),
    );
    if json_c_set_serialization_double_format(
        b"%.0g\0" as *const u8 as *const i8,
        0 as i32,
    ) < 0 as i32
    {
        printf(
            b"ERROR: json_c_set_serialization_double_format() failed\0" as *const u8
                as *const i8,
        );
    }
    printf(
        b"obj(12.0).to_string(%%.0g)=%s\n\0" as *const u8 as *const i8,
        json_object_to_json_string(obj),
    );
    if json_c_set_serialization_double_format(
        b"%.2g\0" as *const u8 as *const i8,
        0 as i32,
    ) < 0 as i32
    {
        printf(
            b"ERROR: json_c_set_serialization_double_format() failed\0" as *const u8
                as *const i8,
        );
    }
    printf(
        b"obj(12.0).to_string(%%.1g)=%s\n\0" as *const u8 as *const i8,
        json_object_to_json_string(obj),
    );
    if json_c_set_serialization_double_format(0 as *const i8, 0 as i32)
        < 0 as i32
    {
        printf(
            b"ERROR: json_c_set_serialization_double_format() failed\0" as *const u8
                as *const i8,
        );
    }
    json_object_put(obj);
    obj = json_object_new_double(-12.0f64);
    printf(
        b"obj(-12.0).to_string(default format)=%s\n\0" as *const u8
            as *const i8,
        json_object_to_json_string(obj),
    );
    json_object_put(obj);
    obj = json_object_new_double(zero_dot_zero / zero_dot_zero);
    printf(
        b"obj(0.0/0.0)=%s\n\0" as *const u8 as *const i8,
        json_object_to_json_string(obj),
    );
    json_object_put(obj);
    obj = json_object_new_double(1.0f64 / zero_dot_zero);
    printf(
        b"obj(1.0/0.0)=%s\n\0" as *const u8 as *const i8,
        json_object_to_json_string(obj),
    );
    json_object_put(obj);
    obj = json_object_new_double(-1.0f64 / zero_dot_zero);
    printf(
        b"obj(-1.0/0.0)=%s\n\0" as *const u8 as *const i8,
        json_object_to_json_string(obj),
    );
    json_object_put(obj);
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
