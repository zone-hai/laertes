use ::libc;
extern "C" {
<<<<<<< HEAD
    
    
    
=======
    pub type _xmlBuf;
    pub type _xmlDict;
    fn __xmlInitializeDict() -> i32;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: u64,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: i32,
        _: u64,
    ) -> *mut libc::c_void;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
<<<<<<< HEAD
    
    
    
    
=======
    fn xmlResetError(err: xmlErrorPtr);
    fn xmlInitializeGlobalState(gs: xmlGlobalStatePtr);
    fn __xmlGenericError() -> *mut xmlGenericErrorFunc;
    fn __xmlGenericErrorContext() -> *mut *mut libc::c_void;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
    fn pthread_self() -> pthread_t;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> i32;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> i32;
    fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> i32;
    fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> i32;
    fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> i32;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> i32;
    fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> i32;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> i32;
    fn pthread_key_delete(__key: pthread_key_t) -> i32;
    fn pthread_once(
        __once_control: *mut pthread_once_t,
        __init_routine: Option::<unsafe extern "C" fn() -> ()>,
    ) -> i32;
    fn pthread_getspecific(__key: pthread_key_t) -> *mut libc::c_void;
    fn pthread_setspecific(
        __key: pthread_key_t,
        __pointer: *const libc::c_void,
    ) -> i32;
    fn pthread_key_create(
        __key: *mut pthread_key_t,
        __destr_function: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    ) -> i32;
}
<<<<<<< HEAD
pub use crate::src::dict::__xmlInitializeDict;
pub use crate::src::error::xmlResetError;
pub use crate::src::globals::__xmlGenericError;
pub use crate::src::globals::__xmlGenericErrorContext;
pub use crate::src::globals::xmlInitializeGlobalState;
pub use crate::src::buf::_xmlBuf;
pub use crate::src::dict::_xmlDict;
pub type xmlChar = crate::src::HTMLparser::xmlChar;
pub type size_t = crate::src::HTMLparser::size_t;
=======
pub type xmlChar = u8;
pub type size_t = u64;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [i8; 40],
    pub __align: i64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: i32,
    pub __count: u32,
    pub __owner: i32,
    pub __nusers: u32,
    pub __kind: i32,
    pub __spins: i16,
    pub __elision: i16,
    pub __list: __pthread_list_t,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed_3 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub c2rust_unnamed: C2RustUnnamed_1,
    pub c2rust_unnamed_0: C2RustUnnamed,
    pub __g_refs: [u32; 2],
    pub __g_size: [u32; 2],
    pub __g1_orig_size: u32,
    pub __wrefs: u32,
    pub __g_signals: [u32; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __g1_start: u64,
    pub __g1_start32: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub __low: u32,
    pub __high: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub __wseq: u64,
    pub __wseq32: C2RustUnnamed_2,
}
<<<<<<< HEAD
// #[derive(Copy, Clone)]

pub type C2RustUnnamed_2 = crate::src::threads::C2RustUnnamed_0;
=======
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub __low: u32,
    pub __high: u32,
}
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
pub type pthread_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [i8; 4],
    pub __align: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_condattr_t {
    pub __size: [i8; 4],
    pub __align: i32,
}
pub type pthread_key_t = u32;
pub type pthread_once_t = i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [i8; 48],
    pub __align: i64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlMutex {
    pub lock: pthread_mutex_t,
}
<<<<<<< HEAD
pub type xmlMutex = crate::src::dict::xmlMutex;
pub type xmlMutexPtr = crate::src::dict::xmlMutexPtr;
=======
pub type xmlMutex = _xmlMutex;
pub type xmlMutexPtr = *mut xmlMutex;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlRMutex {
    pub lock: pthread_mutex_t,
    pub held: u32,
    pub waiters: u32,
    pub tid: pthread_t,
    pub cv: pthread_cond_t,
}
<<<<<<< HEAD
pub type xmlRMutex = crate::src::catalog::xmlRMutex;
pub type xmlRMutexPtr = crate::src::catalog::xmlRMutexPtr;
// #[derive(Copy, Clone)]

pub type _xmlParserInputBuffer = crate::src::HTMLparser::_xmlParserInputBuffer;
pub type xmlBufPtr = crate::src::HTMLparser::xmlBufPtr;
pub type xmlBuf = crate::src::HTMLparser::xmlBuf;
pub type xmlCharEncodingHandlerPtr = crate::src::HTMLparser::xmlCharEncodingHandlerPtr;
pub type xmlCharEncodingHandler = crate::src::HTMLparser::xmlCharEncodingHandler;
// #[derive(Copy, Clone)]

