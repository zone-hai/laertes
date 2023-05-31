use ::libc;
extern "C" {
    
    
    
    static mut stderr: *mut FILE;
    fn vfprintf(
        _: *mut FILE,
        _: *const i8,
        _: ::std::ffi::VaList,
    ) -> i32;
    fn vprintf(_: *const i8, _: ::std::ffi::VaList) -> i32;
    fn vsyslog(__pri: i32, __fmt: *const i8, __ap: ::std::ffi::VaList);
}
pub use crate::src::json_visit::_IO_marker;
pub use crate::src::tests::test1::_IO_codecvt;
pub use crate::src::tests::test1::_IO_wide_data;
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: u32,
    pub fp_offset: u32,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type va_list = __builtin_va_list;
pub type size_t = crate::src::apps::json_parse::size_t;
pub type __off_t = crate::src::apps::json_parse::__off_t;
pub type __off64_t = crate::src::apps::json_parse::__off64_t;
// #[derive(Copy, Clone)]

pub type _IO_FILE = crate::src::apps::json_parse::_IO_FILE;
pub type _IO_lock_t = crate::src::apps::json_parse::_IO_lock_t;
pub type FILE = crate::src::apps::json_parse::FILE;
static mut _syslog: i32 = 0 as i32;
static mut _debug: i32 = 0 as i32;
#[no_mangle]
pub unsafe extern "C" fn mc_set_debug(mut debug: i32) {
    _debug = debug;
}
#[no_mangle]
pub unsafe extern "C" fn mc_get_debug() -> i32 {
    return _debug;
}
#[no_mangle]
pub unsafe extern "C" fn mc_set_syslog(mut syslog: i32) {
    _syslog = syslog;
}
#[no_mangle]
pub unsafe extern "C" fn mc_debug(mut msg: *const i8, mut args: ...) {
    let mut ap: ::std::ffi::VaListImpl;
    if _debug != 0 {
        ap = args.clone();
        if _syslog != 0 {
            vsyslog(7 as i32, msg, ap.as_va_list());
        } else {
            vprintf(msg, ap.as_va_list());
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn mc_error(mut msg: *const i8, mut args: ...) {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    if _syslog != 0 {
        vsyslog(3 as i32, msg, ap.as_va_list());
    } else {
        vfprintf(stderr, msg, ap.as_va_list());
    };
}
#[no_mangle]
pub unsafe extern "C" fn mc_info(mut msg: *const i8, mut args: ...) {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    if _syslog != 0 {
        vsyslog(6 as i32, msg, ap.as_va_list());
    } else {
        vfprintf(stderr, msg, ap.as_va_list());
    };
}
