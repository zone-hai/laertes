use :: libc;
extern "C" {
    fn malloc(_: u64) -> *mut core::ffi::c_void;
    fn realloc(_: *mut core::ffi::c_void, _: u64) -> *mut core::ffi::c_void;
    fn free(__ptr: *mut core::ffi::c_void);
    fn bsearch(
        __key: *const core::ffi::c_void,
        __base: *const core::ffi::c_void,
        __nmemb: u64,
        __size: u64,
        __compar: Option<
            unsafe extern "C" fn(_: *const core::ffi::c_void, _: *const core::ffi::c_void) -> i32,
        >,
    ) -> *mut core::ffi::c_void;
    fn qsort(
        __base: *mut core::ffi::c_void,
        __nmemb: u64,
        __size: u64,
        __compar: Option<
            unsafe extern "C" fn(_: *const core::ffi::c_void, _: *const core::ffi::c_void) -> i32,
        >,
    );
    fn memmove(
        _: *mut core::ffi::c_void,
        _: *const core::ffi::c_void,
        _: u64,
    ) -> *mut core::ffi::c_void;
    fn memset(_: *mut core::ffi::c_void, _: i32, _: u64) -> *mut core::ffi::c_void;
}
pub type size_t = u64;
pub type __compar_fn_t =
    Option<unsafe extern "C" fn(_: *const core::ffi::c_void, _: *const core::ffi::c_void) -> i32>;
