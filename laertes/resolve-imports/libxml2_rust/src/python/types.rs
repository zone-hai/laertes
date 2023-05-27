use ::libc;
extern "C" {
    pub type _IO_wide_data;
<<<<<<< HEAD
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type PyMemberDef;
    pub type _xmlBuf;
    pub type _xmlDict;
    pub type _xmlHashTable;
    pub type _xmlStartTag;
    pub type _xmlAutomataState;
    pub type _xmlAutomata;
    pub type _xmlValidState;
    pub type _xmlRegexp;
    pub type _xmlCatalog;
    pub type _xmlXPathCompExpr;
    pub type _xmlRelaxNG;
    pub type _xmlRelaxNGParserCtxt;
    pub type _xmlRelaxNGValidCtxt;
    pub type _xmlSchema;
    pub type _xmlSchemaParserCtxt;
    pub type _xmlSchemaValidCtxt;
    pub type _xmlTextReader;
    fn PyLong_FromLong(_: i64) -> *mut PyObject;
    fn PyType_IsSubtype(_: *mut PyTypeObject, _: *mut PyTypeObject) -> i32;
    fn PyObject_GetAttrString(_: *mut PyObject, _: *const i8) -> *mut PyObject;
    fn PyObject_HasAttrString(_: *mut PyObject, _: *const i8) -> i32;
    static mut _Py_NoneStruct: PyObject;
    fn PyUnicodeUCS4_AsUTF8String(unicode: *mut PyObject) -> *mut PyObject;
    fn PyInt_FromLong(_: i64) -> *mut PyObject;
    static mut PyBool_Type: PyTypeObject;
    static mut _Py_TrueStruct: PyIntObject;
    static mut PyFloat_Type: PyTypeObject;
    fn PyFloat_FromDouble(_: f64) -> *mut PyObject;
    fn PyString_FromString(_: *const i8) -> *mut PyObject;
    fn PyList_New(size: Py_ssize_t) -> *mut PyObject;
    fn PyList_Size(_: *mut PyObject) -> Py_ssize_t;
    fn PyList_GetItem(_: *mut PyObject, _: Py_ssize_t) -> *mut PyObject;
    fn PyList_SetItem(_: *mut PyObject, _: Py_ssize_t, _: *mut PyObject) -> i32;
    static mut PyCapsule_Type: PyTypeObject;
    fn PyCapsule_New(
        pointer: *mut libc::c_void,
        name: *const i8,
        destructor: PyCapsule_Destructor,
    ) -> *mut PyObject;
    fn PyCapsule_GetPointer(
        capsule: *mut PyObject,
        name: *const i8,
    ) -> *mut libc::c_void;
    fn xmlStrndup(cur: *const xmlChar, len: i32) -> *mut xmlChar;
    static mut xmlFree: xmlFreeFunc;
    fn xmlXPathFreeObject(obj: xmlXPathObjectPtr);
    fn xmlXPathNodeSetCreate(val: xmlNodePtr) -> xmlNodeSetPtr;
    fn xmlXPathWrapString(val: *mut xmlChar) -> xmlXPathObjectPtr;
    fn xmlXPathNewFloat(val: f64) -> xmlXPathObjectPtr;
    fn xmlXPathNewBoolean(val: i32) -> xmlXPathObjectPtr;
    fn xmlXPathNodeSetAdd(cur: xmlNodeSetPtr, val: xmlNodePtr) -> i32;
    fn xmlXPathWrapNodeSet(val: xmlNodeSetPtr) -> xmlXPathObjectPtr;
    fn xmlXPathNodeSetFreeNs(ns: xmlNsPtr);
}
=======
    
    
    
    
    pub type _xmlDict;
    
    
    pub type _xmlAutomataState;
    pub type _xmlAutomata;
    pub type _xmlValidState;
    
    pub type _xmlCatalog;
    
    pub type _xmlRelaxNG;
    
    pub type _xmlRelaxNGValidCtxt;
    
    pub type _xmlSchemaParserCtxt;
    pub type _xmlSchemaValidCtxt;
    pub type _xmlTextReader;
    fn PyLong_FromLong(_: i64) -> * mut crate::src::python::libxml::_object;
    fn PyType_IsSubtype(_: * mut crate::src::python::libxml::_typeobject, _: * mut crate::src::python::libxml::_typeobject) -> i32;
    fn PyObject_GetAttrString(_: * mut crate::src::python::libxml::_object, _: * const i8) -> * mut crate::src::python::libxml::_object;
    fn PyObject_HasAttrString(_: * mut crate::src::python::libxml::_object, _: * const i8) -> i32;
    static mut _Py_NoneStruct: crate::src::python::libxml::_object;
    fn PyUnicodeUCS4_AsUTF8String(unicode: * mut crate::src::python::libxml::_object) -> * mut crate::src::python::libxml::_object;
    fn PyInt_FromLong(_: i64) -> * mut crate::src::python::libxml::_object;
    static mut PyBool_Type: crate::src::python::libxml::_typeobject;
    static mut _Py_TrueStruct: crate::src::python::types::PyIntObject;
    static mut PyFloat_Type: crate::src::python::libxml::_typeobject;
    fn PyFloat_FromDouble(_: f64) -> * mut crate::src::python::libxml::_object;
    fn PyString_FromString(_: * const i8) -> * mut crate::src::python::libxml::_object;
    fn PyList_New(size: i64) -> * mut crate::src::python::libxml::_object;
    fn PyList_Size(_: * mut crate::src::python::libxml::_object) -> i64;
    fn PyList_GetItem(_: * mut crate::src::python::libxml::_object, _: i64) -> * mut crate::src::python::libxml::_object;
    fn PyList_SetItem(_: * mut crate::src::python::libxml::_object, _: i64, _: * mut crate::src::python::libxml::_object) -> i32;
    static mut PyCapsule_Type: crate::src::python::libxml::_typeobject;
    fn PyCapsule_New(
        pointer: * mut core::ffi::c_void,
        name: * const i8,
        destructor: Option<unsafe extern "C"  fn(_: * mut crate::src::python::libxml::_object,) -> ()>,
    ) -> * mut crate::src::python::libxml::_object;
    fn PyCapsule_GetPointer(
        capsule: * mut crate::src::python::libxml::_object,
        name: * const i8,
    ) -> * mut core::ffi::c_void;
    fn xmlStrndup(cur: * const u8, len: i32) -> * mut u8;
    static mut xmlFree: Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,) -> ()>;
    fn xmlXPathFreeObject(obj: * mut crate::src::python::libxml::_xmlXPathObject);
    fn xmlXPathNodeSetCreate(val: * mut crate::src::python::libxml::_xmlNode) -> * mut crate::src::python::libxml::_xmlNodeSet;
    fn xmlXPathWrapString(val: * mut u8) -> * mut crate::src::python::libxml::_xmlXPathObject;
    fn xmlXPathNewFloat(val: f64) -> * mut crate::src::python::libxml::_xmlXPathObject;
    fn xmlXPathNewBoolean(val: i32) -> * mut crate::src::python::libxml::_xmlXPathObject;
    fn xmlXPathNodeSetAdd(cur: * mut crate::src::python::libxml::_xmlNodeSet, val: * mut crate::src::python::libxml::_xmlNode) -> i32;
    fn xmlXPathWrapNodeSet(val: * mut crate::src::python::libxml::_xmlNodeSet) -> * mut crate::src::python::libxml::_xmlXPathObject;
    fn xmlXPathNodeSetFreeNs(ns: * mut crate::src::python::libxml::_xmlNs);
}
pub use crate::src::python::libxml2_py::_xmlSchema;
pub use crate::src::python::libxml2_py::_IO_codecvt;
pub use crate::src::python::libxml2_py::_IO_marker;
pub use crate::src::python::libxml2_py::_xmlHashTable;
pub use crate::src::python::libxml2_py::_xmlRegexp;
pub use crate::src::python::libxml2_py::_xmlRelaxNGParserCtxt;
pub use crate::src::python::libxml2_py::_xmlStartTag;
pub use crate::src::python::libxml::PyMemberDef;
pub use crate::src::python::libxml::_xmlBuf;
pub use crate::src::python::libxml::_xmlXPathCompExpr;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
pub type size_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __ssize_t = i64;
<<<<<<< HEAD
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut i8,
    pub _IO_read_end: *mut i8,
    pub _IO_read_base: *mut i8,
    pub _IO_write_base: *mut i8,
    pub _IO_write_ptr: *mut i8,
    pub _IO_write_end: *mut i8,
    pub _IO_buf_base: *mut i8,
    pub _IO_buf_end: *mut i8,
    pub _IO_save_base: *mut i8,
    pub _IO_backup_base: *mut i8,
    pub _IO_save_end: *mut i8,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: __off_t,
    pub _cur_column: u16,
    pub _vtable_offset: i8,
    pub _shortbuf: [i8; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: i32,
    pub _unused2: [i8; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type ssize_t = __ssize_t;
pub type Py_ssize_t = ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _object {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut _typeobject,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _typeobject {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut _typeobject,
    pub ob_size: Py_ssize_t,
    pub tp_name: *const i8,
    pub tp_basicsize: Py_ssize_t,
    pub tp_itemsize: Py_ssize_t,
    pub tp_dealloc: destructor,
    pub tp_print: printfunc,
    pub tp_getattr: getattrfunc,
    pub tp_setattr: setattrfunc,
    pub tp_compare: cmpfunc,
    pub tp_repr: reprfunc,
    pub tp_as_number: *mut PyNumberMethods,
    pub tp_as_sequence: *mut PySequenceMethods,
    pub tp_as_mapping: *mut PyMappingMethods,
    pub tp_hash: hashfunc,
    pub tp_call: ternaryfunc,
    pub tp_str: reprfunc,
    pub tp_getattro: getattrofunc,
    pub tp_setattro: setattrofunc,
    pub tp_as_buffer: *mut PyBufferProcs,
    pub tp_flags: i64,
    pub tp_doc: *const i8,
    pub tp_traverse: traverseproc,
    pub tp_clear: inquiry,
    pub tp_richcompare: richcmpfunc,
    pub tp_weaklistoffset: Py_ssize_t,
    pub tp_iter: getiterfunc,
    pub tp_iternext: iternextfunc,
    pub tp_methods: *mut PyMethodDef,
    pub tp_members: *mut PyMemberDef,
    pub tp_getset: *mut PyGetSetDef,
    pub tp_base: *mut _typeobject,
    pub tp_dict: *mut PyObject,
    pub tp_descr_get: descrgetfunc,
    pub tp_descr_set: descrsetfunc,
    pub tp_dictoffset: Py_ssize_t,
    pub tp_init: initproc,
    pub tp_alloc: allocfunc,
    pub tp_new: newfunc,
    pub tp_free: freefunc,
    pub tp_is_gc: inquiry,
    pub tp_bases: *mut PyObject,
    pub tp_mro: *mut PyObject,
    pub tp_cache: *mut PyObject,
    pub tp_subclasses: *mut PyObject,
    pub tp_weaklist: *mut PyObject,
    pub tp_del: destructor,
    pub tp_version_tag: u32,
}
pub type destructor = Option::<unsafe extern "C" fn(*mut PyObject) -> ()>;
pub type PyObject = _object;
pub type inquiry = Option::<unsafe extern "C" fn(*mut PyObject) -> i32>;
pub type freefunc = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type newfunc = Option::<
    unsafe extern "C" fn(*mut _typeobject, *mut PyObject, *mut PyObject) -> *mut PyObject,
>;
pub type allocfunc = Option::<
    unsafe extern "C" fn(*mut _typeobject, Py_ssize_t) -> *mut PyObject,
>;
pub type initproc = Option::<
    unsafe extern "C" fn(*mut PyObject, *mut PyObject, *mut PyObject) -> i32,
>;
pub type descrsetfunc = Option::<
    unsafe extern "C" fn(*mut PyObject, *mut PyObject, *mut PyObject) -> i32,
>;
pub type descrgetfunc = Option::<
    unsafe extern "C" fn(*mut PyObject, *mut PyObject, *mut PyObject) -> *mut PyObject,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyGetSetDef {
    pub name: *mut i8,
    pub get: getter,
    pub set: setter,
    pub doc: *mut i8,
    pub closure: *mut libc::c_void,
}
pub type setter = Option::<
    unsafe extern "C" fn(*mut PyObject, *mut PyObject, *mut libc::c_void) -> i32,
>;
pub type getter = Option::<
    unsafe extern "C" fn(*mut PyObject, *mut libc::c_void) -> *mut PyObject,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyMethodDef {
    pub ml_name: *const i8,
    pub ml_meth: PyCFunction,
    pub ml_flags: i32,
    pub ml_doc: *const i8,
}
pub type PyCFunction = Option::<
    unsafe extern "C" fn(*mut PyObject, *mut PyObject) -> *mut PyObject,
>;
pub type iternextfunc = Option::<unsafe extern "C" fn(*mut PyObject) -> *mut PyObject>;
pub type getiterfunc = Option::<unsafe extern "C" fn(*mut PyObject) -> *mut PyObject>;
pub type richcmpfunc = Option::<
    unsafe extern "C" fn(*mut PyObject, *mut PyObject, i32) -> *mut PyObject,
>;
pub type traverseproc = Option::<
    unsafe extern "C" fn(*mut PyObject, visitproc, *mut libc::c_void) -> i32,
>;
pub type visitproc = Option::<
    unsafe extern "C" fn(*mut PyObject, *mut libc::c_void) -> i32,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyBufferProcs {
    pub bf_getreadbuffer: readbufferproc,
    pub bf_getwritebuffer: writebufferproc,
    pub bf_getsegcount: segcountproc,
    pub bf_getcharbuffer: charbufferproc,
    pub bf_getbuffer: getbufferproc,
    pub bf_releasebuffer: releasebufferproc,
}
pub type releasebufferproc = Option::<
    unsafe extern "C" fn(*mut PyObject, *mut Py_buffer) -> (),
>;
pub type Py_buffer = bufferinfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bufferinfo {
    pub buf: *mut libc::c_void,
    pub obj: *mut PyObject,
    pub len: Py_ssize_t,
    pub itemsize: Py_ssize_t,
    pub readonly: i32,
    pub ndim: i32,
    pub format: *mut i8,
    pub shape: *mut Py_ssize_t,
    pub strides: *mut Py_ssize_t,
    pub suboffsets: *mut Py_ssize_t,
    pub smalltable: [Py_ssize_t; 2],
    pub internal: *mut libc::c_void,
}
pub type getbufferproc = Option::<
    unsafe extern "C" fn(*mut PyObject, *mut Py_buffer, i32) -> i32,
>;
pub type charbufferproc = Option::<
    unsafe extern "C" fn(*mut PyObject, Py_ssize_t, *mut *mut i8) -> Py_ssize_t,
>;
pub type segcountproc = Option::<
    unsafe extern "C" fn(*mut PyObject, *mut Py_ssize_t) -> Py_ssize_t,
>;
pub type writebufferproc = Option::<
    unsafe extern "C" fn(*mut PyObject, Py_ssize_t, *mut *mut libc::c_void) -> Py_ssize_t,
>;
pub type readbufferproc = Option::<
    unsafe extern "C" fn(*mut PyObject, Py_ssize_t, *mut *mut libc::c_void) -> Py_ssize_t,
>;
pub type setattrofunc = Option::<
    unsafe extern "C" fn(*mut PyObject, *mut PyObject, *mut PyObject) -> i32,
>;
pub type getattrofunc = Option::<
    unsafe extern "C" fn(*mut PyObject, *mut PyObject) -> *mut PyObject,
>;
pub type reprfunc = Option::<unsafe extern "C" fn(*mut PyObject) -> *mut PyObject>;
pub type ternaryfunc = Option::<
    unsafe extern "C" fn(*mut PyObject, *mut PyObject, *mut PyObject) -> *mut PyObject,
>;
pub type hashfunc = Option::<unsafe extern "C" fn(*mut PyObject) -> i64>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyMappingMethods {
    pub mp_length: lenfunc,
    pub mp_subscript: binaryfunc,
    pub mp_ass_subscript: objobjargproc,
}
pub type objobjargproc = Option::<
    unsafe extern "C" fn(*mut PyObject, *mut PyObject, *mut PyObject) -> i32,
>;
pub type binaryfunc = Option::<
    unsafe extern "C" fn(*mut PyObject, *mut PyObject) -> *mut PyObject,
>;
pub type lenfunc = Option::<unsafe extern "C" fn(*mut PyObject) -> Py_ssize_t>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PySequenceMethods {
    pub sq_length: lenfunc,
    pub sq_concat: binaryfunc,
    pub sq_repeat: ssizeargfunc,
    pub sq_item: ssizeargfunc,
    pub sq_slice: ssizessizeargfunc,
    pub sq_ass_item: ssizeobjargproc,
    pub sq_ass_slice: ssizessizeobjargproc,
    pub sq_contains: objobjproc,
    pub sq_inplace_concat: binaryfunc,
    pub sq_inplace_repeat: ssizeargfunc,
}
pub type ssizeargfunc = Option::<
    unsafe extern "C" fn(*mut PyObject, Py_ssize_t) -> *mut PyObject,
>;
pub type objobjproc = Option::<
    unsafe extern "C" fn(*mut PyObject, *mut PyObject) -> i32,
>;
pub type ssizessizeobjargproc = Option::<
    unsafe extern "C" fn(
        *mut PyObject,
        Py_ssize_t,
        Py_ssize_t,
        *mut PyObject,
    ) -> i32,
>;
pub type ssizeobjargproc = Option::<
    unsafe extern "C" fn(*mut PyObject, Py_ssize_t, *mut PyObject) -> i32,
>;
pub type ssizessizeargfunc = Option::<
    unsafe extern "C" fn(*mut PyObject, Py_ssize_t, Py_ssize_t) -> *mut PyObject,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyNumberMethods {
    pub nb_add: binaryfunc,
    pub nb_subtract: binaryfunc,
    pub nb_multiply: binaryfunc,
    pub nb_divide: binaryfunc,
    pub nb_remainder: binaryfunc,
    pub nb_divmod: binaryfunc,
    pub nb_power: ternaryfunc,
    pub nb_negative: unaryfunc,
    pub nb_positive: unaryfunc,
    pub nb_absolute: unaryfunc,
    pub nb_nonzero: inquiry,
    pub nb_invert: unaryfunc,
    pub nb_lshift: binaryfunc,
    pub nb_rshift: binaryfunc,
    pub nb_and: binaryfunc,
    pub nb_xor: binaryfunc,
    pub nb_or: binaryfunc,
    pub nb_coerce: coercion,
    pub nb_int: unaryfunc,
    pub nb_long: unaryfunc,
    pub nb_float: unaryfunc,
    pub nb_oct: unaryfunc,
    pub nb_hex: unaryfunc,
    pub nb_inplace_add: binaryfunc,
    pub nb_inplace_subtract: binaryfunc,
    pub nb_inplace_multiply: binaryfunc,
    pub nb_inplace_divide: binaryfunc,
    pub nb_inplace_remainder: binaryfunc,
    pub nb_inplace_power: ternaryfunc,
    pub nb_inplace_lshift: binaryfunc,
    pub nb_inplace_rshift: binaryfunc,
    pub nb_inplace_and: binaryfunc,
    pub nb_inplace_xor: binaryfunc,
    pub nb_inplace_or: binaryfunc,
    pub nb_floor_divide: binaryfunc,
    pub nb_true_divide: binaryfunc,
    pub nb_inplace_floor_divide: binaryfunc,
    pub nb_inplace_true_divide: binaryfunc,
    pub nb_index: unaryfunc,
}
pub type unaryfunc = Option::<unsafe extern "C" fn(*mut PyObject) -> *mut PyObject>;
pub type coercion = Option::<
    unsafe extern "C" fn(*mut *mut PyObject, *mut *mut PyObject) -> i32,
>;
pub type cmpfunc = Option::<
    unsafe extern "C" fn(*mut PyObject, *mut PyObject) -> i32,
>;
pub type setattrfunc = Option::<
    unsafe extern "C" fn(*mut PyObject, *mut i8, *mut PyObject) -> i32,
>;
pub type getattrfunc = Option::<
    unsafe extern "C" fn(*mut PyObject, *mut i8) -> *mut PyObject,
>;
pub type printfunc = Option::<
    unsafe extern "C" fn(*mut PyObject, *mut FILE, i32) -> i32,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyVarObject {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut _typeobject,
    pub ob_size: Py_ssize_t,
}
pub type PyTypeObject = _typeobject;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyIntObject {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut _typeobject,
    pub ob_ival: i64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyFloatObject {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut _typeobject,
    pub ob_fval: f64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyStringObject {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut _typeobject,
    pub ob_size: Py_ssize_t,
=======
// #[derive(Copy, Clone)]

pub type _IO_FILE = crate::src::python::libxml::_IO_FILE;
pub type _IO_lock_t = ();
pub type FILE = crate::src::python::libxml::_IO_FILE;
pub type ssize_t = i64;
pub type Py_ssize_t = i64;
// #[derive(Copy, Clone)]

pub type _object = crate::src::python::libxml::_object;
// #[derive(Copy, Clone)]

pub type _typeobject = crate::src::python::libxml::_typeobject;
pub type destructor<'a1> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::python::libxml::_object>,) -> ()>;
pub type PyObject = crate::src::python::libxml::_object;
pub type inquiry<'a1> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::python::libxml::_object>,) -> i32>;
pub type freefunc<'a1> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,) -> ()>;
pub type newfunc<'a1, 'a2, 'a3, 'a4> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::python::libxml::_typeobject>,_: Option<&'a2 mut crate::src::python::libxml::_object>,_: Option<&'a3 mut crate::src::python::libxml::_object>,) -> Option<&'a4 mut crate::src::python::libxml::_object>>;
pub type allocfunc<'a1, 'a2> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::python::libxml::_typeobject>,_: i64,) -> Option<&'a2 mut crate::src::python::libxml::_object>>;
pub type initproc<'a1, 'a2, 'a3> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::python::libxml::_object>,_: Option<&'a2 mut crate::src::python::libxml::_object>,_: Option<&'a3 mut crate::src::python::libxml::_object>,) -> i32>;
pub type descrsetfunc<'a1, 'a2, 'a3> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::python::libxml::_object>,_: Option<&'a2 mut crate::src::python::libxml::_object>,_: Option<&'a3 mut crate::src::python::libxml::_object>,) -> i32>;
pub type descrgetfunc<'a1, 'a2, 'a3, 'a4> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::python::libxml::_object>,_: Option<&'a2 mut crate::src::python::libxml::_object>,_: Option<&'a3 mut crate::src::python::libxml::_object>,) -> Option<&'a4 mut crate::src::python::libxml::_object>>;
// #[derive(Copy, Clone)]

pub type PyGetSetDef = crate::src::python::libxml::PyGetSetDef;
pub type setter<'a1, 'a2, 'a3> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::python::libxml::_object>,_: Option<&'a2 mut crate::src::python::libxml::_object>,_: Option<&'a3 mut core::ffi::c_void>,) -> i32>;
pub type getter<'a1, 'a2, 'a3> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::python::libxml::_object>,_: Option<&'a2 mut core::ffi::c_void>,) -> Option<&'a3 mut crate::src::python::libxml::_object>>;
// #[derive(Copy, Clone)]

pub type PyMethodDef = crate::src::python::libxml::PyMethodDef;
pub type PyCFunction<'a1, 'a2, 'a3> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::python::libxml::_object>,_: Option<&'a2 mut crate::src::python::libxml::_object>,) -> Option<&'a3 mut crate::src::python::libxml::_object>>;
pub type iternextfunc<'a1, 'a2> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::python::libxml::_object>,) -> Option<&'a2 mut crate::src::python::libxml::_object>>;
pub type getiterfunc<'a1, 'a2> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::python::libxml::_object>,) -> Option<&'a2 mut crate::src::python::libxml::_object>>;
pub type richcmpfunc<'a1, 'a2, 'a3> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::python::libxml::_object>,_: Option<&'a2 mut crate::src::python::libxml::_object>,_: i32,) -> Option<&'a3 mut crate::src::python::libxml::_object>>;
pub type traverseproc<'a1, 'a2> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::python::libxml::_object>,_: Option<unsafe extern "C"  fn(_: * mut crate::src::python::libxml::_object,_: * mut core::ffi::c_void,) -> i32>,_: Option<&'a2 mut core::ffi::c_void>,) -> i32>;
pub type visitproc<'a1, 'a2> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::python::libxml::_object>,_: Option<&'a2 mut core::ffi::c_void>,) -> i32>;
// #[derive(Copy, Clone)]

