use ::libc;
extern "C" {
<<<<<<< HEAD
    
    
    
    
    
    
    
=======
    pub type _xmlBuf;
    pub type _xmlDict;
    pub type _xmlMutex;
    fn __xmlGlobalInitMutexDestroy();
    fn xmlCharStrdup(cur: *const i8) -> *mut xmlChar;
    fn xmlStrdup(cur: *const xmlChar) -> *mut xmlChar;
    static mut __xmlRegisterCallbacks: i32;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
    fn malloc(_: u64) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: i32,
        _: u64,
    ) -> *mut libc::c_void;
<<<<<<< HEAD
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
}
pub use crate::src::SAX2::xmlSAX2AttributeDecl;
pub use crate::src::SAX2::xmlSAX2CDataBlock;
pub use crate::src::SAX2::xmlSAX2Characters;
pub use crate::src::SAX2::xmlSAX2Comment;
pub use crate::src::SAX2::xmlSAX2ElementDecl;
pub use crate::src::SAX2::xmlSAX2EndDocument;
pub use crate::src::SAX2::xmlSAX2EndElement;
pub use crate::src::SAX2::xmlSAX2EntityDecl;
pub use crate::src::SAX2::xmlSAX2ExternalSubset;
pub use crate::src::SAX2::xmlSAX2GetColumnNumber;
pub use crate::src::SAX2::xmlSAX2GetEntity;
pub use crate::src::SAX2::xmlSAX2GetLineNumber;
pub use crate::src::SAX2::xmlSAX2GetParameterEntity;
pub use crate::src::SAX2::xmlSAX2GetPublicId;
pub use crate::src::SAX2::xmlSAX2GetSystemId;
pub use crate::src::SAX2::xmlSAX2HasExternalSubset;
pub use crate::src::SAX2::xmlSAX2HasInternalSubset;
pub use crate::src::SAX2::xmlSAX2IgnorableWhitespace;
pub use crate::src::SAX2::xmlSAX2InternalSubset;
pub use crate::src::SAX2::xmlSAX2IsStandalone;
pub use crate::src::SAX2::xmlSAX2NotationDecl;
pub use crate::src::SAX2::xmlSAX2ProcessingInstruction;
pub use crate::src::SAX2::xmlSAX2Reference;
pub use crate::src::SAX2::xmlSAX2ResolveEntity;
pub use crate::src::SAX2::xmlSAX2SetDocumentLocator;
pub use crate::src::SAX2::xmlSAX2StartDocument;
pub use crate::src::SAX2::xmlSAX2StartElement;
pub use crate::src::SAX2::xmlSAX2UnparsedEntityDecl;
pub use crate::src::SAX::inithtmlDefaultSAXHandler;
pub use crate::src::SAX::initxmlDefaultSAXHandler;
pub use crate::src::error::xmlGenericErrorDefaultFunc;
pub use crate::src::error::xmlParserError;
pub use crate::src::error::xmlParserWarning;
pub use crate::src::error::xmlResetError;
pub use crate::src::threads::__xmlGlobalInitMutexDestroy;
pub use crate::src::threads::xmlFreeMutex;
pub use crate::src::threads::xmlGetGlobalState;
pub use crate::src::threads::xmlIsMainThread;
pub use crate::src::threads::xmlMutexLock;
pub use crate::src::threads::xmlMutexUnlock;
pub use crate::src::threads::xmlNewMutex;
pub use crate::src::xmlIO::__xmlOutputBufferCreateFilename;
pub use crate::src::xmlIO::__xmlParserInputBufferCreateFilename;
pub use crate::src::xmlstring::xmlCharStrdup;
pub use crate::src::xmlstring::xmlStrdup;
pub use crate::src::buf::_xmlBuf;
pub use crate::src::dict::_xmlDict;
pub use crate::src::tree::__xmlRegisterCallbacks;
pub use crate::src::threads::_xmlMutex;
pub type xmlChar = crate::src::HTMLparser::xmlChar;
pub type size_t = crate::src::HTMLparser::size_t;
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
    fn xmlParserError(ctx: *mut libc::c_void, msg: *const i8, _: ...);
    fn xmlParserWarning(ctx: *mut libc::c_void, msg: *const i8, _: ...);
    fn xmlResetError(err: xmlErrorPtr);
    fn __xmlParserInputBufferCreateFilename(
        URI: *const i8,
        enc: xmlCharEncoding,
    ) -> xmlParserInputBufferPtr;
    fn __xmlOutputBufferCreateFilename(
        URI: *const i8,
        encoder: xmlCharEncodingHandlerPtr,
        compression: i32,
    ) -> xmlOutputBufferPtr;
    fn xmlSAX2GetPublicId(ctx: *mut libc::c_void) -> *const xmlChar;
    fn xmlSAX2GetSystemId(ctx: *mut libc::c_void) -> *const xmlChar;
    fn xmlSAX2SetDocumentLocator(ctx: *mut libc::c_void, loc: xmlSAXLocatorPtr);
    fn xmlSAX2GetLineNumber(ctx: *mut libc::c_void) -> i32;
    fn xmlSAX2GetColumnNumber(ctx: *mut libc::c_void) -> i32;
    fn xmlSAX2IsStandalone(ctx: *mut libc::c_void) -> i32;
    fn xmlSAX2HasInternalSubset(ctx: *mut libc::c_void) -> i32;
    fn xmlSAX2HasExternalSubset(ctx: *mut libc::c_void) -> i32;
    fn xmlSAX2InternalSubset(
        ctx: *mut libc::c_void,
        name: *const xmlChar,
        ExternalID: *const xmlChar,
        SystemID: *const xmlChar,
    );
    fn xmlSAX2ExternalSubset(
        ctx: *mut libc::c_void,
        name: *const xmlChar,
        ExternalID: *const xmlChar,
        SystemID: *const xmlChar,
    );
    fn xmlSAX2GetEntity(ctx: *mut libc::c_void, name: *const xmlChar) -> xmlEntityPtr;
    fn xmlSAX2GetParameterEntity(
        ctx: *mut libc::c_void,
        name: *const xmlChar,
    ) -> xmlEntityPtr;
    fn xmlSAX2ResolveEntity(
        ctx: *mut libc::c_void,
        publicId: *const xmlChar,
        systemId: *const xmlChar,
    ) -> xmlParserInputPtr;
    fn xmlSAX2EntityDecl(
        ctx: *mut libc::c_void,
        name: *const xmlChar,
        type_0: i32,
        publicId: *const xmlChar,
        systemId: *const xmlChar,
        content: *mut xmlChar,
    );
    fn xmlSAX2AttributeDecl(
        ctx: *mut libc::c_void,
        elem: *const xmlChar,
        fullname: *const xmlChar,
        type_0: i32,
        def: i32,
        defaultValue: *const xmlChar,
        tree: xmlEnumerationPtr,
    );
    fn xmlSAX2ElementDecl(
        ctx: *mut libc::c_void,
        name: *const xmlChar,
        type_0: i32,
        content: xmlElementContentPtr,
    );
    fn xmlSAX2NotationDecl(
        ctx: *mut libc::c_void,
        name: *const xmlChar,
        publicId: *const xmlChar,
        systemId: *const xmlChar,
    );
    fn xmlSAX2UnparsedEntityDecl(
        ctx: *mut libc::c_void,
        name: *const xmlChar,
        publicId: *const xmlChar,
        systemId: *const xmlChar,
        notationName: *const xmlChar,
    );
    fn xmlSAX2StartDocument(ctx: *mut libc::c_void);
    fn xmlSAX2EndDocument(ctx: *mut libc::c_void);
    fn xmlSAX2StartElement(
        ctx: *mut libc::c_void,
        fullname: *const xmlChar,
        atts: *mut *const xmlChar,
    );
    fn xmlSAX2EndElement(ctx: *mut libc::c_void, name: *const xmlChar);
    fn xmlSAX2Reference(ctx: *mut libc::c_void, name: *const xmlChar);
    fn xmlSAX2Characters(ctx: *mut libc::c_void, ch: *const xmlChar, len: i32);
    fn xmlSAX2IgnorableWhitespace(
        ctx: *mut libc::c_void,
        ch: *const xmlChar,
        len: i32,
    );
    fn xmlSAX2ProcessingInstruction(
        ctx: *mut libc::c_void,
        target: *const xmlChar,
        data: *const xmlChar,
    );
    fn xmlSAX2Comment(ctx: *mut libc::c_void, value: *const xmlChar);
    fn xmlSAX2CDataBlock(
        ctx: *mut libc::c_void,
        value: *const xmlChar,
        len: i32,
    );
    fn xmlNewMutex() -> xmlMutexPtr;
    fn xmlFreeMutex(tok: xmlMutexPtr);
    fn xmlMutexLock(tok: xmlMutexPtr);
    fn xmlMutexUnlock(tok: xmlMutexPtr);
    fn xmlIsMainThread() -> i32;
    fn xmlGetGlobalState() -> xmlGlobalStatePtr;
    fn initxmlDefaultSAXHandler(hdlr: *mut xmlSAXHandlerV1, warning: i32);
    fn inithtmlDefaultSAXHandler(hdlr: *mut xmlSAXHandlerV1);
    fn xmlGenericErrorDefaultFunc(
        ctx: *mut libc::c_void,
        msg: *const i8,
        _: ...
    );
}
pub type xmlChar = u8;
pub type size_t = u64;
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
pub type xmlMutexPtr = crate::src::dict::xmlMutexPtr;
pub type xmlMutex = crate::src::dict::xmlMutex;
=======
pub type xmlFreeFunc = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type xmlMallocFunc = Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>;
pub type xmlReallocFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
>;
pub type xmlStrdupFunc = Option::<
    unsafe extern "C" fn(*const i8) -> *mut i8,
