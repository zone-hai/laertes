use :: libc;
extern "C" {
    pub type _xmlMutex;
    fn __xmlGlobalInitMutexDestroy();
    fn xmlCharStrdup(cur: *const i8) -> *mut u8;
    fn xmlStrdup(cur: *const u8) -> *mut u8;
    static mut __xmlRegisterCallbacks: i32;
    fn malloc(_: u64) -> *mut core::ffi::c_void;
    fn realloc(_: *mut core::ffi::c_void, _: u64) -> *mut core::ffi::c_void;
    fn free(__ptr: *mut core::ffi::c_void);
    fn memset(_: *mut core::ffi::c_void, _: i32, _: u64) -> *mut core::ffi::c_void;
    fn __xmlParserInputBufferCreateFilename(
        URI: *const i8,
        enc: i32,
    ) -> *mut crate::src::HTMLparser::_xmlParserInputBuffer;
    fn __xmlOutputBufferCreateFilename(
        URI: *const i8,
        encoder: *mut crate::src::HTMLparser::_xmlCharEncodingHandler,
        compression: i32,
    ) -> *mut crate::src::HTMLtree::_xmlOutputBuffer;
    fn xmlNewMutex() -> *mut crate::src::globals::_xmlMutex;
    fn xmlFreeMutex(tok: *mut crate::src::globals::_xmlMutex);
    fn xmlMutexLock(tok: *mut crate::src::globals::_xmlMutex);
    fn xmlMutexUnlock(tok: *mut crate::src::globals::_xmlMutex);
    fn xmlIsMainThread() -> i32;
    fn xmlGetGlobalState() -> *mut crate::src::globals::_xmlGlobalState;
}
pub use crate::src::{
    buf::_xmlBuf,
    dict::_xmlDict,
    error::{xmlGenericErrorDefaultFunc, xmlParserError, xmlParserWarning, xmlResetError},
    SAX::{inithtmlDefaultSAXHandler, initxmlDefaultSAXHandler},
    SAX2::{
        xmlSAX2AttributeDecl, xmlSAX2CDataBlock, xmlSAX2Characters, xmlSAX2Comment,
        xmlSAX2ElementDecl, xmlSAX2EndDocument, xmlSAX2EndElement, xmlSAX2EntityDecl,
        xmlSAX2ExternalSubset, xmlSAX2GetColumnNumber, xmlSAX2GetEntity, xmlSAX2GetLineNumber,
        xmlSAX2GetParameterEntity, xmlSAX2GetPublicId, xmlSAX2GetSystemId,
        xmlSAX2HasExternalSubset, xmlSAX2HasInternalSubset, xmlSAX2IgnorableWhitespace,
        xmlSAX2InternalSubset, xmlSAX2IsStandalone, xmlSAX2NotationDecl,
        xmlSAX2ProcessingInstruction, xmlSAX2Reference, xmlSAX2ResolveEntity,
        xmlSAX2SetDocumentLocator, xmlSAX2StartDocument, xmlSAX2StartElement,
        xmlSAX2UnparsedEntityDecl,
    },
};
pub type xmlChar = u8;
pub type size_t = u64;
pub type _xmlParserInputBuffer = crate::src::HTMLparser::_xmlParserInputBuffer;
pub type xmlBufPtr = *mut crate::src::buf::_xmlBuf;
pub type xmlBuf = crate::src::buf::_xmlBuf;
pub type xmlCharEncodingHandlerPtr = *mut crate::src::HTMLparser::_xmlCharEncodingHandler;
pub type xmlCharEncodingHandler = crate::src::HTMLparser::_xmlCharEncodingHandler;
pub type _xmlCharEncodingHandler = crate::src::HTMLparser::_xmlCharEncodingHandler;
pub type iconv_t = *mut core::ffi::c_void;
pub type xmlCharEncodingOutputFunc =
    Option<unsafe extern "C" fn(_: *mut u8, _: *mut i32, _: *const u8, _: *mut i32) -> i32>;
pub type xmlCharEncodingInputFunc =
    Option<unsafe extern "C" fn(_: *mut u8, _: *mut i32, _: *const u8, _: *mut i32) -> i32>;
pub type xmlInputCloseCallback = Option<unsafe extern "C" fn(_: *mut core::ffi::c_void) -> i32>;
pub type xmlInputReadCallback =
    Option<unsafe extern "C" fn(_: *mut core::ffi::c_void, _: *mut i8, _: i32) -> i32>;
pub type xmlParserInputBuffer = crate::src::HTMLparser::_xmlParserInputBuffer;
pub type xmlParserInputBufferPtr = *mut crate::src::HTMLparser::_xmlParserInputBuffer;
pub type _xmlOutputBuffer = crate::src::HTMLtree::_xmlOutputBuffer;
pub type xmlOutputCloseCallback = Option<unsafe extern "C" fn(_: *mut core::ffi::c_void) -> i32>;
pub type xmlOutputWriteCallback =
    Option<unsafe extern "C" fn(_: *mut core::ffi::c_void, _: *const i8, _: i32) -> i32>;
pub type xmlOutputBuffer = crate::src::HTMLtree::_xmlOutputBuffer;
pub type xmlOutputBufferPtr = *mut crate::src::HTMLtree::_xmlOutputBuffer;
pub type _xmlParserInput = crate::src::HTMLparser::_xmlParserInput;
pub type xmlParserInputDeallocate = Option<unsafe extern "C" fn(_: *mut u8) -> ()>;
pub type xmlParserInput = crate::src::HTMLparser::_xmlParserInput;
pub type xmlParserInputPtr = *mut crate::src::HTMLparser::_xmlParserInput;
pub type _xmlNode = crate::src::HTMLparser::_xmlNode;
pub type xmlNs = crate::src::HTMLparser::_xmlNs;
pub type _xmlNs = crate::src::HTMLparser::_xmlNs;
pub type _xmlDoc = crate::src::HTMLparser::_xmlDoc;
pub type _xmlDtd = crate::src::HTMLparser::_xmlDtd;
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
pub type xmlNsType = u32;
pub type _xmlAttr = crate::src::HTMLparser::_xmlAttr;
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
pub type xmlError = crate::src::HTMLparser::_xmlError;
pub type _xmlError = crate::src::HTMLparser::_xmlError;
pub type xmlErrorLevel = u32;
pub const XML_ERR_FATAL: xmlErrorLevel = 3;
pub const XML_ERR_ERROR: xmlErrorLevel = 2;
pub const XML_ERR_WARNING: xmlErrorLevel = 1;
pub const XML_ERR_NONE: xmlErrorLevel = 0;
pub type xmlNodePtr = *mut crate::src::HTMLparser::_xmlNode;
pub type xmlNode = crate::src::HTMLparser::_xmlNode;
pub type xmlStructuredErrorFunc = Option<
    unsafe extern "C" fn(
        _: *mut core::ffi::c_void,
        _: *mut crate::src::HTMLparser::_xmlError,
    ) -> (),
>;
pub type xmlErrorPtr = *mut crate::src::HTMLparser::_xmlError;
pub type externalSubsetSAXFunc = Option<
    unsafe extern "C" fn(_: *mut core::ffi::c_void, _: *const u8, _: *const u8, _: *const u8) -> (),
>;
pub type cdataBlockSAXFunc =
    Option<unsafe extern "C" fn(_: *mut core::ffi::c_void, _: *const u8, _: i32) -> ()>;
pub type getParameterEntitySAXFunc = Option<
    unsafe extern "C" fn(
        _: *mut core::ffi::c_void,
        _: *const u8,
    ) -> *mut crate::src::HTMLparser::_xmlEntity,
>;
pub type xmlEntityPtr = *mut crate::src::HTMLparser::_xmlEntity;
pub type xmlEntity = crate::src::HTMLparser::_xmlEntity;
pub type _xmlEntity = crate::src::HTMLparser::_xmlEntity;
pub type xmlEntityType = u32;
pub const XML_INTERNAL_PREDEFINED_ENTITY: xmlEntityType = 6;
pub const XML_EXTERNAL_PARAMETER_ENTITY: xmlEntityType = 5;
pub const XML_INTERNAL_PARAMETER_ENTITY: xmlEntityType = 4;
pub const XML_EXTERNAL_GENERAL_UNPARSED_ENTITY: xmlEntityType = 3;
pub const XML_EXTERNAL_GENERAL_PARSED_ENTITY: xmlEntityType = 2;
pub const XML_INTERNAL_GENERAL_ENTITY: xmlEntityType = 1;
pub type fatalErrorSAXFunc =
    Option<unsafe extern "C" fn(_: *mut core::ffi::c_void, _: *const i8, ...) -> ()>;