pub type PyBufferProcs = crate::src::python::libxml::PyBufferProcs;
pub type releasebufferproc<'a1, 'a2> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::python::libxml::_object>,_: Option<&'a2 mut crate::src::python::libxml::bufferinfo>,) -> ()>;
pub type Py_buffer = crate::src::python::libxml::bufferinfo;
// #[derive(Copy, Clone)]

pub type bufferinfo = crate::src::python::libxml::bufferinfo;
pub type getbufferproc<'a1, 'a2> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::python::libxml::_object>,_: Option<&'a2 mut crate::src::python::libxml::bufferinfo>,_: i32,) -> i32>;
pub type charbufferproc<'a1, 'a2, 'a3> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::python::libxml::_object>,_: i64,_: Option<&'a2 mut Option<&'a3 mut i8>>,) -> i64>;
pub type segcountproc<'a1, 'a2> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::python::libxml::_object>,_: Option<&'a2 mut i64>,) -> i64>;
pub type writebufferproc<'a1, 'a2, 'a3> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::python::libxml::_object>,_: i64,_: Option<&'a2 mut Option<&'a3 mut core::ffi::c_void>>,) -> i64>;
pub type readbufferproc<'a1, 'a2, 'a3> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::python::libxml::_object>,_: i64,_: Option<&'a2 mut Option<&'a3 mut core::ffi::c_void>>,) -> i64>;
pub type setattrofunc<'a1, 'a2, 'a3> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::python::libxml::_object>,_: Option<&'a2 mut crate::src::python::libxml::_object>,_: Option<&'a3 mut crate::src::python::libxml::_object>,) -> i32>;
pub type getattrofunc<'a1, 'a2, 'a3> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::python::libxml::_object>,_: Option<&'a2 mut crate::src::python::libxml::_object>,) -> Option<&'a3 mut crate::src::python::libxml::_object>>;
pub type reprfunc<'a1, 'a2> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::python::libxml::_object>,) -> Option<&'a2 mut crate::src::python::libxml::_object>>;
pub type ternaryfunc<'a1, 'a2, 'a3, 'a4> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::python::libxml::_object>,_: Option<&'a2 mut crate::src::python::libxml::_object>,_: Option<&'a3 mut crate::src::python::libxml::_object>,) -> Option<&'a4 mut crate::src::python::libxml::_object>>;
pub type hashfunc<'a1> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::python::libxml::_object>,) -> i64>;
// #[derive(Copy, Clone)]

pub type PyMappingMethods = crate::src::python::libxml::PyMappingMethods;
pub type objobjargproc<'a1, 'a2, 'a3> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::python::libxml::_object>,_: Option<&'a2 mut crate::src::python::libxml::_object>,_: Option<&'a3 mut crate::src::python::libxml::_object>,) -> i32>;
pub type binaryfunc<'a1, 'a2, 'a3> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::python::libxml::_object>,_: Option<&'a2 mut crate::src::python::libxml::_object>,) -> Option<&'a3 mut crate::src::python::libxml::_object>>;
pub type lenfunc<'a1> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::python::libxml::_object>,) -> i64>;
// #[derive(Copy, Clone)]

pub type PySequenceMethods = crate::src::python::libxml::PySequenceMethods;
pub type ssizeargfunc<'a1, 'a2> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::python::libxml::_object>,_: i64,) -> Option<&'a2 mut crate::src::python::libxml::_object>>;
pub type objobjproc<'a1, 'a2> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::python::libxml::_object>,_: Option<&'a2 mut crate::src::python::libxml::_object>,) -> i32>;
pub type ssizessizeobjargproc<'a1, 'a2> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::python::libxml::_object>,_: i64,_: i64,_: Option<&'a2 mut crate::src::python::libxml::_object>,) -> i32>;
pub type ssizeobjargproc<'a1, 'a2> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::python::libxml::_object>,_: i64,_: Option<&'a2 mut crate::src::python::libxml::_object>,) -> i32>;
pub type ssizessizeargfunc<'a1, 'a2> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::python::libxml::_object>,_: i64,_: i64,) -> Option<&'a2 mut crate::src::python::libxml::_object>>;
// #[derive(Copy, Clone)]

pub type PyNumberMethods = crate::src::python::libxml::PyNumberMethods;
pub type unaryfunc<'a1, 'a2> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::python::libxml::_object>,) -> Option<&'a2 mut crate::src::python::libxml::_object>>;
pub type coercion<'a1, 'a2, 'a3, 'a4> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut Option<&'a2 mut crate::src::python::libxml::_object>>,_: Option<&'a3 mut Option<&'a4 mut crate::src::python::libxml::_object>>,) -> i32>;
pub type cmpfunc<'a1, 'a2> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::python::libxml::_object>,_: Option<&'a2 mut crate::src::python::libxml::_object>,) -> i32>;
pub type setattrfunc<'a1, 'a2, 'a3> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::python::libxml::_object>,_: Option<&'a2 mut i8>,_: Option<&'a3 mut crate::src::python::libxml::_object>,) -> i32>;
pub type getattrfunc<'a1, 'a2, 'a3> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::python::libxml::_object>,_: Option<&'a2 mut i8>,) -> Option<&'a3 mut crate::src::python::libxml::_object>>;
pub type printfunc<'a1, 'a2> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::python::libxml::_object>,_: Option<&'a2 mut crate::src::python::libxml::_IO_FILE>,_: i32,) -> i32>;
// #[derive(Copy, Clone)]

