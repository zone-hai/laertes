use ::libc;
extern "C" {
    
    
    
    
    fn printf(_: * const i8, _: ...) -> i32;
}
pub use crate::src::json_object::json_object_new_double;
pub use crate::src::json_object::json_object_put;
pub use crate::src::json_object::json_object_to_json_string_ext;
pub use crate::src::json_object::json_object;
unsafe fn main_0() -> i32 {
    let mut json: * mut crate::src::json_object::json_object = 0 as *mut json_object;
    json = json_object_new_double(1.0f64);
    printf(
        b"json = %s\n\0" as *const u8 as *const i8,
        json_object_to_json_string_ext(json, (1 as i32) << 1 as i32),
    );
    json_object_put(json);
    json = json_object_new_double(-1.0f64);
    printf(
        b"json = %s\n\0" as *const u8 as *const i8,
        json_object_to_json_string_ext(json, (1 as i32) << 1 as i32),
    );
    json_object_put(json);
    json = json_object_new_double(1.23f64);
    printf(
        b"json = %s\n\0" as *const u8 as *const i8,
        json_object_to_json_string_ext(json, (1 as i32) << 1 as i32),
    );
    json_object_put(json);
    json = json_object_new_double(123456789.0f64);
    printf(
        b"json = %s\n\0" as *const u8 as *const i8,
        json_object_to_json_string_ext(json, (1 as i32) << 1 as i32),
    );
    json_object_put(json);
    json = json_object_new_double(123456789.123f64);
    printf(
        b"json = %s\n\0" as *const u8 as *const i8,
        json_object_to_json_string_ext(json, (1 as i32) << 1 as i32),
    );
    json_object_put(json);
    return 0 as i32;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
use crate::laertes_rt::*;
