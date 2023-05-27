#![feature(linkage)]
#![feature(extern_types)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_mut)]
#![allow(unused_assignments)]
#![allow(non_snake_case)]

extern "C" {
    pub type __sFILEX;
    pub type FILE;
    static mut __stdinp: *mut FILE;
    static mut __stdoutp: *mut FILE;
    static mut __stderrp: *mut FILE;
    fn free(_: *mut std::os::raw::c_void);
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    fn malloc(_: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    fn getenv(_: *const std::os::raw::c_char) -> *mut std::os::raw::c_char;
    fn strcmp(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> std::os::raw::c_int;
    fn strcpy(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char)
     -> *mut std::os::raw::c_char;
    fn strlen(_: *const std::os::raw::c_char) -> std::os::raw::c_ulong;
    fn strstr(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char)
     -> *mut std::os::raw::c_char;
}

pub type __uint16_t = std::os::raw::c_ushort;
pub type __int32_t = std::os::raw::c_int;
pub type __uint32_t = std::os::raw::c_uint;
pub type __int64_t = std::os::raw::c_longlong;
pub type __uint64_t = std::os::raw::c_ulonglong;
pub type __darwin_size_t = std::os::raw::c_ulong;
pub type size_t = __darwin_size_t;

pub type Char = std::os::raw::c_char;
pub type Bool = std::os::raw::c_uchar;
pub type UChar = std::os::raw::c_uchar;
pub type Int32 = std::os::raw::c_int;
pub type UInt32 = std::os::raw::c_uint;
pub type UInt16 = std::os::raw::c_ushort;
pub type Int16 = std::os::raw::c_short;
pub type IntNative = std::os::raw::c_int;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Cell {
    pub name: *mut Char,
    pub link: *mut Cell,
}

/*---------------------------------------------------*/
/*--- Misc (file handling) data decls             ---*/
/*---------------------------------------------------*/
static mut verbosity: Int32 = 0;
static mut keepInputFiles: Bool = 0;
static mut smallMode: Bool = 0;
static mut forceOverwrite: Bool = 0;
static mut noisy: Bool = 0;
static mut numFileNames: Int32 = 0;
static mut exitValue: Int32 = 0;
static mut opMode: Int32 = 0;
static mut srcMode: Int32 = 0;
static mut longestFileName: Int32 = 0;
static mut tmpName: [Char; 1034] = [0; 1034];
static mut progName: *mut Char = 0 as *const Char as *mut Char;
/*---------------------------------------------*/
unsafe extern "C" fn myMalloc(mut n: Int32) -> *mut std::os::raw::c_void {
    let mut p: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;
    p = malloc(n as size_t);
    if p.is_null() { panic!(); }
    return p;
}
/*---------------------------------------------*/
unsafe extern "C" fn mkCell() -> *mut Cell {
    let mut c: *mut Cell = 0 as *mut Cell;
    c =
        myMalloc(::std::mem::size_of::<Cell>() as std::os::raw::c_ulong as Int32) as
            *mut Cell;
    (*c).name = 0 as *mut Char;
    (*c).link = 0 as *mut Cell;
    return c;
}
/*---------------------------------------------*/
unsafe extern "C" fn snocString(mut root: *mut Cell, mut name: *mut Char)
 -> *mut Cell {
    if root.is_null() {
        let mut tmp: *mut Cell = mkCell();
        (*tmp).name =
            myMalloc((5 as std::os::raw::c_int as
                          std::os::raw::c_ulong).wrapping_add(strlen(name)) as Int32)
                as *mut Char;
        strcpy((*tmp).name, name);
        return tmp
    } else {
        let mut tmp_0: *mut Cell = root;
        while !(*tmp_0).link.is_null() { tmp_0 = (*tmp_0).link }
        (*tmp_0).link = snocString((*tmp_0).link, name);
        return root
    };
}
/*---------------------------------------------*/
unsafe extern "C" fn addFlagsFromEnvVar(mut argList: *mut *mut Cell,
                                        mut varName: *mut Char) {
    *argList = snocString(*argList, varName);
}
unsafe fn main_0(mut argc: IntNative, mut argv: *mut *mut Char) -> IntNative {
    let mut i: Int32 = 0;
    let mut j: Int32 = 0;
    let mut argList: *mut Cell = 0 as *mut Cell;
    let mut aa: *mut Cell = 0 as *mut Cell;
    let mut decode: Bool = 0;
    /*-- Copy flags from env var BZIP2, and 
        expand filename wildcards in arg list.
   --*/
    argList = 0 as *mut Cell;
    addFlagsFromEnvVar(&mut argList,
                       b"BZIP2\x00" as *const u8 as *const std::os::raw::c_char as
                           *mut Char);
    addFlagsFromEnvVar(&mut argList,
                       b"BZIP\x00" as *const u8 as *const std::os::raw::c_char as
                           *mut Char);
    i = 1 as std::os::raw::c_int;
    while i <= argc - 1 as std::os::raw::c_int {
        argList = snocString(argList, *argv.offset(i as isize));
        i += 1
    }
    /* Free the argument list memory to mollify leak detectors 
      (eg) Purify, Checker.  Serves no other useful purpose.
   */
    aa = argList;
    while !aa.is_null() {
        let mut aa2: *mut Cell = (*aa).link;
        if !(*aa).name.is_null() { free((*aa).name as *mut std::os::raw::c_void); }
        free(aa as *mut std::os::raw::c_void);
        aa = aa2
    }
    return exitValue;
}

pub fn main() {
    let mut args: Vec<*mut std::os::raw::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as IntNative,
                                    args.as_mut_ptr() as *mut *mut Char) as
                                 i32)
    }
}