pub type PyVarObject<'a> = crate::src::python::libxml::PyVarObject<'a>;
pub type PyTypeObject = crate::src::python::libxml::_typeobject;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyIntObject {
    pub ob_refcnt: i64,
    pub ob_type: * mut crate::src::python::libxml::_typeobject,
    pub ob_ival: i64,
}
impl PyIntObject {
    pub const fn new() -> Self {
        PyIntObject {
        ob_refcnt: 0,
        ob_type: (0 as * mut crate::src::python::libxml::_typeobject),
        ob_ival: 0
        }
    }
}

impl std::default::Default for PyIntObject {
    fn default() -> Self { PyIntObject::new() }
}

// #[derive(Copy, Clone)]
#[repr(C)]
pub struct PyFloatObject<'a> {
    pub ob_refcnt: i64,
    pub ob_type: Option<&'a mut crate::src::python::libxml::_typeobject>,
    pub ob_fval: f64,
}
impl<'a> PyFloatObject<'a> {
    pub const fn new() -> Self {
        PyFloatObject {
        ob_refcnt: 0,
        ob_type: None,
        ob_fval: 0.0
        }
    }
}

impl<'a> std::default::Default for PyFloatObject<'a> {
    fn default() -> Self { PyFloatObject::new() }
}

// #[derive(Copy, Clone)]
#[repr(C)]
pub struct PyStringObject<'a> {
    pub ob_refcnt: i64,
    pub ob_type: Option<&'a mut crate::src::python::libxml::_typeobject>,
    pub ob_size: i64,
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
    pub ob_shash: i64,
    pub ob_sstate: i32,
    pub ob_sval: [i8; 1],
}
<<<<<<< HEAD
pub type PyCapsule_Destructor = Option::<unsafe extern "C" fn(*mut PyObject) -> ()>;
pub type xmlChar = u8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlParserInputBuffer {
    pub context: *mut libc::c_void,
    pub readcallback: xmlInputReadCallback,
    pub closecallback: xmlInputCloseCallback,
    pub encoder: xmlCharEncodingHandlerPtr,
    pub buffer: xmlBufPtr,
    pub raw: xmlBufPtr,
    pub compressed: i32,
    pub error: i32,
    pub rawconsumed: u64,
}
pub type xmlBufPtr = *mut xmlBuf;
pub type xmlBuf = _xmlBuf;
pub type xmlCharEncodingHandlerPtr = *mut xmlCharEncodingHandler;
pub type xmlCharEncodingHandler = _xmlCharEncodingHandler;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlCharEncodingHandler {
    pub name: *mut i8,
    pub input: xmlCharEncodingInputFunc,
    pub output: xmlCharEncodingOutputFunc,
    pub iconv_in: iconv_t,
    pub iconv_out: iconv_t,
}
pub type iconv_t = *mut libc::c_void;
pub type xmlCharEncodingOutputFunc = Option::<
    unsafe extern "C" fn(
        *mut u8,
        *mut i32,
        *const u8,
        *mut i32,
    ) -> i32,
>;
pub type xmlCharEncodingInputFunc = Option::<
    unsafe extern "C" fn(
        *mut u8,
        *mut i32,
        *const u8,
        *mut i32,
    ) -> i32,
>;
pub type xmlInputCloseCallback = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> i32,
>;
pub type xmlInputReadCallback = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *mut i8,
        i32,
    ) -> i32,
>;
pub type xmlParserInputBuffer = _xmlParserInputBuffer;
pub type xmlParserInputBufferPtr = *mut xmlParserInputBuffer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlOutputBuffer {
    pub context: *mut libc::c_void,
    pub writecallback: xmlOutputWriteCallback,
    pub closecallback: xmlOutputCloseCallback,
    pub encoder: xmlCharEncodingHandlerPtr,
    pub buffer: xmlBufPtr,
    pub conv: xmlBufPtr,
    pub written: i32,
    pub error: i32,
}
pub type xmlOutputCloseCallback = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> i32,
>;
pub type xmlOutputWriteCallback = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const i8,
        i32,
    ) -> i32,
>;
pub type xmlOutputBuffer = _xmlOutputBuffer;
pub type xmlOutputBufferPtr = *mut xmlOutputBuffer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlParserInput {
    pub buf: xmlParserInputBufferPtr,
    pub filename: *const i8,
    pub directory: *const i8,
    pub base: *const xmlChar,
    pub cur: *const xmlChar,
    pub end: *const xmlChar,
    pub length: i32,
    pub line: i32,
    pub col: i32,
    pub consumed: u64,
    pub free: xmlParserInputDeallocate,
    pub encoding: *const xmlChar,
    pub version: *const xmlChar,
    pub standalone: i32,
    pub id: i32,
}
pub type xmlParserInputDeallocate = Option::<unsafe extern "C" fn(*mut xmlChar) -> ()>;
pub type xmlParserInput = _xmlParserInput;
pub type xmlParserInputPtr = *mut xmlParserInput;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlParserCtxt {
    pub sax: *mut _xmlSAXHandler,
    pub userData: *mut libc::c_void,
    pub myDoc: xmlDocPtr,
    pub wellFormed: i32,
    pub replaceEntities: i32,
    pub version: *const xmlChar,
    pub encoding: *const xmlChar,
    pub standalone: i32,
    pub html: i32,
    pub input: xmlParserInputPtr,
    pub inputNr: i32,
    pub inputMax: i32,
    pub inputTab: *mut xmlParserInputPtr,
    pub node: xmlNodePtr,
    pub nodeNr: i32,
    pub nodeMax: i32,
    pub nodeTab: *mut xmlNodePtr,
    pub record_info: i32,
    pub node_seq: xmlParserNodeInfoSeq,
    pub errNo: i32,
    pub hasExternalSubset: i32,
    pub hasPErefs: i32,
    pub external: i32,
    pub valid: i32,
    pub validate: i32,
    pub vctxt: xmlValidCtxt,
    pub instate: xmlParserInputState,
    pub token: i32,
    pub directory: *mut i8,
    pub name: *const xmlChar,
    pub nameNr: i32,
    pub nameMax: i32,
    pub nameTab: *mut *const xmlChar,
    pub nbChars: i64,
    pub checkIndex: i64,
    pub keepBlanks: i32,
    pub disableSAX: i32,
    pub inSubset: i32,
    pub intSubName: *const xmlChar,
    pub extSubURI: *mut xmlChar,
    pub extSubSystem: *mut xmlChar,
    pub space: *mut i32,
    pub spaceNr: i32,
    pub spaceMax: i32,
    pub spaceTab: *mut i32,
    pub depth: i32,
    pub entity: xmlParserInputPtr,
    pub charset: i32,
    pub nodelen: i32,
    pub nodemem: i32,
    pub pedantic: i32,
    pub _private: *mut libc::c_void,
    pub loadsubset: i32,
    pub linenumbers: i32,
    pub catalogs: *mut libc::c_void,
    pub recovery: i32,
    pub progressive: i32,
    pub dict: xmlDictPtr,
    pub atts: *mut *const xmlChar,
    pub maxatts: i32,
    pub docdict: i32,
    pub str_xml: *const xmlChar,
    pub str_xmlns: *const xmlChar,
    pub str_xml_ns: *const xmlChar,
    pub sax2: i32,
    pub nsNr: i32,
    pub nsMax: i32,
    pub nsTab: *mut *const xmlChar,
    pub attallocs: *mut i32,
    pub pushTab: *mut xmlStartTag,
    pub attsDefault: xmlHashTablePtr,
    pub attsSpecial: xmlHashTablePtr,
    pub nsWellFormed: i32,
    pub options: i32,
    pub dictNames: i32,
    pub freeElemsNr: i32,
    pub freeElems: xmlNodePtr,
    pub freeAttrsNr: i32,
    pub freeAttrs: xmlAttrPtr,
    pub lastError: xmlError,
    pub parseMode: xmlParserMode,
    pub nbentities: u64,
    pub sizeentities: u64,
    pub nodeInfo: *mut xmlParserNodeInfo,
    pub nodeInfoNr: i32,
    pub nodeInfoMax: i32,
    pub nodeInfoTab: *mut xmlParserNodeInfo,
    pub input_id: i32,
    pub sizeentcopy: u64,
}
pub type xmlParserNodeInfo = _xmlParserNodeInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlParserNodeInfo {
    pub node: *const _xmlNode,
    pub begin_pos: u64,
    pub begin_line: u64,
    pub end_pos: u64,
    pub end_line: u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlNode {
    pub _private: *mut libc::c_void,
    pub type_0: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlNode,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub ns: *mut xmlNs,
    pub content: *mut xmlChar,
    pub properties: *mut _xmlAttr,
    pub nsDef: *mut xmlNs,
    pub psvi: *mut libc::c_void,
    pub line: u16,
    pub extra: u16,
}
pub type xmlNs = _xmlNs;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlNs {
    pub next: *mut _xmlNs,
    pub type_0: xmlNsType,
    pub href: *const xmlChar,
    pub prefix: *const xmlChar,
    pub _private: *mut libc::c_void,
    pub context: *mut _xmlDoc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlDoc {
    pub _private: *mut libc::c_void,
    pub type_0: xmlElementType,
    pub name: *mut i8,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlNode,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub compression: i32,
    pub standalone: i32,
    pub intSubset: *mut _xmlDtd,
    pub extSubset: *mut _xmlDtd,
    pub oldNs: *mut _xmlNs,
    pub version: *const xmlChar,
    pub encoding: *const xmlChar,
    pub ids: *mut libc::c_void,
    pub refs: *mut libc::c_void,
    pub URL: *const xmlChar,
    pub charset: i32,
    pub dict: *mut _xmlDict,
    pub psvi: *mut libc::c_void,
    pub parseFlags: i32,
    pub properties: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlDtd {
    pub _private: *mut libc::c_void,
    pub type_0: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlDoc,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub notations: *mut libc::c_void,
    pub elements: *mut libc::c_void,
    pub attributes: *mut libc::c_void,
    pub entities: *mut libc::c_void,
    pub ExternalID: *const xmlChar,
    pub SystemID: *const xmlChar,
    pub pentities: *mut libc::c_void,
}
=======
impl<'a> PyStringObject<'a> {
    pub const fn new() -> Self {
        PyStringObject {
        ob_refcnt: 0,
        ob_type: None,
        ob_size: 0,
        ob_shash: 0,
        ob_sstate: 0,
        ob_sval: [0,]
        }
    }
}

impl<'a> std::default::Default for PyStringObject<'a> {
    fn default() -> Self { PyStringObject::new() }
}

pub type PyCapsule_Destructor<'a1> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::python::libxml::_object>,) -> ()>;
pub type xmlChar = u8;
// #[derive(Copy, Clone)]

pub type _xmlParserInputBuffer = crate::src::python::libxml::_xmlParserInputBuffer;
pub type xmlBufPtr = * mut crate::src::python::libxml::_xmlBuf;
pub type xmlBuf = crate::src::python::libxml::_xmlBuf;
pub type xmlCharEncodingHandlerPtr = * mut crate::src::python::libxml::_xmlCharEncodingHandler;
pub type xmlCharEncodingHandler = crate::src::python::libxml::_xmlCharEncodingHandler;
// #[derive(Copy, Clone)]

pub type _xmlCharEncodingHandler = crate::src::python::libxml::_xmlCharEncodingHandler;
pub type iconv_t = * mut core::ffi::c_void;
pub type xmlCharEncodingOutputFunc<'a1, 'a2, 'a3, 'a4> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut u8>,_: Option<&'a2 mut i32>,_: Option<&'a3 u8>,_: Option<&'a4 mut i32>,) -> i32>;
pub type xmlCharEncodingInputFunc<'a1, 'a2, 'a3, 'a4> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut u8>,_: Option<&'a2 mut i32>,_: Option<&'a3 u8>,_: Option<&'a4 mut i32>,) -> i32>;
pub type xmlInputCloseCallback<'a1> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,) -> i32>;
pub type xmlInputReadCallback<'a1, 'a2> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,_: Option<&'a2 mut i8>,_: i32,) -> i32>;
pub type xmlParserInputBuffer = crate::src::python::libxml::_xmlParserInputBuffer;
pub type xmlParserInputBufferPtr = * mut crate::src::python::libxml::_xmlParserInputBuffer;
// #[derive(Copy, Clone)]

pub type _xmlOutputBuffer = crate::src::python::libxml::_xmlOutputBuffer;
pub type xmlOutputCloseCallback<'a1> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,) -> i32>;
pub type xmlOutputWriteCallback<'a1, 'a2> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,_: Option<&'a2 i8>,_: i32,) -> i32>;
pub type xmlOutputBuffer = crate::src::python::libxml::_xmlOutputBuffer;
pub type xmlOutputBufferPtr = * mut crate::src::python::libxml::_xmlOutputBuffer;
// #[derive(Copy, Clone)]

pub type _xmlParserInput = crate::src::python::libxml::_xmlParserInput;
pub type xmlParserInputDeallocate<'a1> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut u8>,) -> ()>;
pub type xmlParserInput = crate::src::python::libxml::_xmlParserInput;
pub type xmlParserInputPtr = * mut crate::src::python::libxml::_xmlParserInput;
// #[derive(Copy, Clone)]

pub type _xmlParserCtxt = crate::src::python::libxml::_xmlParserCtxt;
pub type xmlParserNodeInfo = crate::src::python::libxml::_xmlParserNodeInfo;
// #[derive(Copy, Clone)]

pub type _xmlParserNodeInfo = crate::src::python::libxml::_xmlParserNodeInfo;
// #[derive(Copy, Clone)]

pub type _xmlNode = crate::src::python::libxml::_xmlNode;
pub type xmlNs = crate::src::python::libxml::_xmlNs;
// #[derive(Copy, Clone)]

pub type _xmlNs = crate::src::python::libxml::_xmlNs;
// #[derive(Copy, Clone)]

pub type _xmlDoc = crate::src::python::libxml::_xmlDoc;
// #[derive(Copy, Clone)]

pub type _xmlDtd = crate::src::python::libxml::_xmlDtd;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
pub type xmlElementType = u32;
pub const XML_XINCLUDE_END: xmlElementType = 20;
pub const XML_XINCLUDE_START: xmlElementType = 19;
pub const XML_NAMESPACE_DECL: xmlElementType = 18;
pub const XML_ENTITY_DECL: xmlElementType = 17;
pub const XML_ATTRIBUTE_DECL: xmlElementType = 16;
pub const XML_ELEMENT_DECL: xmlElementType = 15;
pub const XML_DTD_NODE: xmlElementType = 14;
pub const XML_HTML_DOCUMENT_NODE: xmlElementType = 13;
pub const XML_NOTATION_NODE: xmlElementType = 12;
pub const XML_DOCUMENT_FRAG_NODE: xmlElementType = 11;
pub const XML_DOCUMENT_TYPE_NODE: xmlElementType = 10;
pub const XML_DOCUMENT_NODE: xmlElementType = 9;
pub const XML_COMMENT_NODE: xmlElementType = 8;
pub const XML_PI_NODE: xmlElementType = 7;
pub const XML_ENTITY_NODE: xmlElementType = 6;
pub const XML_ENTITY_REF_NODE: xmlElementType = 5;
pub const XML_CDATA_SECTION_NODE: xmlElementType = 4;
pub const XML_TEXT_NODE: xmlElementType = 3;
pub const XML_ATTRIBUTE_NODE: xmlElementType = 2;
pub const XML_ELEMENT_NODE: xmlElementType = 1;
<<<<<<< HEAD
pub type xmlNsType = xmlElementType;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlAttr {
    pub _private: *mut libc::c_void,
    pub type_0: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlNode,
    pub next: *mut _xmlAttr,
    pub prev: *mut _xmlAttr,
    pub doc: *mut _xmlDoc,
    pub ns: *mut xmlNs,
    pub atype: xmlAttributeType,
    pub psvi: *mut libc::c_void,
}
=======
pub type xmlNsType = u32;
// #[derive(Copy, Clone)]

pub type _xmlAttr = crate::src::python::libxml::_xmlAttr;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
pub type xmlAttributeType = u32;
pub const XML_ATTRIBUTE_NOTATION: xmlAttributeType = 10;
pub const XML_ATTRIBUTE_ENUMERATION: xmlAttributeType = 9;
pub const XML_ATTRIBUTE_NMTOKENS: xmlAttributeType = 8;
pub const XML_ATTRIBUTE_NMTOKEN: xmlAttributeType = 7;
pub const XML_ATTRIBUTE_ENTITIES: xmlAttributeType = 6;
pub const XML_ATTRIBUTE_ENTITY: xmlAttributeType = 5;
pub const XML_ATTRIBUTE_IDREFS: xmlAttributeType = 4;
pub const XML_ATTRIBUTE_IDREF: xmlAttributeType = 3;
pub const XML_ATTRIBUTE_ID: xmlAttributeType = 2;
pub const XML_ATTRIBUTE_CDATA: xmlAttributeType = 1;
pub type xmlParserMode = u32;
pub const XML_PARSE_READER: xmlParserMode = 5;
pub const XML_PARSE_PUSH_SAX: xmlParserMode = 4;
pub const XML_PARSE_PUSH_DOM: xmlParserMode = 3;
pub const XML_PARSE_SAX: xmlParserMode = 2;
pub const XML_PARSE_DOM: xmlParserMode = 1;
pub const XML_PARSE_UNKNOWN: xmlParserMode = 0;
<<<<<<< HEAD
pub type xmlError = _xmlError;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlError {
    pub domain: i32,
    pub code: i32,
    pub message: *mut i8,
    pub level: xmlErrorLevel,
    pub file: *mut i8,
    pub line: i32,
    pub str1: *mut i8,
    pub str2: *mut i8,
    pub str3: *mut i8,
    pub int1: i32,
    pub int2: i32,
    pub ctxt: *mut libc::c_void,
    pub node: *mut libc::c_void,
}
=======
pub type xmlError = crate::src::python::libxml::_xmlError;
// #[derive(Copy, Clone)]

pub type _xmlError = crate::src::python::libxml::_xmlError;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
pub type xmlErrorLevel = u32;
pub const XML_ERR_FATAL: xmlErrorLevel = 3;
pub const XML_ERR_ERROR: xmlErrorLevel = 2;
pub const XML_ERR_WARNING: xmlErrorLevel = 1;
pub const XML_ERR_NONE: xmlErrorLevel = 0;
<<<<<<< HEAD
pub type xmlAttrPtr = *mut xmlAttr;
pub type xmlAttr = _xmlAttr;
pub type xmlNodePtr = *mut xmlNode;
pub type xmlNode = _xmlNode;
pub type xmlHashTablePtr = *mut xmlHashTable;
pub type xmlHashTable = _xmlHashTable;
pub type xmlStartTag = _xmlStartTag;
pub type xmlDictPtr = *mut xmlDict;
pub type xmlDict = _xmlDict;
=======
pub type xmlAttrPtr = * mut crate::src::python::libxml::_xmlAttr;
pub type xmlAttr = crate::src::python::libxml::_xmlAttr;
pub type xmlNodePtr = * mut crate::src::python::libxml::_xmlNode;
pub type xmlNode = crate::src::python::libxml::_xmlNode;
pub type xmlHashTablePtr = * mut crate::src::python::libxml2_py::_xmlHashTable;
pub type xmlHashTable = crate::src::python::libxml2_py::_xmlHashTable;
pub type xmlStartTag = crate::src::python::libxml2_py::_xmlStartTag;
pub type xmlDictPtr = * mut crate::src::python::types::_xmlDict;
pub type xmlDict = crate::src::python::types::_xmlDict;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
pub type xmlParserInputState = i32;
pub const XML_PARSER_PUBLIC_LITERAL: xmlParserInputState = 16;
pub const XML_PARSER_IGNORE: xmlParserInputState = 15;
pub const XML_PARSER_EPILOG: xmlParserInputState = 14;
pub const XML_PARSER_SYSTEM_LITERAL: xmlParserInputState = 13;
pub const XML_PARSER_ATTRIBUTE_VALUE: xmlParserInputState = 12;
pub const XML_PARSER_ENTITY_VALUE: xmlParserInputState = 11;
pub const XML_PARSER_ENTITY_DECL: xmlParserInputState = 10;
pub const XML_PARSER_END_TAG: xmlParserInputState = 9;
pub const XML_PARSER_CDATA_SECTION: xmlParserInputState = 8;
pub const XML_PARSER_CONTENT: xmlParserInputState = 7;
pub const XML_PARSER_START_TAG: xmlParserInputState = 6;
pub const XML_PARSER_COMMENT: xmlParserInputState = 5;
pub const XML_PARSER_PROLOG: xmlParserInputState = 4;
pub const XML_PARSER_DTD: xmlParserInputState = 3;
pub const XML_PARSER_PI: xmlParserInputState = 2;
pub const XML_PARSER_MISC: xmlParserInputState = 1;
pub const XML_PARSER_START: xmlParserInputState = 0;
pub const XML_PARSER_EOF: xmlParserInputState = -1;
<<<<<<< HEAD
pub type xmlValidCtxt = _xmlValidCtxt;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlValidCtxt {
    pub userData: *mut libc::c_void,
    pub error: xmlValidityErrorFunc,
    pub warning: xmlValidityWarningFunc,
    pub node: xmlNodePtr,
    pub nodeNr: i32,
    pub nodeMax: i32,
    pub nodeTab: *mut xmlNodePtr,
    pub flags: u32,
    pub doc: xmlDocPtr,
    pub valid: i32,
    pub vstate: *mut xmlValidState,
    pub vstateNr: i32,
    pub vstateMax: i32,
    pub vstateTab: *mut xmlValidState,
    pub am: xmlAutomataPtr,
    pub state: xmlAutomataStatePtr,
}
pub type xmlAutomataStatePtr = *mut xmlAutomataState;
pub type xmlAutomataState = _xmlAutomataState;
pub type xmlAutomataPtr = *mut xmlAutomata;
pub type xmlAutomata = _xmlAutomata;
pub type xmlValidState = _xmlValidState;
pub type xmlDocPtr = *mut xmlDoc;
pub type xmlDoc = _xmlDoc;
pub type xmlValidityWarningFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const i8, ...) -> (),
>;
pub type xmlValidityErrorFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const i8, ...) -> (),
>;
pub type xmlParserNodeInfoSeq = _xmlParserNodeInfoSeq;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlParserNodeInfoSeq {
    pub maximum: u64,
    pub length: u64,
    pub buffer: *mut xmlParserNodeInfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSAXHandler {
    pub internalSubset: internalSubsetSAXFunc,
    pub isStandalone: isStandaloneSAXFunc,
    pub hasInternalSubset: hasInternalSubsetSAXFunc,
    pub hasExternalSubset: hasExternalSubsetSAXFunc,
    pub resolveEntity: resolveEntitySAXFunc,
    pub getEntity: getEntitySAXFunc,
    pub entityDecl: entityDeclSAXFunc,
    pub notationDecl: notationDeclSAXFunc,
    pub attributeDecl: attributeDeclSAXFunc,
    pub elementDecl: elementDeclSAXFunc,
    pub unparsedEntityDecl: unparsedEntityDeclSAXFunc,
    pub setDocumentLocator: setDocumentLocatorSAXFunc,
    pub startDocument: startDocumentSAXFunc,
    pub endDocument: endDocumentSAXFunc,
    pub startElement: startElementSAXFunc,
    pub endElement: endElementSAXFunc,
    pub reference: referenceSAXFunc,
    pub characters: charactersSAXFunc,
    pub ignorableWhitespace: ignorableWhitespaceSAXFunc,
    pub processingInstruction: processingInstructionSAXFunc,
    pub comment: commentSAXFunc,
    pub warning: warningSAXFunc,
    pub error: errorSAXFunc,
    pub fatalError: fatalErrorSAXFunc,
    pub getParameterEntity: getParameterEntitySAXFunc,
    pub cdataBlock: cdataBlockSAXFunc,
    pub externalSubset: externalSubsetSAXFunc,
    pub initialized: u32,
    pub _private: *mut libc::c_void,
    pub startElementNs: startElementNsSAX2Func,
    pub endElementNs: endElementNsSAX2Func,
    pub serror: xmlStructuredErrorFunc,
}
pub type xmlStructuredErrorFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, xmlErrorPtr) -> (),
>;
pub type xmlErrorPtr = *mut xmlError;
pub type endElementNsSAX2Func = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const xmlChar,
        *const xmlChar,
        *const xmlChar,
    ) -> (),