pub type errorSAXFunc =
    Option<unsafe extern "C" fn(_: *mut core::ffi::c_void, _: *const i8, ...) -> ()>;
pub type warningSAXFunc =
    Option<unsafe extern "C" fn(_: *mut core::ffi::c_void, _: *const i8, ...) -> ()>;
pub type commentSAXFunc =
    Option<unsafe extern "C" fn(_: *mut core::ffi::c_void, _: *const u8) -> ()>;
pub type processingInstructionSAXFunc =
    Option<unsafe extern "C" fn(_: *mut core::ffi::c_void, _: *const u8, _: *const u8) -> ()>;
pub type ignorableWhitespaceSAXFunc =
    Option<unsafe extern "C" fn(_: *mut core::ffi::c_void, _: *const u8, _: i32) -> ()>;
pub type charactersSAXFunc =
    Option<unsafe extern "C" fn(_: *mut core::ffi::c_void, _: *const u8, _: i32) -> ()>;
pub type referenceSAXFunc =
    Option<unsafe extern "C" fn(_: *mut core::ffi::c_void, _: *const u8) -> ()>;
pub type endElementSAXFunc =
    Option<unsafe extern "C" fn(_: *mut core::ffi::c_void, _: *const u8) -> ()>;
pub type startElementSAXFunc =
    Option<unsafe extern "C" fn(_: *mut core::ffi::c_void, _: *const u8, _: *mut *const u8) -> ()>;
pub type endDocumentSAXFunc = Option<unsafe extern "C" fn(_: *mut core::ffi::c_void) -> ()>;
pub type startDocumentSAXFunc = Option<unsafe extern "C" fn(_: *mut core::ffi::c_void) -> ()>;
pub type setDocumentLocatorSAXFunc = Option<
    unsafe extern "C" fn(
        _: *mut core::ffi::c_void,
        _: *mut crate::src::HTMLparser::_xmlSAXLocator,
    ) -> (),
>;
pub type xmlSAXLocatorPtr = *mut crate::src::HTMLparser::_xmlSAXLocator;
pub type xmlSAXLocator = crate::src::HTMLparser::_xmlSAXLocator;
pub type _xmlSAXLocator = crate::src::HTMLparser::_xmlSAXLocator;
pub type unparsedEntityDeclSAXFunc = Option<
    unsafe extern "C" fn(
        _: *mut core::ffi::c_void,
        _: *const u8,
        _: *const u8,
        _: *const u8,
        _: *const u8,
    ) -> (),
>;
pub type elementDeclSAXFunc = Option<
    unsafe extern "C" fn(
        _: *mut core::ffi::c_void,
        _: *const u8,
        _: i32,
        _: *mut crate::src::HTMLparser::_xmlElementContent,
    ) -> (),
>;
pub type xmlElementContentPtr = *mut crate::src::HTMLparser::_xmlElementContent;
pub type xmlElementContent = crate::src::HTMLparser::_xmlElementContent;
pub type _xmlElementContent = crate::src::HTMLparser::_xmlElementContent;
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
pub type attributeDeclSAXFunc = Option<
    unsafe extern "C" fn(
        _: *mut core::ffi::c_void,
        _: *const u8,
        _: *const u8,
        _: i32,
        _: i32,
        _: *const u8,
        _: *mut crate::src::HTMLparser::_xmlEnumeration,
    ) -> (),
>;
pub type xmlEnumerationPtr = *mut crate::src::HTMLparser::_xmlEnumeration;
pub type xmlEnumeration = crate::src::HTMLparser::_xmlEnumeration;
pub type _xmlEnumeration = crate::src::HTMLparser::_xmlEnumeration;
pub type notationDeclSAXFunc = Option<
    unsafe extern "C" fn(_: *mut core::ffi::c_void, _: *const u8, _: *const u8, _: *const u8) -> (),
>;
pub type entityDeclSAXFunc = Option<
    unsafe extern "C" fn(
        _: *mut core::ffi::c_void,
        _: *const u8,
        _: i32,
        _: *const u8,
        _: *const u8,
        _: *mut u8,
    ) -> (),
>;
pub type getEntitySAXFunc = Option<
    unsafe extern "C" fn(
        _: *mut core::ffi::c_void,
        _: *const u8,
    ) -> *mut crate::src::HTMLparser::_xmlEntity,
>;
pub type resolveEntitySAXFunc = Option<
    unsafe extern "C" fn(
        _: *mut core::ffi::c_void,
        _: *const u8,
        _: *const u8,
    ) -> *mut crate::src::HTMLparser::_xmlParserInput,
>;
pub type hasExternalSubsetSAXFunc = Option<unsafe extern "C" fn(_: *mut core::ffi::c_void) -> i32>;
pub type hasInternalSubsetSAXFunc = Option<unsafe extern "C" fn(_: *mut core::ffi::c_void) -> i32>;
pub type isStandaloneSAXFunc = Option<unsafe extern "C" fn(_: *mut core::ffi::c_void) -> i32>;
pub type internalSubsetSAXFunc = Option<
    unsafe extern "C" fn(_: *mut core::ffi::c_void, _: *const u8, _: *const u8, _: *const u8) -> (),
>;
pub type xmlBufferAllocationScheme = u32;
pub const XML_BUFFER_ALLOC_BOUNDED: xmlBufferAllocationScheme = 5;
pub const XML_BUFFER_ALLOC_HYBRID: xmlBufferAllocationScheme = 4;
pub const XML_BUFFER_ALLOC_IO: xmlBufferAllocationScheme = 3;
pub const XML_BUFFER_ALLOC_IMMUTABLE: xmlBufferAllocationScheme = 2;
pub const XML_BUFFER_ALLOC_EXACT: xmlBufferAllocationScheme = 1;
pub const XML_BUFFER_ALLOC_DOUBLEIT: xmlBufferAllocationScheme = 0;
pub type xmlGenericErrorFunc =
    Option<unsafe extern "C" fn(_: *mut core::ffi::c_void, _: *const i8, ...) -> ()>;
pub type _xmlSAXHandlerV1 = crate::src::HTMLparser::_xmlSAXHandlerV1;
pub type xmlSAXHandlerV1 = crate::src::HTMLparser::_xmlSAXHandlerV1;
pub type xmlCharEncoding = i32;
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
pub type xmlFreeFunc = Option<unsafe extern "C" fn(_: *mut core::ffi::c_void) -> ()>;
pub type xmlMallocFunc = Option<unsafe extern "C" fn(_: u64) -> *mut core::ffi::c_void>;
pub type xmlReallocFunc =
    Option<unsafe extern "C" fn(_: *mut core::ffi::c_void, _: u64) -> *mut core::ffi::c_void>;
pub type xmlStrdupFunc = Option<unsafe extern "C" fn(_: *const i8) -> *mut i8>;
pub type xmlMutexPtr = *mut crate::src::globals::_xmlMutex;
pub type xmlMutex = crate::src::globals::_xmlMutex;
pub type xmlParserInputBufferCreateFilenameFunc = Option<
    unsafe extern "C" fn(
        _: *const i8,
        _: i32,
    ) -> *mut crate::src::HTMLparser::_xmlParserInputBuffer,
>;
pub type xmlOutputBufferCreateFilenameFunc = Option<
    unsafe extern "C" fn(
        _: *const i8,
        _: *mut crate::src::HTMLparser::_xmlCharEncodingHandler,
        _: i32,
    ) -> *mut crate::src::HTMLtree::_xmlOutputBuffer,
>;
pub type xmlRegisterNodeFunc =
    Option<unsafe extern "C" fn(_: *mut crate::src::HTMLparser::_xmlNode) -> ()>;