pub type _xmlCharEncodingHandler = crate::src::HTMLparser::_xmlCharEncodingHandler;
pub type iconv_t = crate::src::HTMLparser::iconv_t;
pub type xmlCharEncodingOutputFunc = crate::src::HTMLparser::xmlCharEncodingOutputFunc;
pub type xmlCharEncodingInputFunc = crate::src::HTMLparser::xmlCharEncodingInputFunc;
pub type xmlInputCloseCallback = crate::src::HTMLparser::xmlInputCloseCallback;
pub type xmlInputReadCallback = crate::src::HTMLparser::xmlInputReadCallback;
pub type xmlParserInputBuffer = crate::src::HTMLparser::xmlParserInputBuffer;
pub type xmlParserInputBufferPtr = crate::src::HTMLparser::xmlParserInputBufferPtr;
// #[derive(Copy, Clone)]

pub type _xmlOutputBuffer = crate::src::HTMLtree::_xmlOutputBuffer;
pub type xmlOutputCloseCallback = crate::src::HTMLtree::xmlOutputCloseCallback;
pub type xmlOutputWriteCallback = crate::src::HTMLtree::xmlOutputWriteCallback;
pub type xmlOutputBuffer = crate::src::HTMLtree::xmlOutputBuffer;
pub type xmlOutputBufferPtr = crate::src::HTMLtree::xmlOutputBufferPtr;
// #[derive(Copy, Clone)]

pub type _xmlParserInput = crate::src::HTMLparser::_xmlParserInput;
pub type xmlParserInputDeallocate = crate::src::HTMLparser::xmlParserInputDeallocate;
pub type xmlParserInput = crate::src::HTMLparser::xmlParserInput;
pub type xmlParserInputPtr = crate::src::HTMLparser::xmlParserInputPtr;
// #[derive(Copy, Clone)]

pub type _xmlNode = crate::src::HTMLparser::_xmlNode;
pub type xmlNs = crate::src::HTMLparser::xmlNs;
// #[derive(Copy, Clone)]

pub type _xmlNs = crate::src::HTMLparser::_xmlNs;
// #[derive(Copy, Clone)]

pub type _xmlDoc = crate::src::HTMLparser::_xmlDoc;
// #[derive(Copy, Clone)]

pub type _xmlDtd = crate::src::HTMLparser::_xmlDtd;
pub type xmlElementType = crate::src::HTMLparser::xmlElementType;
=======
pub type xmlRMutex = _xmlRMutex;
pub type xmlRMutexPtr = *mut xmlRMutex;
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
pub type xmlElementType = u32;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
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
pub type xmlNsType = crate::src::HTMLparser::xmlNsType;
// #[derive(Copy, Clone)]

pub type _xmlAttr = crate::src::HTMLparser::_xmlAttr;
pub type xmlAttributeType = crate::src::HTMLparser::xmlAttributeType;
=======
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
pub type xmlAttributeType = u32;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
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
<<<<<<< HEAD
pub type xmlError = crate::src::HTMLparser::xmlError;
// #[derive(Copy, Clone)]

pub type _xmlError = crate::src::HTMLparser::_xmlError;
pub type xmlErrorLevel = crate::src::HTMLparser::xmlErrorLevel;
=======
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
pub type xmlErrorLevel = u32;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
pub const XML_ERR_FATAL: xmlErrorLevel = 3;
pub const XML_ERR_ERROR: xmlErrorLevel = 2;
pub const XML_ERR_WARNING: xmlErrorLevel = 1;
pub const XML_ERR_NONE: xmlErrorLevel = 0;
<<<<<<< HEAD
pub type xmlNodePtr = crate::src::HTMLparser::xmlNodePtr;
pub type xmlNode = crate::src::HTMLparser::xmlNode;
pub type xmlStructuredErrorFunc = crate::src::HTMLparser::xmlStructuredErrorFunc;
pub type xmlErrorPtr = crate::src::HTMLparser::xmlErrorPtr;
pub type externalSubsetSAXFunc = crate::src::HTMLparser::externalSubsetSAXFunc;
pub type cdataBlockSAXFunc = crate::src::HTMLparser::cdataBlockSAXFunc;
pub type getParameterEntitySAXFunc = crate::src::HTMLparser::getParameterEntitySAXFunc;
pub type xmlEntityPtr = crate::src::HTMLparser::xmlEntityPtr;
pub type xmlEntity = crate::src::HTMLparser::xmlEntity;
// #[derive(Copy, Clone)]