>;
pub type startElementNsSAX2Func = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const xmlChar,
        *const xmlChar,
        *const xmlChar,
        i32,
        *mut *const xmlChar,
        i32,
        i32,
        *mut *const xmlChar,
    ) -> (),
>;
pub type externalSubsetSAXFunc = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const xmlChar,
        *const xmlChar,
        *const xmlChar,
    ) -> (),
>;
pub type cdataBlockSAXFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const xmlChar, i32) -> (),
>;
pub type getParameterEntitySAXFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const xmlChar) -> xmlEntityPtr,
>;
pub type xmlEntityPtr = *mut xmlEntity;
pub type xmlEntity = _xmlEntity;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlEntity {
    pub _private: *mut libc::c_void,
    pub type_0: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlDtd,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub orig: *mut xmlChar,
    pub content: *mut xmlChar,
    pub length: i32,
    pub etype: xmlEntityType,
    pub ExternalID: *const xmlChar,
    pub SystemID: *const xmlChar,
    pub nexte: *mut _xmlEntity,
    pub URI: *const xmlChar,
    pub owner: i32,
    pub checked: i32,
}
=======
pub type xmlValidCtxt = crate::src::python::libxml::_xmlValidCtxt;
// #[derive(Copy, Clone)]

pub type _xmlValidCtxt = crate::src::python::libxml::_xmlValidCtxt;
pub type xmlAutomataStatePtr = * mut crate::src::python::types::_xmlAutomataState;
pub type xmlAutomataState = crate::src::python::types::_xmlAutomataState;
pub type xmlAutomataPtr = * mut crate::src::python::types::_xmlAutomata;
pub type xmlAutomata = crate::src::python::types::_xmlAutomata;
pub type xmlValidState = crate::src::python::types::_xmlValidState;
pub type xmlDocPtr = * mut crate::src::python::libxml::_xmlDoc;
pub type xmlDoc = crate::src::python::libxml::_xmlDoc;
pub type xmlValidityWarningFunc<'a1, 'a2> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,_: Option<&'a2 i8>,...) -> ()>;
pub type xmlValidityErrorFunc<'a1, 'a2> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,_: Option<&'a2 i8>,...) -> ()>;
pub type xmlParserNodeInfoSeq = crate::src::python::libxml::_xmlParserNodeInfoSeq;
// #[derive(Copy, Clone)]

pub type _xmlParserNodeInfoSeq = crate::src::python::libxml::_xmlParserNodeInfoSeq;
// #[derive(Copy, Clone)]

pub type _xmlSAXHandler = crate::src::python::libxml::_xmlSAXHandler;
pub type xmlStructuredErrorFunc<'a1, 'a2> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,_: Option<&'a2 mut crate::src::python::libxml::_xmlError>,) -> ()>;
pub type xmlErrorPtr = * mut crate::src::python::libxml::_xmlError;
pub type endElementNsSAX2Func<'a1, 'a2, 'a3, 'a4> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,_: Option<&'a2 u8>,_: Option<&'a3 u8>,_: Option<&'a4 u8>,) -> ()>;
pub type startElementNsSAX2Func<'a1, 'a2, 'a3, 'a4, 'a5, 'a6, 'a7, 'a8> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,_: Option<&'a2 u8>,_: Option<&'a3 u8>,_: Option<&'a4 u8>,_: i32,_: Option<&'a5 mut Option<&'a6 u8>>,_: i32,_: i32,_: Option<&'a7 mut Option<&'a8 u8>>,) -> ()>;
pub type externalSubsetSAXFunc<'a1, 'a2, 'a3, 'a4> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,_: Option<&'a2 u8>,_: Option<&'a3 u8>,_: Option<&'a4 u8>,) -> ()>;
pub type cdataBlockSAXFunc<'a1, 'a2> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,_: Option<&'a2 u8>,_: i32,) -> ()>;
pub type getParameterEntitySAXFunc<'a1, 'a2, 'a3> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,_: Option<&'a2 u8>,) -> Option<&'a3 mut crate::src::python::libxml::_xmlEntity>>;
pub type xmlEntityPtr = * mut crate::src::python::libxml::_xmlEntity;
pub type xmlEntity = crate::src::python::libxml::_xmlEntity;
// #[derive(Copy, Clone)]

pub type _xmlEntity = crate::src::python::libxml::_xmlEntity;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
pub type xmlEntityType = u32;
pub const XML_INTERNAL_PREDEFINED_ENTITY: xmlEntityType = 6;
pub const XML_EXTERNAL_PARAMETER_ENTITY: xmlEntityType = 5;
pub const XML_INTERNAL_PARAMETER_ENTITY: xmlEntityType = 4;
pub const XML_EXTERNAL_GENERAL_UNPARSED_ENTITY: xmlEntityType = 3;
pub const XML_EXTERNAL_GENERAL_PARSED_ENTITY: xmlEntityType = 2;
pub const XML_INTERNAL_GENERAL_ENTITY: xmlEntityType = 1;
<<<<<<< HEAD
pub type fatalErrorSAXFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const i8, ...) -> (),
>;
pub type errorSAXFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const i8, ...) -> (),
>;
pub type warningSAXFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const i8, ...) -> (),
>;
pub type commentSAXFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const xmlChar) -> (),
>;
pub type processingInstructionSAXFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const xmlChar, *const xmlChar) -> (),
>;
pub type ignorableWhitespaceSAXFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const xmlChar, i32) -> (),
>;
pub type charactersSAXFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const xmlChar, i32) -> (),
>;
pub type referenceSAXFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const xmlChar) -> (),
>;
pub type endElementSAXFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const xmlChar) -> (),
>;
pub type startElementSAXFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const xmlChar, *mut *const xmlChar) -> (),
>;
pub type endDocumentSAXFunc = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type startDocumentSAXFunc = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type setDocumentLocatorSAXFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, xmlSAXLocatorPtr) -> (),
>;
pub type xmlSAXLocatorPtr = *mut xmlSAXLocator;
pub type xmlSAXLocator = _xmlSAXLocator;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSAXLocator {
    pub getPublicId: Option::<unsafe extern "C" fn(*mut libc::c_void) -> *const xmlChar>,
    pub getSystemId: Option::<unsafe extern "C" fn(*mut libc::c_void) -> *const xmlChar>,
    pub getLineNumber: Option::<unsafe extern "C" fn(*mut libc::c_void) -> i32>,
    pub getColumnNumber: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> i32,
    >,
}
pub type unparsedEntityDeclSAXFunc = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const xmlChar,
        *const xmlChar,
        *const xmlChar,
        *const xmlChar,
    ) -> (),
