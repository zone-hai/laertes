use ::libc;
extern "C" {
<<<<<<< HEAD
    
    
    
    
    
    
    
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
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
pub use crate::src::SAX2::xmlSAX2InternalSubset;
pub use crate::src::SAX2::xmlSAX2IsStandalone;
pub use crate::src::SAX2::xmlSAX2NotationDecl;
pub use crate::src::SAX2::xmlSAX2ProcessingInstruction;
pub use crate::src::SAX2::xmlSAX2Reference;
pub use crate::src::SAX2::xmlSAX2ResolveEntity;
pub use crate::src::SAX2::xmlSAX2StartDocument;
pub use crate::src::SAX2::xmlSAX2StartElement;
pub use crate::src::SAX2::xmlSAX2UnparsedEntityDecl;
pub use crate::src::error::xmlParserValidityError;
pub use crate::src::error::xmlParserValidityWarning;
pub use crate::src::globals::__xmlGenericError;
pub use crate::src::globals::__xmlGenericErrorContext;
pub use crate::src::buf::_xmlBuf;
pub use crate::src::dict::_xmlDict;
pub use crate::src::hash::_xmlHashTable;
pub use crate::src::parser::_xmlStartTag;
pub use crate::src::valid::_xmlValidState;
pub use crate::src::xmlregexp::_xmlAutomata;
pub use crate::src::xmlregexp::_xmlAutomataState;
pub type xmlChar = crate::src::HTMLparser::xmlChar;
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

pub type _xmlParserInput = crate::src::HTMLparser::_xmlParserInput;
pub type xmlParserInputDeallocate = crate::src::HTMLparser::xmlParserInputDeallocate;
pub type xmlParserInput = crate::src::HTMLparser::xmlParserInput;
pub type xmlParserInputPtr = crate::src::HTMLparser::xmlParserInputPtr;
// #[derive(Copy, Clone)]

pub type _xmlParserCtxt = crate::src::HTMLparser::_xmlParserCtxt;
pub type xmlParserNodeInfo = crate::src::HTMLparser::xmlParserNodeInfo;
// #[derive(Copy, Clone)]

pub type _xmlParserNodeInfo = crate::src::HTMLparser::_xmlParserNodeInfo;
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
    pub type _xmlBuf;
    pub type _xmlDict;
    pub type _xmlHashTable;
    pub type _xmlStartTag;
    pub type _xmlAutomataState;
    pub type _xmlAutomata;
    pub type _xmlValidState;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn __xmlGenericErrorContext() -> *mut *mut libc::c_void;
    fn __xmlGenericError() -> *mut xmlGenericErrorFunc;
    fn xmlParserValidityError(ctx: *mut libc::c_void, msg: *const i8, _: ...);
    fn xmlParserValidityWarning(
        ctx: *mut libc::c_void,
        msg: *const i8,
        _: ...
    );
    fn xmlSAX2GetPublicId(ctx: *mut libc::c_void) -> *const xmlChar;
    fn xmlSAX2GetSystemId(ctx: *mut libc::c_void) -> *const xmlChar;
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
}
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
pub type xmlParserMode = crate::src::HTMLparser::xmlParserMode;
=======
pub type xmlParserMode = u32;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
pub const XML_PARSE_READER: xmlParserMode = 5;
pub const XML_PARSE_PUSH_SAX: xmlParserMode = 4;
pub const XML_PARSE_PUSH_DOM: xmlParserMode = 3;
pub const XML_PARSE_SAX: xmlParserMode = 2;
pub const XML_PARSE_DOM: xmlParserMode = 1;
pub const XML_PARSE_UNKNOWN: xmlParserMode = 0;
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
pub type xmlAttrPtr = crate::src::HTMLparser::xmlAttrPtr;
pub type xmlAttr = crate::src::HTMLparser::xmlAttr;
pub type xmlNodePtr = crate::src::HTMLparser::xmlNodePtr;
pub type xmlNode = crate::src::HTMLparser::xmlNode;
pub type xmlHashTablePtr = crate::src::HTMLparser::xmlHashTablePtr;
pub type xmlHashTable = crate::src::HTMLparser::xmlHashTable;
pub type xmlStartTag = crate::src::HTMLparser::xmlStartTag;
pub type xmlDictPtr = crate::src::HTMLparser::xmlDictPtr;
pub type xmlDict = crate::src::HTMLparser::xmlDict;
pub type xmlParserInputState = crate::src::HTMLparser::xmlParserInputState;
=======
pub type xmlAttrPtr = *mut xmlAttr;
pub type xmlAttr = _xmlAttr;
pub type xmlNodePtr = *mut xmlNode;
pub type xmlNode = _xmlNode;
pub type xmlHashTablePtr = *mut xmlHashTable;
pub type xmlHashTable = _xmlHashTable;
pub type xmlStartTag = _xmlStartTag;
pub type xmlDictPtr = *mut xmlDict;
pub type xmlDict = _xmlDict;
pub type xmlParserInputState = i32;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
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
pub type xmlValidCtxt = crate::src::HTMLparser::xmlValidCtxt;
// #[derive(Copy, Clone)]

pub type _xmlValidCtxt = crate::src::HTMLparser::_xmlValidCtxt;
pub type xmlAutomataStatePtr = crate::src::HTMLparser::xmlAutomataStatePtr;
pub type xmlAutomataState = crate::src::HTMLparser::xmlAutomataState;
pub type xmlAutomataPtr = crate::src::HTMLparser::xmlAutomataPtr;
pub type xmlAutomata = crate::src::HTMLparser::xmlAutomata;
pub type xmlValidState = crate::src::HTMLparser::xmlValidState;
pub type xmlDocPtr = crate::src::HTMLparser::xmlDocPtr;
pub type xmlDoc = crate::src::HTMLparser::xmlDoc;
pub type xmlValidityWarningFunc = crate::src::HTMLparser::xmlValidityWarningFunc;
pub type xmlValidityErrorFunc = crate::src::HTMLparser::xmlValidityErrorFunc;
pub type xmlParserNodeInfoSeq = crate::src::HTMLparser::xmlParserNodeInfoSeq;
// #[derive(Copy, Clone)]

pub type _xmlParserNodeInfoSeq = crate::src::HTMLparser::_xmlParserNodeInfoSeq;
// #[derive(Copy, Clone)]