pub type xmlDeregisterNodeFunc =
    Option<unsafe extern "C" fn(_: *mut crate::src::HTMLparser::_xmlNode) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlGlobalState {
    pub xmlParserVersion: *const i8,
    pub xmlDefaultSAXLocator: crate::src::HTMLparser::_xmlSAXLocator,
    pub xmlDefaultSAXHandler: crate::src::HTMLparser::_xmlSAXHandlerV1,
    pub docbDefaultSAXHandler: crate::src::HTMLparser::_xmlSAXHandlerV1,
    pub htmlDefaultSAXHandler: crate::src::HTMLparser::_xmlSAXHandlerV1,
    pub xmlFree: Option<unsafe extern "C" fn(_: *mut core::ffi::c_void) -> ()>,
    pub xmlMalloc: Option<unsafe extern "C" fn(_: u64) -> *mut core::ffi::c_void>,
    pub xmlMemStrdup: Option<unsafe extern "C" fn(_: *const i8) -> *mut i8>,
    pub xmlRealloc:
        Option<unsafe extern "C" fn(_: *mut core::ffi::c_void, _: u64) -> *mut core::ffi::c_void>,
    pub xmlGenericError:
        Option<unsafe extern "C" fn(_: *mut core::ffi::c_void, _: *const i8, ...) -> ()>,
    pub xmlStructuredError: Option<
        unsafe extern "C" fn(
            _: *mut core::ffi::c_void,
            _: *mut crate::src::HTMLparser::_xmlError,
        ) -> (),
    >,
    pub xmlGenericErrorContext: *mut core::ffi::c_void,
    pub oldXMLWDcompatibility: i32,
    pub xmlBufferAllocScheme: u32,
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
    pub xmlRegisterNodeDefaultValue:
        Option<unsafe extern "C" fn(_: *mut crate::src::HTMLparser::_xmlNode) -> ()>,
    pub xmlDeregisterNodeDefaultValue:
        Option<unsafe extern "C" fn(_: *mut crate::src::HTMLparser::_xmlNode) -> ()>,
    pub xmlMallocAtomic: Option<unsafe extern "C" fn(_: u64) -> *mut core::ffi::c_void>,
    pub xmlLastError: crate::src::HTMLparser::_xmlError,
    pub xmlParserInputBufferCreateFilenameValue: Option<
        unsafe extern "C" fn(
            _: *const i8,
            _: i32,
        ) -> *mut crate::src::HTMLparser::_xmlParserInputBuffer,
    >,
    pub xmlOutputBufferCreateFilenameValue: Option<
        unsafe extern "C" fn(
            _: *const i8,
            _: *mut crate::src::HTMLparser::_xmlCharEncodingHandler,
            _: i32,
        ) -> *mut crate::src::HTMLtree::_xmlOutputBuffer,
    >,
    pub xmlStructuredErrorContext: *mut core::ffi::c_void,
}
impl _xmlGlobalState {
    pub const fn new() -> Self {
        _xmlGlobalState {
            xmlParserVersion: (0 as *const i8),
            xmlDefaultSAXLocator: crate::src::HTMLparser::_xmlSAXLocator::new(),
            xmlDefaultSAXHandler: crate::src::HTMLparser::_xmlSAXHandlerV1::new(),
            docbDefaultSAXHandler: crate::src::HTMLparser::_xmlSAXHandlerV1::new(),
            htmlDefaultSAXHandler: crate::src::HTMLparser::_xmlSAXHandlerV1::new(),
            xmlFree: None,
            xmlMalloc: None,
            xmlMemStrdup: None,
            xmlRealloc: None,
            xmlGenericError: None,
            xmlStructuredError: None,
            xmlGenericErrorContext: (0 as *mut core::ffi::c_void),
            oldXMLWDcompatibility: 0,
            xmlBufferAllocScheme: 0,
            xmlDefaultBufferSize: 0,
            xmlSubstituteEntitiesDefaultValue: 0,
            xmlDoValidityCheckingDefaultValue: 0,
            xmlGetWarningsDefaultValue: 0,
            xmlKeepBlanksDefaultValue: 0,
            xmlLineNumbersDefaultValue: 0,
            xmlLoadExtDtdDefaultValue: 0,
            xmlParserDebugEntities: 0,
            xmlPedanticParserDefaultValue: 0,
            xmlSaveNoEmptyTags: 0,
            xmlIndentTreeOutput: 0,
            xmlTreeIndentString: (0 as *const i8),
            xmlRegisterNodeDefaultValue: None,
            xmlDeregisterNodeDefaultValue: None,
            xmlMallocAtomic: None,
            xmlLastError: crate::src::HTMLparser::_xmlError::new(),
            xmlParserInputBufferCreateFilenameValue: None,
            xmlOutputBufferCreateFilenameValue: None,
            xmlStructuredErrorContext: (0 as *mut core::ffi::c_void),
        }
    }
}
impl std::default::Default for _xmlGlobalState {
    fn default() -> Self {
        _xmlGlobalState::new()
    }
}
pub type xmlGlobalState = crate::src::globals::_xmlGlobalState;
pub type xmlGlobalStatePtr = *mut crate::src::globals::_xmlGlobalState;
static mut xmlThrDefMutex: *mut crate::src::globals::_xmlMutex =
    0 as *const xmlMutex as xmlMutexPtr;
#[no_mangle]
pub extern "C" fn xmlInitGlobals() {
    if (unsafe { xmlThrDefMutex }).is_null() {
        (unsafe { xmlThrDefMutex = xmlNewMutex() });
    }
}
#[no_mangle]
pub static mut xmlFree: Option<unsafe extern "C" fn(_: *mut core::ffi::c_void) -> ()> =
     Some(free);
#[no_mangle]
pub static mut xmlMalloc: Option<unsafe extern "C" fn(_: u64) -> *mut core::ffi::c_void> =
     Some(malloc);
#[no_mangle]
pub static mut xmlMallocAtomic: Option<unsafe extern "C" fn(_: u64) -> *mut core::ffi::c_void> =
     Some(malloc);
#[no_mangle]
pub static mut xmlRealloc: Option<
    unsafe extern "C" fn(_: *mut core::ffi::c_void, _: u64) -> *mut core::ffi::c_void,
> =  Some(realloc);
extern "C" fn xmlPosixStrdup(mut cur: *const i8) -> *mut i8 {
    return (unsafe { xmlCharStrdup(cur) }) as *mut i8;
}
#[no_mangle]
pub static mut xmlMemStrdup: Option<unsafe extern "C" fn(_: *const i8) -> *mut i8> =
     Some(xmlPosixStrdup);
#[no_mangle]
pub static mut xmlParserVersion: *const i8 = b"21000-GITv2.10.0\0" as *const u8 as *const i8;
#[no_mangle]
pub static mut xmlBufferAllocScheme: u32 = XML_BUFFER_ALLOC_EXACT;
static mut xmlBufferAllocSchemeThrDef: u32 = XML_BUFFER_ALLOC_EXACT;
#[no_mangle]
pub static mut xmlDefaultBufferSize: i32 = 4096 as i32;
static mut xmlDefaultBufferSizeThrDef: i32 = 4096 as i32;
#[no_mangle]
pub static mut oldXMLWDcompatibility: i32 = 0 as i32;
#[no_mangle]
pub static mut xmlParserDebugEntities: i32 = 0 as i32;
static mut xmlParserDebugEntitiesThrDef: i32 = 0 as i32;
#[no_mangle]
pub static mut xmlDoValidityCheckingDefaultValue: i32 = 0 as i32;
static mut xmlDoValidityCheckingDefaultValueThrDef: i32 = 0 as i32;
#[no_mangle]
pub static mut xmlGetWarningsDefaultValue: i32 = 1 as i32;
static mut xmlGetWarningsDefaultValueThrDef: i32 = 1 as i32;
#[no_mangle]
pub static mut xmlLoadExtDtdDefaultValue: i32 = 0 as i32;
static mut xmlLoadExtDtdDefaultValueThrDef: i32 = 0 as i32;
#[no_mangle]
pub static mut xmlPedanticParserDefaultValue: i32 = 0 as i32;
static mut xmlPedanticParserDefaultValueThrDef: i32 = 0 as i32;
#[no_mangle]
pub static mut xmlLineNumbersDefaultValue: i32 = 0 as i32;
static mut xmlLineNumbersDefaultValueThrDef: i32 = 0 as i32;
#[no_mangle]
pub static mut xmlKeepBlanksDefaultValue: i32 = 1 as i32;
static mut xmlKeepBlanksDefaultValueThrDef: i32 = 1 as i32;
#[no_mangle]
pub static mut xmlSubstituteEntitiesDefaultValue: i32 = 0 as i32;
static mut xmlSubstituteEntitiesDefaultValueThrDef: i32 = 0 as i32;
#[no_mangle]
pub static mut xmlRegisterNodeDefaultValue: Option<
    unsafe extern "C" fn(_: *mut crate::src::HTMLparser::_xmlNode) -> (),