>;
pub type elementDeclSAXFunc = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const xmlChar,
        i32,
        xmlElementContentPtr,
    ) -> (),
>;
pub type xmlElementContentPtr = *mut xmlElementContent;
pub type xmlElementContent = _xmlElementContent;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlElementContent {
    pub type_0: xmlElementContentType,
    pub ocur: xmlElementContentOccur,
    pub name: *const xmlChar,
    pub c1: *mut _xmlElementContent,
    pub c2: *mut _xmlElementContent,
    pub parent: *mut _xmlElementContent,
    pub prefix: *const xmlChar,
}
=======
pub type fatalErrorSAXFunc<'a1, 'a2> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,_: Option<&'a2 i8>,...) -> ()>;
pub type errorSAXFunc<'a1, 'a2> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,_: Option<&'a2 i8>,...) -> ()>;
pub type warningSAXFunc<'a1, 'a2> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,_: Option<&'a2 i8>,...) -> ()>;
pub type commentSAXFunc<'a1, 'a2> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,_: Option<&'a2 u8>,) -> ()>;
pub type processingInstructionSAXFunc<'a1, 'a2, 'a3> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,_: Option<&'a2 u8>,_: Option<&'a3 u8>,) -> ()>;
pub type ignorableWhitespaceSAXFunc<'a1, 'a2> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,_: Option<&'a2 u8>,_: i32,) -> ()>;
pub type charactersSAXFunc<'a1, 'a2> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,_: Option<&'a2 u8>,_: i32,) -> ()>;
pub type referenceSAXFunc<'a1, 'a2> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,_: Option<&'a2 u8>,) -> ()>;
pub type endElementSAXFunc<'a1, 'a2> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,_: Option<&'a2 u8>,) -> ()>;
pub type startElementSAXFunc<'a1, 'a2, 'a3, 'a4> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,_: Option<&'a2 u8>,_: Option<&'a3 mut Option<&'a4 u8>>,) -> ()>;
pub type endDocumentSAXFunc<'a1> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,) -> ()>;
pub type startDocumentSAXFunc<'a1> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,) -> ()>;
pub type setDocumentLocatorSAXFunc<'a1, 'a2> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,_: Option<&'a2 mut crate::src::python::libxml::_xmlSAXLocator>,) -> ()>;
pub type xmlSAXLocatorPtr = * mut crate::src::python::libxml::_xmlSAXLocator;
pub type xmlSAXLocator = crate::src::python::libxml::_xmlSAXLocator;
// #[derive(Copy, Clone)]

pub type _xmlSAXLocator = crate::src::python::libxml::_xmlSAXLocator;
pub type unparsedEntityDeclSAXFunc<'a1, 'a2, 'a3, 'a4, 'a5> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,_: Option<&'a2 u8>,_: Option<&'a3 u8>,_: Option<&'a4 u8>,_: Option<&'a5 u8>,) -> ()>;
pub type elementDeclSAXFunc<'a1, 'a2, 'a3> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,_: Option<&'a2 u8>,_: i32,_: Option<&'a3 mut crate::src::python::libxml::_xmlElementContent>,) -> ()>;
pub type xmlElementContentPtr = * mut crate::src::python::libxml::_xmlElementContent;
pub type xmlElementContent = crate::src::python::libxml::_xmlElementContent;
// #[derive(Copy, Clone)]

pub type _xmlElementContent = crate::src::python::libxml::_xmlElementContent;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
pub type xmlElementContentOccur = u32;
pub const XML_ELEMENT_CONTENT_PLUS: xmlElementContentOccur = 4;
pub const XML_ELEMENT_CONTENT_MULT: xmlElementContentOccur = 3;
pub const XML_ELEMENT_CONTENT_OPT: xmlElementContentOccur = 2;
pub const XML_ELEMENT_CONTENT_ONCE: xmlElementContentOccur = 1;
pub type xmlElementContentType = u32;
pub const XML_ELEMENT_CONTENT_OR: xmlElementContentType = 4;
pub const XML_ELEMENT_CONTENT_SEQ: xmlElementContentType = 3;
pub const XML_ELEMENT_CONTENT_ELEMENT: xmlElementContentType = 2;
pub const XML_ELEMENT_CONTENT_PCDATA: xmlElementContentType = 1;
<<<<<<< HEAD
pub type attributeDeclSAXFunc = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const xmlChar,
        *const xmlChar,
        i32,
        i32,
        *const xmlChar,
        xmlEnumerationPtr,
    ) -> (),
>;
pub type xmlEnumerationPtr = *mut xmlEnumeration;
pub type xmlEnumeration = _xmlEnumeration;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlEnumeration {
    pub next: *mut _xmlEnumeration,
    pub name: *const xmlChar,
}
pub type notationDeclSAXFunc = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const xmlChar,
        *const xmlChar,
        *const xmlChar,
    ) -> (),
>;
pub type entityDeclSAXFunc = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const xmlChar,
        i32,
        *const xmlChar,
        *const xmlChar,
        *mut xmlChar,
    ) -> (),
>;
pub type getEntitySAXFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const xmlChar) -> xmlEntityPtr,
>;
pub type resolveEntitySAXFunc = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const xmlChar,
        *const xmlChar,
    ) -> xmlParserInputPtr,
>;
pub type hasExternalSubsetSAXFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> i32,
>;
pub type hasInternalSubsetSAXFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> i32,
>;
pub type isStandaloneSAXFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> i32,
>;
pub type internalSubsetSAXFunc = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const xmlChar,
        *const xmlChar,
        *const xmlChar,
    ) -> (),
>;
pub type xmlParserCtxt = _xmlParserCtxt;
pub type xmlParserCtxtPtr = *mut xmlParserCtxt;
=======
pub type attributeDeclSAXFunc<'a1, 'a2, 'a3, 'a4, 'a5> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,_: Option<&'a2 u8>,_: Option<&'a3 u8>,_: i32,_: i32,_: Option<&'a4 u8>,_: Option<&'a5 mut crate::src::python::libxml::_xmlEnumeration>,) -> ()>;
pub type xmlEnumerationPtr = * mut crate::src::python::libxml::_xmlEnumeration;
pub type xmlEnumeration = crate::src::python::libxml::_xmlEnumeration;
// #[derive(Copy, Clone)]

pub type _xmlEnumeration = crate::src::python::libxml::_xmlEnumeration;
pub type notationDeclSAXFunc<'a1, 'a2, 'a3, 'a4> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,_: Option<&'a2 u8>,_: Option<&'a3 u8>,_: Option<&'a4 u8>,) -> ()>;
pub type entityDeclSAXFunc<'a1, 'a2, 'a3, 'a4, 'a5> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,_: Option<&'a2 u8>,_: i32,_: Option<&'a3 u8>,_: Option<&'a4 u8>,_: Option<&'a5 mut u8>,) -> ()>;
pub type getEntitySAXFunc<'a1, 'a2, 'a3> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,_: Option<&'a2 u8>,) -> Option<&'a3 mut crate::src::python::libxml::_xmlEntity>>;
pub type resolveEntitySAXFunc<'a1, 'a2, 'a3, 'a4> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,_: Option<&'a2 u8>,_: Option<&'a3 u8>,) -> Option<&'a4 mut crate::src::python::libxml::_xmlParserInput>>;
pub type hasExternalSubsetSAXFunc<'a1> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,) -> i32>;
pub type hasInternalSubsetSAXFunc<'a1> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,) -> i32>;
pub type isStandaloneSAXFunc<'a1> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,) -> i32>;
pub type internalSubsetSAXFunc<'a1, 'a2, 'a3, 'a4> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,_: Option<&'a2 u8>,_: Option<&'a3 u8>,_: Option<&'a4 u8>,) -> ()>;
pub type xmlParserCtxt = crate::src::python::libxml::_xmlParserCtxt;
pub type xmlParserCtxtPtr = * mut crate::src::python::libxml::_xmlParserCtxt;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
pub type xmlAttributeDefault = u32;
pub const XML_ATTRIBUTE_FIXED: xmlAttributeDefault = 4;
pub const XML_ATTRIBUTE_IMPLIED: xmlAttributeDefault = 3;
pub const XML_ATTRIBUTE_REQUIRED: xmlAttributeDefault = 2;
pub const XML_ATTRIBUTE_NONE: xmlAttributeDefault = 1;
<<<<<<< HEAD
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlAttribute {
    pub _private: *mut libc::c_void,
    pub type_0: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlDtd,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub nexth: *mut _xmlAttribute,
    pub atype: xmlAttributeType,
    pub def: xmlAttributeDefault,
    pub defaultValue: *const xmlChar,
    pub tree: xmlEnumerationPtr,
    pub prefix: *const xmlChar,
    pub elem: *const xmlChar,
}
pub type xmlAttribute = _xmlAttribute;
pub type xmlAttributePtr = *mut xmlAttribute;
=======
// #[derive(Copy, Clone)]

pub type _xmlAttribute = crate::src::python::libxml2_py::_xmlAttribute;
pub type xmlAttribute = crate::src::python::libxml2_py::_xmlAttribute;
pub type xmlAttributePtr = * mut crate::src::python::libxml2_py::_xmlAttribute;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
pub type xmlElementTypeVal = u32;
pub const XML_ELEMENT_TYPE_ELEMENT: xmlElementTypeVal = 4;
pub const XML_ELEMENT_TYPE_MIXED: xmlElementTypeVal = 3;
pub const XML_ELEMENT_TYPE_ANY: xmlElementTypeVal = 2;
pub const XML_ELEMENT_TYPE_EMPTY: xmlElementTypeVal = 1;
pub const XML_ELEMENT_TYPE_UNDEFINED: xmlElementTypeVal = 0;
<<<<<<< HEAD
pub type xmlRegexp = _xmlRegexp;
pub type xmlRegexpPtr = *mut xmlRegexp;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlElement {
    pub _private: *mut libc::c_void,
    pub type_0: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlDtd,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub etype: xmlElementTypeVal,
    pub content: xmlElementContentPtr,
    pub attributes: xmlAttributePtr,
    pub prefix: *const xmlChar,
    pub contModel: xmlRegexpPtr,
}
pub type xmlElement = _xmlElement;
pub type xmlElementPtr = *mut xmlElement;
pub type xmlNsPtr = *mut xmlNs;
pub type xmlFreeFunc = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type xmlValidCtxtPtr = *mut xmlValidCtxt;
pub type xmlCatalog = _xmlCatalog;
pub type xmlCatalogPtr = *mut xmlCatalog;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlURI {
    pub scheme: *mut i8,
    pub opaque: *mut i8,
    pub authority: *mut i8,
    pub server: *mut i8,
    pub user: *mut i8,
    pub port: i32,
    pub path: *mut i8,
    pub query: *mut i8,
    pub fragment: *mut i8,
    pub cleanup: i32,
    pub query_raw: *mut i8,
}
pub type xmlURI = _xmlURI;
pub type xmlURIPtr = *mut xmlURI;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlXPathContext {
    pub doc: xmlDocPtr,
    pub node: xmlNodePtr,
    pub nb_variables_unused: i32,
    pub max_variables_unused: i32,
    pub varHash: xmlHashTablePtr,
    pub nb_types: i32,
    pub max_types: i32,
    pub types: xmlXPathTypePtr,
    pub nb_funcs_unused: i32,
    pub max_funcs_unused: i32,
    pub funcHash: xmlHashTablePtr,
    pub nb_axis: i32,
    pub max_axis: i32,
    pub axis: xmlXPathAxisPtr,
    pub namespaces: *mut xmlNsPtr,
    pub nsNr: i32,
    pub user: *mut libc::c_void,
    pub contextSize: i32,
    pub proximityPosition: i32,
    pub xptr: i32,
    pub here: xmlNodePtr,
    pub origin: xmlNodePtr,
    pub nsHash: xmlHashTablePtr,
    pub varLookupFunc: xmlXPathVariableLookupFunc,
    pub varLookupData: *mut libc::c_void,
    pub extra: *mut libc::c_void,
    pub function: *const xmlChar,
    pub functionURI: *const xmlChar,
    pub funcLookupFunc: xmlXPathFuncLookupFunc,
    pub funcLookupData: *mut libc::c_void,
    pub tmpNsList: *mut xmlNsPtr,
    pub tmpNsNr: i32,
    pub userData: *mut libc::c_void,
    pub error: xmlStructuredErrorFunc,
    pub lastError: xmlError,
    pub debugNode: xmlNodePtr,
    pub dict: xmlDictPtr,
    pub flags: i32,
    pub cache: *mut libc::c_void,
    pub opLimit: u64,
    pub opCount: u64,
    pub depth: i32,
}
pub type xmlXPathFuncLookupFunc = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const xmlChar,
        *const xmlChar,
    ) -> xmlXPathFunction,
>;
pub type xmlXPathFunction = Option::<
    unsafe extern "C" fn(xmlXPathParserContextPtr, i32) -> (),
>;
pub type xmlXPathParserContextPtr = *mut xmlXPathParserContext;
pub type xmlXPathParserContext = _xmlXPathParserContext;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlXPathParserContext {
    pub cur: *const xmlChar,
    pub base: *const xmlChar,
    pub error: i32,
    pub context: xmlXPathContextPtr,
    pub value: xmlXPathObjectPtr,
    pub valueNr: i32,
    pub valueMax: i32,
    pub valueTab: *mut xmlXPathObjectPtr,
    pub comp: xmlXPathCompExprPtr,
    pub xptr: i32,
    pub ancestor: xmlNodePtr,
    pub valueFrame: i32,
}
pub type xmlXPathCompExprPtr = *mut xmlXPathCompExpr;
pub type xmlXPathCompExpr = _xmlXPathCompExpr;
pub type xmlXPathObjectPtr = *mut xmlXPathObject;
pub type xmlXPathObject = _xmlXPathObject;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlXPathObject {
    pub type_0: xmlXPathObjectType,
    pub nodesetval: xmlNodeSetPtr,
    pub boolval: i32,
    pub floatval: f64,
    pub stringval: *mut xmlChar,
    pub user: *mut libc::c_void,
    pub index: i32,
    pub user2: *mut libc::c_void,
    pub index2: i32,
}
pub type xmlNodeSetPtr = *mut xmlNodeSet;
pub type xmlNodeSet = _xmlNodeSet;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlNodeSet {
    pub nodeNr: i32,
    pub nodeMax: i32,
    pub nodeTab: *mut xmlNodePtr,
}
=======
pub type xmlRegexp = crate::src::python::libxml2_py::_xmlRegexp;
pub type xmlRegexpPtr = * mut crate::src::python::libxml2_py::_xmlRegexp;
// #[derive(Copy, Clone)]

