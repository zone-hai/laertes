use ::libc;
extern "C" {
    
    
    
    
    
    
    
    
    
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: u64,
    ) -> *mut libc::c_void;
    fn __errno_location() -> *mut i32;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strrchr(_: *const i8, _: i32) -> *mut i8;
    fn strstr(_: *const i8, _: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn strdup(_: *const i8) -> *mut i8;
    fn vasprintf(
        __ptr: *mut *mut i8,
        __f: *const i8,
        __arg: ::std::ffi::VaList,
    ) -> i32;
    fn strtol(
        _: *const i8,
        _: *mut *mut i8,
        _: i32,
    ) -> i64;
    fn free(__ptr: *mut libc::c_void);
}
pub use crate::src::json_object::json_object_array_add;
pub use crate::src::json_object::json_object_array_get_idx;
pub use crate::src::json_object::json_object_array_length;
pub use crate::src::json_object::json_object_array_put_idx;
pub use crate::src::json_object::json_object_is_type;
pub use crate::src::json_object::json_object_object_add;
pub use crate::src::json_object::json_object_object_get_ex;
pub use crate::src::json_object::json_object_put;
pub use crate::src::json_object::json_object;
pub type __builtin_va_list = crate::src::debug::__builtin_va_list;
// #[derive(Copy, Clone)]

