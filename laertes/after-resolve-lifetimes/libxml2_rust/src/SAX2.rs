use ::libc;
extern "C" {
    
    
    
    
    
    
    
    
    fn xmlStrcat(cur: * mut u8, add: * const u8) -> * mut u8;
    fn xmlStrlen(str: * const u8) -> i32;
    fn xmlStrEqual(str1: * const u8, str2: * const u8) -> i32;
    fn xmlStrndup(cur: * const u8, len: i32) -> * mut u8;
    fn xmlStrdup(cur: * const u8) -> * mut u8;
    static mut __xmlRegisterCallbacks: i32;
    fn memcpy(
        _: * mut core::ffi::c_void,
        _: * const core::ffi::c_void,
        _: u64,
    ) -> * mut core::ffi::c_void;
    fn memset(
        _: * mut core::ffi::c_void,
        _: i32,
        _: u64,
    ) -> * mut core::ffi::c_void;
    fn xmlValidateNCName(value: * const u8, space: i32) -> i32;
    fn xmlBuildQName(
        ncname: * const u8,
        prefix: * const u8,
        memory: * mut u8,
        len: i32,
    ) -> * mut u8;
    fn xmlCreateIntSubset(
        doc: * mut crate::src::HTMLparser::_xmlDoc,
        name: * const u8,
        ExternalID: * const u8,
        SystemID: * const u8,
    ) -> * mut crate::src::HTMLparser::_xmlDtd;
    fn xmlNewDtd(
        doc: * mut crate::src::HTMLparser::_xmlDoc,
        name: * const u8,
        ExternalID: * const u8,
        SystemID: * const u8,
    ) -> * mut crate::src::HTMLparser::_xmlDtd;
    fn xmlGetIntSubset(doc: * const crate::src::HTMLparser::_xmlDoc) -> * mut crate::src::HTMLparser::_xmlDtd;
    fn xmlFreeDtd(cur: * mut crate::src::HTMLparser::_xmlDtd);
    fn xmlNewNs(
        node: * mut crate::src::HTMLparser::_xmlNode,
        href: * const u8,
        prefix: * const u8,
    ) -> * mut crate::src::HTMLparser::_xmlNs;
    fn xmlNewDoc(version: * const u8) -> * mut crate::src::HTMLparser::_xmlDoc;
    fn xmlNewNsProp(
        node: * mut crate::src::HTMLparser::_xmlNode,
        ns: * mut crate::src::HTMLparser::_xmlNs,
        name: * const u8,
        value: * const u8,
    ) -> * mut crate::src::HTMLparser::_xmlAttr;
    fn xmlNewNsPropEatName(
        node: * mut crate::src::HTMLparser::_xmlNode,
        ns: * mut crate::src::HTMLparser::_xmlNs,
        name: * mut u8,
        value: * const u8,
    ) -> * mut crate::src::HTMLparser::_xmlAttr;
    fn xmlNewDocNode(
        doc: * mut crate::src::HTMLparser::_xmlDoc,
        ns: * mut crate::src::HTMLparser::_xmlNs,
        name: * const u8,
        content: * const u8,
    ) -> * mut crate::src::HTMLparser::_xmlNode;
    fn xmlNewDocNodeEatName(
        doc: * mut crate::src::HTMLparser::_xmlDoc,
        ns: * mut crate::src::HTMLparser::_xmlNs,
        name: * mut u8,
        content: * const u8,
    ) -> * mut crate::src::HTMLparser::_xmlNode;
    fn xmlNewDocText(doc: * const crate::src::HTMLparser::_xmlDoc, content: * const u8) -> * mut crate::src::HTMLparser::_xmlNode;
    fn xmlNewDocPI(
        doc: * mut crate::src::HTMLparser::_xmlDoc,
        name: * const u8,
        content: * const u8,
    ) -> * mut crate::src::HTMLparser::_xmlNode;
    fn xmlNewDocComment(doc: * mut crate::src::HTMLparser::_xmlDoc, content: * const u8) -> * mut crate::src::HTMLparser::_xmlNode;
    fn xmlNewCDataBlock(
        doc: * mut crate::src::HTMLparser::_xmlDoc,
        content: * const u8,
        len: i32,
    ) -> * mut crate::src::HTMLparser::_xmlNode;
    fn xmlNewCharRef(doc: * mut crate::src::HTMLparser::_xmlDoc, name: * const u8) -> * mut crate::src::HTMLparser::_xmlNode;
    fn xmlNewReference(doc: * const crate::src::HTMLparser::_xmlDoc, name: * const u8) -> * mut crate::src::HTMLparser::_xmlNode;
    fn xmlAddChild(parent: * mut crate::src::HTMLparser::_xmlNode, cur: * mut crate::src::HTMLparser::_xmlNode) -> * mut crate::src::HTMLparser::_xmlNode;
    fn xmlAddSibling(cur: * mut crate::src::HTMLparser::_xmlNode, elem: * mut crate::src::HTMLparser::_xmlNode) -> * mut crate::src::HTMLparser::_xmlNode;
    fn xmlUnlinkNode(cur: * mut crate::src::HTMLparser::_xmlNode);
    fn xmlTextConcat(
        node: * mut crate::src::HTMLparser::_xmlNode,
        content: * const u8,
        len: i32,
    ) -> i32;
    fn xmlFreeNode(cur: * mut crate::src::HTMLparser::_xmlNode);
    fn xmlSearchNs(
        doc: * mut crate::src::HTMLparser::_xmlDoc,
        node: * mut crate::src::HTMLparser::_xmlNode,
        nameSpace: * const u8,
    ) -> * mut crate::src::HTMLparser::_xmlNs;
    fn xmlSetNs(node: * mut crate::src::HTMLparser::_xmlNode, ns: * mut crate::src::HTMLparser::_xmlNs);
    fn xmlStringGetNodeList(doc: * const crate::src::HTMLparser::_xmlDoc, value: * const u8) -> * mut crate::src::HTMLparser::_xmlNode;
    fn xmlStringLenGetNodeList(
        doc: * const crate::src::HTMLparser::_xmlDoc,
        value: * const u8,
        len: i32,
    ) -> * mut crate::src::HTMLparser::_xmlNode;
    
    
    
    
    
    
    
    fn xmlAddNotationDecl(
        ctxt: * mut crate::src::HTMLparser::_xmlValidCtxt,
        dtd: * mut crate::src::HTMLparser::_xmlDtd,
        name: * const u8,
        PublicID: * const u8,
        SystemID: * const u8,
    ) -> * mut crate::src::SAX2::_xmlNotation;
    fn xmlAddElementDecl(
        ctxt: * mut crate::src::HTMLparser::_xmlValidCtxt,
        dtd: * mut crate::src::HTMLparser::_xmlDtd,
        name: * const u8,
        type_0: u32,
        content: * mut crate::src::HTMLparser::_xmlElementContent,
    ) -> * mut crate::src::SAX2::_xmlElement;
    fn xmlFreeEnumeration(cur: * mut crate::src::HTMLparser::_xmlEnumeration);
    fn xmlAddAttributeDecl(
        ctxt: * mut crate::src::HTMLparser::_xmlValidCtxt,
        dtd: * mut crate::src::HTMLparser::_xmlDtd,
        elem: * const u8,
        name: * const u8,
        ns: * const u8,
        type_0: u32,
        def: u32,
        defaultValue: * const u8,
        tree: * mut crate::src::HTMLparser::_xmlEnumeration,
    ) -> * mut crate::src::SAX2::_xmlAttribute;
    fn xmlAddID(
        ctxt: * mut crate::src::HTMLparser::_xmlValidCtxt,
        doc: * mut crate::src::HTMLparser::_xmlDoc,
        value: * const u8,
        attr: * mut crate::src::HTMLparser::_xmlAttr,
    ) -> * mut crate::src::SAX2::_xmlID;
    fn xmlIsID(doc: * mut crate::src::HTMLparser::_xmlDoc, elem: * mut crate::src::HTMLparser::_xmlNode, attr: * mut crate::src::HTMLparser::_xmlAttr) -> i32;
    fn xmlAddRef(
        ctxt: * mut crate::src::HTMLparser::_xmlValidCtxt,
        doc: * mut crate::src::HTMLparser::_xmlDoc,
        value: * const u8,
        attr: * mut crate::src::HTMLparser::_xmlAttr,
    ) -> * mut crate::src::SAX2::_xmlRef;
    fn xmlIsRef(doc: * mut crate::src::HTMLparser::_xmlDoc, elem: * mut crate::src::HTMLparser::_xmlNode, attr: * mut crate::src::HTMLparser::_xmlAttr) -> i32;
    fn xmlValidateRoot(ctxt: * mut crate::src::HTMLparser::_xmlValidCtxt, doc: * mut crate::src::HTMLparser::_xmlDoc) -> i32;
    fn xmlValidateElementDecl(
        ctxt: * mut crate::src::HTMLparser::_xmlValidCtxt,
        doc: * mut crate::src::HTMLparser::_xmlDoc,
        elem: * mut crate::src::SAX2::_xmlElement,
    ) -> i32;
    fn xmlValidNormalizeAttributeValue(
        doc: * mut crate::src::HTMLparser::_xmlDoc,
        elem: * mut crate::src::HTMLparser::_xmlNode,
        name: * const u8,
        value: * const u8,
    ) -> * mut u8;
    fn xmlValidCtxtNormalizeAttributeValue(
        ctxt: * mut crate::src::HTMLparser::_xmlValidCtxt,
        doc: * mut crate::src::HTMLparser::_xmlDoc,
        elem: * mut crate::src::HTMLparser::_xmlNode,
        name: * const u8,
        value: * const u8,
    ) -> * mut u8;
    fn xmlValidateAttributeDecl(
        ctxt: * mut crate::src::HTMLparser::_xmlValidCtxt,
        doc: * mut crate::src::HTMLparser::_xmlDoc,
        attr: * mut crate::src::SAX2::_xmlAttribute,
    ) -> i32;
    fn xmlValidateNotationDecl(
        ctxt: * mut crate::src::HTMLparser::_xmlValidCtxt,
        doc: * mut crate::src::HTMLparser::_xmlDoc,
        nota: * mut crate::src::SAX2::_xmlNotation,
    ) -> i32;
    fn xmlValidateDtdFinal(ctxt: * mut crate::src::HTMLparser::_xmlValidCtxt, doc: * mut crate::src::HTMLparser::_xmlDoc) -> i32;
    fn xmlValidateOneElement(
        ctxt: * mut crate::src::HTMLparser::_xmlValidCtxt,
        doc: * mut crate::src::HTMLparser::_xmlDoc,
        elem: * mut crate::src::HTMLparser::_xmlNode,
    ) -> i32;
    fn xmlValidateOneAttribute(
        ctxt: * mut crate::src::HTMLparser::_xmlValidCtxt,
        doc: * mut crate::src::HTMLparser::_xmlDoc,
        elem: * mut crate::src::HTMLparser::_xmlNode,
        attr: * mut crate::src::HTMLparser::_xmlAttr,
        value: * const u8,
    ) -> i32;
    fn xmlValidateOneNamespace(
        ctxt: * mut crate::src::HTMLparser::_xmlValidCtxt,
        doc: * mut crate::src::HTMLparser::_xmlDoc,
        elem: * mut crate::src::HTMLparser::_xmlNode,
        prefix: * const u8,
        ns: * mut crate::src::HTMLparser::_xmlNs,
        value: * const u8,
    ) -> i32;
    fn xmlValidateDocumentFinal(ctxt: * mut crate::src::HTMLparser::_xmlValidCtxt, doc: * mut crate::src::HTMLparser::_xmlDoc) -> i32;
    fn xmlGetDtdQAttrDesc(
        dtd: * mut crate::src::HTMLparser::_xmlDtd,
        elem: * const u8,
        name: * const u8,
        prefix: * const u8,
    ) -> * mut crate::src::SAX2::_xmlAttribute;
    fn xmlGetDtdQElementDesc(
        dtd: * mut crate::src::HTMLparser::_xmlDtd,
        name: * const u8,
        prefix: * const u8,
    ) -> * mut crate::src::SAX2::_xmlElement;
    
    
    fn xmlLoadExternalEntity(
        URL: * const i8,
        ID: * const i8,
        ctxt: * mut crate::src::HTMLparser::_xmlParserCtxt,
    ) -> * mut crate::src::HTMLparser::_xmlParserInput;
    
    
    
    
    
    
    
    
    
    
    
    static xmlStringText: [u8; 0];
    
    
    
    
    
    
    
    
    
    
    
    fn xmlBuildURI(URI: * const u8, base: * const u8) -> * mut u8;
    fn xmlParseURI(str: * const i8) -> * mut crate::src::SAX2::_xmlURI;
    fn xmlFreeURI(uri: * mut crate::src::SAX2::_xmlURI);
    fn xmlCanonicPath(path: * const u8) -> * mut u8;
    fn xmlPathToURI(path: * const u8) -> * mut u8;
    
    
}
pub use crate::src::HTMLparser::htmlNewDocNoDtD;
pub use crate::src::HTMLtree::htmlIsBooleanAttr;
pub use crate::src::dict::xmlDictLookup;
pub use crate::src::dict::xmlDictOwns;
pub use crate::src::dict::xmlDictQLookup;
pub use crate::src::dict::xmlDictReference;
pub use crate::src::encoding::xmlDetectCharEncoding;
pub use crate::src::entities::xmlAddDocEntity;
pub use crate::src::entities::xmlAddDtdEntity;
pub use crate::src::entities::xmlGetDocEntity;
pub use crate::src::entities::xmlGetParameterEntity;
pub use crate::src::entities::xmlGetPredefinedEntity;
pub use crate::src::error::__xmlRaiseError;
pub use crate::src::error::xmlParserError;
pub use crate::src::error::xmlParserWarning;
pub use crate::src::globals::__htmlDefaultSAXHandler;
pub use crate::src::globals::__xmlDefaultSAXHandler;
pub use crate::src::globals::__xmlRegisterNodeDefaultValue;
pub use crate::src::parser::nodePop;
pub use crate::src::parser::nodePush;
pub use crate::src::parser::xmlParseExternalSubset;
pub use crate::src::parser::xmlPopInput;
pub use crate::src::parser::xmlPushInput;
pub use crate::src::parser::xmlSplitQName;
pub use crate::src::parser::xmlStringDecodeEntities;
pub use crate::src::parser::xmlStringLenDecodeEntities;
pub use crate::src::parserInternals::xmlErrMemory;
pub use crate::src::parserInternals::xmlFreeInputStream;
pub use crate::src::parserInternals::xmlParserAddNodeInfo;
pub use crate::src::parserInternals::xmlSwitchEncoding;
pub use crate::src::catalog::_xmlAutomata;
pub use crate::src::debugXML::_xmlRegexp;
pub use crate::src::debugXML::_xmlValidState;
pub use crate::src::encoding::_xmlAutomataState;
pub use crate::src::globals::xmlFree;
pub use crate::src::globals::xmlMalloc;
pub use crate::src::globals::xmlRealloc;
pub use crate::src::buf::_xmlBuf;
pub use crate::src::dict::_xmlDict;
pub use crate::src::hash::_xmlHashTable;
pub use crate::src::parser::_xmlStartTag;
pub type xmlChar = u8;
pub type size_t = u64;
pub type ptrdiff_t = i64;
pub type xmlFreeFunc = Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,) -> ()>;
pub type xmlMallocFunc = Option<unsafe extern "C"  fn(_: u64,) -> * mut core::ffi::c_void>;
pub type xmlReallocFunc = Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,_: u64,) -> * mut core::ffi::c_void>;
// #[derive(Copy, Clone)]

pub type _xmlParserInputBuffer = crate::src::HTMLparser::_xmlParserInputBuffer;
pub type xmlBufPtr = * mut crate::src::buf::_xmlBuf;
pub type xmlBuf = crate::src::buf::_xmlBuf;
pub type xmlCharEncodingHandlerPtr = * mut crate::src::HTMLparser::_xmlCharEncodingHandler;
pub type xmlCharEncodingHandler = crate::src::HTMLparser::_xmlCharEncodingHandler;
// #[derive(Copy, Clone)]

pub type _xmlCharEncodingHandler = crate::src::HTMLparser::_xmlCharEncodingHandler;
pub type iconv_t = * mut core::ffi::c_void;
pub type xmlCharEncodingOutputFunc = Option<unsafe extern "C"  fn(_: * mut u8,_: * mut i32,_: * const u8,_: * mut i32,) -> i32>;
pub type xmlCharEncodingInputFunc = Option<unsafe extern "C"  fn(_: * mut u8,_: * mut i32,_: * const u8,_: * mut i32,) -> i32>;
pub type xmlInputCloseCallback = Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,) -> i32>;
pub type xmlInputReadCallback = Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,_: * mut i8,_: i32,) -> i32>;
pub type xmlParserInputBuffer = crate::src::HTMLparser::_xmlParserInputBuffer;
pub type xmlParserInputBufferPtr = * mut crate::src::HTMLparser::_xmlParserInputBuffer;
// #[derive(Copy, Clone)]

pub type _xmlParserInput = crate::src::HTMLparser::_xmlParserInput;
pub type xmlParserInputDeallocate = Option<unsafe extern "C"  fn(_: * mut u8,) -> ()>;
pub type xmlParserInput = crate::src::HTMLparser::_xmlParserInput;
pub type xmlParserInputPtr = * mut crate::src::HTMLparser::_xmlParserInput;
// #[derive(Copy, Clone)]

pub type _xmlParserCtxt = crate::src::HTMLparser::_xmlParserCtxt;
pub type xmlParserNodeInfo = crate::src::HTMLparser::_xmlParserNodeInfo;
// #[derive(Copy, Clone)]

pub type _xmlParserNodeInfo = crate::src::HTMLparser::_xmlParserNodeInfo;
// #[derive(Copy, Clone)]

pub type _xmlNode = crate::src::HTMLparser::_xmlNode;
pub type xmlNs = crate::src::HTMLparser::_xmlNs;
// #[derive(Copy, Clone)]

pub type _xmlNs = crate::src::HTMLparser::_xmlNs;
// #[derive(Copy, Clone)]

pub type _xmlDoc = crate::src::HTMLparser::_xmlDoc;
// #[derive(Copy, Clone)]

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
// #[derive(Copy, Clone)]

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
pub type xmlParserMode = u32;
pub const XML_PARSE_READER: xmlParserMode = 5;
pub const XML_PARSE_PUSH_SAX: xmlParserMode = 4;
pub const XML_PARSE_PUSH_DOM: xmlParserMode = 3;
pub const XML_PARSE_SAX: xmlParserMode = 2;
pub const XML_PARSE_DOM: xmlParserMode = 1;
pub const XML_PARSE_UNKNOWN: xmlParserMode = 0;
pub type xmlError = crate::src::HTMLparser::_xmlError;
// #[derive(Copy, Clone)]

pub type _xmlError = crate::src::HTMLparser::_xmlError;
pub type xmlErrorLevel = u32;
pub const XML_ERR_FATAL: xmlErrorLevel = 3;
pub const XML_ERR_ERROR: xmlErrorLevel = 2;
pub const XML_ERR_WARNING: xmlErrorLevel = 1;
pub const XML_ERR_NONE: xmlErrorLevel = 0;
pub type xmlAttrPtr = * mut crate::src::HTMLparser::_xmlAttr;
pub type xmlAttr = crate::src::HTMLparser::_xmlAttr;
pub type xmlNodePtr = * mut crate::src::HTMLparser::_xmlNode;
pub type xmlNode = crate::src::HTMLparser::_xmlNode;
pub type xmlHashTablePtr = * mut crate::src::hash::_xmlHashTable;
pub type xmlHashTable = crate::src::hash::_xmlHashTable;
pub type xmlStartTag = crate::src::parser::_xmlStartTag;
pub type xmlDictPtr = * mut crate::src::dict::_xmlDict;
pub type xmlDict = crate::src::dict::_xmlDict;
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
pub type xmlValidCtxt = crate::src::HTMLparser::_xmlValidCtxt;
// #[derive(Copy, Clone)]

pub type _xmlValidCtxt = crate::src::HTMLparser::_xmlValidCtxt;
pub type xmlAutomataStatePtr = * mut crate::src::encoding::_xmlAutomataState;
pub type xmlAutomataState = crate::src::encoding::_xmlAutomataState;
pub type xmlAutomataPtr = * mut crate::src::catalog::_xmlAutomata;
pub type xmlAutomata = crate::src::catalog::_xmlAutomata;
pub type xmlValidState = crate::src::debugXML::_xmlValidState;
pub type xmlDocPtr = * mut crate::src::HTMLparser::_xmlDoc;
pub type xmlDoc = crate::src::HTMLparser::_xmlDoc;
pub type xmlValidityWarningFunc = Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,_: * const i8,...) -> ()>;
pub type xmlValidityErrorFunc = Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,_: * const i8,...) -> ()>;
pub type xmlParserNodeInfoSeq = crate::src::HTMLparser::_xmlParserNodeInfoSeq;
// #[derive(Copy, Clone)]

pub type _xmlParserNodeInfoSeq = crate::src::HTMLparser::_xmlParserNodeInfoSeq;
// #[derive(Copy, Clone)]

pub type _xmlSAXHandler = crate::src::HTMLparser::_xmlSAXHandler;
pub type xmlStructuredErrorFunc = Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,_: * mut crate::src::HTMLparser::_xmlError,) -> ()>;
pub type xmlErrorPtr = * mut crate::src::HTMLparser::_xmlError;
pub type endElementNsSAX2Func = Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,_: * const u8,_: * const u8,_: * const u8,) -> ()>;
pub type startElementNsSAX2Func = Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,_: * const u8,_: * const u8,_: * const u8,_: i32,_: * mut * const u8,_: i32,_: i32,_: * mut * const u8,) -> ()>;
pub type externalSubsetSAXFunc = Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,_: * const u8,_: * const u8,_: * const u8,) -> ()>;
pub type cdataBlockSAXFunc = Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,_: * const u8,_: i32,) -> ()>;
pub type getParameterEntitySAXFunc = Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,_: * const u8,) -> * mut crate::src::HTMLparser::_xmlEntity>;
pub type xmlEntityPtr = * mut crate::src::HTMLparser::_xmlEntity;
pub type xmlEntity = crate::src::HTMLparser::_xmlEntity;
// #[derive(Copy, Clone)]

pub type _xmlEntity = crate::src::HTMLparser::_xmlEntity;
pub type xmlEntityType = u32;
pub const XML_INTERNAL_PREDEFINED_ENTITY: xmlEntityType = 6;
pub const XML_EXTERNAL_PARAMETER_ENTITY: xmlEntityType = 5;
pub const XML_INTERNAL_PARAMETER_ENTITY: xmlEntityType = 4;
pub const XML_EXTERNAL_GENERAL_UNPARSED_ENTITY: xmlEntityType = 3;
pub const XML_EXTERNAL_GENERAL_PARSED_ENTITY: xmlEntityType = 2;
pub const XML_INTERNAL_GENERAL_ENTITY: xmlEntityType = 1;
pub type fatalErrorSAXFunc = Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,_: * const i8,...) -> ()>;
pub type errorSAXFunc = Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,_: * const i8,...) -> ()>;
pub type warningSAXFunc = Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,_: * const i8,...) -> ()>;
pub type commentSAXFunc = Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,_: * const u8,) -> ()>;
pub type processingInstructionSAXFunc = Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,_: * const u8,_: * const u8,) -> ()>;
pub type ignorableWhitespaceSAXFunc = Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,_: * const u8,_: i32,) -> ()>;
pub type charactersSAXFunc = Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,_: * const u8,_: i32,) -> ()>;
pub type referenceSAXFunc = Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,_: * const u8,) -> ()>;
pub type endElementSAXFunc = Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,_: * const u8,) -> ()>;
pub type startElementSAXFunc = Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,_: * const u8,_: * mut * const u8,) -> ()>;
pub type endDocumentSAXFunc = Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,) -> ()>;
pub type startDocumentSAXFunc = Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,) -> ()>;
pub type setDocumentLocatorSAXFunc = Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,_: * mut crate::src::HTMLparser::_xmlSAXLocator,) -> ()>;
pub type xmlSAXLocatorPtr = * mut crate::src::HTMLparser::_xmlSAXLocator;
pub type xmlSAXLocator = crate::src::HTMLparser::_xmlSAXLocator;
// #[derive(Copy, Clone)]

pub type _xmlSAXLocator = crate::src::HTMLparser::_xmlSAXLocator;
pub type unparsedEntityDeclSAXFunc = Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,_: * const u8,_: * const u8,_: * const u8,_: * const u8,) -> ()>;
pub type elementDeclSAXFunc = Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,_: * const u8,_: i32,_: * mut crate::src::HTMLparser::_xmlElementContent,) -> ()>;
pub type xmlElementContentPtr = * mut crate::src::HTMLparser::_xmlElementContent;
pub type xmlElementContent = crate::src::HTMLparser::_xmlElementContent;
// #[derive(Copy, Clone)]

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
pub type attributeDeclSAXFunc = Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,_: * const u8,_: * const u8,_: i32,_: i32,_: * const u8,_: * mut crate::src::HTMLparser::_xmlEnumeration,) -> ()>;
pub type xmlEnumerationPtr = * mut crate::src::HTMLparser::_xmlEnumeration;
pub type xmlEnumeration = crate::src::HTMLparser::_xmlEnumeration;
// #[derive(Copy, Clone)]

pub type _xmlEnumeration = crate::src::HTMLparser::_xmlEnumeration;
pub type notationDeclSAXFunc = Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,_: * const u8,_: * const u8,_: * const u8,) -> ()>;
pub type entityDeclSAXFunc = Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,_: * const u8,_: i32,_: * const u8,_: * const u8,_: * mut u8,) -> ()>;
pub type getEntitySAXFunc = Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,_: * const u8,) -> * mut crate::src::HTMLparser::_xmlEntity>;
pub type resolveEntitySAXFunc = Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,_: * const u8,_: * const u8,) -> * mut crate::src::HTMLparser::_xmlParserInput>;
pub type hasExternalSubsetSAXFunc = Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,) -> i32>;
pub type hasInternalSubsetSAXFunc = Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,) -> i32>;
pub type isStandaloneSAXFunc = Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,) -> i32>;
pub type internalSubsetSAXFunc = Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,_: * const u8,_: * const u8,_: * const u8,) -> ()>;
pub type xmlParserCtxt = crate::src::HTMLparser::_xmlParserCtxt;
pub type xmlParserCtxtPtr = * mut crate::src::HTMLparser::_xmlParserCtxt;
pub type xmlSAXHandler = crate::src::HTMLparser::_xmlSAXHandler;
pub type xmlSAXHandlerPtr = * mut crate::src::HTMLparser::_xmlSAXHandler;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlNotation {
    pub name: * const u8,
    pub PublicID: * const u8,
    pub SystemID: * const u8,
}
impl _xmlNotation {
    pub const fn new() -> Self {
        _xmlNotation {
        name: (0 as * const u8),
        PublicID: (0 as * const u8),
        SystemID: (0 as * const u8)
        }
    }
}

impl std::default::Default for _xmlNotation {
    fn default() -> Self { _xmlNotation::new() }
}

pub type xmlNotation = crate::src::SAX2::_xmlNotation;
pub type xmlNotationPtr = * mut crate::src::SAX2::_xmlNotation;
pub type xmlAttributeDefault = u32;
pub const XML_ATTRIBUTE_FIXED: xmlAttributeDefault = 4;
pub const XML_ATTRIBUTE_IMPLIED: xmlAttributeDefault = 3;
pub const XML_ATTRIBUTE_REQUIRED: xmlAttributeDefault = 2;
pub const XML_ATTRIBUTE_NONE: xmlAttributeDefault = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlAttribute {
    pub _private: * mut core::ffi::c_void,
    pub type_0: u32,
    pub name: * const u8,
    pub children: * mut crate::src::HTMLparser::_xmlNode,
    pub last: * mut crate::src::HTMLparser::_xmlNode,
    pub parent: * mut crate::src::HTMLparser::_xmlDtd,
    pub next: * mut crate::src::HTMLparser::_xmlNode,
    pub prev: * mut crate::src::HTMLparser::_xmlNode,
    pub doc: * mut crate::src::HTMLparser::_xmlDoc,
    pub nexth: * mut crate::src::SAX2::_xmlAttribute,
    pub atype: u32,
    pub def: u32,
    pub defaultValue: * const u8,
    pub tree: * mut crate::src::HTMLparser::_xmlEnumeration,
    pub prefix: * const u8,
    pub elem: * const u8,
}
impl _xmlAttribute {
    pub const fn new() -> Self {
        _xmlAttribute {
        _private: (0 as * mut core::ffi::c_void),
        type_0: 0,
        name: (0 as * const u8),
        children: (0 as * mut crate::src::HTMLparser::_xmlNode),
        last: (0 as * mut crate::src::HTMLparser::_xmlNode),
        parent: (0 as * mut crate::src::HTMLparser::_xmlDtd),
        next: (0 as * mut crate::src::HTMLparser::_xmlNode),
        prev: (0 as * mut crate::src::HTMLparser::_xmlNode),
        doc: (0 as * mut crate::src::HTMLparser::_xmlDoc),
        nexth: (0 as * mut crate::src::SAX2::_xmlAttribute),
        atype: 0,
        def: 0,
        defaultValue: (0 as * const u8),
        tree: (0 as * mut crate::src::HTMLparser::_xmlEnumeration),
        prefix: (0 as * const u8),
        elem: (0 as * const u8)
        }
    }
}