pub type array_list_free_fn = unsafe extern "C" fn(_: *mut core::ffi::c_void) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct array_list {
    pub array: *mut *mut core::ffi::c_void,
    pub length: u64,
    pub size: u64,
    pub free_fn: Option<unsafe extern "C" fn(_: *mut core::ffi::c_void) -> ()>,
}
impl array_list {
    pub const fn new() -> Self {
        array_list {
            array: (0 as *mut *mut core::ffi::c_void),
            length: 0,
            size: 0,
            free_fn: None,
        }
    }
}
impl std::default::Default for array_list {
    fn default() -> Self {
        array_list::new()
    }
}
#[no_mangle]
pub extern "C" fn array_list_new(
    mut free_fn: Option<unsafe extern "C" fn(_: *mut core::ffi::c_void) -> ()>,
) -> *mut crate::src::arraylist::array_list {
    return array_list_new2(free_fn, 32 as i32);
}
#[no_mangle]
pub extern "C" fn array_list_new2(
    mut free_fn: Option<unsafe extern "C" fn(_: *mut core::ffi::c_void) -> ()>,
    mut initial_size: i32,
) -> *mut crate::src::arraylist::array_list {
    let mut arr: *mut crate::src::arraylist::array_list = 0 as *mut array_list;
    if initial_size < 0 as i32
        || initial_size as size_t
            >= (9223372036854775807 as i64 as u64)
                .wrapping_mul(2 as u64)
                .wrapping_add(1 as u64)
                .wrapping_div(::std::mem::size_of::<*mut libc::c_void>() as u64)
    {
        return 0 as *mut array_list;
    }
    arr = (unsafe { malloc(::std::mem::size_of::<array_list>() as u64) }) as *mut array_list;
    if arr.is_null() {
        return 0 as *mut array_list;
    }
    (unsafe { (*arr).size = initial_size as size_t });
    (unsafe { (*arr).length = 0 as i32 as size_t });
    let (unsafe { ref mut fresh0 }) = (*arr).free_fn;
    *fresh0 = free_fn;
    let (unsafe { ref mut fresh1 }) = (*arr).array;
    *fresh1 = (unsafe { malloc(((*arr).size).wrapping_mul(::std::mem::size_of::<*mut libc::c_void>() as u64)) })
        as *mut *mut libc::c_void;
    if (*fresh1).is_null() {
        (unsafe { free(arr as *mut libc::c_void) });
        return 0 as *mut array_list;
    }
    return arr;
}
#[no_mangle]
pub extern "C" fn array_list_free(mut arr: *mut crate::src::arraylist::array_list) {
    let mut i: u64 = 0;
    i = 0 as i32 as size_t;
    while i < (unsafe { (*arr).length }) {
        if !(unsafe { (*((*arr).array).offset(i as isize)) }).is_null() {
            (unsafe { ((*arr).free_fn).expect("non-null function pointer")(
                *((*arr).array).offset(i as isize),
            ) });
        }
        i = i.wrapping_add(1);
    }
    (unsafe { free((*arr).array as *mut libc::c_void) });
    (unsafe { free(arr as *mut libc::c_void) });
}
#[no_mangle]
pub extern "C" fn array_list_get_idx(
    mut arr: *mut crate::src::arraylist::array_list,
    mut i: u64,
) -> *mut core::ffi::c_void {
    if i >= (unsafe { (*arr).length }) {
        return 0 as *mut libc::c_void;
    }
    return (unsafe { *((*arr).array).offset(i as isize) });
}
extern "C" fn array_list_expand_internal(
    mut arr: *mut crate::src::arraylist::array_list,
    mut max: u64,
) -> i32 {
    let mut t: *mut core::ffi::c_void = (0 as *mut core::ffi::c_void);
    let mut new_size: u64 = 0;
    if max < (unsafe { (*arr).size }) {
        return 0 as i32;
    }
    if (unsafe { (*arr).size })
        >= (9223372036854775807 as i64 as u64)
            .wrapping_mul(2 as u64)
            .wrapping_add(1 as u64)
            .wrapping_div(2 as i32 as u64)
    {
        new_size = max;
    } else {
        new_size = (unsafe { (*arr).size }) << 1 as i32;
        if new_size < max {
            new_size = max;
        }
    }
    if new_size
        > (!(0 as i32 as size_t)).wrapping_div(::std::mem::size_of::<*mut libc::c_void>() as u64)
    {
        return -(1 as i32);
    }
    t = (unsafe { realloc(
        (*arr).array as *mut libc::c_void,
        new_size.wrapping_mul(::std::mem::size_of::<*mut libc::c_void>() as u64),
    ) });
    if t.is_null() {
        return -(1 as i32);
    }
    let (unsafe { ref mut fresh2 }) = (*arr).array;
    *fresh2 = t as *mut *mut libc::c_void;
    (unsafe { (*arr).size = new_size });
    return 0 as i32;
}
#[no_mangle]
pub extern "C" fn array_list_shrink(
    mut arr: *mut crate::src::arraylist::array_list,
    mut empty_slots: u64,
) -> i32 {
    let mut t: *mut core::ffi::c_void = (0 as *mut core::ffi::c_void);
    let mut new_size: u64 = 0;
    if empty_slots
        >= (9223372036854775807 as i64 as u64)
            .wrapping_mul(2 as u64)
            .wrapping_add(1 as u64)
            .wrapping_div(::std::mem::size_of::<*mut libc::c_void>() as u64)
            .wrapping_sub((unsafe { (*arr).length }))
    {
        return -(1 as i32);
    }
    new_size = (unsafe { ((*arr).length) }).wrapping_add(empty_slots);
    if new_size == (unsafe { (*arr).size }) {
        return 0 as i32;
    }
    if new_size > (unsafe { (*arr).size }) {
        return array_list_expand_internal(arr, new_size);
    }
    if new_size == 0 as i32 as u64 {
        new_size = 1 as i32 as size_t;
    }
    t = (unsafe { realloc(
        (*arr).array as *mut libc::c_void,
        new_size.wrapping_mul(::std::mem::size_of::<*mut libc::c_void>() as u64),
    ) });
    if t.is_null() {
        return -(1 as i32);
    }
    let (unsafe { ref mut fresh3 }) = (*arr).array;
    *fresh3 = t as *mut *mut libc::c_void;
    (unsafe { (*arr).size = new_size });
    return 0 as i32;
}
#[no_mangle]
pub extern "C" fn array_list_put_idx(
    mut arr: *mut crate::src::arraylist::array_list,
    mut idx: u64,
    mut data: *mut core::ffi::c_void,
) -> i32 {
    if idx
        > (9223372036854775807 as i64 as u64)
            .wrapping_mul(2 as u64)
            .wrapping_add(1 as u64)
            .wrapping_sub(1 as i32 as u64)
    {
        return -(1 as i32);
    }
    if array_list_expand_internal(arr, idx.wrapping_add(1 as i32 as u64)) != 0 {
        return -(1 as i32);
    }
    if idx < (unsafe { (*arr).length }) && !(unsafe { (*((*arr).array).offset(idx as isize)) }).is_null() {
        (unsafe { ((*arr).free_fn).expect("non-null function pointer")(*((*arr).array).offset(idx as isize)) });
    }
    let (unsafe { ref mut fresh4 }) = *(unsafe { ((*arr).array).offset(idx as isize) });
    *fresh4 = data;
    if idx > (unsafe { (*arr).length }) {
        (unsafe { memset(
            ((*arr).array).offset((*arr).length as isize) as *mut libc::c_void,
            0 as i32,
            idx.wrapping_sub((*arr).length)
                .wrapping_mul(::std::mem::size_of::<*mut libc::c_void>() as u64),
        ) });
    }
    if (unsafe { (*arr).length }) <= idx {
        (unsafe { (*arr).length = idx.wrapping_add(1 as i32 as u64) });
    }
    return 0 as i32;
}
#[no_mangle]
pub extern "C" fn array_list_add(
    mut arr: *mut crate::src::arraylist::array_list,
    mut data: *mut core::ffi::c_void,
) -> i32 {
    let mut idx: u64 = (unsafe { (*arr).length });
    if idx
        > (9223372036854775807 as i64 as u64)
            .wrapping_mul(2 as u64)
            .wrapping_add(1 as u64)
            .wrapping_sub(1 as i32 as u64)
    {
        return -(1 as i32);
    }
    if array_list_expand_internal(arr, idx.wrapping_add(1 as i32 as u64)) != 0 {
        return -(1 as i32);
    }
    let (unsafe { ref mut fresh5 }) = *(unsafe { ((*arr).array).offset(idx as isize) });
    *fresh5 = data;
    let (unsafe { ref mut fresh6 }) = (*arr).length;
    *fresh6 = (*fresh6).wrapping_add(1);
    return 0 as i32;
}
#[no_mangle]
pub extern "C" fn array_list_sort(
    mut arr: *mut crate::src::arraylist::array_list,
    mut compar: Option<
        unsafe extern "C" fn(_: *const core::ffi::c_void, _: *const core::ffi::c_void) -> i32,
    >,
) {
    (unsafe { qsort(
        (*arr).array as *mut libc::c_void,
        (*arr).length,
        ::std::mem::size_of::<*mut libc::c_void>() as u64,
        compar,
    ) });
}
#[no_mangle]
pub extern "C" fn array_list_bsearch(
    mut key: *mut *const core::ffi::c_void,
    mut arr: *mut crate::src::arraylist::array_list,
    mut compar: Option<
        unsafe extern "C" fn(_: *const core::ffi::c_void, _: *const core::ffi::c_void) -> i32,
    >,
) -> *mut core::ffi::c_void {
    return (unsafe { bsearch(
        key as *const libc::c_void,
        (*arr).array as *const libc::c_void,
        (*arr).length,
        ::std::mem::size_of::<*mut libc::c_void>() as u64,
        compar,
    ) });
}
#[no_mangle]
pub extern "C" fn array_list_length(mut arr: *mut crate::src::arraylist::array_list) -> u64 {
    return (unsafe { (*arr).length });
}
#[no_mangle]
pub extern "C" fn array_list_del_idx(
    mut arr: *mut crate::src::arraylist::array_list,
    mut idx: u64,
    mut count: u64,
) -> i32 {
    let mut i: u64 = 0;
    let mut stop: u64 = 0;
    if idx
        > (9223372036854775807 as i64 as u64)
            .wrapping_mul(2 as u64)
            .wrapping_add(1 as u64)
            .wrapping_sub(count)
    {
        return -(1 as i32);
    }
    stop = idx.wrapping_add(count);
    if idx >= (unsafe { (*arr).length }) || stop > (unsafe { (*arr).length }) {
        return -(1 as i32);
    }
    i = idx;
    while i < stop {
        if !(unsafe { (*((*arr).array).offset(i as isize)) }).is_null() {
            (unsafe { ((*arr).free_fn).expect("non-null function pointer")(
                *((*arr).array).offset(i as isize),
            ) });
        }
        i = i.wrapping_add(1);
    }
    (unsafe { memmove(
        ((*arr).array).offset(idx as isize) as *mut libc::c_void,
        ((*arr).array).offset(stop as isize) as *const libc::c_void,
        ((*arr).length)
            .wrapping_sub(stop)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_void>() as u64),
    ) });
    let (unsafe { ref mut fresh7 }) = (*arr).length;
    *fresh7 = (*fresh7 as u64).wrapping_sub(count) as size_t as size_t;
    return 0 as i32;
}
use crate::laertes_rt::*;