> = None;
static mut xmlRegisterNodeDefaultValueThrDef: Option<
    unsafe extern "C" fn(_: *mut crate::src::HTMLparser::_xmlNode) -> (),
> = None;
#[no_mangle]
pub static mut xmlDeregisterNodeDefaultValue: Option<
    unsafe extern "C" fn(_: *mut crate::src::HTMLparser::_xmlNode) -> (),
> = None;
static mut xmlDeregisterNodeDefaultValueThrDef: Option<
    unsafe extern "C" fn(_: *mut crate::src::HTMLparser::_xmlNode) -> (),
> = None;
#[no_mangle]
pub static mut xmlParserInputBufferCreateFilenameValue: Option<
    unsafe extern "C" fn(
        _: *const i8,
        _: i32,
    ) -> *mut crate::src::HTMLparser::_xmlParserInputBuffer,
> = None;
static mut xmlParserInputBufferCreateFilenameValueThrDef: Option<
    unsafe extern "C" fn(
        _: *const i8,
        _: i32,
    ) -> *mut crate::src::HTMLparser::_xmlParserInputBuffer,
> = None;
#[no_mangle]
pub static mut xmlOutputBufferCreateFilenameValue: Option<
    unsafe extern "C" fn(
        _: *const i8,
        _: *mut crate::src::HTMLparser::_xmlCharEncodingHandler,
        _: i32,
    ) -> *mut crate::src::HTMLtree::_xmlOutputBuffer,
> = None;
static mut xmlOutputBufferCreateFilenameValueThrDef: Option<
    unsafe extern "C" fn(
        _: *const i8,
        _: *mut crate::src::HTMLparser::_xmlCharEncodingHandler,
        _: i32,
    ) -> *mut crate::src::HTMLtree::_xmlOutputBuffer,
> = None;
#[no_mangle]
pub static mut xmlGenericError: Option<
    unsafe extern "C" fn(_: *mut core::ffi::c_void, _: *const i8, ...) -> (),
> =  Some(xmlGenericErrorDefaultFunc);
static mut xmlGenericErrorThrDef: Option<
    unsafe extern "C" fn(_: *mut core::ffi::c_void, _: *const i8, ...) -> (),
> =  Some(xmlGenericErrorDefaultFunc);
#[no_mangle]
pub static mut xmlStructuredError: Option<
    unsafe extern "C" fn(
        _: *mut core::ffi::c_void,
        _: *mut crate::src::HTMLparser::_xmlError,
    ) -> (),
> = None;
static mut xmlStructuredErrorThrDef: Option<
    unsafe extern "C" fn(
        _: *mut core::ffi::c_void,
        _: *mut crate::src::HTMLparser::_xmlError,
    ) -> (),
> = None;
#[no_mangle]
pub static mut xmlGenericErrorContext: *mut core::ffi::c_void =
    0 as *const libc::c_void as *mut libc::c_void;
static mut xmlGenericErrorContextThrDef: *mut core::ffi::c_void =
    0 as *const libc::c_void as *mut libc::c_void;
#[no_mangle]
pub static mut xmlStructuredErrorContext: *mut core::ffi::c_void =
    0 as *const libc::c_void as *mut libc::c_void;
static mut xmlStructuredErrorContextThrDef: *mut core::ffi::c_void =
    0 as *const libc::c_void as *mut libc::c_void;