>;
pub type xmlMutexPtr = *mut xmlMutex;
pub type xmlMutex = _xmlMutex;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
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
<<<<<<< HEAD
pub type xmlRegisterNodeFunc = crate::src::HTMLparser::xmlRegisterNodeFunc;
=======
pub type xmlRegisterNodeFunc = Option::<unsafe extern "C" fn(xmlNodePtr) -> ()>;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
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
static mut xmlThrDefMutex: xmlMutexPtr = 0 as *const xmlMutex as xmlMutexPtr;
#[no_mangle]
pub unsafe extern "C" fn xmlInitGlobals() {
    if xmlThrDefMutex.is_null() {
        xmlThrDefMutex = xmlNewMutex();
    }
}
#[no_mangle]
pub static mut xmlFree: xmlFreeFunc = unsafe {
    Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ())
};
#[no_mangle]
pub static mut xmlMalloc: xmlMallocFunc = unsafe {
    Some(malloc as unsafe extern "C" fn(u64) -> *mut libc::c_void)
};
#[no_mangle]
pub static mut xmlMallocAtomic: xmlMallocFunc = unsafe {
    Some(malloc as unsafe extern "C" fn(u64) -> *mut libc::c_void)
};
#[no_mangle]
pub static mut xmlRealloc: xmlReallocFunc = unsafe {
    Some(
        realloc
            as unsafe extern "C" fn(
                *mut libc::c_void,
                u64,
            ) -> *mut libc::c_void,
    )
};
unsafe extern "C" fn xmlPosixStrdup(mut cur: *const i8) -> *mut i8 {
    return xmlCharStrdup(cur) as *mut i8;
}
#[no_mangle]
pub static mut xmlMemStrdup: xmlStrdupFunc = unsafe {
    Some(
        xmlPosixStrdup as unsafe extern "C" fn(*const i8) -> *mut i8,
    )
};
#[no_mangle]
pub static mut xmlParserVersion: *const i8 = b"21000-GITv2.10.0\0" as *const u8
    as *const i8;