pub type _xmlElement = crate::src::python::libxml2_py::_xmlElement;
pub type xmlElement = crate::src::python::libxml2_py::_xmlElement;
pub type xmlElementPtr = * mut crate::src::python::libxml2_py::_xmlElement;
pub type xmlNsPtr = * mut crate::src::python::libxml::_xmlNs;
pub type xmlFreeFunc<'a1> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,) -> ()>;
pub type xmlValidCtxtPtr = * mut crate::src::python::libxml::_xmlValidCtxt;
pub type xmlCatalog = crate::src::python::types::_xmlCatalog;
pub type xmlCatalogPtr = * mut crate::src::python::types::_xmlCatalog;
// #[derive(Copy, Clone)]

pub type _xmlURI = crate::src::python::libxml2_py::_xmlURI;
pub type xmlURI = crate::src::python::libxml2_py::_xmlURI;
pub type xmlURIPtr = * mut crate::src::python::libxml2_py::_xmlURI;
// #[derive(Copy, Clone)]

pub type _xmlXPathContext = crate::src::python::libxml::_xmlXPathContext;
pub type xmlXPathFuncLookupFunc<'a1, 'a2, 'a3> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,_: Option<&'a2 u8>,_: Option<&'a3 u8>,) -> Option<unsafe extern "C"  fn(_: * mut crate::src::python::libxml::_xmlXPathParserContext,_: i32,) -> ()>>;
pub type xmlXPathFunction<'a1> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::python::libxml::_xmlXPathParserContext>,_: i32,) -> ()>;
pub type xmlXPathParserContextPtr = * mut crate::src::python::libxml::_xmlXPathParserContext;
pub type xmlXPathParserContext = crate::src::python::libxml::_xmlXPathParserContext;
// #[derive(Copy, Clone)]

pub type _xmlXPathParserContext = crate::src::python::libxml::_xmlXPathParserContext;
pub type xmlXPathCompExprPtr = * mut crate::src::python::libxml::_xmlXPathCompExpr;
pub type xmlXPathCompExpr = crate::src::python::libxml::_xmlXPathCompExpr;
pub type xmlXPathObjectPtr = * mut crate::src::python::libxml::_xmlXPathObject;
pub type xmlXPathObject = crate::src::python::libxml::_xmlXPathObject;
// #[derive(Copy, Clone)]

pub type _xmlXPathObject = crate::src::python::libxml::_xmlXPathObject;
pub type xmlNodeSetPtr = * mut crate::src::python::libxml::_xmlNodeSet;
pub type xmlNodeSet = crate::src::python::libxml::_xmlNodeSet;
// #[derive(Copy, Clone)]

pub type _xmlNodeSet = crate::src::python::libxml::_xmlNodeSet;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
pub type xmlXPathObjectType = u32;
pub const XPATH_XSLT_TREE: xmlXPathObjectType = 9;
pub const XPATH_USERS: xmlXPathObjectType = 8;
pub const XPATH_STRING: xmlXPathObjectType = 4;
pub const XPATH_NUMBER: xmlXPathObjectType = 3;
pub const XPATH_BOOLEAN: xmlXPathObjectType = 2;
pub const XPATH_NODESET: xmlXPathObjectType = 1;
pub const XPATH_UNDEFINED: xmlXPathObjectType = 0;
<<<<<<< HEAD
pub type xmlXPathContextPtr = *mut xmlXPathContext;
pub type xmlXPathContext = _xmlXPathContext;
pub type xmlXPathVariableLookupFunc = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const xmlChar,
        *const xmlChar,
    ) -> xmlXPathObjectPtr,
>;
pub type xmlXPathAxisPtr = *mut xmlXPathAxis;
pub type xmlXPathAxis = _xmlXPathAxis;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlXPathAxis {
    pub name: *const xmlChar,
    pub func: xmlXPathAxisFunc,
}
pub type xmlXPathAxisFunc = Option::<
    unsafe extern "C" fn(
        xmlXPathParserContextPtr,
        xmlXPathObjectPtr,
    ) -> xmlXPathObjectPtr,
>;
pub type xmlXPathTypePtr = *mut xmlXPathType;
pub type xmlXPathType = _xmlXPathType;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlXPathType {
    pub name: *const xmlChar,
    pub func: xmlXPathConvertFunc,
}
pub type xmlXPathConvertFunc = Option::<
    unsafe extern "C" fn(xmlXPathObjectPtr, i32) -> i32,
>;
pub type xmlRelaxNG = _xmlRelaxNG;
pub type xmlRelaxNGPtr = *mut xmlRelaxNG;
pub type xmlRelaxNGParserCtxt = _xmlRelaxNGParserCtxt;
pub type xmlRelaxNGParserCtxtPtr = *mut xmlRelaxNGParserCtxt;
pub type xmlRelaxNGValidCtxt = _xmlRelaxNGValidCtxt;
pub type xmlRelaxNGValidCtxtPtr = *mut xmlRelaxNGValidCtxt;
pub type xmlSchema = _xmlSchema;
pub type xmlSchemaPtr = *mut xmlSchema;
pub type xmlSchemaParserCtxt = _xmlSchemaParserCtxt;
pub type xmlSchemaParserCtxtPtr = *mut xmlSchemaParserCtxt;
pub type xmlSchemaValidCtxt = _xmlSchemaValidCtxt;
pub type xmlSchemaValidCtxtPtr = *mut xmlSchemaValidCtxt;
pub type xmlTextReader = _xmlTextReader;
pub type xmlTextReaderPtr = *mut xmlTextReader;
pub type xmlTextReaderLocatorPtr = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyxmlNode_Object {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut _typeobject,
    pub obj: xmlNodePtr,
}
#[no_mangle]
pub unsafe extern "C" fn libxml_intWrap(mut val: i32) -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
=======
pub type xmlXPathContextPtr = * mut crate::src::python::libxml::_xmlXPathContext;
pub type xmlXPathContext = crate::src::python::libxml::_xmlXPathContext;
pub type xmlXPathVariableLookupFunc<'a1, 'a2, 'a3, 'a4> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,_: Option<&'a2 u8>,_: Option<&'a3 u8>,) -> Option<&'a4 mut crate::src::python::libxml::_xmlXPathObject>>;
pub type xmlXPathAxisPtr = * mut crate::src::python::libxml::_xmlXPathAxis;
pub type xmlXPathAxis = crate::src::python::libxml::_xmlXPathAxis;
// #[derive(Copy, Clone)]

pub type _xmlXPathAxis = crate::src::python::libxml::_xmlXPathAxis;
pub type xmlXPathAxisFunc<'a1, 'a2, 'a3> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::python::libxml::_xmlXPathParserContext>,_: Option<&'a2 mut crate::src::python::libxml::_xmlXPathObject>,) -> Option<&'a3 mut crate::src::python::libxml::_xmlXPathObject>>;
pub type xmlXPathTypePtr = * mut crate::src::python::libxml::_xmlXPathType;
pub type xmlXPathType = crate::src::python::libxml::_xmlXPathType;
// #[derive(Copy, Clone)]

pub type _xmlXPathType = crate::src::python::libxml::_xmlXPathType;
pub type xmlXPathConvertFunc<'a1> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::python::libxml::_xmlXPathObject>,_: i32,) -> i32>;
pub type xmlRelaxNG = crate::src::python::types::_xmlRelaxNG;
pub type xmlRelaxNGPtr = * mut crate::src::python::types::_xmlRelaxNG;
pub type xmlRelaxNGParserCtxt = crate::src::python::libxml2_py::_xmlRelaxNGParserCtxt;
pub type xmlRelaxNGParserCtxtPtr = * mut crate::src::python::libxml2_py::_xmlRelaxNGParserCtxt;
pub type xmlRelaxNGValidCtxt = crate::src::python::types::_xmlRelaxNGValidCtxt;
pub type xmlRelaxNGValidCtxtPtr = * mut crate::src::python::types::_xmlRelaxNGValidCtxt;
pub type xmlSchema = crate::src::python::libxml2_py::_xmlSchema;
pub type xmlSchemaPtr = * mut crate::src::python::libxml2_py::_xmlSchema;
pub type xmlSchemaParserCtxt = crate::src::python::types::_xmlSchemaParserCtxt;
pub type xmlSchemaParserCtxtPtr = * mut crate::src::python::types::_xmlSchemaParserCtxt;
pub type xmlSchemaValidCtxt = crate::src::python::types::_xmlSchemaValidCtxt;
pub type xmlSchemaValidCtxtPtr = * mut crate::src::python::types::_xmlSchemaValidCtxt;
pub type xmlTextReader = crate::src::python::types::_xmlTextReader;
pub type xmlTextReaderPtr = * mut crate::src::python::types::_xmlTextReader;
pub type xmlTextReaderLocatorPtr = * mut core::ffi::c_void;
// #[derive(Copy, Clone)]