impl std::default::Default for _xmlAttribute {
    fn default() -> Self { _xmlAttribute::new() }
}

pub type xmlAttribute = crate::src::SAX2::_xmlAttribute;
pub type xmlAttributePtr = * mut crate::src::SAX2::_xmlAttribute;
pub type xmlElementTypeVal = u32;
pub const XML_ELEMENT_TYPE_ELEMENT: xmlElementTypeVal = 4;
pub const XML_ELEMENT_TYPE_MIXED: xmlElementTypeVal = 3;
pub const XML_ELEMENT_TYPE_ANY: xmlElementTypeVal = 2;
pub const XML_ELEMENT_TYPE_EMPTY: xmlElementTypeVal = 1;
pub const XML_ELEMENT_TYPE_UNDEFINED: xmlElementTypeVal = 0;
pub type xmlRegexp = crate::src::debugXML::_xmlRegexp;
pub type xmlRegexpPtr = * mut crate::src::debugXML::_xmlRegexp;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlElement {
    pub _private: * mut core::ffi::c_void,
    pub type_0: u32,
    pub name: * const u8,
    pub children: * mut crate::src::HTMLparser::_xmlNode,
    pub last: * mut crate::src::HTMLparser::_xmlNode,
    pub parent: * mut crate::src::HTMLparser::_xmlDtd,
    pub next: * mut crate::src::HTMLparser::_xmlNode,
    pub prev: * mut crate::src::HTMLparser::_xmlNode,
    pub doc: * mut crate::src::HTMLparser::_xmlDoc,
    pub etype: u32,
    pub content: * mut crate::src::HTMLparser::_xmlElementContent,
    pub attributes: * mut crate::src::SAX2::_xmlAttribute,
    pub prefix: * const u8,
    pub contModel: * mut crate::src::debugXML::_xmlRegexp,
}
impl _xmlElement {
    pub const fn new() -> Self {
        _xmlElement {
        _private: (0 as * mut core::ffi::c_void),
        type_0: 0,
        name: (0 as * const u8),
        children: (0 as * mut crate::src::HTMLparser::_xmlNode),
        last: (0 as * mut crate::src::HTMLparser::_xmlNode),
        parent: (0 as * mut crate::src::HTMLparser::_xmlDtd),
        next: (0 as * mut crate::src::HTMLparser::_xmlNode),
        prev: (0 as * mut crate::src::HTMLparser::_xmlNode),
        doc: (0 as * mut crate::src::HTMLparser::_xmlDoc),
        etype: 0,
        content: (0 as * mut crate::src::HTMLparser::_xmlElementContent),
        attributes: (0 as * mut crate::src::SAX2::_xmlAttribute),
        prefix: (0 as * const u8),
        contModel: (0 as * mut crate::src::debugXML::_xmlRegexp)
        }
    }
}

impl std::default::Default for _xmlElement {
    fn default() -> Self { _xmlElement::new() }
}

pub type xmlElement = crate::src::SAX2::_xmlElement;
pub type xmlElementPtr = * mut crate::src::SAX2::_xmlElement;
pub type xmlNsPtr = * mut crate::src::HTMLparser::_xmlNs;
pub type xmlDtd = crate::src::HTMLparser::_xmlDtd;
pub type xmlDtdPtr = * mut crate::src::HTMLparser::_xmlDtd;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlID {
    pub next: * mut crate::src::SAX2::_xmlID,
    pub value: * const u8,
    pub attr: * mut crate::src::HTMLparser::_xmlAttr,
    pub name: * const u8,
    pub lineno: i32,
    pub doc: * mut crate::src::HTMLparser::_xmlDoc,
}
impl _xmlID {
    pub const fn new() -> Self {
        _xmlID {
        next: (0 as * mut crate::src::SAX2::_xmlID),
        value: (0 as * const u8),
        attr: (0 as * mut crate::src::HTMLparser::_xmlAttr),
        name: (0 as * const u8),
        lineno: 0,
        doc: (0 as * mut crate::src::HTMLparser::_xmlDoc)
        }
    }
}

impl std::default::Default for _xmlID {
    fn default() -> Self { _xmlID::new() }
}

pub type xmlID = crate::src::SAX2::_xmlID;
pub type xmlIDPtr = * mut crate::src::SAX2::_xmlID;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlRef {
    pub next: * mut crate::src::SAX2::_xmlRef,
    pub value: * const u8,
    pub attr: * mut crate::src::HTMLparser::_xmlAttr,
    pub name: * const u8,
    pub lineno: i32,
}
impl _xmlRef {
    pub const fn new() -> Self {
        _xmlRef {
        next: (0 as * mut crate::src::SAX2::_xmlRef),
        value: (0 as * const u8),
        attr: (0 as * mut crate::src::HTMLparser::_xmlAttr),
        name: (0 as * const u8),
        lineno: 0
        }
    }
}

impl std::default::Default for _xmlRef {
    fn default() -> Self { _xmlRef::new() }
}