#[no_mangle]
pub static mut xmlLastError: crate::src::HTMLparser::_xmlError = xmlError {
    domain: 0,
    code: 0,
    message: 0 as *const i8 as *mut i8,
    level: XML_ERR_NONE,
    file: 0 as *const i8 as *mut i8,
    line: 0,
    str1: 0 as *const i8 as *mut i8,
    str2: 0 as *const i8 as *mut i8,
    str3: 0 as *const i8 as *mut i8,
    int1: 0,
    int2: 0,
    ctxt: 0 as *const libc::c_void as *mut libc::c_void,
    node: 0 as *const libc::c_void as *mut libc::c_void,
};
#[no_mangle]
pub static mut xmlIndentTreeOutput: i32 = 1 as i32;
static mut xmlIndentTreeOutputThrDef: i32 = 1 as i32;
#[no_mangle]
pub static mut xmlTreeIndentString: *const i8 = b"  \0" as *const u8 as *const i8;
static mut xmlTreeIndentStringThrDef: *const i8 = b"  \0" as *const u8 as *const i8;
#[no_mangle]
pub static mut xmlSaveNoEmptyTags: i32 = 0 as i32;
static mut xmlSaveNoEmptyTagsThrDef: i32 = 0 as i32;
#[no_mangle]
pub static mut xmlDefaultSAXHandler: crate::src::HTMLparser::_xmlSAXHandlerV1 =  {
    {
        let mut init = _xmlSAXHandlerV1 {
            internalSubset: Some(xmlSAX2InternalSubset),
            isStandalone: Some(xmlSAX2IsStandalone),
            hasInternalSubset: Some(xmlSAX2HasInternalSubset),
            hasExternalSubset: Some(xmlSAX2HasExternalSubset),
            resolveEntity: Some(xmlSAX2ResolveEntity),
            getEntity: Some(xmlSAX2GetEntity),
            entityDecl: Some(xmlSAX2EntityDecl),
            notationDecl: Some(xmlSAX2NotationDecl),
            attributeDecl: Some(xmlSAX2AttributeDecl),
            elementDecl: Some(xmlSAX2ElementDecl),
            unparsedEntityDecl: Some(xmlSAX2UnparsedEntityDecl),
            setDocumentLocator: Some(xmlSAX2SetDocumentLocator),
            startDocument: Some(xmlSAX2StartDocument),
            endDocument: Some(xmlSAX2EndDocument),
            startElement: Some(xmlSAX2StartElement),
            endElement: Some(xmlSAX2EndElement),
            reference: Some(xmlSAX2Reference),
            characters: Some(xmlSAX2Characters),
            ignorableWhitespace: Some(xmlSAX2Characters),
            processingInstruction: Some(xmlSAX2ProcessingInstruction),
            comment: Some(xmlSAX2Comment),
            warning: Some(xmlParserWarning),
            error: Some(xmlParserError),
            fatalError: Some(xmlParserError),
            getParameterEntity: Some(xmlSAX2GetParameterEntity),
            cdataBlock: Some(xmlSAX2CDataBlock),
            externalSubset: Some(xmlSAX2ExternalSubset),
            initialized: 0 as i32 as u32,
        };
        init
    }
};
#[no_mangle]
pub static mut xmlDefaultSAXLocator: crate::src::HTMLparser::_xmlSAXLocator =  {
    {
        let mut init = _xmlSAXLocator {
            getPublicId: Some(xmlSAX2GetPublicId),
            getSystemId: Some(xmlSAX2GetSystemId),
            getLineNumber: Some(xmlSAX2GetLineNumber),
            getColumnNumber: Some(xmlSAX2GetColumnNumber),
        };
        init
    }
};
#[no_mangle]
pub static mut htmlDefaultSAXHandler: crate::src::HTMLparser::_xmlSAXHandlerV1 =  {
    {
        let mut init = _xmlSAXHandlerV1 {
            internalSubset: Some(xmlSAX2InternalSubset),
            isStandalone: None,
            hasInternalSubset: None,
            hasExternalSubset: None,
            resolveEntity: None,
            getEntity: Some(xmlSAX2GetEntity),
            entityDecl: None,
            notationDecl: None,
            attributeDecl: None,
            elementDecl: None,
            unparsedEntityDecl: None,
            setDocumentLocator: Some(xmlSAX2SetDocumentLocator),
            startDocument: Some(xmlSAX2StartDocument),
            endDocument: Some(xmlSAX2EndDocument),
            startElement: Some(xmlSAX2StartElement),
            endElement: Some(xmlSAX2EndElement),
            reference: None,
            characters: Some(xmlSAX2Characters),
            ignorableWhitespace: Some(xmlSAX2IgnorableWhitespace),
            processingInstruction: Some(xmlSAX2ProcessingInstruction),
            comment: Some(xmlSAX2Comment),
            warning: Some(xmlParserWarning),
            error: Some(xmlParserError),
            fatalError: Some(xmlParserError),
            getParameterEntity: Some(xmlSAX2GetParameterEntity),
            cdataBlock: Some(xmlSAX2CDataBlock),
            externalSubset: None,
            initialized: 0 as i32 as u32,
        };
        init
    }
};
#[no_mangle]
pub extern "C" fn xmlInitializeGlobalState<'a1>(
    mut gs: Option<&'a1 mut crate::src::globals::_xmlGlobalState>,
) {
    if (unsafe { xmlThrDefMutex }).is_null() {
        xmlInitGlobals();
    }
    (unsafe { xmlMutexLock(xmlThrDefMutex) });
    inithtmlDefaultSAXHandler(Some(
        &mut (*(borrow_mut(&mut gs)).unwrap()).htmlDefaultSAXHandler,
    ));
    (*(borrow_mut(&mut gs)).unwrap()).oldXMLWDcompatibility = 0 as i32;
    (*(borrow_mut(&mut gs)).unwrap()).xmlBufferAllocScheme = unsafe { xmlBufferAllocSchemeThrDef };
    (*(borrow_mut(&mut gs)).unwrap()).xmlDefaultBufferSize = unsafe { xmlDefaultBufferSizeThrDef };
    initxmlDefaultSAXHandler(
        Some(&mut (*(borrow_mut(&mut gs)).unwrap()).xmlDefaultSAXHandler),
        1 as i32,
    );
    let fresh0 = &mut ((*(borrow_mut(&mut gs)).unwrap())
        .xmlDefaultSAXLocator
        .getPublicId);
    *fresh0 = Some(xmlSAX2GetPublicId);
    let fresh1 = &mut ((*(borrow_mut(&mut gs)).unwrap())
        .xmlDefaultSAXLocator
        .getSystemId);
    *fresh1 = Some(xmlSAX2GetSystemId);
    let fresh2 = &mut ((*(borrow_mut(&mut gs)).unwrap())
        .xmlDefaultSAXLocator
        .getLineNumber);
    *fresh2 = Some(xmlSAX2GetLineNumber);
    let fresh3 = &mut ((*(borrow_mut(&mut gs)).unwrap())
        .xmlDefaultSAXLocator
        .getColumnNumber);
    *fresh3 = Some(xmlSAX2GetColumnNumber);
    (*(borrow_mut(&mut gs)).unwrap()).xmlDoValidityCheckingDefaultValue =
        unsafe { xmlDoValidityCheckingDefaultValueThrDef };
    let fresh4 = &mut ((*(borrow_mut(&mut gs)).unwrap()).xmlFree);
    *fresh4 = unsafe { core::intrinsics::transmute::<
        Option<unsafe extern "C" fn(_: *mut core::ffi::c_void) -> ()>,
        Option<unsafe extern "C" fn(_: *mut core::ffi::c_void) -> ()>,
    >(Some(free)) };
    let fresh5 = &mut ((*(borrow_mut(&mut gs)).unwrap()).xmlMalloc);
    *fresh5 = unsafe { core::intrinsics::transmute::<
        Option<unsafe extern "C" fn(_: u64) -> *mut core::ffi::c_void>,
        Option<unsafe extern "C" fn(_: u64) -> *mut core::ffi::c_void>,
    >(Some(malloc)) };
    let fresh6 = &mut ((*(borrow_mut(&mut gs)).unwrap()).xmlMallocAtomic);
    *fresh6 = unsafe { core::intrinsics::transmute::<
        Option<unsafe extern "C" fn(_: u64) -> *mut core::ffi::c_void>,
        Option<unsafe extern "C" fn(_: u64) -> *mut core::ffi::c_void>,
    >(Some(malloc)) };
    let fresh7 = &mut ((*(borrow_mut(&mut gs)).unwrap()).xmlRealloc);
    *fresh7 = unsafe { core::intrinsics::transmute::<
        Option<unsafe extern "C" fn(_: *mut core::ffi::c_void, _: u64) -> *mut core::ffi::c_void>,
        Option<unsafe extern "C" fn(_: *mut core::ffi::c_void, _: u64) -> *mut core::ffi::c_void>,
    >(Some(realloc)) };
    let fresh8 = &mut ((*(borrow_mut(&mut gs)).unwrap()).xmlMemStrdup);
    *fresh8 = unsafe { core::intrinsics::transmute::<
        Option<unsafe extern "C" fn(_: *const u8) -> *mut u8>,
        Option<unsafe extern "C" fn(_: *const i8) -> *mut i8>,
    >(Some(xmlStrdup)) };
    (*(borrow_mut(&mut gs)).unwrap()).xmlGetWarningsDefaultValue = unsafe { xmlGetWarningsDefaultValueThrDef };
    (*(borrow_mut(&mut gs)).unwrap()).xmlIndentTreeOutput = unsafe { xmlIndentTreeOutputThrDef };
    let fresh9 = &mut ((*(borrow_mut(&mut gs)).unwrap()).xmlTreeIndentString);
    *fresh9 = unsafe { xmlTreeIndentStringThrDef };
    (*(borrow_mut(&mut gs)).unwrap()).xmlKeepBlanksDefaultValue = unsafe { xmlKeepBlanksDefaultValueThrDef };
    (*(borrow_mut(&mut gs)).unwrap()).xmlLineNumbersDefaultValue = unsafe { xmlLineNumbersDefaultValueThrDef };
    (*(borrow_mut(&mut gs)).unwrap()).xmlLoadExtDtdDefaultValue = unsafe { xmlLoadExtDtdDefaultValueThrDef };
    (*(borrow_mut(&mut gs)).unwrap()).xmlParserDebugEntities = unsafe { xmlParserDebugEntitiesThrDef };
    let fresh10 = &mut ((*(borrow_mut(&mut gs)).unwrap()).xmlParserVersion);
    *fresh10 = b"21000\0" as *const u8 as *const i8;
    (*(borrow_mut(&mut gs)).unwrap()).xmlPedanticParserDefaultValue =
        unsafe { xmlPedanticParserDefaultValueThrDef };
    (*(borrow_mut(&mut gs)).unwrap()).xmlSaveNoEmptyTags = unsafe { xmlSaveNoEmptyTagsThrDef };
    (*(borrow_mut(&mut gs)).unwrap()).xmlSubstituteEntitiesDefaultValue =
        unsafe { xmlSubstituteEntitiesDefaultValueThrDef };
    let fresh11 = &mut ((*(borrow_mut(&mut gs)).unwrap()).xmlGenericError);
    *fresh11 = unsafe { xmlGenericErrorThrDef };
    let fresh12 = &mut ((*(borrow_mut(&mut gs)).unwrap()).xmlStructuredError);
    *fresh12 = unsafe { xmlStructuredErrorThrDef };
    let fresh13 = &mut ((*(borrow_mut(&mut gs)).unwrap()).xmlGenericErrorContext);
    *fresh13 = unsafe { xmlGenericErrorContextThrDef };
    let fresh14 = &mut ((*(borrow_mut(&mut gs)).unwrap()).xmlStructuredErrorContext);
    *fresh14 = unsafe { xmlStructuredErrorContextThrDef };
    let fresh15 = &mut ((*(borrow_mut(&mut gs)).unwrap()).xmlRegisterNodeDefaultValue);
    *fresh15 = unsafe { xmlRegisterNodeDefaultValueThrDef };
    let fresh16 = &mut ((*(borrow_mut(&mut gs)).unwrap()).xmlDeregisterNodeDefaultValue);
    *fresh16 = unsafe { xmlDeregisterNodeDefaultValueThrDef };
    let fresh17 = &mut ((*(borrow_mut(&mut gs)).unwrap()).xmlParserInputBufferCreateFilenameValue);
    *fresh17 = unsafe { xmlParserInputBufferCreateFilenameValueThrDef };
    let fresh18 = &mut ((*(borrow_mut(&mut gs)).unwrap()).xmlOutputBufferCreateFilenameValue);
    *fresh18 = unsafe { xmlOutputBufferCreateFilenameValueThrDef };
    (unsafe { memset(
        &mut (*(borrow_mut(&mut gs)).unwrap()).xmlLastError as *mut xmlError as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<xmlError>() as u64,
    ) });
    (unsafe { xmlMutexUnlock(xmlThrDefMutex) });
}
#[no_mangle]
pub extern "C" fn xmlCleanupGlobals() {
    xmlResetError(unsafe { &mut xmlLastError });
    if !(unsafe { xmlThrDefMutex }).is_null() {
        (unsafe { xmlFreeMutex(xmlThrDefMutex) });
        (unsafe { xmlThrDefMutex = 0 as xmlMutexPtr });
    }
    (unsafe { __xmlGlobalInitMutexDestroy() });
}
#[no_mangle]
pub extern "C" fn xmlThrDefSetGenericErrorFunc(
    mut ctx: *mut core::ffi::c_void,
    mut handler: Option<unsafe extern "C" fn(_: *mut core::ffi::c_void, _: *const i8, ...) -> ()>,
) {
    (unsafe { xmlMutexLock(xmlThrDefMutex) });
    (unsafe { xmlGenericErrorContextThrDef = ctx });
    if handler.is_some() {
        (unsafe { xmlGenericErrorThrDef = handler });
    } else {
        (unsafe { xmlGenericErrorThrDef = Some(xmlGenericErrorDefaultFunc) });
    }
    (unsafe { xmlMutexUnlock(xmlThrDefMutex) });
}
#[no_mangle]
pub extern "C" fn xmlThrDefSetStructuredErrorFunc(
    mut ctx: *mut core::ffi::c_void,
    mut handler: Option<
        unsafe extern "C" fn(
            _: *mut core::ffi::c_void,
            _: *mut crate::src::HTMLparser::_xmlError,
        ) -> (),
    >,
) {
    (unsafe { xmlMutexLock(xmlThrDefMutex) });
    (unsafe { xmlStructuredErrorContextThrDef = ctx });
    (unsafe { xmlStructuredErrorThrDef = handler });
    (unsafe { xmlMutexUnlock(xmlThrDefMutex) });
}
#[no_mangle]
pub extern "C" fn xmlRegisterNodeDefault(
    mut func: Option<unsafe extern "C" fn(_: *mut crate::src::HTMLparser::_xmlNode) -> ()>,
) -> Option<unsafe extern "C" fn(_: *mut crate::src::HTMLparser::_xmlNode) -> ()> {
    let mut old: Option<unsafe extern "C" fn(_: *mut crate::src::HTMLparser::_xmlNode) -> ()> =
        unsafe { xmlRegisterNodeDefaultValue };
    (unsafe { __xmlRegisterCallbacks = 1 as i32 });
    (unsafe { xmlRegisterNodeDefaultValue = func });
    return old;
}
#[no_mangle]
pub extern "C" fn xmlThrDefRegisterNodeDefault(
    mut func: Option<unsafe extern "C" fn(_: *mut crate::src::HTMLparser::_xmlNode) -> ()>,
) -> Option<unsafe extern "C" fn(_: *mut crate::src::HTMLparser::_xmlNode) -> ()> {
    let mut old: Option<unsafe extern "C" fn(_: *mut crate::src::HTMLparser::_xmlNode) -> ()> =
        None;
    (unsafe { xmlMutexLock(xmlThrDefMutex) });
    old = unsafe { xmlRegisterNodeDefaultValueThrDef };
    (unsafe { __xmlRegisterCallbacks = 1 as i32 });
    (unsafe { xmlRegisterNodeDefaultValueThrDef = func });
    (unsafe { xmlMutexUnlock(xmlThrDefMutex) });
    return old;
}
#[no_mangle]
pub extern "C" fn xmlDeregisterNodeDefault(
    mut func: Option<unsafe extern "C" fn(_: *mut crate::src::HTMLparser::_xmlNode) -> ()>,
) -> Option<unsafe extern "C" fn(_: *mut crate::src::HTMLparser::_xmlNode) -> ()> {
    let mut old: Option<unsafe extern "C" fn(_: *mut crate::src::HTMLparser::_xmlNode) -> ()> =
        unsafe { xmlDeregisterNodeDefaultValue };
    (unsafe { __xmlRegisterCallbacks = 1 as i32 });
    (unsafe { xmlDeregisterNodeDefaultValue = func });
    return old;
}
#[no_mangle]
pub extern "C" fn xmlThrDefDeregisterNodeDefault(
    mut func: Option<unsafe extern "C" fn(_: *mut crate::src::HTMLparser::_xmlNode) -> ()>,
) -> Option<unsafe extern "C" fn(_: *mut crate::src::HTMLparser::_xmlNode) -> ()> {
    let mut old: Option<unsafe extern "C" fn(_: *mut crate::src::HTMLparser::_xmlNode) -> ()> =
        None;
    (unsafe { xmlMutexLock(xmlThrDefMutex) });
    old = unsafe { xmlDeregisterNodeDefaultValueThrDef };
    (unsafe { __xmlRegisterCallbacks = 1 as i32 });
    (unsafe { xmlDeregisterNodeDefaultValueThrDef = func });
    (unsafe { xmlMutexUnlock(xmlThrDefMutex) });
    return old;
}
#[no_mangle]
pub extern "C" fn xmlThrDefParserInputBufferCreateFilenameDefault(
    mut func: Option<
        unsafe extern "C" fn(
            _: *const i8,
            _: i32,
        ) -> *mut crate::src::HTMLparser::_xmlParserInputBuffer,
    >,
) -> Option<
    unsafe extern "C" fn(
        _: *const i8,
        _: i32,
    ) -> *mut crate::src::HTMLparser::_xmlParserInputBuffer,