pub type PyxmlNode_Object<'a> = crate::src::python::libxml::PyxmlNode_Object<'a>;
#[no_mangle]
pub unsafe extern "C" fn libxml_intWrap(mut val: i32) -> * mut crate::src::python::libxml::_object {
    let mut ret: * mut crate::src::python::libxml::_object = 0 as *mut PyObject;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
    ret = PyInt_FromLong(val as i64);
    return ret;
}
#[no_mangle]
<<<<<<< HEAD
pub unsafe extern "C" fn libxml_longWrap(mut val: i64) -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
=======
pub unsafe extern "C" fn libxml_longWrap(mut val: i64) -> * mut crate::src::python::libxml::_object {
    let mut ret: * mut crate::src::python::libxml::_object = 0 as *mut PyObject;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
    ret = PyLong_FromLong(val);
    return ret;
}
#[no_mangle]
<<<<<<< HEAD
pub unsafe extern "C" fn libxml_doubleWrap(mut val: f64) -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
=======
pub unsafe extern "C" fn libxml_doubleWrap(mut val: f64) -> * mut crate::src::python::libxml::_object {
    let mut ret: * mut crate::src::python::libxml::_object = 0 as *mut PyObject;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
    ret = PyFloat_FromDouble(val);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_charPtrWrap(
<<<<<<< HEAD
    mut str: *mut i8,
) -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if str.is_null() {
        let ref mut fresh0 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
=======
    mut str: * mut i8,
) -> * mut crate::src::python::libxml::_object {
    let mut ret: * mut crate::src::python::libxml::_object = 0 as *mut PyObject;
    if str.is_null() {
        let ref mut fresh0 = (*((Some(&mut _Py_NoneStruct))).unwrap()).ob_refcnt;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
        *fresh0 += 1;
        return &mut _Py_NoneStruct;
    }
    ret = PyString_FromString(str);
    xmlFree.expect("non-null function pointer")(str as *mut libc::c_void);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_charPtrConstWrap(
<<<<<<< HEAD
    mut str: *const i8,
) -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if str.is_null() {
        let ref mut fresh1 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
=======
    mut str: * const i8,
) -> * mut crate::src::python::libxml::_object {
    let mut ret: * mut crate::src::python::libxml::_object = 0 as *mut PyObject;
    if str.is_null() {
        let ref mut fresh1 = (*((Some(&mut _Py_NoneStruct))).unwrap()).ob_refcnt;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
        *fresh1 += 1;
        return &mut _Py_NoneStruct;
    }
    ret = PyString_FromString(str);
    return ret;
}
#[no_mangle]
<<<<<<< HEAD
pub unsafe extern "C" fn libxml_xmlCharPtrWrap(mut str: *mut xmlChar) -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if str.is_null() {
        let ref mut fresh2 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
=======
pub unsafe extern "C" fn libxml_xmlCharPtrWrap(mut str: * mut u8) -> * mut crate::src::python::libxml::_object {
    let mut ret: * mut crate::src::python::libxml::_object = 0 as *mut PyObject;
    if str.is_null() {
        let ref mut fresh2 = (*((Some(&mut _Py_NoneStruct))).unwrap()).ob_refcnt;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
        *fresh2 += 1;
        return &mut _Py_NoneStruct;
    }
    ret = PyString_FromString(str as *mut i8);
    xmlFree.expect("non-null function pointer")(str as *mut libc::c_void);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlCharPtrConstWrap(
<<<<<<< HEAD
    mut str: *const xmlChar,
) -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if str.is_null() {
        let ref mut fresh3 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
=======
    mut str: * const u8,
) -> * mut crate::src::python::libxml::_object {
    let mut ret: * mut crate::src::python::libxml::_object = 0 as *mut PyObject;
    if str.is_null() {
        let ref mut fresh3 = (*((Some(&mut _Py_NoneStruct))).unwrap()).ob_refcnt;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
        *fresh3 += 1;
        return &mut _Py_NoneStruct;
    }
    ret = PyString_FromString(str as *mut i8);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_constcharPtrWrap(
<<<<<<< HEAD
    mut str: *const i8,
) -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if str.is_null() {
        let ref mut fresh4 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
=======
    mut str: * const i8,
) -> * mut crate::src::python::libxml::_object {
    let mut ret: * mut crate::src::python::libxml::_object = 0 as *mut PyObject;
    if str.is_null() {
        let ref mut fresh4 = (*((Some(&mut _Py_NoneStruct))).unwrap()).ob_refcnt;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
        *fresh4 += 1;
        return &mut _Py_NoneStruct;
    }
    ret = PyString_FromString(str);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_constxmlCharPtrWrap(
<<<<<<< HEAD
    mut str: *const xmlChar,
) -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if str.is_null() {
        let ref mut fresh5 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
=======
    mut str: * const u8,
) -> * mut crate::src::python::libxml::_object {
    let mut ret: * mut crate::src::python::libxml::_object = 0 as *mut PyObject;
    if str.is_null() {
        let ref mut fresh5 = (*((Some(&mut _Py_NoneStruct))).unwrap()).ob_refcnt;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
        *fresh5 += 1;
        return &mut _Py_NoneStruct;
    }
    ret = PyString_FromString(str as *mut i8);
    return ret;
}
#[no_mangle]
<<<<<<< HEAD
pub unsafe extern "C" fn libxml_xmlDocPtrWrap(mut doc: xmlDocPtr) -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if doc.is_null() {
        let ref mut fresh6 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
=======
pub unsafe extern "C" fn libxml_xmlDocPtrWrap(mut doc: * mut crate::src::python::libxml::_xmlDoc) -> * mut crate::src::python::libxml::_object {
    let mut ret: * mut crate::src::python::libxml::_object = 0 as *mut PyObject;
    if doc.is_null() {
        let ref mut fresh6 = (*((Some(&mut _Py_NoneStruct))).unwrap()).ob_refcnt;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
        *fresh6 += 1;
        return &mut _Py_NoneStruct;
    }
    ret = PyCapsule_New(
        doc as *mut libc::c_void,
        b"xmlDocPtr\0" as *const u8 as *const i8 as *mut i8,
        None,
    );
    return ret;
}
#[no_mangle]
<<<<<<< HEAD
pub unsafe extern "C" fn libxml_xmlNodePtrWrap(mut node: xmlNodePtr) -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if node.is_null() {
        let ref mut fresh7 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
=======
pub unsafe extern "C" fn libxml_xmlNodePtrWrap(mut node: * mut crate::src::python::libxml::_xmlNode) -> * mut crate::src::python::libxml::_object {
    let mut ret: * mut crate::src::python::libxml::_object = 0 as *mut PyObject;
    if node.is_null() {
        let ref mut fresh7 = (*((Some(&mut _Py_NoneStruct))).unwrap()).ob_refcnt;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
        *fresh7 += 1;
        return &mut _Py_NoneStruct;
    }
    ret = PyCapsule_New(
        node as *mut libc::c_void,
        b"xmlNodePtr\0" as *const u8 as *const i8 as *mut i8,
        None,
    );
    return ret;
}
#[no_mangle]
<<<<<<< HEAD
pub unsafe extern "C" fn libxml_xmlURIPtrWrap(mut uri: xmlURIPtr) -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if uri.is_null() {
        let ref mut fresh8 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
=======
pub unsafe extern "C" fn libxml_xmlURIPtrWrap(mut uri: * mut crate::src::python::libxml2_py::_xmlURI) -> * mut crate::src::python::libxml::_object {
    let mut ret: * mut crate::src::python::libxml::_object = 0 as *mut PyObject;
    if uri.is_null() {
        let ref mut fresh8 = (*((Some(&mut _Py_NoneStruct))).unwrap()).ob_refcnt;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
        *fresh8 += 1;
        return &mut _Py_NoneStruct;
    }
    ret = PyCapsule_New(
        uri as *mut libc::c_void,
        b"xmlURIPtr\0" as *const u8 as *const i8 as *mut i8,
        None,
    );
    return ret;
}
#[no_mangle]
<<<<<<< HEAD
pub unsafe extern "C" fn libxml_xmlNsPtrWrap(mut ns: xmlNsPtr) -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if ns.is_null() {
        let ref mut fresh9 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
=======
pub unsafe extern "C" fn libxml_xmlNsPtrWrap(mut ns: * mut crate::src::python::libxml::_xmlNs) -> * mut crate::src::python::libxml::_object {
    let mut ret: * mut crate::src::python::libxml::_object = 0 as *mut PyObject;
    if ns.is_null() {
        let ref mut fresh9 = (*((Some(&mut _Py_NoneStruct))).unwrap()).ob_refcnt;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
        *fresh9 += 1;
        return &mut _Py_NoneStruct;
    }
    ret = PyCapsule_New(
        ns as *mut libc::c_void,
        b"xmlNsPtr\0" as *const u8 as *const i8 as *mut i8,
        None,
    );
    return ret;
}
#[no_mangle]
<<<<<<< HEAD
pub unsafe extern "C" fn libxml_xmlAttrPtrWrap(mut attr: xmlAttrPtr) -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if attr.is_null() {
        let ref mut fresh10 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
=======
pub unsafe extern "C" fn libxml_xmlAttrPtrWrap(mut attr: * mut crate::src::python::libxml::_xmlAttr) -> * mut crate::src::python::libxml::_object {
    let mut ret: * mut crate::src::python::libxml::_object = 0 as *mut PyObject;
    if attr.is_null() {
        let ref mut fresh10 = (*((Some(&mut _Py_NoneStruct))).unwrap()).ob_refcnt;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
        *fresh10 += 1;
        return &mut _Py_NoneStruct;
    }
    ret = PyCapsule_New(
        attr as *mut libc::c_void,
        b"xmlAttrPtr\0" as *const u8 as *const i8 as *mut i8,
        None,
    );
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlAttributePtrWrap(
<<<<<<< HEAD
    mut attr: xmlAttributePtr,
) -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if attr.is_null() {
        let ref mut fresh11 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
=======
    mut attr: * mut crate::src::python::libxml2_py::_xmlAttribute,
) -> * mut crate::src::python::libxml::_object {
    let mut ret: * mut crate::src::python::libxml::_object = 0 as *mut PyObject;
    if attr.is_null() {
        let ref mut fresh11 = (*((Some(&mut _Py_NoneStruct))).unwrap()).ob_refcnt;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
        *fresh11 += 1;
        return &mut _Py_NoneStruct;
    }
    ret = PyCapsule_New(
        attr as *mut libc::c_void,
        b"xmlAttributePtr\0" as *const u8 as *const i8 as *mut i8,
        None,
    );
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlElementPtrWrap(
<<<<<<< HEAD
    mut elem: xmlElementPtr,
) -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if elem.is_null() {
        let ref mut fresh12 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
=======
    mut elem: * mut crate::src::python::libxml2_py::_xmlElement,
) -> * mut crate::src::python::libxml::_object {
    let mut ret: * mut crate::src::python::libxml::_object = 0 as *mut PyObject;
    if elem.is_null() {
        let ref mut fresh12 = (*((Some(&mut _Py_NoneStruct))).unwrap()).ob_refcnt;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
        *fresh12 += 1;
        return &mut _Py_NoneStruct;
    }
    ret = PyCapsule_New(
        elem as *mut libc::c_void,
        b"xmlElementPtr\0" as *const u8 as *const i8 as *mut i8,
        None,
    );
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlXPathContextPtrWrap(
<<<<<<< HEAD
    mut ctxt: xmlXPathContextPtr,
) -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if ctxt.is_null() {
        let ref mut fresh13 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
=======
    mut ctxt: * mut crate::src::python::libxml::_xmlXPathContext,
) -> * mut crate::src::python::libxml::_object {
    let mut ret: * mut crate::src::python::libxml::_object = 0 as *mut PyObject;
    if ctxt.is_null() {
        let ref mut fresh13 = (*((Some(&mut _Py_NoneStruct))).unwrap()).ob_refcnt;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
        *fresh13 += 1;
        return &mut _Py_NoneStruct;
    }
    ret = PyCapsule_New(
        ctxt as *mut libc::c_void,
        b"xmlXPathContextPtr\0" as *const u8 as *const i8 as *mut i8,
        None,
    );
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlXPathParserContextPtrWrap(
<<<<<<< HEAD
    mut ctxt: xmlXPathParserContextPtr,
) -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if ctxt.is_null() {
        let ref mut fresh14 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
=======
    mut ctxt: * mut crate::src::python::libxml::_xmlXPathParserContext,
) -> * mut crate::src::python::libxml::_object {
    let mut ret: * mut crate::src::python::libxml::_object = 0 as *mut PyObject;
    if ctxt.is_null() {
        let ref mut fresh14 = (*((Some(&mut _Py_NoneStruct))).unwrap()).ob_refcnt;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
        *fresh14 += 1;
        return &mut _Py_NoneStruct;
    }
    ret = PyCapsule_New(
        ctxt as *mut libc::c_void,
        b"xmlXPathParserContextPtr\0" as *const u8 as *const i8
            as *mut i8,
        None,
    );
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlParserCtxtPtrWrap(
<<<<<<< HEAD
    mut ctxt: xmlParserCtxtPtr,
) -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if ctxt.is_null() {
        let ref mut fresh15 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
=======
    mut ctxt: * mut crate::src::python::libxml::_xmlParserCtxt,
) -> * mut crate::src::python::libxml::_object {
    let mut ret: * mut crate::src::python::libxml::_object = 0 as *mut PyObject;
    if ctxt.is_null() {
        let ref mut fresh15 = (*((Some(&mut _Py_NoneStruct))).unwrap()).ob_refcnt;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
        *fresh15 += 1;
        return &mut _Py_NoneStruct;
    }
    ret = PyCapsule_New(
        ctxt as *mut libc::c_void,
        b"xmlParserCtxtPtr\0" as *const u8 as *const i8 as *mut i8,
        None,
    );
    return ret;
}
<<<<<<< HEAD
unsafe extern "C" fn libxml_xmlXPathDestructNsNode(mut cap: *mut PyObject) {
=======
unsafe extern "C" fn libxml_xmlXPathDestructNsNode(mut cap: * mut crate::src::python::libxml::_object) {
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
    xmlXPathNodeSetFreeNs(
        PyCapsule_GetPointer(cap, b"xmlNsPtr\0" as *const u8 as *const i8)
            as xmlNsPtr,
    );
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlXPathObjectPtrWrap(
<<<<<<< HEAD
    mut obj: xmlXPathObjectPtr,
) -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if obj.is_null() {
        let ref mut fresh16 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
=======
    mut obj: * mut crate::src::python::libxml::_xmlXPathObject,
) -> * mut crate::src::python::libxml::_object {
    let mut ret: * mut crate::src::python::libxml::_object = 0 as *mut PyObject;
    if obj.is_null() {
        let ref mut fresh16 = (*((Some(&mut _Py_NoneStruct))).unwrap()).ob_refcnt;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
        *fresh16 += 1;
        return &mut _Py_NoneStruct;
    }
    match (*obj).type_0 as u32 {
        9 => {
            if ((*obj).nodesetval).is_null()
                || (*(*obj).nodesetval).nodeNr == 0 as i32
                || ((*(*obj).nodesetval).nodeTab).is_null()
            {
                ret = PyList_New(0 as i32 as Py_ssize_t);
            } else {
                let mut i: i32 = 0;
                let mut len: i32 = 0 as i32;
<<<<<<< HEAD
                let mut node: xmlNodePtr = 0 as *mut xmlNode;
=======
                let mut node: * mut crate::src::python::libxml::_xmlNode = 0 as *mut xmlNode;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
                node = (**((*(*obj).nodesetval).nodeTab)
                    .offset(0 as i32 as isize))
                    .children;
                while !node.is_null() {
                    len += 1;
                    node = (*node).next;
                }
                ret = PyList_New(len as Py_ssize_t);
                node = (**((*(*obj).nodesetval).nodeTab)
                    .offset(0 as i32 as isize))
                    .children;
                i = 0 as i32;
                while i < len {
                    PyList_SetItem(ret, i as Py_ssize_t, libxml_xmlNodePtrWrap(node));
                    node = (*node).next;
                    i += 1;
                }
            }
            return ret;
        }
        1 => {
            if ((*obj).nodesetval).is_null()
                || (*(*obj).nodesetval).nodeNr == 0 as i32
            {
                ret = PyList_New(0 as i32 as Py_ssize_t);
            } else {
                let mut i_0: i32 = 0;
<<<<<<< HEAD
                let mut node_0: xmlNodePtr = 0 as *mut xmlNode;
=======
                let mut node_0: * mut crate::src::python::libxml::_xmlNode = 0 as *mut xmlNode;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
                ret = PyList_New((*(*obj).nodesetval).nodeNr as Py_ssize_t);
                i_0 = 0 as i32;
                while i_0 < (*(*obj).nodesetval).nodeNr {
                    node_0 = *((*(*obj).nodesetval).nodeTab).offset(i_0 as isize);
                    if (*node_0).type_0 as u32
                        == XML_NAMESPACE_DECL as i32 as u32
                    {
<<<<<<< HEAD
                        let mut ns: *mut PyObject = PyCapsule_New(
=======
                        let mut ns: * mut crate::src::python::libxml::_object = PyCapsule_New(
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
                            node_0 as *mut libc::c_void,
                            b"xmlNsPtr\0" as *const u8 as *const i8
                                as *mut i8,
                            Some(
<<<<<<< HEAD
                                libxml_xmlXPathDestructNsNode
                                    as unsafe extern "C" fn(*mut PyObject) -> (),
=======
                                libxml_xmlXPathDestructNsNode,
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
                            ),
                        );
                        PyList_SetItem(ret, i_0 as Py_ssize_t, ns);
                        let ref mut fresh17 = *((*(*obj).nodesetval).nodeTab)
                            .offset(i_0 as isize);
                        *fresh17 = 0 as xmlNodePtr;
                    } else {
                        PyList_SetItem(
                            ret,
                            i_0 as Py_ssize_t,
                            libxml_xmlNodePtrWrap(node_0),
                        );
                    }
                    i_0 += 1;
                }
            }
        }
        2 => {
            ret = PyInt_FromLong((*obj).boolval as i64);
        }
        3 => {
            ret = PyFloat_FromDouble((*obj).floatval);
        }
        4 => {
            ret = PyString_FromString((*obj).stringval as *mut i8);
        }
        _ => {
<<<<<<< HEAD
            let ref mut fresh18 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
=======
            let ref mut fresh18 = (*((Some(&mut _Py_NoneStruct))).unwrap()).ob_refcnt;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
            *fresh18 += 1;
            ret = &mut _Py_NoneStruct;
        }
    }
    xmlXPathFreeObject(obj);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlXPathObjectPtrConvert(
<<<<<<< HEAD
    mut obj: *mut PyObject,
) -> xmlXPathObjectPtr {
    let mut ret: xmlXPathObjectPtr = 0 as xmlXPathObjectPtr;
=======
    mut obj: * mut crate::src::python::libxml::_object,
) -> * mut crate::src::python::libxml::_xmlXPathObject {
    let mut ret: * mut crate::src::python::libxml::_xmlXPathObject = 0 as xmlXPathObjectPtr;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
    if obj.is_null() {
        return 0 as xmlXPathObjectPtr;
    }
    if (*obj).ob_type == &mut PyFloat_Type as *mut PyTypeObject
        || PyType_IsSubtype((*obj).ob_type, &mut PyFloat_Type) != 0
    {
        ret = xmlXPathNewFloat((*(obj as *mut PyFloatObject)).ob_fval);
    } else if (*(*obj).ob_type).tp_flags & (1 as i64) << 24 as i32
            != 0 as i32 as i64
        {
        ret = xmlXPathNewFloat((*(obj as *mut PyIntObject)).ob_ival as f64);
    } else if (*obj).ob_type == &mut PyBool_Type as *mut PyTypeObject {
        if obj == &mut _Py_TrueStruct as *mut PyIntObject as *mut PyObject {
            ret = xmlXPathNewBoolean(1 as i32);
        } else {
            ret = xmlXPathNewBoolean(0 as i32);
        }
    } else if (*(*obj).ob_type).tp_flags & (1 as i64) << 27 as i32
            != 0 as i32 as i64
        {
<<<<<<< HEAD
        let mut str: *mut xmlChar = 0 as *mut xmlChar;
=======
        let mut str: * mut u8 = 0 as *mut xmlChar;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
        str = xmlStrndup(
            ((*(obj as *mut PyStringObject)).ob_sval).as_mut_ptr() as *const xmlChar,
            (*(obj as *mut PyVarObject)).ob_size as i32,
        );
        ret = xmlXPathWrapString(str);
    } else if (*(*obj).ob_type).tp_flags & (1 as i64) << 28 as i32
            != 0 as i32 as i64
        {
<<<<<<< HEAD
        let mut str_0: *mut xmlChar = 0 as *mut xmlChar;
        let mut b: *mut PyObject = 0 as *mut PyObject;
=======
        let mut str_0: * mut u8 = 0 as *mut xmlChar;
        let mut b: * mut crate::src::python::libxml::_object = 0 as *mut PyObject;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
        b = PyUnicodeUCS4_AsUTF8String(obj);
        if !b.is_null() {
            str_0 = xmlStrndup(
                ((*(b as *mut PyStringObject)).ob_sval).as_mut_ptr() as *const xmlChar,
                (*(b as *mut PyVarObject)).ob_size as i32,
            );
            let ref mut fresh19 = (*b).ob_refcnt;
            *fresh19 -= 1;
            if !(*fresh19 != 0 as i32 as i64) {
                (Some(((*(*b).ob_type).tp_dealloc).expect("non-null function pointer")))
                    .expect("non-null function pointer")(b);
            }
        }
        ret = xmlXPathWrapString(str_0);
    } else if (*(*obj).ob_type).tp_flags & (1 as i64) << 25 as i32
            != 0 as i32 as i64
        {
        let mut i: i32 = 0;
<<<<<<< HEAD
        let mut node: *mut PyObject = 0 as *mut PyObject;
        let mut cur: xmlNodePtr = 0 as *mut xmlNode;
        let mut set: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
=======
        let mut node: * mut crate::src::python::libxml::_object = 0 as *mut PyObject;
        let mut cur: * mut crate::src::python::libxml::_xmlNode = 0 as *mut xmlNode;
        let mut set: * mut crate::src::python::libxml::_xmlNodeSet = 0 as *mut xmlNodeSet;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
        set = xmlXPathNodeSetCreate(0 as xmlNodePtr);
        i = 0 as i32;
        while (i as i64) < PyList_Size(obj) {
            node = PyList_GetItem(obj, i as Py_ssize_t);
            if !(node.is_null() || ((*node).ob_type).is_null()) {
                cur = 0 as xmlNodePtr;
                if (*node).ob_type == &mut PyCapsule_Type as *mut PyTypeObject {
                    cur = if node == &mut _Py_NoneStruct as *mut PyObject {
                        0 as xmlNodePtr
                    } else {
                        (*(node as *mut PyxmlNode_Object)).obj
                    };
                } else if PyObject_HasAttrString(
                        node,
                        b"_o\0" as *const u8 as *const i8 as *mut i8,
                    ) != 0
                        && PyObject_HasAttrString(
                            node,
                            b"get_doc\0" as *const u8 as *const i8
                                as *mut i8,
                        ) != 0
                    {
<<<<<<< HEAD
                    let mut wrapper: *mut PyObject = 0 as *mut PyObject;
=======
                    let mut wrapper: * mut crate::src::python::libxml::_object = 0 as *mut PyObject;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
                    wrapper = PyObject_GetAttrString(
                        node,
                        b"_o\0" as *const u8 as *const i8 as *mut i8,
                    );
                    if !wrapper.is_null() {
                        cur = if wrapper == &mut _Py_NoneStruct as *mut PyObject {
                            0 as xmlNodePtr
                        } else {
                            (*(wrapper as *mut PyxmlNode_Object)).obj
                        };
                    }
                }
                if !cur.is_null() {
                    xmlXPathNodeSetAdd(set, cur);
                }
            }
            i += 1;
        }
        ret = xmlXPathWrapNodeSet(set);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlValidCtxtPtrWrap(
<<<<<<< HEAD
    mut valid: xmlValidCtxtPtr,
) -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if valid.is_null() {
        let ref mut fresh20 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
=======
    mut valid: * mut crate::src::python::libxml::_xmlValidCtxt,
) -> * mut crate::src::python::libxml::_object {
    let mut ret: * mut crate::src::python::libxml::_object = 0 as *mut PyObject;
    if valid.is_null() {
        let ref mut fresh20 = (*((Some(&mut _Py_NoneStruct))).unwrap()).ob_refcnt;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
        *fresh20 += 1;
        return &mut _Py_NoneStruct;
    }
    ret = PyCapsule_New(
        valid as *mut libc::c_void,
        b"xmlValidCtxtPtr\0" as *const u8 as *const i8 as *mut i8,
        None,
    );
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlCatalogPtrWrap(
<<<<<<< HEAD
    mut catal: xmlCatalogPtr,
) -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if catal.is_null() {
        let ref mut fresh21 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
=======
    mut catal: * mut crate::src::python::types::_xmlCatalog,
) -> * mut crate::src::python::libxml::_object {
    let mut ret: * mut crate::src::python::libxml::_object = 0 as *mut PyObject;
    if catal.is_null() {
        let ref mut fresh21 = (*((Some(&mut _Py_NoneStruct))).unwrap()).ob_refcnt;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
        *fresh21 += 1;
        return &mut _Py_NoneStruct;
    }
    ret = PyCapsule_New(
        catal as *mut libc::c_void,
        b"xmlCatalogPtr\0" as *const u8 as *const i8 as *mut i8,
        None,
    );
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlOutputBufferPtrWrap(
<<<<<<< HEAD
    mut buffer: xmlOutputBufferPtr,
) -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if buffer.is_null() {
        let ref mut fresh22 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
=======
    mut buffer: * mut crate::src::python::libxml::_xmlOutputBuffer,
) -> * mut crate::src::python::libxml::_object {
    let mut ret: * mut crate::src::python::libxml::_object = 0 as *mut PyObject;
    if buffer.is_null() {
        let ref mut fresh22 = (*((Some(&mut _Py_NoneStruct))).unwrap()).ob_refcnt;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
        *fresh22 += 1;
        return &mut _Py_NoneStruct;
    }
    ret = PyCapsule_New(
        buffer as *mut libc::c_void,
        b"xmlOutputBufferPtr\0" as *const u8 as *const i8 as *mut i8,
        None,
    );
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlParserInputBufferPtrWrap(
<<<<<<< HEAD
    mut buffer: xmlParserInputBufferPtr,
) -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if buffer.is_null() {
        let ref mut fresh23 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
=======
    mut buffer: * mut crate::src::python::libxml::_xmlParserInputBuffer,
) -> * mut crate::src::python::libxml::_object {
    let mut ret: * mut crate::src::python::libxml::_object = 0 as *mut PyObject;
    if buffer.is_null() {
        let ref mut fresh23 = (*((Some(&mut _Py_NoneStruct))).unwrap()).ob_refcnt;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
        *fresh23 += 1;
        return &mut _Py_NoneStruct;
    }
    ret = PyCapsule_New(
        buffer as *mut libc::c_void,
        b"xmlParserInputBufferPtr\0" as *const u8 as *const i8
            as *mut i8,
        None,
    );
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlRegexpPtrWrap(
<<<<<<< HEAD
    mut regexp: xmlRegexpPtr,
) -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if regexp.is_null() {
        let ref mut fresh24 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
=======
    mut regexp: * mut crate::src::python::libxml2_py::_xmlRegexp,
) -> * mut crate::src::python::libxml::_object {
    let mut ret: * mut crate::src::python::libxml::_object = 0 as *mut PyObject;
    if regexp.is_null() {
        let ref mut fresh24 = (*((Some(&mut _Py_NoneStruct))).unwrap()).ob_refcnt;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
        *fresh24 += 1;
        return &mut _Py_NoneStruct;
    }
    ret = PyCapsule_New(
        regexp as *mut libc::c_void,
        b"xmlRegexpPtr\0" as *const u8 as *const i8 as *mut i8,
        None,
    );
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlTextReaderPtrWrap(
<<<<<<< HEAD
    mut reader: xmlTextReaderPtr,
) -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if reader.is_null() {
        let ref mut fresh25 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
=======
    mut reader: * mut crate::src::python::types::_xmlTextReader,
) -> * mut crate::src::python::libxml::_object {
    let mut ret: * mut crate::src::python::libxml::_object = 0 as *mut PyObject;
    if reader.is_null() {
        let ref mut fresh25 = (*((Some(&mut _Py_NoneStruct))).unwrap()).ob_refcnt;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
        *fresh25 += 1;
        return &mut _Py_NoneStruct;
    }
    ret = PyCapsule_New(
        reader as *mut libc::c_void,
        b"xmlTextReaderPtr\0" as *const u8 as *const i8 as *mut i8,
        None,
    );
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlTextReaderLocatorPtrWrap(
<<<<<<< HEAD
    mut locator: xmlTextReaderLocatorPtr,
) -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if locator.is_null() {
        let ref mut fresh26 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
=======
    mut locator: * mut core::ffi::c_void,
) -> * mut crate::src::python::libxml::_object {
    let mut ret: * mut crate::src::python::libxml::_object = 0 as *mut PyObject;
    if locator.is_null() {
        let ref mut fresh26 = (*((Some(&mut _Py_NoneStruct))).unwrap()).ob_refcnt;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
        *fresh26 += 1;
        return &mut _Py_NoneStruct;
    }
    ret = PyCapsule_New(
        locator,
        b"xmlTextReaderLocatorPtr\0" as *const u8 as *const i8
            as *mut i8,
        None,
    );
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlRelaxNGPtrWrap(
<<<<<<< HEAD
    mut ctxt: xmlRelaxNGPtr,
) -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if ctxt.is_null() {
        let ref mut fresh27 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
=======
    mut ctxt: * mut crate::src::python::types::_xmlRelaxNG,
) -> * mut crate::src::python::libxml::_object {
    let mut ret: * mut crate::src::python::libxml::_object = 0 as *mut PyObject;
    if ctxt.is_null() {
        let ref mut fresh27 = (*((Some(&mut _Py_NoneStruct))).unwrap()).ob_refcnt;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
        *fresh27 += 1;
        return &mut _Py_NoneStruct;
    }
    ret = PyCapsule_New(
        ctxt as *mut libc::c_void,
        b"xmlRelaxNGPtr\0" as *const u8 as *const i8 as *mut i8,
        None,
    );
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlRelaxNGParserCtxtPtrWrap(
<<<<<<< HEAD
    mut ctxt: xmlRelaxNGParserCtxtPtr,
) -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if ctxt.is_null() {
        let ref mut fresh28 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
=======
    mut ctxt: * mut crate::src::python::libxml2_py::_xmlRelaxNGParserCtxt,
) -> * mut crate::src::python::libxml::_object {
    let mut ret: * mut crate::src::python::libxml::_object = 0 as *mut PyObject;
    if ctxt.is_null() {
        let ref mut fresh28 = (*((Some(&mut _Py_NoneStruct))).unwrap()).ob_refcnt;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
        *fresh28 += 1;
        return &mut _Py_NoneStruct;
    }
    ret = PyCapsule_New(
        ctxt as *mut libc::c_void,
        b"xmlRelaxNGParserCtxtPtr\0" as *const u8 as *const i8
            as *mut i8,
        None,
    );
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlRelaxNGValidCtxtPtrWrap(
<<<<<<< HEAD
    mut valid: xmlRelaxNGValidCtxtPtr,
) -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if valid.is_null() {
        let ref mut fresh29 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
=======
    mut valid: * mut crate::src::python::types::_xmlRelaxNGValidCtxt,
) -> * mut crate::src::python::libxml::_object {
    let mut ret: * mut crate::src::python::libxml::_object = 0 as *mut PyObject;
    if valid.is_null() {
        let ref mut fresh29 = (*((Some(&mut _Py_NoneStruct))).unwrap()).ob_refcnt;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
        *fresh29 += 1;
        return &mut _Py_NoneStruct;
    }
    ret = PyCapsule_New(
        valid as *mut libc::c_void,
        b"xmlRelaxNGValidCtxtPtr\0" as *const u8 as *const i8
            as *mut i8,
        None,
    );
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlSchemaPtrWrap(
<<<<<<< HEAD
    mut ctxt: xmlSchemaPtr,
) -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if ctxt.is_null() {
        let ref mut fresh30 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
=======
    mut ctxt: * mut crate::src::python::libxml2_py::_xmlSchema,
) -> * mut crate::src::python::libxml::_object {
    let mut ret: * mut crate::src::python::libxml::_object = 0 as *mut PyObject;
    if ctxt.is_null() {
        let ref mut fresh30 = (*((Some(&mut _Py_NoneStruct))).unwrap()).ob_refcnt;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
        *fresh30 += 1;
        return &mut _Py_NoneStruct;
    }
    ret = PyCapsule_New(
        ctxt as *mut libc::c_void,
        b"xmlSchemaPtr\0" as *const u8 as *const i8 as *mut i8,
        None,
    );
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlSchemaParserCtxtPtrWrap(
<<<<<<< HEAD
    mut ctxt: xmlSchemaParserCtxtPtr,
) -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if ctxt.is_null() {
        let ref mut fresh31 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
=======
    mut ctxt: * mut crate::src::python::types::_xmlSchemaParserCtxt,
) -> * mut crate::src::python::libxml::_object {
    let mut ret: * mut crate::src::python::libxml::_object = 0 as *mut PyObject;
    if ctxt.is_null() {
        let ref mut fresh31 = (*((Some(&mut _Py_NoneStruct))).unwrap()).ob_refcnt;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
        *fresh31 += 1;
        return &mut _Py_NoneStruct;
    }
    ret = PyCapsule_New(
        ctxt as *mut libc::c_void,
        b"xmlSchemaParserCtxtPtr\0" as *const u8 as *const i8
            as *mut i8,
        None,
    );
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlSchemaValidCtxtPtrWrap(
<<<<<<< HEAD
    mut valid: xmlSchemaValidCtxtPtr,
) -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if valid.is_null() {
        let ref mut fresh32 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
=======
    mut valid: * mut crate::src::python::types::_xmlSchemaValidCtxt,
) -> * mut crate::src::python::libxml::_object {
    let mut ret: * mut crate::src::python::libxml::_object = 0 as *mut PyObject;
    if valid.is_null() {
        let ref mut fresh32 = (*((Some(&mut _Py_NoneStruct))).unwrap()).ob_refcnt;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
        *fresh32 += 1;
        return &mut _Py_NoneStruct;
    }
    ret = PyCapsule_New(
        valid as *mut libc::c_void,
        b"xmlSchemaValidCtxtPtr\0" as *const u8 as *const i8
            as *mut i8,
        None,
    );
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlErrorPtrWrap(
<<<<<<< HEAD
    mut error: xmlErrorPtr,
) -> *mut PyObject {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    if error.is_null() {
        let ref mut fresh33 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
=======
    mut error: * mut crate::src::python::libxml::_xmlError,
) -> * mut crate::src::python::libxml::_object {
    let mut ret: * mut crate::src::python::libxml::_object = 0 as *mut PyObject;
    if error.is_null() {
        let ref mut fresh33 = (*((Some(&mut _Py_NoneStruct))).unwrap()).ob_refcnt;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
        *fresh33 += 1;
        return &mut _Py_NoneStruct;
    }
    ret = PyCapsule_New(
        error as *mut libc::c_void,
        b"xmlErrorPtr\0" as *const u8 as *const i8 as *mut i8,
        None,
    );
    return ret;
}
<<<<<<< HEAD
=======
use crate::laertes_rt::*;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
