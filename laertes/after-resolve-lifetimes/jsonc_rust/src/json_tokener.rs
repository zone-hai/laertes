use :: libc;
extern "C" {
    pub type __locale_data;
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    fn __errno_location() -> *mut i32;
    fn strtod(_: *const i8, _: *mut *mut i8) -> f64;
    fn calloc(_: u64, _: u64) -> *mut core::ffi::c_void;
    fn free(__ptr: *mut core::ffi::c_void);
    fn memcpy(
        _: *mut core::ffi::c_void,
        _: *const core::ffi::c_void,
        _: u64,
    ) -> *mut core::ffi::c_void;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn strdup(_: *const i8) -> *mut i8;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn strncasecmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn newlocale(
        __category_mask: i32,
        __locale: *const i8,
        __base: *mut crate::src::json_tokener::__locale_struct,
    ) -> *mut crate::src::json_tokener::__locale_struct;
    fn freelocale(__dataset: *mut crate::src::json_tokener::__locale_struct);
    fn duplocale(
        __dataset: *mut crate::src::json_tokener::__locale_struct,
    ) -> *mut crate::src::json_tokener::__locale_struct;
    fn uselocale(
        __dataset: *mut crate::src::json_tokener::__locale_struct,
    ) -> *mut crate::src::json_tokener::__locale_struct;
}
pub use crate::src::{
    debug::mc_debug,
    json_object::{
        json_object_array_add, json_object_array_shrink, json_object_get, json_object_new_array,
        json_object_new_boolean, json_object_new_double, json_object_new_double_s,
        json_object_new_int64, json_object_new_object, json_object_new_string_len,
        json_object_new_uint64, json_object_object_add, json_object_put,
    },
    json_util::{json_parse_int64, json_parse_uint64},
    printbuf::{printbuf_free, printbuf_memappend, printbuf_new, printbuf_reset},
};
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type size_t = u64;
#[repr(C)]
pub struct __locale_struct {
    pub __locales: [*mut crate::src::json_tokener::__locale_data; 13],
    pub __ctype_b: *const u16,
    pub __ctype_tolower: *const i32,
    pub __ctype_toupper: *const i32,
    pub __names: [*const i8; 13],
}
impl __locale_struct {
    pub const fn new() -> Self {
        __locale_struct {
            __locales: [
                (0 as *mut crate::src::json_tokener::__locale_data),
                (0 as *mut crate::src::json_tokener::__locale_data),
                (0 as *mut crate::src::json_tokener::__locale_data),
                (0 as *mut crate::src::json_tokener::__locale_data),
                (0 as *mut crate::src::json_tokener::__locale_data),
                (0 as *mut crate::src::json_tokener::__locale_data),
                (0 as *mut crate::src::json_tokener::__locale_data),
                (0 as *mut crate::src::json_tokener::__locale_data),
                (0 as *mut crate::src::json_tokener::__locale_data),
                (0 as *mut crate::src::json_tokener::__locale_data),
                (0 as *mut crate::src::json_tokener::__locale_data),
                (0 as *mut crate::src::json_tokener::__locale_data),
                (0 as *mut crate::src::json_tokener::__locale_data),
            ],
            __ctype_b: (0 as *const u16),
            __ctype_tolower: (0 as *const i32),
            __ctype_toupper: (0 as *const i32),
            __names: [
                (0 as *const i8),
                (0 as *const i8),
                (0 as *const i8),
                (0 as *const i8),
                (0 as *const i8),
                (0 as *const i8),
                (0 as *const i8),
                (0 as *const i8),
                (0 as *const i8),
                (0 as *const i8),
                (0 as *const i8),
                (0 as *const i8),
                (0 as *const i8),
            ],
        }
    }
}
impl std::default::Default for __locale_struct {
    fn default() -> Self {
        __locale_struct::new()
    }
}
pub type __locale_t = *mut crate::src::json_tokener::__locale_struct;
pub type locale_t = *mut crate::src::json_tokener::__locale_struct;
pub type int64_t = i64;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type printbuf = crate::src::apps::json_parse::printbuf;
pub type json_object = crate::src::json_object::json_object;
pub type json_object_delete_fn = unsafe extern "C" fn(
    _: *mut crate::src::json_object::json_object,
    _: *mut core::ffi::c_void,
) -> ();
pub type json_object_to_json_string_fn = unsafe extern "C" fn(
    _: *mut crate::src::json_object::json_object,
    _: *mut crate::src::apps::json_parse::printbuf,
    _: i32,
    _: i32,
) -> i32;
pub type json_type = u32;
pub const json_type_string: json_type = 6;
pub const json_type_array: json_type = 5;
pub const json_type_object: json_type = 4;
pub const json_type_int: json_type = 3;
pub const json_type_double: json_type = 2;
pub const json_type_boolean: json_type = 1;
pub const json_type_null: json_type = 0;
pub type json_bool = i32;
pub type json_tokener_error = u32;
pub const json_tokener_error_size: json_tokener_error = 15;
pub const json_tokener_error_parse_utf8_string: json_tokener_error = 14;
pub const json_tokener_error_parse_comment: json_tokener_error = 13;
pub const json_tokener_error_parse_string: json_tokener_error = 12;
pub const json_tokener_error_parse_object_value_sep: json_tokener_error = 11;
pub const json_tokener_error_parse_object_key_sep: json_tokener_error = 10;
pub const json_tokener_error_parse_object_key_name: json_tokener_error = 9;
pub const json_tokener_error_parse_array: json_tokener_error = 8;
pub const json_tokener_error_parse_number: json_tokener_error = 7;
pub const json_tokener_error_parse_boolean: json_tokener_error = 6;
pub const json_tokener_error_parse_null: json_tokener_error = 5;
pub const json_tokener_error_parse_unexpected: json_tokener_error = 4;
pub const json_tokener_error_parse_eof: json_tokener_error = 3;
pub const json_tokener_error_depth: json_tokener_error = 2;
pub const json_tokener_continue: json_tokener_error = 1;
pub const json_tokener_success: json_tokener_error = 0;
pub type json_tokener_state = u32;
pub const json_tokener_state_inf: json_tokener_state = 26;
pub const json_tokener_state_object_field_start_after_sep: json_tokener_state = 25;
pub const json_tokener_state_array_after_sep: json_tokener_state = 24;
pub const json_tokener_state_object_sep: json_tokener_state = 23;
pub const json_tokener_state_object_value_add: json_tokener_state = 22;
pub const json_tokener_state_object_value: json_tokener_state = 21;
pub const json_tokener_state_object_field_end: json_tokener_state = 20;
pub const json_tokener_state_object_field: json_tokener_state = 19;
pub const json_tokener_state_object_field_start: json_tokener_state = 18;
pub const json_tokener_state_array_sep: json_tokener_state = 17;
pub const json_tokener_state_array_add: json_tokener_state = 16;
pub const json_tokener_state_array: json_tokener_state = 15;
pub const json_tokener_state_number: json_tokener_state = 14;
pub const json_tokener_state_boolean: json_tokener_state = 13;
pub const json_tokener_state_escape_unicode_need_u: json_tokener_state = 12;
pub const json_tokener_state_escape_unicode_need_escape: json_tokener_state = 11;
pub const json_tokener_state_escape_unicode: json_tokener_state = 10;
pub const json_tokener_state_string_escape: json_tokener_state = 9;
pub const json_tokener_state_string: json_tokener_state = 8;
pub const json_tokener_state_comment_end: json_tokener_state = 7;
pub const json_tokener_state_comment_eol: json_tokener_state = 6;
pub const json_tokener_state_comment: json_tokener_state = 5;
pub const json_tokener_state_comment_start: json_tokener_state = 4;
pub const json_tokener_state_null: json_tokener_state = 3;
pub const json_tokener_state_finish: json_tokener_state = 2;
pub const json_tokener_state_start: json_tokener_state = 1;
pub const json_tokener_state_eatws: json_tokener_state = 0;
pub type json_tokener_srec<'a> = crate::src::apps::json_parse::json_tokener_srec<'a>;
pub type json_tokener<'a> = crate::src::apps::json_parse::json_tokener<'a>;
#[inline]
extern "C" fn is_ws_char(mut c: i8) -> i32 {
    return (c as i32 == ' ' as i32
        || c as i32 == '\t' as i32
        || c as i32 == '\n' as i32
        || c as i32 == '\r' as i32) as i32;
}
#[inline]
extern "C" fn is_hex_char(mut c: i8) -> i32 {
    return (c as i32 >= '0' as i32 && c as i32 <= '9' as i32
        || c as i32 >= 'A' as i32 && c as i32 <= 'F' as i32
        || c as i32 >= 'a' as i32 && c as i32 <= 'f' as i32) as i32;
}
static mut json_null_str: [i8; 5] =
    unsafe { *core::intrinsics::transmute::<&'_ [u8; 5], &'_ [i8; 5]>(b"null\0") };
static mut json_null_str_len: i32 = 0;
static mut json_inf_str: [i8; 9] =
    unsafe { *core::intrinsics::transmute::<&'_ [u8; 9], &'_ [i8; 9]>(b"Infinity\0") };
static mut json_inf_str_invert: [i8; 9] =
    unsafe { *core::intrinsics::transmute::<&'_ [u8; 9], &'_ [i8; 9]>(b"iNFINITY\0") };
static mut json_inf_str_len: u32 = 0;
static mut json_nan_str: [i8; 4] =
    unsafe { *core::intrinsics::transmute::<&'_ [u8; 4], &'_ [i8; 4]>(b"NaN\0") };
static mut json_nan_str_len: i32 = 0;
static mut json_true_str: [i8; 5] =
    unsafe { *core::intrinsics::transmute::<&'_ [u8; 5], &'_ [i8; 5]>(b"true\0") };
static mut json_true_str_len: i32 = 0;
static mut json_false_str: [i8; 6] =
    unsafe { *core::intrinsics::transmute::<&'_ [u8; 6], &'_ [i8; 6]>(b"false\0") };
static mut json_false_str_len: i32 = 0;
static mut json_tokener_errors: [*const i8; 16] = [
    b"success\0" as *const u8 as *const i8,
    b"continue\0" as *const u8 as *const i8,
    b"nesting too deep\0" as *const u8 as *const i8,
    b"unexpected end of data\0" as *const u8 as *const i8,
    b"unexpected character\0" as *const u8 as *const i8,
    b"null expected\0" as *const u8 as *const i8,
    b"boolean expected\0" as *const u8 as *const i8,
    b"number expected\0" as *const u8 as *const i8,
    b"array value separator ',' expected\0" as *const u8 as *const i8,
    b"quoted object property name expected\0" as *const u8 as *const i8,
    b"object property name separator ':' expected\0" as *const u8 as *const i8,
    b"object value separator ',' expected\0" as *const u8 as *const i8,
    b"invalid string sequence\0" as *const u8 as *const i8,
    b"expected comment\0" as *const u8 as *const i8,
    b"invalid utf-8 string\0" as *const u8 as *const i8,
    b"buffer size overflow\0" as *const u8 as *const i8,
];
#[no_mangle]
pub extern "C" fn json_tokener_error_desc(mut jerr: u32) -> *const i8 {
    let mut jerr_int: i32 = jerr as i32;
    if jerr_int < 0 as i32
        || jerr_int
            >= (::std::mem::size_of::<[*const i8; 16]>() as u64)
                .wrapping_div(::std::mem::size_of::<*const i8>() as u64) as i32
    {
        return b"Unknown error, invalid json_tokener_error value passed to json_tokener_error_desc()\0" as * const u8 as * const i8 ;
    }
    return (unsafe { json_tokener_errors[jerr as usize] });
}
#[no_mangle]
pub extern "C" fn json_tokener_get_error<'a1>(
    mut tok: *mut crate::src::apps::json_parse::json_tokener<'a1>,
) -> u32 {
    return (unsafe { (*tok).err });
}
static mut utf8_replacement_char: [u8; 3] =
    [0xef as i32 as u8, 0xbf as i32 as u8, 0xbd as i32 as u8];