pub type xmlRef = crate::src::SAX2::_xmlRef;
pub type xmlRefPtr = * mut crate::src::SAX2::_xmlRef;
pub type C2RustUnnamed = u32;
pub const XML_DOC_HTML: C2RustUnnamed = 128;
pub const XML_DOC_INTERNAL: C2RustUnnamed = 64;
pub const XML_DOC_USERBUILT: C2RustUnnamed = 32;
pub const XML_DOC_XINCLUDE: C2RustUnnamed = 16;
pub const XML_DOC_DTDVALID: C2RustUnnamed = 8;
pub const XML_DOC_OLD10: C2RustUnnamed = 4;
pub const XML_DOC_NSVALID: C2RustUnnamed = 2;
pub const XML_DOC_WELLFORMED: C2RustUnnamed = 1;
pub type C2RustUnnamed_0 = u32;
pub const XML_FROM_URI: C2RustUnnamed_0 = 30;
pub const XML_FROM_BUFFER: C2RustUnnamed_0 = 29;
pub const XML_FROM_SCHEMATRONV: C2RustUnnamed_0 = 28;
pub const XML_FROM_I18N: C2RustUnnamed_0 = 27;
pub const XML_FROM_MODULE: C2RustUnnamed_0 = 26;
pub const XML_FROM_WRITER: C2RustUnnamed_0 = 25;
pub const XML_FROM_CHECK: C2RustUnnamed_0 = 24;
pub const XML_FROM_VALID: C2RustUnnamed_0 = 23;
pub const XML_FROM_XSLT: C2RustUnnamed_0 = 22;
pub const XML_FROM_C14N: C2RustUnnamed_0 = 21;
pub const XML_FROM_CATALOG: C2RustUnnamed_0 = 20;
pub const XML_FROM_RELAXNGV: C2RustUnnamed_0 = 19;
pub const XML_FROM_RELAXNGP: C2RustUnnamed_0 = 18;
pub const XML_FROM_SCHEMASV: C2RustUnnamed_0 = 17;
pub const XML_FROM_SCHEMASP: C2RustUnnamed_0 = 16;
pub const XML_FROM_DATATYPE: C2RustUnnamed_0 = 15;
pub const XML_FROM_REGEXP: C2RustUnnamed_0 = 14;
pub const XML_FROM_XPOINTER: C2RustUnnamed_0 = 13;
pub const XML_FROM_XPATH: C2RustUnnamed_0 = 12;
pub const XML_FROM_XINCLUDE: C2RustUnnamed_0 = 11;
pub const XML_FROM_HTTP: C2RustUnnamed_0 = 10;
pub const XML_FROM_FTP: C2RustUnnamed_0 = 9;
pub const XML_FROM_IO: C2RustUnnamed_0 = 8;
pub const XML_FROM_OUTPUT: C2RustUnnamed_0 = 7;
pub const XML_FROM_MEMORY: C2RustUnnamed_0 = 6;
pub const XML_FROM_HTML: C2RustUnnamed_0 = 5;
pub const XML_FROM_DTD: C2RustUnnamed_0 = 4;
pub const XML_FROM_NAMESPACE: C2RustUnnamed_0 = 3;
pub const XML_FROM_TREE: C2RustUnnamed_0 = 2;
pub const XML_FROM_PARSER: C2RustUnnamed_0 = 1;
pub const XML_FROM_NONE: C2RustUnnamed_0 = 0;
pub type xmlParserErrors = u32;
pub const XML_BUF_OVERFLOW: xmlParserErrors = 7000;
pub const XML_I18N_NO_OUTPUT: xmlParserErrors = 6004;
pub const XML_I18N_CONV_FAILED: xmlParserErrors = 6003;
pub const XML_I18N_EXCESS_HANDLER: xmlParserErrors = 6002;
pub const XML_I18N_NO_HANDLER: xmlParserErrors = 6001;
pub const XML_I18N_NO_NAME: xmlParserErrors = 6000;
pub const XML_CHECK_NAME_NOT_NULL: xmlParserErrors = 5037;
pub const XML_CHECK_WRONG_NAME: xmlParserErrors = 5036;
pub const XML_CHECK_OUTSIDE_DICT: xmlParserErrors = 5035;
pub const XML_CHECK_NOT_NCNAME: xmlParserErrors = 5034;
pub const XML_CHECK_NO_DICT: xmlParserErrors = 5033;
pub const XML_CHECK_NOT_UTF8: xmlParserErrors = 5032;
pub const XML_CHECK_NS_ANCESTOR: xmlParserErrors = 5031;
pub const XML_CHECK_NS_SCOPE: xmlParserErrors = 5030;
pub const XML_CHECK_WRONG_PARENT: xmlParserErrors = 5029;
pub const XML_CHECK_NO_HREF: xmlParserErrors = 5028;
pub const XML_CHECK_NOT_NS_DECL: xmlParserErrors = 5027;
pub const XML_CHECK_NOT_ENTITY_DECL: xmlParserErrors = 5026;
pub const XML_CHECK_NOT_ELEM_DECL: xmlParserErrors = 5025;
pub const XML_CHECK_NOT_ATTR_DECL: xmlParserErrors = 5024;
pub const XML_CHECK_NOT_ATTR: xmlParserErrors = 5023;
pub const XML_CHECK_NOT_DTD: xmlParserErrors = 5022;
pub const XML_CHECK_WRONG_NEXT: xmlParserErrors = 5021;
pub const XML_CHECK_NO_NEXT: xmlParserErrors = 5020;
pub const XML_CHECK_WRONG_PREV: xmlParserErrors = 5019;
pub const XML_CHECK_NO_PREV: xmlParserErrors = 5018;
pub const XML_CHECK_WRONG_DOC: xmlParserErrors = 5017;
pub const XML_CHECK_NO_ELEM: xmlParserErrors = 5016;
pub const XML_CHECK_NO_NAME: xmlParserErrors = 5015;
pub const XML_CHECK_NO_DOC: xmlParserErrors = 5014;
pub const XML_CHECK_NO_PARENT: xmlParserErrors = 5013;
pub const XML_CHECK_ENTITY_TYPE: xmlParserErrors = 5012;
pub const XML_CHECK_UNKNOWN_NODE: xmlParserErrors = 5011;
pub const XML_CHECK_FOUND_NOTATION: xmlParserErrors = 5010;
pub const XML_CHECK_FOUND_FRAGMENT: xmlParserErrors = 5009;
pub const XML_CHECK_FOUND_DOCTYPE: xmlParserErrors = 5008;
pub const XML_CHECK_FOUND_COMMENT: xmlParserErrors = 5007;
pub const XML_CHECK_FOUND_PI: xmlParserErrors = 5006;
pub const XML_CHECK_FOUND_ENTITY: xmlParserErrors = 5005;
pub const XML_CHECK_FOUND_ENTITYREF: xmlParserErrors = 5004;
pub const XML_CHECK_FOUND_CDATA: xmlParserErrors = 5003;
pub const XML_CHECK_FOUND_TEXT: xmlParserErrors = 5002;
pub const XML_CHECK_FOUND_ATTRIBUTE: xmlParserErrors = 5001;
pub const XML_CHECK_FOUND_ELEMENT: xmlParserErrors = 5000;
pub const XML_MODULE_CLOSE: xmlParserErrors = 4901;
pub const XML_MODULE_OPEN: xmlParserErrors = 4900;
pub const XML_SCHEMATRONV_REPORT: xmlParserErrors = 4001;
pub const XML_SCHEMATRONV_ASSERT: xmlParserErrors = 4000;
pub const XML_SCHEMAP_COS_ALL_LIMITED: xmlParserErrors = 3091;
pub const XML_SCHEMAP_A_PROPS_CORRECT_3: xmlParserErrors = 3090;
pub const XML_SCHEMAP_AU_PROPS_CORRECT: xmlParserErrors = 3089;
pub const XML_SCHEMAP_COS_CT_EXTENDS_1_2: xmlParserErrors = 3088;
pub const XML_SCHEMAP_AG_PROPS_CORRECT: xmlParserErrors = 3087;
pub const XML_SCHEMAP_WARN_ATTR_POINTLESS_PROH: xmlParserErrors = 3086;
pub const XML_SCHEMAP_WARN_ATTR_REDECL_PROH: xmlParserErrors = 3085;
pub const XML_SCHEMAP_WARN_UNLOCATED_SCHEMA: xmlParserErrors = 3084;
pub const XML_SCHEMAP_WARN_SKIP_SCHEMA: xmlParserErrors = 3083;
pub const XML_SCHEMAP_SRC_IMPORT: xmlParserErrors = 3082;
pub const XML_SCHEMAP_SRC_REDEFINE: xmlParserErrors = 3081;
pub const XML_SCHEMAP_C_PROPS_CORRECT: xmlParserErrors = 3080;
pub const XML_SCHEMAP_A_PROPS_CORRECT_2: xmlParserErrors = 3079;
pub const XML_SCHEMAP_AU_PROPS_CORRECT_2: xmlParserErrors = 3078;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_3: xmlParserErrors = 3077;
pub const XML_SCHEMAP_SRC_CT_1: xmlParserErrors = 3076;
pub const XML_SCHEMAP_MG_PROPS_CORRECT_2: xmlParserErrors = 3075;
pub const XML_SCHEMAP_MG_PROPS_CORRECT_1: xmlParserErrors = 3074;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_3: xmlParserErrors = 3073;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_2: xmlParserErrors = 3072;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_1: xmlParserErrors = 3071;
pub const XML_SCHEMAP_NOT_DETERMINISTIC: xmlParserErrors = 3070;
pub const XML_SCHEMAP_INTERNAL: xmlParserErrors = 3069;
pub const XML_SCHEMAP_SRC_IMPORT_2_2: xmlParserErrors = 3068;
pub const XML_SCHEMAP_SRC_IMPORT_2_1: xmlParserErrors = 3067;
pub const XML_SCHEMAP_SRC_IMPORT_2: xmlParserErrors = 3066;
pub const XML_SCHEMAP_SRC_IMPORT_1_2: xmlParserErrors = 3065;
pub const XML_SCHEMAP_SRC_IMPORT_1_1: xmlParserErrors = 3064;
pub const XML_SCHEMAP_COS_CT_EXTENDS_1_1: xmlParserErrors = 3063;
pub const XML_SCHEMAP_CVC_SIMPLE_TYPE: xmlParserErrors = 3062;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_2_2_2: xmlParserErrors = 3061;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_2_2_1: xmlParserErrors = 3060;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_2_1: xmlParserErrors = 3059;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_1: xmlParserErrors = 3058;
pub const XML_SCHEMAP_NO_XSI: xmlParserErrors = 3057;
pub const XML_SCHEMAP_NO_XMLNS: xmlParserErrors = 3056;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_4: xmlParserErrors = 3055;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_3_2: xmlParserErrors = 3054;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_3_1: xmlParserErrors = 3053;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_2: xmlParserErrors = 3052;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_1: xmlParserErrors = 3051;
pub const XML_SCHEMAP_SRC_INCLUDE: xmlParserErrors = 3050;
pub const XML_SCHEMAP_E_PROPS_CORRECT_6: xmlParserErrors = 3049;
pub const XML_SCHEMAP_E_PROPS_CORRECT_5: xmlParserErrors = 3048;
pub const XML_SCHEMAP_E_PROPS_CORRECT_4: xmlParserErrors = 3047;
pub const XML_SCHEMAP_E_PROPS_CORRECT_3: xmlParserErrors = 3046;
pub const XML_SCHEMAP_E_PROPS_CORRECT_2: xmlParserErrors = 3045;
pub const XML_SCHEMAP_P_PROPS_CORRECT_2_2: xmlParserErrors = 3044;
pub const XML_SCHEMAP_P_PROPS_CORRECT_2_1: xmlParserErrors = 3043;
pub const XML_SCHEMAP_P_PROPS_CORRECT_1: xmlParserErrors = 3042;
pub const XML_SCHEMAP_SRC_ELEMENT_3: xmlParserErrors = 3041;
pub const XML_SCHEMAP_SRC_ELEMENT_2_2: xmlParserErrors = 3040;
pub const XML_SCHEMAP_SRC_ELEMENT_2_1: xmlParserErrors = 3039;
pub const XML_SCHEMAP_SRC_ELEMENT_1: xmlParserErrors = 3038;
pub const XML_SCHEMAP_S4S_ATTR_INVALID_VALUE: xmlParserErrors = 3037;
pub const XML_SCHEMAP_S4S_ATTR_MISSING: xmlParserErrors = 3036;
pub const XML_SCHEMAP_S4S_ATTR_NOT_ALLOWED: xmlParserErrors = 3035;
pub const XML_SCHEMAP_S4S_ELEM_MISSING: xmlParserErrors = 3034;
pub const XML_SCHEMAP_S4S_ELEM_NOT_ALLOWED: xmlParserErrors = 3033;
pub const XML_SCHEMAP_COS_ST_DERIVED_OK_2_2: xmlParserErrors = 3032;
pub const XML_SCHEMAP_COS_ST_DERIVED_OK_2_1: xmlParserErrors = 3031;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_5: xmlParserErrors = 3030;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_4: xmlParserErrors = 3029;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_3: xmlParserErrors = 3028;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_1: xmlParserErrors = 3027;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_2: xmlParserErrors = 3026;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_1_2: xmlParserErrors = 3025;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_1: xmlParserErrors = 3024;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_1: xmlParserErrors = 3023;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_5: xmlParserErrors = 3022;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_4: xmlParserErrors = 3021;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_3: xmlParserErrors = 3020;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_2: xmlParserErrors = 3019;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_1: xmlParserErrors = 3018;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_1_2: xmlParserErrors = 3017;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_1_1: xmlParserErrors = 3016;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_1: xmlParserErrors = 3015;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_3_2: xmlParserErrors = 3014;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_3_1: xmlParserErrors = 3013;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_2: xmlParserErrors = 3012;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_1: xmlParserErrors = 3011;
pub const XML_SCHEMAP_ST_PROPS_CORRECT_3: xmlParserErrors = 3010;
pub const XML_SCHEMAP_ST_PROPS_CORRECT_2: xmlParserErrors = 3009;
pub const XML_SCHEMAP_ST_PROPS_CORRECT_1: xmlParserErrors = 3008;
pub const XML_SCHEMAP_SRC_UNION_MEMBERTYPES_OR_SIMPLETYPES: xmlParserErrors = 3007;
pub const XML_SCHEMAP_SRC_LIST_ITEMTYPE_OR_SIMPLETYPE: xmlParserErrors = 3006;
pub const XML_SCHEMAP_SRC_RESTRICTION_BASE_OR_SIMPLETYPE: xmlParserErrors = 3005;
pub const XML_SCHEMAP_SRC_RESOLVE: xmlParserErrors = 3004;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_4: xmlParserErrors = 3003;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_3: xmlParserErrors = 3002;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_2: xmlParserErrors = 3001;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_1: xmlParserErrors = 3000;
pub const XML_HTTP_UNKNOWN_HOST: xmlParserErrors = 2022;
pub const XML_HTTP_USE_IP: xmlParserErrors = 2021;
pub const XML_HTTP_URL_SYNTAX: xmlParserErrors = 2020;
pub const XML_FTP_URL_SYNTAX: xmlParserErrors = 2003;
pub const XML_FTP_ACCNT: xmlParserErrors = 2002;
pub const XML_FTP_EPSV_ANSWER: xmlParserErrors = 2001;
pub const XML_FTP_PASV_ANSWER: xmlParserErrors = 2000;
pub const XML_C14N_RELATIVE_NAMESPACE: xmlParserErrors = 1955;
pub const XML_C14N_UNKNOW_NODE: xmlParserErrors = 1954;
pub const XML_C14N_INVALID_NODE: xmlParserErrors = 1953;
pub const XML_C14N_CREATE_STACK: xmlParserErrors = 1952;
pub const XML_C14N_REQUIRES_UTF8: xmlParserErrors = 1951;
pub const XML_C14N_CREATE_CTXT: xmlParserErrors = 1950;
pub const XML_XPTR_EXTRA_OBJECTS: xmlParserErrors = 1903;
pub const XML_XPTR_EVAL_FAILED: xmlParserErrors = 1902;
pub const XML_XPTR_CHILDSEQ_START: xmlParserErrors = 1901;
pub const XML_XPTR_UNKNOWN_SCHEME: xmlParserErrors = 1900;
pub const XML_SCHEMAV_MISC: xmlParserErrors = 1879;
pub const XML_SCHEMAV_CVC_WILDCARD: xmlParserErrors = 1878;
pub const XML_SCHEMAV_CVC_IDC: xmlParserErrors = 1877;
pub const XML_SCHEMAV_CVC_TYPE_2: xmlParserErrors = 1876;
pub const XML_SCHEMAV_CVC_TYPE_1: xmlParserErrors = 1875;
pub const XML_SCHEMAV_CVC_AU: xmlParserErrors = 1874;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_1: xmlParserErrors = 1873;
pub const XML_SCHEMAV_DOCUMENT_ELEMENT_MISSING: xmlParserErrors = 1872;
pub const XML_SCHEMAV_ELEMENT_CONTENT: xmlParserErrors = 1871;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_5_2: xmlParserErrors = 1870;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_5_1: xmlParserErrors = 1869;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_4: xmlParserErrors = 1868;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_3_2_2: xmlParserErrors = 1867;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_3_2_1: xmlParserErrors = 1866;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_3_1: xmlParserErrors = 1865;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_4: xmlParserErrors = 1864;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_3: xmlParserErrors = 1863;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_2: xmlParserErrors = 1862;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_1: xmlParserErrors = 1861;
pub const XML_SCHEMAV_CVC_ELT_7: xmlParserErrors = 1860;
pub const XML_SCHEMAV_CVC_ELT_6: xmlParserErrors = 1859;
pub const XML_SCHEMAV_CVC_ELT_5_2_2_2_2: xmlParserErrors = 1858;
pub const XML_SCHEMAV_CVC_ELT_5_2_2_2_1: xmlParserErrors = 1857;
pub const XML_SCHEMAV_CVC_ELT_5_2_2_1: xmlParserErrors = 1856;
pub const XML_SCHEMAV_CVC_ELT_5_2_1: xmlParserErrors = 1855;
pub const XML_SCHEMAV_CVC_ELT_5_1_2: xmlParserErrors = 1854;
pub const XML_SCHEMAV_CVC_ELT_5_1_1: xmlParserErrors = 1853;
pub const XML_SCHEMAV_CVC_ELT_4_3: xmlParserErrors = 1852;
pub const XML_SCHEMAV_CVC_ELT_4_2: xmlParserErrors = 1851;
pub const XML_SCHEMAV_CVC_ELT_4_1: xmlParserErrors = 1850;
pub const XML_SCHEMAV_CVC_ELT_3_2_2: xmlParserErrors = 1849;
pub const XML_SCHEMAV_CVC_ELT_3_2_1: xmlParserErrors = 1848;
pub const XML_SCHEMAV_CVC_ELT_3_1: xmlParserErrors = 1847;
pub const XML_SCHEMAV_CVC_ELT_2: xmlParserErrors = 1846;
pub const XML_SCHEMAV_CVC_ELT_1: xmlParserErrors = 1845;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_4: xmlParserErrors = 1844;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_3: xmlParserErrors = 1843;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_2: xmlParserErrors = 1842;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_1: xmlParserErrors = 1841;
pub const XML_SCHEMAV_CVC_ENUMERATION_VALID: xmlParserErrors = 1840;
pub const XML_SCHEMAV_CVC_PATTERN_VALID: xmlParserErrors = 1839;
pub const XML_SCHEMAV_CVC_FRACTIONDIGITS_VALID: xmlParserErrors = 1838;
pub const XML_SCHEMAV_CVC_TOTALDIGITS_VALID: xmlParserErrors = 1837;
pub const XML_SCHEMAV_CVC_MAXEXCLUSIVE_VALID: xmlParserErrors = 1836;
pub const XML_SCHEMAV_CVC_MINEXCLUSIVE_VALID: xmlParserErrors = 1835;
pub const XML_SCHEMAV_CVC_MAXINCLUSIVE_VALID: xmlParserErrors = 1834;
pub const XML_SCHEMAV_CVC_MININCLUSIVE_VALID: xmlParserErrors = 1833;
pub const XML_SCHEMAV_CVC_MAXLENGTH_VALID: xmlParserErrors = 1832;
pub const XML_SCHEMAV_CVC_MINLENGTH_VALID: xmlParserErrors = 1831;
pub const XML_SCHEMAV_CVC_LENGTH_VALID: xmlParserErrors = 1830;
pub const XML_SCHEMAV_CVC_FACET_VALID: xmlParserErrors = 1829;
pub const XML_SCHEMAV_CVC_TYPE_3_1_2: xmlParserErrors = 1828;
pub const XML_SCHEMAV_CVC_TYPE_3_1_1: xmlParserErrors = 1827;
pub const XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_3: xmlParserErrors = 1826;
pub const XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_2: xmlParserErrors = 1825;
pub const XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_1: xmlParserErrors = 1824;
pub const XML_SCHEMAV_FACET: xmlParserErrors = 1823;
pub const XML_SCHEMAV_VALUE: xmlParserErrors = 1822;
pub const XML_SCHEMAV_ATTRINVALID: xmlParserErrors = 1821;
pub const XML_SCHEMAV_ATTRUNKNOWN: xmlParserErrors = 1820;
pub const XML_SCHEMAV_NOTSIMPLE: xmlParserErrors = 1819;
pub const XML_SCHEMAV_INTERNAL: xmlParserErrors = 1818;
pub const XML_SCHEMAV_CONSTRUCT: xmlParserErrors = 1817;
pub const XML_SCHEMAV_NOTDETERMINIST: xmlParserErrors = 1816;
pub const XML_SCHEMAV_INVALIDELEM: xmlParserErrors = 1815;
pub const XML_SCHEMAV_INVALIDATTR: xmlParserErrors = 1814;
pub const XML_SCHEMAV_EXTRACONTENT: xmlParserErrors = 1813;
pub const XML_SCHEMAV_NOTNILLABLE: xmlParserErrors = 1812;
pub const XML_SCHEMAV_HAVEDEFAULT: xmlParserErrors = 1811;
pub const XML_SCHEMAV_ELEMCONT: xmlParserErrors = 1810;
pub const XML_SCHEMAV_NOTEMPTY: xmlParserErrors = 1809;
pub const XML_SCHEMAV_ISABSTRACT: xmlParserErrors = 1808;
pub const XML_SCHEMAV_NOROLLBACK: xmlParserErrors = 1807;
pub const XML_SCHEMAV_NOTYPE: xmlParserErrors = 1806;
pub const XML_SCHEMAV_WRONGELEM: xmlParserErrors = 1805;
pub const XML_SCHEMAV_MISSING: xmlParserErrors = 1804;
pub const XML_SCHEMAV_NOTTOPLEVEL: xmlParserErrors = 1803;
pub const XML_SCHEMAV_UNDECLAREDELEM: xmlParserErrors = 1802;
pub const XML_SCHEMAV_NOROOT: xmlParserErrors = 1801;
pub const XML_SCHEMAP_COS_CT_EXTENDS_1_3: xmlParserErrors = 1800;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_3: xmlParserErrors = 1799;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_2: xmlParserErrors = 1798;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_1: xmlParserErrors = 1797;
pub const XML_SCHEMAP_SRC_IMPORT_3_2: xmlParserErrors = 1796;
pub const XML_SCHEMAP_SRC_IMPORT_3_1: xmlParserErrors = 1795;
pub const XML_SCHEMAP_UNION_NOT_EXPRESSIBLE: xmlParserErrors = 1794;
pub const XML_SCHEMAP_INTERSECTION_NOT_EXPRESSIBLE: xmlParserErrors = 1793;
pub const XML_SCHEMAP_WILDCARD_INVALID_NS_MEMBER: xmlParserErrors = 1792;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_3: xmlParserErrors = 1791;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_2: xmlParserErrors = 1790;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_2: xmlParserErrors = 1789;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_1: xmlParserErrors = 1788;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_1: xmlParserErrors = 1787;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_5: xmlParserErrors = 1786;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_4: xmlParserErrors = 1785;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_3: xmlParserErrors = 1784;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_2: xmlParserErrors = 1783;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_1: xmlParserErrors = 1782;
pub const XML_SCHEMAP_REF_AND_CONTENT: xmlParserErrors = 1781;
pub const XML_SCHEMAP_INVALID_ATTR_NAME: xmlParserErrors = 1780;
pub const XML_SCHEMAP_MISSING_SIMPLETYPE_CHILD: xmlParserErrors = 1779;
pub const XML_SCHEMAP_INVALID_ATTR_INLINE_COMBINATION: xmlParserErrors = 1778;
pub const XML_SCHEMAP_INVALID_ATTR_COMBINATION: xmlParserErrors = 1777;
pub const XML_SCHEMAP_SUPERNUMEROUS_LIST_ITEM_TYPE: xmlParserErrors = 1776;
pub const XML_SCHEMAP_RECURSIVE: xmlParserErrors = 1775;
pub const XML_SCHEMAP_INVALID_ATTR_USE: xmlParserErrors = 1774;
pub const XML_SCHEMAP_UNKNOWN_MEMBER_TYPE: xmlParserErrors = 1773;
pub const XML_SCHEMAP_NOT_SCHEMA: xmlParserErrors = 1772;
pub const XML_SCHEMAP_INCLUDE_SCHEMA_NO_URI: xmlParserErrors = 1771;
pub const XML_SCHEMAP_INCLUDE_SCHEMA_NOT_URI: xmlParserErrors = 1770;
pub const XML_SCHEMAP_UNKNOWN_INCLUDE_CHILD: xmlParserErrors = 1769;
pub const XML_SCHEMAP_DEF_AND_PREFIX: xmlParserErrors = 1768;
pub const XML_SCHEMAP_UNKNOWN_PREFIX: xmlParserErrors = 1767;
pub const XML_SCHEMAP_FAILED_PARSE: xmlParserErrors = 1766;
pub const XML_SCHEMAP_REDEFINED_NOTATION: xmlParserErrors = 1765;
pub const XML_SCHEMAP_REDEFINED_ATTR: xmlParserErrors = 1764;
pub const XML_SCHEMAP_REDEFINED_ATTRGROUP: xmlParserErrors = 1763;
pub const XML_SCHEMAP_REDEFINED_ELEMENT: xmlParserErrors = 1762;
pub const XML_SCHEMAP_REDEFINED_TYPE: xmlParserErrors = 1761;
pub const XML_SCHEMAP_REDEFINED_GROUP: xmlParserErrors = 1760;
pub const XML_SCHEMAP_NOROOT: xmlParserErrors = 1759;
pub const XML_SCHEMAP_NOTHING_TO_PARSE: xmlParserErrors = 1758;
pub const XML_SCHEMAP_FAILED_LOAD: xmlParserErrors = 1757;
pub const XML_SCHEMAP_REGEXP_INVALID: xmlParserErrors = 1756;
pub const XML_SCHEMAP_ELEM_DEFAULT_FIXED: xmlParserErrors = 1755;
pub const XML_SCHEMAP_UNKNOWN_UNION_CHILD: xmlParserErrors = 1754;
pub const XML_SCHEMAP_UNKNOWN_TYPE: xmlParserErrors = 1753;
pub const XML_SCHEMAP_UNKNOWN_SIMPLETYPE_CHILD: xmlParserErrors = 1752;
pub const XML_SCHEMAP_UNKNOWN_SIMPLECONTENT_CHILD: xmlParserErrors = 1751;
pub const XML_SCHEMAP_UNKNOWN_SEQUENCE_CHILD: xmlParserErrors = 1750;
pub const XML_SCHEMAP_UNKNOWN_SCHEMAS_CHILD: xmlParserErrors = 1749;
pub const XML_SCHEMAP_UNKNOWN_RESTRICTION_CHILD: xmlParserErrors = 1748;
pub const XML_SCHEMAP_UNKNOWN_REF: xmlParserErrors = 1747;
pub const XML_SCHEMAP_UNKNOWN_PROCESSCONTENT_CHILD: xmlParserErrors = 1746;
pub const XML_SCHEMAP_UNKNOWN_NOTATION_CHILD: xmlParserErrors = 1745;
pub const XML_SCHEMAP_UNKNOWN_LIST_CHILD: xmlParserErrors = 1744;
pub const XML_SCHEMAP_UNKNOWN_IMPORT_CHILD: xmlParserErrors = 1743;
pub const XML_SCHEMAP_UNKNOWN_GROUP_CHILD: xmlParserErrors = 1742;
pub const XML_SCHEMAP_UNKNOWN_FACET_TYPE: xmlParserErrors = 1741;
pub const XML_SCHEMAP_UNKNOWN_FACET_CHILD: xmlParserErrors = 1740;
pub const XML_SCHEMAP_UNKNOWN_EXTENSION_CHILD: xmlParserErrors = 1739;
pub const XML_SCHEMAP_UNKNOWN_ELEM_CHILD: xmlParserErrors = 1738;
pub const XML_SCHEMAP_UNKNOWN_COMPLEXTYPE_CHILD: xmlParserErrors = 1737;
pub const XML_SCHEMAP_UNKNOWN_COMPLEXCONTENT_CHILD: xmlParserErrors = 1736;
pub const XML_SCHEMAP_UNKNOWN_CHOICE_CHILD: xmlParserErrors = 1735;
pub const XML_SCHEMAP_UNKNOWN_BASE_TYPE: xmlParserErrors = 1734;
pub const XML_SCHEMAP_UNKNOWN_ATTRIBUTE_GROUP: xmlParserErrors = 1733;
pub const XML_SCHEMAP_UNKNOWN_ATTRGRP_CHILD: xmlParserErrors = 1732;
pub const XML_SCHEMAP_UNKNOWN_ATTR_CHILD: xmlParserErrors = 1731;
pub const XML_SCHEMAP_UNKNOWN_ANYATTRIBUTE_CHILD: xmlParserErrors = 1730;
pub const XML_SCHEMAP_UNKNOWN_ALL_CHILD: xmlParserErrors = 1729;
pub const XML_SCHEMAP_TYPE_AND_SUBTYPE: xmlParserErrors = 1728;
pub const XML_SCHEMAP_SIMPLETYPE_NONAME: xmlParserErrors = 1727;
pub const XML_SCHEMAP_RESTRICTION_NONAME_NOREF: xmlParserErrors = 1726;
pub const XML_SCHEMAP_REF_AND_SUBTYPE: xmlParserErrors = 1725;
pub const XML_SCHEMAP_NOTYPE_NOREF: xmlParserErrors = 1724;
pub const XML_SCHEMAP_NOTATION_NO_NAME: xmlParserErrors = 1723;
pub const XML_SCHEMAP_NOATTR_NOREF: xmlParserErrors = 1722;
pub const XML_SCHEMAP_INVALID_WHITE_SPACE: xmlParserErrors = 1721;
pub const XML_SCHEMAP_INVALID_REF_AND_SUBTYPE: xmlParserErrors = 1720;
pub const XML_SCHEMAP_INVALID_MINOCCURS: xmlParserErrors = 1719;
pub const XML_SCHEMAP_INVALID_MAXOCCURS: xmlParserErrors = 1718;
pub const XML_SCHEMAP_INVALID_FACET_VALUE: xmlParserErrors = 1717;
pub const XML_SCHEMAP_INVALID_FACET: xmlParserErrors = 1716;
pub const XML_SCHEMAP_INVALID_ENUM: xmlParserErrors = 1715;
pub const XML_SCHEMAP_INVALID_BOOLEAN: xmlParserErrors = 1714;
pub const XML_SCHEMAP_IMPORT_SCHEMA_NOT_URI: xmlParserErrors = 1713;
pub const XML_SCHEMAP_IMPORT_REDEFINE_NSNAME: xmlParserErrors = 1712;
pub const XML_SCHEMAP_IMPORT_NAMESPACE_NOT_URI: xmlParserErrors = 1711;
pub const XML_SCHEMAP_GROUP_NONAME_NOREF: xmlParserErrors = 1710;
pub const XML_SCHEMAP_FAILED_BUILD_IMPORT: xmlParserErrors = 1709;
pub const XML_SCHEMAP_FACET_NO_VALUE: xmlParserErrors = 1708;
pub const XML_SCHEMAP_EXTENSION_NO_BASE: xmlParserErrors = 1707;
pub const XML_SCHEMAP_ELEM_NONAME_NOREF: xmlParserErrors = 1706;
pub const XML_SCHEMAP_ELEMFORMDEFAULT_VALUE: xmlParserErrors = 1705;
pub const XML_SCHEMAP_COMPLEXTYPE_NONAME_NOREF: xmlParserErrors = 1704;
pub const XML_SCHEMAP_ATTR_NONAME_NOREF: xmlParserErrors = 1703;
pub const XML_SCHEMAP_ATTRGRP_NONAME_NOREF: xmlParserErrors = 1702;
pub const XML_SCHEMAP_ATTRFORMDEFAULT_VALUE: xmlParserErrors = 1701;
pub const XML_SCHEMAP_PREFIX_UNDEFINED: xmlParserErrors = 1700;
pub const XML_CATALOG_RECURSION: xmlParserErrors = 1654;
pub const XML_CATALOG_NOT_CATALOG: xmlParserErrors = 1653;
pub const XML_CATALOG_PREFER_VALUE: xmlParserErrors = 1652;
pub const XML_CATALOG_ENTRY_BROKEN: xmlParserErrors = 1651;
pub const XML_CATALOG_MISSING_ATTR: xmlParserErrors = 1650;
pub const XML_XINCLUDE_FRAGMENT_ID: xmlParserErrors = 1618;
pub const XML_XINCLUDE_DEPRECATED_NS: xmlParserErrors = 1617;
pub const XML_XINCLUDE_FALLBACK_NOT_IN_INCLUDE: xmlParserErrors = 1616;
pub const XML_XINCLUDE_FALLBACKS_IN_INCLUDE: xmlParserErrors = 1615;
pub const XML_XINCLUDE_INCLUDE_IN_INCLUDE: xmlParserErrors = 1614;
pub const XML_XINCLUDE_XPTR_RESULT: xmlParserErrors = 1613;
pub const XML_XINCLUDE_XPTR_FAILED: xmlParserErrors = 1612;
pub const XML_XINCLUDE_MULTIPLE_ROOT: xmlParserErrors = 1611;
pub const XML_XINCLUDE_UNKNOWN_ENCODING: xmlParserErrors = 1610;
pub const XML_XINCLUDE_BUILD_FAILED: xmlParserErrors = 1609;
pub const XML_XINCLUDE_INVALID_CHAR: xmlParserErrors = 1608;
pub const XML_XINCLUDE_TEXT_DOCUMENT: xmlParserErrors = 1607;
pub const XML_XINCLUDE_TEXT_FRAGMENT: xmlParserErrors = 1606;
pub const XML_XINCLUDE_HREF_URI: xmlParserErrors = 1605;
pub const XML_XINCLUDE_NO_FALLBACK: xmlParserErrors = 1604;
pub const XML_XINCLUDE_NO_HREF: xmlParserErrors = 1603;
pub const XML_XINCLUDE_ENTITY_DEF_MISMATCH: xmlParserErrors = 1602;
pub const XML_XINCLUDE_PARSE_VALUE: xmlParserErrors = 1601;
pub const XML_XINCLUDE_RECURSION: xmlParserErrors = 1600;
pub const XML_IO_EAFNOSUPPORT: xmlParserErrors = 1556;
pub const XML_IO_EALREADY: xmlParserErrors = 1555;
pub const XML_IO_EADDRINUSE: xmlParserErrors = 1554;
pub const XML_IO_ENETUNREACH: xmlParserErrors = 1553;
pub const XML_IO_ECONNREFUSED: xmlParserErrors = 1552;
pub const XML_IO_EISCONN: xmlParserErrors = 1551;
pub const XML_IO_ENOTSOCK: xmlParserErrors = 1550;
pub const XML_IO_LOAD_ERROR: xmlParserErrors = 1549;
pub const XML_IO_BUFFER_FULL: xmlParserErrors = 1548;
pub const XML_IO_NO_INPUT: xmlParserErrors = 1547;
pub const XML_IO_WRITE: xmlParserErrors = 1546;
pub const XML_IO_FLUSH: xmlParserErrors = 1545;
pub const XML_IO_ENCODER: xmlParserErrors = 1544;
pub const XML_IO_NETWORK_ATTEMPT: xmlParserErrors = 1543;
pub const XML_IO_EXDEV: xmlParserErrors = 1542;
pub const XML_IO_ETIMEDOUT: xmlParserErrors = 1541;
pub const XML_IO_ESRCH: xmlParserErrors = 1540;
pub const XML_IO_ESPIPE: xmlParserErrors = 1539;
pub const XML_IO_EROFS: xmlParserErrors = 1538;
pub const XML_IO_ERANGE: xmlParserErrors = 1537;
pub const XML_IO_EPIPE: xmlParserErrors = 1536;
pub const XML_IO_EPERM: xmlParserErrors = 1535;
pub const XML_IO_ENXIO: xmlParserErrors = 1534;
pub const XML_IO_ENOTTY: xmlParserErrors = 1533;
pub const XML_IO_ENOTSUP: xmlParserErrors = 1532;
pub const XML_IO_ENOTEMPTY: xmlParserErrors = 1531;
pub const XML_IO_ENOTDIR: xmlParserErrors = 1530;
pub const XML_IO_ENOSYS: xmlParserErrors = 1529;
pub const XML_IO_ENOSPC: xmlParserErrors = 1528;
pub const XML_IO_ENOMEM: xmlParserErrors = 1527;
pub const XML_IO_ENOLCK: xmlParserErrors = 1526;
pub const XML_IO_ENOEXEC: xmlParserErrors = 1525;
pub const XML_IO_ENOENT: xmlParserErrors = 1524;
pub const XML_IO_ENODEV: xmlParserErrors = 1523;
pub const XML_IO_ENFILE: xmlParserErrors = 1522;
pub const XML_IO_ENAMETOOLONG: xmlParserErrors = 1521;
pub const XML_IO_EMSGSIZE: xmlParserErrors = 1520;
pub const XML_IO_EMLINK: xmlParserErrors = 1519;
pub const XML_IO_EMFILE: xmlParserErrors = 1518;
pub const XML_IO_EISDIR: xmlParserErrors = 1517;
pub const XML_IO_EIO: xmlParserErrors = 1516;
pub const XML_IO_EINVAL: xmlParserErrors = 1515;
pub const XML_IO_EINTR: xmlParserErrors = 1514;
pub const XML_IO_EINPROGRESS: xmlParserErrors = 1513;
pub const XML_IO_EFBIG: xmlParserErrors = 1512;
pub const XML_IO_EFAULT: xmlParserErrors = 1511;
pub const XML_IO_EEXIST: xmlParserErrors = 1510;
pub const XML_IO_EDOM: xmlParserErrors = 1509;
pub const XML_IO_EDEADLK: xmlParserErrors = 1508;
pub const XML_IO_ECHILD: xmlParserErrors = 1507;
pub const XML_IO_ECANCELED: xmlParserErrors = 1506;
pub const XML_IO_EBUSY: xmlParserErrors = 1505;
pub const XML_IO_EBADMSG: xmlParserErrors = 1504;
pub const XML_IO_EBADF: xmlParserErrors = 1503;
pub const XML_IO_EAGAIN: xmlParserErrors = 1502;
pub const XML_IO_EACCES: xmlParserErrors = 1501;
pub const XML_IO_UNKNOWN: xmlParserErrors = 1500;
pub const XML_REGEXP_COMPILE_ERROR: xmlParserErrors = 1450;
pub const XML_SAVE_UNKNOWN_ENCODING: xmlParserErrors = 1403;
pub const XML_SAVE_NO_DOCTYPE: xmlParserErrors = 1402;
pub const XML_SAVE_CHAR_INVALID: xmlParserErrors = 1401;
pub const XML_SAVE_NOT_UTF8: xmlParserErrors = 1400;
pub const XML_TREE_NOT_UTF8: xmlParserErrors = 1303;
pub const XML_TREE_UNTERMINATED_ENTITY: xmlParserErrors = 1302;
pub const XML_TREE_INVALID_DEC: xmlParserErrors = 1301;
pub const XML_TREE_INVALID_HEX: xmlParserErrors = 1300;
pub const XML_XPATH_INVALID_CHAR_ERROR: xmlParserErrors = 1221;
pub const XML_XPATH_ENCODING_ERROR: xmlParserErrors = 1220;
pub const XML_XPATH_UNDEF_PREFIX_ERROR: xmlParserErrors = 1219;
pub const XML_XPTR_SUB_RESOURCE_ERROR: xmlParserErrors = 1218;
pub const XML_XPTR_RESOURCE_ERROR: xmlParserErrors = 1217;
pub const XML_XPTR_SYNTAX_ERROR: xmlParserErrors = 1216;
pub const XML_XPATH_MEMORY_ERROR: xmlParserErrors = 1215;
pub const XML_XPATH_INVALID_CTXT_POSITION: xmlParserErrors = 1214;
pub const XML_XPATH_INVALID_CTXT_SIZE: xmlParserErrors = 1213;
pub const XML_XPATH_INVALID_ARITY: xmlParserErrors = 1212;
pub const XML_XPATH_INVALID_TYPE: xmlParserErrors = 1211;
pub const XML_XPATH_INVALID_OPERAND: xmlParserErrors = 1210;
pub const XML_XPATH_UNKNOWN_FUNC_ERROR: xmlParserErrors = 1209;
pub const XML_XPATH_UNCLOSED_ERROR: xmlParserErrors = 1208;
pub const XML_XPATH_EXPR_ERROR: xmlParserErrors = 1207;
pub const XML_XPATH_INVALID_PREDICATE_ERROR: xmlParserErrors = 1206;
pub const XML_XPATH_UNDEF_VARIABLE_ERROR: xmlParserErrors = 1205;
pub const XML_XPATH_VARIABLE_REF_ERROR: xmlParserErrors = 1204;
pub const XML_XPATH_START_LITERAL_ERROR: xmlParserErrors = 1203;
pub const XML_XPATH_UNFINISHED_LITERAL_ERROR: xmlParserErrors = 1202;
pub const XML_XPATH_NUMBER_ERROR: xmlParserErrors = 1201;
pub const XML_XPATH_EXPRESSION_OK: xmlParserErrors = 1200;
pub const XML_RNGP_XML_NS: xmlParserErrors = 1122;
pub const XML_RNGP_XMLNS_NAME: xmlParserErrors = 1121;
pub const XML_RNGP_VALUE_NO_CONTENT: xmlParserErrors = 1120;
pub const XML_RNGP_VALUE_EMPTY: xmlParserErrors = 1119;
pub const XML_RNGP_URI_NOT_ABSOLUTE: xmlParserErrors = 1118;
pub const XML_RNGP_URI_FRAGMENT: xmlParserErrors = 1117;
pub const XML_RNGP_UNKNOWN_TYPE_LIB: xmlParserErrors = 1116;
pub const XML_RNGP_UNKNOWN_CONSTRUCT: xmlParserErrors = 1115;
pub const XML_RNGP_UNKNOWN_COMBINE: xmlParserErrors = 1114;
pub const XML_RNGP_UNKNOWN_ATTRIBUTE: xmlParserErrors = 1113;
pub const XML_RNGP_TYPE_VALUE: xmlParserErrors = 1112;
pub const XML_RNGP_TYPE_NOT_FOUND: xmlParserErrors = 1111;
pub const XML_RNGP_TYPE_MISSING: xmlParserErrors = 1110;
pub const XML_RNGP_TEXT_HAS_CHILD: xmlParserErrors = 1109;
pub const XML_RNGP_TEXT_EXPECTED: xmlParserErrors = 1108;
pub const XML_RNGP_START_MISSING: xmlParserErrors = 1107;
pub const XML_RNGP_START_EMPTY: xmlParserErrors = 1106;
pub const XML_RNGP_START_CONTENT: xmlParserErrors = 1105;
pub const XML_RNGP_START_CHOICE_AND_INTERLEAVE: xmlParserErrors = 1104;
pub const XML_RNGP_REF_NOT_EMPTY: xmlParserErrors = 1103;
pub const XML_RNGP_REF_NO_NAME: xmlParserErrors = 1102;
pub const XML_RNGP_REF_NO_DEF: xmlParserErrors = 1101;
pub const XML_RNGP_REF_NAME_INVALID: xmlParserErrors = 1100;
pub const XML_RNGP_REF_CYCLE: xmlParserErrors = 1099;
pub const XML_RNGP_REF_CREATE_FAILED: xmlParserErrors = 1098;
pub const XML_RNGP_PREFIX_UNDEFINED: xmlParserErrors = 1097;
pub const XML_RNGP_PAT_START_VALUE: xmlParserErrors = 1096;
pub const XML_RNGP_PAT_START_TEXT: xmlParserErrors = 1095;
pub const XML_RNGP_PAT_START_ONEMORE: xmlParserErrors = 1094;
pub const XML_RNGP_PAT_START_LIST: xmlParserErrors = 1093;
pub const XML_RNGP_PAT_START_INTERLEAVE: xmlParserErrors = 1092;
pub const XML_RNGP_PAT_START_GROUP: xmlParserErrors = 1091;
pub const XML_RNGP_PAT_START_EMPTY: xmlParserErrors = 1090;
pub const XML_RNGP_PAT_START_DATA: xmlParserErrors = 1089;
pub const XML_RNGP_PAT_START_ATTR: xmlParserErrors = 1088;
pub const XML_RNGP_PAT_ONEMORE_INTERLEAVE_ATTR: xmlParserErrors = 1087;
pub const XML_RNGP_PAT_ONEMORE_GROUP_ATTR: xmlParserErrors = 1086;
pub const XML_RNGP_PAT_NSNAME_EXCEPT_NSNAME: xmlParserErrors = 1085;
pub const XML_RNGP_PAT_NSNAME_EXCEPT_ANYNAME: xmlParserErrors = 1084;
pub const XML_RNGP_PAT_LIST_TEXT: xmlParserErrors = 1083;
pub const XML_RNGP_PAT_LIST_REF: xmlParserErrors = 1082;
pub const XML_RNGP_PAT_LIST_LIST: xmlParserErrors = 1081;
pub const XML_RNGP_PAT_LIST_INTERLEAVE: xmlParserErrors = 1080;
pub const XML_RNGP_PAT_LIST_ELEM: xmlParserErrors = 1079;
pub const XML_RNGP_PAT_LIST_ATTR: xmlParserErrors = 1078;
pub const XML_RNGP_PAT_DATA_EXCEPT_TEXT: xmlParserErrors = 1077;
pub const XML_RNGP_PAT_DATA_EXCEPT_REF: xmlParserErrors = 1076;
pub const XML_RNGP_PAT_DATA_EXCEPT_ONEMORE: xmlParserErrors = 1075;
pub const XML_RNGP_PAT_DATA_EXCEPT_LIST: xmlParserErrors = 1074;
pub const XML_RNGP_PAT_DATA_EXCEPT_INTERLEAVE: xmlParserErrors = 1073;
pub const XML_RNGP_PAT_DATA_EXCEPT_GROUP: xmlParserErrors = 1072;
pub const XML_RNGP_PAT_DATA_EXCEPT_EMPTY: xmlParserErrors = 1071;
pub const XML_RNGP_PAT_DATA_EXCEPT_ELEM: xmlParserErrors = 1070;
pub const XML_RNGP_PAT_DATA_EXCEPT_ATTR: xmlParserErrors = 1069;
pub const XML_RNGP_PAT_ATTR_ELEM: xmlParserErrors = 1068;
pub const XML_RNGP_PAT_ATTR_ATTR: xmlParserErrors = 1067;
pub const XML_RNGP_PAT_ANYNAME_EXCEPT_ANYNAME: xmlParserErrors = 1066;
pub const XML_RNGP_PARSE_ERROR: xmlParserErrors = 1065;
pub const XML_RNGP_PARENTREF_NOT_EMPTY: xmlParserErrors = 1064;
pub const XML_RNGP_PARENTREF_NO_PARENT: xmlParserErrors = 1063;
pub const XML_RNGP_PARENTREF_NO_NAME: xmlParserErrors = 1062;
pub const XML_RNGP_PARENTREF_NAME_INVALID: xmlParserErrors = 1061;
pub const XML_RNGP_PARENTREF_CREATE_FAILED: xmlParserErrors = 1060;
pub const XML_RNGP_PARAM_NAME_MISSING: xmlParserErrors = 1059;
pub const XML_RNGP_PARAM_FORBIDDEN: xmlParserErrors = 1058;
pub const XML_RNGP_NSNAME_NO_NS: xmlParserErrors = 1057;
pub const XML_RNGP_NSNAME_ATTR_ANCESTOR: xmlParserErrors = 1056;
pub const XML_RNGP_NOTALLOWED_NOT_EMPTY: xmlParserErrors = 1055;
pub const XML_RNGP_NEED_COMBINE: xmlParserErrors = 1054;
pub const XML_RNGP_NAME_MISSING: xmlParserErrors = 1053;
pub const XML_RNGP_MISSING_HREF: xmlParserErrors = 1052;
pub const XML_RNGP_INVALID_VALUE: xmlParserErrors = 1051;
pub const XML_RNGP_INVALID_URI: xmlParserErrors = 1050;
pub const XML_RNGP_INVALID_DEFINE_NAME: xmlParserErrors = 1049;
pub const XML_RNGP_INTERLEAVE_NO_CONTENT: xmlParserErrors = 1048;
pub const XML_RNGP_INTERLEAVE_EMPTY: xmlParserErrors = 1047;
pub const XML_RNGP_INTERLEAVE_CREATE_FAILED: xmlParserErrors = 1046;
pub const XML_RNGP_INTERLEAVE_ADD: xmlParserErrors = 1045;
pub const XML_RNGP_INCLUDE_RECURSE: xmlParserErrors = 1044;
pub const XML_RNGP_INCLUDE_FAILURE: xmlParserErrors = 1043;
pub const XML_RNGP_INCLUDE_EMPTY: xmlParserErrors = 1042;
pub const XML_RNGP_HREF_ERROR: xmlParserErrors = 1041;
pub const XML_RNGP_GROUP_ATTR_CONFLICT: xmlParserErrors = 1040;
pub const XML_RNGP_GRAMMAR_NO_START: xmlParserErrors = 1039;
pub const XML_RNGP_GRAMMAR_MISSING: xmlParserErrors = 1038;
pub const XML_RNGP_GRAMMAR_EMPTY: xmlParserErrors = 1037;
pub const XML_RNGP_GRAMMAR_CONTENT: xmlParserErrors = 1036;
pub const XML_RNGP_FOREIGN_ELEMENT: xmlParserErrors = 1035;
pub const XML_RNGP_FORBIDDEN_ATTRIBUTE: xmlParserErrors = 1034;
pub const XML_RNGP_EXTERNALREF_RECURSE: xmlParserErrors = 1033;
pub const XML_RNGP_EXTERNAL_REF_FAILURE: xmlParserErrors = 1032;
pub const XML_RNGP_EXTERNALREF_EMTPY: xmlParserErrors = 1031;
pub const XML_RNGP_EXCEPT_NO_CONTENT: xmlParserErrors = 1030;
pub const XML_RNGP_EXCEPT_MULTIPLE: xmlParserErrors = 1029;
pub const XML_RNGP_EXCEPT_MISSING: xmlParserErrors = 1028;
pub const XML_RNGP_EXCEPT_EMPTY: xmlParserErrors = 1027;
pub const XML_RNGP_ERROR_TYPE_LIB: xmlParserErrors = 1026;
pub const XML_RNGP_EMPTY_NOT_EMPTY: xmlParserErrors = 1025;
pub const XML_RNGP_EMPTY_CONTENT: xmlParserErrors = 1024;
pub const XML_RNGP_EMPTY_CONSTRUCT: xmlParserErrors = 1023;
pub const XML_RNGP_EMPTY: xmlParserErrors = 1022;
pub const XML_RNGP_ELEM_TEXT_CONFLICT: xmlParserErrors = 1021;
pub const XML_RNGP_ELEMENT_NO_CONTENT: xmlParserErrors = 1020;
pub const XML_RNGP_ELEMENT_NAME: xmlParserErrors = 1019;
pub const XML_RNGP_ELEMENT_CONTENT: xmlParserErrors = 1018;
pub const XML_RNGP_ELEMENT_EMPTY: xmlParserErrors = 1017;
pub const XML_RNGP_ELEM_CONTENT_ERROR: xmlParserErrors = 1016;
pub const XML_RNGP_ELEM_CONTENT_EMPTY: xmlParserErrors = 1015;
pub const XML_RNGP_DEFINE_NAME_MISSING: xmlParserErrors = 1014;
pub const XML_RNGP_DEFINE_MISSING: xmlParserErrors = 1013;
pub const XML_RNGP_DEFINE_EMPTY: xmlParserErrors = 1012;
pub const XML_RNGP_DEFINE_CREATE_FAILED: xmlParserErrors = 1011;
pub const XML_RNGP_DEF_CHOICE_AND_INTERLEAVE: xmlParserErrors = 1010;
pub const XML_RNGP_DATA_CONTENT: xmlParserErrors = 1009;
pub const XML_RNGP_CREATE_FAILURE: xmlParserErrors = 1008;
pub const XML_RNGP_CHOICE_EMPTY: xmlParserErrors = 1007;
pub const XML_RNGP_CHOICE_CONTENT: xmlParserErrors = 1006;
pub const XML_RNGP_ATTRIBUTE_NOOP: xmlParserErrors = 1005;
pub const XML_RNGP_ATTRIBUTE_EMPTY: xmlParserErrors = 1004;
pub const XML_RNGP_ATTRIBUTE_CONTENT: xmlParserErrors = 1003;
pub const XML_RNGP_ATTRIBUTE_CHILDREN: xmlParserErrors = 1002;
pub const XML_RNGP_ATTR_CONFLICT: xmlParserErrors = 1001;
pub const XML_RNGP_ANYNAME_ATTR_ANCESTOR: xmlParserErrors = 1000;
pub const XML_HTML_INCORRECTLY_OPENED_COMMENT: xmlParserErrors = 802;
pub const XML_HTML_UNKNOWN_TAG: xmlParserErrors = 801;
pub const XML_HTML_STRUCURE_ERROR: xmlParserErrors = 800;
pub const XML_DTD_DUP_TOKEN: xmlParserErrors = 541;
pub const XML_DTD_XMLID_TYPE: xmlParserErrors = 540;
pub const XML_DTD_XMLID_VALUE: xmlParserErrors = 539;
pub const XML_DTD_STANDALONE_DEFAULTED: xmlParserErrors = 538;
pub const XML_DTD_UNKNOWN_NOTATION: xmlParserErrors = 537;
pub const XML_DTD_UNKNOWN_ID: xmlParserErrors = 536;
pub const XML_DTD_UNKNOWN_ENTITY: xmlParserErrors = 535;
pub const XML_DTD_UNKNOWN_ELEM: xmlParserErrors = 534;
pub const XML_DTD_UNKNOWN_ATTRIBUTE: xmlParserErrors = 533;
pub const XML_DTD_STANDALONE_WHITE_SPACE: xmlParserErrors = 532;
pub const XML_DTD_ROOT_NAME: xmlParserErrors = 531;
pub const XML_DTD_NOT_STANDALONE: xmlParserErrors = 530;
pub const XML_DTD_NOT_PCDATA: xmlParserErrors = 529;
pub const XML_DTD_NOT_EMPTY: xmlParserErrors = 528;
pub const XML_DTD_NOTATION_VALUE: xmlParserErrors = 527;
pub const XML_DTD_NOTATION_REDEFINED: xmlParserErrors = 526;
pub const XML_DTD_NO_ROOT: xmlParserErrors = 525;
pub const XML_DTD_NO_PREFIX: xmlParserErrors = 524;
pub const XML_DTD_NO_ELEM_NAME: xmlParserErrors = 523;
pub const XML_DTD_NO_DTD: xmlParserErrors = 522;
pub const XML_DTD_NO_DOC: xmlParserErrors = 521;
pub const XML_DTD_MULTIPLE_ID: xmlParserErrors = 520;
pub const XML_DTD_MIXED_CORRUPT: xmlParserErrors = 519;
pub const XML_DTD_MISSING_ATTRIBUTE: xmlParserErrors = 518;
pub const XML_DTD_LOAD_ERROR: xmlParserErrors = 517;
pub const XML_DTD_INVALID_DEFAULT: xmlParserErrors = 516;
pub const XML_DTD_INVALID_CHILD: xmlParserErrors = 515;
pub const XML_DTD_ID_SUBSET: xmlParserErrors = 514;
pub const XML_DTD_ID_REDEFINED: xmlParserErrors = 513;
pub const XML_DTD_ID_FIXED: xmlParserErrors = 512;
pub const XML_DTD_ENTITY_TYPE: xmlParserErrors = 511;
pub const XML_DTD_EMPTY_NOTATION: xmlParserErrors = 510;
pub const XML_DTD_ELEM_REDEFINED: xmlParserErrors = 509;
pub const XML_DTD_ELEM_NAMESPACE: xmlParserErrors = 508;
pub const XML_DTD_ELEM_DEFAULT_NAMESPACE: xmlParserErrors = 507;
pub const XML_DTD_DIFFERENT_PREFIX: xmlParserErrors = 506;
pub const XML_DTD_CONTENT_NOT_DETERMINIST: xmlParserErrors = 505;
pub const XML_DTD_CONTENT_MODEL: xmlParserErrors = 504;
pub const XML_DTD_CONTENT_ERROR: xmlParserErrors = 503;
pub const XML_DTD_ATTRIBUTE_VALUE: xmlParserErrors = 502;
pub const XML_DTD_ATTRIBUTE_REDEFINED: xmlParserErrors = 501;
pub const XML_DTD_ATTRIBUTE_DEFAULT: xmlParserErrors = 500;
pub const XML_NS_ERR_COLON: xmlParserErrors = 205;
pub const XML_NS_ERR_EMPTY: xmlParserErrors = 204;
pub const XML_NS_ERR_ATTRIBUTE_REDEFINED: xmlParserErrors = 203;
pub const XML_NS_ERR_QNAME: xmlParserErrors = 202;
pub const XML_NS_ERR_UNDEFINED_NAMESPACE: xmlParserErrors = 201;
pub const XML_NS_ERR_XML_NAMESPACE: xmlParserErrors = 200;
pub const XML_ERR_COMMENT_ABRUPTLY_ENDED: xmlParserErrors = 112;
pub const XML_ERR_USER_STOP: xmlParserErrors = 111;
pub const XML_ERR_NAME_TOO_LONG: xmlParserErrors = 110;
pub const XML_ERR_VERSION_MISMATCH: xmlParserErrors = 109;
pub const XML_ERR_UNKNOWN_VERSION: xmlParserErrors = 108;
pub const XML_WAR_ENTITY_REDEFINED: xmlParserErrors = 107;
pub const XML_WAR_NS_COLUMN: xmlParserErrors = 106;
pub const XML_ERR_NOTATION_PROCESSING: xmlParserErrors = 105;
pub const XML_ERR_ENTITY_PROCESSING: xmlParserErrors = 104;
pub const XML_ERR_NOT_STANDALONE: xmlParserErrors = 103;
pub const XML_WAR_SPACE_VALUE: xmlParserErrors = 102;
pub const XML_ERR_MISSING_ENCODING: xmlParserErrors = 101;
pub const XML_WAR_NS_URI_RELATIVE: xmlParserErrors = 100;
pub const XML_WAR_NS_URI: xmlParserErrors = 99;
pub const XML_WAR_LANG_VALUE: xmlParserErrors = 98;
pub const XML_WAR_UNKNOWN_VERSION: xmlParserErrors = 97;
pub const XML_ERR_VERSION_MISSING: xmlParserErrors = 96;
pub const XML_ERR_CONDSEC_INVALID_KEYWORD: xmlParserErrors = 95;
pub const XML_ERR_NO_DTD: xmlParserErrors = 94;
pub const XML_WAR_CATALOG_PI: xmlParserErrors = 93;
pub const XML_ERR_URI_FRAGMENT: xmlParserErrors = 92;
pub const XML_ERR_INVALID_URI: xmlParserErrors = 91;
pub const XML_ERR_ENTITY_BOUNDARY: xmlParserErrors = 90;
pub const XML_ERR_ENTITY_LOOP: xmlParserErrors = 89;
pub const XML_ERR_ENTITY_PE_INTERNAL: xmlParserErrors = 88;
pub const XML_ERR_ENTITY_CHAR_ERROR: xmlParserErrors = 87;
pub const XML_ERR_EXTRA_CONTENT: xmlParserErrors = 86;
pub const XML_ERR_NOT_WELL_BALANCED: xmlParserErrors = 85;
pub const XML_ERR_VALUE_REQUIRED: xmlParserErrors = 84;
pub const XML_ERR_CONDSEC_INVALID: xmlParserErrors = 83;
pub const XML_ERR_EXT_ENTITY_STANDALONE: xmlParserErrors = 82;
pub const XML_ERR_INVALID_ENCODING: xmlParserErrors = 81;
pub const XML_ERR_HYPHEN_IN_COMMENT: xmlParserErrors = 80;
pub const XML_ERR_ENCODING_NAME: xmlParserErrors = 79;
pub const XML_ERR_STANDALONE_VALUE: xmlParserErrors = 78;
pub const XML_ERR_TAG_NOT_FINISHED: xmlParserErrors = 77;
pub const XML_ERR_TAG_NAME_MISMATCH: xmlParserErrors = 76;
pub const XML_ERR_EQUAL_REQUIRED: xmlParserErrors = 75;
pub const XML_ERR_LTSLASH_REQUIRED: xmlParserErrors = 74;
pub const XML_ERR_GT_REQUIRED: xmlParserErrors = 73;
pub const XML_ERR_LT_REQUIRED: xmlParserErrors = 72;
pub const XML_ERR_PUBID_REQUIRED: xmlParserErrors = 71;
pub const XML_ERR_URI_REQUIRED: xmlParserErrors = 70;
pub const XML_ERR_PCDATA_REQUIRED: xmlParserErrors = 69;
pub const XML_ERR_NAME_REQUIRED: xmlParserErrors = 68;
pub const XML_ERR_NMTOKEN_REQUIRED: xmlParserErrors = 67;
pub const XML_ERR_SEPARATOR_REQUIRED: xmlParserErrors = 66;
pub const XML_ERR_SPACE_REQUIRED: xmlParserErrors = 65;
pub const XML_ERR_RESERVED_XML_NAME: xmlParserErrors = 64;
pub const XML_ERR_CDATA_NOT_FINISHED: xmlParserErrors = 63;
pub const XML_ERR_MISPLACED_CDATA_END: xmlParserErrors = 62;
pub const XML_ERR_DOCTYPE_NOT_FINISHED: xmlParserErrors = 61;
pub const XML_ERR_EXT_SUBSET_NOT_FINISHED: xmlParserErrors = 60;
pub const XML_ERR_CONDSEC_NOT_FINISHED: xmlParserErrors = 59;
pub const XML_ERR_CONDSEC_NOT_STARTED: xmlParserErrors = 58;
pub const XML_ERR_XMLDECL_NOT_FINISHED: xmlParserErrors = 57;
pub const XML_ERR_XMLDECL_NOT_STARTED: xmlParserErrors = 56;
pub const XML_ERR_ELEMCONTENT_NOT_FINISHED: xmlParserErrors = 55;
pub const XML_ERR_ELEMCONTENT_NOT_STARTED: xmlParserErrors = 54;
pub const XML_ERR_MIXED_NOT_FINISHED: xmlParserErrors = 53;
pub const XML_ERR_MIXED_NOT_STARTED: xmlParserErrors = 52;
pub const XML_ERR_ATTLIST_NOT_FINISHED: xmlParserErrors = 51;
pub const XML_ERR_ATTLIST_NOT_STARTED: xmlParserErrors = 50;
pub const XML_ERR_NOTATION_NOT_FINISHED: xmlParserErrors = 49;
pub const XML_ERR_NOTATION_NOT_STARTED: xmlParserErrors = 48;
pub const XML_ERR_PI_NOT_FINISHED: xmlParserErrors = 47;
pub const XML_ERR_PI_NOT_STARTED: xmlParserErrors = 46;
pub const XML_ERR_COMMENT_NOT_FINISHED: xmlParserErrors = 45;
pub const XML_ERR_LITERAL_NOT_FINISHED: xmlParserErrors = 44;
pub const XML_ERR_LITERAL_NOT_STARTED: xmlParserErrors = 43;
pub const XML_ERR_ATTRIBUTE_REDEFINED: xmlParserErrors = 42;
pub const XML_ERR_ATTRIBUTE_WITHOUT_VALUE: xmlParserErrors = 41;
pub const XML_ERR_ATTRIBUTE_NOT_FINISHED: xmlParserErrors = 40;
pub const XML_ERR_ATTRIBUTE_NOT_STARTED: xmlParserErrors = 39;
pub const XML_ERR_LT_IN_ATTRIBUTE: xmlParserErrors = 38;
pub const XML_ERR_ENTITY_NOT_FINISHED: xmlParserErrors = 37;
pub const XML_ERR_ENTITY_NOT_STARTED: xmlParserErrors = 36;
pub const XML_ERR_NS_DECL_ERROR: xmlParserErrors = 35;
pub const XML_ERR_STRING_NOT_CLOSED: xmlParserErrors = 34;
pub const XML_ERR_STRING_NOT_STARTED: xmlParserErrors = 33;
pub const XML_ERR_UNSUPPORTED_ENCODING: xmlParserErrors = 32;
pub const XML_ERR_UNKNOWN_ENCODING: xmlParserErrors = 31;
pub const XML_ERR_ENTITY_IS_PARAMETER: xmlParserErrors = 30;
pub const XML_ERR_ENTITY_IS_EXTERNAL: xmlParserErrors = 29;
pub const XML_ERR_UNPARSED_ENTITY: xmlParserErrors = 28;
pub const XML_WAR_UNDECLARED_ENTITY: xmlParserErrors = 27;
pub const XML_ERR_UNDECLARED_ENTITY: xmlParserErrors = 26;
pub const XML_ERR_PEREF_SEMICOL_MISSING: xmlParserErrors = 25;
pub const XML_ERR_PEREF_NO_NAME: xmlParserErrors = 24;
pub const XML_ERR_ENTITYREF_SEMICOL_MISSING: xmlParserErrors = 23;
pub const XML_ERR_ENTITYREF_NO_NAME: xmlParserErrors = 22;
pub const XML_ERR_PEREF_IN_INT_SUBSET: xmlParserErrors = 21;
pub const XML_ERR_PEREF_IN_EPILOG: xmlParserErrors = 20;
pub const XML_ERR_PEREF_IN_PROLOG: xmlParserErrors = 19;
pub const XML_ERR_PEREF_AT_EOF: xmlParserErrors = 18;
pub const XML_ERR_ENTITYREF_IN_DTD: xmlParserErrors = 17;
pub const XML_ERR_ENTITYREF_IN_EPILOG: xmlParserErrors = 16;
pub const XML_ERR_ENTITYREF_IN_PROLOG: xmlParserErrors = 15;
pub const XML_ERR_ENTITYREF_AT_EOF: xmlParserErrors = 14;
pub const XML_ERR_CHARREF_IN_DTD: xmlParserErrors = 13;
pub const XML_ERR_CHARREF_IN_EPILOG: xmlParserErrors = 12;
pub const XML_ERR_CHARREF_IN_PROLOG: xmlParserErrors = 11;
pub const XML_ERR_CHARREF_AT_EOF: xmlParserErrors = 10;
pub const XML_ERR_INVALID_CHAR: xmlParserErrors = 9;
pub const XML_ERR_INVALID_CHARREF: xmlParserErrors = 8;
pub const XML_ERR_INVALID_DEC_CHARREF: xmlParserErrors = 7;
pub const XML_ERR_INVALID_HEX_CHARREF: xmlParserErrors = 6;
pub const XML_ERR_DOCUMENT_END: xmlParserErrors = 5;
pub const XML_ERR_DOCUMENT_EMPTY: xmlParserErrors = 4;
pub const XML_ERR_DOCUMENT_START: xmlParserErrors = 3;
pub const XML_ERR_NO_MEMORY: xmlParserErrors = 2;
pub const XML_ERR_INTERNAL_ERROR: xmlParserErrors = 1;
pub const XML_ERR_OK: xmlParserErrors = 0;
pub type xmlGenericErrorFunc = Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,_: * const i8,...) -> ()>;
pub type xmlValidCtxtPtr = * mut crate::src::HTMLparser::_xmlValidCtxt;
pub type xmlParserNodeInfoPtr = * mut crate::src::HTMLparser::_xmlParserNodeInfo;
// #[derive(Copy, Clone)]

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
pub type C2RustUnnamed_1 = u32;
pub const XML_PARSE_BIG_LINES: C2RustUnnamed_1 = 4194304;
pub const XML_PARSE_IGNORE_ENC: C2RustUnnamed_1 = 2097152;
pub const XML_PARSE_OLDSAX: C2RustUnnamed_1 = 1048576;
pub const XML_PARSE_HUGE: C2RustUnnamed_1 = 524288;
pub const XML_PARSE_NOBASEFIX: C2RustUnnamed_1 = 262144;
pub const XML_PARSE_OLD10: C2RustUnnamed_1 = 131072;
pub const XML_PARSE_COMPACT: C2RustUnnamed_1 = 65536;
pub const XML_PARSE_NOXINCNODE: C2RustUnnamed_1 = 32768;
pub const XML_PARSE_NOCDATA: C2RustUnnamed_1 = 16384;
pub const XML_PARSE_NSCLEAN: C2RustUnnamed_1 = 8192;
pub const XML_PARSE_NODICT: C2RustUnnamed_1 = 4096;
pub const XML_PARSE_NONET: C2RustUnnamed_1 = 2048;
pub const XML_PARSE_XINCLUDE: C2RustUnnamed_1 = 1024;
pub const XML_PARSE_SAX1: C2RustUnnamed_1 = 512;
pub const XML_PARSE_NOBLANKS: C2RustUnnamed_1 = 256;
pub const XML_PARSE_PEDANTIC: C2RustUnnamed_1 = 128;
pub const XML_PARSE_NOWARNING: C2RustUnnamed_1 = 64;
pub const XML_PARSE_NOERROR: C2RustUnnamed_1 = 32;
pub const XML_PARSE_DTDVALID: C2RustUnnamed_1 = 16;
pub const XML_PARSE_DTDATTR: C2RustUnnamed_1 = 8;
pub const XML_PARSE_DTDLOAD: C2RustUnnamed_1 = 4;
pub const XML_PARSE_NOENT: C2RustUnnamed_1 = 2;
pub const XML_PARSE_RECOVER: C2RustUnnamed_1 = 1;
pub type htmlDocPtr = * mut crate::src::HTMLparser::_xmlDoc;
pub type xmlURIPtr = * mut crate::src::SAX2::_xmlURI;
pub type xmlURI = crate::src::SAX2::_xmlURI;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlURI {
    pub scheme: * mut i8,
    pub opaque: * mut i8,
    pub authority: * mut i8,
    pub server: * mut i8,
    pub user: * mut i8,
    pub port: i32,
    pub path: * mut i8,
    pub query: * mut i8,
    pub fragment: * mut i8,
    pub cleanup: i32,
    pub query_raw: * mut i8,
}
impl _xmlURI {
    pub const fn new() -> Self {
        _xmlURI {
        scheme: (0 as * mut i8),
        opaque: (0 as * mut i8),
        authority: (0 as * mut i8),
        server: (0 as * mut i8),
        user: (0 as * mut i8),
        port: 0,
        path: (0 as * mut i8),
        query: (0 as * mut i8),
        fragment: (0 as * mut i8),
        cleanup: 0,
        query_raw: (0 as * mut i8)
        }
    }
}