pub type _xmlEntity = crate::src::HTMLparser::_xmlEntity;
pub type xmlEntityType = crate::src::HTMLparser::xmlEntityType;
=======
pub type xmlNodePtr = *mut xmlNode;
pub type xmlNode = _xmlNode;
pub type xmlStructuredErrorFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, xmlErrorPtr) -> (),
>;
pub type xmlErrorPtr = *mut xmlError;
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
pub type xmlEntityType = u32;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
pub const XML_INTERNAL_PREDEFINED_ENTITY: xmlEntityType = 6;
pub const XML_EXTERNAL_PARAMETER_ENTITY: xmlEntityType = 5;
pub const XML_INTERNAL_PARAMETER_ENTITY: xmlEntityType = 4;
pub const XML_EXTERNAL_GENERAL_UNPARSED_ENTITY: xmlEntityType = 3;
pub const XML_EXTERNAL_GENERAL_PARSED_ENTITY: xmlEntityType = 2;
pub const XML_INTERNAL_GENERAL_ENTITY: xmlEntityType = 1;
<<<<<<< HEAD
pub type fatalErrorSAXFunc = crate::src::HTMLparser::fatalErrorSAXFunc;
pub type errorSAXFunc = crate::src::HTMLparser::errorSAXFunc;
pub type warningSAXFunc = crate::src::HTMLparser::warningSAXFunc;
pub type commentSAXFunc = crate::src::HTMLparser::commentSAXFunc;
pub type processingInstructionSAXFunc = crate::src::HTMLparser::processingInstructionSAXFunc;
pub type ignorableWhitespaceSAXFunc = crate::src::HTMLparser::ignorableWhitespaceSAXFunc;
pub type charactersSAXFunc = crate::src::HTMLparser::charactersSAXFunc;
pub type referenceSAXFunc = crate::src::HTMLparser::referenceSAXFunc;
pub type endElementSAXFunc = crate::src::HTMLparser::endElementSAXFunc;
pub type startElementSAXFunc = crate::src::HTMLparser::startElementSAXFunc;
pub type endDocumentSAXFunc = crate::src::HTMLparser::endDocumentSAXFunc;
pub type startDocumentSAXFunc = crate::src::HTMLparser::startDocumentSAXFunc;
pub type setDocumentLocatorSAXFunc = crate::src::HTMLparser::setDocumentLocatorSAXFunc;
pub type xmlSAXLocatorPtr = crate::src::HTMLparser::xmlSAXLocatorPtr;
pub type xmlSAXLocator = crate::src::HTMLparser::xmlSAXLocator;
// #[derive(Copy, Clone)]

pub type _xmlSAXLocator = crate::src::HTMLparser::_xmlSAXLocator;
pub type unparsedEntityDeclSAXFunc = crate::src::HTMLparser::unparsedEntityDeclSAXFunc;
pub type elementDeclSAXFunc = crate::src::HTMLparser::elementDeclSAXFunc;
pub type xmlElementContentPtr = crate::src::HTMLparser::xmlElementContentPtr;
pub type xmlElementContent = crate::src::HTMLparser::xmlElementContent;
// #[derive(Copy, Clone)]

pub type _xmlElementContent = crate::src::HTMLparser::_xmlElementContent;
pub type xmlElementContentOccur = crate::src::HTMLparser::xmlElementContentOccur;
=======
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
pub type xmlElementContentOccur = u32;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
pub const XML_ELEMENT_CONTENT_PLUS: xmlElementContentOccur = 4;
pub const XML_ELEMENT_CONTENT_MULT: xmlElementContentOccur = 3;
pub const XML_ELEMENT_CONTENT_OPT: xmlElementContentOccur = 2;
pub const XML_ELEMENT_CONTENT_ONCE: xmlElementContentOccur = 1;
<<<<<<< HEAD
pub type xmlElementContentType = crate::src::HTMLparser::xmlElementContentType;
=======
pub type xmlElementContentType = u32;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
pub const XML_ELEMENT_CONTENT_OR: xmlElementContentType = 4;
pub const XML_ELEMENT_CONTENT_SEQ: xmlElementContentType = 3;
pub const XML_ELEMENT_CONTENT_ELEMENT: xmlElementContentType = 2;
pub const XML_ELEMENT_CONTENT_PCDATA: xmlElementContentType = 1;
<<<<<<< HEAD
pub type attributeDeclSAXFunc = crate::src::HTMLparser::attributeDeclSAXFunc;
pub type xmlEnumerationPtr = crate::src::HTMLparser::xmlEnumerationPtr;
pub type xmlEnumeration = crate::src::HTMLparser::xmlEnumeration;
// #[derive(Copy, Clone)]