#[no_mangle]
pub static mut xmlBufferAllocScheme: xmlBufferAllocationScheme = XML_BUFFER_ALLOC_EXACT;
static mut xmlBufferAllocSchemeThrDef: xmlBufferAllocationScheme = XML_BUFFER_ALLOC_EXACT;
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
pub static mut xmlRegisterNodeDefaultValue: xmlRegisterNodeFunc = None;
static mut xmlRegisterNodeDefaultValueThrDef: xmlRegisterNodeFunc = None;
#[no_mangle]
pub static mut xmlDeregisterNodeDefaultValue: xmlDeregisterNodeFunc = None;
static mut xmlDeregisterNodeDefaultValueThrDef: xmlDeregisterNodeFunc = None;
#[no_mangle]
pub static mut xmlParserInputBufferCreateFilenameValue: xmlParserInputBufferCreateFilenameFunc = None;
static mut xmlParserInputBufferCreateFilenameValueThrDef: xmlParserInputBufferCreateFilenameFunc = None;
#[no_mangle]
pub static mut xmlOutputBufferCreateFilenameValue: xmlOutputBufferCreateFilenameFunc = None;
static mut xmlOutputBufferCreateFilenameValueThrDef: xmlOutputBufferCreateFilenameFunc = None;
#[no_mangle]
pub static mut xmlGenericError: xmlGenericErrorFunc = unsafe {
    Some(
        xmlGenericErrorDefaultFunc
            as unsafe extern "C" fn(*mut libc::c_void, *const i8, ...) -> (),
    )
};
static mut xmlGenericErrorThrDef: xmlGenericErrorFunc = unsafe {
    Some(
        xmlGenericErrorDefaultFunc
            as unsafe extern "C" fn(*mut libc::c_void, *const i8, ...) -> (),
    )
};
#[no_mangle]
pub static mut xmlStructuredError: xmlStructuredErrorFunc = None;
static mut xmlStructuredErrorThrDef: xmlStructuredErrorFunc = None;
#[no_mangle]
pub static mut xmlGenericErrorContext: *mut libc::c_void = 0 as *const libc::c_void
    as *mut libc::c_void;
static mut xmlGenericErrorContextThrDef: *mut libc::c_void = 0 as *const libc::c_void
    as *mut libc::c_void;
#[no_mangle]
pub static mut xmlStructuredErrorContext: *mut libc::c_void = 0 as *const libc::c_void
    as *mut libc::c_void;
static mut xmlStructuredErrorContextThrDef: *mut libc::c_void = 0 as *const libc::c_void
    as *mut libc::c_void;