#[no_mangle]
pub extern "C" fn json_tokener_new_ex<'a1>(
    mut depth: i32,
) -> *mut crate::src::apps::json_parse::json_tokener<'a1> {
    let mut tok: *mut crate::src::apps::json_parse::json_tokener<'_> = 0 as *mut json_tokener;
    tok = (unsafe { calloc(
        1 as i32 as u64,
        ::std::mem::size_of::<json_tokener>() as u64,
    ) }) as *mut json_tokener;
    if tok.is_null() {
        return 0 as *mut json_tokener;
    }
    let (unsafe { ref mut fresh0 }) = (*tok).stack;
    *fresh0 = (unsafe { calloc(
        depth as u64,
        ::std::mem::size_of::<json_tokener_srec>() as u64,
    ) }) as *mut json_tokener_srec;
    if (unsafe { ((*tok).stack) }).is_null() {
        (unsafe { free(tok as *mut libc::c_void) });
        return 0 as *mut json_tokener;
    }
    let (unsafe { ref mut fresh1 }) = (*tok).pb;
    *fresh1 = printbuf_new();
    if (unsafe { ((*tok).pb) }).is_null() {
        (unsafe { free((*tok).stack as *mut libc::c_void) });
        (unsafe { free(tok as *mut libc::c_void) });
        return 0 as *mut json_tokener;
    }
    (unsafe { (*tok).max_depth = depth });
    json_tokener_reset(tok);
    return tok;
}
#[no_mangle]
pub extern "C" fn json_tokener_new<'a1>() -> *mut crate::src::apps::json_parse::json_tokener<'a1> {
    return json_tokener_new_ex(32 as i32);
}
#[no_mangle]
pub extern "C" fn json_tokener_free<'a1>(
    mut tok: *mut crate::src::apps::json_parse::json_tokener<'a1>,
) {
    json_tokener_reset(tok);
    if !(unsafe { ((*tok).pb) }).is_null() {
        printbuf_free((unsafe { (*tok).pb }));
    }
    (unsafe { free((*tok).stack as *mut libc::c_void) });
    (unsafe { free(tok as *mut libc::c_void) });
}
extern "C" fn json_tokener_reset_level<'a1>(
    mut tok: *mut crate::src::apps::json_parse::json_tokener<'a1>,
    mut depth: i32,
) {
    (unsafe { (*((*tok).stack).offset(depth as isize)).state = json_tokener_state_eatws });
    (unsafe { (*((*tok).stack).offset(depth as isize)).saved_state = json_tokener_state_start });
    json_object_put((unsafe { (*((*tok).stack).offset(depth as isize)).current }));
    let (unsafe { ref mut fresh2 }) = (*(unsafe { ((*tok).stack).offset(depth as isize) })).current;
    *fresh2 = 0 as *mut json_object;
    (unsafe { free((*((*tok).stack).offset(depth as isize)).obj_field_name as *mut libc::c_void) });
    let (unsafe { ref mut fresh3 }) = (*(unsafe { ((*tok).stack).offset(depth as isize) })).obj_field_name;
    *fresh3 = 0 as *mut i8;
}
#[no_mangle]
pub extern "C" fn json_tokener_reset<'a1>(
    mut tok: *mut crate::src::apps::json_parse::json_tokener<'a1>,
) {
    let mut i: i32 = 0;
    if tok.is_null() {
        return;
    }
    i = (unsafe { (*tok).depth });
    while i >= 0 as i32 {
        json_tokener_reset_level(tok, i);
        i -= 1;
    }
    (unsafe { (*tok).depth = 0 as i32 });
    (unsafe { (*tok).err = json_tokener_success });
}
#[no_mangle]
pub extern "C" fn json_tokener_parse(
    mut str: *const i8,
) -> *mut crate::src::json_object::json_object {
    let mut jerr_ignored: u32 = json_tokener_success;
    let mut obj: *mut crate::src::json_object::json_object = 0 as *mut json_object;
    obj = json_tokener_parse_verbose(str, Some(&mut jerr_ignored));
    return obj;
}
#[no_mangle]
pub extern "C" fn json_tokener_parse_verbose<'a1>(
    mut str: *const i8,
    mut error: Option<&'a1 mut u32>,
) -> *mut crate::src::json_object::json_object {
    let mut tok: *mut crate::src::apps::json_parse::json_tokener<'_> = 0 as *mut json_tokener;
    let mut obj: *mut crate::src::json_object::json_object = 0 as *mut json_object;
    tok = json_tokener_new();
    if tok.is_null() {
        return 0 as *mut json_object;
    }
    obj = json_tokener_parse_ex(tok, str, -(1 as i32));
    *(borrow_mut(&mut error)).unwrap() = (unsafe { (*tok).err });
    if (unsafe { (*tok).err }) as u32 != json_tokener_success as i32 as u32 {
        if !obj.is_null() {
            json_object_put(obj);
        }
        obj = 0 as *mut json_object;
    }
    json_tokener_free(tok);
    return obj;
}
#[no_mangle]
pub extern "C" fn json_tokener_parse_ex<'a1>(
    mut tok: *mut crate::src::apps::json_parse::json_tokener<'a1>,
    mut str: *const i8,
    mut len: i32,
) -> *mut crate::src::json_object::json_object {
    let mut case_start_3: *const i8 = 0 as *const i8;
    let mut case_start_1: *const i8 = 0 as *const i8;
    let mut case_start_0: *const i8 = 0 as *const i8;
    let mut case_start: *const i8 = 0 as *const i8;
    let mut size: i32 = 0;
    let mut size_nan: i32 = 0;
    let mut size1: i32 = 0;
    let mut size2: i32 = 0;
    let mut current_block: u64;
    let mut obj: *mut crate::src::json_object::json_object = 0 as *mut json_object;
    let mut c: i8 = '\u{1}' as i32 as i8;
    let mut nBytes: u32 = 0 as i32 as u32;
    let mut nBytesp: Option<&'_ mut u32> = Some(&mut nBytes);
    let mut oldlocale: *mut crate::src::json_tokener::__locale_struct = (unsafe { uselocale(0 as locale_t) });
    let mut newloc: *mut crate::src::json_tokener::__locale_struct = 0 as *mut __locale_struct;
    (unsafe { (*tok).char_offset = 0 as i32 });
    (unsafe { (*tok).err = json_tokener_success });
    if len < -(1 as i32) || len == -(1 as i32) && (unsafe { strlen(str) }) > 2147483647 as i32 as u64 {
        (unsafe { (*tok).err = json_tokener_error_size });
        return 0 as *mut json_object;
    }
    let mut duploc: *mut crate::src::json_tokener::__locale_struct = (unsafe { duplocale(oldlocale) });
    newloc = (unsafe { newlocale(
        (1 as i32) << 1 as i32,
        b"C\0" as *const u8 as *const i8,
        duploc,
    ) });
    if newloc.is_null() {
        (unsafe { freelocale(duploc) });
        return 0 as *mut json_object;
    }
    (unsafe { uselocale(newloc) });
    's_76: while if (unsafe { (*tok).char_offset }) == len {
        if (unsafe { (*tok).depth }) == 0 as i32
            && (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state }) as u32
                == json_tokener_state_eatws as i32 as u32
            && (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).saved_state }) as u32
                == json_tokener_state_finish as i32 as u32
        {
            (unsafe { (*tok).err = json_tokener_success });
            0 as i32
        } else {
            (unsafe { (*tok).err = json_tokener_continue });
            0 as i32
        }
    } else if (unsafe { (*tok).flags }) & 0x10 as i32 != 0
        && json_tokener_validate_utf8((unsafe { *str }), borrow_mut(&mut nBytesp)) == 0
    {
        (unsafe { (*tok).err = json_tokener_error_parse_utf8_string });
        0 as i32
    } else {
        c = (unsafe { *str });
        1 as i32
    } != 0
    {
        loop {
            match (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state }) as u32 {
                0 => {
                    while is_ws_char(c) != 0 {
                        str = (unsafe { str.offset(1) });
                        let (unsafe { ref mut fresh4 }) = (*tok).char_offset;
                        *fresh4 += 1;
                        if c == 0
                            || (if (unsafe { (*tok).char_offset }) == len {
                                (if (unsafe { (*tok).depth }) == 0 as i32
                                    && (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state }) as u32
                                        == json_tokener_state_eatws as i32 as u32
                                    && (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).saved_state })
                                        as u32
                                        == json_tokener_state_finish as i32 as u32
                                {
                                    (unsafe { (*tok).err = json_tokener_success });
                                    0 as i32
                                } else {
                                    (unsafe { (*tok).err = json_tokener_continue });
                                    0 as i32
                                })
                            } else {
                                (if (unsafe { (*tok).flags }) & 0x10 as i32 != 0
                                    && json_tokener_validate_utf8((unsafe { *str }), borrow_mut(&mut nBytesp))
                                        == 0
                                {
                                    (unsafe { (*tok).err = json_tokener_error_parse_utf8_string });
                                    0 as i32
                                } else {
                                    c = (unsafe { *str });
                                    1 as i32
                                })
                            }) == 0
                        {
                            break 's_76;
                        }
                    }
                    if c as i32 == '/' as i32 && (unsafe { (*tok).flags }) & 0x1 as i32 == 0 {
                        printbuf_reset((unsafe { (*tok).pb }));
                        if (unsafe { (*(*tok).pb).size }) - (unsafe { (*(*tok).pb).bpos }) > 1 as i32 {
                            (unsafe { memcpy(
                                ((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize)
                                    as *mut libc::c_void,
                                &mut c as *mut i8 as *const libc::c_void,
                                1 as i32 as u64,
                            ) });
                            (unsafe { (*(*tok).pb).bpos += 1 as i32 });
                            (unsafe { *((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize) =
                                '\u{0}' as i32 as i8 });
                        } else {
                            printbuf_memappend((unsafe { (*tok).pb }), &mut c, 1 as i32);
                        }
                        (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state =
                            json_tokener_state_comment_start });
                        current_block = 14910428593596980134;
                        break;
                    } else {
                        (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state =
                            (*((*tok).stack).offset((*tok).depth as isize)).saved_state });
                    }
                },
                1 => match c as i32 {
                    123 => {
                        (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state =
                            json_tokener_state_eatws });
                        (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).saved_state =
                            json_tokener_state_object_field_start });
                        let (unsafe { ref mut fresh5 }) =
                            (*(unsafe { ((*tok).stack).offset((*tok).depth as isize) })).current;
                        *fresh5 = json_object_new_object();
                        if (unsafe { ((*((*tok).stack).offset((*tok).depth as isize)).current) }).is_null() {
                            break 's_76;
                        } else {
                            current_block = 14910428593596980134;
                            break;
                        }
                    },
                    91 => {
                        (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state =
                            json_tokener_state_eatws });
                        (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).saved_state =
                            json_tokener_state_array });
                        let (unsafe { ref mut fresh6 }) =
                            (*(unsafe { ((*tok).stack).offset((*tok).depth as isize) })).current;
                        *fresh6 = json_object_new_array();
                        if (unsafe { ((*((*tok).stack).offset((*tok).depth as isize)).current) }).is_null() {
                            break 's_76;
                        } else {
                            current_block = 14910428593596980134;
                            break;
                        }
                    },
                    73 | 105 => {
                        (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state =
                            json_tokener_state_inf });
                        printbuf_reset((unsafe { (*tok).pb }));
                        (unsafe { (*tok).st_pos = 0 as i32 });
                    },
                    78 | 110 => {
                        (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state =
                            json_tokener_state_null });
                        printbuf_reset((unsafe { (*tok).pb }));
                        (unsafe { (*tok).st_pos = 0 as i32 });
                    },
                    39 => {
                        if (unsafe { (*tok).flags }) & 0x1 as i32 != 0 {
                            current_block = 7990025728955927862;
                            break;
                        } else {
                            current_block = 5965298418754406754;
                            break;
                        }
                    },
                    34 => {
                        current_block = 5965298418754406754;
                        break;
                    },
                    84 | 116 | 70 | 102 => {
                        (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state =
                            json_tokener_state_boolean });
                        printbuf_reset((unsafe { (*tok).pb }));
                        (unsafe { (*tok).st_pos = 0 as i32 });
                    },
                    48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 45 => {
                        (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state =
                            json_tokener_state_number });
                        printbuf_reset((unsafe { (*tok).pb }));
                        (unsafe { (*tok).is_double = 0 as i32 });
                    },
                    _ => {
                        (unsafe { (*tok).err = json_tokener_error_parse_unexpected });
                        break 's_76;
                    },
                },
                2 => {
                    if (unsafe { (*tok).depth }) == 0 as i32 {
                        break 's_76;
                    }
                    obj = json_object_get((unsafe { (*((*tok).stack).offset((*tok).depth as isize)).current }));
                    json_tokener_reset_level(tok, (unsafe { (*tok).depth }));
                    let (unsafe { ref mut fresh7 }) = (*tok).depth;
                    *fresh7 -= 1;
                },
                26 => {
                    let mut is_negative: i32 = 0 as i32;
                    while (unsafe { (*tok).st_pos }) < (unsafe { json_inf_str_len }) as i32 {
                        let mut inf_char: i8 = (unsafe { *str });
                        if inf_char as i32 != (unsafe { json_inf_str[(*tok).st_pos as usize] }) as i32
                            && ((unsafe { (*tok).flags }) & 0x1 as i32 != 0
                                || inf_char as i32
                                    != (unsafe { json_inf_str_invert[(*tok).st_pos as usize] }) as i32)
                        {
                            (unsafe { (*tok).err = json_tokener_error_parse_unexpected });
                            break 's_76;
                        } else {
                            let (unsafe { ref mut fresh8 }) = (*tok).st_pos;
                            *fresh8 += 1;
                            str = (unsafe { str.offset(1) });
                            let (unsafe { ref mut fresh9 }) = (*tok).char_offset;
                            *fresh9 += 1;
                            if !(if (unsafe { (*tok).char_offset }) == len {
                                if (unsafe { (*tok).depth }) == 0 as i32
                                    && (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state }) as u32
                                        == json_tokener_state_eatws as i32 as u32
                                    && (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).saved_state })
                                        as u32
                                        == json_tokener_state_finish as i32 as u32
                                {
                                    (unsafe { (*tok).err = json_tokener_success });
                                    0 as i32
                                } else {
                                    (unsafe { (*tok).err = json_tokener_continue });
                                    0 as i32
                                }
                            } else if (unsafe { (*tok).flags }) & 0x10 as i32 != 0
                                && json_tokener_validate_utf8((unsafe { *str }), borrow_mut(&mut nBytesp)) == 0
                            {
                                (unsafe { (*tok).err = json_tokener_error_parse_utf8_string });
                                0 as i32
                            } else {
                                c = (unsafe { *str });
                                1 as i32
                            } == 0)
                            {
                                continue;
                            }
                            break 's_76;
                        }
                    }
                    if (unsafe { (*(*tok).pb).bpos }) > 0 as i32 && (unsafe { *(*(*tok).pb).buf }) as i32 == '-' as i32 {
                        is_negative = 1 as i32;
                    }
                    let (unsafe { ref mut fresh10 }) = (*(unsafe { ((*tok).stack).offset((*tok).depth as isize) })).current;
                    *fresh10 = json_object_new_double(
                        (if is_negative != 0 {
                            -::std::f32::INFINITY
                        } else {
                            ::std::f32::INFINITY
                        }) as f64,
                    );
                    if (unsafe { ((*((*tok).stack).offset((*tok).depth as isize)).current) }).is_null() {
                        break 's_76;
                    }
                    (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).saved_state =
                        json_tokener_state_finish });
                    (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state =
                        json_tokener_state_eatws });
                },
                3 => {
                    size = 0;
                    size_nan = 0;
                    if (unsafe { (*(*tok).pb).size }) - (unsafe { (*(*tok).pb).bpos }) > 1 as i32 {
                        (unsafe { memcpy(
                            ((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize)
                                as *mut libc::c_void,
                            &mut c as *mut i8 as *const libc::c_void,
                            1 as i32 as u64,
                        ) });
                        (unsafe { (*(*tok).pb).bpos += 1 as i32 });
                        (unsafe { *((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize) =
                            '\u{0}' as i32 as i8 });
                    } else {
                        printbuf_memappend((unsafe { (*tok).pb }), &mut c, 1 as i32);
                    }
                    size = if ((unsafe { (*tok).st_pos }) + 1 as i32) < (unsafe { json_null_str_len }) {
                        (unsafe { (*tok).st_pos }) + 1 as i32
                    } else {
                        (unsafe { json_null_str_len })
                    };
                    size_nan = if ((unsafe { (*tok).st_pos }) + 1 as i32) < (unsafe { json_nan_str_len }) {
                        (unsafe { (*tok).st_pos }) + 1 as i32
                    } else {
                        (unsafe { json_nan_str_len })
                    };
                    if (unsafe { (*tok).flags }) & 0x1 as i32 == 0
                        && (unsafe { strncasecmp(json_null_str.as_ptr(), (*(*tok).pb).buf, size as u64) })
                            == 0 as i32
                        || (unsafe { strncmp(json_null_str.as_ptr(), (*(*tok).pb).buf, size as u64) })
                            == 0 as i32
                    {
                        if !((unsafe { (*tok).st_pos }) == (unsafe { json_null_str_len })) {
                            current_block = 13823707972521062695;
                            break;
                        }
                        let (unsafe { ref mut fresh11 }) =
                            (*(unsafe { ((*tok).stack).offset((*tok).depth as isize) })).current;
                        *fresh11 = 0 as *mut json_object;
                        (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).saved_state =
                            json_tokener_state_finish });
                        (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state =
                            json_tokener_state_eatws });
                    } else if (unsafe { (*tok).flags }) & 0x1 as i32 == 0
                        && (unsafe { strncasecmp(json_nan_str.as_ptr(), (*(*tok).pb).buf, size_nan as u64) })
                            == 0 as i32
                        || (unsafe { strncmp(json_nan_str.as_ptr(), (*(*tok).pb).buf, size_nan as u64) })
                            == 0 as i32
                    {
                        if !((unsafe { (*tok).st_pos }) == (unsafe { json_nan_str_len })) {
                            current_block = 13823707972521062695;
                            break;
                        }
                        let (unsafe { ref mut fresh12 }) =
                            (*(unsafe { ((*tok).stack).offset((*tok).depth as isize) })).current;
                        *fresh12 = json_object_new_double(::std::f32::NAN as f64);
                        if (unsafe { ((*((*tok).stack).offset((*tok).depth as isize)).current) }).is_null() {
                            break 's_76;
                        }
                        (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).saved_state =
                            json_tokener_state_finish });
                        (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state =
                            json_tokener_state_eatws });
                    } else {
                        (unsafe { (*tok).err = json_tokener_error_parse_null });
                        break 's_76;
                    }
                },
                4 => {
                    if c as i32 == '*' as i32 {
                        current_block = 11162283542402356847;
                        break;
                    } else {
                        current_block = 17917672080766325409;
                        break;
                    }
                },
                5 => {
                    case_start = str;
                    current_block = 18221534353613080499;
                    break;
                },
                6 => {
                    case_start_0 = str;
                    current_block = 6662862405959679103;
                    break;
                },
                7 => {
                    if (unsafe { (*(*tok).pb).size }) - (unsafe { (*(*tok).pb).bpos }) > 1 as i32 {
                        (unsafe { memcpy(
                            ((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize)
                                as *mut libc::c_void,
                            &mut c as *mut i8 as *const libc::c_void,
                            1 as i32 as u64,
                        ) });
                        (unsafe { (*(*tok).pb).bpos += 1 as i32 });
                        (unsafe { *((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize) =
                            '\u{0}' as i32 as i8 });
                    } else {
                        printbuf_memappend((unsafe { (*tok).pb }), &mut c, 1 as i32);
                    }
                    if c as i32 == '/' as i32 {
                        (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state =
                            json_tokener_state_eatws });
                    } else {
                        (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state =
                            json_tokener_state_comment });
                    }
                    current_block = 14910428593596980134;
                    break;
                },
                8 => {
                    case_start_1 = str;
                    current_block = 13526015532137226550;
                    break;
                },
                9 => match c as i32 {
                    34 | 92 | 47 => {
                        current_block = 5851233167266824712;
                        break;
                    },
                    98 | 110 | 114 | 116 | 102 => {
                        current_block = 902433882565356955;
                        break;
                    },
                    117 => {
                        current_block = 17501499944854094182;
                        break;
                    },
                    _ => {
                        current_block = 15463013039495622015;
                        break;
                    },
                },
                10 => {
                    current_block = 5600328731811258759;
                    break;
                },
                11 => {
                    if c == 0 || c as i32 != '\\' as i32 {
                        if (unsafe { (*(*tok).pb).size }) - (unsafe { (*(*tok).pb).bpos }) > 3 as i32 {
                            (unsafe { memcpy(
                                ((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize)
                                    as *mut libc::c_void,
                                utf8_replacement_char.as_mut_ptr() as *mut i8
                                    as *const libc::c_void,
                                3 as i32 as u64,
                            ) });
                            (unsafe { (*(*tok).pb).bpos += 3 as i32 });
                            (unsafe { *((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize) =
                                '\u{0}' as i32 as i8 });
                        } else {
                            printbuf_memappend(
                                (unsafe { (*tok).pb }),
                                (unsafe { utf8_replacement_char.as_mut_ptr() }) as *mut i8,
                                3 as i32,
                            );
                        }
                        (unsafe { (*tok).high_surrogate = 0 as i32 as u32 });
                        (unsafe { (*tok).ucs_char = 0 as i32 as u32 });
                        (unsafe { (*tok).st_pos = 0 as i32 });
                        (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state =
                            (*((*tok).stack).offset((*tok).depth as isize)).saved_state });
                    } else {
                        (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state =
                            json_tokener_state_escape_unicode_need_u });
                        current_block = 14910428593596980134;
                        break;
                    }
                },
                12 => {
                    if c == 0 || c as i32 != 'u' as i32 {
                        if (unsafe { (*(*tok).pb).size }) - (unsafe { (*(*tok).pb).bpos }) > 3 as i32 {
                            (unsafe { memcpy(
                                ((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize)
                                    as *mut libc::c_void,
                                utf8_replacement_char.as_mut_ptr() as *mut i8
                                    as *const libc::c_void,
                                3 as i32 as u64,
                            ) });
                            (unsafe { (*(*tok).pb).bpos += 3 as i32 });
                            (unsafe { *((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize) =
                                '\u{0}' as i32 as i8 });
                        } else {
                            printbuf_memappend(
                                (unsafe { (*tok).pb }),
                                (unsafe { utf8_replacement_char.as_mut_ptr() }) as *mut i8,
                                3 as i32,
                            );
                        }
                        (unsafe { (*tok).high_surrogate = 0 as i32 as u32 });
                        (unsafe { (*tok).ucs_char = 0 as i32 as u32 });
                        (unsafe { (*tok).st_pos = 0 as i32 });
                        (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state =
                            json_tokener_state_string_escape });
                    } else {
                        (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state =
                            json_tokener_state_escape_unicode });
                        current_block = 14910428593596980134;
                        break;
                    }
                },
                13 => {
                    size1 = 0;
                    size2 = 0;
                    if (unsafe { (*(*tok).pb).size }) - (unsafe { (*(*tok).pb).bpos }) > 1 as i32 {
                        (unsafe { memcpy(
                            ((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize)
                                as *mut libc::c_void,
                            &mut c as *mut i8 as *const libc::c_void,
                            1 as i32 as u64,
                        ) });
                        (unsafe { (*(*tok).pb).bpos += 1 as i32 });
                        (unsafe { *((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize) =
                            '\u{0}' as i32 as i8 });
                    } else {
                        printbuf_memappend((unsafe { (*tok).pb }), &mut c, 1 as i32);
                    }
                    size1 = if ((unsafe { (*tok).st_pos }) + 1 as i32) < (unsafe { json_true_str_len }) {
                        (unsafe { (*tok).st_pos }) + 1 as i32
                    } else {
                        (unsafe { json_true_str_len })
                    };
                    size2 = if ((unsafe { (*tok).st_pos }) + 1 as i32) < (unsafe { json_false_str_len }) {
                        (unsafe { (*tok).st_pos }) + 1 as i32
                    } else {
                        (unsafe { json_false_str_len })
                    };
                    if (unsafe { (*tok).flags }) & 0x1 as i32 == 0
                        && (unsafe { strncasecmp(json_true_str.as_ptr(), (*(*tok).pb).buf, size1 as u64) })
                            == 0 as i32
                        || (unsafe { strncmp(json_true_str.as_ptr(), (*(*tok).pb).buf, size1 as u64) })
                            == 0 as i32
                    {
                        if !((unsafe { (*tok).st_pos }) == (unsafe { json_true_str_len })) {
                            current_block = 6015864261243718670;
                            break;
                        }
                        let (unsafe { ref mut fresh27 }) =
                            (*(unsafe { ((*tok).stack).offset((*tok).depth as isize) })).current;
                        *fresh27 = json_object_new_boolean(1 as i32);
                        if (unsafe { ((*((*tok).stack).offset((*tok).depth as isize)).current) }).is_null() {
                            break 's_76;
                        }
                        (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).saved_state =
                            json_tokener_state_finish });
                        (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state =
                            json_tokener_state_eatws });
                    } else if (unsafe { (*tok).flags }) & 0x1 as i32 == 0
                        && (unsafe { strncasecmp(json_false_str.as_ptr(), (*(*tok).pb).buf, size2 as u64) })
                            == 0 as i32
                        || (unsafe { strncmp(json_false_str.as_ptr(), (*(*tok).pb).buf, size2 as u64) })
                            == 0 as i32
                    {
                        if !((unsafe { (*tok).st_pos }) == (unsafe { json_false_str_len })) {
                            current_block = 6015864261243718670;
                            break;
                        }
                        let (unsafe { ref mut fresh28 }) =
                            (*(unsafe { ((*tok).stack).offset((*tok).depth as isize) })).current;
                        *fresh28 = json_object_new_boolean(0 as i32);
                        if (unsafe { ((*((*tok).stack).offset((*tok).depth as isize)).current) }).is_null() {
                            break 's_76;
                        }
                        (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).saved_state =
                            json_tokener_state_finish });
                        (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state =
                            json_tokener_state_eatws });
                    } else {
                        (unsafe { (*tok).err = json_tokener_error_parse_boolean });
                        break 's_76;
                    }
                },
                14 => {
                    let mut case_start_2: *const i8 = str;
                    let mut case_len: i32 = 0 as i32;
                    let mut is_exponent: i32 = 0 as i32;
                    let mut neg_sign_ok: i32 = 1 as i32;
                    let mut pos_sign_ok: i32 = 0 as i32;
                    if (unsafe { (*(*tok).pb).bpos }) > 0 as i32 {
                        let mut e_loc: *mut i8 = (unsafe { strchr((*(*tok).pb).buf, 'e' as i32) });
                        if e_loc.is_null() {
                            e_loc = (unsafe { strchr((*(*tok).pb).buf, 'E' as i32) });
                        }
                        if !e_loc.is_null() {
                            let mut last_saved_char: *mut i8 = (unsafe { &mut *((*(*tok).pb).buf)
                                .offset(((*(*tok).pb).bpos - 1 as i32) as isize) })
                                as *mut i8;
                            is_exponent = 1 as i32;
                            neg_sign_ok = 1 as i32;
                            pos_sign_ok = neg_sign_ok;
                            if e_loc != last_saved_char {
                                neg_sign_ok = 0 as i32;
                                pos_sign_ok = 0 as i32;
                            }
                        }
                    }
                    while c as i32 != 0
                        && (c as i32 >= '0' as i32 && c as i32 <= '9' as i32
                            || is_exponent == 0
                                && (c as i32 == 'e' as i32 || c as i32 == 'E' as i32)
                            || neg_sign_ok != 0 && c as i32 == '-' as i32
                            || pos_sign_ok != 0 && c as i32 == '+' as i32
                            || (unsafe { (*tok).is_double }) == 0 && c as i32 == '.' as i32)
                    {
                        neg_sign_ok = 0 as i32;
                        pos_sign_ok = neg_sign_ok;
                        case_len += 1;
                        match c as i32 {
                            46 => {
                                (unsafe { (*tok).is_double = 1 as i32 });
                                pos_sign_ok = 1 as i32;
                                neg_sign_ok = 1 as i32;
                            },
                            101 | 69 => {
                                is_exponent = 1 as i32;
                                (unsafe { (*tok).is_double = 1 as i32 });
                                neg_sign_ok = 1 as i32;
                                pos_sign_ok = neg_sign_ok;
                            },
                            _ => {},
                        }
                        str = (unsafe { str.offset(1) });
                        let (unsafe { ref mut fresh30 }) = (*tok).char_offset;
                        *fresh30 += 1;
                        if !(c == 0
                            || (if (unsafe { (*tok).char_offset }) == len {
                                (if (unsafe { (*tok).depth }) == 0 as i32
                                    && (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state }) as u32
                                        == json_tokener_state_eatws as i32 as u32
                                    && (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).saved_state })
                                        as u32
                                        == json_tokener_state_finish as i32 as u32
                                {
                                    (unsafe { (*tok).err = json_tokener_success });
                                    0 as i32
                                } else {
                                    (unsafe { (*tok).err = json_tokener_continue });
                                    0 as i32
                                })
                            } else {
                                (if (unsafe { (*tok).flags }) & 0x10 as i32 != 0
                                    && json_tokener_validate_utf8((unsafe { *str }), borrow_mut(&mut nBytesp))
                                        == 0
                                {
                                    (unsafe { (*tok).err = json_tokener_error_parse_utf8_string });
                                    0 as i32
                                } else {
                                    c = (unsafe { *str });
                                    1 as i32
                                })
                            }) == 0)
                        {
                            continue;
                        }
                        if (unsafe { (*(*tok).pb).size }) - (unsafe { (*(*tok).pb).bpos }) > case_len {
                            (unsafe { memcpy(
                                ((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize)
                                    as *mut libc::c_void,
                                case_start_2 as *const libc::c_void,
                                case_len as u64,
                            ) });
                            (unsafe { (*(*tok).pb).bpos += case_len });
                            (unsafe { *((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize) =
                                '\u{0}' as i32 as i8 });
                        } else {
                            printbuf_memappend((unsafe { (*tok).pb }), case_start_2, case_len);
                        }
                        break 's_76;
                    }
                    if (unsafe { (*tok).depth }) > 0 as i32
                        && c as i32 != ',' as i32
                        && c as i32 != ']' as i32
                        && c as i32 != '}' as i32
                        && c as i32 != '/' as i32
                        && c as i32 != 'I' as i32
                        && c as i32 != 'i' as i32
                        && is_ws_char(c) == 0
                    {
                        (unsafe { (*tok).err = json_tokener_error_parse_number });
                        break 's_76;
                    } else {
                        if case_len > 0 as i32 {
                            if (unsafe { (*(*tok).pb).size }) - (unsafe { (*(*tok).pb).bpos }) > case_len {
                                (unsafe { memcpy(
                                    ((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize)
                                        as *mut libc::c_void,
                                    case_start_2 as *const libc::c_void,
                                    case_len as u64,
                                ) });
                                (unsafe { (*(*tok).pb).bpos += case_len });
                                (unsafe { *((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize) =
                                    '\u{0}' as i32 as i8 });
                            } else {
                                printbuf_memappend((unsafe { (*tok).pb }), case_start_2, case_len);
                            }
                        }
                        if (unsafe { *((*(*tok).pb).buf).offset(0 as i32 as isize) }) as i32 == '-' as i32
                            && case_len <= 1 as i32
                            && (c as i32 == 'i' as i32 || c as i32 == 'I' as i32)
                        {
                            (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state =
                                json_tokener_state_inf });
                            (unsafe { (*tok).st_pos = 0 as i32 });
                        } else {
                            if (unsafe { (*tok).is_double }) != 0 && (unsafe { (*tok).flags }) & 0x1 as i32 == 0 {
                                while (unsafe { (*(*tok).pb).bpos }) > 1 as i32 {
                                    let mut last_char: i8 = (unsafe { *((*(*tok).pb).buf)
                                        .offset(((*(*tok).pb).bpos - 1 as i32) as isize) });
                                    if last_char as i32 != 'e' as i32
                                        && last_char as i32 != 'E' as i32
                                        && last_char as i32 != '-' as i32
                                        && last_char as i32 != '+' as i32
                                    {
                                        break;
                                    }
                                    (unsafe { *((*(*tok).pb).buf)
                                        .offset(((*(*tok).pb).bpos - 1 as i32) as isize) =
                                        '\u{0}' as i32 as i8 });
                                    let (unsafe { ref mut fresh31 }) = (*(*tok).pb).bpos;
                                    *fresh31 -= 1;
                                }
                            }
                            let mut num64: i64 = 0;
                            let mut numuint64: u64 = 0;
                            let mut numd: f64 = 0.;
                            if (unsafe { (*tok).is_double }) == 0
                                && (unsafe { *((*(*tok).pb).buf).offset(0 as i32 as isize) }) as i32
                                    == '-' as i32
                                && json_parse_int64((unsafe { (*(*tok).pb).buf }), Some(&mut num64)) == 0 as i32
                            {
                                if (unsafe { *__errno_location() }) == 34 as i32
                                    && (unsafe { (*tok).flags }) & 0x1 as i32 != 0
                                {
                                    (unsafe { (*tok).err = json_tokener_error_parse_number });
                                    break 's_76;
                                } else {
                                    let (unsafe { ref mut fresh32 }) =
                                        (*(unsafe { ((*tok).stack).offset((*tok).depth as isize) })).current;
                                    *fresh32 = json_object_new_int64(num64);
                                    if (unsafe { ((*((*tok).stack).offset((*tok).depth as isize)).current) })
                                        .is_null()
                                    {
                                        break 's_76;
                                    }
                                }
                            } else if (unsafe { (*tok).is_double }) == 0
                                && (unsafe { *((*(*tok).pb).buf).offset(0 as i32 as isize) }) as i32
                                    != '-' as i32
                                && json_parse_uint64((unsafe { (*(*tok).pb).buf }), Some(&mut numuint64))
                                    == 0 as i32
                            {
                                if (unsafe { *__errno_location() }) == 34 as i32
                                    && (unsafe { (*tok).flags }) & 0x1 as i32 != 0
                                {
                                    (unsafe { (*tok).err = json_tokener_error_parse_number });
                                    break 's_76;
                                } else if numuint64 != 0
                                    && (unsafe { *((*(*tok).pb).buf).offset(0 as i32 as isize) }) as i32
                                        == '0' as i32
                                    && (unsafe { (*tok).flags }) & 0x1 as i32 != 0
                                {
                                    (unsafe { (*tok).err = json_tokener_error_parse_number });
                                    break 's_76;
                                } else if numuint64 <= 9223372036854775807 as i64 as u64 {
                                    num64 = numuint64 as int64_t;
                                    let (unsafe { ref mut fresh33 }) =
                                        (*(unsafe { ((*tok).stack).offset((*tok).depth as isize) })).current;
                                    *fresh33 = json_object_new_int64(num64);
                                    if (unsafe { ((*((*tok).stack).offset((*tok).depth as isize)).current) })
                                        .is_null()
                                    {
                                        break 's_76;
                                    }
                                } else {
                                    let (unsafe { ref mut fresh34 }) =
                                        (*(unsafe { ((*tok).stack).offset((*tok).depth as isize) })).current;
                                    *fresh34 = json_object_new_uint64(numuint64);
                                    if (unsafe { ((*((*tok).stack).offset((*tok).depth as isize)).current) })
                                        .is_null()
                                    {
                                        break 's_76;
                                    }
                                }
                            } else if (unsafe { (*tok).is_double }) != 0
                                && json_tokener_parse_double(
                                    (unsafe { (*(*tok).pb).buf }),
                                    (unsafe { (*(*tok).pb).bpos }),
                                    Some(&mut numd),
                                ) == 0 as i32
                            {
                                let (unsafe { ref mut fresh35 }) =
                                    (*(unsafe { ((*tok).stack).offset((*tok).depth as isize) })).current;
                                *fresh35 = json_object_new_double_s(numd, (unsafe { (*(*tok).pb).buf }));
                                if (unsafe { ((*((*tok).stack).offset((*tok).depth as isize)).current) })
                                    .is_null()
                                {
                                    break 's_76;
                                }
                            } else {
                                (unsafe { (*tok).err = json_tokener_error_parse_number });
                                break 's_76;
                            }
                            (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).saved_state =
                                json_tokener_state_finish });
                            (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state =
                                json_tokener_state_eatws });
                        }
                    }
                },
                24 | 15 => {
                    if c as i32 == ']' as i32 {
                        json_object_array_shrink(
                            (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).current }),
                            0 as i32,
                        );
                        if (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state }) as u32
                            == json_tokener_state_array_after_sep as i32 as u32
                            && (unsafe { (*tok).flags }) & 0x1 as i32 != 0
                        {
                            current_block = 5524635047616071233;
                            break;
                        } else {
                            current_block = 955764296456093747;
                            break;
                        }
                    } else if (unsafe { (*tok).depth }) >= (unsafe { (*tok).max_depth }) - 1 as i32 {
                        (unsafe { (*tok).err = json_tokener_error_depth });
                        break 's_76;
                    } else {
                        (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state =
                            json_tokener_state_array_add });
                        let (unsafe { ref mut fresh36 }) = (*tok).depth;
                        *fresh36 += 1;
                        json_tokener_reset_level(tok, (unsafe { (*tok).depth }));
                    }
                },
                16 => {
                    if json_object_array_add(
                        (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).current }),
                        obj,
                    ) != 0 as i32
                    {
                        break 's_76;
                    }
                    (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).saved_state =
                        json_tokener_state_array_sep });
                    (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state =
                        json_tokener_state_eatws });
                },
                17 => {
                    if c as i32 == ']' as i32 {
                        current_block = 17927085222977126794;
                        break;
                    } else {
                        current_block = 9898910953550671049;
                        break;
                    }
                },
                18 | 25 => {
                    if c as i32 == '}' as i32 {
                        current_block = 9952967170209510081;
                        break;
                    } else {
                        current_block = 8644614806574383103;
                        break;
                    }
                },
                19 => {
                    case_start_3 = str;
                    current_block = 16251542583832361733;
                    break;
                },
                20 => {
                    if c as i32 == ':' as i32 {
                        current_block = 14575203612169622940;
                        break;
                    } else {
                        current_block = 5151117443143562747;
                        break;
                    }
                },
                21 => {
                    if (unsafe { (*tok).depth }) >= (unsafe { (*tok).max_depth }) - 1 as i32 {
                        (unsafe { (*tok).err = json_tokener_error_depth });
                        break 's_76;
                    } else {
                        (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state =
                            json_tokener_state_object_value_add });
                        let (unsafe { ref mut fresh42 }) = (*tok).depth;
                        *fresh42 += 1;
                        json_tokener_reset_level(tok, (unsafe { (*tok).depth }));
                    }
                },
                22 => {
                    json_object_object_add(
                        (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).current }),
                        (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).obj_field_name }),
                        obj,
                    );
                    (unsafe { free(
                        (*((*tok).stack).offset((*tok).depth as isize)).obj_field_name
                            as *mut libc::c_void,
                    ) });
                    let (unsafe { ref mut fresh43 }) =
                        (*(unsafe { ((*tok).stack).offset((*tok).depth as isize) })).obj_field_name;
                    *fresh43 = 0 as *mut i8;
                    (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).saved_state =
                        json_tokener_state_object_sep });
                    (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state =
                        json_tokener_state_eatws });
                },
                23 => {
                    if c as i32 == '}' as i32 {
                        current_block = 10945600426047866894;
                        break;
                    } else {
                        current_block = 16760829112842239754;
                        break;
                    }
                },
                _ => {
                    current_block = 14910428593596980134;
                    break;
                },
            }
        }
        match current_block {
            16251542583832361733 => {
                loop {
                    if c as i32 == (unsafe { (*tok).quote_char }) as i32 {
                        if ((unsafe { (*(*tok).pb).size }) - (unsafe { (*(*tok).pb).bpos })) as i64
                            > (unsafe { str.offset_from(case_start_3) }) as i64
                        {
                            (unsafe { memcpy(
                                ((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize)
                                    as *mut libc::c_void,
                                case_start_3 as *const libc::c_void,
                                str.offset_from(case_start_3) as i64 as u64,
                            ) });
                            let (unsafe { ref mut fresh37 }) = (*(*tok).pb).bpos;
                            *fresh37 =
                                (*fresh37 as i64 + (unsafe { str.offset_from(case_start_3) }) as i64) as i32;
                            (unsafe { *((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize) =
                                '\u{0}' as i32 as i8 });
                        } else {
                            printbuf_memappend(
                                (unsafe { (*tok).pb }),
                                case_start_3,
                                (unsafe { str.offset_from(case_start_3) }) as i64 as i32,
                            );
                        }
                        let (unsafe { ref mut fresh38 }) =
                            (*(unsafe { ((*tok).stack).offset((*tok).depth as isize) })).obj_field_name;
                        *fresh38 = (unsafe { strdup((*(*tok).pb).buf) });
                        (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).saved_state =
                            json_tokener_state_object_field_end });
                        (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state =
                            json_tokener_state_eatws });
                        break;
                    } else if c as i32 == '\\' as i32 {
                        if ((unsafe { (*(*tok).pb).size }) - (unsafe { (*(*tok).pb).bpos })) as i64
                            > (unsafe { str.offset_from(case_start_3) }) as i64
                        {
                            (unsafe { memcpy(
                                ((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize)
                                    as *mut libc::c_void,
                                case_start_3 as *const libc::c_void,
                                str.offset_from(case_start_3) as i64 as u64,
                            ) });
                            let (unsafe { ref mut fresh39 }) = (*(*tok).pb).bpos;
                            *fresh39 =
                                (*fresh39 as i64 + (unsafe { str.offset_from(case_start_3) }) as i64) as i32;
                            (unsafe { *((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize) =
                                '\u{0}' as i32 as i8 });
                        } else {
                            printbuf_memappend(
                                (unsafe { (*tok).pb }),
                                case_start_3,
                                (unsafe { str.offset_from(case_start_3) }) as i64 as i32,
                            );
                        }
                        (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).saved_state =
                            json_tokener_state_object_field });
                        (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state =
                            json_tokener_state_string_escape });
                        break;
                    } else {
                        str = (unsafe { str.offset(1) });
                        let (unsafe { ref mut fresh40 }) = (*tok).char_offset;
                        *fresh40 += 1;
                        if !(c == 0
                            || (if (unsafe { (*tok).char_offset }) == len {
                                (if (unsafe { (*tok).depth }) == 0 as i32
                                    && (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state }) as u32
                                        == json_tokener_state_eatws as i32 as u32
                                    && (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).saved_state })
                                        as u32
                                        == json_tokener_state_finish as i32 as u32
                                {
                                    (unsafe { (*tok).err = json_tokener_success });
                                    0 as i32
                                } else {
                                    (unsafe { (*tok).err = json_tokener_continue });
                                    0 as i32
                                })
                            } else {
                                (if (unsafe { (*tok).flags }) & 0x10 as i32 != 0
                                    && json_tokener_validate_utf8((unsafe { *str }), borrow_mut(&mut nBytesp))
                                        == 0
                                {
                                    (unsafe { (*tok).err = json_tokener_error_parse_utf8_string });
                                    0 as i32
                                } else {
                                    c = (unsafe { *str });
                                    1 as i32
                                })
                            }) == 0)
                        {
                            continue;
                        }
                        if ((unsafe { (*(*tok).pb).size }) - (unsafe { (*(*tok).pb).bpos })) as i64
                            > (unsafe { str.offset_from(case_start_3) }) as i64
                        {
                            (unsafe { memcpy(
                                ((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize)
                                    as *mut libc::c_void,
                                case_start_3 as *const libc::c_void,
                                str.offset_from(case_start_3) as i64 as u64,
                            ) });
                            let (unsafe { ref mut fresh41 }) = (*(*tok).pb).bpos;
                            *fresh41 =
                                (*fresh41 as i64 + (unsafe { str.offset_from(case_start_3) }) as i64) as i32;
                            (unsafe { *((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize) =
                                '\u{0}' as i32 as i8 });
                        } else {
                            printbuf_memappend(
                                (unsafe { (*tok).pb }),
                                case_start_3,
                                (unsafe { str.offset_from(case_start_3) }) as i64 as i32,
                            );
                        }
                        break 's_76;
                    }
                }
                current_block = 14910428593596980134;
            },
            8644614806574383103 => {
                if c as i32 == '"' as i32 || c as i32 == '\'' as i32 {
                    (unsafe { (*tok).quote_char = c });
                    printbuf_reset((unsafe { (*tok).pb }));
                    (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state =
                        json_tokener_state_object_field });
                } else {
                    (unsafe { (*tok).err = json_tokener_error_parse_object_key_name });
                    break;
                }
                current_block = 14910428593596980134;
            },
            9952967170209510081 => {
                if (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state }) as u32
                    == json_tokener_state_object_field_start_after_sep as i32 as u32
                    && (unsafe { (*tok).flags }) & 0x1 as i32 != 0
                {
                    (unsafe { (*tok).err = json_tokener_error_parse_unexpected });
                    break;
                } else {
                    (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).saved_state =
                        json_tokener_state_finish });
                    (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state =
                        json_tokener_state_eatws });
                }
                current_block = 14910428593596980134;
            },
            9898910953550671049 => {
                if c as i32 == ',' as i32 {
                    (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).saved_state =
                        json_tokener_state_array_after_sep });
                    (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state =
                        json_tokener_state_eatws });
                } else {
                    (unsafe { (*tok).err = json_tokener_error_parse_array });
                    break;
                }
                current_block = 14910428593596980134;
            },
            955764296456093747 => {
                (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).saved_state =
                    json_tokener_state_finish });
                (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state = json_tokener_state_eatws });
                current_block = 14910428593596980134;
            },
            16760829112842239754 => {
                if c as i32 == ',' as i32 {
                    (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).saved_state =
                        json_tokener_state_object_field_start_after_sep });
                    (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state =
                        json_tokener_state_eatws });
                } else {
                    (unsafe { (*tok).err = json_tokener_error_parse_object_value_sep });
                    break;
                }
                current_block = 14910428593596980134;
            },
            5600328731811258759 => {
                loop {
                    if c == 0 || is_hex_char(c) == 0 {
                        (unsafe { (*tok).err = json_tokener_error_parse_string });
                        break 's_76;
                    } else {
                        (unsafe { (*tok).ucs_char |= ((if c as i32 <= '9' as i32 {
                            c as i32 - '0' as i32
                        } else {
                            (c as i32 & 7 as i32) + 9 as i32
                        }) as u32)
                            << (3 as i32 - (*tok).st_pos) * 4 as i32 });
                        let (unsafe { ref mut fresh25 }) = (*tok).st_pos;
                        *fresh25 += 1;
                        if (unsafe { (*tok).st_pos }) >= 4 as i32 {
                            break;
                        }
                        str = (unsafe { str.offset(1) });
                        let (unsafe { ref mut fresh26 }) = (*tok).char_offset;
                        *fresh26 += 1;
                        if !(if (unsafe { (*tok).char_offset }) == len {
                            if (unsafe { (*tok).depth }) == 0 as i32
                                && (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state }) as u32
                                    == json_tokener_state_eatws as i32 as u32
                                && (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).saved_state })
                                    as u32
                                    == json_tokener_state_finish as i32 as u32
                            {
                                (unsafe { (*tok).err = json_tokener_success });
                                0 as i32
                            } else {
                                (unsafe { (*tok).err = json_tokener_continue });
                                0 as i32
                            }
                        } else if (unsafe { (*tok).flags }) & 0x10 as i32 != 0
                            && json_tokener_validate_utf8((unsafe { *str }), borrow_mut(&mut nBytesp)) == 0
                        {
                            (unsafe { (*tok).err = json_tokener_error_parse_utf8_string });
                            0 as i32
                        } else {
                            c = (unsafe { *str });
                            1 as i32
                        } == 0)
                        {
                            continue;
                        }
                        break 's_76;
                    }
                }
                (unsafe { (*tok).st_pos = 0 as i32 });
                if (unsafe { (*tok).high_surrogate }) != 0 {
                    if (unsafe { (*tok).ucs_char }) & 0xfc00 as i32 as u32 == 0xdc00 as i32 as u32 {
                        (unsafe { (*tok).ucs_char = (((*tok).high_surrogate & 0x3ff as i32 as u32)
                            << 10 as i32)
                            .wrapping_add((*tok).ucs_char & 0x3ff as i32 as u32)
                            .wrapping_add(0x10000 as i32 as u32) });
                    } else if (unsafe { (*(*tok).pb).size }) - (unsafe { (*(*tok).pb).bpos }) > 3 as i32 {
                        (unsafe { memcpy(
                            ((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize)
                                as *mut libc::c_void,
                            utf8_replacement_char.as_mut_ptr() as *mut i8 as *const libc::c_void,
                            3 as i32 as u64,
                        ) });
                        (unsafe { (*(*tok).pb).bpos += 3 as i32 });
                        (unsafe { *((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize) =
                            '\u{0}' as i32 as i8 });
                    } else {
                        printbuf_memappend(
                            (unsafe { (*tok).pb }),
                            (unsafe { utf8_replacement_char.as_mut_ptr() }) as *mut i8,
                            3 as i32,
                        );
                    }
                    (unsafe { (*tok).high_surrogate = 0 as i32 as u32 });
                }
                if (unsafe { (*tok).ucs_char }) < 0x80 as i32 as u32 {
                    let mut unescaped_utf: [u8; 1] = [0; 1];
                    unescaped_utf[0 as i32 as usize] = (unsafe { (*tok).ucs_char }) as u8;
                    if (unsafe { (*(*tok).pb).size }) - (unsafe { (*(*tok).pb).bpos }) > 1 as i32 {
                        (unsafe { memcpy(
                            ((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize)
                                as *mut libc::c_void,
                            unescaped_utf.as_mut_ptr() as *mut i8 as *const libc::c_void,
                            1 as i32 as u64,
                        ) });
                        (unsafe { (*(*tok).pb).bpos += 1 as i32 });
                        (unsafe { *((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize) =
                            '\u{0}' as i32 as i8 });
                    } else {
                        printbuf_memappend(
                            (unsafe { (*tok).pb }),
                            unescaped_utf.as_mut_ptr() as *mut i8,
                            1 as i32,
                        );
                    }
                    current_block = 16438646162350502520;
                } else if (unsafe { (*tok).ucs_char }) < 0x800 as i32 as u32 {
                    let mut unescaped_utf_0: [u8; 2] = [0; 2];
                    unescaped_utf_0[0 as i32 as usize] =
                        (0xc0 as i32 as u32 | (unsafe { (*tok).ucs_char }) >> 6 as i32) as u8;
                    unescaped_utf_0[1 as i32 as usize] =
                        (0x80 as i32 as u32 | (unsafe { (*tok).ucs_char }) & 0x3f as i32 as u32) as u8;
                    if (unsafe { (*(*tok).pb).size }) - (unsafe { (*(*tok).pb).bpos }) > 2 as i32 {
                        (unsafe { memcpy(
                            ((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize)
                                as *mut libc::c_void,
                            unescaped_utf_0.as_mut_ptr() as *mut i8 as *const libc::c_void,
                            2 as i32 as u64,
                        ) });
                        (unsafe { (*(*tok).pb).bpos += 2 as i32 });
                        (unsafe { *((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize) =
                            '\u{0}' as i32 as i8 });
                    } else {
                        printbuf_memappend(
                            (unsafe { (*tok).pb }),
                            unescaped_utf_0.as_mut_ptr() as *mut i8,
                            2 as i32,
                        );
                    }
                    current_block = 16438646162350502520;
                } else if (unsafe { (*tok).ucs_char }) & 0xfc00 as i32 as u32 == 0xd800 as i32 as u32 {
                    (unsafe { (*tok).high_surrogate = (*tok).ucs_char });
                    (unsafe { (*tok).ucs_char = 0 as i32 as u32 });
                    (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state =
                        json_tokener_state_escape_unicode_need_escape });
                    current_block = 14910428593596980134;
                } else {
                    if (unsafe { (*tok).ucs_char }) & 0xfc00 as i32 as u32 == 0xdc00 as i32 as u32 {
                        if (unsafe { (*(*tok).pb).size }) - (unsafe { (*(*tok).pb).bpos }) > 3 as i32 {
                            (unsafe { memcpy(
                                ((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize)
                                    as *mut libc::c_void,
                                utf8_replacement_char.as_mut_ptr() as *mut i8
                                    as *const libc::c_void,
                                3 as i32 as u64,
                            ) });
                            (unsafe { (*(*tok).pb).bpos += 3 as i32 });
                            (unsafe { *((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize) =
                                '\u{0}' as i32 as i8 });
                        } else {
                            printbuf_memappend(
                                (unsafe { (*tok).pb }),
                                (unsafe { utf8_replacement_char.as_mut_ptr() }) as *mut i8,
                                3 as i32,
                            );
                        }
                    } else if (unsafe { (*tok).ucs_char }) < 0x10000 as i32 as u32 {
                        let mut unescaped_utf_1: [u8; 3] = [0; 3];
                        unescaped_utf_1[0 as i32 as usize] =
                            (0xe0 as i32 as u32 | (unsafe { (*tok).ucs_char }) >> 12 as i32) as u8;
                        unescaped_utf_1[1 as i32 as usize] = (0x80 as i32 as u32
                            | (unsafe { (*tok).ucs_char }) >> 6 as i32 & 0x3f as i32 as u32)
                            as u8;
                        unescaped_utf_1[2 as i32 as usize] =
                            (0x80 as i32 as u32 | (unsafe { (*tok).ucs_char }) & 0x3f as i32 as u32) as u8;
                        if (unsafe { (*(*tok).pb).size }) - (unsafe { (*(*tok).pb).bpos }) > 3 as i32 {
                            (unsafe { memcpy(
                                ((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize)
                                    as *mut libc::c_void,
                                unescaped_utf_1.as_mut_ptr() as *mut i8 as *const libc::c_void,
                                3 as i32 as u64,
                            ) });
                            (unsafe { (*(*tok).pb).bpos += 3 as i32 });
                            (unsafe { *((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize) =
                                '\u{0}' as i32 as i8 });
                        } else {
                            printbuf_memappend(
                                (unsafe { (*tok).pb }),
                                unescaped_utf_1.as_mut_ptr() as *mut i8,
                                3 as i32,
                            );
                        }
                    } else if (unsafe { (*tok).ucs_char }) < 0x110000 as i32 as u32 {
                        let mut unescaped_utf_2: [u8; 4] = [0; 4];
                        unescaped_utf_2[0 as i32 as usize] = (0xf0 as i32 as u32
                            | (unsafe { (*tok).ucs_char }) >> 18 as i32 & 0x7 as i32 as u32)
                            as u8;
                        unescaped_utf_2[1 as i32 as usize] = (0x80 as i32 as u32
                            | (unsafe { (*tok).ucs_char }) >> 12 as i32 & 0x3f as i32 as u32)
                            as u8;
                        unescaped_utf_2[2 as i32 as usize] = (0x80 as i32 as u32
                            | (unsafe { (*tok).ucs_char }) >> 6 as i32 & 0x3f as i32 as u32)
                            as u8;
                        unescaped_utf_2[3 as i32 as usize] =
                            (0x80 as i32 as u32 | (unsafe { (*tok).ucs_char }) & 0x3f as i32 as u32) as u8;
                        if (unsafe { (*(*tok).pb).size }) - (unsafe { (*(*tok).pb).bpos }) > 4 as i32 {
                            (unsafe { memcpy(
                                ((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize)
                                    as *mut libc::c_void,
                                unescaped_utf_2.as_mut_ptr() as *mut i8 as *const libc::c_void,
                                4 as i32 as u64,
                            ) });
                            (unsafe { (*(*tok).pb).bpos += 4 as i32 });
                            (unsafe { *((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize) =
                                '\u{0}' as i32 as i8 });
                        } else {
                            printbuf_memappend(
                                (unsafe { (*tok).pb }),
                                unescaped_utf_2.as_mut_ptr() as *mut i8,
                                4 as i32,
                            );
                        }
                    } else if (unsafe { (*(*tok).pb).size }) - (unsafe { (*(*tok).pb).bpos }) > 3 as i32 {
                        (unsafe { memcpy(
                            ((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize)
                                as *mut libc::c_void,
                            utf8_replacement_char.as_mut_ptr() as *mut i8 as *const libc::c_void,
                            3 as i32 as u64,
                        ) });
                        (unsafe { (*(*tok).pb).bpos += 3 as i32 });
                        (unsafe { *((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize) =
                            '\u{0}' as i32 as i8 });
                    } else {
                        printbuf_memappend(
                            (unsafe { (*tok).pb }),
                            (unsafe { utf8_replacement_char.as_mut_ptr() }) as *mut i8,
                            3 as i32,
                        );
                    }
                    current_block = 16438646162350502520;
                }
                match current_block {
                    14910428593596980134 => {},
                    _ => {
                        (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state =
                            (*((*tok).stack).offset((*tok).depth as isize)).saved_state });
                        current_block = 14910428593596980134;
                    },
                }
            },
            17501499944854094182 => {
                (unsafe { (*tok).ucs_char = 0 as i32 as u32 });
                (unsafe { (*tok).st_pos = 0 as i32 });
                (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state =
                    json_tokener_state_escape_unicode });
                current_block = 14910428593596980134;
            },
            902433882565356955 => {
                if c as i32 == 'b' as i32 {
                    if (unsafe { (*(*tok).pb).size }) - (unsafe { (*(*tok).pb).bpos }) > 1 as i32 {
                        (unsafe { memcpy(
                            ((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize)
                                as *mut libc::c_void,
                            b"\x08\0" as *const u8 as *const i8 as *const libc::c_void,
                            1 as i32 as u64,
                        ) });
                        (unsafe { (*(*tok).pb).bpos += 1 as i32 });
                        (unsafe { *((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize) =
                            '\u{0}' as i32 as i8 });
                    } else {
                        printbuf_memappend(
                            (unsafe { (*tok).pb }),
                            b"\x08\0" as *const u8 as *const i8,
                            1 as i32,
                        );
                    }
                } else if c as i32 == 'n' as i32 {
                    if (unsafe { (*(*tok).pb).size }) - (unsafe { (*(*tok).pb).bpos }) > 1 as i32 {
                        (unsafe { memcpy(
                            ((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize)
                                as *mut libc::c_void,
                            b"\n\0" as *const u8 as *const i8 as *const libc::c_void,
                            1 as i32 as u64,
                        ) });
                        (unsafe { (*(*tok).pb).bpos += 1 as i32 });
                        (unsafe { *((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize) =
                            '\u{0}' as i32 as i8 });
                    } else {
                        printbuf_memappend((unsafe { (*tok).pb }), b"\n\0" as *const u8 as *const i8, 1 as i32);
                    }
                } else if c as i32 == 'r' as i32 {
                    if (unsafe { (*(*tok).pb).size }) - (unsafe { (*(*tok).pb).bpos }) > 1 as i32 {
                        (unsafe { memcpy(
                            ((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize)
                                as *mut libc::c_void,
                            b"\r\0" as *const u8 as *const i8 as *const libc::c_void,
                            1 as i32 as u64,
                        ) });
                        (unsafe { (*(*tok).pb).bpos += 1 as i32 });
                        (unsafe { *((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize) =
                            '\u{0}' as i32 as i8 });
                    } else {
                        printbuf_memappend((unsafe { (*tok).pb }), b"\r\0" as *const u8 as *const i8, 1 as i32);
                    }
                } else if c as i32 == 't' as i32 {
                    if (unsafe { (*(*tok).pb).size }) - (unsafe { (*(*tok).pb).bpos }) > 1 as i32 {
                        (unsafe { memcpy(
                            ((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize)
                                as *mut libc::c_void,
                            b"\t\0" as *const u8 as *const i8 as *const libc::c_void,
                            1 as i32 as u64,
                        ) });
                        (unsafe { (*(*tok).pb).bpos += 1 as i32 });
                        (unsafe { *((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize) =
                            '\u{0}' as i32 as i8 });
                    } else {
                        printbuf_memappend((unsafe { (*tok).pb }), b"\t\0" as *const u8 as *const i8, 1 as i32);
                    }
                } else if c as i32 == 'f' as i32 {
                    if (unsafe { (*(*tok).pb).size }) - (unsafe { (*(*tok).pb).bpos }) > 1 as i32 {
                        (unsafe { memcpy(
                            ((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize)
                                as *mut libc::c_void,
                            b"\x0C\0" as *const u8 as *const i8 as *const libc::c_void,
                            1 as i32 as u64,
                        ) });
                        (unsafe { (*(*tok).pb).bpos += 1 as i32 });
                        (unsafe { *((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize) =
                            '\u{0}' as i32 as i8 });
                    } else {
                        printbuf_memappend(
                            (unsafe { (*tok).pb }),
                            b"\x0C\0" as *const u8 as *const i8,
                            1 as i32,
                        );
                    }
                }
                (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state =
                    (*((*tok).stack).offset((*tok).depth as isize)).saved_state });
                current_block = 14910428593596980134;
            },
            5851233167266824712 => {
                if (unsafe { (*(*tok).pb).size }) - (unsafe { (*(*tok).pb).bpos }) > 1 as i32 {
                    (unsafe { memcpy(
                        ((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize) as *mut libc::c_void,
                        &mut c as *mut i8 as *const libc::c_void,
                        1 as i32 as u64,
                    ) });
                    (unsafe { (*(*tok).pb).bpos += 1 as i32 });
                    (unsafe { *((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize) = '\u{0}' as i32 as i8 });
                } else {
                    printbuf_memappend((unsafe { (*tok).pb }), &mut c, 1 as i32);
                }
                (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state =
                    (*((*tok).stack).offset((*tok).depth as isize)).saved_state });
                current_block = 14910428593596980134;
            },
            13526015532137226550 => {
                loop {
                    if c as i32 == (unsafe { (*tok).quote_char }) as i32 {
                        if ((unsafe { (*(*tok).pb).size }) - (unsafe { (*(*tok).pb).bpos })) as i64
                            > (unsafe { str.offset_from(case_start_1) }) as i64
                        {
                            (unsafe { memcpy(
                                ((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize)
                                    as *mut libc::c_void,
                                case_start_1 as *const libc::c_void,
                                str.offset_from(case_start_1) as i64 as u64,
                            ) });
                            let (unsafe { ref mut fresh20 }) = (*(*tok).pb).bpos;
                            *fresh20 =
                                (*fresh20 as i64 + (unsafe { str.offset_from(case_start_1) }) as i64) as i32;
                            (unsafe { *((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize) =
                                '\u{0}' as i32 as i8 });
                        } else {
                            printbuf_memappend(
                                (unsafe { (*tok).pb }),
                                case_start_1,
                                (unsafe { str.offset_from(case_start_1) }) as i64 as i32,
                            );
                        }
                        let (unsafe { ref mut fresh21 }) =
                            (*(unsafe { ((*tok).stack).offset((*tok).depth as isize) })).current;
                        *fresh21 = json_object_new_string_len((unsafe { (*(*tok).pb).buf }), (unsafe { (*(*tok).pb).bpos }));
                        if (unsafe { ((*((*tok).stack).offset((*tok).depth as isize)).current) }).is_null() {
                            break 's_76;
                        }
                        (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).saved_state =
                            json_tokener_state_finish });
                        (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state =
                            json_tokener_state_eatws });
                        break;
                    } else if c as i32 == '\\' as i32 {
                        if ((unsafe { (*(*tok).pb).size }) - (unsafe { (*(*tok).pb).bpos })) as i64
                            > (unsafe { str.offset_from(case_start_1) }) as i64
                        {
                            (unsafe { memcpy(
                                ((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize)
                                    as *mut libc::c_void,
                                case_start_1 as *const libc::c_void,
                                str.offset_from(case_start_1) as i64 as u64,
                            ) });
                            let (unsafe { ref mut fresh22 }) = (*(*tok).pb).bpos;
                            *fresh22 =
                                (*fresh22 as i64 + (unsafe { str.offset_from(case_start_1) }) as i64) as i32;
                            (unsafe { *((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize) =
                                '\u{0}' as i32 as i8 });
                        } else {
                            printbuf_memappend(
                                (unsafe { (*tok).pb }),
                                case_start_1,
                                (unsafe { str.offset_from(case_start_1) }) as i64 as i32,
                            );
                        }
                        (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).saved_state =
                            json_tokener_state_string });
                        (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state =
                            json_tokener_state_string_escape });
                        break;
                    } else {
                        str = (unsafe { str.offset(1) });
                        let (unsafe { ref mut fresh23 }) = (*tok).char_offset;
                        *fresh23 += 1;
                        if !(c == 0
                            || (if (unsafe { (*tok).char_offset }) == len {
                                (if (unsafe { (*tok).depth }) == 0 as i32
                                    && (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state }) as u32
                                        == json_tokener_state_eatws as i32 as u32
                                    && (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).saved_state })
                                        as u32
                                        == json_tokener_state_finish as i32 as u32
                                {
                                    (unsafe { (*tok).err = json_tokener_success });
                                    0 as i32
                                } else {
                                    (unsafe { (*tok).err = json_tokener_continue });
                                    0 as i32
                                })
                            } else {
                                (if (unsafe { (*tok).flags }) & 0x10 as i32 != 0
                                    && json_tokener_validate_utf8((unsafe { *str }), borrow_mut(&mut nBytesp))
                                        == 0
                                {
                                    (unsafe { (*tok).err = json_tokener_error_parse_utf8_string });
                                    0 as i32
                                } else {
                                    c = (unsafe { *str });
                                    1 as i32
                                })
                            }) == 0)
                        {
                            continue;
                        }
                        if ((unsafe { (*(*tok).pb).size }) - (unsafe { (*(*tok).pb).bpos })) as i64
                            > (unsafe { str.offset_from(case_start_1) }) as i64
                        {
                            (unsafe { memcpy(
                                ((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize)
                                    as *mut libc::c_void,
                                case_start_1 as *const libc::c_void,
                                str.offset_from(case_start_1) as i64 as u64,
                            ) });
                            let (unsafe { ref mut fresh24 }) = (*(*tok).pb).bpos;
                            *fresh24 =
                                (*fresh24 as i64 + (unsafe { str.offset_from(case_start_1) }) as i64) as i32;
                            (unsafe { *((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize) =
                                '\u{0}' as i32 as i8 });
                        } else {
                            printbuf_memappend(
                                (unsafe { (*tok).pb }),
                                case_start_1,
                                (unsafe { str.offset_from(case_start_1) }) as i64 as i32,
                            );
                        }
                        break 's_76;
                    }
                }
                current_block = 14910428593596980134;
            },
            6662862405959679103 => {
                while c as i32 != '\n' as i32 {
                    str = (unsafe { str.offset(1) });
                    let (unsafe { ref mut fresh17 }) = (*tok).char_offset;
                    *fresh17 += 1;
                    if !(c == 0
                        || (if (unsafe { (*tok).char_offset }) == len {
                            (if (unsafe { (*tok).depth }) == 0 as i32
                                && (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state }) as u32
                                    == json_tokener_state_eatws as i32 as u32
                                && (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).saved_state })
                                    as u32
                                    == json_tokener_state_finish as i32 as u32
                            {
                                (unsafe { (*tok).err = json_tokener_success });
                                0 as i32
                            } else {
                                (unsafe { (*tok).err = json_tokener_continue });
                                0 as i32
                            })
                        } else {
                            (if (unsafe { (*tok).flags }) & 0x10 as i32 != 0
                                && json_tokener_validate_utf8((unsafe { *str }), borrow_mut(&mut nBytesp)) == 0
                            {
                                (unsafe { (*tok).err = json_tokener_error_parse_utf8_string });
                                0 as i32
                            } else {
                                c = (unsafe { *str });
                                1 as i32
                            })
                        }) == 0)
                    {
                        continue;
                    }
                    if ((unsafe { (*(*tok).pb).size }) - (unsafe { (*(*tok).pb).bpos })) as i64
                        > (unsafe { str.offset_from(case_start_0) }) as i64
                    {
                        (unsafe { memcpy(
                            ((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize)
                                as *mut libc::c_void,
                            case_start_0 as *const libc::c_void,
                            str.offset_from(case_start_0) as i64 as u64,
                        ) });
                        let (unsafe { ref mut fresh18 }) = (*(*tok).pb).bpos;
                        *fresh18 = (*fresh18 as i64 + (unsafe { str.offset_from(case_start_0) }) as i64) as i32;
                        (unsafe { *((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize) =
                            '\u{0}' as i32 as i8 });
                    } else {
                        printbuf_memappend(
                            (unsafe { (*tok).pb }),
                            case_start_0,
                            (unsafe { str.offset_from(case_start_0) }) as i64 as i32,
                        );
                    }
                    break 's_76;
                }
                if ((unsafe { (*(*tok).pb).size }) - (unsafe { (*(*tok).pb).bpos })) as i64
                    > (unsafe { str.offset_from(case_start_0) }) as i64
                {
                    (unsafe { memcpy(
                        ((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize) as *mut libc::c_void,
                        case_start_0 as *const libc::c_void,
                        str.offset_from(case_start_0) as i64 as u64,
                    ) });
                    let (unsafe { ref mut fresh19 }) = (*(*tok).pb).bpos;
                    *fresh19 = (*fresh19 as i64 + (unsafe { str.offset_from(case_start_0) }) as i64) as i32;
                    (unsafe { *((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize) = '\u{0}' as i32 as i8 });
                } else {
                    printbuf_memappend(
                        (unsafe { (*tok).pb }),
                        case_start_0,
                        (unsafe { str.offset_from(case_start_0) }) as i64 as i32,
                    );
                }
                (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state = json_tokener_state_eatws });
                current_block = 14910428593596980134;
            },
            18221534353613080499 => {
                while c as i32 != '*' as i32 {
                    str = (unsafe { str.offset(1) });
                    let (unsafe { ref mut fresh14 }) = (*tok).char_offset;
                    *fresh14 += 1;
                    if !(c == 0
                        || (if (unsafe { (*tok).char_offset }) == len {
                            (if (unsafe { (*tok).depth }) == 0 as i32
                                && (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state }) as u32
                                    == json_tokener_state_eatws as i32 as u32
                                && (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).saved_state })
                                    as u32
                                    == json_tokener_state_finish as i32 as u32
                            {
                                (unsafe { (*tok).err = json_tokener_success });
                                0 as i32
                            } else {
                                (unsafe { (*tok).err = json_tokener_continue });
                                0 as i32
                            })
                        } else {
                            (if (unsafe { (*tok).flags }) & 0x10 as i32 != 0
                                && json_tokener_validate_utf8((unsafe { *str }), borrow_mut(&mut nBytesp)) == 0
                            {
                                (unsafe { (*tok).err = json_tokener_error_parse_utf8_string });
                                0 as i32
                            } else {
                                c = (unsafe { *str });
                                1 as i32
                            })
                        }) == 0)
                    {
                        continue;
                    }
                    if ((unsafe { (*(*tok).pb).size }) - (unsafe { (*(*tok).pb).bpos })) as i64
                        > (unsafe { str.offset_from(case_start) }) as i64
                    {
                        (unsafe { memcpy(
                            ((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize)
                                as *mut libc::c_void,
                            case_start as *const libc::c_void,
                            str.offset_from(case_start) as i64 as u64,
                        ) });
                        let (unsafe { ref mut fresh15 }) = (*(*tok).pb).bpos;
                        *fresh15 = (*fresh15 as i64 + (unsafe { str.offset_from(case_start) }) as i64) as i32;
                        (unsafe { *((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize) =
                            '\u{0}' as i32 as i8 });
                    } else {
                        printbuf_memappend(
                            (unsafe { (*tok).pb }),
                            case_start,
                            (unsafe { str.offset_from(case_start) }) as i64 as i32,
                        );
                    }
                    break 's_76;
                }
                if ((unsafe { (*(*tok).pb).size }) - (unsafe { (*(*tok).pb).bpos })) as i64
                    > (unsafe { str.offset(1 as i32 as isize).offset_from(case_start) }) as i64
                {
                    (unsafe { memcpy(
                        ((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize) as *mut libc::c_void,
                        case_start as *const libc::c_void,
                        str.offset(1 as i32 as isize).offset_from(case_start) as i64 as u64,
                    ) });
                    let (unsafe { ref mut fresh16 }) = (*(*tok).pb).bpos;
                    *fresh16 = (*fresh16 as i64
                        + (unsafe { str.offset(1 as i32 as isize).offset_from(case_start) }) as i64)
                        as i32;
                    (unsafe { *((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize) = '\u{0}' as i32 as i8 });
                } else {
                    printbuf_memappend(
                        (unsafe { (*tok).pb }),
                        case_start,
                        (unsafe { str.offset(1 as i32 as isize).offset_from(case_start) }) as i64 as i32,
                    );
                }
                (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state =
                    json_tokener_state_comment_end });
                current_block = 14910428593596980134;
            },
            17917672080766325409 => {
                if c as i32 == '/' as i32 {
                    (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state =
                        json_tokener_state_comment_eol });
                } else {
                    (unsafe { (*tok).err = json_tokener_error_parse_comment });
                    break;
                }
                current_block = 6988365858197790817;
            },
            5965298418754406754 => {
                (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state = json_tokener_state_string });
                printbuf_reset((unsafe { (*tok).pb }));
                (unsafe { (*tok).quote_char = c });
                current_block = 14910428593596980134;
            },
            5524635047616071233 => {
                (unsafe { (*tok).err = json_tokener_error_parse_unexpected });
                break;
            },
            11162283542402356847 => {
                (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state = json_tokener_state_comment });
                current_block = 6988365858197790817;
            },
            15463013039495622015 => {
                (unsafe { (*tok).err = json_tokener_error_parse_string });
                break;
            },
            5151117443143562747 => {
                (unsafe { (*tok).err = json_tokener_error_parse_object_key_sep });
                break;
            },
            10945600426047866894 => {
                (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).saved_state =
                    json_tokener_state_finish });
                (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state = json_tokener_state_eatws });
                current_block = 14910428593596980134;
            },
            14575203612169622940 => {
                (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).saved_state =
                    json_tokener_state_object_value });
                (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state = json_tokener_state_eatws });
                current_block = 14910428593596980134;
            },
            7990025728955927862 => {
                (unsafe { (*tok).err = json_tokener_error_parse_unexpected });
                break;
            },
            17927085222977126794 => {
                json_object_array_shrink(
                    (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).current }),
                    0 as i32,
                );
                (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).saved_state =
                    json_tokener_state_finish });
                (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state = json_tokener_state_eatws });
                current_block = 14910428593596980134;
            },
            13823707972521062695 => {
                let (unsafe { ref mut fresh13 }) = (*tok).st_pos;
                *fresh13 += 1;
                current_block = 14910428593596980134;
            },
            6015864261243718670 => {
                let (unsafe { ref mut fresh29 }) = (*tok).st_pos;
                *fresh29 += 1;
                current_block = 14910428593596980134;
            },
            _ => {},
        }
        match current_block {
            6988365858197790817 => {
                if (unsafe { (*(*tok).pb).size }) - (unsafe { (*(*tok).pb).bpos }) > 1 as i32 {
                    (unsafe { memcpy(
                        ((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize) as *mut libc::c_void,
                        &mut c as *mut i8 as *const libc::c_void,
                        1 as i32 as u64,
                    ) });
                    (unsafe { (*(*tok).pb).bpos += 1 as i32 });
                    (unsafe { *((*(*tok).pb).buf).offset((*(*tok).pb).bpos as isize) = '\u{0}' as i32 as i8 });
                } else {
                    printbuf_memappend((unsafe { (*tok).pb }), &mut c, 1 as i32);
                }
            },
            _ => {},
        }
        str = (unsafe { str.offset(1) });
        let (unsafe { ref mut fresh44 }) = (*tok).char_offset;
        *fresh44 += 1;
        if c == 0 {
            break;
        }
    }
    if (unsafe { (*tok).flags }) & 0x10 as i32 != 0 && nBytes != 0 as i32 as u32 {
        (unsafe { (*tok).err = json_tokener_error_parse_utf8_string });
    }
    if c as i32 != 0
        && (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state }) as u32
            == json_tokener_state_finish as i32 as u32
        && (unsafe { (*tok).depth }) == 0 as i32
        && (unsafe { (*tok).flags }) & (0x1 as i32 | 0x2 as i32) == 0x1 as i32
    {
        (unsafe { (*tok).err = json_tokener_error_parse_unexpected });
    }
    if c == 0 {
        if (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).state }) as u32
            != json_tokener_state_finish as i32 as u32
            && (unsafe { (*((*tok).stack).offset((*tok).depth as isize)).saved_state }) as u32
                != json_tokener_state_finish as i32 as u32
        {
            (unsafe { (*tok).err = json_tokener_error_parse_eof });
        }
    }
    (unsafe { uselocale(oldlocale) });
    (unsafe { freelocale(newloc) });
    if (unsafe { (*tok).err }) as u32 == json_tokener_success as i32 as u32 {
        let mut ret: *mut crate::src::json_object::json_object =
            json_object_get((unsafe { (*((*tok).stack).offset((*tok).depth as isize)).current }));
        let mut ii: i32 = 0;
        ii = (unsafe { (*tok).depth });
        while ii >= 0 as i32 {
            json_tokener_reset_level(tok, ii);
            ii -= 1;
        }
        return ret;
    }
    return 0 as *mut json_object;
}
extern "C" fn json_tokener_validate_utf8<'a1>(c: i8, mut nBytes: Option<&'a1 mut u32>) -> i32 {
    let mut chr: u8 = c as u8;
    if *(borrow(&nBytes)).unwrap() == 0 as i32 as u32 {
        if chr as i32 >= 0x80 as i32 {
            if chr as i32 & 0xe0 as i32 == 0xc0 as i32 {
                *(borrow_mut(&mut nBytes)).unwrap() = 1 as i32 as u32;
            } else if chr as i32 & 0xf0 as i32 == 0xe0 as i32 {
                *(borrow_mut(&mut nBytes)).unwrap() = 2 as i32 as u32;
            } else if chr as i32 & 0xf8 as i32 == 0xf0 as i32 {
                *(borrow_mut(&mut nBytes)).unwrap() = 3 as i32 as u32;
            } else {
                return 0 as i32;
            }
        }
    } else {
        if chr as i32 & 0xc0 as i32 != 0x80 as i32 {
            return 0 as i32;
        }
        *(borrow_mut(&mut nBytes)).unwrap() = (*(borrow(&nBytes)).unwrap()).wrapping_sub(1);
    }
    return 1 as i32;
}
#[no_mangle]
pub extern "C" fn json_tokener_set_flags<'a1>(
    mut tok: *mut crate::src::apps::json_parse::json_tokener<'a1>,
    mut flags: i32,
) {
    (unsafe { (*tok).flags = flags });
}
#[no_mangle]
pub extern "C" fn json_tokener_get_parse_end<'a1>(
    mut tok: *mut crate::src::apps::json_parse::json_tokener<'a1>,
) -> u64 {
    if (unsafe { (*tok).char_offset }) >= 0 as i32 {
    } else {
        (unsafe { __assert_fail(
            b"tok->char_offset >= 0\0" as *const u8 as *const i8,
            b"/home/xial/json-c/json_tokener.c\0" as *const u8 as *const i8,
            1300 as i32 as u32,
            (*core::intrinsics::transmute::<&'_ [u8; 57], &'_ [i8; 57]>(
                b"size_t json_tokener_get_parse_end(struct json_tokener *)\0",
            ))
            .as_ptr(),
        ) });
    }
    return (unsafe { (*tok).char_offset }) as size_t;
}
extern "C" fn json_tokener_parse_double<'a1>(
    mut buf: *const i8,
    mut len: i32,
    mut retval: Option<&'a1 mut f64>,
) -> i32 {
    let mut end: *mut i8 = 0 as *mut i8;
    *(borrow_mut(&mut retval)).unwrap() = (unsafe { strtod(buf, &mut end) });
    if (unsafe { buf.offset(len as isize) }) == end as *const i8 {
        return 0 as i32;
    }
    return 1 as i32;
}
extern "C" fn run_static_initializers() {
    (unsafe { json_null_str_len =
        (::std::mem::size_of::<[i8; 5]>() as u64).wrapping_sub(1 as i32 as u64) as i32 });
    (unsafe { json_inf_str_len =
        (::std::mem::size_of::<[i8; 9]>() as u64).wrapping_sub(1 as i32 as u64) as u32 });
    (unsafe { json_nan_str_len =
        (::std::mem::size_of::<[i8; 4]>() as u64).wrapping_sub(1 as i32 as u64) as i32 });
    (unsafe { json_true_str_len =
        (::std::mem::size_of::<[i8; 5]>() as u64).wrapping_sub(1 as i32 as u64) as i32 });
    (unsafe { json_false_str_len =
        (::std::mem::size_of::<[i8; 6]>() as u64).wrapping_sub(1 as i32 as u64) as i32 });
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn() -> (); 1] = [run_static_initializers];
use crate::laertes_rt::*;