pub type _xmlEnumeration = crate::src::HTMLparser::_xmlEnumeration;
pub type notationDeclSAXFunc = crate::src::HTMLparser::notationDeclSAXFunc;
pub type entityDeclSAXFunc = crate::src::HTMLparser::entityDeclSAXFunc;
pub type getEntitySAXFunc = crate::src::HTMLparser::getEntitySAXFunc;
pub type resolveEntitySAXFunc = crate::src::HTMLparser::resolveEntitySAXFunc;
pub type hasExternalSubsetSAXFunc = crate::src::HTMLparser::hasExternalSubsetSAXFunc;
pub type hasInternalSubsetSAXFunc = crate::src::HTMLparser::hasInternalSubsetSAXFunc;
pub type isStandaloneSAXFunc = crate::src::HTMLparser::isStandaloneSAXFunc;
pub type internalSubsetSAXFunc = crate::src::HTMLparser::internalSubsetSAXFunc;
pub type xmlBufferAllocationScheme = crate::src::HTMLtree::xmlBufferAllocationScheme;
=======
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
pub type xmlBufferAllocationScheme = u32;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
pub const XML_BUFFER_ALLOC_BOUNDED: xmlBufferAllocationScheme = 5;
pub const XML_BUFFER_ALLOC_HYBRID: xmlBufferAllocationScheme = 4;
pub const XML_BUFFER_ALLOC_IO: xmlBufferAllocationScheme = 3;
pub const XML_BUFFER_ALLOC_IMMUTABLE: xmlBufferAllocationScheme = 2;
pub const XML_BUFFER_ALLOC_EXACT: xmlBufferAllocationScheme = 1;
pub const XML_BUFFER_ALLOC_DOUBLEIT: xmlBufferAllocationScheme = 0;
<<<<<<< HEAD
pub type xmlGenericErrorFunc = crate::src::HTMLparser::xmlGenericErrorFunc;
// #[derive(Copy, Clone)]

pub type _xmlSAXHandlerV1 = crate::src::HTMLparser::_xmlSAXHandlerV1;
pub type xmlSAXHandlerV1 = crate::src::HTMLparser::xmlSAXHandlerV1;
pub type xmlCharEncoding = crate::src::HTMLparser::xmlCharEncoding;
=======
pub type xmlGenericErrorFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const i8, ...) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlSAXHandlerV1 {
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
}
pub type xmlSAXHandlerV1 = _xmlSAXHandlerV1;
pub type xmlCharEncoding = i32;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
pub const XML_CHAR_ENCODING_ASCII: xmlCharEncoding = 22;
pub const XML_CHAR_ENCODING_EUC_JP: xmlCharEncoding = 21;
pub const XML_CHAR_ENCODING_SHIFT_JIS: xmlCharEncoding = 20;
pub const XML_CHAR_ENCODING_2022_JP: xmlCharEncoding = 19;
pub const XML_CHAR_ENCODING_8859_9: xmlCharEncoding = 18;
pub const XML_CHAR_ENCODING_8859_8: xmlCharEncoding = 17;
pub const XML_CHAR_ENCODING_8859_7: xmlCharEncoding = 16;
pub const XML_CHAR_ENCODING_8859_6: xmlCharEncoding = 15;
pub const XML_CHAR_ENCODING_8859_5: xmlCharEncoding = 14;
pub const XML_CHAR_ENCODING_8859_4: xmlCharEncoding = 13;
pub const XML_CHAR_ENCODING_8859_3: xmlCharEncoding = 12;
pub const XML_CHAR_ENCODING_8859_2: xmlCharEncoding = 11;
pub const XML_CHAR_ENCODING_8859_1: xmlCharEncoding = 10;
pub const XML_CHAR_ENCODING_UCS2: xmlCharEncoding = 9;
pub const XML_CHAR_ENCODING_UCS4_3412: xmlCharEncoding = 8;
pub const XML_CHAR_ENCODING_UCS4_2143: xmlCharEncoding = 7;
pub const XML_CHAR_ENCODING_EBCDIC: xmlCharEncoding = 6;
pub const XML_CHAR_ENCODING_UCS4BE: xmlCharEncoding = 5;
pub const XML_CHAR_ENCODING_UCS4LE: xmlCharEncoding = 4;
pub const XML_CHAR_ENCODING_UTF16BE: xmlCharEncoding = 3;
pub const XML_CHAR_ENCODING_UTF16LE: xmlCharEncoding = 2;
pub const XML_CHAR_ENCODING_UTF8: xmlCharEncoding = 1;
pub const XML_CHAR_ENCODING_NONE: xmlCharEncoding = 0;
pub const XML_CHAR_ENCODING_ERROR: xmlCharEncoding = -1;
<<<<<<< HEAD
pub type xmlFreeFunc = crate::src::HTMLparser::xmlFreeFunc;
pub type xmlMallocFunc = crate::src::HTMLparser::xmlMallocFunc;
pub type xmlReallocFunc = crate::src::HTMLparser::xmlReallocFunc;
pub type xmlStrdupFunc = crate::src::encoding::xmlStrdupFunc;
pub type xmlParserInputBufferCreateFilenameFunc = crate::src::globals::xmlParserInputBufferCreateFilenameFunc;
pub type xmlOutputBufferCreateFilenameFunc = crate::src::globals::xmlOutputBufferCreateFilenameFunc;
pub type xmlRegisterNodeFunc = crate::src::HTMLparser::xmlRegisterNodeFunc;
pub type xmlDeregisterNodeFunc = crate::src::globals::xmlDeregisterNodeFunc;
// #[derive(Copy, Clone)]

