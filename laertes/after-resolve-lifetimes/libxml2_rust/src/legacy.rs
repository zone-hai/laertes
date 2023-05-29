use ::libc;
extern "C" {
    
    
    
    
    
    
    
    fn strcmp(_: * const i8, _: * const i8) -> i32;
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
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
pub use crate::src::catalog::_xmlAutomata;
pub use crate::src::debugXML::_xmlValidState;
pub use crate::src::encoding::_xmlAutomataState;
pub use crate::src::parser::_xmlStartTag;
pub type xmlChar = u8;
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
pub type xmlNsPtr = * mut crate::src::HTMLparser::_xmlNs;
pub type xmlGenericErrorFunc = Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,_: * const i8,...) -> ()>;
pub type htmlParserCtxtPtr = * mut crate::src::HTMLparser::_xmlParserCtxt;
#[no_mangle]
pub unsafe extern "C" fn htmlDecodeEntities<'a1, 'a2>(
    mut ctxt: Option<&'a1 mut crate::src::HTMLparser::_xmlParserCtxt>,
    mut len: i32,
    mut end: u8,
    mut end2: u8,
    mut end3: u8,
) -> Option<&'a2 mut u8> {
    static mut deprecated: i32 = 0 as i32;
    if deprecated == 0 {
        (*(borrow(& __xmlGenericError())).unwrap())
            .expect(
                "non-null function pointer",
            )(
            *(__xmlGenericErrorContext()).unwrap(),
            b"htmlDecodeEntities() deprecated function reached\n\0" as *const u8
                as *const i8,
        );
        deprecated = 1 as i32;
    }
    return Option::<&'_ mut u8>::None;
}
#[no_mangle]
pub extern "C" fn xmlInitializePredefinedEntities() {}
#[no_mangle]
pub extern "C" fn xmlCleanupPredefinedEntities() {}
static mut xmlFeaturesList: [* const i8; 42] = [
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
pub unsafe extern "C" fn xmlGetFeaturesList<'a1, 'a2>(
    mut len: Option<&'a1 mut i32>,
    mut result: Option<crate::__laertes_array::CustomSlice<'a2, * const i8, &'a2 mut  [* const i8]>>,
) -> i32 {
    let mut ret: i32 = 0;
    let mut i: i32 = 0;
    ret = (::std::mem::size_of::<[*const i8; 42]>() as u64)
        .wrapping_div(::std::mem::size_of::<*const i8>() as u64)
        as i32;
    if borrow(& len).is_none() || crate::__laertes_array::borrow(& result)/*borrow*/.is_none() {
        return ret;
    }
    if *(borrow(& len)).unwrap() < 0 as i32 || *(borrow(& len)).unwrap() >= 1000 as i32 {
        return -(1 as i32);
    }
    if *(borrow(& len)).unwrap() > ret {
        *(borrow_mut(&mut len)).unwrap() = ret;
    }
    i = 0 as i32;
    while i < *(borrow(& len)).unwrap() {
        let ref mut fresh0 = (*crate::__laertes_array::GetMut::<&mut _>::get_offset_mut(result.as_mut().unwrap(), (i as isize)));
        *fresh0 = xmlFeaturesList[i as usize];
        i += 1;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlGetFeature<'a1>(
    mut ctxt: Option<&'a1 mut crate::src::HTMLparser::_xmlParserCtxt>,
    mut name: * const i8,
    mut result: * mut core::ffi::c_void,
) -> i32 {
    if borrow(& ctxt).is_none() || name.is_null() || result.is_null() {
        return -(1 as i32);
    }
    if strcmp(name, b"validate\0" as *const u8 as *const i8) == 0 {
        *(result as *mut i32) = (*(borrow_mut(&mut ctxt)).unwrap()).validate;
    } else if strcmp(name, b"keep blanks\0" as *const u8 as *const i8) == 0 {
        *(result as *mut i32) = (*(borrow_mut(&mut ctxt)).unwrap()).keepBlanks;
    } else if strcmp(name, b"disable SAX\0" as *const u8 as *const i8) == 0 {
        *(result as *mut i32) = (*(borrow_mut(&mut ctxt)).unwrap()).disableSAX;
    } else if strcmp(
            name,
            b"fetch external entities\0" as *const u8 as *const i8,
        ) == 0
        {
        *(result as *mut i32) = (*(borrow_mut(&mut ctxt)).unwrap()).loadsubset;
    } else if strcmp(name, b"substitute entities\0" as *const u8 as *const i8)
            == 0
        {
        *(result as *mut i32) = (*(borrow_mut(&mut ctxt)).unwrap()).replaceEntities;
    } else if strcmp(name, b"gather line info\0" as *const u8 as *const i8)
            == 0
        {
        *(result as *mut i32) = (*(borrow_mut(&mut ctxt)).unwrap()).record_info;
    } else if strcmp(name, b"user data\0" as *const u8 as *const i8) == 0 {
        let ref mut fresh1 = *(result as *mut *mut libc::c_void);
        *fresh1 = (*(borrow_mut(&mut ctxt)).unwrap()).userData;
    } else if strcmp(name, b"is html\0" as *const u8 as *const i8) == 0 {
        *(result as *mut i32) = (*(borrow_mut(&mut ctxt)).unwrap()).html;
    } else if strcmp(name, b"is standalone\0" as *const u8 as *const i8) == 0 {
        *(result as *mut i32) = (*(borrow_mut(&mut ctxt)).unwrap()).standalone;
    } else if strcmp(name, b"document\0" as *const u8 as *const i8) == 0 {
        let ref mut fresh2 = *(result as *mut xmlDocPtr);
        *fresh2 = (*(borrow_mut(&mut ctxt)).unwrap()).myDoc;
    } else if strcmp(name, b"is well formed\0" as *const u8 as *const i8) == 0
        {
        *(result as *mut i32) = (*(borrow_mut(&mut ctxt)).unwrap()).wellFormed;
    } else if strcmp(name, b"is valid\0" as *const u8 as *const i8) == 0 {
        *(result as *mut i32) = (*(borrow_mut(&mut ctxt)).unwrap()).valid;
    } else if strcmp(name, b"SAX block\0" as *const u8 as *const i8) == 0 {
        let ref mut fresh3 = *(result as *mut xmlSAXHandlerPtr);
        *fresh3 = (*(borrow_mut(&mut ctxt)).unwrap()).sax;
    } else if strcmp(
            name,
            b"SAX function internalSubset\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh4 = *(result as *mut internalSubsetSAXFunc);
        *fresh4 = (*(*(borrow_mut(&mut ctxt)).unwrap()).sax).internalSubset;
    } else if strcmp(
            name,
            b"SAX function isStandalone\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh5 = *(result as *mut isStandaloneSAXFunc);
        *fresh5 = (*(*(borrow_mut(&mut ctxt)).unwrap()).sax).isStandalone;
    } else if strcmp(
            name,
            b"SAX function hasInternalSubset\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh6 = *(result as *mut hasInternalSubsetSAXFunc);
        *fresh6 = (*(*(borrow_mut(&mut ctxt)).unwrap()).sax).hasInternalSubset;
    } else if strcmp(
            name,
            b"SAX function hasExternalSubset\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh7 = *(result as *mut hasExternalSubsetSAXFunc);
        *fresh7 = (*(*(borrow_mut(&mut ctxt)).unwrap()).sax).hasExternalSubset;
    } else if strcmp(
            name,
            b"SAX function resolveEntity\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh8 = *(result as *mut resolveEntitySAXFunc);
        *fresh8 = (*(*(borrow_mut(&mut ctxt)).unwrap()).sax).resolveEntity;
    } else if strcmp(
            name,
            b"SAX function getEntity\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh9 = *(result as *mut getEntitySAXFunc);
        *fresh9 = (*(*(borrow_mut(&mut ctxt)).unwrap()).sax).getEntity;
    } else if strcmp(
            name,
            b"SAX function entityDecl\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh10 = *(result as *mut entityDeclSAXFunc);
        *fresh10 = (*(*(borrow_mut(&mut ctxt)).unwrap()).sax).entityDecl;
    } else if strcmp(
            name,
            b"SAX function notationDecl\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh11 = *(result as *mut notationDeclSAXFunc);
        *fresh11 = (*(*(borrow_mut(&mut ctxt)).unwrap()).sax).notationDecl;
    } else if strcmp(
            name,
            b"SAX function attributeDecl\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh12 = *(result as *mut attributeDeclSAXFunc);
        *fresh12 = (*(*(borrow_mut(&mut ctxt)).unwrap()).sax).attributeDecl;
    } else if strcmp(
            name,
            b"SAX function elementDecl\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh13 = *(result as *mut elementDeclSAXFunc);
        *fresh13 = (*(*(borrow_mut(&mut ctxt)).unwrap()).sax).elementDecl;
    } else if strcmp(
            name,
            b"SAX function unparsedEntityDecl\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh14 = *(result as *mut unparsedEntityDeclSAXFunc);
        *fresh14 = (*(*(borrow_mut(&mut ctxt)).unwrap()).sax).unparsedEntityDecl;
    } else if strcmp(
            name,
            b"SAX function setDocumentLocator\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh15 = *(result as *mut setDocumentLocatorSAXFunc);
        *fresh15 = (*(*(borrow_mut(&mut ctxt)).unwrap()).sax).setDocumentLocator;
    } else if strcmp(
            name,
            b"SAX function startDocument\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh16 = *(result as *mut startDocumentSAXFunc);
        *fresh16 = (*(*(borrow_mut(&mut ctxt)).unwrap()).sax).startDocument;
    } else if strcmp(
            name,
            b"SAX function endDocument\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh17 = *(result as *mut endDocumentSAXFunc);
        *fresh17 = (*(*(borrow_mut(&mut ctxt)).unwrap()).sax).endDocument;
    } else if strcmp(
            name,
            b"SAX function startElement\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh18 = *(result as *mut startElementSAXFunc);
        *fresh18 = (*(*(borrow_mut(&mut ctxt)).unwrap()).sax).startElement;
    } else if strcmp(
            name,
            b"SAX function endElement\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh19 = *(result as *mut endElementSAXFunc);
        *fresh19 = (*(*(borrow_mut(&mut ctxt)).unwrap()).sax).endElement;
    } else if strcmp(
            name,
            b"SAX function reference\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh20 = *(result as *mut referenceSAXFunc);
        *fresh20 = (*(*(borrow_mut(&mut ctxt)).unwrap()).sax).reference;
    } else if strcmp(
            name,
            b"SAX function characters\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh21 = *(result as *mut charactersSAXFunc);
        *fresh21 = (*(*(borrow_mut(&mut ctxt)).unwrap()).sax).characters;
    } else if strcmp(
            name,
            b"SAX function ignorableWhitespace\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh22 = *(result as *mut ignorableWhitespaceSAXFunc);
        *fresh22 = (*(*(borrow_mut(&mut ctxt)).unwrap()).sax).ignorableWhitespace;
    } else if strcmp(
            name,
            b"SAX function processingInstruction\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh23 = *(result as *mut processingInstructionSAXFunc);
        *fresh23 = (*(*(borrow_mut(&mut ctxt)).unwrap()).sax).processingInstruction;
    } else if strcmp(name, b"SAX function comment\0" as *const u8 as *const i8)
            == 0
        {
        let ref mut fresh24 = *(result as *mut commentSAXFunc);
        *fresh24 = (*(*(borrow_mut(&mut ctxt)).unwrap()).sax).comment;
    } else if strcmp(name, b"SAX function warning\0" as *const u8 as *const i8)
            == 0
        {
        let ref mut fresh25 = *(result as *mut warningSAXFunc);
        *fresh25 = (*(*(borrow_mut(&mut ctxt)).unwrap()).sax).warning;
    } else if strcmp(name, b"SAX function error\0" as *const u8 as *const i8)
            == 0
        {
        let ref mut fresh26 = *(result as *mut errorSAXFunc);
        *fresh26 = (*(*(borrow_mut(&mut ctxt)).unwrap()).sax).error;
    } else if strcmp(
            name,
            b"SAX function fatalError\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh27 = *(result as *mut fatalErrorSAXFunc);
        *fresh27 = (*(*(borrow_mut(&mut ctxt)).unwrap()).sax).fatalError;
    } else if strcmp(
            name,
            b"SAX function getParameterEntity\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh28 = *(result as *mut getParameterEntitySAXFunc);
        *fresh28 = (*(*(borrow_mut(&mut ctxt)).unwrap()).sax).getParameterEntity;
    } else if strcmp(
            name,
            b"SAX function cdataBlock\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh29 = *(result as *mut cdataBlockSAXFunc);
        *fresh29 = (*(*(borrow_mut(&mut ctxt)).unwrap()).sax).cdataBlock;
    } else if strcmp(
            name,
            b"SAX function externalSubset\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh30 = *(result as *mut externalSubsetSAXFunc);
        *fresh30 = (*(*(borrow_mut(&mut ctxt)).unwrap()).sax).externalSubset;
    } else {
        return -(1 as i32)
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn xmlSetFeature<'a1>(
    mut ctxt: Option<&'a1 mut crate::src::HTMLparser::_xmlParserCtxt>,
    mut name: * const i8,
    mut value: * mut core::ffi::c_void,
) -> i32 {
    if borrow(& ctxt).is_none() || name.is_null() || value.is_null() {
        return -(1 as i32);
    }
    if strcmp(name, b"validate\0" as *const u8 as *const i8) == 0 {
        let mut newvalidate: i32 = *(value as *mut i32);
        if (*(borrow(& ctxt)).unwrap()).validate == 0 && newvalidate != 0 as i32 {
            if ((*(borrow(& ctxt)).unwrap()).vctxt.warning).is_none() {
                let ref mut fresh31 = (*(borrow_mut(&mut ctxt)).unwrap()).vctxt.warning;
                *fresh31 = Some(
                    xmlParserValidityWarning,
                );
            }
            if ((*(borrow(& ctxt)).unwrap()).vctxt.error).is_none() {
                let ref mut fresh32 = (*(borrow_mut(&mut ctxt)).unwrap()).vctxt.error;
                *fresh32 = Some(
                    xmlParserValidityError,
                );
            }
            (*(borrow_mut(&mut ctxt)).unwrap()).vctxt.nodeMax = 0 as i32;
        }
        (*(borrow_mut(&mut ctxt)).unwrap()).validate = newvalidate;
    } else if strcmp(name, b"keep blanks\0" as *const u8 as *const i8) == 0 {
        (*(borrow_mut(&mut ctxt)).unwrap()).keepBlanks = *(value as *mut i32);
    } else if strcmp(name, b"disable SAX\0" as *const u8 as *const i8) == 0 {
        (*(borrow_mut(&mut ctxt)).unwrap()).disableSAX = *(value as *mut i32);
    } else if strcmp(
            name,
            b"fetch external entities\0" as *const u8 as *const i8,
        ) == 0
        {
        (*(borrow_mut(&mut ctxt)).unwrap()).loadsubset = *(value as *mut i32);
    } else if strcmp(name, b"substitute entities\0" as *const u8 as *const i8)
            == 0
        {
        (*(borrow_mut(&mut ctxt)).unwrap()).replaceEntities = *(value as *mut i32);
    } else if strcmp(name, b"gather line info\0" as *const u8 as *const i8)
            == 0
        {
        (*(borrow_mut(&mut ctxt)).unwrap()).record_info = *(value as *mut i32);
    } else if strcmp(name, b"user data\0" as *const u8 as *const i8) == 0 {
        let ref mut fresh33 = (*(borrow_mut(&mut ctxt)).unwrap()).userData;
        *fresh33 = *(value as *mut *mut libc::c_void);
    } else if strcmp(name, b"is html\0" as *const u8 as *const i8) == 0 {
        (*(borrow_mut(&mut ctxt)).unwrap()).html = *(value as *mut i32);
    } else if strcmp(name, b"is standalone\0" as *const u8 as *const i8) == 0 {
        (*(borrow_mut(&mut ctxt)).unwrap()).standalone = *(value as *mut i32);
    } else if strcmp(name, b"document\0" as *const u8 as *const i8) == 0 {
        let ref mut fresh34 = (*(borrow_mut(&mut ctxt)).unwrap()).myDoc;
        *fresh34 = *(value as *mut xmlDocPtr);
    } else if strcmp(name, b"is well formed\0" as *const u8 as *const i8) == 0
        {
        (*(borrow_mut(&mut ctxt)).unwrap()).wellFormed = *(value as *mut i32);
    } else if strcmp(name, b"is valid\0" as *const u8 as *const i8) == 0 {
        (*(borrow_mut(&mut ctxt)).unwrap()).valid = *(value as *mut i32);
    } else if strcmp(name, b"SAX block\0" as *const u8 as *const i8) == 0 {
        let ref mut fresh35 = (*(borrow_mut(&mut ctxt)).unwrap()).sax;
        *fresh35 = *(value as *mut xmlSAXHandlerPtr);
    } else if strcmp(
            name,
            b"SAX function internalSubset\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh36 = (*(*(borrow_mut(&mut ctxt)).unwrap()).sax).internalSubset;
        *fresh36 = *(value as *mut internalSubsetSAXFunc);
    } else if strcmp(
            name,
            b"SAX function isStandalone\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh37 = (*(*(borrow_mut(&mut ctxt)).unwrap()).sax).isStandalone;
        *fresh37 = *(value as *mut isStandaloneSAXFunc);
    } else if strcmp(
            name,
            b"SAX function hasInternalSubset\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh38 = (*(*(borrow_mut(&mut ctxt)).unwrap()).sax).hasInternalSubset;
        *fresh38 = *(value as *mut hasInternalSubsetSAXFunc);
    } else if strcmp(
            name,
            b"SAX function hasExternalSubset\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh39 = (*(*(borrow_mut(&mut ctxt)).unwrap()).sax).hasExternalSubset;
        *fresh39 = *(value as *mut hasExternalSubsetSAXFunc);
    } else if strcmp(
            name,
            b"SAX function resolveEntity\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh40 = (*(*(borrow_mut(&mut ctxt)).unwrap()).sax).resolveEntity;
        *fresh40 = *(value as *mut resolveEntitySAXFunc);
    } else if strcmp(
            name,
            b"SAX function getEntity\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh41 = (*(*(borrow_mut(&mut ctxt)).unwrap()).sax).getEntity;
        *fresh41 = *(value as *mut getEntitySAXFunc);
    } else if strcmp(
            name,
            b"SAX function entityDecl\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh42 = (*(*(borrow_mut(&mut ctxt)).unwrap()).sax).entityDecl;
        *fresh42 = *(value as *mut entityDeclSAXFunc);
    } else if strcmp(
            name,
            b"SAX function notationDecl\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh43 = (*(*(borrow_mut(&mut ctxt)).unwrap()).sax).notationDecl;
        *fresh43 = *(value as *mut notationDeclSAXFunc);
    } else if strcmp(
            name,
            b"SAX function attributeDecl\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh44 = (*(*(borrow_mut(&mut ctxt)).unwrap()).sax).attributeDecl;
        *fresh44 = *(value as *mut attributeDeclSAXFunc);
    } else if strcmp(
            name,
            b"SAX function elementDecl\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh45 = (*(*(borrow_mut(&mut ctxt)).unwrap()).sax).elementDecl;
        *fresh45 = *(value as *mut elementDeclSAXFunc);
    } else if strcmp(
            name,
            b"SAX function unparsedEntityDecl\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh46 = (*(*(borrow_mut(&mut ctxt)).unwrap()).sax).unparsedEntityDecl;
        *fresh46 = *(value as *mut unparsedEntityDeclSAXFunc);
    } else if strcmp(
            name,
            b"SAX function setDocumentLocator\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh47 = (*(*(borrow_mut(&mut ctxt)).unwrap()).sax).setDocumentLocator;
        *fresh47 = *(value as *mut setDocumentLocatorSAXFunc);
    } else if strcmp(
            name,
            b"SAX function startDocument\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh48 = (*(*(borrow_mut(&mut ctxt)).unwrap()).sax).startDocument;
        *fresh48 = *(value as *mut startDocumentSAXFunc);
    } else if strcmp(
            name,
            b"SAX function endDocument\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh49 = (*(*(borrow_mut(&mut ctxt)).unwrap()).sax).endDocument;
        *fresh49 = *(value as *mut endDocumentSAXFunc);
    } else if strcmp(
            name,
            b"SAX function startElement\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh50 = (*(*(borrow_mut(&mut ctxt)).unwrap()).sax).startElement;
        *fresh50 = *(value as *mut startElementSAXFunc);
    } else if strcmp(
            name,
            b"SAX function endElement\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh51 = (*(*(borrow_mut(&mut ctxt)).unwrap()).sax).endElement;
        *fresh51 = *(value as *mut endElementSAXFunc);
    } else if strcmp(
            name,
            b"SAX function reference\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh52 = (*(*(borrow_mut(&mut ctxt)).unwrap()).sax).reference;
        *fresh52 = *(value as *mut referenceSAXFunc);
    } else if strcmp(
            name,
            b"SAX function characters\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh53 = (*(*(borrow_mut(&mut ctxt)).unwrap()).sax).characters;
        *fresh53 = *(value as *mut charactersSAXFunc);
    } else if strcmp(
            name,
            b"SAX function ignorableWhitespace\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh54 = (*(*(borrow_mut(&mut ctxt)).unwrap()).sax).ignorableWhitespace;
        *fresh54 = *(value as *mut ignorableWhitespaceSAXFunc);
    } else if strcmp(
            name,
            b"SAX function processingInstruction\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh55 = (*(*(borrow_mut(&mut ctxt)).unwrap()).sax).processingInstruction;
        *fresh55 = *(value as *mut processingInstructionSAXFunc);
    } else if strcmp(name, b"SAX function comment\0" as *const u8 as *const i8)
            == 0
        {
        let ref mut fresh56 = (*(*(borrow_mut(&mut ctxt)).unwrap()).sax).comment;
        *fresh56 = *(value as *mut commentSAXFunc);
    } else if strcmp(name, b"SAX function warning\0" as *const u8 as *const i8)
            == 0
        {
        let ref mut fresh57 = (*(*(borrow_mut(&mut ctxt)).unwrap()).sax).warning;
        *fresh57 = *(value as *mut warningSAXFunc);
    } else if strcmp(name, b"SAX function error\0" as *const u8 as *const i8)
            == 0
        {
        let ref mut fresh58 = (*(*(borrow_mut(&mut ctxt)).unwrap()).sax).error;
        *fresh58 = *(value as *mut errorSAXFunc);
    } else if strcmp(
            name,
            b"SAX function fatalError\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh59 = (*(*(borrow_mut(&mut ctxt)).unwrap()).sax).fatalError;
        *fresh59 = *(value as *mut fatalErrorSAXFunc);
    } else if strcmp(
            name,
            b"SAX function getParameterEntity\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh60 = (*(*(borrow_mut(&mut ctxt)).unwrap()).sax).getParameterEntity;
        *fresh60 = *(value as *mut getParameterEntitySAXFunc);
    } else if strcmp(
            name,
            b"SAX function cdataBlock\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh61 = (*(*(borrow_mut(&mut ctxt)).unwrap()).sax).cdataBlock;
        *fresh61 = *(value as *mut cdataBlockSAXFunc);
    } else if strcmp(
            name,
            b"SAX function externalSubset\0" as *const u8 as *const i8,
        ) == 0
        {
        let ref mut fresh62 = (*(*(borrow_mut(&mut ctxt)).unwrap()).sax).externalSubset;
        *fresh62 = *(value as *mut externalSubsetSAXFunc);
    } else {
        return -(1 as i32)
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn xmlDecodeEntities<'a1, 'a2>(
    mut ctxt: Option<&'a1 mut crate::src::HTMLparser::_xmlParserCtxt>,
    mut len: i32,
    mut what: i32,
    mut end: u8,
    mut end2: u8,
    mut end3: u8,
) -> Option<&'a2 mut u8> {
    static mut deprecated: i32 = 0 as i32;
    if deprecated == 0 {
        (*(borrow(& __xmlGenericError())).unwrap())
            .expect(
                "non-null function pointer",
            )(
            *(__xmlGenericErrorContext()).unwrap(),
            b"xmlDecodeEntities() deprecated function reached\n\0" as *const u8
                as *const i8,
        );
        deprecated = 1 as i32;
    }
    return Option::<&'_ mut u8>::None;
}
#[no_mangle]
pub unsafe extern "C" fn xmlNamespaceParseNCName<'a1, 'a2>(
    mut ctxt: Option<&'a1 mut crate::src::HTMLparser::_xmlParserCtxt>,
) -> Option<&'a2 mut u8> {
    static mut deprecated: i32 = 0 as i32;
    if deprecated == 0 {
        (*(borrow(& __xmlGenericError())).unwrap())
            .expect(
                "non-null function pointer",
            )(
            *(__xmlGenericErrorContext()).unwrap(),
            b"xmlNamespaceParseNCName() deprecated function reached\n\0" as *const u8
                as *const i8,
        );
        deprecated = 1 as i32;
    }
    return Option::<&'_ mut u8>::None;
}
#[no_mangle]
pub unsafe extern "C" fn xmlNamespaceParseQName<'a1, 'a2, 'a3, 'a4>(
    mut ctxt: Option<&'a1 mut crate::src::HTMLparser::_xmlParserCtxt>,
    mut prefix: Option<&'a2 mut Option<&'a3 mut u8>>,
) -> Option<&'a4 mut u8> {
    static mut deprecated: i32 = 0 as i32;
    if deprecated == 0 {
        (*(borrow(& __xmlGenericError())).unwrap())
            .expect(
                "non-null function pointer",
            )(
            *(__xmlGenericErrorContext()).unwrap(),
            b"xmlNamespaceParseQName() deprecated function reached\n\0" as *const u8
                as *const i8,
        );
        deprecated = 1 as i32;
    }
    return Option::<&'_ mut u8>::None;
}
#[no_mangle]
pub unsafe extern "C" fn xmlNamespaceParseNSDef<'a1, 'a2>(
    mut ctxt: Option<&'a1 mut crate::src::HTMLparser::_xmlParserCtxt>,
) -> Option<&'a2 mut u8> {
    static mut deprecated: i32 = 0 as i32;
    if deprecated == 0 {
        (*(borrow(& __xmlGenericError())).unwrap())
            .expect(
                "non-null function pointer",
            )(
            *(__xmlGenericErrorContext()).unwrap(),
            b"xmlNamespaceParseNSDef() deprecated function reached\n\0" as *const u8
                as *const i8,
        );
        deprecated = 1 as i32;
    }
    return Option::<&'_ mut u8>::None;
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseQuotedString<'a1, 'a2>(
    mut ctxt: Option<&'a1 mut crate::src::HTMLparser::_xmlParserCtxt>,
) -> Option<&'a2 mut u8> {
    static mut deprecated: i32 = 0 as i32;
    if deprecated == 0 {
        (*(borrow(& __xmlGenericError())).unwrap())
            .expect(
                "non-null function pointer",
            )(
            *(__xmlGenericErrorContext()).unwrap(),
            b"xmlParseQuotedString() deprecated function reached\n\0" as *const u8
                as *const i8,
        );
        deprecated = 1 as i32;
    }
    return Option::<&'_ mut u8>::None;
}
#[no_mangle]
pub unsafe extern "C" fn xmlParseNamespace<'a1>(mut ctxt: Option<&'a1 mut crate::src::HTMLparser::_xmlParserCtxt>) {
    static mut deprecated: i32 = 0 as i32;
    if deprecated == 0 {
        (*(borrow(& __xmlGenericError())).unwrap())
            .expect(
                "non-null function pointer",
            )(
            *(__xmlGenericErrorContext()).unwrap(),
            b"xmlParseNamespace() deprecated function reached\n\0" as *const u8
                as *const i8,
        );
        deprecated = 1 as i32;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlScanName<'a1, 'a2>(mut ctxt: Option<&'a1 mut crate::src::HTMLparser::_xmlParserCtxt>) -> Option<&'a2 mut u8> {
    static mut deprecated: i32 = 0 as i32;
    if deprecated == 0 {
        (*(borrow(& __xmlGenericError())).unwrap())
            .expect(
                "non-null function pointer",
            )(
            *(__xmlGenericErrorContext()).unwrap(),
            b"xmlScanName() deprecated function reached\n\0" as *const u8
                as *const i8,
        );
        deprecated = 1 as i32;
    }
    return Option::<&'_ mut u8>::None;
}
#[no_mangle]
pub unsafe extern "C" fn xmlParserHandleReference<'a1>(mut ctxt: Option<&'a1 mut crate::src::HTMLparser::_xmlParserCtxt>) {
    static mut deprecated: i32 = 0 as i32;
    if deprecated == 0 {
        (*(borrow(& __xmlGenericError())).unwrap())
            .expect(
                "non-null function pointer",
            )(
            *(__xmlGenericErrorContext()).unwrap(),
            b"xmlParserHandleReference() deprecated function reached\n\0" as *const u8
                as *const i8,
        );
        deprecated = 1 as i32;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlHandleEntity<'a1, 'a2>(
    mut ctxt: Option<&'a1 mut crate::src::HTMLparser::_xmlParserCtxt>,
    mut entity: Option<&'a2 mut crate::src::HTMLparser::_xmlEntity>,
) {
    static mut deprecated: i32 = 0 as i32;
    if deprecated == 0 {
        (*(borrow(& __xmlGenericError())).unwrap())
            .expect(
                "non-null function pointer",
            )(
            *(__xmlGenericErrorContext()).unwrap(),
            b"xmlHandleEntity() deprecated function reached\n\0" as *const u8
                as *const i8,
        );
        deprecated = 1 as i32;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlNewGlobalNs<'a1, 'a2, 'a3, 'a4>(
    mut doc: Option<&'a1 mut crate::src::HTMLparser::_xmlDoc>,
    mut href: Option<&'a2 u8>,
    mut prefix: Option<&'a3 u8>,
) -> Option<&'a4 mut crate::src::HTMLparser::_xmlNs> {
    static mut deprecated: i32 = 0 as i32;
    if deprecated == 0 {
        (*(borrow(& __xmlGenericError())).unwrap())
            .expect(
                "non-null function pointer",
            )(
            *(__xmlGenericErrorContext()).unwrap(),
            b"xmlNewGlobalNs() deprecated function reached\n\0" as *const u8
                as *const i8,
        );
        deprecated = 1 as i32;
    }
    return Option::<&'_ mut crate::src::HTMLparser::_xmlNs>::None;
}
#[no_mangle]
pub unsafe extern "C" fn xmlUpgradeOldNs<'a1>(mut doc: Option<&'a1 mut crate::src::HTMLparser::_xmlDoc>) {
    static mut deprecated: i32 = 0 as i32;
    if deprecated == 0 {
        (*(borrow(& __xmlGenericError())).unwrap())
            .expect(
                "non-null function pointer",
            )(
            *(__xmlGenericErrorContext()).unwrap(),
            b"xmlUpgradeOldNs() deprecated function reached\n\0" as *const u8
                as *const i8,
        );
        deprecated = 1 as i32;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlEncodeEntities<'a1, 'a2, 'a3>(
    mut doc: Option<&'a1 mut crate::src::HTMLparser::_xmlDoc>,
    mut input: Option<&'a2 u8>,
) -> Option<&'a3 u8> {
    static mut warning: i32 = 1 as i32;
    if warning != 0 {
        (*(borrow(& __xmlGenericError())).unwrap())
            .expect(
                "non-null function pointer",
            )(
            *(__xmlGenericErrorContext()).unwrap(),
            b"Deprecated API xmlEncodeEntities() used\n\0" as *const u8
                as *const i8,
        );
        (*(borrow(& __xmlGenericError())).unwrap())
            .expect(
                "non-null function pointer",
            )(
            *(__xmlGenericErrorContext()).unwrap(),
            b"   change code to use xmlEncodeEntitiesReentrant()\n\0" as *const u8
                as *const i8,
        );
        warning = 0 as i32;
    }
    return Option::<&'_ u8>::None;
}
static mut deprecated_v1_msg: i32 = 0 as i32;
#[no_mangle]
pub unsafe extern "C" fn getPublicId(mut ctx: * mut core::ffi::c_void) -> * const u8 {
    if deprecated_v1_msg == 0 as i32 {
        (*(borrow(& __xmlGenericError())).unwrap())
            .expect(
                "non-null function pointer",
            )(
            *(__xmlGenericErrorContext()).unwrap(),
            b"Use of deprecated SAXv1 function %s\n\0" as *const u8
                as *const i8,
            b"getPublicId\0" as *const u8 as *const i8,
        );
    }
    deprecated_v1_msg += 1;
    return xmlSAX2GetPublicId(ctx);
}
#[no_mangle]
pub unsafe extern "C" fn getSystemId(mut ctx: * mut core::ffi::c_void) -> * const u8 {
    if deprecated_v1_msg == 0 as i32 {
        (*(borrow(& __xmlGenericError())).unwrap())
            .expect(
                "non-null function pointer",
            )(
            *(__xmlGenericErrorContext()).unwrap(),
            b"Use of deprecated SAXv1 function %s\n\0" as *const u8
                as *const i8,
            b"getSystemId\0" as *const u8 as *const i8,
        );
    }
    deprecated_v1_msg += 1;
    return xmlSAX2GetSystemId(ctx);
}
#[no_mangle]
pub unsafe extern "C" fn getLineNumber(mut ctx: * mut core::ffi::c_void) -> i32 {
    if deprecated_v1_msg == 0 as i32 {
        (*(borrow(& __xmlGenericError())).unwrap())
            .expect(
                "non-null function pointer",
            )(
            *(__xmlGenericErrorContext()).unwrap(),
            b"Use of deprecated SAXv1 function %s\n\0" as *const u8
                as *const i8,
            b"getLineNumber\0" as *const u8 as *const i8,
        );
    }
    deprecated_v1_msg += 1;
    return xmlSAX2GetLineNumber(ctx);
}
#[no_mangle]
pub unsafe extern "C" fn getColumnNumber(mut ctx: * mut core::ffi::c_void) -> i32 {
    if deprecated_v1_msg == 0 as i32 {
        (*(borrow(& __xmlGenericError())).unwrap())
            .expect(
                "non-null function pointer",
            )(
            *(__xmlGenericErrorContext()).unwrap(),
            b"Use of deprecated SAXv1 function %s\n\0" as *const u8
                as *const i8,
            b"getColumnNumber\0" as *const u8 as *const i8,
        );
    }
    deprecated_v1_msg += 1;
    return xmlSAX2GetColumnNumber(ctx);
}
#[no_mangle]
pub unsafe extern "C" fn isStandalone(mut ctx: * mut core::ffi::c_void) -> i32 {
    if deprecated_v1_msg == 0 as i32 {
        (*(borrow(& __xmlGenericError())).unwrap())
            .expect(
                "non-null function pointer",
            )(
            *(__xmlGenericErrorContext()).unwrap(),
            b"Use of deprecated SAXv1 function %s\n\0" as *const u8
                as *const i8,
            b"isStandalone\0" as *const u8 as *const i8,
        );
    }
    deprecated_v1_msg += 1;
    return xmlSAX2IsStandalone(ctx);
}
#[no_mangle]
pub unsafe extern "C" fn hasInternalSubset(mut ctx: * mut core::ffi::c_void) -> i32 {
    if deprecated_v1_msg == 0 as i32 {
        (*(borrow(& __xmlGenericError())).unwrap())
            .expect(
                "non-null function pointer",
            )(
            *(__xmlGenericErrorContext()).unwrap(),
            b"Use of deprecated SAXv1 function %s\n\0" as *const u8
                as *const i8,
            b"hasInternalSubset\0" as *const u8 as *const i8,
        );
    }
    deprecated_v1_msg += 1;
    return xmlSAX2HasInternalSubset(ctx);
}
#[no_mangle]
pub unsafe extern "C" fn hasExternalSubset(mut ctx: * mut core::ffi::c_void) -> i32 {
    if deprecated_v1_msg == 0 as i32 {
        (*(borrow(& __xmlGenericError())).unwrap())
            .expect(
                "non-null function pointer",
            )(
            *(__xmlGenericErrorContext()).unwrap(),
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
    mut ctx: * mut core::ffi::c_void,
    mut name: * const u8,
    mut ExternalID: * const u8,
    mut SystemID: * const u8,
) {
    if deprecated_v1_msg == 0 as i32 {
        (*(borrow(& __xmlGenericError())).unwrap())
            .expect(
                "non-null function pointer",
            )(
            *(__xmlGenericErrorContext()).unwrap(),
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
    mut ctx: * mut core::ffi::c_void,
    mut name: * const u8,
    mut ExternalID: * const u8,
    mut SystemID: * const u8,
) {
    if deprecated_v1_msg == 0 as i32 {
        (*(borrow(& __xmlGenericError())).unwrap())
            .expect(
                "non-null function pointer",
            )(
            *(__xmlGenericErrorContext()).unwrap(),
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
    mut ctx: * mut core::ffi::c_void,
    mut publicId: * const u8,
    mut systemId: * const u8,
) -> * mut crate::src::HTMLparser::_xmlParserInput {
    if deprecated_v1_msg == 0 as i32 {
        (*(borrow(& __xmlGenericError())).unwrap())
            .expect(
                "non-null function pointer",
            )(
            *(__xmlGenericErrorContext()).unwrap(),
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
    mut ctx: * mut core::ffi::c_void,
    mut name: * const u8,
) -> * mut crate::src::HTMLparser::_xmlEntity {
    if deprecated_v1_msg == 0 as i32 {
        (*(borrow(& __xmlGenericError())).unwrap())
            .expect(
                "non-null function pointer",
            )(
            *(__xmlGenericErrorContext()).unwrap(),
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
    mut ctx: * mut core::ffi::c_void,
    mut name: * const u8,
) -> * mut crate::src::HTMLparser::_xmlEntity {
    if deprecated_v1_msg == 0 as i32 {
        (*(borrow(& __xmlGenericError())).unwrap())
            .expect(
                "non-null function pointer",
            )(
            *(__xmlGenericErrorContext()).unwrap(),
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
    mut ctx: * mut core::ffi::c_void,
    mut name: * const u8,
    mut type_0: i32,
    mut publicId: * const u8,
    mut systemId: * const u8,
    mut content: * mut u8,
) {
    if deprecated_v1_msg == 0 as i32 {
        (*(borrow(& __xmlGenericError())).unwrap())
            .expect(
                "non-null function pointer",
            )(
            *(__xmlGenericErrorContext()).unwrap(),
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
    mut ctx: * mut core::ffi::c_void,
    mut elem: * const u8,
    mut fullname: * const u8,
    mut type_0: i32,
    mut def: i32,
    mut defaultValue: * const u8,
    mut tree: * mut crate::src::HTMLparser::_xmlEnumeration,
) {
    if deprecated_v1_msg == 0 as i32 {
        (*(borrow(& __xmlGenericError())).unwrap())
            .expect(
                "non-null function pointer",
            )(
            *(__xmlGenericErrorContext()).unwrap(),
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
    mut ctx: * mut core::ffi::c_void,
    mut name: * const u8,
    mut type_0: i32,
    mut content: * mut crate::src::HTMLparser::_xmlElementContent,
) {
    if deprecated_v1_msg == 0 as i32 {
        (*(borrow(& __xmlGenericError())).unwrap())
            .expect(
                "non-null function pointer",
            )(
            *(__xmlGenericErrorContext()).unwrap(),
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
    mut ctx: * mut core::ffi::c_void,
    mut name: * const u8,
    mut publicId: * const u8,
    mut systemId: * const u8,
) {
    if deprecated_v1_msg == 0 as i32 {
        (*(borrow(& __xmlGenericError())).unwrap())
            .expect(
                "non-null function pointer",
            )(
            *(__xmlGenericErrorContext()).unwrap(),
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
    mut ctx: * mut core::ffi::c_void,
    mut name: * const u8,
    mut publicId: * const u8,
    mut systemId: * const u8,
    mut notationName: * const u8,
) {
    if deprecated_v1_msg == 0 as i32 {
        (*(borrow(& __xmlGenericError())).unwrap())
            .expect(
                "non-null function pointer",
            )(
            *(__xmlGenericErrorContext()).unwrap(),
            b"Use of deprecated SAXv1 function %s\n\0" as *const u8
                as *const i8,
            b"unparsedEntityDecl\0" as *const u8 as *const i8,
        );
    }
    deprecated_v1_msg += 1;
    xmlSAX2UnparsedEntityDecl(ctx, name, publicId, systemId, notationName);
}
#[no_mangle]
pub unsafe extern "C" fn setDocumentLocator<'a1, 'a2>(
    mut ctx: Option<&'a1 mut core::ffi::c_void>,
    mut loc: Option<&'a2 mut crate::src::HTMLparser::_xmlSAXLocator>,
) {
    if deprecated_v1_msg == 0 as i32 {
        (*(borrow(& __xmlGenericError())).unwrap())
            .expect(
                "non-null function pointer",
            )(
            *(__xmlGenericErrorContext()).unwrap(),
            b"Use of deprecated SAXv1 function %s\n\0" as *const u8
                as *const i8,
            b"setDocumentLocator\0" as *const u8 as *const i8,
        );
    }
    deprecated_v1_msg += 1;
}
#[no_mangle]
pub unsafe extern "C" fn startDocument(mut ctx: * mut core::ffi::c_void) {
    xmlSAX2StartDocument(ctx);
}
#[no_mangle]
pub unsafe extern "C" fn endDocument(mut ctx: * mut core::ffi::c_void) {
    if deprecated_v1_msg == 0 as i32 {
        (*(borrow(& __xmlGenericError())).unwrap())
            .expect(
                "non-null function pointer",
            )(
            *(__xmlGenericErrorContext()).unwrap(),
            b"Use of deprecated SAXv1 function %s\n\0" as *const u8
                as *const i8,
            b"endDocument\0" as *const u8 as *const i8,
        );
    }
    deprecated_v1_msg += 1;
    xmlSAX2EndDocument(ctx);
}
#[no_mangle]
pub unsafe extern "C" fn attribute<'a1, 'a2, 'a3>(
    mut ctx: Option<&'a1 mut core::ffi::c_void>,
    mut fullname: Option<&'a2 u8>,
    mut value: Option<&'a3 u8>,
) {
    if deprecated_v1_msg == 0 as i32 {
        (*(borrow(& __xmlGenericError())).unwrap())
            .expect(
                "non-null function pointer",
            )(
            *(__xmlGenericErrorContext()).unwrap(),
            b"Use of deprecated SAXv1 function %s\n\0" as *const u8
                as *const i8,
            b"attribute\0" as *const u8 as *const i8,
        );
    }
    deprecated_v1_msg += 1;
}
#[no_mangle]
pub unsafe extern "C" fn startElement(
    mut ctx: * mut core::ffi::c_void,
    mut fullname: * const u8,
    mut atts: * mut * const u8,
) {
    xmlSAX2StartElement(ctx, fullname, atts);
}
#[no_mangle]
pub unsafe extern "C" fn endElement(
    mut ctx: * mut core::ffi::c_void,
    mut name: * const u8,
) {
    if deprecated_v1_msg == 0 as i32 {
        (*(borrow(& __xmlGenericError())).unwrap())
            .expect(
                "non-null function pointer",
            )(
            *(__xmlGenericErrorContext()).unwrap(),
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
    mut ctx: * mut core::ffi::c_void,
    mut name: * const u8,
) {
    if deprecated_v1_msg == 0 as i32 {
        (*(borrow(& __xmlGenericError())).unwrap())
            .expect(
                "non-null function pointer",
            )(
            *(__xmlGenericErrorContext()).unwrap(),
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
    mut ctx: * mut core::ffi::c_void,
    mut ch: * const u8,
    mut len: i32,
) {
    if deprecated_v1_msg == 0 as i32 {
        (*(borrow(& __xmlGenericError())).unwrap())
            .expect(
                "non-null function pointer",
            )(
            *(__xmlGenericErrorContext()).unwrap(),
            b"Use of deprecated SAXv1 function %s\n\0" as *const u8
                as *const i8,
            b"characters\0" as *const u8 as *const i8,
        );
    }
    deprecated_v1_msg += 1;
    xmlSAX2Characters(ctx, ch, len);
}
#[no_mangle]
pub unsafe extern "C" fn ignorableWhitespace<'a1, 'a2>(
    mut ctx: Option<&'a1 mut core::ffi::c_void>,
    mut ch: Option<&'a2 u8>,
    mut len: i32,
) {
    if deprecated_v1_msg == 0 as i32 {
        (*(borrow(& __xmlGenericError())).unwrap())
            .expect(
                "non-null function pointer",
            )(
            *(__xmlGenericErrorContext()).unwrap(),
            b"Use of deprecated SAXv1 function %s\n\0" as *const u8
                as *const i8,
            b"ignorableWhitespace\0" as *const u8 as *const i8,
        );
    }
    deprecated_v1_msg += 1;
}
#[no_mangle]
pub unsafe extern "C" fn processingInstruction(
    mut ctx: * mut core::ffi::c_void,
    mut target: * const u8,
    mut data: * const u8,
) {
    if deprecated_v1_msg == 0 as i32 {
        (*(borrow(& __xmlGenericError())).unwrap())
            .expect(
                "non-null function pointer",
            )(
            *(__xmlGenericErrorContext()).unwrap(),
            b"Use of deprecated SAXv1 function %s\n\0" as *const u8
                as *const i8,
            b"processingInstruction\0" as *const u8 as *const i8,
        );
    }
    deprecated_v1_msg += 1;
    xmlSAX2ProcessingInstruction(ctx, target, data);
}
#[no_mangle]
pub unsafe extern "C" fn globalNamespace<'a1, 'a2, 'a3>(
    mut ctx: Option<&'a1 mut core::ffi::c_void>,
    mut href: Option<&'a2 u8>,
    mut prefix: Option<&'a3 u8>,
) {
    if deprecated_v1_msg == 0 as i32 {
        (*(borrow(& __xmlGenericError())).unwrap())
            .expect(
                "non-null function pointer",
            )(
            *(__xmlGenericErrorContext()).unwrap(),
            b"Use of deprecated SAXv1 function %s\n\0" as *const u8
                as *const i8,
            b"globalNamespace\0" as *const u8 as *const i8,
        );
    }
    deprecated_v1_msg += 1;
}
#[no_mangle]
pub unsafe extern "C" fn setNamespace<'a1, 'a2>(
    mut ctx: Option<&'a1 mut core::ffi::c_void>,
    mut name: Option<&'a2 u8>,
) {
    if deprecated_v1_msg == 0 as i32 {
        (*(borrow(& __xmlGenericError())).unwrap())
            .expect(
                "non-null function pointer",
            )(
            *(__xmlGenericErrorContext()).unwrap(),
            b"Use of deprecated SAXv1 function %s\n\0" as *const u8
                as *const i8,
            b"setNamespace\0" as *const u8 as *const i8,
        );
    }
    deprecated_v1_msg += 1;
}
#[no_mangle]
pub unsafe extern "C" fn getNamespace<'a1, 'a2>(mut ctx: Option<&'a1 mut core::ffi::c_void>) -> Option<&'a2 mut crate::src::HTMLparser::_xmlNs> {
    if deprecated_v1_msg == 0 as i32 {
        (*(borrow(& __xmlGenericError())).unwrap())
            .expect(
                "non-null function pointer",
            )(
            *(__xmlGenericErrorContext()).unwrap(),
            b"Use of deprecated SAXv1 function %s\n\0" as *const u8
                as *const i8,
            b"getNamespace\0" as *const u8 as *const i8,
        );
    }
    deprecated_v1_msg += 1;
    return Option::<&'_ mut crate::src::HTMLparser::_xmlNs>::None;
}
#[no_mangle]
pub unsafe extern "C" fn checkNamespace<'a1, 'a2>(
    mut ctx: Option<&'a1 mut core::ffi::c_void>,
    mut namespace: Option<&'a2 mut u8>,
) -> i32 {
    if deprecated_v1_msg == 0 as i32 {
        (*(borrow(& __xmlGenericError())).unwrap())
            .expect(
                "non-null function pointer",
            )(
            *(__xmlGenericErrorContext()).unwrap(),
            b"Use of deprecated SAXv1 function %s\n\0" as *const u8
                as *const i8,
            b"checkNamespace\0" as *const u8 as *const i8,
        );
    }
    deprecated_v1_msg += 1;
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn namespaceDecl<'a1, 'a2, 'a3>(
    mut ctx: Option<&'a1 mut core::ffi::c_void>,
    mut href: Option<&'a2 u8>,
    mut prefix: Option<&'a3 u8>,
) {
    if deprecated_v1_msg == 0 as i32 {
        (*(borrow(& __xmlGenericError())).unwrap())
            .expect(
                "non-null function pointer",
            )(
            *(__xmlGenericErrorContext()).unwrap(),
            b"Use of deprecated SAXv1 function %s\n\0" as *const u8
                as *const i8,
            b"namespaceDecl\0" as *const u8 as *const i8,
        );
    }
    deprecated_v1_msg += 1;
}
#[no_mangle]
pub unsafe extern "C" fn comment(mut ctx: * mut core::ffi::c_void, mut value: * const u8) {
    if deprecated_v1_msg == 0 as i32 {
        (*(borrow(& __xmlGenericError())).unwrap())
            .expect(
                "non-null function pointer",
            )(
            *(__xmlGenericErrorContext()).unwrap(),
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
    mut ctx: * mut core::ffi::c_void,
    mut value: * const u8,
    mut len: i32,
) {
    if deprecated_v1_msg == 0 as i32 {
        (*(borrow(& __xmlGenericError())).unwrap())
            .expect(
                "non-null function pointer",
            )(
            *(__xmlGenericErrorContext()).unwrap(),
            b"Use of deprecated SAXv1 function %s\n\0" as *const u8
                as *const i8,
            b"cdataBlock\0" as *const u8 as *const i8,
        );
    }
    deprecated_v1_msg += 1;
    xmlSAX2CDataBlock(ctx, value, len);
}
use crate::laertes_rt::*;