pub type __va_list_tag = crate::src::debug::__va_list_tag;
pub type json_bool = crate::src::json_object::json_bool;
pub type json_type = crate::src::json_object::json_type;
pub const json_type_string: json_type = 6;
pub const json_type_array: json_type = 5;
pub const json_type_object: json_type = 4;
pub const json_type_int: json_type = 3;
pub const json_type_double: json_type = 2;
pub const json_type_boolean: json_type = 1;
pub const json_type_null: json_type = 0;
pub type size_t = crate::src::apps::json_parse::size_t;
pub type va_list = crate::src::debug::va_list;
unsafe extern "C" fn string_replace_all_occurrences_with_char(
    mut s: *mut i8,
    mut occur: *const i8,
    mut repl_char: i8,
) {
    let mut slen: size_t = strlen(s);
    let mut skip: size_t = (strlen(occur))
        .wrapping_sub(1 as i32 as u64);
    let mut p: *mut i8 = s;
    loop {
        p = strstr(p, occur);
        if p.is_null() {
            break;
        }
        *p = repl_char;
        p = p.offset(1);
        slen = (slen as u64).wrapping_sub(skip) as size_t as size_t;
        memmove(
            p as *mut libc::c_void,
            p.offset(skip as isize) as *const libc::c_void,
            slen
                .wrapping_sub(p.offset_from(s) as i64 as u64)
                .wrapping_add(1 as i32 as u64),
        );
    };
}
unsafe extern "C" fn is_valid_index(
    mut jo: *mut json_object,
    mut path: *const i8,
    mut idx: *mut size_t,
) -> i32 {
    let mut i: size_t = 0;
    let mut len: size_t = strlen(path);
    let mut idx_val: i64 = -(1 as i32) as i64;
    if len == 1 as i32 as u64 {
        if *path.offset(0 as i32 as isize) as i32 >= '0' as i32
            && *path.offset(0 as i32 as isize) as i32 <= '9' as i32
        {
            *idx = (*path.offset(0 as i32 as isize) as i32 - '0' as i32)
                as size_t;
        } else {
            *__errno_location() = 22 as i32;
            return 0 as i32;
        }
    } else {
        if *path.offset(0 as i32 as isize) as i32 == '0' as i32 {
            *__errno_location() = 22 as i32;
            return 0 as i32;
        }
        i = 0 as i32 as size_t;
        while i < len {
            if !(*path.offset(i as isize) as i32 >= '0' as i32
                && *path.offset(i as isize) as i32 <= '9' as i32)
            {
                *__errno_location() = 22 as i32;
                return 0 as i32;
            }
            i = i.wrapping_add(1);
        }
        idx_val = strtol(path, 0 as *mut *mut i8, 10 as i32);
        if idx_val < 0 as i32 as i64 {
            *__errno_location() = 22 as i32;
            return 0 as i32;
        }
        *idx = idx_val as size_t;
    }
    len = json_object_array_length(jo);
    if *idx >= len {
        *__errno_location() = 2 as i32;
        return 0 as i32;
    }
    return 1 as i32;
}
unsafe extern "C" fn json_pointer_get_single_path(
    mut obj: *mut json_object,
    mut path: *mut i8,
    mut value: *mut *mut json_object,
) -> i32 {
    if json_object_is_type(obj, json_type_array) != 0 {
        let mut idx: size_t = 0;
        if is_valid_index(obj, path, &mut idx) == 0 {
            return -(1 as i32);
        }
        obj = json_object_array_get_idx(obj, idx);
        if !obj.is_null() {
            if !value.is_null() {
                *value = obj;
            }
            return 0 as i32;
        }
        *__errno_location() = 2 as i32;
        return -(1 as i32);
    }
    string_replace_all_occurrences_with_char(
        path,
        b"~1\0" as *const u8 as *const i8,
        '/' as i32 as i8,
    );
    string_replace_all_occurrences_with_char(
        path,
        b"~0\0" as *const u8 as *const i8,
        '~' as i32 as i8,
    );
    if json_object_object_get_ex(obj, path, value) == 0 {
        *__errno_location() = 2 as i32;
        return -(1 as i32);
    }
    return 0 as i32;
}
unsafe extern "C" fn json_pointer_set_single_path(
    mut parent: *mut json_object,
    mut path: *const i8,
    mut value: *mut json_object,
) -> i32 {
    if json_object_is_type(parent, json_type_array) != 0 {
        let mut idx: size_t = 0;
        if *path.offset(0 as i32 as isize) as i32 == '-' as i32
            && *path.offset(1 as i32 as isize) as i32 == '\u{0}' as i32
        {
            return json_object_array_add(parent, value);
        }
        if is_valid_index(parent, path, &mut idx) == 0 {
            return -(1 as i32);
        }
        return json_object_array_put_idx(parent, idx, value);
    }
    if json_object_is_type(parent, json_type_object) != 0 {
        return json_object_object_add(parent, path, value);
    }
    *__errno_location() = 2 as i32;
    return -(1 as i32);
}
unsafe extern "C" fn json_pointer_get_recursive(
    mut obj: *mut json_object,
    mut path: *mut i8,
    mut value: *mut *mut json_object,
) -> i32 {
    let mut endp: *mut i8 = 0 as *mut i8;
    let mut rc: i32 = 0;
    if *path.offset(0 as i32 as isize) as i32 != '/' as i32 {
        *__errno_location() = 22 as i32;
        return -(1 as i32);
    }
    path = path.offset(1);
    endp = strchr(path, '/' as i32);
    if !endp.is_null() {
        *endp = '\u{0}' as i32 as i8;
    }
    rc = json_pointer_get_single_path(obj, path, &mut obj);
    if rc != 0 {
        return rc;
    }
    if !endp.is_null() {
        *endp = '/' as i32 as i8;
        return json_pointer_get_recursive(obj, endp, value);
    }
    if !value.is_null() {
        *value = obj;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn json_pointer_get(
    mut obj: *mut json_object,
    mut path: *const i8,
    mut res: *mut *mut json_object,
) -> i32 {
    let mut path_copy: *mut i8 = 0 as *mut i8;
    let mut rc: i32 = 0;
    if obj.is_null() || path.is_null() {
        *__errno_location() = 22 as i32;
        return -(1 as i32);
    }
    if *path.offset(0 as i32 as isize) as i32 == '\u{0}' as i32 {
        if !res.is_null() {
            *res = obj;
        }
        return 0 as i32;
    }
    path_copy = strdup(path);
    if path_copy.is_null() {
        *__errno_location() = 12 as i32;
        return -(1 as i32);
    }
    rc = json_pointer_get_recursive(obj, path_copy, res);
    free(path_copy as *mut libc::c_void);
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn json_pointer_getf(
    mut obj: *mut json_object,
    mut res: *mut *mut json_object,
    mut path_fmt: *const i8,
    mut args: ...
) -> i32 {
    let mut path_copy: *mut i8 = 0 as *mut i8;
    let mut rc: i32 = 0 as i32;
    let mut args_0: ::std::ffi::VaListImpl;
    if obj.is_null() || path_fmt.is_null() {
        *__errno_location() = 22 as i32;
        return -(1 as i32);
    }
    args_0 = args.clone();
    rc = vasprintf(&mut path_copy, path_fmt, args_0.as_va_list());
    if rc < 0 as i32 {
        return rc;
    }
    if *path_copy.offset(0 as i32 as isize) as i32 == '\u{0}' as i32 {
        if !res.is_null() {
            *res = obj;
        }
    } else {
        rc = json_pointer_get_recursive(obj, path_copy, res);
    }
    free(path_copy as *mut libc::c_void);
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn json_pointer_set(
    mut obj: *mut *mut json_object,
    mut path: *const i8,
    mut value: *mut json_object,
) -> i32 {
    let mut endp: *const i8 = 0 as *const i8;
    let mut path_copy: *mut i8 = 0 as *mut i8;
    let mut set: *mut json_object = 0 as *mut json_object;
    let mut rc: i32 = 0;
    if obj.is_null() || path.is_null() {
        *__errno_location() = 22 as i32;
        return -(1 as i32);
    }
    if *path.offset(0 as i32 as isize) as i32 == '\u{0}' as i32 {
        json_object_put(*obj);
        *obj = value;
        return 0 as i32;
    }
    if *path.offset(0 as i32 as isize) as i32 != '/' as i32 {
        *__errno_location() = 22 as i32;
        return -(1 as i32);
    }
    endp = strrchr(path, '/' as i32);
    if endp == path {
        path = path.offset(1);
        return json_pointer_set_single_path(*obj, path, value);
    }
    path_copy = strdup(path);
    if path_copy.is_null() {
        *__errno_location() = 12 as i32;
        return -(1 as i32);
    }
    *path_copy
        .offset(
            endp.offset_from(path) as i64 as isize,
        ) = '\u{0}' as i32 as i8;
    rc = json_pointer_get_recursive(*obj, path_copy, &mut set);
    free(path_copy as *mut libc::c_void);
    if rc != 0 {
        return rc;
    }
    endp = endp.offset(1);
    return json_pointer_set_single_path(set, endp, value);
}
#[no_mangle]
pub unsafe extern "C" fn json_pointer_setf(
    mut obj: *mut *mut json_object,
    mut value: *mut json_object,
    mut path_fmt: *const i8,
    mut args: ...
) -> i32 {
    let mut current_block: u64;
    let mut endp: *mut i8 = 0 as *mut i8;
    let mut path_copy: *mut i8 = 0 as *mut i8;
    let mut set: *mut json_object = 0 as *mut json_object;
    let mut args_0: ::std::ffi::VaListImpl;
    let mut rc: i32 = 0 as i32;
    if obj.is_null() || path_fmt.is_null() {
        *__errno_location() = 22 as i32;
        return -(1 as i32);
    }
    args_0 = args.clone();
    rc = vasprintf(&mut path_copy, path_fmt, args_0.as_va_list());
    if rc < 0 as i32 {
        return rc;
    }
    if *path_copy.offset(0 as i32 as isize) as i32 == '\u{0}' as i32 {
        json_object_put(*obj);
        *obj = value;
    } else if *path_copy.offset(0 as i32 as isize) as i32 != '/' as i32 {
        *__errno_location() = 22 as i32;
        rc = -(1 as i32);
    } else {
        endp = strrchr(path_copy, '/' as i32);
        if endp == path_copy {
            set = *obj;
            current_block = 1863480813282067938;
        } else {
            *endp = '\u{0}' as i32 as i8;
            rc = json_pointer_get_recursive(*obj, path_copy, &mut set);
            if rc != 0 {
                current_block = 14315698657705028467;
            } else {
                current_block = 1863480813282067938;
            }
        }
        match current_block {
            14315698657705028467 => {}
            _ => {
                endp = endp.offset(1);
                rc = json_pointer_set_single_path(set, endp, value);
            }
        }
    }
    free(path_copy as *mut libc::c_void);
    return rc;
}