pub type _xmlGlobalState = crate::src::globals::_xmlGlobalState;
pub type xmlGlobalState = crate::src::globals::xmlGlobalState;
pub type xmlGlobalStatePtr = crate::src::globals::xmlGlobalStatePtr;
=======
pub type xmlFreeFunc = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type xmlMallocFunc = Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>;
pub type xmlReallocFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
>;
pub type xmlStrdupFunc = Option::<
    unsafe extern "C" fn(*const i8) -> *mut i8,
>;
pub type xmlParserInputBufferCreateFilenameFunc = Option::<
    unsafe extern "C" fn(*const i8, xmlCharEncoding) -> xmlParserInputBufferPtr,
>;
pub type xmlOutputBufferCreateFilenameFunc = Option::<
    unsafe extern "C" fn(
        *const i8,
        xmlCharEncodingHandlerPtr,
        i32,
    ) -> xmlOutputBufferPtr,
>;
pub type xmlRegisterNodeFunc = Option::<unsafe extern "C" fn(xmlNodePtr) -> ()>;
pub type xmlDeregisterNodeFunc = Option::<unsafe extern "C" fn(xmlNodePtr) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlGlobalState {
    pub xmlParserVersion: *const i8,
    pub xmlDefaultSAXLocator: xmlSAXLocator,
    pub xmlDefaultSAXHandler: xmlSAXHandlerV1,
    pub docbDefaultSAXHandler: xmlSAXHandlerV1,
    pub htmlDefaultSAXHandler: xmlSAXHandlerV1,
    pub xmlFree: xmlFreeFunc,
    pub xmlMalloc: xmlMallocFunc,
    pub xmlMemStrdup: xmlStrdupFunc,
    pub xmlRealloc: xmlReallocFunc,
    pub xmlGenericError: xmlGenericErrorFunc,
    pub xmlStructuredError: xmlStructuredErrorFunc,
    pub xmlGenericErrorContext: *mut libc::c_void,
    pub oldXMLWDcompatibility: i32,
    pub xmlBufferAllocScheme: xmlBufferAllocationScheme,
    pub xmlDefaultBufferSize: i32,
    pub xmlSubstituteEntitiesDefaultValue: i32,
    pub xmlDoValidityCheckingDefaultValue: i32,
    pub xmlGetWarningsDefaultValue: i32,
    pub xmlKeepBlanksDefaultValue: i32,
    pub xmlLineNumbersDefaultValue: i32,
    pub xmlLoadExtDtdDefaultValue: i32,
    pub xmlParserDebugEntities: i32,
    pub xmlPedanticParserDefaultValue: i32,
    pub xmlSaveNoEmptyTags: i32,
    pub xmlIndentTreeOutput: i32,
    pub xmlTreeIndentString: *const i8,
    pub xmlRegisterNodeDefaultValue: xmlRegisterNodeFunc,
    pub xmlDeregisterNodeDefaultValue: xmlDeregisterNodeFunc,
    pub xmlMallocAtomic: xmlMallocFunc,
    pub xmlLastError: xmlError,
    pub xmlParserInputBufferCreateFilenameValue: xmlParserInputBufferCreateFilenameFunc,
    pub xmlOutputBufferCreateFilenameValue: xmlOutputBufferCreateFilenameFunc,
    pub xmlStructuredErrorContext: *mut libc::c_void,
}
pub type xmlGlobalState = _xmlGlobalState;
pub type xmlGlobalStatePtr = *mut xmlGlobalState;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
pub type C2RustUnnamed_3 = u32;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed_3 = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed_3 = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed_3 = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed_3 = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed_3 = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed_3 = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed_3 = 1;
#[inline]
<<<<<<< HEAD
 extern "C" fn pthread_equal(
=======
unsafe extern "C" fn pthread_equal(
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
    mut __thread1: pthread_t,
    mut __thread2: pthread_t,
) -> i32 {
    return (__thread1 == __thread2) as i32;
}
static mut libxml_is_threaded: i32 = -(1 as i32);
static mut globalkey: pthread_key_t = 0;
static mut mainthread: pthread_t = 0;
static mut once_control: pthread_once_t = 0 as i32;
static mut once_control_init: pthread_once_t = 0 as i32;
static mut global_init_lock: pthread_mutex_t = pthread_mutex_t {
    __data: {
        let mut init = __pthread_mutex_s {
            __lock: 0 as i32,
            __count: 0 as i32 as u32,
            __owner: 0 as i32,
            __nusers: 0 as i32 as u32,
            __kind: PTHREAD_MUTEX_TIMED_NP as i32,
            __spins: 0 as i32 as i16,
            __elision: 0 as i32 as i16,
            __list: {
                let mut init = __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};
static mut xmlLibraryLock: xmlRMutexPtr = 0 as *const xmlRMutex as xmlRMutexPtr;
#[no_mangle]
pub unsafe extern "C" fn xmlNewMutex() -> xmlMutexPtr {
    let mut tok: xmlMutexPtr = 0 as *mut xmlMutex;
    tok = malloc(::std::mem::size_of::<xmlMutex>() as u64) as xmlMutexPtr;
    if tok.is_null() {
        return 0 as xmlMutexPtr;
    }
    if libxml_is_threaded != 0 as i32 {
        pthread_mutex_init(&mut (*tok).lock, 0 as *const pthread_mutexattr_t);
    }
    return tok;
}
#[no_mangle]
pub unsafe extern "C" fn xmlFreeMutex(mut tok: xmlMutexPtr) {
    if tok.is_null() {
        return;
    }
    if libxml_is_threaded != 0 as i32 {
        pthread_mutex_destroy(&mut (*tok).lock);
    }
    free(tok as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn xmlMutexLock(mut tok: xmlMutexPtr) {
    if tok.is_null() {
        return;
    }
    if libxml_is_threaded != 0 as i32 {
        pthread_mutex_lock(&mut (*tok).lock);
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlMutexUnlock(mut tok: xmlMutexPtr) {
    if tok.is_null() {
        return;
    }
    if libxml_is_threaded != 0 as i32 {
        pthread_mutex_unlock(&mut (*tok).lock);
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlNewRMutex() -> xmlRMutexPtr {
    let mut tok: xmlRMutexPtr = 0 as *mut xmlRMutex;
    tok = malloc(::std::mem::size_of::<xmlRMutex>() as u64) as xmlRMutexPtr;
    if tok.is_null() {
        return 0 as xmlRMutexPtr;
    }
    if libxml_is_threaded != 0 as i32 {
        pthread_mutex_init(&mut (*tok).lock, 0 as *const pthread_mutexattr_t);
        (*tok).held = 0 as i32 as u32;
        (*tok).waiters = 0 as i32 as u32;
        pthread_cond_init(&mut (*tok).cv, 0 as *const pthread_condattr_t);
    }
    return tok;
}
#[no_mangle]
pub unsafe extern "C" fn xmlFreeRMutex(mut tok: xmlRMutexPtr) {
    if tok.is_null() {
        return;
    }
    if libxml_is_threaded != 0 as i32 {
        pthread_mutex_destroy(&mut (*tok).lock);
        pthread_cond_destroy(&mut (*tok).cv);
    }
    free(tok as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn xmlRMutexLock(mut tok: xmlRMutexPtr) {
    if tok.is_null() {
        return;
    }
    if libxml_is_threaded == 0 as i32 {
        return;
    }
    pthread_mutex_lock(&mut (*tok).lock);
    if (*tok).held != 0 {
        if pthread_equal((*tok).tid, pthread_self()) != 0 {
            let ref mut fresh0 = (*tok).held;
            *fresh0 = (*fresh0).wrapping_add(1);
            pthread_mutex_unlock(&mut (*tok).lock);
            return;
        } else {
            let ref mut fresh1 = (*tok).waiters;
            *fresh1 = (*fresh1).wrapping_add(1);
            while (*tok).held != 0 {
                pthread_cond_wait(&mut (*tok).cv, &mut (*tok).lock);
            }
            let ref mut fresh2 = (*tok).waiters;
            *fresh2 = (*fresh2).wrapping_sub(1);
        }
    }
    (*tok).tid = pthread_self();
    (*tok).held = 1 as i32 as u32;
    pthread_mutex_unlock(&mut (*tok).lock);
}
#[no_mangle]
pub unsafe extern "C" fn xmlRMutexUnlock(mut tok: xmlRMutexPtr) {
    if tok.is_null() {
        return;
    }
    if libxml_is_threaded == 0 as i32 {
        return;
    }
    pthread_mutex_lock(&mut (*tok).lock);
    let ref mut fresh3 = (*tok).held;
    *fresh3 = (*fresh3).wrapping_sub(1);
    if (*tok).held == 0 as i32 as u32 {
        if (*tok).waiters != 0 {
            pthread_cond_signal(&mut (*tok).cv);
        }
        memset(
            &mut (*tok).tid as *mut pthread_t as *mut libc::c_void,
            0 as i32,
            ::std::mem::size_of::<pthread_t>() as u64,
        );
    }
    pthread_mutex_unlock(&mut (*tok).lock);
}
#[no_mangle]
pub unsafe extern "C" fn __xmlGlobalInitMutexLock() {
    if (Some(
        pthread_mutex_lock as unsafe extern "C" fn(*mut pthread_mutex_t) -> i32,
    ))
        .is_none()
    {
        return;
    }
    pthread_mutex_lock(&mut global_init_lock);
}
#[no_mangle]
pub unsafe extern "C" fn __xmlGlobalInitMutexUnlock() {
    if (Some(
        pthread_mutex_unlock as unsafe extern "C" fn(*mut pthread_mutex_t) -> i32,
    ))
        .is_none()
    {
        return;
    }
    pthread_mutex_unlock(&mut global_init_lock);
}
#[no_mangle]
<<<<<<< HEAD
pub extern "C" fn __xmlGlobalInitMutexDestroy() {}
=======
pub unsafe extern "C" fn __xmlGlobalInitMutexDestroy() {}
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
unsafe extern "C" fn xmlFreeGlobalState(mut state: *mut libc::c_void) {
    let mut gs: *mut xmlGlobalState = state as *mut xmlGlobalState;
    xmlResetError(&mut (*gs).xmlLastError);
    free(state);
}
unsafe extern "C" fn xmlNewGlobalState() -> xmlGlobalStatePtr {
    let mut gs: *mut xmlGlobalState = 0 as *mut xmlGlobalState;
    gs = malloc(::std::mem::size_of::<xmlGlobalState>() as u64)
        as *mut xmlGlobalState;
    if gs.is_null() {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"xmlGetGlobalState: out of memory\n\0" as *const u8 as *const i8,
        );
        return 0 as xmlGlobalStatePtr;
    }
    memset(
        gs as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<xmlGlobalState>() as u64,
    );
    xmlInitializeGlobalState(gs);
    return gs;
}
#[no_mangle]
pub unsafe extern "C" fn xmlGetGlobalState() -> xmlGlobalStatePtr {
    let mut globalval: *mut xmlGlobalState = 0 as *mut xmlGlobalState;
    if libxml_is_threaded == 0 as i32 {
        return 0 as xmlGlobalStatePtr;
    }
    pthread_once(&mut once_control, Some(xmlOnceInit as unsafe extern "C" fn() -> ()));
    globalval = pthread_getspecific(globalkey) as *mut xmlGlobalState;
    if globalval.is_null() {
        let mut tsd: *mut xmlGlobalState = xmlNewGlobalState();
        if tsd.is_null() {
            return 0 as xmlGlobalStatePtr;
        }
        pthread_setspecific(globalkey, tsd as *const libc::c_void);
        return tsd;
    }
    return globalval;
}
#[no_mangle]
pub unsafe extern "C" fn xmlGetThreadId() -> i32 {
    let mut id: pthread_t = 0;
    let mut ret: i32 = 0;
    if libxml_is_threaded == 0 as i32 {
        return 0 as i32;
    }
    id = pthread_self();
    memcpy(
        &mut ret as *mut i32 as *mut libc::c_void,
        &mut id as *mut pthread_t as *const libc::c_void,
        ::std::mem::size_of::<i32>() as u64,
    );
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlIsMainThread() -> i32 {
    if libxml_is_threaded == -(1 as i32) {
        xmlInitThreads();
    }
    if libxml_is_threaded == 0 as i32 {
        return 1 as i32;
    }
    pthread_once(&mut once_control, Some(xmlOnceInit as unsafe extern "C" fn() -> ()));
    return pthread_equal(mainthread, pthread_self());
}
#[no_mangle]
pub unsafe extern "C" fn xmlLockLibrary() {
    xmlRMutexLock(xmlLibraryLock);
}
#[no_mangle]
pub unsafe extern "C" fn xmlUnlockLibrary() {
    xmlRMutexUnlock(xmlLibraryLock);
}
#[no_mangle]
pub unsafe extern "C" fn xmlInitThreads() {
    if libxml_is_threaded == -(1 as i32) {
        if (Some(
            pthread_once
                as unsafe extern "C" fn(
                    *mut pthread_once_t,
                    Option::<unsafe extern "C" fn() -> ()>,
                ) -> i32,
        ))
            .is_some()
            && (Some(
                pthread_getspecific
                    as unsafe extern "C" fn(pthread_key_t) -> *mut libc::c_void,
            ))
                .is_some()
            && (Some(
                pthread_setspecific
                    as unsafe extern "C" fn(
                        pthread_key_t,
                        *const libc::c_void,
                    ) -> i32,
            ))
                .is_some()
            && (Some(
                pthread_key_create
                    as unsafe extern "C" fn(
                        *mut pthread_key_t,
                        Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
                    ) -> i32,
            ))
                .is_some()
            && (Some(
                pthread_key_delete as unsafe extern "C" fn(pthread_key_t) -> i32,
            ))
                .is_some()
            && (Some(
                pthread_mutex_init
                    as unsafe extern "C" fn(
                        *mut pthread_mutex_t,
                        *const pthread_mutexattr_t,
                    ) -> i32,
            ))
                .is_some()
            && (Some(
                pthread_mutex_destroy
                    as unsafe extern "C" fn(*mut pthread_mutex_t) -> i32,
            ))
                .is_some()
            && (Some(
                pthread_mutex_lock
                    as unsafe extern "C" fn(*mut pthread_mutex_t) -> i32,
            ))
                .is_some()
            && (Some(
                pthread_mutex_unlock
                    as unsafe extern "C" fn(*mut pthread_mutex_t) -> i32,
            ))
                .is_some()
            && (Some(
                pthread_cond_init
                    as unsafe extern "C" fn(
                        *mut pthread_cond_t,
                        *const pthread_condattr_t,
                    ) -> i32,
            ))
                .is_some()
            && (Some(
                pthread_cond_destroy
                    as unsafe extern "C" fn(*mut pthread_cond_t) -> i32,
            ))
                .is_some()
            && (Some(
                pthread_cond_wait
                    as unsafe extern "C" fn(
                        *mut pthread_cond_t,
                        *mut pthread_mutex_t,
                    ) -> i32,
            ))
                .is_some()
            && (Some(
                pthread_equal
                    as unsafe extern "C" fn(pthread_t, pthread_t) -> i32,
            ))
                .is_some()
            && (Some(pthread_self as unsafe extern "C" fn() -> pthread_t)).is_some()
            && (Some(
                pthread_cond_signal
                    as unsafe extern "C" fn(*mut pthread_cond_t) -> i32,
            ))
                .is_some()
        {
            libxml_is_threaded = 1 as i32;
        } else {
            libxml_is_threaded = 0 as i32;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlCleanupThreads() {
    if libxml_is_threaded != 0 as i32 {
        pthread_key_delete(globalkey);
    }
    once_control = once_control_init;
}
unsafe extern "C" fn xmlOnceInit() {
    pthread_key_create(
        &mut globalkey,
        Some(xmlFreeGlobalState as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
    mainthread = pthread_self();
    __xmlInitializeDict();
}