pub type _xmlSAXHandler = crate::src::HTMLparser::_xmlSAXHandler;
pub type xmlStructuredErrorFunc = crate::src::HTMLparser::xmlStructuredErrorFunc;
pub type xmlErrorPtr = crate::src::HTMLparser::xmlErrorPtr;
pub type endElementNsSAX2Func = crate::src::HTMLparser::endElementNsSAX2Func;
pub type startElementNsSAX2Func = crate::src::HTMLparser::startElementNsSAX2Func;
pub type externalSubsetSAXFunc = crate::src::HTMLparser::externalSubsetSAXFunc;
pub type cdataBlockSAXFunc = crate::src::HTMLparser::cdataBlockSAXFunc;
pub type getParameterEntitySAXFunc = crate::src::HTMLparser::getParameterEntitySAXFunc;
pub type xmlEntityPtr = crate::src::HTMLparser::xmlEntityPtr;
pub type xmlEntity = crate::src::HTMLparser::xmlEntity;
// #[derive(Copy, Clone)]

pub type _xmlEntity = crate::src::HTMLparser::_xmlEntity;
pub type xmlEntityType = crate::src::HTMLparser::xmlEntityType;
=======
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
pub type xmlParserCtxt = crate::src::HTMLparser::xmlParserCtxt;
pub type xmlParserCtxtPtr = crate::src::HTMLparser::xmlParserCtxtPtr;
pub type xmlSAXHandler = crate::src::HTMLparser::xmlSAXHandler;
pub type xmlSAXHandlerPtr = crate::src::HTMLparser::xmlSAXHandlerPtr;
pub type xmlNsPtr = crate::src::HTMLtree::xmlNsPtr;
pub type xmlGenericErrorFunc = crate::src::HTMLparser::xmlGenericErrorFunc;
pub type htmlParserCtxtPtr = crate::src::HTMLparser::htmlParserCtxtPtr;
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
pub type xmlParserCtxt = _xmlParserCtxt;
pub type xmlParserCtxtPtr = *mut xmlParserCtxt;
pub type xmlSAXHandler = _xmlSAXHandler;
pub type xmlSAXHandlerPtr = *mut xmlSAXHandler;
pub type xmlNsPtr = *mut xmlNs;
pub type xmlGenericErrorFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const i8, ...) -> (),
>;
pub type htmlParserCtxtPtr = xmlParserCtxtPtr;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
#[no_mangle]
pub unsafe extern "C" fn htmlDecodeEntities(
    mut ctxt: htmlParserCtxtPtr,
    mut len: i32,
    mut end: xmlChar,
    mut end2: xmlChar,
    mut end3: xmlChar,
) -> *mut xmlChar {
    static mut deprecated: i32 = 0 as i32;
    if deprecated == 0 {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"htmlDecodeEntities() deprecated function reached\n\0" as *const u8
                as *const i8,
        );
        deprecated = 1 as i32;
    }
    return 0 as *mut xmlChar;
}
#[no_mangle]
<<<<<<< HEAD
pub extern "C" fn xmlInitializePredefinedEntities() {}
#[no_mangle]
pub extern "C" fn xmlCleanupPredefinedEntities() {}
=======
pub unsafe extern "C" fn xmlInitializePredefinedEntities() {}
#[no_mangle]
pub unsafe extern "C" fn xmlCleanupPredefinedEntities() {}
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
static mut xmlFeaturesList: [*const i8; 42] = [
    b"validate\0" as *const u8 as *const i8,
    b"load subset\0" as *const u8 as *const i8,
    b"keep blanks\0" as *const u8 as *const i8,
    b"disable SAX\0" as *const u8 as *const i8,
    b"fetch external entities\0" as *const u8 as *const i8,
    b"substitute entities\0" as *const u8 as *const i8,
    b"gather line info\0" as *const u8 as *const i8,
    b"user data\0" as *const u8 as *const i8,
    b"is html\0" as *const u8 as *const i8,
    b"is standalone\0" as *const u8 as *const i8,
    b"stop parser\0" as *const u8 as *const i8,
    b"document\0" as *const u8 as *const i8,
    b"is well formed\0" as *const u8 as *const i8,
    b"is valid\0" as *const u8 as *const i8,
    b"SAX block\0" as *const u8 as *const i8,
    b"SAX function internalSubset\0" as *const u8 as *const i8,
    b"SAX function isStandalone\0" as *const u8 as *const i8,
    b"SAX function hasInternalSubset\0" as *const u8 as *const i8,
    b"SAX function hasExternalSubset\0" as *const u8 as *const i8,
    b"SAX function resolveEntity\0" as *const u8 as *const i8,
    b"SAX function getEntity\0" as *const u8 as *const i8,
    b"SAX function entityDecl\0" as *const u8 as *const i8,
    b"SAX function notationDecl\0" as *const u8 as *const i8,
    b"SAX function attributeDecl\0" as *const u8 as *const i8,
    b"SAX function elementDecl\0" as *const u8 as *const i8,
    b"SAX function unparsedEntityDecl\0" as *const u8 as *const i8,
    b"SAX function setDocumentLocator\0" as *const u8 as *const i8,
    b"SAX function startDocument\0" as *const u8 as *const i8,
    b"SAX function endDocument\0" as *const u8 as *const i8,
    b"SAX function startElement\0" as *const u8 as *const i8,
    b"SAX function endElement\0" as *const u8 as *const i8,
    b"SAX function reference\0" as *const u8 as *const i8,
    b"SAX function characters\0" as *const u8 as *const i8,
    b"SAX function ignorableWhitespace\0" as *const u8 as *const i8,
    b"SAX function processingInstruction\0" as *const u8 as *const i8,
    b"SAX function comment\0" as *const u8 as *const i8,
    b"SAX function warning\0" as *const u8 as *const i8,
    b"SAX function error\0" as *const u8 as *const i8,
    b"SAX function fatalError\0" as *const u8 as *const i8,
    b"SAX function getParameterEntity\0" as *const u8 as *const i8,
    b"SAX function cdataBlock\0" as *const u8 as *const i8,
    b"SAX function externalSubset\0" as *const u8 as *const i8,
];
#[no_mangle]
pub unsafe extern "C" fn xmlGetFeaturesList(
    mut len: *mut i32,
    mut result: *mut *const i8,
) -> i32 {
    let mut ret: i32 = 0;
    let mut i: i32 = 0;
    ret = (::std::mem::size_of::<[*const i8; 42]>() as u64)
        .wrapping_div(::std::mem::size_of::<*const i8>() as u64)
        as i32;
    if len.is_null() || result.is_null() {
        return ret;
    }
    if *len < 0 as i32 || *len >= 1000 as i32 {
        return -(1 as i32);
    }
    if *len > ret {
        *len = ret;
    }
    i = 0 as i32;
    while i < *len {
        let ref mut fresh0 = *result.offset(i as isize);
        *fresh0 = xmlFeaturesList[i as usize];
        i += 1;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlGetFeature(
    mut ctxt: xmlParserCtxtPtr,
    mut name: *const i8,
    mut result: *mut libc::c_void,
) -> i32 {
    if ctxt.is_null() || name.is_null() || result.is_null() {
        return -(1 as i32);
    }
    if strcmp(name, b"validate\0" as *const u8 as *const i8) == 0 {
        *(result as *mut i32) = (*ctxt).validate;
    } else if strcmp(name, b"keep blanks\0" as *const u8 as *const i8) == 0 {
        *(result as *mut i32) = (*ctxt).keepBlanks;
    } else if strcmp(name, b"disable SAX\0" as *const u8 as *const i8) == 0 {
        *(result as *mut i32) = (*ctxt).disableSAX;
    } else if strcmp(
            name,
            b"fetch external entities\0" as *const u8 as *const i8,
        ) == 0
        {
        *(result as *mut i32) = (*ctxt).loadsubset;
    } else if strcmp(name, b"substitute entities\0" as *const u8 as *const i8)
            == 0
        {
        *(result as *mut i32) = (*ctxt).replaceEntities;
    } else if strcmp(name, b"gather line info\0" as *const u8 as *const i8)
            == 0
        {
        *(result as *mut i32) = (*ctxt).record_info;
    } else if strcmp(name, b"user data\0" as *const u8 as *const i8) == 0 {
        let ref mut fresh1 = *(result as *mut *mut libc::c_void);
        *fresh1 = (*ctxt).userData;
    } else if strcmp(name, b"is html\0" as *const u8 as *const i8) == 0 {
        *(result as *mut i32) = (*ctxt).html;
    } else if strcmp(name, b"is standalone\0" as *const u8 as *const i8) == 0 {
        *(result as *mut i32) = (*ctxt).standalone;
    } else if strcmp(name, b"document\0" as *const u8 as *const i8) == 0 {
        let ref mut fresh2 = *(result as *mut xmlDocPtr);
        *fresh2 = (*ctxt).myDoc;
    } else if strcmp(name, b"is well formed\0" as *const u8 as *const i8) == 0
        {
        *(result as *mut i32) = (*ctxt).wellFormed;
    } else if strcmp(name, b"is valid\0" as *const u8 as *const i8) == 0 {
        *(result as *mut i32) = (*ctxt).valid;
    } else if strcmp(name, b"SAX block\0" as *const u8 as *const i8) == 0 {
        let ref mut fresh3 = *(result as *mut xmlSAXHandlerPtr);
        *fresh3 = (*ctxt).sax;
    } else if strcmp(
            name,
            b"SAX function internalSubset\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh4 = *(result as *mut internalSubsetSAXFunc);
        *fresh4 = (*(*ctxt).sax).internalSubset;
    } else if strcmp(
            name,
            b"SAX function isStandalone\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh5 = *(result as *mut isStandaloneSAXFunc);
        *fresh5 = (*(*ctxt).sax).isStandalone;
    } else if strcmp(
            name,
            b"SAX function hasInternalSubset\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh6 = *(result as *mut hasInternalSubsetSAXFunc);
        *fresh6 = (*(*ctxt).sax).hasInternalSubset;
    } else if strcmp(
            name,
            b"SAX function hasExternalSubset\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh7 = *(result as *mut hasExternalSubsetSAXFunc);
        *fresh7 = (*(*ctxt).sax).hasExternalSubset;
    } else if strcmp(
            name,
            b"SAX function resolveEntity\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh8 = *(result as *mut resolveEntitySAXFunc);
        *fresh8 = (*(*ctxt).sax).resolveEntity;
    } else if strcmp(
            name,
            b"SAX function getEntity\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh9 = *(result as *mut getEntitySAXFunc);
        *fresh9 = (*(*ctxt).sax).getEntity;
    } else if strcmp(
            name,
            b"SAX function entityDecl\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh10 = *(result as *mut entityDeclSAXFunc);
        *fresh10 = (*(*ctxt).sax).entityDecl;
    } else if strcmp(
            name,
            b"SAX function notationDecl\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh11 = *(result as *mut notationDeclSAXFunc);
        *fresh11 = (*(*ctxt).sax).notationDecl;
    } else if strcmp(
            name,
            b"SAX function attributeDecl\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh12 = *(result as *mut attributeDeclSAXFunc);
        *fresh12 = (*(*ctxt).sax).attributeDecl;
    } else if strcmp(
            name,
            b"SAX function elementDecl\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh13 = *(result as *mut elementDeclSAXFunc);
        *fresh13 = (*(*ctxt).sax).elementDecl;
    } else if strcmp(
            name,
            b"SAX function unparsedEntityDecl\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh14 = *(result as *mut unparsedEntityDeclSAXFunc);
        *fresh14 = (*(*ctxt).sax).unparsedEntityDecl;
    } else if strcmp(
            name,
            b"SAX function setDocumentLocator\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh15 = *(result as *mut setDocumentLocatorSAXFunc);
        *fresh15 = (*(*ctxt).sax).setDocumentLocator;
    } else if strcmp(
            name,
            b"SAX function startDocument\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh16 = *(result as *mut startDocumentSAXFunc);
        *fresh16 = (*(*ctxt).sax).startDocument;
    } else if strcmp(
            name,
            b"SAX function endDocument\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh17 = *(result as *mut endDocumentSAXFunc);
        *fresh17 = (*(*ctxt).sax).endDocument;
    } else if strcmp(
            name,
            b"SAX function startElement\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh18 = *(result as *mut startElementSAXFunc);
        *fresh18 = (*(*ctxt).sax).startElement;
    } else if strcmp(
            name,
            b"SAX function endElement\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh19 = *(result as *mut endElementSAXFunc);
        *fresh19 = (*(*ctxt).sax).endElement;
    } else if strcmp(
            name,
            b"SAX function reference\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh20 = *(result as *mut referenceSAXFunc);
        *fresh20 = (*(*ctxt).sax).reference;
    } else if strcmp(
            name,
            b"SAX function characters\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh21 = *(result as *mut charactersSAXFunc);
        *fresh21 = (*(*ctxt).sax).characters;
    } else if strcmp(
            name,
            b"SAX function ignorableWhitespace\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh22 = *(result as *mut ignorableWhitespaceSAXFunc);
        *fresh22 = (*(*ctxt).sax).ignorableWhitespace;
    } else if strcmp(
            name,
            b"SAX function processingInstruction\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh23 = *(result as *mut processingInstructionSAXFunc);
        *fresh23 = (*(*ctxt).sax).processingInstruction;
    } else if strcmp(name, b"SAX function comment\0" as *const u8 as *const i8)
            == 0
        {
        let ref mut fresh24 = *(result as *mut commentSAXFunc);
        *fresh24 = (*(*ctxt).sax).comment;
    } else if strcmp(name, b"SAX function warning\0" as *const u8 as *const i8)
            == 0
        {
        let ref mut fresh25 = *(result as *mut warningSAXFunc);
        *fresh25 = (*(*ctxt).sax).warning;
    } else if strcmp(name, b"SAX function error\0" as *const u8 as *const i8)
            == 0
        {
        let ref mut fresh26 = *(result as *mut errorSAXFunc);
        *fresh26 = (*(*ctxt).sax).error;
    } else if strcmp(
            name,
            b"SAX function fatalError\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh27 = *(result as *mut fatalErrorSAXFunc);
        *fresh27 = (*(*ctxt).sax).fatalError;
    } else if strcmp(
            name,
            b"SAX function getParameterEntity\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh28 = *(result as *mut getParameterEntitySAXFunc);
        *fresh28 = (*(*ctxt).sax).getParameterEntity;
    } else if strcmp(
            name,
            b"SAX function cdataBlock\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh29 = *(result as *mut cdataBlockSAXFunc);
        *fresh29 = (*(*ctxt).sax).cdataBlock;
    } else if strcmp(
            name,
            b"SAX function externalSubset\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh30 = *(result as *mut externalSubsetSAXFunc);
        *fresh30 = (*(*ctxt).sax).externalSubset;
    } else {
        return -(1 as i32)
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn xmlSetFeature(
    mut ctxt: xmlParserCtxtPtr,
    mut name: *const i8,
    mut value: *mut libc::c_void,
) -> i32 {
    if ctxt.is_null() || name.is_null() || value.is_null() {
        return -(1 as i32);
    }
    if strcmp(name, b"validate\0" as *const u8 as *const i8) == 0 {
        let mut newvalidate: i32 = *(value as *mut i32);
        if (*ctxt).validate == 0 && newvalidate != 0 as i32 {
            if ((*ctxt).vctxt.warning).is_none() {
                let ref mut fresh31 = (*ctxt).vctxt.warning;
                *fresh31 = Some(
                    xmlParserValidityWarning
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *const i8,
                            ...
                        ) -> (),
                );
            }
            if ((*ctxt).vctxt.error).is_none() {
                let ref mut fresh32 = (*ctxt).vctxt.error;
                *fresh32 = Some(
                    xmlParserValidityError
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *const i8,
                            ...
                        ) -> (),
                );
            }
            (*ctxt).vctxt.nodeMax = 0 as i32;
        }
        (*ctxt).validate = newvalidate;
    } else if strcmp(name, b"keep blanks\0" as *const u8 as *const i8) == 0 {
        (*ctxt).keepBlanks = *(value as *mut i32);
    } else if strcmp(name, b"disable SAX\0" as *const u8 as *const i8) == 0 {
        (*ctxt).disableSAX = *(value as *mut i32);
    } else if strcmp(
            name,
            b"fetch external entities\0" as *const u8 as *const i8,
        ) == 0
        {
        (*ctxt).loadsubset = *(value as *mut i32);
    } else if strcmp(name, b"substitute entities\0" as *const u8 as *const i8)
            == 0
        {
        (*ctxt).replaceEntities = *(value as *mut i32);
    } else if strcmp(name, b"gather line info\0" as *const u8 as *const i8)
            == 0
        {
        (*ctxt).record_info = *(value as *mut i32);
    } else if strcmp(name, b"user data\0" as *const u8 as *const i8) == 0 {
        let ref mut fresh33 = (*ctxt).userData;
        *fresh33 = *(value as *mut *mut libc::c_void);
    } else if strcmp(name, b"is html\0" as *const u8 as *const i8) == 0 {
        (*ctxt).html = *(value as *mut i32);
    } else if strcmp(name, b"is standalone\0" as *const u8 as *const i8) == 0 {
        (*ctxt).standalone = *(value as *mut i32);
    } else if strcmp(name, b"document\0" as *const u8 as *const i8) == 0 {
        let ref mut fresh34 = (*ctxt).myDoc;
        *fresh34 = *(value as *mut xmlDocPtr);
    } else if strcmp(name, b"is well formed\0" as *const u8 as *const i8) == 0
        {
        (*ctxt).wellFormed = *(value as *mut i32);
    } else if strcmp(name, b"is valid\0" as *const u8 as *const i8) == 0 {
        (*ctxt).valid = *(value as *mut i32);
    } else if strcmp(name, b"SAX block\0" as *const u8 as *const i8) == 0 {
        let ref mut fresh35 = (*ctxt).sax;
        *fresh35 = *(value as *mut xmlSAXHandlerPtr);
    } else if strcmp(
            name,
            b"SAX function internalSubset\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh36 = (*(*ctxt).sax).internalSubset;
        *fresh36 = *(value as *mut internalSubsetSAXFunc);
    } else if strcmp(
            name,
            b"SAX function isStandalone\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh37 = (*(*ctxt).sax).isStandalone;
        *fresh37 = *(value as *mut isStandaloneSAXFunc);
    } else if strcmp(
            name,
            b"SAX function hasInternalSubset\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh38 = (*(*ctxt).sax).hasInternalSubset;
        *fresh38 = *(value as *mut hasInternalSubsetSAXFunc);
    } else if strcmp(
            name,
            b"SAX function hasExternalSubset\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh39 = (*(*ctxt).sax).hasExternalSubset;
        *fresh39 = *(value as *mut hasExternalSubsetSAXFunc);
    } else if strcmp(
            name,
            b"SAX function resolveEntity\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh40 = (*(*ctxt).sax).resolveEntity;
        *fresh40 = *(value as *mut resolveEntitySAXFunc);
    } else if strcmp(
            name,
            b"SAX function getEntity\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh41 = (*(*ctxt).sax).getEntity;
        *fresh41 = *(value as *mut getEntitySAXFunc);
    } else if strcmp(
            name,
            b"SAX function entityDecl\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh42 = (*(*ctxt).sax).entityDecl;
        *fresh42 = *(value as *mut entityDeclSAXFunc);
    } else if strcmp(
            name,
            b"SAX function notationDecl\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh43 = (*(*ctxt).sax).notationDecl;
        *fresh43 = *(value as *mut notationDeclSAXFunc);
    } else if strcmp(
            name,
            b"SAX function attributeDecl\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh44 = (*(*ctxt).sax).attributeDecl;
        *fresh44 = *(value as *mut attributeDeclSAXFunc);
    } else if strcmp(
            name,
            b"SAX function elementDecl\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh45 = (*(*ctxt).sax).elementDecl;
        *fresh45 = *(value as *mut elementDeclSAXFunc);
    } else if strcmp(
            name,
            b"SAX function unparsedEntityDecl\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh46 = (*(*ctxt).sax).unparsedEntityDecl;
        *fresh46 = *(value as *mut unparsedEntityDeclSAXFunc);
    } else if strcmp(
            name,
            b"SAX function setDocumentLocator\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh47 = (*(*ctxt).sax).setDocumentLocator;
        *fresh47 = *(value as *mut setDocumentLocatorSAXFunc);
    } else if strcmp(
            name,
            b"SAX function startDocument\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh48 = (*(*ctxt).sax).startDocument;
        *fresh48 = *(value as *mut startDocumentSAXFunc);
    } else if strcmp(
            name,
            b"SAX function endDocument\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh49 = (*(*ctxt).sax).endDocument;
        *fresh49 = *(value as *mut endDocumentSAXFunc);
    } else if strcmp(
            name,
            b"SAX function startElement\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh50 = (*(*ctxt).sax).startElement;
        *fresh50 = *(value as *mut startElementSAXFunc);
    } else if strcmp(
            name,
            b"SAX function endElement\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh51 = (*(*ctxt).sax).endElement;
        *fresh51 = *(value as *mut endElementSAXFunc);
    } else if strcmp(
            name,
            b"SAX function reference\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh52 = (*(*ctxt).sax).reference;
        *fresh52 = *(value as *mut referenceSAXFunc);
    } else if strcmp(
            name,
            b"SAX function characters\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh53 = (*(*ctxt).sax).characters;
        *fresh53 = *(value as *mut charactersSAXFunc);
    } else if strcmp(
            name,
            b"SAX function ignorableWhitespace\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh54 = (*(*ctxt).sax).ignorableWhitespace;
        *fresh54 = *(value as *mut ignorableWhitespaceSAXFunc);
    } else if strcmp(
            name,
            b"SAX function processingInstruction\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh55 = (*(*ctxt).sax).processingInstruction;
        *fresh55 = *(value as *mut processingInstructionSAXFunc);
    } else if strcmp(name, b"SAX function comment\0" as *const u8 as *const i8)
            == 0
        {
        let ref mut fresh56 = (*(*ctxt).sax).comment;
        *fresh56 = *(value as *mut commentSAXFunc);
    } else if strcmp(name, b"SAX function warning\0" as *const u8 as *const i8)
            == 0
        {
        let ref mut fresh57 = (*(*ctxt).sax).warning;
        *fresh57 = *(value as *mut warningSAXFunc);
    } else if strcmp(name, b"SAX function error\0" as *const u8 as *const i8)
            == 0
        {
        let ref mut fresh58 = (*(*ctxt).sax).error;
        *fresh58 = *(value as *mut errorSAXFunc);
    } else if strcmp(
            name,
            b"SAX function fatalError\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh59 = (*(*ctxt).sax).fatalError;
        *fresh59 = *(value as *mut fatalErrorSAXFunc);
    } else if strcmp(
            name,
            b"SAX function getParameterEntity\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh60 = (*(*ctxt).sax).getParameterEntity;
        *fresh60 = *(value as *mut getParameterEntitySAXFunc);
    } else if strcmp(
            name,
            b"SAX function cdataBlock\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh61 = (*(*ctxt).sax).cdataBlock;
        *fresh61 = *(value as *mut cdataBlockSAXFunc);
    } else if strcmp(
            name,
            b"SAX function externalSubset\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh62 = (*(*ctxt).sax).externalSubset;
        *fresh62 = *(value as *mut externalSubsetSAXFunc);
    } else {
        return -(1 as i32)
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn xmlDecodeEntities(
    mut ctxt: xmlParserCtxtPtr,
    mut len: i32,
    mut what: i32,
    mut end: xmlChar,
    mut end2: xmlChar,
    mut end3: xmlChar,
) -> *mut xmlChar {
    static mut deprecated: i32 = 0 as i32;
    if deprecated == 0 {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"xmlDecodeEntities() deprecated function reached\n\0" as *const u8
                as *const i8,
        );
        deprecated = 1 as i32;
    }
    return 0 as *mut xmlChar;
}
#[no_mangle]
pub unsafe extern "C" fn xmlNamespaceParseNCName(
    mut ctxt: xmlParserCtxtPtr,
) -> *mut xmlChar {
    static mut deprecated: i32 = 0 as i32;
    if deprecated == 0 {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"xmlNamespaceParseNCName() deprecated function reached\n\0" as *const u8
                as *const i8,
        );
        deprecated = 1 as i32;
    }
    return 0 as *mut xmlChar;
}
#[no_mangle]
pub unsafe extern "C" fn xmlNamespaceParseQName(
    mut ctxt: xmlParserCtxtPtr,
    mut prefix: *mut *mut xmlChar,
) -> *mut xmlChar {
    static mut deprecated: i32 = 0 as i32;
    if deprecated == 0 {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"xmlNamespaceParseQName() deprecated function reached\n\0" as *const u8
                as *const i8,
        );
        deprecated = 1 as i32;
    }
    return 0 as *mut xmlChar;
}
#[no_mangle]
pub unsafe extern "C" fn xmlNamespaceParseNSDef(
    mut ctxt: xmlParserCtxtPtr,
) -> *mut xmlChar {
    static mut deprecated: i32 = 0 as i32;
    if deprecated == 0 {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"xmlNamespaceParseNSDef() deprecated function reached\n\0" as *const u8
                as *const i8,
        );
        deprecated = 1 as i32;
    }
    return 0 as *mut xmlChar;
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseQuotedString(
    mut ctxt: xmlParserCtxtPtr,
) -> *mut xmlChar {
    static mut deprecated: i32 = 0 as i32;
    if deprecated == 0 {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"xmlParseQuotedString() deprecated function reached\n\0" as *const u8
                as *const i8,
        );
        deprecated = 1 as i32;
    }
    return 0 as *mut xmlChar;
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseNamespace(mut ctxt: xmlParserCtxtPtr) {
    static mut deprecated: i32 = 0 as i32;
    if deprecated == 0 {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"xmlParseNamespace() deprecated function reached\n\0" as *const u8
                as *const i8,
        );
        deprecated = 1 as i32;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlScanName(mut ctxt: xmlParserCtxtPtr) -> *mut xmlChar {
    static mut deprecated: i32 = 0 as i32;
    if deprecated == 0 {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"xmlScanName() deprecated function reached\n\0" as *const u8
                as *const i8,
        );
        deprecated = 1 as i32;
    }
    return 0 as *mut xmlChar;
}
#[no_mangle]
pub unsafe extern "C" fn xmlParserHandleReference(mut ctxt: xmlParserCtxtPtr) {
    static mut deprecated: i32 = 0 as i32;
    if deprecated == 0 {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"xmlParserHandleReference() deprecated function reached\n\0" as *const u8
                as *const i8,
        );
        deprecated = 1 as i32;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlHandleEntity(
    mut ctxt: xmlParserCtxtPtr,
    mut entity: xmlEntityPtr,
) {
    static mut deprecated: i32 = 0 as i32;
    if deprecated == 0 {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"xmlHandleEntity() deprecated function reached\n\0" as *const u8
                as *const i8,
        );
        deprecated = 1 as i32;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlNewGlobalNs(
    mut doc: xmlDocPtr,
    mut href: *const xmlChar,
    mut prefix: *const xmlChar,
) -> xmlNsPtr {
    static mut deprecated: i32 = 0 as i32;
    if deprecated == 0 {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"xmlNewGlobalNs() deprecated function reached\n\0" as *const u8
                as *const i8,
        );
        deprecated = 1 as i32;
    }
    return 0 as xmlNsPtr;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUpgradeOldNs(mut doc: xmlDocPtr) {
    static mut deprecated: i32 = 0 as i32;
    if deprecated == 0 {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"xmlUpgradeOldNs() deprecated function reached\n\0" as *const u8
                as *const i8,
        );
        deprecated = 1 as i32;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlEncodeEntities(
    mut doc: xmlDocPtr,
    mut input: *const xmlChar,
) -> *const xmlChar {
    static mut warning: i32 = 1 as i32;
    if warning != 0 {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"Deprecated API xmlEncodeEntities() used\n\0" as *const u8
                as *const i8,
        );
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"   change code to use xmlEncodeEntitiesReentrant()\n\0" as *const u8
                as *const i8,
        );
        warning = 0 as i32;
    }
    return 0 as *const xmlChar;
}
static mut deprecated_v1_msg: i32 = 0 as i32;
#[no_mangle]
pub unsafe extern "C" fn getPublicId(mut ctx: *mut libc::c_void) -> *const xmlChar {
    if deprecated_v1_msg == 0 as i32 {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"Use of deprecated SAXv1 function %s\n\0" as *const u8
                as *const i8,
            b"getPublicId\0" as *const u8 as *const i8,
        );
    }
    deprecated_v1_msg += 1;
    return xmlSAX2GetPublicId(ctx);
}
#[no_mangle]
pub unsafe extern "C" fn getSystemId(mut ctx: *mut libc::c_void) -> *const xmlChar {
    if deprecated_v1_msg == 0 as i32 {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"Use of deprecated SAXv1 function %s\n\0" as *const u8
                as *const i8,
            b"getSystemId\0" as *const u8 as *const i8,
        );
    }
    deprecated_v1_msg += 1;
    return xmlSAX2GetSystemId(ctx);
}
#[no_mangle]
pub unsafe extern "C" fn getLineNumber(mut ctx: *mut libc::c_void) -> i32 {
    if deprecated_v1_msg == 0 as i32 {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"Use of deprecated SAXv1 function %s\n\0" as *const u8
                as *const i8,
            b"getLineNumber\0" as *const u8 as *const i8,
        );
    }
    deprecated_v1_msg += 1;
    return xmlSAX2GetLineNumber(ctx);
}
#[no_mangle]
pub unsafe extern "C" fn getColumnNumber(mut ctx: *mut libc::c_void) -> i32 {
    if deprecated_v1_msg == 0 as i32 {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"Use of deprecated SAXv1 function %s\n\0" as *const u8
                as *const i8,
            b"getColumnNumber\0" as *const u8 as *const i8,
        );
    }
    deprecated_v1_msg += 1;
    return xmlSAX2GetColumnNumber(ctx);
}
#[no_mangle]
pub unsafe extern "C" fn isStandalone(mut ctx: *mut libc::c_void) -> i32 {
    if deprecated_v1_msg == 0 as i32 {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"Use of deprecated SAXv1 function %s\n\0" as *const u8
                as *const i8,
            b"isStandalone\0" as *const u8 as *const i8,
        );
    }
    deprecated_v1_msg += 1;
    return xmlSAX2IsStandalone(ctx);
}
#[no_mangle]
pub unsafe extern "C" fn hasInternalSubset(mut ctx: *mut libc::c_void) -> i32 {
    if deprecated_v1_msg == 0 as i32 {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"Use of deprecated SAXv1 function %s\n\0" as *const u8
                as *const i8,
            b"hasInternalSubset\0" as *const u8 as *const i8,
        );
    }
    deprecated_v1_msg += 1;
    return xmlSAX2HasInternalSubset(ctx);
}
#[no_mangle]
pub unsafe extern "C" fn hasExternalSubset(mut ctx: *mut libc::c_void) -> i32 {
    if deprecated_v1_msg == 0 as i32 {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"Use of deprecated SAXv1 function %s\n\0" as *const u8
                as *const i8,
            b"hasExternalSubset\0" as *const u8 as *const i8,
        );
    }
    deprecated_v1_msg += 1;
    return xmlSAX2HasExternalSubset(ctx);
}
#[no_mangle]
pub unsafe extern "C" fn internalSubset(
    mut ctx: *mut libc::c_void,
    mut name: *const xmlChar,
    mut ExternalID: *const xmlChar,
    mut SystemID: *const xmlChar,
) {
    if deprecated_v1_msg == 0 as i32 {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"Use of deprecated SAXv1 function %s\n\0" as *const u8
                as *const i8,
            b"internalSubset\0" as *const u8 as *const i8,
        );
    }
    deprecated_v1_msg += 1;
    xmlSAX2InternalSubset(ctx, name, ExternalID, SystemID);
}
#[no_mangle]
pub unsafe extern "C" fn externalSubset(
    mut ctx: *mut libc::c_void,
    mut name: *const xmlChar,
    mut ExternalID: *const xmlChar,
    mut SystemID: *const xmlChar,
) {
    if deprecated_v1_msg == 0 as i32 {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"Use of deprecated SAXv1 function %s\n\0" as *const u8
                as *const i8,
            b"externalSubset\0" as *const u8 as *const i8,
        );
    }
    deprecated_v1_msg += 1;
    xmlSAX2ExternalSubset(ctx, name, ExternalID, SystemID);
}
#[no_mangle]
pub unsafe extern "C" fn resolveEntity(
    mut ctx: *mut libc::c_void,
    mut publicId: *const xmlChar,
    mut systemId: *const xmlChar,
) -> xmlParserInputPtr {
    if deprecated_v1_msg == 0 as i32 {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"Use of deprecated SAXv1 function %s\n\0" as *const u8
                as *const i8,
            b"resolveEntity\0" as *const u8 as *const i8,
        );
    }
    deprecated_v1_msg += 1;
    return xmlSAX2ResolveEntity(ctx, publicId, systemId);
}
#[no_mangle]
pub unsafe extern "C" fn getEntity(
    mut ctx: *mut libc::c_void,
    mut name: *const xmlChar,
) -> xmlEntityPtr {
    if deprecated_v1_msg == 0 as i32 {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"Use of deprecated SAXv1 function %s\n\0" as *const u8
                as *const i8,
            b"getEntity\0" as *const u8 as *const i8,
        );
    }
    deprecated_v1_msg += 1;
    return xmlSAX2GetEntity(ctx, name);
}
#[no_mangle]
pub unsafe extern "C" fn getParameterEntity(
    mut ctx: *mut libc::c_void,
    mut name: *const xmlChar,
) -> xmlEntityPtr {
    if deprecated_v1_msg == 0 as i32 {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"Use of deprecated SAXv1 function %s\n\0" as *const u8
                as *const i8,
            b"getParameterEntity\0" as *const u8 as *const i8,
        );
    }
    deprecated_v1_msg += 1;
    return xmlSAX2GetParameterEntity(ctx, name);
}
#[no_mangle]
pub unsafe extern "C" fn entityDecl(
    mut ctx: *mut libc::c_void,
    mut name: *const xmlChar,
    mut type_0: i32,
    mut publicId: *const xmlChar,
    mut systemId: *const xmlChar,
    mut content: *mut xmlChar,
) {
    if deprecated_v1_msg == 0 as i32 {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"Use of deprecated SAXv1 function %s\n\0" as *const u8
                as *const i8,
            b"entityDecl\0" as *const u8 as *const i8,
        );
    }
    deprecated_v1_msg += 1;
    xmlSAX2EntityDecl(ctx, name, type_0, publicId, systemId, content);
}
#[no_mangle]
pub unsafe extern "C" fn attributeDecl(
    mut ctx: *mut libc::c_void,
    mut elem: *const xmlChar,
    mut fullname: *const xmlChar,
    mut type_0: i32,
    mut def: i32,
    mut defaultValue: *const xmlChar,
    mut tree: xmlEnumerationPtr,
) {
    if deprecated_v1_msg == 0 as i32 {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"Use of deprecated SAXv1 function %s\n\0" as *const u8
                as *const i8,
            b"attributeDecl\0" as *const u8 as *const i8,
        );
    }
    deprecated_v1_msg += 1;
    xmlSAX2AttributeDecl(ctx, elem, fullname, type_0, def, defaultValue, tree);
}
#[no_mangle]
pub unsafe extern "C" fn elementDecl(
    mut ctx: *mut libc::c_void,
    mut name: *const xmlChar,
    mut type_0: i32,
    mut content: xmlElementContentPtr,
) {
    if deprecated_v1_msg == 0 as i32 {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"Use of deprecated SAXv1 function %s\n\0" as *const u8
                as *const i8,
            b"elementDecl\0" as *const u8 as *const i8,
        );
    }
    deprecated_v1_msg += 1;
    xmlSAX2ElementDecl(ctx, name, type_0, content);
}
#[no_mangle]
pub unsafe extern "C" fn notationDecl(
    mut ctx: *mut libc::c_void,
    mut name: *const xmlChar,
    mut publicId: *const xmlChar,
    mut systemId: *const xmlChar,
) {
    if deprecated_v1_msg == 0 as i32 {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"Use of deprecated SAXv1 function %s\n\0" as *const u8
                as *const i8,
            b"notationDecl\0" as *const u8 as *const i8,
        );
    }
    deprecated_v1_msg += 1;
    xmlSAX2NotationDecl(ctx, name, publicId, systemId);
}
#[no_mangle]
pub unsafe extern "C" fn unparsedEntityDecl(
    mut ctx: *mut libc::c_void,
    mut name: *const xmlChar,
    mut publicId: *const xmlChar,
    mut systemId: *const xmlChar,
    mut notationName: *const xmlChar,
) {
    if deprecated_v1_msg == 0 as i32 {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"Use of deprecated SAXv1 function %s\n\0" as *const u8
                as *const i8,
            b"unparsedEntityDecl\0" as *const u8 as *const i8,
        );
    }
    deprecated_v1_msg += 1;
    xmlSAX2UnparsedEntityDecl(ctx, name, publicId, systemId, notationName);
}
#[no_mangle]
pub unsafe extern "C" fn setDocumentLocator(
    mut ctx: *mut libc::c_void,
    mut loc: xmlSAXLocatorPtr,
) {
    if deprecated_v1_msg == 0 as i32 {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"Use of deprecated SAXv1 function %s\n\0" as *const u8
                as *const i8,
            b"setDocumentLocator\0" as *const u8 as *const i8,
        );
    }
    deprecated_v1_msg += 1;
}
#[no_mangle]
pub unsafe extern "C" fn startDocument(mut ctx: *mut libc::c_void) {
    xmlSAX2StartDocument(ctx);
}
#[no_mangle]
pub unsafe extern "C" fn endDocument(mut ctx: *mut libc::c_void) {
    if deprecated_v1_msg == 0 as i32 {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"Use of deprecated SAXv1 function %s\n\0" as *const u8
                as *const i8,
            b"endDocument\0" as *const u8 as *const i8,
        );
    }
    deprecated_v1_msg += 1;
    xmlSAX2EndDocument(ctx);
}
#[no_mangle]
pub unsafe extern "C" fn attribute(
    mut ctx: *mut libc::c_void,
    mut fullname: *const xmlChar,
    mut value: *const xmlChar,
) {
    if deprecated_v1_msg == 0 as i32 {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"Use of deprecated SAXv1 function %s\n\0" as *const u8
                as *const i8,
            b"attribute\0" as *const u8 as *const i8,
        );
    }
    deprecated_v1_msg += 1;
}
#[no_mangle]
pub unsafe extern "C" fn startElement(
    mut ctx: *mut libc::c_void,
    mut fullname: *const xmlChar,
    mut atts: *mut *const xmlChar,
) {
    xmlSAX2StartElement(ctx, fullname, atts);
}
#[no_mangle]
pub unsafe extern "C" fn endElement(
    mut ctx: *mut libc::c_void,
    mut name: *const xmlChar,
) {
    if deprecated_v1_msg == 0 as i32 {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"Use of deprecated SAXv1 function %s\n\0" as *const u8
                as *const i8,
            b"endElement\0" as *const u8 as *const i8,
        );
    }
    deprecated_v1_msg += 1;
    xmlSAX2EndElement(ctx, name);
}
#[no_mangle]
pub unsafe extern "C" fn reference(
    mut ctx: *mut libc::c_void,
    mut name: *const xmlChar,
) {
    if deprecated_v1_msg == 0 as i32 {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"Use of deprecated SAXv1 function %s\n\0" as *const u8
                as *const i8,
            b"reference\0" as *const u8 as *const i8,
        );
    }
    deprecated_v1_msg += 1;
    xmlSAX2Reference(ctx, name);
}
#[no_mangle]
pub unsafe extern "C" fn characters(
    mut ctx: *mut libc::c_void,
    mut ch: *const xmlChar,
    mut len: i32,
) {
    if deprecated_v1_msg == 0 as i32 {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"Use of deprecated SAXv1 function %s\n\0" as *const u8
                as *const i8,
            b"characters\0" as *const u8 as *const i8,
        );
    }
    deprecated_v1_msg += 1;
    xmlSAX2Characters(ctx, ch, len);
}
#[no_mangle]
pub unsafe extern "C" fn ignorableWhitespace(
    mut ctx: *mut libc::c_void,
    mut ch: *const xmlChar,
    mut len: i32,
) {
    if deprecated_v1_msg == 0 as i32 {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"Use of deprecated SAXv1 function %s\n\0" as *const u8
                as *const i8,
            b"ignorableWhitespace\0" as *const u8 as *const i8,
        );
    }
    deprecated_v1_msg += 1;
}
#[no_mangle]
pub unsafe extern "C" fn processingInstruction(
    mut ctx: *mut libc::c_void,
    mut target: *const xmlChar,
    mut data: *const xmlChar,
) {
    if deprecated_v1_msg == 0 as i32 {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"Use of deprecated SAXv1 function %s\n\0" as *const u8
                as *const i8,
            b"processingInstruction\0" as *const u8 as *const i8,
        );
    }
    deprecated_v1_msg += 1;
    xmlSAX2ProcessingInstruction(ctx, target, data);
}
#[no_mangle]
pub unsafe extern "C" fn globalNamespace(
    mut ctx: *mut libc::c_void,
    mut href: *const xmlChar,
    mut prefix: *const xmlChar,
) {
    if deprecated_v1_msg == 0 as i32 {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"Use of deprecated SAXv1 function %s\n\0" as *const u8
                as *const i8,
            b"globalNamespace\0" as *const u8 as *const i8,
        );
    }
    deprecated_v1_msg += 1;
}
#[no_mangle]
pub unsafe extern "C" fn setNamespace(
    mut ctx: *mut libc::c_void,
    mut name: *const xmlChar,
) {
    if deprecated_v1_msg == 0 as i32 {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"Use of deprecated SAXv1 function %s\n\0" as *const u8
                as *const i8,
            b"setNamespace\0" as *const u8 as *const i8,
        );
    }
    deprecated_v1_msg += 1;
}
#[no_mangle]
pub unsafe extern "C" fn getNamespace(mut ctx: *mut libc::c_void) -> xmlNsPtr {
    if deprecated_v1_msg == 0 as i32 {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"Use of deprecated SAXv1 function %s\n\0" as *const u8
                as *const i8,
            b"getNamespace\0" as *const u8 as *const i8,
        );
    }
    deprecated_v1_msg += 1;
    return 0 as xmlNsPtr;
}
#[no_mangle]
pub unsafe extern "C" fn checkNamespace(
    mut ctx: *mut libc::c_void,
    mut namespace: *mut xmlChar,
) -> i32 {
    if deprecated_v1_msg == 0 as i32 {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"Use of deprecated SAXv1 function %s\n\0" as *const u8
                as *const i8,
            b"checkNamespace\0" as *const u8 as *const i8,
        );
    }
    deprecated_v1_msg += 1;
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn namespaceDecl(
    mut ctx: *mut libc::c_void,
    mut href: *const xmlChar,
    mut prefix: *const xmlChar,
) {
    if deprecated_v1_msg == 0 as i32 {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"Use of deprecated SAXv1 function %s\n\0" as *const u8
                as *const i8,
            b"namespaceDecl\0" as *const u8 as *const i8,
        );
    }
    deprecated_v1_msg += 1;
}
#[no_mangle]
pub unsafe extern "C" fn comment(mut ctx: *mut libc::c_void, mut value: *const xmlChar) {
    if deprecated_v1_msg == 0 as i32 {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"Use of deprecated SAXv1 function %s\n\0" as *const u8
                as *const i8,
            b"comment\0" as *const u8 as *const i8,
        );
    }
    deprecated_v1_msg += 1;
    xmlSAX2Comment(ctx, value);
}
#[no_mangle]
pub unsafe extern "C" fn cdataBlock(
    mut ctx: *mut libc::c_void,
    mut value: *const xmlChar,
    mut len: i32,
) {
    if deprecated_v1_msg == 0 as i32 {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"Use of deprecated SAXv1 function %s\n\0" as *const u8
                as *const i8,
            b"cdataBlock\0" as *const u8 as *const i8,
        );
    }
    deprecated_v1_msg += 1;
    xmlSAX2CDataBlock(ctx, value, len);
}