impl std::default::Default for _xmlURI {
    fn default() -> Self { _xmlURI::new() }
}

pub type xmlRegisterNodeFunc = Option<unsafe extern "C"  fn(_: * mut crate::src::HTMLparser::_xmlNode,) -> ()>;
unsafe extern "C" fn xmlSAX2ErrMemory(
    mut ctxt: * mut crate::src::HTMLparser::_xmlParserCtxt,
    mut msg: * const i8,
) {
    let mut schannel: Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,_: * mut crate::src::HTMLparser::_xmlError,) -> ()> = None;
    let mut str1: * const i8 = b"out of memory\n\0" as *const u8
        as *const i8;
    if !ctxt.is_null() {
        (*ctxt).errNo = XML_ERR_NO_MEMORY as i32;
        if !((*ctxt).sax).is_null()
            && (*(*ctxt).sax).initialized == 0xdeedbeaf as u32
        {
            schannel = (*(*ctxt).sax).serror;
        }
        __xmlRaiseError(
            schannel,
            (*ctxt).vctxt.error,
            (*ctxt).vctxt.userData,
            ctxt as *mut libc::c_void,
            0 as *mut libc::c_void,
            XML_FROM_PARSER as i32,
            XML_ERR_NO_MEMORY as i32,
            XML_ERR_ERROR,
            0 as *const i8,
            0 as i32,
            str1,
            0 as *const i8,
            0 as *const i8,
            0 as i32,
            0 as i32,
            msg,
            str1,
            0 as *mut libc::c_void,
        );
        (*ctxt).errNo = XML_ERR_NO_MEMORY as i32;
        (*ctxt).instate = XML_PARSER_EOF;
        (*ctxt).disableSAX = 1 as i32;
    } else {
        __xmlRaiseError(
            schannel,
            None,
            0 as *mut libc::c_void,
            ctxt as *mut libc::c_void,
            0 as *mut libc::c_void,
            XML_FROM_PARSER as i32,
            XML_ERR_NO_MEMORY as i32,
            XML_ERR_ERROR,
            0 as *const i8,
            0 as i32,
            str1,
            0 as *const i8,
            0 as *const i8,
            0 as i32,
            0 as i32,
            msg,
            str1,
            0 as *mut libc::c_void,
        );
    };
}
unsafe extern "C" fn xmlErrValid(
    mut ctxt: * mut crate::src::HTMLparser::_xmlParserCtxt,
    mut error: u32,
    mut msg: * const i8,
    mut str1: * const i8,
    mut str2: * const i8,
) {
    let mut schannel: Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,_: * mut crate::src::HTMLparser::_xmlError,) -> ()> = None;
    if !ctxt.is_null() && (*ctxt).disableSAX != 0 as i32
        && (*ctxt).instate as i32 == XML_PARSER_EOF as i32
    {
        return;
    }
    if !ctxt.is_null() {
        (*ctxt).errNo = error as i32;
        if !((*ctxt).sax).is_null()
            && (*(*ctxt).sax).initialized == 0xdeedbeaf as u32
        {
            schannel = (*(*ctxt).sax).serror;
        }
        __xmlRaiseError(
            schannel,
            (*ctxt).vctxt.error,
            (*ctxt).vctxt.userData,
            ctxt as *mut libc::c_void,
            0 as *mut libc::c_void,
            XML_FROM_DTD as i32,
            error as i32,
            XML_ERR_ERROR,
            0 as *const i8,
            0 as i32,
            str1,
            str2,
            0 as *const i8,
            0 as i32,
            0 as i32,
            msg,
            str1,
            str2,
        );
        (*ctxt).valid = 0 as i32;
    } else {
        __xmlRaiseError(
            schannel,
            None,
            0 as *mut libc::c_void,
            ctxt as *mut libc::c_void,
            0 as *mut libc::c_void,
            XML_FROM_DTD as i32,
            error as i32,
            XML_ERR_ERROR,
            0 as *const i8,
            0 as i32,
            str1,
            str2,
            0 as *const i8,
            0 as i32,
            0 as i32,
            msg,
            str1,
            str2,
        );
    };
}
unsafe extern "C" fn xmlFatalErrMsg(
    mut ctxt: * mut crate::src::HTMLparser::_xmlParserCtxt,
    mut error: u32,
    mut msg: * const i8,
    mut str1: * const u8,
    mut str2: * const u8,
) {
    if !ctxt.is_null() && (*ctxt).disableSAX != 0 as i32
        && (*ctxt).instate as i32 == XML_PARSER_EOF as i32
    {
        return;
    }
    if !ctxt.is_null() {
        (*ctxt).errNo = error as i32;
    }
    __xmlRaiseError(
        None,
        None,
        0 as *mut libc::c_void,
        ctxt as *mut libc::c_void,
        0 as *mut libc::c_void,
        XML_FROM_PARSER as i32,
        error as i32,
        XML_ERR_FATAL,
        0 as *const i8,
        0 as i32,
        str1 as *const i8,
        str2 as *const i8,
        0 as *const i8,
        0 as i32,
        0 as i32,
        msg,
        str1,
        str2,
    );
    if !ctxt.is_null() {
        (*ctxt).wellFormed = 0 as i32;
        (*ctxt).valid = 0 as i32;
        if (*ctxt).recovery == 0 as i32 {
            (*ctxt).disableSAX = 1 as i32;
        }
    }
}
unsafe extern "C" fn xmlWarnMsg(
    mut ctxt: * mut crate::src::HTMLparser::_xmlParserCtxt,
    mut error: u32,
    mut msg: * const i8,
    mut str1: * const u8,
) {
    if !ctxt.is_null() && (*ctxt).disableSAX != 0 as i32
        && (*ctxt).instate as i32 == XML_PARSER_EOF as i32
    {
        return;
    }
    if !ctxt.is_null() {
        (*ctxt).errNo = error as i32;
    }
    __xmlRaiseError(
        None,
        None,
        0 as *mut libc::c_void,
        ctxt as *mut libc::c_void,
        0 as *mut libc::c_void,
        XML_FROM_PARSER as i32,
        error as i32,
        XML_ERR_WARNING,
        0 as *const i8,
        0 as i32,
        str1 as *const i8,
        0 as *const i8,
        0 as *const i8,
        0 as i32,
        0 as i32,
        msg,
        str1,
    );
}
unsafe extern "C" fn xmlNsWarnMsg(
    mut ctxt: * mut crate::src::HTMLparser::_xmlParserCtxt,
    mut error: u32,
    mut msg: * const i8,
    mut str1: * const u8,
    mut str2: * const u8,
) {
    if !ctxt.is_null() && (*ctxt).disableSAX != 0 as i32
        && (*ctxt).instate as i32 == XML_PARSER_EOF as i32
    {
        return;
    }
    if !ctxt.is_null() {
        (*ctxt).errNo = error as i32;
    }
    __xmlRaiseError(
        None,
        None,
        0 as *mut libc::c_void,
        ctxt as *mut libc::c_void,
        0 as *mut libc::c_void,
        XML_FROM_NAMESPACE as i32,
        error as i32,
        XML_ERR_WARNING,
        0 as *const i8,
        0 as i32,
        str1 as *const i8,
        str2 as *const i8,
        0 as *const i8,
        0 as i32,
        0 as i32,
        msg,
        str1,
        str2,
    );
}
#[no_mangle]
pub extern "C" fn xmlSAX2GetPublicId(
    mut ctx: * mut core::ffi::c_void,
) -> * const u8 {
    return 0 as *const xmlChar;
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2GetSystemId(
    mut ctx: * mut core::ffi::c_void,
) -> * const u8 {
    let mut ctxt: * mut crate::src::HTMLparser::_xmlParserCtxt = ctx as xmlParserCtxtPtr;
    if ctx.is_null() || ((*ctxt).input).is_null() {
        return 0 as *const xmlChar;
    }
    return (*(*ctxt).input).filename as *const xmlChar;
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2GetLineNumber(
    mut ctx: * mut core::ffi::c_void,
) -> i32 {
    let mut ctxt: * mut crate::src::HTMLparser::_xmlParserCtxt = ctx as xmlParserCtxtPtr;
    if ctx.is_null() || ((*ctxt).input).is_null() {
        return 0 as i32;
    }
    return (*(*ctxt).input).line;
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2GetColumnNumber(
    mut ctx: * mut core::ffi::c_void,
) -> i32 {
    let mut ctxt: * mut crate::src::HTMLparser::_xmlParserCtxt = ctx as xmlParserCtxtPtr;
    if ctx.is_null() || ((*ctxt).input).is_null() {
        return 0 as i32;
    }
    return (*(*ctxt).input).col;
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2IsStandalone(mut ctx: * mut core::ffi::c_void) -> i32 {
    let mut ctxt: * mut crate::src::HTMLparser::_xmlParserCtxt = ctx as xmlParserCtxtPtr;
    if ctx.is_null() || ((*ctxt).myDoc).is_null() {
        return 0 as i32;
    }
    return ((*(*ctxt).myDoc).standalone == 1 as i32) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2HasInternalSubset(
    mut ctx: * mut core::ffi::c_void,
) -> i32 {
    let mut ctxt: * mut crate::src::HTMLparser::_xmlParserCtxt = ctx as xmlParserCtxtPtr;
    if ctxt.is_null() || ((*ctxt).myDoc).is_null() {
        return 0 as i32;
    }
    return ((*(*ctxt).myDoc).intSubset != 0 as *mut libc::c_void as *mut _xmlDtd)
        as i32;
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2HasExternalSubset(
    mut ctx: * mut core::ffi::c_void,
) -> i32 {
    let mut ctxt: * mut crate::src::HTMLparser::_xmlParserCtxt = ctx as xmlParserCtxtPtr;
    if ctxt.is_null() || ((*ctxt).myDoc).is_null() {
        return 0 as i32;
    }
    return ((*(*ctxt).myDoc).extSubset != 0 as *mut libc::c_void as *mut _xmlDtd)
        as i32;
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2InternalSubset(
    mut ctx: * mut core::ffi::c_void,
    mut name: * const u8,
    mut ExternalID: * const u8,
    mut SystemID: * const u8,
) {
    let mut ctxt: * mut crate::src::HTMLparser::_xmlParserCtxt = ctx as xmlParserCtxtPtr;
    let mut dtd: * mut crate::src::HTMLparser::_xmlDtd = 0 as *mut xmlDtd;
    if ctx.is_null() {
        return;
    }
    if ((*ctxt).myDoc).is_null() {
        return;
    }
    dtd = xmlGetIntSubset((*ctxt).myDoc as *const xmlDoc);
    if !dtd.is_null() {
        if (*ctxt).html != 0 {
            return;
        }
        xmlUnlinkNode(dtd as xmlNodePtr);
        xmlFreeDtd(dtd);
        let ref mut fresh0 = (*(*ctxt).myDoc).intSubset;
        *fresh0 = 0 as *mut _xmlDtd;
    }
    let ref mut fresh1 = (*(*ctxt).myDoc).intSubset;
    *fresh1 = xmlCreateIntSubset((*ctxt).myDoc, name, ExternalID, SystemID);
    if ((*(*ctxt).myDoc).intSubset).is_null() {
        xmlSAX2ErrMemory(
            ctxt,
            b"xmlSAX2InternalSubset\0" as *const u8 as *const i8,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2ExternalSubset(
    mut ctx: * mut core::ffi::c_void,
    mut name: * const u8,
    mut ExternalID: * const u8,
    mut SystemID: * const u8,
) {
    let mut ctxt: * mut crate::src::HTMLparser::_xmlParserCtxt = ctx as xmlParserCtxtPtr;
    if ctx.is_null() {
        return;
    }
    if (!ExternalID.is_null() || !SystemID.is_null())
        && (((*ctxt).validate != 0 || (*ctxt).loadsubset != 0 as i32)
            && ((*ctxt).wellFormed != 0 && !((*ctxt).myDoc).is_null()))
    {
        let mut oldinput: * mut crate::src::HTMLparser::_xmlParserInput = 0 as *mut xmlParserInput;
        let mut oldinputNr: i32 = 0;
        let mut oldinputMax: i32 = 0;
        let mut oldinputTab: * mut * mut crate::src::HTMLparser::_xmlParserInput = 0 as *mut xmlParserInputPtr;
        let mut input: * mut crate::src::HTMLparser::_xmlParserInput = 0 as xmlParserInputPtr;
        let mut enc: i32 = XML_CHAR_ENCODING_NONE;
        let mut oldcharset: i32 = 0;
        let mut oldencoding: * const u8 = 0 as *const xmlChar;
        if !((*ctxt).sax).is_null() && ((*(*ctxt).sax).resolveEntity).is_some() {
            input = ((*(*ctxt).sax).resolveEntity)
                .expect(
                    "non-null function pointer",
                )((*ctxt).userData, ExternalID, SystemID);
        }
        if input.is_null() {
            return;
        }
        xmlNewDtd((*ctxt).myDoc, name, ExternalID, SystemID);
        oldinput = (*ctxt).input;
        oldinputNr = (*ctxt).inputNr;
        oldinputMax = (*ctxt).inputMax;
        oldinputTab = (*ctxt).inputTab;
        oldcharset = (*ctxt).charset;
        oldencoding = (*ctxt).encoding;
        let ref mut fresh2 = (*ctxt).encoding;
        *fresh2 = 0 as *const xmlChar;
        let ref mut fresh3 = (*ctxt).inputTab;
        *fresh3 = xmlMalloc
            .expect(
                "non-null function pointer",
            )(
            (5 as i32 as u64)
                .wrapping_mul(
                    ::std::mem::size_of::<xmlParserInputPtr>() as u64,
                ),
        ) as *mut xmlParserInputPtr;
        if ((*ctxt).inputTab).is_null() {
            xmlSAX2ErrMemory(
                ctxt,
                b"xmlSAX2ExternalSubset\0" as *const u8 as *const i8,
            );
            let ref mut fresh4 = (*ctxt).input;
            *fresh4 = oldinput;
            (*ctxt).inputNr = oldinputNr;
            (*ctxt).inputMax = oldinputMax;
            let ref mut fresh5 = (*ctxt).inputTab;
            *fresh5 = oldinputTab;
            (*ctxt).charset = oldcharset;
            let ref mut fresh6 = (*ctxt).encoding;
            *fresh6 = oldencoding;
            return;
        }
        (*ctxt).inputNr = 0 as i32;
        (*ctxt).inputMax = 5 as i32;
        let ref mut fresh7 = (*ctxt).input;
        *fresh7 = 0 as xmlParserInputPtr;
        xmlPushInput(ctxt, input);
        if (*(*ctxt).input).length >= 4 as i32 {
            enc = xmlDetectCharEncoding((*(*ctxt).input).cur, 4 as i32);
            xmlSwitchEncoding(ctxt, enc);
        }
        if ((*input).filename).is_null() {
            let ref mut fresh8 = (*input).filename;
            *fresh8 = xmlCanonicPath(SystemID) as *mut i8;
        }
        (*input).line = 1 as i32;
        (*input).col = 1 as i32;
        let ref mut fresh9 = (*input).base;
        *fresh9 = (*(*ctxt).input).cur;
        let ref mut fresh10 = (*input).cur;
        *fresh10 = (*(*ctxt).input).cur;
        let ref mut fresh11 = (*input).free;
        *fresh11 = None;
        xmlParseExternalSubset(ctxt, ExternalID, SystemID);
        while (*ctxt).inputNr > 1 as i32 {
            xmlPopInput(ctxt);
        }
        xmlFreeInputStream((*ctxt).input);
        xmlFree
            .expect("non-null function pointer")((*ctxt).inputTab as *mut libc::c_void);
        let ref mut fresh12 = (*ctxt).input;
        *fresh12 = oldinput;
        (*ctxt).inputNr = oldinputNr;
        (*ctxt).inputMax = oldinputMax;
        let ref mut fresh13 = (*ctxt).inputTab;
        *fresh13 = oldinputTab;
        (*ctxt).charset = oldcharset;
        if !((*ctxt).encoding).is_null()
            && (((*ctxt).dict).is_null()
                || xmlDictOwns((*ctxt).dict, (*ctxt).encoding) == 0)
        {
            xmlFree
                .expect(
                    "non-null function pointer",
                )((*ctxt).encoding as *mut xmlChar as *mut libc::c_void);
        }
        let ref mut fresh14 = (*ctxt).encoding;
        *fresh14 = oldencoding;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2ResolveEntity(
    mut ctx: * mut core::ffi::c_void,
    mut publicId: * const u8,
    mut systemId: * const u8,
) -> * mut crate::src::HTMLparser::_xmlParserInput {
    let mut ctxt: * mut crate::src::HTMLparser::_xmlParserCtxt = ctx as xmlParserCtxtPtr;
    let mut ret: * mut crate::src::HTMLparser::_xmlParserInput = 0 as *mut xmlParserInput;
    let mut URI: * mut u8 = 0 as *mut xmlChar;
    let mut base: * const i8 = 0 as *const i8;
    if ctx.is_null() {
        return 0 as xmlParserInputPtr;
    }
    if !((*ctxt).input).is_null() {
        base = (*(*ctxt).input).filename;
    }
    if base.is_null() {
        base = (*ctxt).directory;
    }
    URI = xmlBuildURI(systemId, base as *const xmlChar);
    ret = xmlLoadExternalEntity(
        URI as *const i8,
        publicId as *const i8,
        ctxt,
    );
    if !URI.is_null() {
        xmlFree.expect("non-null function pointer")(URI as *mut libc::c_void);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2GetEntity(
    mut ctx: * mut core::ffi::c_void,
    mut name: * const u8,
) -> * mut crate::src::HTMLparser::_xmlEntity {
    let mut ctxt: * mut crate::src::HTMLparser::_xmlParserCtxt = ctx as xmlParserCtxtPtr;
    let mut ret: * mut crate::src::HTMLparser::_xmlEntity = 0 as xmlEntityPtr;
    if ctx.is_null() {
        return 0 as xmlEntityPtr;
    }
    if (*ctxt).inSubset == 0 as i32 {
        ret = xmlGetPredefinedEntity(name);
        if !ret.is_null() {
            return ret;
        }
    }
    if !((*ctxt).myDoc).is_null() && (*(*ctxt).myDoc).standalone == 1 as i32 {
        if (*ctxt).inSubset == 2 as i32 {
            (*(*ctxt).myDoc).standalone = 0 as i32;
            ret = xmlGetDocEntity((*ctxt).myDoc as *const xmlDoc, name);
            (*(*ctxt).myDoc).standalone = 1 as i32;
        } else {
            ret = xmlGetDocEntity((*ctxt).myDoc as *const xmlDoc, name);
            if ret.is_null() {
                (*(*ctxt).myDoc).standalone = 0 as i32;
                ret = xmlGetDocEntity((*ctxt).myDoc as *const xmlDoc, name);
                if !ret.is_null() {
                    xmlFatalErrMsg(
                        ctxt,
                        XML_ERR_NOT_STANDALONE,
                        b"Entity(%s) document marked standalone but requires external subset\n\0"
                            as *const u8 as *const i8,
                        name,
                        0 as *const xmlChar,
                    );
                }
                (*(*ctxt).myDoc).standalone = 1 as i32;
            }
        }
    } else {
        ret = xmlGetDocEntity((*ctxt).myDoc as *const xmlDoc, name);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2GetParameterEntity(
    mut ctx: * mut core::ffi::c_void,
    mut name: * const u8,
) -> * mut crate::src::HTMLparser::_xmlEntity {
    let mut ctxt: * mut crate::src::HTMLparser::_xmlParserCtxt = ctx as xmlParserCtxtPtr;
    let mut ret: * mut crate::src::HTMLparser::_xmlEntity = 0 as *mut xmlEntity;
    if ctx.is_null() {
        return 0 as xmlEntityPtr;
    }
    ret = xmlGetParameterEntity((*ctxt).myDoc, name);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2EntityDecl(
    mut ctx: * mut core::ffi::c_void,
    mut name: * const u8,
    mut type_0: i32,
    mut publicId: * const u8,
    mut systemId: * const u8,
    mut content: * mut u8,
) {
    let mut ent: * mut crate::src::HTMLparser::_xmlEntity = 0 as *mut xmlEntity;
    let mut ctxt: * mut crate::src::HTMLparser::_xmlParserCtxt = ctx as xmlParserCtxtPtr;
    if ctx.is_null() {
        return;
    }
    if (*ctxt).inSubset == 1 as i32 {
        ent = xmlAddDocEntity((*ctxt).myDoc, name, type_0, publicId, systemId, content);
        if ent.is_null() && (*ctxt).pedantic != 0 {
            xmlWarnMsg(
                ctxt,
                XML_WAR_ENTITY_REDEFINED,
                b"Entity(%s) already defined in the internal subset\n\0" as *const u8
                    as *const i8,
                name,
            );
        }
        if !ent.is_null() && ((*ent).URI).is_null() && !systemId.is_null() {
            let mut URI: * mut u8 = 0 as *mut xmlChar;
            let mut base: * const i8 = 0 as *const i8;
            if !((*ctxt).input).is_null() {
                base = (*(*ctxt).input).filename;
            }
            if base.is_null() {
                base = (*ctxt).directory;
            }
            URI = xmlBuildURI(systemId, base as *const xmlChar);
            let ref mut fresh15 = (*ent).URI;
            *fresh15 = URI;
        }
    } else if (*ctxt).inSubset == 2 as i32 {
        ent = xmlAddDtdEntity((*ctxt).myDoc, name, type_0, publicId, systemId, content);
        if ent.is_null() && (*ctxt).pedantic != 0 && !((*ctxt).sax).is_null()
            && ((*(*ctxt).sax).warning).is_some()
        {
            ((*(*ctxt).sax).warning)
                .expect(
                    "non-null function pointer",
                )(
                (*ctxt).userData,
                b"Entity(%s) already defined in the external subset\n\0" as *const u8
                    as *const i8,
                name,
            );
        }
        if !ent.is_null() && ((*ent).URI).is_null() && !systemId.is_null() {
            let mut URI_0: * mut u8 = 0 as *mut xmlChar;
            let mut base_0: * const i8 = 0 as *const i8;
            if !((*ctxt).input).is_null() {
                base_0 = (*(*ctxt).input).filename;
            }
            if base_0.is_null() {
                base_0 = (*ctxt).directory;
            }
            URI_0 = xmlBuildURI(systemId, base_0 as *const xmlChar);
            let ref mut fresh16 = (*ent).URI;
            *fresh16 = URI_0;
        }
    } else {
        xmlFatalErrMsg(
            ctxt,
            XML_ERR_ENTITY_PROCESSING,
            b"SAX.xmlSAX2EntityDecl(%s) called while not in subset\n\0" as *const u8
                as *const i8,
            name,
            0 as *const xmlChar,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2AttributeDecl(
    mut ctx: * mut core::ffi::c_void,
    mut elem: * const u8,
    mut fullname: * const u8,
    mut type_0: i32,
    mut def: i32,
    mut defaultValue: * const u8,
    mut tree: * mut crate::src::HTMLparser::_xmlEnumeration,
) {
    let mut ctxt: * mut crate::src::HTMLparser::_xmlParserCtxt = ctx as xmlParserCtxtPtr;
    let mut attr: * mut crate::src::SAX2::_xmlAttribute = 0 as *mut xmlAttribute;
    let mut name: * mut u8 = 0 as *mut xmlChar;
    let mut prefix: * mut u8 = 0 as *mut xmlChar;
    if ctxt.is_null() || ((*ctxt).myDoc).is_null() {
        return;
    }
    if xmlStrEqual(
        fullname,
        b"xml:id\0" as *const u8 as *const i8 as *mut xmlChar,
    ) != 0 && type_0 != XML_ATTRIBUTE_ID as i32
    {
        let mut tmp: i32 = (*ctxt).valid;
        xmlErrValid(
            ctxt,
            XML_DTD_XMLID_TYPE,
            b"xml:id : attribute type should be ID\n\0" as *const u8
                as *const i8,
            0 as *const i8,
            0 as *const i8,
        );
        (*ctxt).valid = tmp;
    }
    name = xmlSplitQName(ctxt, fullname, Some(&mut prefix));
    (*ctxt).vctxt.valid = 1 as i32;
    if (*ctxt).inSubset == 1 as i32 {
        attr = xmlAddAttributeDecl(
            &mut (*ctxt).vctxt,
            (*(*ctxt).myDoc).intSubset,
            elem,
            name,
            prefix,
            type_0 as xmlAttributeType,
            def as xmlAttributeDefault,
            defaultValue,
            tree,
        );
    } else if (*ctxt).inSubset == 2 as i32 {
        attr = xmlAddAttributeDecl(
            &mut (*ctxt).vctxt,
            (*(*ctxt).myDoc).extSubset,
            elem,
            name,
            prefix,
            type_0 as xmlAttributeType,
            def as xmlAttributeDefault,
            defaultValue,
            tree,
        );
    } else {
        xmlFatalErrMsg(
            ctxt,
            XML_ERR_INTERNAL_ERROR,
            b"SAX.xmlSAX2AttributeDecl(%s) called while not in subset\n\0" as *const u8
                as *const i8,
            name,
            0 as *const xmlChar,
        );
        xmlFree.expect("non-null function pointer")(name as *mut libc::c_void);
        xmlFreeEnumeration(tree);
        return;
    }
    if (*ctxt).vctxt.valid == 0 as i32 {
        (*ctxt).valid = 0 as i32;
    }
    if !attr.is_null() && (*ctxt).validate != 0 && (*ctxt).wellFormed != 0
        && !((*(*ctxt).myDoc).intSubset).is_null()
    {
        (*ctxt).valid
            &= xmlValidateAttributeDecl(&mut (*ctxt).vctxt, (*ctxt).myDoc, attr);
    }
    if !prefix.is_null() {
        xmlFree.expect("non-null function pointer")(prefix as *mut libc::c_void);
    }
    if !name.is_null() {
        xmlFree.expect("non-null function pointer")(name as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2ElementDecl(
    mut ctx: * mut core::ffi::c_void,
    mut name: * const u8,
    mut type_0: i32,
    mut content: * mut crate::src::HTMLparser::_xmlElementContent,
) {
    let mut ctxt: * mut crate::src::HTMLparser::_xmlParserCtxt = ctx as xmlParserCtxtPtr;
    let mut elem: * mut crate::src::SAX2::_xmlElement = 0 as xmlElementPtr;
    if ctxt.is_null() || ((*ctxt).myDoc).is_null() {
        return;
    }
    if (*ctxt).inSubset == 1 as i32 {
        elem = xmlAddElementDecl(
            &mut (*ctxt).vctxt,
            (*(*ctxt).myDoc).intSubset,
            name,
            type_0 as xmlElementTypeVal,
            content,
        );
    } else if (*ctxt).inSubset == 2 as i32 {
        elem = xmlAddElementDecl(
            &mut (*ctxt).vctxt,
            (*(*ctxt).myDoc).extSubset,
            name,
            type_0 as xmlElementTypeVal,
            content,
        );
    } else {
        xmlFatalErrMsg(
            ctxt,
            XML_ERR_INTERNAL_ERROR,
            b"SAX.xmlSAX2ElementDecl(%s) called while not in subset\n\0" as *const u8
                as *const i8,
            name,
            0 as *const xmlChar,
        );
        return;
    }
    if elem.is_null() {
        (*ctxt).valid = 0 as i32;
    }
    if (*ctxt).validate != 0 && (*ctxt).wellFormed != 0 && !((*ctxt).myDoc).is_null()
        && !((*(*ctxt).myDoc).intSubset).is_null()
    {
        (*ctxt).valid &= xmlValidateElementDecl(&mut (*ctxt).vctxt, (*ctxt).myDoc, elem);
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2NotationDecl(
    mut ctx: * mut core::ffi::c_void,
    mut name: * const u8,
    mut publicId: * const u8,
    mut systemId: * const u8,
) {
    let mut ctxt: * mut crate::src::HTMLparser::_xmlParserCtxt = ctx as xmlParserCtxtPtr;
    let mut nota: * mut crate::src::SAX2::_xmlNotation = 0 as xmlNotationPtr;
    if ctxt.is_null() || ((*ctxt).myDoc).is_null() {
        return;
    }
    if publicId.is_null() && systemId.is_null() {
        xmlFatalErrMsg(
            ctxt,
            XML_ERR_NOTATION_PROCESSING,
            b"SAX.xmlSAX2NotationDecl(%s) externalID or PublicID missing\n\0"
                as *const u8 as *const i8,
            name,
            0 as *const xmlChar,
        );
        return;
    } else {
        if (*ctxt).inSubset == 1 as i32 {
            nota = xmlAddNotationDecl(
                &mut (*ctxt).vctxt,
                (*(*ctxt).myDoc).intSubset,
                name,
                publicId,
                systemId,
            );
        } else if (*ctxt).inSubset == 2 as i32 {
            nota = xmlAddNotationDecl(
                &mut (*ctxt).vctxt,
                (*(*ctxt).myDoc).extSubset,
                name,
                publicId,
                systemId,
            );
        } else {
            xmlFatalErrMsg(
                ctxt,
                XML_ERR_NOTATION_PROCESSING,
                b"SAX.xmlSAX2NotationDecl(%s) called while not in subset\n\0"
                    as *const u8 as *const i8,
                name,
                0 as *const xmlChar,
            );
            return;
        }
    }
    if nota.is_null() {
        (*ctxt).valid = 0 as i32;
    }
    if (*ctxt).validate != 0 && (*ctxt).wellFormed != 0
        && !((*(*ctxt).myDoc).intSubset).is_null()
    {
        (*ctxt).valid
            &= xmlValidateNotationDecl(&mut (*ctxt).vctxt, (*ctxt).myDoc, nota);
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2UnparsedEntityDecl(
    mut ctx: * mut core::ffi::c_void,
    mut name: * const u8,
    mut publicId: * const u8,
    mut systemId: * const u8,
    mut notationName: * const u8,
) {
    let mut ent: * mut crate::src::HTMLparser::_xmlEntity = 0 as *mut xmlEntity;
    let mut ctxt: * mut crate::src::HTMLparser::_xmlParserCtxt = ctx as xmlParserCtxtPtr;
    if ctx.is_null() {
        return;
    }
    if (*ctxt).inSubset == 1 as i32 {
        ent = xmlAddDocEntity(
            (*ctxt).myDoc,
            name,
            XML_EXTERNAL_GENERAL_UNPARSED_ENTITY as i32,
            publicId,
            systemId,
            notationName,
        );
        if ent.is_null() && (*ctxt).pedantic != 0 && !((*ctxt).sax).is_null()
            && ((*(*ctxt).sax).warning).is_some()
        {
            ((*(*ctxt).sax).warning)
                .expect(
                    "non-null function pointer",
                )(
                (*ctxt).userData,
                b"Entity(%s) already defined in the internal subset\n\0" as *const u8
                    as *const i8,
                name,
            );
        }
        if !ent.is_null() && ((*ent).URI).is_null() && !systemId.is_null() {
            let mut URI: * mut u8 = 0 as *mut xmlChar;
            let mut base: * const i8 = 0 as *const i8;
            if !((*ctxt).input).is_null() {
                base = (*(*ctxt).input).filename;
            }
            if base.is_null() {
                base = (*ctxt).directory;
            }
            URI = xmlBuildURI(systemId, base as *const xmlChar);
            let ref mut fresh17 = (*ent).URI;
            *fresh17 = URI;
        }
    } else if (*ctxt).inSubset == 2 as i32 {
        ent = xmlAddDtdEntity(
            (*ctxt).myDoc,
            name,
            XML_EXTERNAL_GENERAL_UNPARSED_ENTITY as i32,
            publicId,
            systemId,
            notationName,
        );
        if ent.is_null() && (*ctxt).pedantic != 0 && !((*ctxt).sax).is_null()
            && ((*(*ctxt).sax).warning).is_some()
        {
            ((*(*ctxt).sax).warning)
                .expect(
                    "non-null function pointer",
                )(
                (*ctxt).userData,
                b"Entity(%s) already defined in the external subset\n\0" as *const u8
                    as *const i8,
                name,
            );
        }
        if !ent.is_null() && ((*ent).URI).is_null() && !systemId.is_null() {
            let mut URI_0: * mut u8 = 0 as *mut xmlChar;
            let mut base_0: * const i8 = 0 as *const i8;
            if !((*ctxt).input).is_null() {
                base_0 = (*(*ctxt).input).filename;
            }
            if base_0.is_null() {
                base_0 = (*ctxt).directory;
            }
            URI_0 = xmlBuildURI(systemId, base_0 as *const xmlChar);
            let ref mut fresh18 = (*ent).URI;
            *fresh18 = URI_0;
        }
    } else {
        xmlFatalErrMsg(
            ctxt,
            XML_ERR_INTERNAL_ERROR,
            b"SAX.xmlSAX2UnparsedEntityDecl(%s) called while not in subset\n\0"
                as *const u8 as *const i8,
            name,
            0 as *const xmlChar,
        );
    };
}
#[no_mangle]
pub extern "C" fn xmlSAX2SetDocumentLocator(
    mut ctx: * mut core::ffi::c_void,
    mut loc: * mut crate::src::HTMLparser::_xmlSAXLocator,
) {}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2StartDocument(mut ctx: * mut core::ffi::c_void) {
    let mut ctxt: * mut crate::src::HTMLparser::_xmlParserCtxt = ctx as xmlParserCtxtPtr;
    let mut doc: * mut crate::src::HTMLparser::_xmlDoc = 0 as *mut xmlDoc;
    if ctx.is_null() {
        return;
    }
    if (*ctxt).html != 0 {
        if ((*ctxt).myDoc).is_null() {
            let ref mut fresh19 = (*ctxt).myDoc;
            *fresh19 = htmlNewDocNoDtD(0 as *const xmlChar, 0 as *const xmlChar);
        }
        if ((*ctxt).myDoc).is_null() {
            xmlSAX2ErrMemory(
                ctxt,
                b"xmlSAX2StartDocument\0" as *const u8 as *const i8,
            );
            return;
        }
        (*(*ctxt).myDoc).properties = XML_DOC_HTML as i32;
        (*(*ctxt).myDoc).parseFlags = (*ctxt).options;
    } else {
        let ref mut fresh20 = (*ctxt).myDoc;
        *fresh20 = xmlNewDoc((*ctxt).version);
        doc = *fresh20;
        if !doc.is_null() {
            (*doc).properties = 0 as i32;
            if (*ctxt).options & XML_PARSE_OLD10 as i32 != 0 {
                (*doc).properties |= XML_DOC_OLD10 as i32;
            }
            (*doc).parseFlags = (*ctxt).options;
            if !((*ctxt).encoding).is_null() {
                let ref mut fresh21 = (*doc).encoding;
                *fresh21 = xmlStrdup((*ctxt).encoding);
            } else {
                let ref mut fresh22 = (*doc).encoding;
                *fresh22 = 0 as *const xmlChar;
            }
            (*doc).standalone = (*ctxt).standalone;
        } else {
            xmlSAX2ErrMemory(
                ctxt,
                b"xmlSAX2StartDocument\0" as *const u8 as *const i8,
            );
            return;
        }
        if (*ctxt).dictNames != 0 && !doc.is_null() {
            let ref mut fresh23 = (*doc).dict;
            *fresh23 = (*ctxt).dict;
            xmlDictReference((*doc).dict);
        }
    }
    if !((*ctxt).myDoc).is_null() && ((*(*ctxt).myDoc).URL).is_null()
        && !((*ctxt).input).is_null() && !((*(*ctxt).input).filename).is_null()
    {
        let ref mut fresh24 = (*(*ctxt).myDoc).URL;
        *fresh24 = xmlPathToURI((*(*ctxt).input).filename as *const xmlChar);
        if ((*(*ctxt).myDoc).URL).is_null() {
            xmlSAX2ErrMemory(
                ctxt,
                b"xmlSAX2StartDocument\0" as *const u8 as *const i8,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2EndDocument(mut ctx: * mut core::ffi::c_void) {
    let mut ctxt: * mut crate::src::HTMLparser::_xmlParserCtxt = ctx as xmlParserCtxtPtr;
    if ctx.is_null() {
        return;
    }
    if (*ctxt).validate != 0 && (*ctxt).wellFormed != 0 && !((*ctxt).myDoc).is_null()
        && !((*(*ctxt).myDoc).intSubset).is_null()
    {
        (*ctxt).valid &= xmlValidateDocumentFinal(&mut (*ctxt).vctxt, (*ctxt).myDoc);
    }
    if !((*ctxt).encoding).is_null() && !((*ctxt).myDoc).is_null()
        && ((*(*ctxt).myDoc).encoding).is_null()
    {
        let ref mut fresh25 = (*(*ctxt).myDoc).encoding;
        *fresh25 = (*ctxt).encoding;
        let ref mut fresh26 = (*ctxt).encoding;
        *fresh26 = 0 as *const xmlChar;
    }
    if !((*ctxt).inputTab).is_null() && (*ctxt).inputNr > 0 as i32
        && !(*((*ctxt).inputTab).offset(0 as i32 as isize)).is_null()
        && !((**((*ctxt).inputTab).offset(0 as i32 as isize)).encoding).is_null()
        && !((*ctxt).myDoc).is_null() && ((*(*ctxt).myDoc).encoding).is_null()
    {
        let ref mut fresh27 = (*(*ctxt).myDoc).encoding;
        *fresh27 = xmlStrdup(
            (**((*ctxt).inputTab).offset(0 as i32 as isize)).encoding,
        );
    }
    if (*ctxt).charset != XML_CHAR_ENCODING_NONE as i32
        && !((*ctxt).myDoc).is_null()
        && (*(*ctxt).myDoc).charset == XML_CHAR_ENCODING_NONE as i32
    {
        (*(*ctxt).myDoc).charset = (*ctxt).charset;
    }
}
unsafe extern "C" fn xmlNsErrMsg(
    mut ctxt: * mut crate::src::HTMLparser::_xmlParserCtxt,
    mut error: u32,
    mut msg: * const i8,
    mut str1: * const u8,
    mut str2: * const u8,
) {
    if !ctxt.is_null() && (*ctxt).disableSAX != 0 as i32
        && (*ctxt).instate as i32 == XML_PARSER_EOF as i32
    {
        return;
    }
    if !ctxt.is_null() {
        (*ctxt).errNo = error as i32;
    }
    __xmlRaiseError(
        None,
        None,
        0 as *mut libc::c_void,
        ctxt as *mut libc::c_void,
        0 as *mut libc::c_void,
        XML_FROM_NAMESPACE as i32,
        error as i32,
        XML_ERR_ERROR,
        0 as *const i8,
        0 as i32,
        str1 as *const i8,
        str2 as *const i8,
        0 as *const i8,
        0 as i32,
        0 as i32,
        msg,
        str1,
        str2,
    );
}
unsafe extern "C" fn xmlSAX2AttributeInternal(
    mut ctx: * mut core::ffi::c_void,
    mut fullname: * const u8,
    mut value: * const u8,
    mut prefix: * const u8,
) {
    let mut current_block: u64;
    let mut ctxt: * mut crate::src::HTMLparser::_xmlParserCtxt = ctx as xmlParserCtxtPtr;
    let mut ret: * mut crate::src::HTMLparser::_xmlAttr = 0 as *mut xmlAttr;
    let mut name: * mut u8 = 0 as *mut xmlChar;
    let mut ns: * mut u8 = 0 as *mut xmlChar;
    let mut nval: * mut u8 = 0 as *mut xmlChar;
    let mut namespace: * mut crate::src::HTMLparser::_xmlNs = 0 as *mut xmlNs;
    if (*ctxt).html != 0 {
        name = xmlStrdup(fullname);
        ns = 0 as *mut xmlChar;
        namespace = 0 as xmlNsPtr;
    } else {
        name = xmlSplitQName(ctxt, fullname, Some(&mut ns));
        if !name.is_null()
            && *name.offset(0 as i32 as isize) as i32 == 0 as i32
        {
            if xmlStrEqual(
                ns,
                b"xmlns\0" as *const u8 as *const i8 as *mut xmlChar,
            ) != 0
            {
                xmlNsErrMsg(
                    ctxt,
                    XML_ERR_NS_DECL_ERROR,
                    b"invalid namespace declaration '%s'\n\0" as *const u8
                        as *const i8,
                    fullname,
                    0 as *const xmlChar,
                );
            } else {
                xmlNsWarnMsg(
                    ctxt,
                    XML_WAR_NS_COLUMN,
                    b"Avoid attribute ending with ':' like '%s'\n\0" as *const u8
                        as *const i8,
                    fullname,
                    0 as *const xmlChar,
                );
            }
            if !ns.is_null() {
                xmlFree.expect("non-null function pointer")(ns as *mut libc::c_void);
            }
            ns = 0 as *mut xmlChar;
            xmlFree.expect("non-null function pointer")(name as *mut libc::c_void);
            name = xmlStrdup(fullname);
        }
    }
    if name.is_null() {
        xmlSAX2ErrMemory(
            ctxt,
            b"xmlSAX2StartElement\0" as *const u8 as *const i8,
        );
        if !ns.is_null() {
            xmlFree.expect("non-null function pointer")(ns as *mut libc::c_void);
        }
        return;
    }
    if (*ctxt).html != 0 && value.is_null() && htmlIsBooleanAttr(fullname) != 0 {
        nval = xmlStrdup(fullname);
        value = nval as *const xmlChar;
    } else {
        (*ctxt).vctxt.valid = 1 as i32;
        nval = xmlValidCtxtNormalizeAttributeValue(
            &mut (*ctxt).vctxt,
            (*ctxt).myDoc,
            (*ctxt).node,
            fullname,
            value,
        );
        if (*ctxt).vctxt.valid != 1 as i32 {
            (*ctxt).valid = 0 as i32;
        }
        if !nval.is_null() {
            value = nval;
        }
    }
    if (*ctxt).html == 0 && ns.is_null()
        && *name.offset(0 as i32 as isize) as i32 == 'x' as i32
        && *name.offset(1 as i32 as isize) as i32 == 'm' as i32
        && *name.offset(2 as i32 as isize) as i32 == 'l' as i32
        && *name.offset(3 as i32 as isize) as i32 == 'n' as i32
        && *name.offset(4 as i32 as isize) as i32 == 's' as i32
        && *name.offset(5 as i32 as isize) as i32 == 0 as i32
    {
        let mut nsret: * mut crate::src::HTMLparser::_xmlNs = 0 as *mut xmlNs;
        let mut val: * mut u8 = 0 as *mut xmlChar;
        if (*ctxt).replaceEntities == 0 {
            let ref mut fresh28 = (*ctxt).depth;
            *fresh28 += 1;
            val = xmlStringDecodeEntities(
                ctxt,
                value,
                1 as i32,
                0 as i32 as xmlChar,
                0 as i32 as xmlChar,
                0 as i32 as xmlChar,
            );
            let ref mut fresh29 = (*ctxt).depth;
            *fresh29 -= 1;
            if val.is_null() {
                xmlSAX2ErrMemory(
                    ctxt,
                    b"xmlSAX2StartElement\0" as *const u8 as *const i8,
                );
                if !name.is_null() {
                    xmlFree
                        .expect("non-null function pointer")(name as *mut libc::c_void);
                }
                if !nval.is_null() {
                    xmlFree
                        .expect("non-null function pointer")(nval as *mut libc::c_void);
                }
                return;
            }
        } else {
            val = value as *mut xmlChar;
        }
        if *val.offset(0 as i32 as isize) as i32 != 0 as i32 {
            let mut uri: * mut crate::src::SAX2::_xmlURI = 0 as *mut xmlURI;
            uri = xmlParseURI(val as *const i8);
            if uri.is_null() {
                if !((*ctxt).sax).is_null() && ((*(*ctxt).sax).warning).is_some() {
                    ((*(*ctxt).sax).warning)
                        .expect(
                            "non-null function pointer",
                        )(
                        (*ctxt).userData,
                        b"xmlns: %s not a valid URI\n\0" as *const u8
                            as *const i8,
                        val,
                    );
                }
            } else {
                if ((*uri).scheme).is_null() {
                    if !((*ctxt).sax).is_null() && ((*(*ctxt).sax).warning).is_some() {
                        ((*(*ctxt).sax).warning)
                            .expect(
                                "non-null function pointer",
                            )(
                            (*ctxt).userData,
                            b"xmlns: URI %s is not absolute\n\0" as *const u8
                                as *const i8,
                            val,
                        );
                    }
                }
                xmlFreeURI(uri);
            }
        }
        nsret = xmlNewNs((*ctxt).node, val, 0 as *const xmlChar);
        if !nsret.is_null() && (*ctxt).validate != 0 && (*ctxt).wellFormed != 0
            && !((*ctxt).myDoc).is_null() && !((*(*ctxt).myDoc).intSubset).is_null()
        {
            (*ctxt).valid
                &= xmlValidateOneNamespace(
                    &mut (*ctxt).vctxt,
                    (*ctxt).myDoc,
                    (*ctxt).node,
                    prefix,
                    nsret,
                    val,
                );
        }
        if !name.is_null() {
            xmlFree.expect("non-null function pointer")(name as *mut libc::c_void);
        }
        if !nval.is_null() {
            xmlFree.expect("non-null function pointer")(nval as *mut libc::c_void);
        }
        if val != value as *mut xmlChar {
            xmlFree.expect("non-null function pointer")(val as *mut libc::c_void);
        }
        return;
    }
    if (*ctxt).html == 0 && !ns.is_null()
        && *ns.offset(0 as i32 as isize) as i32 == 'x' as i32
        && *ns.offset(1 as i32 as isize) as i32 == 'm' as i32
        && *ns.offset(2 as i32 as isize) as i32 == 'l' as i32
        && *ns.offset(3 as i32 as isize) as i32 == 'n' as i32
        && *ns.offset(4 as i32 as isize) as i32 == 's' as i32
        && *ns.offset(5 as i32 as isize) as i32 == 0 as i32
    {
        let mut nsret_0: * mut crate::src::HTMLparser::_xmlNs = 0 as *mut xmlNs;
        let mut val_0: * mut u8 = 0 as *mut xmlChar;
        if (*ctxt).replaceEntities == 0 {
            let ref mut fresh30 = (*ctxt).depth;
            *fresh30 += 1;
            val_0 = xmlStringDecodeEntities(
                ctxt,
                value,
                1 as i32,
                0 as i32 as xmlChar,
                0 as i32 as xmlChar,
                0 as i32 as xmlChar,
            );
            let ref mut fresh31 = (*ctxt).depth;
            *fresh31 -= 1;
            if val_0.is_null() {
                xmlSAX2ErrMemory(
                    ctxt,
                    b"xmlSAX2StartElement\0" as *const u8 as *const i8,
                );
                xmlFree.expect("non-null function pointer")(ns as *mut libc::c_void);
                if !name.is_null() {
                    xmlFree
                        .expect("non-null function pointer")(name as *mut libc::c_void);
                }
                if !nval.is_null() {
                    xmlFree
                        .expect("non-null function pointer")(nval as *mut libc::c_void);
                }
                return;
            }
        } else {
            val_0 = value as *mut xmlChar;
        }
        if *val_0.offset(0 as i32 as isize) as i32 == 0 as i32 {
            xmlNsErrMsg(
                ctxt,
                XML_NS_ERR_EMPTY,
                b"Empty namespace name for prefix %s\n\0" as *const u8
                    as *const i8,
                name,
                0 as *const xmlChar,
            );
        }
        if (*ctxt).pedantic != 0 as i32
            && *val_0.offset(0 as i32 as isize) as i32
                != 0 as i32
        {
            let mut uri_0: * mut crate::src::SAX2::_xmlURI = 0 as *mut xmlURI;
            uri_0 = xmlParseURI(val_0 as *const i8);
            if uri_0.is_null() {
                xmlNsWarnMsg(
                    ctxt,
                    XML_WAR_NS_URI,
                    b"xmlns:%s: %s not a valid URI\n\0" as *const u8
                        as *const i8,
                    name,
                    value,
                );
            } else {
                if ((*uri_0).scheme).is_null() {
                    xmlNsWarnMsg(
                        ctxt,
                        XML_WAR_NS_URI_RELATIVE,
                        b"xmlns:%s: URI %s is not absolute\n\0" as *const u8
                            as *const i8,
                        name,
                        value,
                    );
                }
                xmlFreeURI(uri_0);
            }
        }
        nsret_0 = xmlNewNs((*ctxt).node, val_0, name);
        xmlFree.expect("non-null function pointer")(ns as *mut libc::c_void);
        if !nsret_0.is_null() && (*ctxt).validate != 0 && (*ctxt).wellFormed != 0
            && !((*ctxt).myDoc).is_null() && !((*(*ctxt).myDoc).intSubset).is_null()
        {
            (*ctxt).valid
                &= xmlValidateOneNamespace(
                    &mut (*ctxt).vctxt,
                    (*ctxt).myDoc,
                    (*ctxt).node,
                    prefix,
                    nsret_0,
                    value,
                );
        }
        if !name.is_null() {
            xmlFree.expect("non-null function pointer")(name as *mut libc::c_void);
        }
        if !nval.is_null() {
            xmlFree.expect("non-null function pointer")(nval as *mut libc::c_void);
        }
        if val_0 != value as *mut xmlChar {
            xmlFree.expect("non-null function pointer")(val_0 as *mut libc::c_void);
        }
        return;
    }
    if !ns.is_null() {
        namespace = xmlSearchNs((*ctxt).myDoc, (*ctxt).node, ns);
        if namespace.is_null() {
            xmlNsErrMsg(
                ctxt,
                XML_NS_ERR_UNDEFINED_NAMESPACE,
                b"Namespace prefix %s of attribute %s is not defined\n\0" as *const u8
                    as *const i8,
                ns,
                name,
            );
            current_block = 13425230902034816933;
        } else {
            let mut prop: * mut crate::src::HTMLparser::_xmlAttr = 0 as *mut xmlAttr;
            prop = (*(*ctxt).node).properties;
            loop {
                if prop.is_null() {
                    current_block = 13425230902034816933;
                    break;
                }
                if !((*prop).ns).is_null() {
                    if xmlStrEqual(name, (*prop).name) != 0
                        && (namespace == (*prop).ns
                            || xmlStrEqual((*namespace).href, (*(*prop).ns).href) != 0)
                    {
                        xmlNsErrMsg(
                            ctxt,
                            XML_ERR_ATTRIBUTE_REDEFINED,
                            b"Attribute %s in %s redefined\n\0" as *const u8
                                as *const i8,
                            name,
                            (*namespace).href,
                        );
                        (*ctxt).wellFormed = 0 as i32;
                        if (*ctxt).recovery == 0 as i32 {
                            (*ctxt).disableSAX = 1 as i32;
                        }
                        if !name.is_null() {
                            xmlFree
                                .expect(
                                    "non-null function pointer",
                                )(name as *mut libc::c_void);
                        }
                        current_block = 6621008398210872980;
                        break;
                    }
                }
                prop = (*prop).next;
            }
        }
    } else {
        namespace = 0 as xmlNsPtr;
        current_block = 13425230902034816933;
    }
    match current_block {
        13425230902034816933 => {
            ret = xmlNewNsPropEatName(
                (*ctxt).node,
                namespace,
                name,
                0 as *const xmlChar,
            );
            if !ret.is_null() {
                if (*ctxt).replaceEntities == 0 as i32 && (*ctxt).html == 0 {
                    let mut tmp: * mut crate::src::HTMLparser::_xmlNode = 0 as *mut xmlNode;
                    let ref mut fresh32 = (*ret).children;
                    *fresh32 = xmlStringGetNodeList(
                        (*ctxt).myDoc as *const xmlDoc,
                        value,
                    );
                    tmp = (*ret).children;
                    while !tmp.is_null() {
                        let ref mut fresh33 = (*tmp).parent;
                        *fresh33 = ret as xmlNodePtr;
                        if ((*tmp).next).is_null() {
                            let ref mut fresh34 = (*ret).last;
                            *fresh34 = tmp;
                        }
                        tmp = (*tmp).next;
                    }
                } else if !value.is_null() {
                    let ref mut fresh35 = (*ret).children;
                    *fresh35 = xmlNewDocText((*ctxt).myDoc as *const xmlDoc, value);
                    let ref mut fresh36 = (*ret).last;
                    *fresh36 = (*ret).children;
                    if !((*ret).children).is_null() {
                        let ref mut fresh37 = (*(*ret).children).parent;
                        *fresh37 = ret as xmlNodePtr;
                    }
                }
            }
            if (*ctxt).html == 0 && (*ctxt).validate != 0 && (*ctxt).wellFormed != 0
                && !((*ctxt).myDoc).is_null() && !((*(*ctxt).myDoc).intSubset).is_null()
            {
                if (*ctxt).replaceEntities == 0 {
                    let mut val_1: * mut u8 = 0 as *mut xmlChar;
                    let ref mut fresh38 = (*ctxt).depth;
                    *fresh38 += 1;
                    val_1 = xmlStringDecodeEntities(
                        ctxt,
                        value,
                        1 as i32,
                        0 as i32 as xmlChar,
                        0 as i32 as xmlChar,
                        0 as i32 as xmlChar,
                    );
                    let ref mut fresh39 = (*ctxt).depth;
                    *fresh39 -= 1;
                    if val_1.is_null() {
                        (*ctxt).valid
                            &= xmlValidateOneAttribute(
                                &mut (*ctxt).vctxt,
                                (*ctxt).myDoc,
                                (*ctxt).node,
                                ret,
                                value,
                            );
                    } else {
                        let mut nvalnorm: * mut u8 = 0 as *mut xmlChar;
                        nvalnorm = xmlValidNormalizeAttributeValue(
                            (*ctxt).myDoc,
                            (*ctxt).node,
                            fullname,
                            val_1,
                        );
                        if !nvalnorm.is_null() {
                            xmlFree
                                .expect(
                                    "non-null function pointer",
                                )(val_1 as *mut libc::c_void);
                            val_1 = nvalnorm;
                        }
                        (*ctxt).valid
                            &= xmlValidateOneAttribute(
                                &mut (*ctxt).vctxt,
                                (*ctxt).myDoc,
                                (*ctxt).node,
                                ret,
                                val_1,
                            );
                        xmlFree
                            .expect(
                                "non-null function pointer",
                            )(val_1 as *mut libc::c_void);
                    }
                } else {
                    (*ctxt).valid
                        &= xmlValidateOneAttribute(
                            &mut (*ctxt).vctxt,
                            (*ctxt).myDoc,
                            (*ctxt).node,
                            ret,
                            value,
                        );
                }
            } else if (*ctxt).loadsubset & 8 as i32 == 0 as i32
                    && ((*ctxt).replaceEntities == 0 as i32
                        && (*ctxt).external != 2 as i32
                        || (*ctxt).replaceEntities != 0 as i32
                            && (*ctxt).inSubset == 0 as i32)
                    && !((*ret).children).is_null()
                    && (*(*ret).children).type_0 as u32
                        == XML_TEXT_NODE as i32 as u32
                    && ((*(*ret).children).next).is_null()
                {
                let mut content: * mut u8 = (*(*ret).children).content;
                if xmlStrEqual(
                    fullname,
                    b"xml:id\0" as *const u8 as *const i8 as *mut xmlChar,
                ) != 0
                {
                    if xmlValidateNCName(content, 1 as i32) != 0 as i32 {
                        xmlErrValid(
                            ctxt,
                            XML_DTD_XMLID_VALUE,
                            b"xml:id : attribute value %s is not an NCName\n\0"
                                as *const u8 as *const i8,
                            content as *const i8,
                            0 as *const i8,
                        );
                    }
                    xmlAddID(&mut (*ctxt).vctxt, (*ctxt).myDoc, content, ret);
                } else if xmlIsID((*ctxt).myDoc, (*ctxt).node, ret) != 0 {
                    xmlAddID(&mut (*ctxt).vctxt, (*ctxt).myDoc, content, ret);
                } else if xmlIsRef((*ctxt).myDoc, (*ctxt).node, ret) != 0 {
                    xmlAddRef(&mut (*ctxt).vctxt, (*ctxt).myDoc, content, ret);
                }
            }
        }
        _ => {}
    }
    if !nval.is_null() {
        xmlFree.expect("non-null function pointer")(nval as *mut libc::c_void);
    }
    if !ns.is_null() {
        xmlFree.expect("non-null function pointer")(ns as *mut libc::c_void);
    }
}
unsafe extern "C" fn xmlCheckDefaultedAttributes(
    mut ctxt: * mut crate::src::HTMLparser::_xmlParserCtxt,
    mut name: * const u8,
    mut prefix: * const u8,
    mut atts: * mut * const u8,
) {
    let mut elemDecl: * mut crate::src::SAX2::_xmlElement = (0 as * mut crate::src::SAX2::_xmlElement);
    let mut att: * const u8 = 0 as *const xmlChar;
    let mut internal: i32 = 1 as i32;
    let mut i: i32 = 0;
    elemDecl = xmlGetDtdQElementDesc((*(*ctxt).myDoc).intSubset, name, prefix);
    if elemDecl.is_null() {
        elemDecl = xmlGetDtdQElementDesc((*(*ctxt).myDoc).extSubset, name, prefix);
        internal = 0 as i32;
    }
    while !elemDecl.is_null() {
        let mut attr: * mut crate::src::SAX2::_xmlAttribute = (*elemDecl).attributes;
        if (*(*ctxt).myDoc).standalone == 1 as i32
            && !((*(*ctxt).myDoc).extSubset).is_null() && (*ctxt).validate != 0
        {
            while !attr.is_null() {
                if !((*attr).defaultValue).is_null()
                    && xmlGetDtdQAttrDesc(
                        (*(*ctxt).myDoc).extSubset,
                        (*attr).elem,
                        (*attr).name,
                        (*attr).prefix,
                    ) == attr
                    && (xmlGetDtdQAttrDesc(
                        (*(*ctxt).myDoc).intSubset,
                        (*attr).elem,
                        (*attr).name,
                        (*attr).prefix,
                    ))
                        .is_null()
                {
                    let mut fulln: * mut u8 = 0 as *mut xmlChar;
                    if !((*attr).prefix).is_null() {
                        fulln = xmlStrdup((*attr).prefix);
                        fulln = xmlStrcat(
                            fulln,
                            b":\0" as *const u8 as *const i8 as *mut xmlChar,
                        );
                        fulln = xmlStrcat(fulln, (*attr).name);
                    } else {
                        fulln = xmlStrdup((*attr).name);
                    }
                    if fulln.is_null() {
                        xmlSAX2ErrMemory(
                            ctxt,
                            b"xmlSAX2StartElement\0" as *const u8 as *const i8,
                        );
                        break;
                    } else {
                        att = 0 as *const xmlChar;
                        if !atts.is_null() {
                            i = 0 as i32;
                            att = *atts.offset(i as isize);
                            while !att.is_null() {
                                if xmlStrEqual(att, fulln) != 0 {
                                    break;
                                }
                                i += 2 as i32;
                                att = *atts.offset(i as isize);
                            }
                        }
                        if att.is_null() {
                            xmlErrValid(
                                ctxt,
                                XML_DTD_STANDALONE_DEFAULTED,
                                b"standalone: attribute %s on %s defaulted from external subset\n\0"
                                    as *const u8 as *const i8,
                                fulln as *const i8,
                                (*attr).elem as *const i8,
                            );
                        }
                        xmlFree
                            .expect(
                                "non-null function pointer",
                            )(fulln as *mut libc::c_void);
                    }
                }
                attr = (*attr).nexth;
            }
        }
        attr = (*elemDecl).attributes;
        while !attr.is_null() {
            if !((*attr).defaultValue).is_null() {
                if !((*attr).prefix).is_null()
                    && xmlStrEqual(
                        (*attr).prefix,
                        b"xmlns\0" as *const u8 as *const i8 as *mut xmlChar,
                    ) != 0
                    || ((*attr).prefix).is_null()
                        && xmlStrEqual(
                            (*attr).name,
                            b"xmlns\0" as *const u8 as *const i8
                                as *mut xmlChar,
                        ) != 0 || (*ctxt).loadsubset & 4 as i32 != 0
                {
                    let mut tst: * mut crate::src::SAX2::_xmlAttribute = 0 as *mut xmlAttribute;
                    tst = xmlGetDtdQAttrDesc(
                        (*(*ctxt).myDoc).intSubset,
                        (*attr).elem,
                        (*attr).name,
                        (*attr).prefix,
                    );
                    if tst == attr || tst.is_null() {
                        let mut fn_0: [u8; 50] = [0; 50];
                        let mut fulln_0: * mut u8 = 0 as *mut xmlChar;
                        fulln_0 = xmlBuildQName(
                            (*attr).name,
                            (*attr).prefix,
                            fn_0.as_mut_ptr(),
                            50 as i32,
                        );
                        if fulln_0.is_null() {
                            xmlSAX2ErrMemory(
                                ctxt,
                                b"xmlSAX2StartElement\0" as *const u8 as *const i8,
                            );
                            return;
                        }
                        att = 0 as *const xmlChar;
                        if !atts.is_null() {
                            i = 0 as i32;
                            att = *atts.offset(i as isize);
                            while !att.is_null() {
                                if xmlStrEqual(att, fulln_0) != 0 {
                                    break;
                                }
                                i += 2 as i32;
                                att = *atts.offset(i as isize);
                            }
                        }
                        if att.is_null() {
                            xmlSAX2AttributeInternal(
                                ctxt as *mut libc::c_void,
                                fulln_0,
                                (*attr).defaultValue,
                                prefix,
                            );
                        }
                        if fulln_0 != fn_0.as_mut_ptr()
                            && fulln_0 != (*attr).name as *mut xmlChar
                        {
                            xmlFree
                                .expect(
                                    "non-null function pointer",
                                )(fulln_0 as *mut libc::c_void);
                        }
                    }
                }
            }
            attr = (*attr).nexth;
        }
        if !(internal == 1 as i32) {
            break;
        }
        elemDecl = xmlGetDtdQElementDesc((*(*ctxt).myDoc).extSubset, name, prefix);
        internal = 0 as i32;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2StartElement(
    mut ctx: * mut core::ffi::c_void,
    mut fullname: * const u8,
    mut atts: * mut * const u8,
) {
    let mut ctxt: * mut crate::src::HTMLparser::_xmlParserCtxt = ctx as xmlParserCtxtPtr;
    let mut ret: * mut crate::src::HTMLparser::_xmlNode = 0 as *mut xmlNode;
    let mut parent: * mut crate::src::HTMLparser::_xmlNode = 0 as *mut xmlNode;
    let mut ns: * mut crate::src::HTMLparser::_xmlNs = 0 as *mut xmlNs;
    let mut name: * mut u8 = 0 as *mut xmlChar;
    let mut prefix: * mut u8 = 0 as *mut xmlChar;
    let mut att: * const u8 = 0 as *const xmlChar;
    let mut value: * const u8 = 0 as *const xmlChar;
    let mut i: i32 = 0;
    if ctx.is_null() || fullname.is_null() || ((*ctxt).myDoc).is_null() {
        return;
    }
    parent = (*ctxt).node;
    if (*ctxt).validate != 0 && ((*(*ctxt).myDoc).extSubset).is_null()
        && (((*(*ctxt).myDoc).intSubset).is_null()
            || ((*(*(*ctxt).myDoc).intSubset).notations).is_null()
                && ((*(*(*ctxt).myDoc).intSubset).elements).is_null()
                && ((*(*(*ctxt).myDoc).intSubset).attributes).is_null()
                && ((*(*(*ctxt).myDoc).intSubset).entities).is_null())
    {
        xmlErrValid(
            ctxt,
            XML_ERR_NO_DTD,
            b"Validation failed: no DTD found !\0" as *const u8 as *const i8,
            0 as *const i8,
            0 as *const i8,
        );
        (*ctxt).validate = 0 as i32;
    }
    name = xmlSplitQName(ctxt, fullname, Some(&mut prefix));
    ret = xmlNewDocNodeEatName((*ctxt).myDoc, 0 as xmlNsPtr, name, 0 as *const xmlChar);
    if ret.is_null() {
        if !prefix.is_null() {
            xmlFree.expect("non-null function pointer")(prefix as *mut libc::c_void);
        }
        xmlSAX2ErrMemory(
            ctxt,
            b"xmlSAX2StartElement\0" as *const u8 as *const i8,
        );
        return;
    }
    if ((*(*ctxt).myDoc).children).is_null() {
        xmlAddChild((*ctxt).myDoc as xmlNodePtr, ret);
    } else if parent.is_null() {
        parent = (*(*ctxt).myDoc).children;
    }
    (*ctxt).nodemem = -(1 as i32);
    if (*ctxt).linenumbers != 0 {
        if !((*ctxt).input).is_null() {
            if (*(*ctxt).input).line
                < 32767 as i32 * 2 as i32 + 1 as i32
            {
                (*ret).line = (*(*ctxt).input).line as u16;
            } else {
                (*ret)
                    .line = (32767 as i32 * 2 as i32 + 1 as i32)
                    as u16;
            }
        }
    }
    if nodePush(ctxt, ret) < 0 as i32 {
        xmlUnlinkNode(ret);
        xmlFreeNode(ret);
        if !prefix.is_null() {
            xmlFree.expect("non-null function pointer")(prefix as *mut libc::c_void);
        }
        return;
    }
    if !parent.is_null() {
        if (*parent).type_0 as u32
            == XML_ELEMENT_NODE as i32 as u32
        {
            xmlAddChild(parent, ret);
        } else {
            xmlAddSibling(parent, ret);
        }
    }
    if (*ctxt).html == 0 {
        if !((*(*ctxt).myDoc).intSubset).is_null()
            || !((*(*ctxt).myDoc).extSubset).is_null()
        {
            xmlCheckDefaultedAttributes(ctxt, name, prefix, atts);
        }
        if !atts.is_null() {
            i = 0 as i32;
            let mut fresh40 = i;
            i = i + 1;
            att = *atts.offset(fresh40 as isize);
            let mut fresh41 = i;
            i = i + 1;
            value = *atts.offset(fresh41 as isize);
            while !att.is_null() && !value.is_null() {
                if *att.offset(0 as i32 as isize) as i32 == 'x' as i32
                    && *att.offset(1 as i32 as isize) as i32
                        == 'm' as i32
                    && *att.offset(2 as i32 as isize) as i32
                        == 'l' as i32
                    && *att.offset(3 as i32 as isize) as i32
                        == 'n' as i32
                    && *att.offset(4 as i32 as isize) as i32
                        == 's' as i32
                {
                    xmlSAX2AttributeInternal(
                        ctxt as *mut libc::c_void,
                        att,
                        value,
                        prefix,
                    );
                }
                let mut fresh42 = i;
                i = i + 1;
                att = *atts.offset(fresh42 as isize);
                let mut fresh43 = i;
                i = i + 1;
                value = *atts.offset(fresh43 as isize);
            }
        }
        ns = xmlSearchNs((*ctxt).myDoc, ret, prefix);
        if ns.is_null() && !parent.is_null() {
            ns = xmlSearchNs((*ctxt).myDoc, parent, prefix);
        }
        if !prefix.is_null() && ns.is_null() {
            ns = xmlNewNs(ret, 0 as *const xmlChar, prefix);
            xmlNsWarnMsg(
                ctxt,
                XML_NS_ERR_UNDEFINED_NAMESPACE,
                b"Namespace prefix %s is not defined\n\0" as *const u8
                    as *const i8,
                prefix,
                0 as *const xmlChar,
            );
        }
        if !ns.is_null() && !((*ns).href).is_null()
            && (*((*ns).href).offset(0 as i32 as isize) as i32
                != 0 as i32 || !((*ns).prefix).is_null())
        {
            xmlSetNs(ret, ns);
        }
    }
    if !atts.is_null() {
        i = 0 as i32;
        let mut fresh44 = i;
        i = i + 1;
        att = *atts.offset(fresh44 as isize);
        let mut fresh45 = i;
        i = i + 1;
        value = *atts.offset(fresh45 as isize);
        if (*ctxt).html != 0 {
            while !att.is_null() {
                xmlSAX2AttributeInternal(
                    ctxt as *mut libc::c_void,
                    att,
                    value,
                    0 as *const xmlChar,
                );
                let mut fresh46 = i;
                i = i + 1;
                att = *atts.offset(fresh46 as isize);
                let mut fresh47 = i;
                i = i + 1;
                value = *atts.offset(fresh47 as isize);
            }
        } else {
            while !att.is_null() && !value.is_null() {
                if *att.offset(0 as i32 as isize) as i32 != 'x' as i32
                    || *att.offset(1 as i32 as isize) as i32
                        != 'm' as i32
                    || *att.offset(2 as i32 as isize) as i32
                        != 'l' as i32
                    || *att.offset(3 as i32 as isize) as i32
                        != 'n' as i32
                    || *att.offset(4 as i32 as isize) as i32
                        != 's' as i32
                {
                    xmlSAX2AttributeInternal(
                        ctxt as *mut libc::c_void,
                        att,
                        value,
                        0 as *const xmlChar,
                    );
                }
                let mut fresh48 = i;
                i = i + 1;
                att = *atts.offset(fresh48 as isize);
                let mut fresh49 = i;
                i = i + 1;
                value = *atts.offset(fresh49 as isize);
            }
        }
    }
    if (*ctxt).validate != 0
        && (*ctxt).vctxt.flags & (1 as u32) << 0 as i32
            == 0 as i32 as u32
    {
        let mut chk: i32 = 0;
        chk = xmlValidateDtdFinal(&mut (*ctxt).vctxt, (*ctxt).myDoc);
        if chk <= 0 as i32 {
            (*ctxt).valid = 0 as i32;
        }
        if chk < 0 as i32 {
            (*ctxt).wellFormed = 0 as i32;
        }
        (*ctxt).valid &= xmlValidateRoot(&mut (*ctxt).vctxt, (*ctxt).myDoc);
        (*ctxt).vctxt.flags |= (1 as u32) << 0 as i32;
    }
    if !prefix.is_null() {
        xmlFree.expect("non-null function pointer")(prefix as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2EndElement(
    mut ctx: * mut core::ffi::c_void,
    mut name: * const u8,
) {
    let mut ctxt: * mut crate::src::HTMLparser::_xmlParserCtxt = ctx as xmlParserCtxtPtr;
    let mut cur: * mut crate::src::HTMLparser::_xmlNode = 0 as *mut xmlNode;
    if ctx.is_null() {
        return;
    }
    cur = (*ctxt).node;
    if !cur.is_null() && (*ctxt).record_info != 0 {
        (*(*ctxt).nodeInfo)
            .end_pos = ((*(*ctxt).input).cur).offset_from((*(*ctxt).input).base)
            as i64 as u64;
        (*(*ctxt).nodeInfo).end_line = (*(*ctxt).input).line as u64;
        let ref mut fresh50 = (*(*ctxt).nodeInfo).node;
        *fresh50 = cur as *const _xmlNode;
        xmlParserAddNodeInfo(ctxt, (*ctxt).nodeInfo);
    }
    (*ctxt).nodemem = -(1 as i32);
    if (*ctxt).validate != 0 && (*ctxt).wellFormed != 0 && !((*ctxt).myDoc).is_null()
        && !((*(*ctxt).myDoc).intSubset).is_null()
    {
        (*ctxt).valid &= xmlValidateOneElement(&mut (*ctxt).vctxt, (*ctxt).myDoc, cur);
    }
    nodePop(ctxt);
}
unsafe extern "C" fn xmlSAX2TextNode(
    mut ctxt: * mut crate::src::HTMLparser::_xmlParserCtxt,
    mut str: * const u8,
    mut len: i32,
) -> * mut crate::src::HTMLparser::_xmlNode {
    let mut current_block: u64;
    let mut ret: * mut crate::src::HTMLparser::_xmlNode = 0 as *mut xmlNode;
    let mut intern: * const u8 = 0 as *const xmlChar;
    if !((*ctxt).freeElems).is_null() {
        ret = (*ctxt).freeElems;
        let ref mut fresh51 = (*ctxt).freeElems;
        *fresh51 = (*ret).next;
        let ref mut fresh52 = (*ctxt).freeElemsNr;
        *fresh52 -= 1;
    } else {
        ret = xmlMalloc
            .expect(
                "non-null function pointer",
            )(::std::mem::size_of::<xmlNode>() as u64) as xmlNodePtr;
    }
    if ret.is_null() {
        xmlErrMemory(ctxt, b"xmlSAX2Characters\0" as *const u8 as *const i8);
        return 0 as xmlNodePtr;
    }
    memset(
        ret as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<xmlNode>() as u64,
    );
    if (*ctxt).dictNames != 0 {
        let mut cur: u8 = *str.offset(len as isize);
        if len
            < (2 as i32 as u64)
                .wrapping_mul(
                    ::std::mem::size_of::<*mut libc::c_void>() as u64,
                ) as i32
            && (*ctxt).options & XML_PARSE_COMPACT as i32 != 0
        {
            let mut tmp: * mut u8 = &mut (*ret).properties as *mut *mut _xmlAttr
                as *mut xmlChar;
            memcpy(
                tmp as *mut libc::c_void,
                str as *const libc::c_void,
                len as u64,
            );
            *tmp.offset(len as isize) = 0 as i32 as xmlChar;
            intern = tmp;
        } else if len <= 3 as i32
                && (cur as i32 == '"' as i32 || cur as i32 == '\'' as i32
                    || cur as i32 == '<' as i32
                        && *str.offset((len + 1 as i32) as isize) as i32
                            != '!' as i32)
            {
            intern = xmlDictLookup((*ctxt).dict, str, len);
        } else if (*str as i32 == 0x20 as i32
                || 0x9 as i32 <= *str as i32
                    && *str as i32 <= 0xa as i32
                || *str as i32 == 0xd as i32) && len < 60 as i32
                && cur as i32 == '<' as i32
                && *str.offset((len + 1 as i32) as isize) as i32
                    != '!' as i32
            {
            let mut i: i32 = 0;
            i = 1 as i32;
            loop {
                if !(i < len) {
                    current_block = 17478428563724192186;
                    break;
                }
                if !(*str.offset(i as isize) as i32 == 0x20 as i32
                    || 0x9 as i32 <= *str.offset(i as isize) as i32
                        && *str.offset(i as isize) as i32 <= 0xa as i32
                    || *str.offset(i as isize) as i32 == 0xd as i32)
                {
                    current_block = 8377240654589692732;
                    break;
                }
                i += 1;
            }
            match current_block {
                8377240654589692732 => {}
                _ => {
                    intern = xmlDictLookup((*ctxt).dict, str, len);
                }
            }
        }
    }
    (*ret).type_0 = XML_TEXT_NODE;
    let ref mut fresh53 = (*ret).name;
    *fresh53 = xmlStringText.as_ptr();
    if intern.is_null() {
        let ref mut fresh54 = (*ret).content;
        *fresh54 = xmlStrndup(str, len);
        if ((*ret).content).is_null() {
            xmlSAX2ErrMemory(
                ctxt,
                b"xmlSAX2TextNode\0" as *const u8 as *const i8,
            );
            xmlFree.expect("non-null function pointer")(ret as *mut libc::c_void);
            return 0 as xmlNodePtr;
        }
    } else {
        let ref mut fresh55 = (*ret).content;
        *fresh55 = intern as *mut xmlChar;
    }
    if (*ctxt).linenumbers != 0 {
        if !((*ctxt).input).is_null() {
            if (*(*ctxt).input).line
                < 32767 as i32 * 2 as i32 + 1 as i32
            {
                (*ret).line = (*(*ctxt).input).line as u16;
            } else {
                (*ret)
                    .line = (32767 as i32 * 2 as i32 + 1 as i32)
                    as u16;
                if (*ctxt).options & XML_PARSE_BIG_LINES as i32 != 0 {
                    let ref mut fresh56 = (*ret).psvi;
                    *fresh56 = (*(*ctxt).input).line as ptrdiff_t as *mut libc::c_void;
                }
            }
        }
    }
    if __xmlRegisterCallbacks != 0 && (*(borrow(& __xmlRegisterNodeDefaultValue())).unwrap()).is_some() {
        (*(borrow(& __xmlRegisterNodeDefaultValue())).unwrap()).expect("non-null function pointer")(ret);
    }
    return ret;
}
unsafe extern "C" fn xmlSAX2DecodeAttrEntities(
    mut ctxt: * mut crate::src::HTMLparser::_xmlParserCtxt,
    mut str: * const u8,
    mut end: * const u8,
) -> * mut u8 {
    let mut current_block: u64;
    let mut in_0: * const u8 = 0 as *const xmlChar;
    let mut ret: * mut u8 = 0 as *mut xmlChar;
    in_0 = str;
    loop {
        if !(in_0 < end) {
            current_block = 17778012151635330486;
            break;
        }
        let mut fresh57 = in_0;
        in_0 = in_0.offset(1);
        if *fresh57 as i32 == '&' as i32 {
            current_block = 14709023407236833147;
            break;
        }
    }
    match current_block {
        17778012151635330486 => return 0 as *mut xmlChar,
        _ => {
            let ref mut fresh58 = (*ctxt).depth;
            *fresh58 += 1;
            ret = xmlStringLenDecodeEntities(
                ctxt,
                str,
                end.offset_from(str) as i64 as i32,
                1 as i32,
                0 as i32 as xmlChar,
                0 as i32 as xmlChar,
                0 as i32 as xmlChar,
            );
            let ref mut fresh59 = (*ctxt).depth;
            *fresh59 -= 1;
            return ret;
        }
    };
}
unsafe extern "C" fn xmlSAX2AttributeNs(
    mut ctxt: * mut crate::src::HTMLparser::_xmlParserCtxt,
    mut localname: * const u8,
    mut prefix: * const u8,
    mut value: * const u8,
    mut valueend: * const u8,
) {
    let mut ret: * mut crate::src::HTMLparser::_xmlAttr = 0 as *mut xmlAttr;
    let mut namespace: * mut crate::src::HTMLparser::_xmlNs = 0 as xmlNsPtr;
    let mut dup: * mut u8 = 0 as *mut xmlChar;
    if !prefix.is_null() {
        namespace = xmlSearchNs((*ctxt).myDoc, (*ctxt).node, prefix);
    }
    if !((*ctxt).freeAttrs).is_null() {
        ret = (*ctxt).freeAttrs;
        let ref mut fresh60 = (*ctxt).freeAttrs;
        *fresh60 = (*ret).next;
        let ref mut fresh61 = (*ctxt).freeAttrsNr;
        *fresh61 -= 1;
        memset(
            ret as *mut libc::c_void,
            0 as i32,
            ::std::mem::size_of::<xmlAttr>() as u64,
        );
        (*ret).type_0 = XML_ATTRIBUTE_NODE;
        let ref mut fresh62 = (*ret).parent;
        *fresh62 = (*ctxt).node;
        let ref mut fresh63 = (*ret).doc;
        *fresh63 = (*ctxt).myDoc;
        let ref mut fresh64 = (*ret).ns;
        *fresh64 = namespace;
        if (*ctxt).dictNames != 0 {
            let ref mut fresh65 = (*ret).name;
            *fresh65 = localname;
        } else {
            let ref mut fresh66 = (*ret).name;
            *fresh66 = xmlStrdup(localname);
        }
        if ((*(*ctxt).node).properties).is_null() {
            let ref mut fresh67 = (*(*ctxt).node).properties;
            *fresh67 = ret;
        } else {
            let mut prev: * mut crate::src::HTMLparser::_xmlAttr = (*(*ctxt).node).properties;
            while !((*prev).next).is_null() {
                prev = (*prev).next;
            }
            let ref mut fresh68 = (*prev).next;
            *fresh68 = ret;
            let ref mut fresh69 = (*ret).prev;
            *fresh69 = prev;
        }
        if __xmlRegisterCallbacks != 0 && (*(borrow(& __xmlRegisterNodeDefaultValue())).unwrap()).is_some() {
            (*(borrow(& __xmlRegisterNodeDefaultValue())).unwrap())
                .expect("non-null function pointer")(ret as xmlNodePtr);
        }
    } else {
        if (*ctxt).dictNames != 0 {
            ret = xmlNewNsPropEatName(
                (*ctxt).node,
                namespace,
                localname as *mut xmlChar,
                0 as *const xmlChar,
            );
        } else {
            ret = xmlNewNsProp((*ctxt).node, namespace, localname, 0 as *const xmlChar);
        }
        if ret.is_null() {
            xmlErrMemory(
                ctxt,
                b"xmlSAX2AttributeNs\0" as *const u8 as *const i8,
            );
            return;
        }
    }
    if (*ctxt).replaceEntities == 0 as i32 && (*ctxt).html == 0 {
        let mut tmp: * mut crate::src::HTMLparser::_xmlNode = 0 as *mut xmlNode;
        if *valueend as i32 != 0 as i32 {
            tmp = xmlSAX2TextNode(
                ctxt,
                value,
                valueend.offset_from(value) as i64 as i32,
            );
            let ref mut fresh70 = (*ret).children;
            *fresh70 = tmp;
            let ref mut fresh71 = (*ret).last;
            *fresh71 = tmp;
            if !tmp.is_null() {
                let ref mut fresh72 = (*tmp).doc;
                *fresh72 = (*ret).doc;
                let ref mut fresh73 = (*tmp).parent;
                *fresh73 = ret as xmlNodePtr;
            }
        } else {
            let ref mut fresh74 = (*ret).children;
            *fresh74 = xmlStringLenGetNodeList(
                (*ctxt).myDoc as *const xmlDoc,
                value,
                valueend.offset_from(value) as i64 as i32,
            );
            tmp = (*ret).children;
            while !tmp.is_null() {
                let ref mut fresh75 = (*tmp).doc;
                *fresh75 = (*ret).doc;
                let ref mut fresh76 = (*tmp).parent;
                *fresh76 = ret as xmlNodePtr;
                if ((*tmp).next).is_null() {
                    let ref mut fresh77 = (*ret).last;
                    *fresh77 = tmp;
                }
                tmp = (*tmp).next;
            }
        }
    } else if !value.is_null() {
        let mut tmp_0: * mut crate::src::HTMLparser::_xmlNode = 0 as *mut xmlNode;
        tmp_0 = xmlSAX2TextNode(
            ctxt,
            value,
            valueend.offset_from(value) as i64 as i32,
        );
        let ref mut fresh78 = (*ret).children;
        *fresh78 = tmp_0;
        let ref mut fresh79 = (*ret).last;
        *fresh79 = tmp_0;
        if !tmp_0.is_null() {
            let ref mut fresh80 = (*tmp_0).doc;
            *fresh80 = (*ret).doc;
            let ref mut fresh81 = (*tmp_0).parent;
            *fresh81 = ret as xmlNodePtr;
        }
    }
    if (*ctxt).html == 0 && (*ctxt).validate != 0 && (*ctxt).wellFormed != 0
        && !((*ctxt).myDoc).is_null() && !((*(*ctxt).myDoc).intSubset).is_null()
    {
        if (*ctxt).replaceEntities == 0 {
            dup = xmlSAX2DecodeAttrEntities(ctxt, value, valueend);
            if dup.is_null() {
                if *valueend as i32 == 0 as i32 {
                    (*ctxt).valid
                        &= xmlValidateOneAttribute(
                            &mut (*ctxt).vctxt,
                            (*ctxt).myDoc,
                            (*ctxt).node,
                            ret,
                            value,
                        );
                } else {
                    dup = xmlStrndup(
                        value,
                        valueend.offset_from(value) as i64 as i32,
                    );
                    (*ctxt).valid
                        &= xmlValidateOneAttribute(
                            &mut (*ctxt).vctxt,
                            (*ctxt).myDoc,
                            (*ctxt).node,
                            ret,
                            dup,
                        );
                }
            } else {
                if !((*ctxt).attsSpecial).is_null() {
                    let mut nvalnorm: * mut u8 = 0 as *mut xmlChar;
                    let mut fn_0: [u8; 50] = [0; 50];
                    let mut fullname: * mut u8 = 0 as *mut xmlChar;
                    fullname = xmlBuildQName(
                        localname,
                        prefix,
                        fn_0.as_mut_ptr(),
                        50 as i32,
                    );
                    if !fullname.is_null() {
                        (*ctxt).vctxt.valid = 1 as i32;
                        nvalnorm = xmlValidCtxtNormalizeAttributeValue(
                            &mut (*ctxt).vctxt,
                            (*ctxt).myDoc,
                            (*ctxt).node,
                            fullname,
                            dup,
                        );
                        if (*ctxt).vctxt.valid != 1 as i32 {
                            (*ctxt).valid = 0 as i32;
                        }
                        if fullname != fn_0.as_mut_ptr()
                            && fullname != localname as *mut xmlChar
                        {
                            xmlFree
                                .expect(
                                    "non-null function pointer",
                                )(fullname as *mut libc::c_void);
                        }
                        if !nvalnorm.is_null() {
                            xmlFree
                                .expect(
                                    "non-null function pointer",
                                )(dup as *mut libc::c_void);
                            dup = nvalnorm;
                        }
                    }
                }
                (*ctxt).valid
                    &= xmlValidateOneAttribute(
                        &mut (*ctxt).vctxt,
                        (*ctxt).myDoc,
                        (*ctxt).node,
                        ret,
                        dup,
                    );
            }
        } else {
            dup = xmlStrndup(
                value,
                valueend.offset_from(value) as i64 as i32,
            );
            (*ctxt).valid
                &= xmlValidateOneAttribute(
                    &mut (*ctxt).vctxt,
                    (*ctxt).myDoc,
                    (*ctxt).node,
                    ret,
                    dup,
                );
        }
    } else if (*ctxt).loadsubset & 8 as i32 == 0 as i32
            && ((*ctxt).replaceEntities == 0 as i32
                && (*ctxt).external != 2 as i32
                || (*ctxt).replaceEntities != 0 as i32
                    && (*ctxt).inSubset == 0 as i32)
            && !((*ret).children).is_null()
            && (*(*ret).children).type_0 as u32
                == XML_TEXT_NODE as i32 as u32
            && ((*(*ret).children).next).is_null()
        {
        let mut content: * mut u8 = (*(*ret).children).content;
        if prefix == (*ctxt).str_xml
            && *localname.offset(0 as i32 as isize) as i32 == 'i' as i32
            && *localname.offset(1 as i32 as isize) as i32 == 'd' as i32
            && *localname.offset(2 as i32 as isize) as i32
                == 0 as i32
        {
            if xmlValidateNCName(content, 1 as i32) != 0 as i32 {
                xmlErrValid(
                    ctxt,
                    XML_DTD_XMLID_VALUE,
                    b"xml:id : attribute value %s is not an NCName\n\0" as *const u8
                        as *const i8,
                    content as *const i8,
                    0 as *const i8,
                );
            }
            xmlAddID(&mut (*ctxt).vctxt, (*ctxt).myDoc, content, ret);
        } else if xmlIsID((*ctxt).myDoc, (*ctxt).node, ret) != 0 {
            xmlAddID(&mut (*ctxt).vctxt, (*ctxt).myDoc, content, ret);
        } else if xmlIsRef((*ctxt).myDoc, (*ctxt).node, ret) != 0 {
            xmlAddRef(&mut (*ctxt).vctxt, (*ctxt).myDoc, content, ret);
        }
    }
    if !dup.is_null() {
        xmlFree.expect("non-null function pointer")(dup as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2StartElementNs(
    mut ctx: * mut core::ffi::c_void,
    mut localname: * const u8,
    mut prefix: * const u8,
    mut URI: * const u8,
    mut nb_namespaces: i32,
    mut namespaces: * mut * const u8,
    mut nb_attributes: i32,
    mut nb_defaulted: i32,
    mut attributes: * mut * const u8,
) {
    let mut ctxt: * mut crate::src::HTMLparser::_xmlParserCtxt = ctx as xmlParserCtxtPtr;
    let mut ret: * mut crate::src::HTMLparser::_xmlNode = 0 as *mut xmlNode;
    let mut parent: * mut crate::src::HTMLparser::_xmlNode = 0 as *mut xmlNode;
    let mut last: * mut crate::src::HTMLparser::_xmlNs = 0 as xmlNsPtr;
    let mut ns: * mut crate::src::HTMLparser::_xmlNs = 0 as *mut xmlNs;
    let mut uri: * const u8 = 0 as *const xmlChar;
    let mut pref: * const u8 = 0 as *const xmlChar;
    let mut lname: * mut u8 = 0 as *mut xmlChar;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    if ctx.is_null() {
        return;
    }
    parent = (*ctxt).node;
    if (*ctxt).validate != 0 && ((*(*ctxt).myDoc).extSubset).is_null()
        && (((*(*ctxt).myDoc).intSubset).is_null()
            || ((*(*(*ctxt).myDoc).intSubset).notations).is_null()
                && ((*(*(*ctxt).myDoc).intSubset).elements).is_null()
                && ((*(*(*ctxt).myDoc).intSubset).attributes).is_null()
                && ((*(*(*ctxt).myDoc).intSubset).entities).is_null())
    {
        xmlErrValid(
            ctxt,
            XML_DTD_NO_DTD,
            b"Validation failed: no DTD found !\0" as *const u8 as *const i8,
            0 as *const i8,
            0 as *const i8,
        );
        (*ctxt).validate = 0 as i32;
    }
    if !prefix.is_null() && URI.is_null() {
        if (*ctxt).dictNames != 0 {
            let mut fullname: * const u8 = 0 as *const xmlChar;
            fullname = xmlDictQLookup((*ctxt).dict, prefix, localname);
            if !fullname.is_null() {
                localname = fullname;
            }
        } else {
            lname = xmlBuildQName(
                localname,
                prefix,
                0 as *mut xmlChar,
                0 as i32,
            );
        }
    }
    if !((*ctxt).freeElems).is_null() {
        ret = (*ctxt).freeElems;
        let ref mut fresh82 = (*ctxt).freeElems;
        *fresh82 = (*ret).next;
        let ref mut fresh83 = (*ctxt).freeElemsNr;
        *fresh83 -= 1;
        memset(
            ret as *mut libc::c_void,
            0 as i32,
            ::std::mem::size_of::<xmlNode>() as u64,
        );
        let ref mut fresh84 = (*ret).doc;
        *fresh84 = (*ctxt).myDoc;
        (*ret).type_0 = XML_ELEMENT_NODE;
        if (*ctxt).dictNames != 0 {
            let ref mut fresh85 = (*ret).name;
            *fresh85 = localname;
        } else {
            if lname.is_null() {
                let ref mut fresh86 = (*ret).name;
                *fresh86 = xmlStrdup(localname);
            } else {
                let ref mut fresh87 = (*ret).name;
                *fresh87 = lname;
            }
            if ((*ret).name).is_null() {
                xmlSAX2ErrMemory(
                    ctxt,
                    b"xmlSAX2StartElementNs\0" as *const u8 as *const i8,
                );
                return;
            }
        }
        if __xmlRegisterCallbacks != 0 && (*(borrow(& __xmlRegisterNodeDefaultValue())).unwrap()).is_some() {
            (*(borrow(& __xmlRegisterNodeDefaultValue())).unwrap()).expect("non-null function pointer")(ret);
        }
    } else {
        if (*ctxt).dictNames != 0 {
            ret = xmlNewDocNodeEatName(
                (*ctxt).myDoc,
                0 as xmlNsPtr,
                localname as *mut xmlChar,
                0 as *const xmlChar,
            );
        } else if lname.is_null() {
            ret = xmlNewDocNode(
                (*ctxt).myDoc,
                0 as xmlNsPtr,
                localname,
                0 as *const xmlChar,
            );
        } else {
            ret = xmlNewDocNodeEatName(
                (*ctxt).myDoc,
                0 as xmlNsPtr,
                lname,
                0 as *const xmlChar,
            );
        }
        if ret.is_null() {
            xmlSAX2ErrMemory(
                ctxt,
                b"xmlSAX2StartElementNs\0" as *const u8 as *const i8,
            );
            return;
        }
    }
    if (*ctxt).linenumbers != 0 {
        if !((*ctxt).input).is_null() {
            if (*(*ctxt).input).line
                < 32767 as i32 * 2 as i32 + 1 as i32
            {
                (*ret).line = (*(*ctxt).input).line as u16;
            } else {
                (*ret)
                    .line = (32767 as i32 * 2 as i32 + 1 as i32)
                    as u16;
            }
        }
    }
    if parent.is_null() {
        xmlAddChild((*ctxt).myDoc as xmlNodePtr, ret);
    }
    i = 0 as i32;
    j = 0 as i32;
    while j < nb_namespaces {
        let mut fresh88 = i;
        i = i + 1;
        pref = *namespaces.offset(fresh88 as isize);
        let mut fresh89 = i;
        i = i + 1;
        uri = *namespaces.offset(fresh89 as isize);
        ns = xmlNewNs(0 as xmlNodePtr, uri, pref);
        if !ns.is_null() {
            if last.is_null() {
                last = ns;
                let ref mut fresh90 = (*ret).nsDef;
                *fresh90 = last;
            } else {
                let ref mut fresh91 = (*last).next;
                *fresh91 = ns;
                last = ns;
            }
            if !URI.is_null() && prefix == pref {
                let ref mut fresh92 = (*ret).ns;
                *fresh92 = ns;
            }
            if (*ctxt).html == 0 && (*ctxt).validate != 0 && (*ctxt).wellFormed != 0
                && !((*ctxt).myDoc).is_null() && !((*(*ctxt).myDoc).intSubset).is_null()
            {
                (*ctxt).valid
                    &= xmlValidateOneNamespace(
                        &mut (*ctxt).vctxt,
                        (*ctxt).myDoc,
                        ret,
                        prefix,
                        ns,
                        uri,
                    );
            }
        }
        j += 1;
    }
    (*ctxt).nodemem = -(1 as i32);
    if nodePush(ctxt, ret) < 0 as i32 {
        xmlUnlinkNode(ret);
        xmlFreeNode(ret);
        return;
    }
    if !parent.is_null() {
        if (*parent).type_0 as u32
            == XML_ELEMENT_NODE as i32 as u32
        {
            xmlAddChild(parent, ret);
        } else {
            xmlAddSibling(parent, ret);
        }
    }
    if nb_defaulted != 0 as i32
        && (*ctxt).loadsubset & 4 as i32 == 0 as i32
    {
        nb_attributes -= nb_defaulted;
    }
    if !URI.is_null() && ((*ret).ns).is_null() {
        let ref mut fresh93 = (*ret).ns;
        *fresh93 = xmlSearchNs((*ctxt).myDoc, parent, prefix);
        if ((*ret).ns).is_null()
            && xmlStrEqual(
                prefix,
                b"xml\0" as *const u8 as *const i8 as *mut xmlChar,
            ) != 0
        {
            let ref mut fresh94 = (*ret).ns;
            *fresh94 = xmlSearchNs((*ctxt).myDoc, ret, prefix);
        }
        if ((*ret).ns).is_null() {
            ns = xmlNewNs(ret, 0 as *const xmlChar, prefix);
            if ns.is_null() {
                xmlSAX2ErrMemory(
                    ctxt,
                    b"xmlSAX2StartElementNs\0" as *const u8 as *const i8,
                );
                return;
            }
            if !prefix.is_null() {
                xmlNsWarnMsg(
                    ctxt,
                    XML_NS_ERR_UNDEFINED_NAMESPACE,
                    b"Namespace prefix %s was not found\n\0" as *const u8
                        as *const i8,
                    prefix,
                    0 as *const xmlChar,
                );
            } else {
                xmlNsWarnMsg(
                    ctxt,
                    XML_NS_ERR_UNDEFINED_NAMESPACE,
                    b"Namespace default prefix was not found\n\0" as *const u8
                        as *const i8,
                    0 as *const xmlChar,
                    0 as *const xmlChar,
                );
            }
        }
    }
    if nb_attributes > 0 as i32 {
        let mut current_block_110: u64;
        j = 0 as i32;
        i = 0 as i32;
        while i < nb_attributes {
            if !(*attributes.offset((j + 1 as i32) as isize)).is_null()
                && (*attributes.offset((j + 2 as i32) as isize)).is_null()
            {
                if (*ctxt).dictNames != 0 {
                    let mut fullname_0: * const u8 = 0 as *const xmlChar;
                    fullname_0 = xmlDictQLookup(
                        (*ctxt).dict,
                        *attributes.offset((j + 1 as i32) as isize),
                        *attributes.offset(j as isize),
                    );
                    if !fullname_0.is_null() {
                        xmlSAX2AttributeNs(
                            ctxt,
                            fullname_0,
                            0 as *const xmlChar,
                            *attributes.offset((j + 3 as i32) as isize),
                            *attributes.offset((j + 4 as i32) as isize),
                        );
                        current_block_110 = 13003737910779602957;
                    } else {
                        current_block_110 = 8304106758420804164;
                    }
                } else {
                    lname = xmlBuildQName(
                        *attributes.offset(j as isize),
                        *attributes.offset((j + 1 as i32) as isize),
                        0 as *mut xmlChar,
                        0 as i32,
                    );
                    if !lname.is_null() {
                        xmlSAX2AttributeNs(
                            ctxt,
                            lname,
                            0 as *const xmlChar,
                            *attributes.offset((j + 3 as i32) as isize),
                            *attributes.offset((j + 4 as i32) as isize),
                        );
                        xmlFree
                            .expect(
                                "non-null function pointer",
                            )(lname as *mut libc::c_void);
                        current_block_110 = 13003737910779602957;
                    } else {
                        current_block_110 = 8304106758420804164;
                    }
                }
            } else {
                current_block_110 = 8304106758420804164;
            }
            match current_block_110 {
                8304106758420804164 => {
                    xmlSAX2AttributeNs(
                        ctxt,
                        *attributes.offset(j as isize),
                        *attributes.offset((j + 1 as i32) as isize),
                        *attributes.offset((j + 3 as i32) as isize),
                        *attributes.offset((j + 4 as i32) as isize),
                    );
                }
                _ => {}
            }
            i += 1;
            j += 5 as i32;
        }
    }
    if (*ctxt).validate != 0
        && (*ctxt).vctxt.flags & (1 as u32) << 0 as i32
            == 0 as i32 as u32
    {
        let mut chk: i32 = 0;
        chk = xmlValidateDtdFinal(&mut (*ctxt).vctxt, (*ctxt).myDoc);
        if chk <= 0 as i32 {
            (*ctxt).valid = 0 as i32;
        }
        if chk < 0 as i32 {
            (*ctxt).wellFormed = 0 as i32;
        }
        (*ctxt).valid &= xmlValidateRoot(&mut (*ctxt).vctxt, (*ctxt).myDoc);
        (*ctxt).vctxt.flags |= (1 as u32) << 0 as i32;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2EndElementNs(
    mut ctx: * mut core::ffi::c_void,
    mut localname: * const u8,
    mut prefix: * const u8,
    mut URI: * const u8,
) {
    let mut ctxt: * mut crate::src::HTMLparser::_xmlParserCtxt = ctx as xmlParserCtxtPtr;
    let mut node_info: crate::src::HTMLparser::_xmlParserNodeInfo = xmlParserNodeInfo {
        node: 0 as *const _xmlNode,
        begin_pos: 0,
        begin_line: 0,
        end_pos: 0,
        end_line: 0,
    };
    let mut cur: * mut crate::src::HTMLparser::_xmlNode = 0 as *mut xmlNode;
    if ctx.is_null() {
        return;
    }
    cur = (*ctxt).node;
    if (*ctxt).record_info != 0 && !cur.is_null() {
        node_info
            .end_pos = ((*(*ctxt).input).cur).offset_from((*(*ctxt).input).base)
            as i64 as u64;
        node_info.end_line = (*(*ctxt).input).line as u64;
        node_info.node = cur as *const _xmlNode;
        xmlParserAddNodeInfo(ctxt, &mut node_info);
    }
    (*ctxt).nodemem = -(1 as i32);
    if (*ctxt).validate != 0 && (*ctxt).wellFormed != 0 && !((*ctxt).myDoc).is_null()
        && !((*(*ctxt).myDoc).intSubset).is_null()
    {
        (*ctxt).valid &= xmlValidateOneElement(&mut (*ctxt).vctxt, (*ctxt).myDoc, cur);
    }
    nodePop(ctxt);
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2Reference(
    mut ctx: * mut core::ffi::c_void,
    mut name: * const u8,
) {
    let mut ctxt: * mut crate::src::HTMLparser::_xmlParserCtxt = ctx as xmlParserCtxtPtr;
    let mut ret: * mut crate::src::HTMLparser::_xmlNode = 0 as *mut xmlNode;
    if ctx.is_null() {
        return;
    }
    if *name.offset(0 as i32 as isize) as i32 == '#' as i32 {
        ret = xmlNewCharRef((*ctxt).myDoc, name);
    } else {
        ret = xmlNewReference((*ctxt).myDoc as *const xmlDoc, name);
    }
    if (xmlAddChild((*ctxt).node, ret)).is_null() {
        xmlFreeNode(ret);
    }
}
unsafe extern "C" fn xmlSAX2Text(
    mut ctxt: * mut crate::src::HTMLparser::_xmlParserCtxt,
    mut ch: * const u8,
    mut len: i32,
    mut type_0: u32,
) {
    let mut lastChild: * mut crate::src::HTMLparser::_xmlNode = 0 as *mut xmlNode;
    if ctxt.is_null() {
        return;
    }
    if ((*ctxt).node).is_null() {
        return;
    }
    lastChild = (*(*ctxt).node).last;
    if lastChild.is_null() {
        if type_0 as u32 == XML_TEXT_NODE as i32 as u32 {
            lastChild = xmlSAX2TextNode(ctxt, ch, len);
        } else {
            lastChild = xmlNewCDataBlock((*ctxt).myDoc, ch, len);
        }
        if !lastChild.is_null() {
            let ref mut fresh95 = (*(*ctxt).node).children;
            *fresh95 = lastChild;
            let ref mut fresh96 = (*(*ctxt).node).last;
            *fresh96 = lastChild;
            let ref mut fresh97 = (*lastChild).parent;
            *fresh97 = (*ctxt).node;
            let ref mut fresh98 = (*lastChild).doc;
            *fresh98 = (*(*ctxt).node).doc;
            (*ctxt).nodelen = len;
            (*ctxt).nodemem = len + 1 as i32;
        } else {
            xmlSAX2ErrMemory(
                ctxt,
                b"xmlSAX2Characters\0" as *const u8 as *const i8,
            );
            return;
        }
    } else {
        let mut coalesceText: i32 = (!lastChild.is_null()
            && (*lastChild).type_0 as u32 == type_0 as u32
            && (type_0 as u32 != XML_TEXT_NODE as i32 as u32
                || (*lastChild).name == xmlStringText.as_ptr())) as i32;
        if coalesceText != 0 && (*ctxt).nodemem != 0 as i32 {
            if (*lastChild).content
                == &mut (*lastChild).properties as *mut *mut _xmlAttr as *mut xmlChar
            {
                let ref mut fresh99 = (*lastChild).content;
                *fresh99 = xmlStrdup((*lastChild).content);
                let ref mut fresh100 = (*lastChild).properties;
                *fresh100 = 0 as *mut _xmlAttr;
            } else if (*ctxt).nodemem == (*ctxt).nodelen + 1 as i32
                    && xmlDictOwns((*ctxt).dict, (*lastChild).content) != 0
                {
                let ref mut fresh101 = (*lastChild).content;
                *fresh101 = xmlStrdup((*lastChild).content);
            }
            if ((*lastChild).content).is_null() {
                xmlSAX2ErrMemory(
                    ctxt,
                    b"xmlSAX2Characters: xmlStrdup returned NULL\0" as *const u8
                        as *const i8,
                );
                return;
            }
            if ((*ctxt).nodelen as size_t).wrapping_add(len as size_t)
                > 10000000 as i32 as u64
                && (*ctxt).options & XML_PARSE_HUGE as i32 == 0 as i32
            {
                xmlSAX2ErrMemory(
                    ctxt,
                    b"xmlSAX2Characters: huge text node\0" as *const u8
                        as *const i8,
                );
                return;
            }
            if (*ctxt).nodelen as size_t
                > (-(1 as i32) as size_t).wrapping_sub(len as size_t)
                || ((*ctxt).nodemem as size_t).wrapping_add(len as size_t)
                    > (-(1 as i32) as size_t)
                        .wrapping_div(2 as i32 as u64)
            {
                xmlSAX2ErrMemory(
                    ctxt,
                    b"xmlSAX2Characters overflow prevented\0" as *const u8
                        as *const i8,
                );
                return;
            }
            if (*ctxt).nodelen + len >= (*ctxt).nodemem {
                let mut newbuf: * mut u8 = 0 as *mut xmlChar;
                let mut size: u64 = 0;
                size = ((*ctxt).nodemem + len) as size_t;
                size = (size as u64)
                    .wrapping_mul(2 as i32 as u64) as size_t as size_t;
                newbuf = xmlRealloc
                    .expect(
                        "non-null function pointer",
                    )((*lastChild).content as *mut libc::c_void, size) as *mut xmlChar;
                if newbuf.is_null() {
                    xmlSAX2ErrMemory(
                        ctxt,
                        b"xmlSAX2Characters\0" as *const u8 as *const i8,
                    );
                    return;
                }
                (*ctxt).nodemem = size as i32;
                let ref mut fresh102 = (*lastChild).content;
                *fresh102 = newbuf;
            }
            memcpy(
                &mut *((*lastChild).content).offset((*ctxt).nodelen as isize)
                    as *mut xmlChar as *mut libc::c_void,
                ch as *const libc::c_void,
                len as u64,
            );
            (*ctxt).nodelen += len;
            *((*lastChild).content)
                .offset((*ctxt).nodelen as isize) = 0 as i32 as xmlChar;
        } else if coalesceText != 0 {
            if xmlTextConcat(lastChild, ch, len) != 0 {
                xmlSAX2ErrMemory(
                    ctxt,
                    b"xmlSAX2Characters\0" as *const u8 as *const i8,
                );
            }
            if !((*(*ctxt).node).children).is_null() {
                (*ctxt).nodelen = xmlStrlen((*lastChild).content);
                (*ctxt).nodemem = (*ctxt).nodelen + 1 as i32;
            }
        } else {
            if type_0 as u32 == XML_TEXT_NODE as i32 as u32 {
                lastChild = xmlSAX2TextNode(ctxt, ch, len);
                let ref mut fresh103 = (*lastChild).doc;
                *fresh103 = (*ctxt).myDoc;
            } else {
                lastChild = xmlNewCDataBlock((*ctxt).myDoc, ch, len);
            }
            if !lastChild.is_null() {
                xmlAddChild((*ctxt).node, lastChild);
                if !((*(*ctxt).node).children).is_null() {
                    (*ctxt).nodelen = len;
                    (*ctxt).nodemem = len + 1 as i32;
                }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2Characters(
    mut ctx: * mut core::ffi::c_void,
    mut ch: * const u8,
    mut len: i32,
) {
    xmlSAX2Text(ctx as xmlParserCtxtPtr, ch, len, XML_TEXT_NODE);
}
#[no_mangle]
pub extern "C" fn xmlSAX2IgnorableWhitespace(
    mut ctx: * mut core::ffi::c_void,
    mut ch: * const u8,
    mut len: i32,
) {}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2ProcessingInstruction(
    mut ctx: * mut core::ffi::c_void,
    mut target: * const u8,
    mut data: * const u8,
) {
    let mut ctxt: * mut crate::src::HTMLparser::_xmlParserCtxt = ctx as xmlParserCtxtPtr;
    let mut ret: * mut crate::src::HTMLparser::_xmlNode = 0 as *mut xmlNode;
    let mut parent: * mut crate::src::HTMLparser::_xmlNode = 0 as *mut xmlNode;
    if ctx.is_null() {
        return;
    }
    parent = (*ctxt).node;
    ret = xmlNewDocPI((*ctxt).myDoc, target, data);
    if ret.is_null() {
        return;
    }
    if (*ctxt).linenumbers != 0 {
        if !((*ctxt).input).is_null() {
            if (*(*ctxt).input).line
                < 32767 as i32 * 2 as i32 + 1 as i32
            {
                (*ret).line = (*(*ctxt).input).line as u16;
            } else {
                (*ret)
                    .line = (32767 as i32 * 2 as i32 + 1 as i32)
                    as u16;
            }
        }
    }
    if (*ctxt).inSubset == 1 as i32 {
        xmlAddChild((*(*ctxt).myDoc).intSubset as xmlNodePtr, ret);
        return;
    } else {
        if (*ctxt).inSubset == 2 as i32 {
            xmlAddChild((*(*ctxt).myDoc).extSubset as xmlNodePtr, ret);
            return;
        }
    }
    if parent.is_null() {
        xmlAddChild((*ctxt).myDoc as xmlNodePtr, ret);
        return;
    }
    if (*parent).type_0 as u32
        == XML_ELEMENT_NODE as i32 as u32
    {
        xmlAddChild(parent, ret);
    } else {
        xmlAddSibling(parent, ret);
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2Comment(
    mut ctx: * mut core::ffi::c_void,
    mut value: * const u8,
) {
    let mut ctxt: * mut crate::src::HTMLparser::_xmlParserCtxt = ctx as xmlParserCtxtPtr;
    let mut ret: * mut crate::src::HTMLparser::_xmlNode = 0 as *mut xmlNode;
    let mut parent: * mut crate::src::HTMLparser::_xmlNode = 0 as *mut xmlNode;
    if ctx.is_null() {
        return;
    }
    parent = (*ctxt).node;
    ret = xmlNewDocComment((*ctxt).myDoc, value);
    if ret.is_null() {
        return;
    }
    if (*ctxt).linenumbers != 0 {
        if !((*ctxt).input).is_null() {
            if (*(*ctxt).input).line
                < 32767 as i32 * 2 as i32 + 1 as i32
            {
                (*ret).line = (*(*ctxt).input).line as u16;
            } else {
                (*ret)
                    .line = (32767 as i32 * 2 as i32 + 1 as i32)
                    as u16;
            }
        }
    }
    if (*ctxt).inSubset == 1 as i32 {
        xmlAddChild((*(*ctxt).myDoc).intSubset as xmlNodePtr, ret);
        return;
    } else {
        if (*ctxt).inSubset == 2 as i32 {
            xmlAddChild((*(*ctxt).myDoc).extSubset as xmlNodePtr, ret);
            return;
        }
    }
    if parent.is_null() {
        xmlAddChild((*ctxt).myDoc as xmlNodePtr, ret);
        return;
    }
    if (*parent).type_0 as u32
        == XML_ELEMENT_NODE as i32 as u32
    {
        xmlAddChild(parent, ret);
    } else {
        xmlAddSibling(parent, ret);
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2CDataBlock(
    mut ctx: * mut core::ffi::c_void,
    mut value: * const u8,
    mut len: i32,
) {
    xmlSAX2Text(ctx as xmlParserCtxtPtr, value, len, XML_CDATA_SECTION_NODE);
}
static mut xmlSAX2DefaultVersionValue: i32 = 2 as i32;
#[no_mangle]
pub unsafe extern "C" fn xmlSAXDefaultVersion(mut version: i32) -> i32 {
    let mut ret: i32 = xmlSAX2DefaultVersionValue;
    if version != 1 as i32 && version != 2 as i32 {
        return -(1 as i32);
    }
    xmlSAX2DefaultVersionValue = version;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAXVersion(
    mut hdlr: * mut crate::src::HTMLparser::_xmlSAXHandler,
    mut version: i32,
) -> i32 {
    if hdlr.is_null() {
        return -(1 as i32);
    }
    if version == 2 as i32 {
        let ref mut fresh104 = (*hdlr).startElement;
        *fresh104 = None;
        let ref mut fresh105 = (*hdlr).endElement;
        *fresh105 = None;
        let ref mut fresh106 = (*hdlr).startElementNs;
        *fresh106 = Some(
            xmlSAX2StartElementNs,
        );
        let ref mut fresh107 = (*hdlr).endElementNs;
        *fresh107 = Some(
            xmlSAX2EndElementNs,
        );
        let ref mut fresh108 = (*hdlr).serror;
        *fresh108 = None;
        (*hdlr).initialized = 0xdeedbeaf as u32;
    } else if version == 1 as i32 {
        let ref mut fresh109 = (*hdlr).startElement;
        *fresh109 = Some(
            xmlSAX2StartElement,
        );
        let ref mut fresh110 = (*hdlr).endElement;
        *fresh110 = Some(
            xmlSAX2EndElement,
        );
        (*hdlr).initialized = 1 as i32 as u32;
    } else {
        return -(1 as i32)
    }
    let ref mut fresh111 = (*hdlr).internalSubset;
    *fresh111 = Some(
        xmlSAX2InternalSubset,
    );
    let ref mut fresh112 = (*hdlr).externalSubset;
    *fresh112 = Some(
        xmlSAX2ExternalSubset,
    );
    let ref mut fresh113 = (*hdlr).isStandalone;
    *fresh113 = Some(
        xmlSAX2IsStandalone,
    );
    let ref mut fresh114 = (*hdlr).hasInternalSubset;
    *fresh114 = Some(
        xmlSAX2HasInternalSubset,
    );
    let ref mut fresh115 = (*hdlr).hasExternalSubset;
    *fresh115 = Some(
        xmlSAX2HasExternalSubset,
    );
    let ref mut fresh116 = (*hdlr).resolveEntity;
    *fresh116 = Some(
        xmlSAX2ResolveEntity,
    );
    let ref mut fresh117 = (*hdlr).getEntity;
    *fresh117 = Some(
        xmlSAX2GetEntity,
    );
    let ref mut fresh118 = (*hdlr).getParameterEntity;
    *fresh118 = Some(
        xmlSAX2GetParameterEntity,
    );
    let ref mut fresh119 = (*hdlr).entityDecl;
    *fresh119 = Some(
        xmlSAX2EntityDecl,
    );
    let ref mut fresh120 = (*hdlr).attributeDecl;
    *fresh120 = Some(
        xmlSAX2AttributeDecl,
    );
    let ref mut fresh121 = (*hdlr).elementDecl;
    *fresh121 = Some(
        xmlSAX2ElementDecl,
    );
    let ref mut fresh122 = (*hdlr).notationDecl;
    *fresh122 = Some(
        xmlSAX2NotationDecl,
    );
    let ref mut fresh123 = (*hdlr).unparsedEntityDecl;
    *fresh123 = Some(
        xmlSAX2UnparsedEntityDecl,
    );
    let ref mut fresh124 = (*hdlr).setDocumentLocator;
    *fresh124 = Some(
        xmlSAX2SetDocumentLocator,
    );
    let ref mut fresh125 = (*hdlr).startDocument;
    *fresh125 = Some(
        xmlSAX2StartDocument,
    );
    let ref mut fresh126 = (*hdlr).endDocument;
    *fresh126 = Some(
        xmlSAX2EndDocument,
    );
    let ref mut fresh127 = (*hdlr).reference;
    *fresh127 = Some(
        xmlSAX2Reference,
    );
    let ref mut fresh128 = (*hdlr).characters;
    *fresh128 = Some(
        xmlSAX2Characters,
    );
    let ref mut fresh129 = (*hdlr).cdataBlock;
    *fresh129 = Some(
        xmlSAX2CDataBlock,
    );
    let ref mut fresh130 = (*hdlr).ignorableWhitespace;
    *fresh130 = Some(
        xmlSAX2Characters,
    );
    let ref mut fresh131 = (*hdlr).processingInstruction;
    *fresh131 = Some(
        xmlSAX2ProcessingInstruction,
    );
    let ref mut fresh132 = (*hdlr).comment;
    *fresh132 = Some(
        xmlSAX2Comment,
    );
    let ref mut fresh133 = (*hdlr).warning;
    *fresh133 = Some(
        xmlParserWarning,
    );
    let ref mut fresh134 = (*hdlr).error;
    *fresh134 = Some(
        xmlParserError,
    );
    let ref mut fresh135 = (*hdlr).fatalError;
    *fresh135 = Some(
        xmlParserError,
    );
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2InitDefaultSAXHandler(
    mut hdlr: * mut crate::src::HTMLparser::_xmlSAXHandler,
    mut warning: i32,
) {
    if hdlr.is_null() || (*hdlr).initialized != 0 as i32 as u32 {
        return;
    }
    xmlSAXVersion(hdlr, xmlSAX2DefaultVersionValue);
    if warning == 0 as i32 {
        let ref mut fresh136 = (*hdlr).warning;
        *fresh136 = None;
    } else {
        let ref mut fresh137 = (*hdlr).warning;
        *fresh137 = Some(
            xmlParserWarning,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlDefaultSAXHandlerInit() {
    xmlSAXVersion(__xmlDefaultSAXHandler() as xmlSAXHandlerPtr, 1 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn xmlSAX2InitHtmlDefaultSAXHandler(mut hdlr: * mut crate::src::HTMLparser::_xmlSAXHandler) {
    if hdlr.is_null() || (*hdlr).initialized != 0 as i32 as u32 {
        return;
    }
    let ref mut fresh138 = (*hdlr).internalSubset;
    *fresh138 = Some(
        xmlSAX2InternalSubset,
    );
    let ref mut fresh139 = (*hdlr).externalSubset;
    *fresh139 = None;
    let ref mut fresh140 = (*hdlr).isStandalone;
    *fresh140 = None;
    let ref mut fresh141 = (*hdlr).hasInternalSubset;
    *fresh141 = None;
    let ref mut fresh142 = (*hdlr).hasExternalSubset;
    *fresh142 = None;
    let ref mut fresh143 = (*hdlr).resolveEntity;
    *fresh143 = None;
    let ref mut fresh144 = (*hdlr).getEntity;
    *fresh144 = Some(
        xmlSAX2GetEntity,
    );
    let ref mut fresh145 = (*hdlr).getParameterEntity;
    *fresh145 = None;
    let ref mut fresh146 = (*hdlr).entityDecl;
    *fresh146 = None;
    let ref mut fresh147 = (*hdlr).attributeDecl;
    *fresh147 = None;
    let ref mut fresh148 = (*hdlr).elementDecl;
    *fresh148 = None;
    let ref mut fresh149 = (*hdlr).notationDecl;
    *fresh149 = None;
    let ref mut fresh150 = (*hdlr).unparsedEntityDecl;
    *fresh150 = None;
    let ref mut fresh151 = (*hdlr).setDocumentLocator;
    *fresh151 = Some(
        xmlSAX2SetDocumentLocator,
    );
    let ref mut fresh152 = (*hdlr).startDocument;
    *fresh152 = Some(
        xmlSAX2StartDocument,
    );
    let ref mut fresh153 = (*hdlr).endDocument;
    *fresh153 = Some(
        xmlSAX2EndDocument,
    );
    let ref mut fresh154 = (*hdlr).startElement;
    *fresh154 = Some(
        xmlSAX2StartElement,
    );
    let ref mut fresh155 = (*hdlr).endElement;
    *fresh155 = Some(
        xmlSAX2EndElement,
    );
    let ref mut fresh156 = (*hdlr).reference;
    *fresh156 = None;
    let ref mut fresh157 = (*hdlr).characters;
    *fresh157 = Some(
        xmlSAX2Characters,
    );
    let ref mut fresh158 = (*hdlr).cdataBlock;
    *fresh158 = Some(
        xmlSAX2CDataBlock,
    );
    let ref mut fresh159 = (*hdlr).ignorableWhitespace;
    *fresh159 = Some(
        xmlSAX2IgnorableWhitespace,
    );
    let ref mut fresh160 = (*hdlr).processingInstruction;
    *fresh160 = Some(
        xmlSAX2ProcessingInstruction,
    );
    let ref mut fresh161 = (*hdlr).comment;
    *fresh161 = Some(
        xmlSAX2Comment,
    );
    let ref mut fresh162 = (*hdlr).warning;
    *fresh162 = Some(
        xmlParserWarning,
    );
    let ref mut fresh163 = (*hdlr).error;
    *fresh163 = Some(
        xmlParserError,
    );
    let ref mut fresh164 = (*hdlr).fatalError;
    *fresh164 = Some(
        xmlParserError,
    );
    (*hdlr).initialized = 1 as i32 as u32;
}
#[no_mangle]
pub unsafe extern "C" fn htmlDefaultSAXHandlerInit() {
    xmlSAX2InitHtmlDefaultSAXHandler(__htmlDefaultSAXHandler() as xmlSAXHandlerPtr);
}
use crate::laertes_rt::*;