> {
    let mut old: Option<
        unsafe extern "C" fn(
            _: *const i8,
            _: i32,
        ) -> *mut crate::src::HTMLparser::_xmlParserInputBuffer,
    > = None;
    (unsafe { xmlMutexLock(xmlThrDefMutex) });
    old = unsafe { xmlParserInputBufferCreateFilenameValueThrDef };
    if old.is_none() {
        old = Some(__xmlParserInputBufferCreateFilename);
    }
    (unsafe { xmlParserInputBufferCreateFilenameValueThrDef = func });
    (unsafe { xmlMutexUnlock(xmlThrDefMutex) });
    return old;
}
#[no_mangle]
pub extern "C" fn xmlThrDefOutputBufferCreateFilenameDefault(
    mut func: Option<
        unsafe extern "C" fn(
            _: *const i8,
            _: *mut crate::src::HTMLparser::_xmlCharEncodingHandler,
            _: i32,
        ) -> *mut crate::src::HTMLtree::_xmlOutputBuffer,
    >,
) -> Option<
    unsafe extern "C" fn(
        _: *const i8,
        _: *mut crate::src::HTMLparser::_xmlCharEncodingHandler,
        _: i32,
    ) -> *mut crate::src::HTMLtree::_xmlOutputBuffer,
> {
    let mut old: Option<
        unsafe extern "C" fn(
            _: *const i8,
            _: *mut crate::src::HTMLparser::_xmlCharEncodingHandler,
            _: i32,
        ) -> *mut crate::src::HTMLtree::_xmlOutputBuffer,
    > = None;
    (unsafe { xmlMutexLock(xmlThrDefMutex) });
    old = unsafe { xmlOutputBufferCreateFilenameValueThrDef };
    if old.is_none() {
        old = Some(__xmlOutputBufferCreateFilename);
    }
    (unsafe { xmlOutputBufferCreateFilenameValueThrDef = func });
    (unsafe { xmlMutexUnlock(xmlThrDefMutex) });
    return old;
}
#[no_mangle]
pub extern "C" fn __htmlDefaultSAXHandler() -> *mut crate::src::HTMLparser::_xmlSAXHandlerV1 {
    if (unsafe { xmlIsMainThread() }) != 0 {
        return unsafe { &mut htmlDefaultSAXHandler };
    } else {
        return unsafe { &mut (*(xmlGetGlobalState)()).htmlDefaultSAXHandler };
    };
}
#[no_mangle]
pub extern "C" fn __xmlLastError() -> *mut crate::src::HTMLparser::_xmlError {
    if (unsafe { xmlIsMainThread() }) != 0 {
        return unsafe { &mut xmlLastError };
    } else {
        return unsafe { &mut (*(xmlGetGlobalState)()).xmlLastError };
    };
}
#[no_mangle]
pub extern "C" fn __oldXMLWDcompatibility<'a1>() -> Option<&'a1 mut i32> {
    if (unsafe { xmlIsMainThread() }) != 0 {
        return Some(unsafe { &mut oldXMLWDcompatibility });
    } else {
        return Some(unsafe { &mut (*(xmlGetGlobalState)()).oldXMLWDcompatibility });
    };
}
#[no_mangle]
pub extern "C" fn __xmlBufferAllocScheme<'a1>() -> Option<&'a1 mut u32> {
    if (unsafe { xmlIsMainThread() }) != 0 {
        return Some(unsafe { &mut xmlBufferAllocScheme });
    } else {
        return Some(unsafe { &mut (*(xmlGetGlobalState)()).xmlBufferAllocScheme });
    };
}
#[no_mangle]
pub extern "C" fn xmlThrDefBufferAllocScheme(mut v: u32) -> u32 {
    let mut ret: u32 = XML_BUFFER_ALLOC_DOUBLEIT;
    (unsafe { xmlMutexLock(xmlThrDefMutex) });
    ret = unsafe { xmlBufferAllocSchemeThrDef };
    (unsafe { xmlBufferAllocSchemeThrDef = v });
    (unsafe { xmlMutexUnlock(xmlThrDefMutex) });
    return ret;
}
#[no_mangle]
pub extern "C" fn __xmlDefaultBufferSize<'a1>() -> Option<&'a1 mut i32> {
    if (unsafe { xmlIsMainThread() }) != 0 {
        return Some(unsafe { &mut xmlDefaultBufferSize });
    } else {
        return Some(unsafe { &mut (*(xmlGetGlobalState)()).xmlDefaultBufferSize });
    };
}
#[no_mangle]
pub extern "C" fn xmlThrDefDefaultBufferSize(mut v: i32) -> i32 {
    let mut ret: i32 = 0;
    (unsafe { xmlMutexLock(xmlThrDefMutex) });
    ret = unsafe { xmlDefaultBufferSizeThrDef };
    (unsafe { xmlDefaultBufferSizeThrDef = v });
    (unsafe { xmlMutexUnlock(xmlThrDefMutex) });
    return ret;
}
#[no_mangle]
pub extern "C" fn __xmlDefaultSAXHandler() -> *mut crate::src::HTMLparser::_xmlSAXHandlerV1 {
    if (unsafe { xmlIsMainThread() }) != 0 {
        return unsafe { &mut xmlDefaultSAXHandler };
    } else {
        return unsafe { &mut (*(xmlGetGlobalState)()).xmlDefaultSAXHandler };
    };
}
#[no_mangle]
pub extern "C" fn __xmlDefaultSAXLocator() -> *mut crate::src::HTMLparser::_xmlSAXLocator {
    if (unsafe { xmlIsMainThread() }) != 0 {
        return unsafe { &mut xmlDefaultSAXLocator };
    } else {
        return unsafe { &mut (*(xmlGetGlobalState)()).xmlDefaultSAXLocator };
    };
}
#[no_mangle]
pub extern "C" fn __xmlDoValidityCheckingDefaultValue<'a1>() -> Option<&'a1 mut i32> {
    if (unsafe { xmlIsMainThread() }) != 0 {
        return Some(unsafe { &mut xmlDoValidityCheckingDefaultValue });
    } else {
        return Some(unsafe { &mut (*(xmlGetGlobalState)()).xmlDoValidityCheckingDefaultValue });
    };
}
#[no_mangle]
pub extern "C" fn xmlThrDefDoValidityCheckingDefaultValue(mut v: i32) -> i32 {
    let mut ret: i32 = 0;
    (unsafe { xmlMutexLock(xmlThrDefMutex) });
    ret = unsafe { xmlDoValidityCheckingDefaultValueThrDef };
    (unsafe { xmlDoValidityCheckingDefaultValueThrDef = v });
    (unsafe { xmlMutexUnlock(xmlThrDefMutex) });
    return ret;
}
#[no_mangle]
pub extern "C" fn __xmlGenericError<'a1>()
-> Option<&'a1 mut Option<unsafe extern "C" fn(_: *mut core::ffi::c_void, _: *const i8, ...) -> ()>>
{
    if (unsafe { xmlIsMainThread() }) != 0 {
        return Some(unsafe { &mut xmlGenericError });
    } else {
        return Some(unsafe { &mut (*(xmlGetGlobalState)()).xmlGenericError });
    };
}
#[no_mangle]
pub extern "C" fn __xmlStructuredError<'a1>() -> Option<
    &'a1 mut Option<
        unsafe extern "C" fn(
            _: *mut core::ffi::c_void,
            _: *mut crate::src::HTMLparser::_xmlError,
        ) -> (),
    >,
