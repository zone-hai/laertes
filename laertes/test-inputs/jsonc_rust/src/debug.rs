use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn vprintf(_: *const libc::c_char, _: ::std::ffi::VaList) -> libc::c_int;
    fn vsyslog(__pri: libc::c_int, __fmt: *const libc::c_char, __ap: ::std::ffi::VaList);
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type va_list = __builtin_va_list;
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
static mut _syslog: libc::c_int = 0 as libc::c_int;
static mut _debug: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn mc_set_debug(mut debug: libc::c_int) {
    _debug = debug;
}
#[no_mangle]
pub unsafe extern "C" fn mc_get_debug() -> libc::c_int {
    return _debug;
}
#[no_mangle]
pub unsafe extern "C" fn mc_set_syslog(mut syslog: libc::c_int) {
    _syslog = syslog;
}
#[no_mangle]
pub unsafe extern "C" fn mc_debug(mut msg: *const libc::c_char, mut args: ...) {
    let mut ap: ::std::ffi::VaListImpl;
    if _debug != 0 {
        ap = args.clone();
        if _syslog != 0 {
            vsyslog(7 as libc::c_int, msg, ap.as_va_list());
        } else {
            vprintf(msg, ap.as_va_list());
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn mc_error(mut msg: *const libc::c_char, mut args: ...) {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    if _syslog != 0 {
        vsyslog(3 as libc::c_int, msg, ap.as_va_list());
    } else {
        vfprintf(stderr, msg, ap.as_va_list());
    };
}
#[no_mangle]
pub unsafe extern "C" fn mc_info(mut msg: *const libc::c_char, mut args: ...) {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    if _syslog != 0 {
        vsyslog(6 as libc::c_int, msg, ap.as_va_list());
    } else {
        vfprintf(stderr, msg, ap.as_va_list());
    };
}