#[no_mangle]
pub static mut xmlLastError: xmlError = xmlError {
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
pub static mut xmlTreeIndentString: *const i8 = b"  \0" as *const u8
    as *const i8;
static mut xmlTreeIndentStringThrDef: *const i8 = b"  \0" as *const u8
    as *const i8;
#[no_mangle]
pub static mut xmlSaveNoEmptyTags: i32 = 0 as i32;
static mut xmlSaveNoEmptyTagsThrDef: i32 = 0 as i32;
#[no_mangle]
pub static mut xmlDefaultSAXHandler: xmlSAXHandlerV1 = unsafe {
    {
        let mut init = _xmlSAXHandlerV1 {
            internalSubset: Some(
                xmlSAX2InternalSubset
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                        *const xmlChar,
                        *const xmlChar,
                    ) -> (),
            ),
            isStandalone: Some(
                xmlSAX2IsStandalone
                    as unsafe extern "C" fn(*mut libc::c_void) -> i32,
            ),
            hasInternalSubset: Some(
                xmlSAX2HasInternalSubset
                    as unsafe extern "C" fn(*mut libc::c_void) -> i32,
            ),
            hasExternalSubset: Some(
                xmlSAX2HasExternalSubset
                    as unsafe extern "C" fn(*mut libc::c_void) -> i32,
            ),
            resolveEntity: Some(
                xmlSAX2ResolveEntity
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                        *const xmlChar,
                    ) -> xmlParserInputPtr,
            ),
            getEntity: Some(
                xmlSAX2GetEntity
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                    ) -> xmlEntityPtr,
            ),
            entityDecl: Some(
                xmlSAX2EntityDecl
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                        i32,
                        *const xmlChar,
                        *const xmlChar,
                        *mut xmlChar,
                    ) -> (),
            ),
            notationDecl: Some(
                xmlSAX2NotationDecl
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                        *const xmlChar,
                        *const xmlChar,
                    ) -> (),
            ),
            attributeDecl: Some(
                xmlSAX2AttributeDecl
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                        *const xmlChar,
                        i32,
                        i32,
                        *const xmlChar,
                        xmlEnumerationPtr,
                    ) -> (),
            ),
            elementDecl: Some(
                xmlSAX2ElementDecl
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                        i32,
                        xmlElementContentPtr,
                    ) -> (),
            ),
            unparsedEntityDecl: Some(
                xmlSAX2UnparsedEntityDecl
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                        *const xmlChar,
                        *const xmlChar,
                        *const xmlChar,
                    ) -> (),
            ),
            setDocumentLocator: Some(
                xmlSAX2SetDocumentLocator
                    as unsafe extern "C" fn(*mut libc::c_void, xmlSAXLocatorPtr) -> (),
            ),
            startDocument: Some(
                xmlSAX2StartDocument as unsafe extern "C" fn(*mut libc::c_void) -> (),
            ),
            endDocument: Some(
                xmlSAX2EndDocument as unsafe extern "C" fn(*mut libc::c_void) -> (),
            ),
            startElement: Some(
                xmlSAX2StartElement
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                        *mut *const xmlChar,
                    ) -> (),
            ),
            endElement: Some(
                xmlSAX2EndElement
                    as unsafe extern "C" fn(*mut libc::c_void, *const xmlChar) -> (),
            ),
            reference: Some(
                xmlSAX2Reference
                    as unsafe extern "C" fn(*mut libc::c_void, *const xmlChar) -> (),
            ),
            characters: Some(
                xmlSAX2Characters
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                        i32,
                    ) -> (),
            ),
            ignorableWhitespace: Some(
                xmlSAX2Characters
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                        i32,
                    ) -> (),
            ),
            processingInstruction: Some(
                xmlSAX2ProcessingInstruction
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                        *const xmlChar,
                    ) -> (),
            ),
            comment: Some(
                xmlSAX2Comment
                    as unsafe extern "C" fn(*mut libc::c_void, *const xmlChar) -> (),
            ),
            warning: Some(
                xmlParserWarning
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const i8,
                        ...
                    ) -> (),
            ),
            error: Some(
                xmlParserError
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const i8,
                        ...
                    ) -> (),
            ),
            fatalError: Some(
                xmlParserError
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const i8,
                        ...
                    ) -> (),
            ),
            getParameterEntity: Some(
                xmlSAX2GetParameterEntity
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                    ) -> xmlEntityPtr,
            ),
            cdataBlock: Some(
                xmlSAX2CDataBlock
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                        i32,
                    ) -> (),
            ),
            externalSubset: Some(
                xmlSAX2ExternalSubset
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                        *const xmlChar,
                        *const xmlChar,
                    ) -> (),
            ),
            initialized: 0 as i32 as u32,
        };
        init
    }
};
#[no_mangle]
pub static mut xmlDefaultSAXLocator: xmlSAXLocator = unsafe {
    {
        let mut init = _xmlSAXLocator {
            getPublicId: Some(
                xmlSAX2GetPublicId
                    as unsafe extern "C" fn(*mut libc::c_void) -> *const xmlChar,
            ),
            getSystemId: Some(
                xmlSAX2GetSystemId
                    as unsafe extern "C" fn(*mut libc::c_void) -> *const xmlChar,
            ),
            getLineNumber: Some(
                xmlSAX2GetLineNumber
                    as unsafe extern "C" fn(*mut libc::c_void) -> i32,
            ),
            getColumnNumber: Some(
                xmlSAX2GetColumnNumber
                    as unsafe extern "C" fn(*mut libc::c_void) -> i32,
            ),
        };
        init
    }
};
#[no_mangle]
pub static mut htmlDefaultSAXHandler: xmlSAXHandlerV1 = unsafe {
    {
        let mut init = _xmlSAXHandlerV1 {
            internalSubset: Some(
                xmlSAX2InternalSubset
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                        *const xmlChar,
                        *const xmlChar,
                    ) -> (),
            ),
            isStandalone: None,
            hasInternalSubset: None,
            hasExternalSubset: None,
            resolveEntity: None,
            getEntity: Some(
                xmlSAX2GetEntity
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                    ) -> xmlEntityPtr,
            ),
            entityDecl: None,
            notationDecl: None,
            attributeDecl: None,
            elementDecl: None,
            unparsedEntityDecl: None,
            setDocumentLocator: Some(
                xmlSAX2SetDocumentLocator
                    as unsafe extern "C" fn(*mut libc::c_void, xmlSAXLocatorPtr) -> (),
            ),
            startDocument: Some(
                xmlSAX2StartDocument as unsafe extern "C" fn(*mut libc::c_void) -> (),
            ),
            endDocument: Some(
                xmlSAX2EndDocument as unsafe extern "C" fn(*mut libc::c_void) -> (),
            ),
            startElement: Some(
                xmlSAX2StartElement
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                        *mut *const xmlChar,
                    ) -> (),
            ),
            endElement: Some(
                xmlSAX2EndElement
                    as unsafe extern "C" fn(*mut libc::c_void, *const xmlChar) -> (),
            ),
            reference: None,
            characters: Some(
                xmlSAX2Characters
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                        i32,
                    ) -> (),
            ),
            ignorableWhitespace: Some(
                xmlSAX2IgnorableWhitespace
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                        i32,
                    ) -> (),
            ),
            processingInstruction: Some(
                xmlSAX2ProcessingInstruction
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                        *const xmlChar,
                    ) -> (),
            ),
            comment: Some(
                xmlSAX2Comment
                    as unsafe extern "C" fn(*mut libc::c_void, *const xmlChar) -> (),
            ),
            warning: Some(
                xmlParserWarning
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const i8,
                        ...
                    ) -> (),
            ),
            error: Some(
                xmlParserError
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const i8,
                        ...
                    ) -> (),
            ),
            fatalError: Some(
                xmlParserError
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const i8,
                        ...
                    ) -> (),
            ),
            getParameterEntity: Some(
                xmlSAX2GetParameterEntity
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                    ) -> xmlEntityPtr,
            ),
            cdataBlock: Some(
                xmlSAX2CDataBlock
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                        i32,
                    ) -> (),
            ),
            externalSubset: None,
            initialized: 0 as i32 as u32,
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn xmlInitializeGlobalState(mut gs: xmlGlobalStatePtr) {
    if xmlThrDefMutex.is_null() {
        xmlInitGlobals();
    }
    xmlMutexLock(xmlThrDefMutex);
    inithtmlDefaultSAXHandler(&mut (*gs).htmlDefaultSAXHandler);
    (*gs).oldXMLWDcompatibility = 0 as i32;
    (*gs).xmlBufferAllocScheme = xmlBufferAllocSchemeThrDef;
    (*gs).xmlDefaultBufferSize = xmlDefaultBufferSizeThrDef;
    initxmlDefaultSAXHandler(&mut (*gs).xmlDefaultSAXHandler, 1 as i32);
    let ref mut fresh0 = (*gs).xmlDefaultSAXLocator.getPublicId;
    *fresh0 = Some(
        xmlSAX2GetPublicId as unsafe extern "C" fn(*mut libc::c_void) -> *const xmlChar,
    );
    let ref mut fresh1 = (*gs).xmlDefaultSAXLocator.getSystemId;
    *fresh1 = Some(
        xmlSAX2GetSystemId as unsafe extern "C" fn(*mut libc::c_void) -> *const xmlChar,
    );
    let ref mut fresh2 = (*gs).xmlDefaultSAXLocator.getLineNumber;
    *fresh2 = Some(
        xmlSAX2GetLineNumber as unsafe extern "C" fn(*mut libc::c_void) -> i32,
    );
    let ref mut fresh3 = (*gs).xmlDefaultSAXLocator.getColumnNumber;
    *fresh3 = Some(
        xmlSAX2GetColumnNumber as unsafe extern "C" fn(*mut libc::c_void) -> i32,
    );
    (*gs).xmlDoValidityCheckingDefaultValue = xmlDoValidityCheckingDefaultValueThrDef;
    let ref mut fresh4 = (*gs).xmlFree;
    *fresh4 = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        xmlFreeFunc,
    >(Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()));
    let ref mut fresh5 = (*gs).xmlMalloc;
    *fresh5 = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn(u64) -> *mut libc::c_void>,
        xmlMallocFunc,
    >(Some(malloc as unsafe extern "C" fn(u64) -> *mut libc::c_void));
    let ref mut fresh6 = (*gs).xmlMallocAtomic;
    *fresh6 = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn(u64) -> *mut libc::c_void>,
        xmlMallocFunc,
    >(Some(malloc as unsafe extern "C" fn(u64) -> *mut libc::c_void));
    let ref mut fresh7 = (*gs).xmlRealloc;
    *fresh7 = ::std::mem::transmute::<
        Option::<
            unsafe extern "C" fn(*mut libc::c_void, u64) -> *mut libc::c_void,
        >,
        xmlReallocFunc,
    >(
        Some(
            realloc
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    u64,
                ) -> *mut libc::c_void,
        ),
    );
    let ref mut fresh8 = (*gs).xmlMemStrdup;
    *fresh8 = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn(*const xmlChar) -> *mut xmlChar>,
        xmlStrdupFunc,
    >(Some(xmlStrdup as unsafe extern "C" fn(*const xmlChar) -> *mut xmlChar));
    (*gs).xmlGetWarningsDefaultValue = xmlGetWarningsDefaultValueThrDef;
    (*gs).xmlIndentTreeOutput = xmlIndentTreeOutputThrDef;
    let ref mut fresh9 = (*gs).xmlTreeIndentString;
    *fresh9 = xmlTreeIndentStringThrDef;
    (*gs).xmlKeepBlanksDefaultValue = xmlKeepBlanksDefaultValueThrDef;
    (*gs).xmlLineNumbersDefaultValue = xmlLineNumbersDefaultValueThrDef;
    (*gs).xmlLoadExtDtdDefaultValue = xmlLoadExtDtdDefaultValueThrDef;
    (*gs).xmlParserDebugEntities = xmlParserDebugEntitiesThrDef;
    let ref mut fresh10 = (*gs).xmlParserVersion;
    *fresh10 = b"21000\0" as *const u8 as *const i8;
    (*gs).xmlPedanticParserDefaultValue = xmlPedanticParserDefaultValueThrDef;
    (*gs).xmlSaveNoEmptyTags = xmlSaveNoEmptyTagsThrDef;
    (*gs).xmlSubstituteEntitiesDefaultValue = xmlSubstituteEntitiesDefaultValueThrDef;
    let ref mut fresh11 = (*gs).xmlGenericError;
    *fresh11 = xmlGenericErrorThrDef;
    let ref mut fresh12 = (*gs).xmlStructuredError;
    *fresh12 = xmlStructuredErrorThrDef;
    let ref mut fresh13 = (*gs).xmlGenericErrorContext;
    *fresh13 = xmlGenericErrorContextThrDef;
    let ref mut fresh14 = (*gs).xmlStructuredErrorContext;
    *fresh14 = xmlStructuredErrorContextThrDef;
    let ref mut fresh15 = (*gs).xmlRegisterNodeDefaultValue;
    *fresh15 = xmlRegisterNodeDefaultValueThrDef;
    let ref mut fresh16 = (*gs).xmlDeregisterNodeDefaultValue;
    *fresh16 = xmlDeregisterNodeDefaultValueThrDef;
    let ref mut fresh17 = (*gs).xmlParserInputBufferCreateFilenameValue;
    *fresh17 = xmlParserInputBufferCreateFilenameValueThrDef;
    let ref mut fresh18 = (*gs).xmlOutputBufferCreateFilenameValue;
    *fresh18 = xmlOutputBufferCreateFilenameValueThrDef;
    memset(
        &mut (*gs).xmlLastError as *mut xmlError as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<xmlError>() as u64,
    );
    xmlMutexUnlock(xmlThrDefMutex);
}
#[no_mangle]
pub unsafe extern "C" fn xmlCleanupGlobals() {
    xmlResetError(&mut xmlLastError);
    if !xmlThrDefMutex.is_null() {
        xmlFreeMutex(xmlThrDefMutex);
        xmlThrDefMutex = 0 as xmlMutexPtr;
    }
    __xmlGlobalInitMutexDestroy();
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefSetGenericErrorFunc(
    mut ctx: *mut libc::c_void,
    mut handler: xmlGenericErrorFunc,
) {
    xmlMutexLock(xmlThrDefMutex);
    xmlGenericErrorContextThrDef = ctx;
    if handler.is_some() {
        xmlGenericErrorThrDef = handler;
    } else {
        xmlGenericErrorThrDef = Some(
            xmlGenericErrorDefaultFunc
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const i8,
                    ...
                ) -> (),
        );
    }
    xmlMutexUnlock(xmlThrDefMutex);
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefSetStructuredErrorFunc(
    mut ctx: *mut libc::c_void,
    mut handler: xmlStructuredErrorFunc,
) {
    xmlMutexLock(xmlThrDefMutex);
    xmlStructuredErrorContextThrDef = ctx;
    xmlStructuredErrorThrDef = handler;
    xmlMutexUnlock(xmlThrDefMutex);
}
#[no_mangle]
pub unsafe extern "C" fn xmlRegisterNodeDefault(
    mut func: xmlRegisterNodeFunc,
) -> xmlRegisterNodeFunc {
    let mut old: xmlRegisterNodeFunc = xmlRegisterNodeDefaultValue;
    __xmlRegisterCallbacks = 1 as i32;
    xmlRegisterNodeDefaultValue = func;
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefRegisterNodeDefault(
    mut func: xmlRegisterNodeFunc,
) -> xmlRegisterNodeFunc {
    let mut old: xmlRegisterNodeFunc = None;
    xmlMutexLock(xmlThrDefMutex);
    old = xmlRegisterNodeDefaultValueThrDef;
    __xmlRegisterCallbacks = 1 as i32;
    xmlRegisterNodeDefaultValueThrDef = func;
    xmlMutexUnlock(xmlThrDefMutex);
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn xmlDeregisterNodeDefault(
    mut func: xmlDeregisterNodeFunc,
) -> xmlDeregisterNodeFunc {
    let mut old: xmlDeregisterNodeFunc = xmlDeregisterNodeDefaultValue;
    __xmlRegisterCallbacks = 1 as i32;
    xmlDeregisterNodeDefaultValue = func;
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefDeregisterNodeDefault(
    mut func: xmlDeregisterNodeFunc,
) -> xmlDeregisterNodeFunc {
    let mut old: xmlDeregisterNodeFunc = None;
    xmlMutexLock(xmlThrDefMutex);
    old = xmlDeregisterNodeDefaultValueThrDef;
    __xmlRegisterCallbacks = 1 as i32;
    xmlDeregisterNodeDefaultValueThrDef = func;
    xmlMutexUnlock(xmlThrDefMutex);
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefParserInputBufferCreateFilenameDefault(
    mut func: xmlParserInputBufferCreateFilenameFunc,
) -> xmlParserInputBufferCreateFilenameFunc {
    let mut old: xmlParserInputBufferCreateFilenameFunc = None;
    xmlMutexLock(xmlThrDefMutex);
    old = xmlParserInputBufferCreateFilenameValueThrDef;
    if old.is_none() {
        old = Some(
            __xmlParserInputBufferCreateFilename
                as unsafe extern "C" fn(
                    *const i8,
                    xmlCharEncoding,
                ) -> xmlParserInputBufferPtr,
        );
    }
    xmlParserInputBufferCreateFilenameValueThrDef = func;
    xmlMutexUnlock(xmlThrDefMutex);
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefOutputBufferCreateFilenameDefault(
    mut func: xmlOutputBufferCreateFilenameFunc,
) -> xmlOutputBufferCreateFilenameFunc {
    let mut old: xmlOutputBufferCreateFilenameFunc = None;
    xmlMutexLock(xmlThrDefMutex);
    old = xmlOutputBufferCreateFilenameValueThrDef;
    if old.is_none() {
        old = Some(
            __xmlOutputBufferCreateFilename
                as unsafe extern "C" fn(
                    *const i8,
                    xmlCharEncodingHandlerPtr,
                    i32,
                ) -> xmlOutputBufferPtr,
        );
    }
    xmlOutputBufferCreateFilenameValueThrDef = func;
    xmlMutexUnlock(xmlThrDefMutex);
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn __htmlDefaultSAXHandler() -> *mut xmlSAXHandlerV1 {
    if xmlIsMainThread() != 0 {
        return &mut htmlDefaultSAXHandler
    } else {
        return &mut (*(xmlGetGlobalState
            as unsafe extern "C" fn() -> xmlGlobalStatePtr)())
            .htmlDefaultSAXHandler
    };
}
#[no_mangle]
pub unsafe extern "C" fn __xmlLastError() -> *mut xmlError {
    if xmlIsMainThread() != 0 {
        return &mut xmlLastError
    } else {
        return &mut (*(xmlGetGlobalState
            as unsafe extern "C" fn() -> xmlGlobalStatePtr)())
            .xmlLastError
    };
}
#[no_mangle]
pub unsafe extern "C" fn __oldXMLWDcompatibility() -> *mut i32 {
    if xmlIsMainThread() != 0 {
        return &mut oldXMLWDcompatibility
    } else {
        return &mut (*(xmlGetGlobalState
            as unsafe extern "C" fn() -> xmlGlobalStatePtr)())
            .oldXMLWDcompatibility
    };
}
#[no_mangle]
pub unsafe extern "C" fn __xmlBufferAllocScheme() -> *mut xmlBufferAllocationScheme {
    if xmlIsMainThread() != 0 {
        return &mut xmlBufferAllocScheme
    } else {
        return &mut (*(xmlGetGlobalState
            as unsafe extern "C" fn() -> xmlGlobalStatePtr)())
            .xmlBufferAllocScheme
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefBufferAllocScheme(
    mut v: xmlBufferAllocationScheme,
) -> xmlBufferAllocationScheme {
    let mut ret: xmlBufferAllocationScheme = XML_BUFFER_ALLOC_DOUBLEIT;
    xmlMutexLock(xmlThrDefMutex);
    ret = xmlBufferAllocSchemeThrDef;
    xmlBufferAllocSchemeThrDef = v;
    xmlMutexUnlock(xmlThrDefMutex);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn __xmlDefaultBufferSize() -> *mut i32 {
    if xmlIsMainThread() != 0 {
        return &mut xmlDefaultBufferSize
    } else {
        return &mut (*(xmlGetGlobalState
            as unsafe extern "C" fn() -> xmlGlobalStatePtr)())
            .xmlDefaultBufferSize
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefDefaultBufferSize(mut v: i32) -> i32 {
    let mut ret: i32 = 0;
    xmlMutexLock(xmlThrDefMutex);
    ret = xmlDefaultBufferSizeThrDef;
    xmlDefaultBufferSizeThrDef = v;
    xmlMutexUnlock(xmlThrDefMutex);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn __xmlDefaultSAXHandler() -> *mut xmlSAXHandlerV1 {
    if xmlIsMainThread() != 0 {
        return &mut xmlDefaultSAXHandler
    } else {
        return &mut (*(xmlGetGlobalState
            as unsafe extern "C" fn() -> xmlGlobalStatePtr)())
            .xmlDefaultSAXHandler
    };
}
#[no_mangle]
pub unsafe extern "C" fn __xmlDefaultSAXLocator() -> *mut xmlSAXLocator {
    if xmlIsMainThread() != 0 {
        return &mut xmlDefaultSAXLocator
    } else {
        return &mut (*(xmlGetGlobalState
            as unsafe extern "C" fn() -> xmlGlobalStatePtr)())
            .xmlDefaultSAXLocator
    };
}
#[no_mangle]
pub unsafe extern "C" fn __xmlDoValidityCheckingDefaultValue() -> *mut i32 {
    if xmlIsMainThread() != 0 {
        return &mut xmlDoValidityCheckingDefaultValue
    } else {
        return &mut (*(xmlGetGlobalState
            as unsafe extern "C" fn() -> xmlGlobalStatePtr)())
            .xmlDoValidityCheckingDefaultValue
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefDoValidityCheckingDefaultValue(
    mut v: i32,
) -> i32 {
    let mut ret: i32 = 0;
    xmlMutexLock(xmlThrDefMutex);
    ret = xmlDoValidityCheckingDefaultValueThrDef;
    xmlDoValidityCheckingDefaultValueThrDef = v;
    xmlMutexUnlock(xmlThrDefMutex);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn __xmlGenericError() -> *mut xmlGenericErrorFunc {
    if xmlIsMainThread() != 0 {
        return &mut xmlGenericError
    } else {
        return &mut (*(xmlGetGlobalState
            as unsafe extern "C" fn() -> xmlGlobalStatePtr)())
            .xmlGenericError
    };
}
#[no_mangle]
pub unsafe extern "C" fn __xmlStructuredError() -> *mut xmlStructuredErrorFunc {
    if xmlIsMainThread() != 0 {
        return &mut xmlStructuredError
    } else {
        return &mut (*(xmlGetGlobalState
            as unsafe extern "C" fn() -> xmlGlobalStatePtr)())
            .xmlStructuredError
    };
}
#[no_mangle]
pub unsafe extern "C" fn __xmlGenericErrorContext() -> *mut *mut libc::c_void {
    if xmlIsMainThread() != 0 {
        return &mut xmlGenericErrorContext
    } else {
        return &mut (*(xmlGetGlobalState
            as unsafe extern "C" fn() -> xmlGlobalStatePtr)())
            .xmlGenericErrorContext
    };
}
#[no_mangle]
pub unsafe extern "C" fn __xmlStructuredErrorContext() -> *mut *mut libc::c_void {
    if xmlIsMainThread() != 0 {
        return &mut xmlStructuredErrorContext
    } else {
        return &mut (*(xmlGetGlobalState
            as unsafe extern "C" fn() -> xmlGlobalStatePtr)())
            .xmlStructuredErrorContext
    };
}
#[no_mangle]
pub unsafe extern "C" fn __xmlGetWarningsDefaultValue() -> *mut i32 {
    if xmlIsMainThread() != 0 {
        return &mut xmlGetWarningsDefaultValue
    } else {
        return &mut (*(xmlGetGlobalState
            as unsafe extern "C" fn() -> xmlGlobalStatePtr)())
            .xmlGetWarningsDefaultValue
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefGetWarningsDefaultValue(
    mut v: i32,
) -> i32 {
    let mut ret: i32 = 0;
    xmlMutexLock(xmlThrDefMutex);
    ret = xmlGetWarningsDefaultValueThrDef;
    xmlGetWarningsDefaultValueThrDef = v;
    xmlMutexUnlock(xmlThrDefMutex);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn __xmlIndentTreeOutput() -> *mut i32 {
    if xmlIsMainThread() != 0 {
        return &mut xmlIndentTreeOutput
    } else {
        return &mut (*(xmlGetGlobalState
            as unsafe extern "C" fn() -> xmlGlobalStatePtr)())
            .xmlIndentTreeOutput
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefIndentTreeOutput(mut v: i32) -> i32 {
    let mut ret: i32 = 0;
    xmlMutexLock(xmlThrDefMutex);
    ret = xmlIndentTreeOutputThrDef;
    xmlIndentTreeOutputThrDef = v;
    xmlMutexUnlock(xmlThrDefMutex);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn __xmlTreeIndentString() -> *mut *const i8 {
    if xmlIsMainThread() != 0 {
        return &mut xmlTreeIndentString
    } else {
        return &mut (*(xmlGetGlobalState
            as unsafe extern "C" fn() -> xmlGlobalStatePtr)())
            .xmlTreeIndentString
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefTreeIndentString(
    mut v: *const i8,
) -> *const i8 {
    let mut ret: *const i8 = 0 as *const i8;
    xmlMutexLock(xmlThrDefMutex);
    ret = xmlTreeIndentStringThrDef;
    xmlTreeIndentStringThrDef = v;
    xmlMutexUnlock(xmlThrDefMutex);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn __xmlKeepBlanksDefaultValue() -> *mut i32 {
    if xmlIsMainThread() != 0 {
        return &mut xmlKeepBlanksDefaultValue
    } else {
        return &mut (*(xmlGetGlobalState
            as unsafe extern "C" fn() -> xmlGlobalStatePtr)())
            .xmlKeepBlanksDefaultValue
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefKeepBlanksDefaultValue(
    mut v: i32,
) -> i32 {
    let mut ret: i32 = 0;
    xmlMutexLock(xmlThrDefMutex);
    ret = xmlKeepBlanksDefaultValueThrDef;
    xmlKeepBlanksDefaultValueThrDef = v;
    xmlMutexUnlock(xmlThrDefMutex);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn __xmlLineNumbersDefaultValue() -> *mut i32 {
    if xmlIsMainThread() != 0 {
        return &mut xmlLineNumbersDefaultValue
    } else {
        return &mut (*(xmlGetGlobalState
            as unsafe extern "C" fn() -> xmlGlobalStatePtr)())
            .xmlLineNumbersDefaultValue
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefLineNumbersDefaultValue(
    mut v: i32,
) -> i32 {
    let mut ret: i32 = 0;
    xmlMutexLock(xmlThrDefMutex);
    ret = xmlLineNumbersDefaultValueThrDef;
    xmlLineNumbersDefaultValueThrDef = v;
    xmlMutexUnlock(xmlThrDefMutex);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn __xmlLoadExtDtdDefaultValue() -> *mut i32 {
    if xmlIsMainThread() != 0 {
        return &mut xmlLoadExtDtdDefaultValue
    } else {
        return &mut (*(xmlGetGlobalState
            as unsafe extern "C" fn() -> xmlGlobalStatePtr)())
            .xmlLoadExtDtdDefaultValue
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefLoadExtDtdDefaultValue(
    mut v: i32,
) -> i32 {
    let mut ret: i32 = 0;
    xmlMutexLock(xmlThrDefMutex);
    ret = xmlLoadExtDtdDefaultValueThrDef;
    xmlLoadExtDtdDefaultValueThrDef = v;
    xmlMutexUnlock(xmlThrDefMutex);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn __xmlParserDebugEntities() -> *mut i32 {
    if xmlIsMainThread() != 0 {
        return &mut xmlParserDebugEntities
    } else {
        return &mut (*(xmlGetGlobalState
            as unsafe extern "C" fn() -> xmlGlobalStatePtr)())
            .xmlParserDebugEntities
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefParserDebugEntities(
    mut v: i32,
) -> i32 {
    let mut ret: i32 = 0;
    xmlMutexLock(xmlThrDefMutex);
    ret = xmlParserDebugEntitiesThrDef;
    xmlParserDebugEntitiesThrDef = v;
    xmlMutexUnlock(xmlThrDefMutex);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn __xmlParserVersion() -> *mut *const i8 {
    if xmlIsMainThread() != 0 {
        return &mut xmlParserVersion
    } else {
        return &mut (*(xmlGetGlobalState
            as unsafe extern "C" fn() -> xmlGlobalStatePtr)())
            .xmlParserVersion
    };
}
#[no_mangle]
pub unsafe extern "C" fn __xmlPedanticParserDefaultValue() -> *mut i32 {
    if xmlIsMainThread() != 0 {
        return &mut xmlPedanticParserDefaultValue
    } else {
        return &mut (*(xmlGetGlobalState
            as unsafe extern "C" fn() -> xmlGlobalStatePtr)())
            .xmlPedanticParserDefaultValue
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefPedanticParserDefaultValue(
    mut v: i32,
) -> i32 {
    let mut ret: i32 = 0;
    xmlMutexLock(xmlThrDefMutex);
    ret = xmlPedanticParserDefaultValueThrDef;
    xmlPedanticParserDefaultValueThrDef = v;
    xmlMutexUnlock(xmlThrDefMutex);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn __xmlSaveNoEmptyTags() -> *mut i32 {
    if xmlIsMainThread() != 0 {
        return &mut xmlSaveNoEmptyTags
    } else {
        return &mut (*(xmlGetGlobalState
            as unsafe extern "C" fn() -> xmlGlobalStatePtr)())
            .xmlSaveNoEmptyTags
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefSaveNoEmptyTags(mut v: i32) -> i32 {
    let mut ret: i32 = 0;
    xmlMutexLock(xmlThrDefMutex);
    ret = xmlSaveNoEmptyTagsThrDef;
    xmlSaveNoEmptyTagsThrDef = v;
    xmlMutexUnlock(xmlThrDefMutex);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn __xmlSubstituteEntitiesDefaultValue() -> *mut i32 {
    if xmlIsMainThread() != 0 {
        return &mut xmlSubstituteEntitiesDefaultValue
    } else {
        return &mut (*(xmlGetGlobalState
            as unsafe extern "C" fn() -> xmlGlobalStatePtr)())
            .xmlSubstituteEntitiesDefaultValue
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlThrDefSubstituteEntitiesDefaultValue(
    mut v: i32,
) -> i32 {
    let mut ret: i32 = 0;
    xmlMutexLock(xmlThrDefMutex);
    ret = xmlSubstituteEntitiesDefaultValueThrDef;
    xmlSubstituteEntitiesDefaultValueThrDef = v;
    xmlMutexUnlock(xmlThrDefMutex);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn __xmlRegisterNodeDefaultValue() -> *mut xmlRegisterNodeFunc {
    if xmlIsMainThread() != 0 {
        return &mut xmlRegisterNodeDefaultValue
    } else {
        return &mut (*(xmlGetGlobalState
            as unsafe extern "C" fn() -> xmlGlobalStatePtr)())
            .xmlRegisterNodeDefaultValue
    };
}
#[no_mangle]
pub unsafe extern "C" fn __xmlDeregisterNodeDefaultValue() -> *mut xmlDeregisterNodeFunc {
    if xmlIsMainThread() != 0 {
        return &mut xmlDeregisterNodeDefaultValue
    } else {
        return &mut (*(xmlGetGlobalState
            as unsafe extern "C" fn() -> xmlGlobalStatePtr)())
            .xmlDeregisterNodeDefaultValue
    };
}
#[no_mangle]
pub unsafe extern "C" fn __xmlParserInputBufferCreateFilenameValue() -> *mut xmlParserInputBufferCreateFilenameFunc {
    if xmlIsMainThread() != 0 {
        return &mut xmlParserInputBufferCreateFilenameValue
    } else {
        return &mut (*(xmlGetGlobalState
            as unsafe extern "C" fn() -> xmlGlobalStatePtr)())
            .xmlParserInputBufferCreateFilenameValue
    };
}
#[no_mangle]
pub unsafe extern "C" fn __xmlOutputBufferCreateFilenameValue() -> *mut xmlOutputBufferCreateFilenameFunc {
    if xmlIsMainThread() != 0 {
        return &mut xmlOutputBufferCreateFilenameValue
    } else {
        return &mut (*(xmlGetGlobalState
            as unsafe extern "C" fn() -> xmlGlobalStatePtr)())
            .xmlOutputBufferCreateFilenameValue
    };
}