> {
    if (unsafe { xmlIsMainThread() }) != 0 {
        return Some(unsafe { &mut xmlStructuredError });
    } else {
        return Some(unsafe { &mut (*(xmlGetGlobalState)()).xmlStructuredError });
    };
}
#[no_mangle]
pub extern "C" fn __xmlGenericErrorContext<'a1>() -> Option<&'a1 mut *mut core::ffi::c_void> {
    if (unsafe { xmlIsMainThread() }) != 0 {
        return Some(unsafe { &mut xmlGenericErrorContext });
    } else {
        return Some(unsafe { &mut (*(xmlGetGlobalState)()).xmlGenericErrorContext });
    };
}
#[no_mangle]
pub extern "C" fn __xmlStructuredErrorContext<'a1>() -> Option<&'a1 mut *mut core::ffi::c_void> {
    if (unsafe { xmlIsMainThread() }) != 0 {
        return Some(unsafe { &mut xmlStructuredErrorContext });
    } else {
        return Some(unsafe { &mut (*(xmlGetGlobalState)()).xmlStructuredErrorContext });
    };
}
#[no_mangle]
pub extern "C" fn __xmlGetWarningsDefaultValue<'a1>() -> Option<&'a1 mut i32> {
    if (unsafe { xmlIsMainThread() }) != 0 {
        return Some(unsafe { &mut xmlGetWarningsDefaultValue });
    } else {
        return Some(unsafe { &mut (*(xmlGetGlobalState)()).xmlGetWarningsDefaultValue });
    };
}
#[no_mangle]
pub extern "C" fn xmlThrDefGetWarningsDefaultValue(mut v: i32) -> i32 {
    let mut ret: i32 = 0;
    (unsafe { xmlMutexLock(xmlThrDefMutex) });
    ret = unsafe { xmlGetWarningsDefaultValueThrDef };
    (unsafe { xmlGetWarningsDefaultValueThrDef = v });
    (unsafe { xmlMutexUnlock(xmlThrDefMutex) });
    return ret;
}
#[no_mangle]
pub extern "C" fn __xmlIndentTreeOutput<'a1>() -> Option<&'a1 mut i32> {
    if (unsafe { xmlIsMainThread() }) != 0 {
        return Some(unsafe { &mut xmlIndentTreeOutput });
    } else {
        return Some(unsafe { &mut (*(xmlGetGlobalState)()).xmlIndentTreeOutput });
    };
}
#[no_mangle]
pub extern "C" fn xmlThrDefIndentTreeOutput(mut v: i32) -> i32 {
    let mut ret: i32 = 0;
    (unsafe { xmlMutexLock(xmlThrDefMutex) });
    ret = unsafe { xmlIndentTreeOutputThrDef };
    (unsafe { xmlIndentTreeOutputThrDef = v });
    (unsafe { xmlMutexUnlock(xmlThrDefMutex) });
    return ret;
}
#[no_mangle]
pub extern "C" fn __xmlTreeIndentString<'a1>() -> Option<&'a1 mut *const i8> {
    if (unsafe { xmlIsMainThread() }) != 0 {
        return Some(unsafe { &mut xmlTreeIndentString });
    } else {
        return Some(unsafe { &mut (*(xmlGetGlobalState)()).xmlTreeIndentString });
    };
}
#[no_mangle]
pub extern "C" fn xmlThrDefTreeIndentString(mut v: *const i8) -> *const i8 {
    let mut ret: *const i8 = 0 as *const i8;
    (unsafe { xmlMutexLock(xmlThrDefMutex) });
    ret = unsafe { xmlTreeIndentStringThrDef };
    (unsafe { xmlTreeIndentStringThrDef = v });
    (unsafe { xmlMutexUnlock(xmlThrDefMutex) });
    return ret;
}
#[no_mangle]
pub extern "C" fn __xmlKeepBlanksDefaultValue<'a1>() -> Option<&'a1 mut i32> {
    if (unsafe { xmlIsMainThread() }) != 0 {
        return Some(unsafe { &mut xmlKeepBlanksDefaultValue });
    } else {
        return Some(unsafe { &mut (*(xmlGetGlobalState)()).xmlKeepBlanksDefaultValue });
    };
}
#[no_mangle]
pub extern "C" fn xmlThrDefKeepBlanksDefaultValue(mut v: i32) -> i32 {
    let mut ret: i32 = 0;
    (unsafe { xmlMutexLock(xmlThrDefMutex) });
    ret = unsafe { xmlKeepBlanksDefaultValueThrDef };
    (unsafe { xmlKeepBlanksDefaultValueThrDef = v });
    (unsafe { xmlMutexUnlock(xmlThrDefMutex) });
    return ret;
}
#[no_mangle]
pub extern "C" fn __xmlLineNumbersDefaultValue<'a1>() -> Option<&'a1 mut i32> {
    if (unsafe { xmlIsMainThread() }) != 0 {
        return Some(unsafe { &mut xmlLineNumbersDefaultValue });
    } else {
        return Some(unsafe { &mut (*(xmlGetGlobalState)()).xmlLineNumbersDefaultValue });
    };
}
#[no_mangle]
pub extern "C" fn xmlThrDefLineNumbersDefaultValue(mut v: i32) -> i32 {
    let mut ret: i32 = 0;
    (unsafe { xmlMutexLock(xmlThrDefMutex) });
    ret = unsafe { xmlLineNumbersDefaultValueThrDef };
    (unsafe { xmlLineNumbersDefaultValueThrDef = v });
    (unsafe { xmlMutexUnlock(xmlThrDefMutex) });
    return ret;
}
#[no_mangle]
pub extern "C" fn __xmlLoadExtDtdDefaultValue<'a1>() -> Option<&'a1 mut i32> {
    if (unsafe { xmlIsMainThread() }) != 0 {
        return Some(unsafe { &mut xmlLoadExtDtdDefaultValue });
    } else {
        return Some(unsafe { &mut (*(xmlGetGlobalState)()).xmlLoadExtDtdDefaultValue });
    };
}
#[no_mangle]
pub extern "C" fn xmlThrDefLoadExtDtdDefaultValue(mut v: i32) -> i32 {
    let mut ret: i32 = 0;
    (unsafe { xmlMutexLock(xmlThrDefMutex) });
    ret = unsafe { xmlLoadExtDtdDefaultValueThrDef };
    (unsafe { xmlLoadExtDtdDefaultValueThrDef = v });
    (unsafe { xmlMutexUnlock(xmlThrDefMutex) });
    return ret;
}
#[no_mangle]
pub extern "C" fn __xmlParserDebugEntities<'a1>() -> Option<&'a1 mut i32> {
    if (unsafe { xmlIsMainThread() }) != 0 {
        return Some(unsafe { &mut xmlParserDebugEntities });
    } else {
        return Some(unsafe { &mut (*(xmlGetGlobalState)()).xmlParserDebugEntities });
    };
}
#[no_mangle]
pub extern "C" fn xmlThrDefParserDebugEntities(mut v: i32) -> i32 {
    let mut ret: i32 = 0;
    (unsafe { xmlMutexLock(xmlThrDefMutex) });
    ret = unsafe { xmlParserDebugEntitiesThrDef };
    (unsafe { xmlParserDebugEntitiesThrDef = v });
    (unsafe { xmlMutexUnlock(xmlThrDefMutex) });
    return ret;
}
#[no_mangle]
pub extern "C" fn __xmlParserVersion<'a1>() -> Option<&'a1 mut *const i8> {
    if (unsafe { xmlIsMainThread() }) != 0 {
        return Some(unsafe { &mut xmlParserVersion });
    } else {
        return Some(unsafe { &mut (*(xmlGetGlobalState)()).xmlParserVersion });
    };
}
#[no_mangle]
pub extern "C" fn __xmlPedanticParserDefaultValue<'a1>() -> Option<&'a1 mut i32> {
    if (unsafe { xmlIsMainThread() }) != 0 {
        return Some(unsafe { &mut xmlPedanticParserDefaultValue });
    } else {
        return Some(unsafe { &mut (*(xmlGetGlobalState)()).xmlPedanticParserDefaultValue });
    };
}
#[no_mangle]
pub extern "C" fn xmlThrDefPedanticParserDefaultValue(mut v: i32) -> i32 {
    let mut ret: i32 = 0;
    (unsafe { xmlMutexLock(xmlThrDefMutex) });
    ret = unsafe { xmlPedanticParserDefaultValueThrDef };
    (unsafe { xmlPedanticParserDefaultValueThrDef = v });
    (unsafe { xmlMutexUnlock(xmlThrDefMutex) });
    return ret;
}
#[no_mangle]
pub extern "C" fn __xmlSaveNoEmptyTags<'a1>() -> Option<&'a1 mut i32> {
    if (unsafe { xmlIsMainThread() }) != 0 {
        return Some(unsafe { &mut xmlSaveNoEmptyTags });
    } else {
        return Some(unsafe { &mut (*(xmlGetGlobalState)()).xmlSaveNoEmptyTags });
    };
}
#[no_mangle]
pub extern "C" fn xmlThrDefSaveNoEmptyTags(mut v: i32) -> i32 {
    let mut ret: i32 = 0;
    (unsafe { xmlMutexLock(xmlThrDefMutex) });
    ret = unsafe { xmlSaveNoEmptyTagsThrDef };
    (unsafe { xmlSaveNoEmptyTagsThrDef = v });
    (unsafe { xmlMutexUnlock(xmlThrDefMutex) });
    return ret;
}
#[no_mangle]
pub extern "C" fn __xmlSubstituteEntitiesDefaultValue<'a1>() -> Option<&'a1 mut i32> {
    if (unsafe { xmlIsMainThread() }) != 0 {
        return Some(unsafe { &mut xmlSubstituteEntitiesDefaultValue });
    } else {
        return Some(unsafe { &mut (*(xmlGetGlobalState)()).xmlSubstituteEntitiesDefaultValue });
    };
}
#[no_mangle]
pub extern "C" fn xmlThrDefSubstituteEntitiesDefaultValue(mut v: i32) -> i32 {
    let mut ret: i32 = 0;
    (unsafe { xmlMutexLock(xmlThrDefMutex) });
    ret = unsafe { xmlSubstituteEntitiesDefaultValueThrDef };
    (unsafe { xmlSubstituteEntitiesDefaultValueThrDef = v });
    (unsafe { xmlMutexUnlock(xmlThrDefMutex) });
    return ret;
}
#[no_mangle]
pub extern "C" fn __xmlRegisterNodeDefaultValue<'a1>()
-> Option<&'a1 mut Option<unsafe extern "C" fn(_: *mut crate::src::HTMLparser::_xmlNode) -> ()>> {
    if (unsafe { xmlIsMainThread() }) != 0 {
        return Some(unsafe { &mut xmlRegisterNodeDefaultValue });
    } else {
        return Some(unsafe { &mut (*(xmlGetGlobalState)()).xmlRegisterNodeDefaultValue });
    };
}
#[no_mangle]
pub extern "C" fn __xmlDeregisterNodeDefaultValue<'a1>()
-> Option<&'a1 mut Option<unsafe extern "C" fn(_: *mut crate::src::HTMLparser::_xmlNode) -> ()>> {
    if (unsafe { xmlIsMainThread() }) != 0 {
        return Some(unsafe { &mut xmlDeregisterNodeDefaultValue });
    } else {
        return Some(unsafe { &mut (*(xmlGetGlobalState)()).xmlDeregisterNodeDefaultValue });
    };
}
#[no_mangle]
pub extern "C" fn __xmlParserInputBufferCreateFilenameValue<'a1>() -> Option<
    &'a1 mut Option<
        unsafe extern "C" fn(
            _: *const i8,
            _: i32,
        ) -> *mut crate::src::HTMLparser::_xmlParserInputBuffer,
    >,
> {
    if (unsafe { xmlIsMainThread() }) != 0 {
        return Some(unsafe { &mut xmlParserInputBufferCreateFilenameValue });
    } else {
        return Some(unsafe { &mut (*(xmlGetGlobalState)()).xmlParserInputBufferCreateFilenameValue });
    };
}
#[no_mangle]
pub extern "C" fn __xmlOutputBufferCreateFilenameValue<'a1>() -> Option<
    &'a1 mut Option<
        unsafe extern "C" fn(
            _: *const i8,
            _: *mut crate::src::HTMLparser::_xmlCharEncodingHandler,
            _: i32,
        ) -> *mut crate::src::HTMLtree::_xmlOutputBuffer,
    >,
> {
    if (unsafe { xmlIsMainThread() }) != 0 {
        return Some(unsafe { &mut xmlOutputBufferCreateFilenameValue });
    } else {
        return Some(unsafe { &mut (*(xmlGetGlobalState)()).xmlOutputBufferCreateFilenameValue });
    };
}
use crate::laertes_rt::*;
