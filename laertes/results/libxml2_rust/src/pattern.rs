use :: libc;
extern "C" {
    fn xmlStrEqual(str1: *const u8, str2: *const u8) -> i32;
    fn xmlStrndup(cur: *const u8, len: i32) -> *mut u8;
    fn xmlStrdup(cur: *const u8) -> *mut u8;
    fn memset(_: *mut core::ffi::c_void, _: i32, _: u64) -> *mut core::ffi::c_void;
}
pub use crate::src::{
    buf::_xmlBuf,
    catalog::_xmlAutomata,
    chvalid::{
        xmlCharInRange, xmlIsBaseCharGroup, xmlIsCombiningGroup, xmlIsDigitGroup,
        xmlIsExtenderGroup,
    },
    debugXML::_xmlValidState,
    dict::{_xmlDict, xmlDictFree, xmlDictLookup, xmlDictReference},
    encoding::_xmlAutomataState,
    globals::{xmlFree, xmlMalloc, xmlRealloc},
    hash::_xmlHashTable,
    parser::_xmlStartTag,
    parserInternals::xmlStringCurrentChar,
};
pub type xmlChar = u8;
pub type size_t = u64;
pub type xmlFreeFunc = Option<unsafe extern "C" fn(_: *mut core::ffi::c_void) -> ()>;
pub type xmlMallocFunc = Option<unsafe extern "C" fn(_: u64) -> *mut core::ffi::c_void>;
pub type xmlReallocFunc =
    Option<unsafe extern "C" fn(_: *mut core::ffi::c_void, _: u64) -> *mut core::ffi::c_void>;
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
pub type _xmlParserInput = crate::src::HTMLparser::_xmlParserInput;
pub type xmlParserInputDeallocate = Option<unsafe extern "C" fn(_: *mut u8) -> ()>;
pub type xmlParserInput = crate::src::HTMLparser::_xmlParserInput;
pub type xmlParserInputPtr = *mut crate::src::HTMLparser::_xmlParserInput;
pub type _xmlParserCtxt = crate::src::HTMLparser::_xmlParserCtxt;
pub type xmlParserNodeInfo = crate::src::HTMLparser::_xmlParserNodeInfo;
pub type _xmlParserNodeInfo = crate::src::HTMLparser::_xmlParserNodeInfo;
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
pub type xmlParserMode = u32;
pub const XML_PARSE_READER: xmlParserMode = 5;
pub const XML_PARSE_PUSH_SAX: xmlParserMode = 4;
pub const XML_PARSE_PUSH_DOM: xmlParserMode = 3;
pub const XML_PARSE_SAX: xmlParserMode = 2;
pub const XML_PARSE_DOM: xmlParserMode = 1;
pub const XML_PARSE_UNKNOWN: xmlParserMode = 0;
pub type xmlError = crate::src::HTMLparser::_xmlError;
pub type _xmlError = crate::src::HTMLparser::_xmlError;
pub type xmlErrorLevel = u32;
pub const XML_ERR_FATAL: xmlErrorLevel = 3;
pub const XML_ERR_ERROR: xmlErrorLevel = 2;
pub const XML_ERR_WARNING: xmlErrorLevel = 1;
pub const XML_ERR_NONE: xmlErrorLevel = 0;
pub type xmlAttrPtr = *mut crate::src::HTMLparser::_xmlAttr;
pub type xmlAttr = crate::src::HTMLparser::_xmlAttr;
pub type xmlNodePtr = *mut crate::src::HTMLparser::_xmlNode;
pub type xmlNode = crate::src::HTMLparser::_xmlNode;
pub type xmlHashTablePtr = *mut crate::src::hash::_xmlHashTable;
pub type xmlHashTable = crate::src::hash::_xmlHashTable;
pub type xmlStartTag = crate::src::parser::_xmlStartTag;
pub type xmlDictPtr = *mut crate::src::dict::_xmlDict;
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
pub type _xmlValidCtxt = crate::src::HTMLparser::_xmlValidCtxt;
pub type xmlAutomataStatePtr = *mut crate::src::encoding::_xmlAutomataState;
pub type xmlAutomataState = crate::src::encoding::_xmlAutomataState;
pub type xmlAutomataPtr = *mut crate::src::catalog::_xmlAutomata;
pub type xmlAutomata = crate::src::catalog::_xmlAutomata;
pub type xmlValidState = crate::src::debugXML::_xmlValidState;
pub type xmlDocPtr = *mut crate::src::HTMLparser::_xmlDoc;
pub type xmlDoc = crate::src::HTMLparser::_xmlDoc;
pub type xmlValidityWarningFunc =
    Option<unsafe extern "C" fn(_: *mut core::ffi::c_void, _: *const i8, ...) -> ()>;
pub type xmlValidityErrorFunc =
    Option<unsafe extern "C" fn(_: *mut core::ffi::c_void, _: *const i8, ...) -> ()>;
pub type xmlParserNodeInfoSeq = crate::src::HTMLparser::_xmlParserNodeInfoSeq;
pub type _xmlParserNodeInfoSeq = crate::src::HTMLparser::_xmlParserNodeInfoSeq;
pub type _xmlSAXHandler = crate::src::HTMLparser::_xmlSAXHandler;
pub type xmlStructuredErrorFunc = Option<
    unsafe extern "C" fn(
        _: *mut core::ffi::c_void,
        _: *mut crate::src::HTMLparser::_xmlError,
    ) -> (),
>;
pub type xmlErrorPtr = *mut crate::src::HTMLparser::_xmlError;
pub type endElementNsSAX2Func = Option<
    unsafe extern "C" fn(_: *mut core::ffi::c_void, _: *const u8, _: *const u8, _: *const u8) -> (),
>;
pub type startElementNsSAX2Func = Option<
    unsafe extern "C" fn(
        _: *mut core::ffi::c_void,
        _: *const u8,
        _: *const u8,
        _: *const u8,
        _: i32,
        _: *mut *const u8,
        _: i32,
        _: i32,
        _: *mut *const u8,
    ) -> (),
>;
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
pub type xmlParserCtxt = crate::src::HTMLparser::_xmlParserCtxt;
pub type xmlParserCtxtPtr = *mut crate::src::HTMLparser::_xmlParserCtxt;
pub type _xmlChSRange = crate::src::HTMLparser::_xmlChSRange;
pub type xmlChSRange = crate::src::HTMLparser::_xmlChSRange;
pub type _xmlChLRange = crate::src::HTMLparser::_xmlChLRange;
pub type xmlChLRange = crate::src::HTMLparser::_xmlChLRange;
pub type _xmlChRangeGroup = crate::src::HTMLparser::_xmlChRangeGroup;
pub type xmlChRangeGroup = crate::src::HTMLparser::_xmlChRangeGroup;
#[repr(C)]
pub struct _xmlPattern<'a> {
    pub data: Option<&'a mut core::ffi::c_void>,
    pub dict: *mut crate::src::dict::_xmlDict,
    pub next: *mut crate::src::pattern::_xmlPattern<'a>,
    pub pattern: *const u8,
    pub flags: i32,
    pub nbStep: i32,
    pub maxStep: i32,
    pub steps: *mut crate::src::pattern::_xmlStepOp,
    pub stream: *mut crate::src::pattern::_xmlStreamComp,
}
impl<'a> _xmlPattern<'a> {
    pub const fn new() -> Self {
        _xmlPattern {
            data: None,
            dict: (0 as *mut crate::src::dict::_xmlDict),
            next: (0 as *mut crate::src::pattern::_xmlPattern<'a>),
            pattern: (0 as *const u8),
            flags: 0,
            nbStep: 0,
            maxStep: 0,
            steps: (0 as *mut crate::src::pattern::_xmlStepOp),
            stream: (0 as *mut crate::src::pattern::_xmlStreamComp),
        }
    }
}
impl<'a> std::default::Default for _xmlPattern<'a> {
    fn default() -> Self {
        _xmlPattern::new()
    }
}
pub type xmlStreamCompPtr = *mut crate::src::pattern::_xmlStreamComp;
pub type xmlStreamComp = crate::src::pattern::_xmlStreamComp;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlStreamComp {
    pub dict: *mut crate::src::dict::_xmlDict,
    pub nbStep: i32,
    pub maxStep: i32,
    pub steps: *mut crate::src::pattern::_xmlStreamStep,
    pub flags: i32,
}
impl _xmlStreamComp {
    pub const fn new() -> Self {
        _xmlStreamComp {
            dict: (0 as *mut crate::src::dict::_xmlDict),
            nbStep: 0,
            maxStep: 0,
            steps: (0 as *mut crate::src::pattern::_xmlStreamStep),
            flags: 0,
        }
    }
}
impl std::default::Default for _xmlStreamComp {
    fn default() -> Self {
        _xmlStreamComp::new()
    }
}
pub type xmlStreamStepPtr = *mut crate::src::pattern::_xmlStreamStep;
pub type xmlStreamStep = crate::src::pattern::_xmlStreamStep;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlStreamStep {
    pub flags: i32,
    pub name: *const u8,
    pub ns: *const u8,
    pub nodeType: i32,
}
impl _xmlStreamStep {
    pub const fn new() -> Self {
        _xmlStreamStep {
            flags: 0,
            name: (0 as *const u8),
            ns: (0 as *const u8),
            nodeType: 0,
        }
    }
}
impl std::default::Default for _xmlStreamStep {
    fn default() -> Self {
        _xmlStreamStep::new()
    }
}
pub type xmlStepOpPtr = *mut crate::src::pattern::_xmlStepOp;
pub type xmlStepOp = crate::src::pattern::_xmlStepOp;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlStepOp {
    pub op: u32,
    pub value: *const u8,
    pub value2: *const u8,
}
impl _xmlStepOp {
    pub const fn new() -> Self {
        _xmlStepOp {
            op: 0,
            value: (0 as *const u8),
            value2: (0 as *const u8),
        }
    }
}
impl std::default::Default for _xmlStepOp {
    fn default() -> Self {
        _xmlStepOp::new()
    }
}
pub type xmlPatOp = u32;
pub const XML_OP_ALL: xmlPatOp = 8;
pub const XML_OP_NS: xmlPatOp = 7;
pub const XML_OP_ANCESTOR: xmlPatOp = 6;
pub const XML_OP_PARENT: xmlPatOp = 5;
pub const XML_OP_ATTR: xmlPatOp = 4;
pub const XML_OP_CHILD: xmlPatOp = 3;
pub const XML_OP_ELEM: xmlPatOp = 2;
pub const XML_OP_ROOT: xmlPatOp = 1;
pub const XML_OP_END: xmlPatOp = 0;
pub type xmlPattern<'a> = crate::src::pattern::_xmlPattern<'a>;
pub type xmlPatternPtr<'a> = *mut crate::src::pattern::_xmlPattern<'a>;
pub type C2RustUnnamed = u32;
pub const XML_PATTERN_XSFIELD: C2RustUnnamed = 4;
pub const XML_PATTERN_XSSEL: C2RustUnnamed = 2;
pub const XML_PATTERN_XPATH: C2RustUnnamed = 1;
pub const XML_PATTERN_DEFAULT: C2RustUnnamed = 0;
pub type xmlPatParserContextPtr<'a> = *mut crate::src::pattern::_xmlPatParserContext<'a>;
pub type xmlPatParserContext<'a> = crate::src::pattern::_xmlPatParserContext<'a>;
#[repr(C)]
pub struct _xmlPatParserContext<'a> {
    pub cur: *const u8,
    pub base: *const u8,
    pub error: i32,
    pub dict: *mut crate::src::dict::_xmlDict,
    pub comp: *mut crate::src::pattern::_xmlPattern<'a>,
    pub elem: Option<&'a mut crate::src::HTMLparser::_xmlNode>,
    pub namespaces: *mut *const u8,
    pub nb_namespaces: i32,
}
impl<'a> _xmlPatParserContext<'a> {
    pub const fn new() -> Self {
        _xmlPatParserContext {
            cur: (0 as *const u8),
            base: (0 as *const u8),
            error: 0,
            dict: (0 as *mut crate::src::dict::_xmlDict),
            comp: (0 as *mut crate::src::pattern::_xmlPattern<'a>),
            elem: None,
            namespaces: (0 as *mut *const u8),
            nb_namespaces: 0,
        }
    }
}
impl<'a> std::default::Default for _xmlPatParserContext<'a> {
    fn default() -> Self {
        _xmlPatParserContext::new()
    }
}
pub type xmlStepStatePtr = *mut crate::src::pattern::_xmlStepState;
pub type xmlStepState = crate::src::pattern::_xmlStepState;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlStepState {
    pub step: i32,
    pub node: *mut crate::src::HTMLparser::_xmlNode,
}
impl _xmlStepState {
    pub const fn new() -> Self {
        _xmlStepState {
            step: 0,
            node: (0 as *mut crate::src::HTMLparser::_xmlNode),
        }
    }
}
impl std::default::Default for _xmlStepState {
    fn default() -> Self {
        _xmlStepState::new()
    }
}
pub type xmlStepStates = crate::src::pattern::_xmlStepStates;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlStepStates {
    pub nbstates: i32,
    pub maxstates: i32,
    pub states: *mut crate::src::pattern::_xmlStepState,
}
impl _xmlStepStates {
    pub const fn new() -> Self {
        _xmlStepStates {
            nbstates: 0,
            maxstates: 0,
            states: (0 as *mut crate::src::pattern::_xmlStepState),
        }
    }
}
impl std::default::Default for _xmlStepStates {
    fn default() -> Self {
        _xmlStepStates::new()
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlStreamCtxt {
    pub next: *mut crate::src::pattern::_xmlStreamCtxt,
    pub comp: *mut crate::src::pattern::_xmlStreamComp,
    pub nbState: i32,
    pub maxState: i32,
    pub level: i32,
    pub states: *mut i32,
    pub flags: i32,
    pub blockLevel: i32,
}
impl _xmlStreamCtxt {
    pub const fn new() -> Self {
        _xmlStreamCtxt {
            next: (0 as *mut crate::src::pattern::_xmlStreamCtxt),
            comp: (0 as *mut crate::src::pattern::_xmlStreamComp),
            nbState: 0,
            maxState: 0,
            level: 0,
            states: (0 as *mut i32),
            flags: 0,
            blockLevel: 0,
        }
    }
}
impl std::default::Default for _xmlStreamCtxt {
    fn default() -> Self {
        _xmlStreamCtxt::new()
    }
}
pub type xmlStreamCtxt = crate::src::pattern::_xmlStreamCtxt;
pub type xmlStreamCtxtPtr = *mut crate::src::pattern::_xmlStreamCtxt;
extern "C" fn xmlNewPattern<'a1>() -> *mut crate::src::pattern::_xmlPattern<'a1> {
    let mut cur: *mut crate::src::pattern::_xmlPattern<'_> = 0 as *mut xmlPattern;
    cur = (unsafe { xmlMalloc.expect("non-null function pointer")(::std::mem::size_of::<xmlPattern>() as u64) })
        as xmlPatternPtr;
    if cur.is_null() {
        return 0 as xmlPatternPtr;
    }
    (unsafe { memset(
        cur as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<xmlPattern>() as u64,
    ) });
    (unsafe { (*cur).maxStep = 10 as i32 });
    let fresh0 = unsafe { &mut ((*cur).steps) };
    *fresh0 = (unsafe { xmlMalloc.expect("non-null function pointer")(
        ((*cur).maxStep as u64).wrapping_mul(::std::mem::size_of::<xmlStepOp>() as u64),
    ) }) as xmlStepOpPtr;
    if (unsafe { (*cur).steps }).is_null() {
        (unsafe { xmlFree.expect("non-null function pointer")(cur as *mut libc::c_void) });
        return 0 as xmlPatternPtr;
    }
    return cur;
}
#[no_mangle]
pub extern "C" fn xmlFreePattern<'a1>(mut comp: *mut crate::src::pattern::_xmlPattern<'a1>) {
    xmlFreePatternList(comp);
}
extern "C" fn xmlFreePatternInternal<'a1>(mut comp: *mut crate::src::pattern::_xmlPattern<'a1>) {
    let mut op: Option<&'_ mut crate::src::pattern::_xmlStepOp> =
        Option::<&'_ mut crate::src::pattern::_xmlStepOp>::None;
    let mut i: i32 = 0;
    if comp.is_null() {
        return;
    }
    if !(unsafe { (*comp).stream }).is_null() {
        xmlFreeStreamComp(unsafe { (*comp).stream });
    }
    if !(unsafe { (*comp).pattern }).is_null() {
        (unsafe { xmlFree.expect("non-null function pointer")(
            (*comp).pattern as *mut xmlChar as *mut libc::c_void,
        ) });
    }
    if !(unsafe { (*comp).steps }).is_null() {
        if (unsafe { (*comp).dict }).is_null() {
            i = 0 as i32;
            while i < (unsafe { (*comp).nbStep }) {
                op = Some(unsafe { &mut *((*comp).steps).offset(i as isize) });
                if !((*(borrow_mut(&mut op)).unwrap()).value).is_null() {
                    (unsafe { xmlFree.expect("non-null function pointer")(
                        (*(borrow_mut(&mut op)).unwrap()).value as *mut xmlChar
                            as *mut libc::c_void,
                    ) });
                }
                if !((*(borrow_mut(&mut op)).unwrap()).value2).is_null() {
                    (unsafe { xmlFree.expect("non-null function pointer")(
                        (*(borrow_mut(&mut op)).unwrap()).value2 as *mut xmlChar
                            as *mut libc::c_void,
                    ) });
                }
                i += 1;
            }
        }
        (unsafe { xmlFree.expect("non-null function pointer")((*comp).steps as *mut libc::c_void) });
    }
    if !(unsafe { (*comp).dict }).is_null() {
        xmlDictFree(unsafe { (*comp).dict });
    }
    (unsafe { memset(
        comp as *mut libc::c_void,
        -(1 as i32),
        ::std::mem::size_of::<xmlPattern>() as u64,
    ) });
    (unsafe { xmlFree.expect("non-null function pointer")(comp as *mut libc::c_void) });
}
#[no_mangle]
pub extern "C" fn xmlFreePatternList<'a1>(mut comp: *mut crate::src::pattern::_xmlPattern<'a1>) {
    let mut cur: *mut crate::src::pattern::_xmlPattern<'_> = 0 as *mut xmlPattern;
    while !comp.is_null() {
        cur = comp;
        comp = unsafe { (*comp).next };
        let fresh1 = unsafe { &mut ((*cur).next) };
        *fresh1 = 0 as *mut _xmlPattern;
        xmlFreePatternInternal(cur);
    }
}
extern "C" fn xmlNewPatParserContext<'a1>(
    mut pattern: *const u8,
    mut dict: *mut crate::src::dict::_xmlDict,
    mut namespaces: *mut *const u8,
) -> *mut crate::src::pattern::_xmlPatParserContext<'a1> {
    let mut cur: *mut crate::src::pattern::_xmlPatParserContext<'_> = 0 as *mut xmlPatParserContext;
    if pattern.is_null() {
        return 0 as xmlPatParserContextPtr;
    }
    cur = (unsafe { xmlMalloc.expect("non-null function pointer")(
        ::std::mem::size_of::<xmlPatParserContext>() as u64,
    ) }) as xmlPatParserContextPtr;
    if cur.is_null() {
        return 0 as xmlPatParserContextPtr;
    }
    (unsafe { memset(
        cur as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<xmlPatParserContext>() as u64,
    ) });
    let fresh2 = unsafe { &mut ((*cur).dict) };
    *fresh2 = dict;
    let fresh3 = unsafe { &mut ((*cur).cur) };
    *fresh3 = pattern;
    let fresh4 = unsafe { &mut ((*cur).base) };
    *fresh4 = pattern;
    if !namespaces.is_null() {
        let mut i: i32 = 0;
        i = 0 as i32;
        while !(unsafe { *namespaces.offset((2 as i32 * i) as isize) }).is_null() {
            i += 1;
        }
        (unsafe { (*cur).nb_namespaces = i });
    } else {
        (unsafe { (*cur).nb_namespaces = 0 as i32 });
    }
    let fresh5 = unsafe { &mut ((*cur).namespaces) };
    *fresh5 = namespaces;
    return cur;
}
extern "C" fn xmlFreePatParserContext<'a1>(
    mut ctxt: *mut crate::src::pattern::_xmlPatParserContext<'a1>,
) {
    if ctxt.is_null() {
        return;
    }
    (unsafe { memset(
        ctxt as *mut libc::c_void,
        -(1 as i32),
        ::std::mem::size_of::<xmlPatParserContext>() as u64,
    ) });
    (unsafe { xmlFree.expect("non-null function pointer")(ctxt as *mut libc::c_void) });
}
extern "C" fn xmlPatternAdd<'a1, 'a2>(
    mut _ctxt: *mut crate::src::pattern::_xmlPatParserContext<'a1>,
    mut comp: *mut crate::src::pattern::_xmlPattern<'a2>,
    mut op: u32,
    mut value: *mut u8,
    mut value2: *mut u8,
) -> i32 {
    if (unsafe { (*comp).nbStep }) >= (unsafe { (*comp).maxStep }) {
        let mut temp: *mut crate::src::pattern::_xmlStepOp = 0 as *mut xmlStepOp;
        temp = (unsafe { xmlRealloc.expect("non-null function pointer")(
            (*comp).steps as *mut libc::c_void,
            (((*comp).maxStep * 2 as i32) as u64)
                .wrapping_mul(::std::mem::size_of::<xmlStepOp>() as u64),
        ) }) as xmlStepOpPtr;
        if temp.is_null() {
            return -(1 as i32);
        }
        let fresh6 = unsafe { &mut ((*comp).steps) };
        *fresh6 = temp;
        (unsafe { (*comp).maxStep *= 2 as i32 });
    }
    (unsafe { (*((*comp).steps).offset((*comp).nbStep as isize)).op = op });
    let fresh7 = unsafe { &mut ((*((*comp).steps).offset((*comp).nbStep as isize)).value) };
    *fresh7 = value;
    let fresh8 = unsafe { &mut ((*((*comp).steps).offset((*comp).nbStep as isize)).value2) };
    *fresh8 = value2;
    let fresh9 = unsafe { &mut ((*comp).nbStep) };
    *fresh9 += 1;
    return 0 as i32;
}
extern "C" fn xmlReversePattern<'a1>(mut comp: *mut crate::src::pattern::_xmlPattern<'a1>) -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    if (unsafe { (*comp).nbStep }) > 0 as i32
        && (unsafe { (*((*comp).steps).offset(0 as i32 as isize)).op }) as u32 == XML_OP_ANCESTOR as i32 as u32
    {
        i = 0 as i32;
        j = 1 as i32;
        while j < (unsafe { (*comp).nbStep }) {
            let fresh10 = unsafe { &mut ((*((*comp).steps).offset(i as isize)).value) };
            *fresh10 = unsafe { (*((*comp).steps).offset(j as isize)).value };
            let fresh11 = unsafe { &mut ((*((*comp).steps).offset(i as isize)).value2) };
            *fresh11 = unsafe { (*((*comp).steps).offset(j as isize)).value2 };
            (unsafe { (*((*comp).steps).offset(i as isize)).op = (*((*comp).steps).offset(j as isize)).op });
            i += 1;
            j += 1;
        }
        let fresh12 = unsafe { &mut ((*comp).nbStep) };
        *fresh12 -= 1;
    }
    if (unsafe { (*comp).nbStep }) >= (unsafe { (*comp).maxStep }) {
        let mut temp: *mut crate::src::pattern::_xmlStepOp = 0 as *mut xmlStepOp;
        temp = (unsafe { xmlRealloc.expect("non-null function pointer")(
            (*comp).steps as *mut libc::c_void,
            (((*comp).maxStep * 2 as i32) as u64)
                .wrapping_mul(::std::mem::size_of::<xmlStepOp>() as u64),
        ) }) as xmlStepOpPtr;
        if temp.is_null() {
            return -(1 as i32);
        }
        let fresh13 = unsafe { &mut ((*comp).steps) };
        *fresh13 = temp;
        (unsafe { (*comp).maxStep *= 2 as i32 });
    }
    i = 0 as i32;
    j = (unsafe { (*comp).nbStep }) - 1 as i32;
    while j > i {
        let mut tmp: *const u8 = 0 as *const xmlChar;
        let mut op: u32 = XML_OP_END;
        tmp = unsafe { (*((*comp).steps).offset(i as isize)).value };
        let fresh14 = unsafe { &mut ((*((*comp).steps).offset(i as isize)).value) };
        *fresh14 = unsafe { (*((*comp).steps).offset(j as isize)).value };
        let fresh15 = unsafe { &mut ((*((*comp).steps).offset(j as isize)).value) };
        *fresh15 = tmp;
        tmp = unsafe { (*((*comp).steps).offset(i as isize)).value2 };
        let fresh16 = unsafe { &mut ((*((*comp).steps).offset(i as isize)).value2) };
        *fresh16 = unsafe { (*((*comp).steps).offset(j as isize)).value2 };
        let fresh17 = unsafe { &mut ((*((*comp).steps).offset(j as isize)).value2) };
        *fresh17 = tmp;
        op = unsafe { (*((*comp).steps).offset(i as isize)).op };
        (unsafe { (*((*comp).steps).offset(i as isize)).op = (*((*comp).steps).offset(j as isize)).op });
        (unsafe { (*((*comp).steps).offset(j as isize)).op = op });
        j -= 1;
        i += 1;
    }
    let fresh18 = unsafe { &mut ((*((*comp).steps).offset((*comp).nbStep as isize)).value) };
    *fresh18 = 0 as *const xmlChar;
    let fresh19 = unsafe { &mut ((*((*comp).steps).offset((*comp).nbStep as isize)).value2) };
    *fresh19 = 0 as *const xmlChar;
    let fresh20 = unsafe { &mut ((*comp).nbStep) };
    let mut fresh21 = *fresh20;
    *fresh20 = *fresh20 + 1;
    (unsafe { (*((*comp).steps).offset(fresh21 as isize)).op = XML_OP_END });
    return 0 as i32;
}
extern "C" fn xmlPatPushState<'a1>(
    mut states: Option<&'a1 mut crate::src::pattern::_xmlStepStates>,
    mut step: i32,
    mut node: *mut crate::src::HTMLparser::_xmlNode,
) -> i32 {
    if ((*(borrow(&states)).unwrap()).states).is_null()
        || (*(borrow(&states)).unwrap()).maxstates <= 0 as i32
    {
        (*(borrow_mut(&mut states)).unwrap()).maxstates = 4 as i32;
        (*(borrow_mut(&mut states)).unwrap()).nbstates = 0 as i32;
        let fresh22 = &mut ((*(borrow_mut(&mut states)).unwrap()).states);
        *fresh22 = (unsafe { xmlMalloc.expect("non-null function pointer")(
            (4 as i32 as u64).wrapping_mul(::std::mem::size_of::<xmlStepState>() as u64),
        ) }) as xmlStepStatePtr;
    } else if (*(borrow(&states)).unwrap()).maxstates <= (*(borrow(&states)).unwrap()).nbstates {
        let mut tmp: *mut crate::src::pattern::_xmlStepState = 0 as *mut xmlStepState;
        tmp = (unsafe { xmlRealloc.expect("non-null function pointer")(
            (*(borrow_mut(&mut states)).unwrap()).states as *mut libc::c_void,
            ((2 as i32 * (*(borrow(&states)).unwrap()).maxstates) as u64)
                .wrapping_mul(::std::mem::size_of::<xmlStepState>() as u64),
        ) }) as xmlStepStatePtr;
        if tmp.is_null() {
            return -(1 as i32);
        }
        let fresh23 = &mut ((*(borrow_mut(&mut states)).unwrap()).states);
        *fresh23 = tmp;
        (*(borrow_mut(&mut states)).unwrap()).maxstates *= 2 as i32;
    }
    (unsafe { (*((*(borrow(&states)).unwrap()).states)
        .offset((*(borrow(&states)).unwrap()).nbstates as isize))
    .step = step });
    let fresh24 = &mut ((*(borrow_mut(&mut states)).unwrap()).nbstates);
    let mut fresh25 = *fresh24;
    *fresh24 = *fresh24 + 1;
    let fresh26 = unsafe { &mut ((*((*(borrow(&states)).unwrap()).states).offset(fresh25 as isize)).node) };
    *fresh26 = node;
    return 0 as i32;
}
extern "C" fn xmlPatMatch<'a1>(
    mut comp: *mut crate::src::pattern::_xmlPattern<'a1>,
    mut node: *mut crate::src::HTMLparser::_xmlNode,
) -> i32 {
    let mut current_block: u64;
    let mut i: i32 = 0;
    let mut step: Option<&'_ mut crate::src::pattern::_xmlStepOp> =
        Option::<&'_ mut crate::src::pattern::_xmlStepOp>::None;
    let mut states: crate::src::pattern::_xmlStepStates = {
        let mut init = _xmlStepStates {
            nbstates: 0 as i32,
            maxstates: 0 as i32,
            states: 0 as xmlStepStatePtr,
        };
        init
    };
    if comp.is_null() || node.is_null() {
        return -(1 as i32);
    }
    i = 0 as i32;
    while i < (unsafe { (*comp).nbStep }) {
        step = Some(unsafe { &mut *((*comp).steps).offset(i as isize) });
        match (*(borrow(&step)).unwrap()).op as u32 {
            0 => {
                break;
            },
            1 => {
                if (unsafe { (*node).type_0 }) as u32 == XML_NAMESPACE_DECL as i32 as u32 {
                    current_block = 6451473480150109090;
                } else {
                    node = unsafe { (*node).parent };
                    if (unsafe { (*node).type_0 }) as u32 == XML_DOCUMENT_NODE as i32 as u32
                        || (unsafe { (*node).type_0 }) as u32 == XML_HTML_DOCUMENT_NODE as i32 as u32
                    {
                        current_block = 820271813250567934;
                    } else {
                        current_block = 6451473480150109090;
                    }
                }
            },
            2 => {
                if (unsafe { (*node).type_0 }) as u32 != XML_ELEMENT_NODE as i32 as u32 {
                    current_block = 6451473480150109090;
                } else if ((*(borrow(&step)).unwrap()).value).is_null() {
                    current_block = 820271813250567934;
                } else if (unsafe { *((*(borrow(&step)).unwrap()).value).offset(0 as i32 as isize) }) as i32
                    != (unsafe { *((*node).name).offset(0 as i32 as isize) }) as i32
                {
                    current_block = 6451473480150109090;
                } else if (unsafe { xmlStrEqual((*(borrow(&step)).unwrap()).value, (*node).name) }) == 0 {
                    current_block = 6451473480150109090;
                } else if (unsafe { (*node).ns }).is_null() {
                    if !((*(borrow(&step)).unwrap()).value2).is_null() {
                        current_block = 6451473480150109090;
                    } else {
                        current_block = 820271813250567934;
                    }
                } else if !(unsafe { (*(*node).ns).href }).is_null() {
                    if ((*(borrow(&step)).unwrap()).value2).is_null() {
                        current_block = 6451473480150109090;
                    } else if (unsafe { xmlStrEqual((*(borrow(&step)).unwrap()).value2, (*(*node).ns).href) })
                        == 0
                    {
                        current_block = 6451473480150109090;
                    } else {
                        current_block = 820271813250567934;
                    }
                } else {
                    current_block = 820271813250567934;
                }
            },
            3 => {
                let mut lst: *mut crate::src::HTMLparser::_xmlNode = 0 as *mut xmlNode;
                if (unsafe { (*node).type_0 }) as u32 != XML_ELEMENT_NODE as i32 as u32
                    && (unsafe { (*node).type_0 }) as u32 != XML_DOCUMENT_NODE as i32 as u32
                    && (unsafe { (*node).type_0 }) as u32 != XML_HTML_DOCUMENT_NODE as i32 as u32
                {
                    current_block = 6451473480150109090;
                } else {
                    lst = unsafe { (*node).children };
                    if !((*(borrow(&step)).unwrap()).value).is_null() {
                        while !lst.is_null() {
                            if (unsafe { (*lst).type_0 }) as u32 == XML_ELEMENT_NODE as i32 as u32
                                && (unsafe { *((*(borrow(&step)).unwrap()).value).offset(0 as i32 as isize) })
                                    as i32
                                    == (unsafe { *((*lst).name).offset(0 as i32 as isize) }) as i32
                                && (unsafe { xmlStrEqual((*(borrow(&step)).unwrap()).value, (*lst).name) }) != 0
                            {
                                break;
                            }
                            lst = unsafe { (*lst).next };
                        }
                        if !lst.is_null() {
                            current_block = 820271813250567934;
                        } else {
                            current_block = 6451473480150109090;
                        }
                    } else {
                        current_block = 6451473480150109090;
                    }
                }
            },
            4 => {
                if (unsafe { (*node).type_0 }) as u32 != XML_ATTRIBUTE_NODE as i32 as u32 {
                    current_block = 6451473480150109090;
                } else {
                    if !((*(borrow(&step)).unwrap()).value).is_null() {
                        if (unsafe { *((*(borrow(&step)).unwrap()).value).offset(0 as i32 as isize) }) as i32
                            != (unsafe { *((*node).name).offset(0 as i32 as isize) }) as i32
                        {
                            current_block = 6451473480150109090;
                        } else if (unsafe { xmlStrEqual((*(borrow(&step)).unwrap()).value, (*node).name) }) == 0
                        {
                            current_block = 6451473480150109090;
                        } else {
                            current_block = 6450597802325118133;
                        }
                    } else {
                        current_block = 6450597802325118133;
                    }
                    match current_block {
                        6451473480150109090 => {},
                        _ => {
                            if (unsafe { (*node).ns }).is_null() {
                                if !((*(borrow(&step)).unwrap()).value2).is_null() {
                                    current_block = 6451473480150109090;
                                } else {
                                    current_block = 820271813250567934;
                                }
                            } else if !((*(borrow(&step)).unwrap()).value2).is_null() {
                                if (unsafe { xmlStrEqual(
                                    (*(borrow(&step)).unwrap()).value2,
                                    (*(*node).ns).href,
                                ) }) == 0
                                {
                                    current_block = 6451473480150109090;
                                } else {
                                    current_block = 820271813250567934;
                                }
                            } else {
                                current_block = 820271813250567934;
                            }
                        },
                    }
                }
            },
            5 => {
                if (unsafe { (*node).type_0 }) as u32 == XML_DOCUMENT_NODE as i32 as u32
                    || (unsafe { (*node).type_0 }) as u32 == XML_HTML_DOCUMENT_NODE as i32 as u32
                    || (unsafe { (*node).type_0 }) as u32 == XML_NAMESPACE_DECL as i32 as u32
                {
                    current_block = 6451473480150109090;
                } else {
                    node = unsafe { (*node).parent };
                    if node.is_null() {
                        current_block = 6451473480150109090;
                    } else if ((*(borrow(&step)).unwrap()).value).is_null() {
                        current_block = 820271813250567934;
                    } else if (unsafe { *((*(borrow(&step)).unwrap()).value).offset(0 as i32 as isize) }) as i32
                        != (unsafe { *((*node).name).offset(0 as i32 as isize) }) as i32
                    {
                        current_block = 6451473480150109090;
                    } else if (unsafe { xmlStrEqual((*(borrow(&step)).unwrap()).value, (*node).name) }) == 0 {
                        current_block = 6451473480150109090;
                    } else if (unsafe { (*node).ns }).is_null() {
                        if !((*(borrow(&step)).unwrap()).value2).is_null() {
                            current_block = 6451473480150109090;
                        } else {
                            current_block = 820271813250567934;
                        }
                    } else if !(unsafe { (*(*node).ns).href }).is_null() {
                        if ((*(borrow(&step)).unwrap()).value2).is_null() {
                            current_block = 6451473480150109090;
                        } else if (unsafe { xmlStrEqual(
                            (*(borrow(&step)).unwrap()).value2,
                            (*(*node).ns).href,
                        ) }) == 0
                        {
                            current_block = 6451473480150109090;
                        } else {
                            current_block = 820271813250567934;
                        }
                    } else {
                        current_block = 820271813250567934;
                    }
                }
            },
            6 => {
                if ((*(borrow(&step)).unwrap()).value).is_null() {
                    i += 1;
                    step = Some(unsafe { &mut *((*comp).steps).offset(i as isize) });
                    if (*(borrow(&step)).unwrap()).op as u32 == XML_OP_ROOT as i32 as u32 {
                        break;
                    }
                    if (*(borrow(&step)).unwrap()).op as u32 != XML_OP_ELEM as i32 as u32 {
                        current_block = 6451473480150109090;
                    } else {
                        if ((*(borrow_mut(&mut step)).unwrap()).value).is_null() {
                            return -(1 as i32);
                        }
                        current_block = 10067844863897285902;
                    }
                } else {
                    current_block = 10067844863897285902;
                }
                match current_block {
                    6451473480150109090 => {},
                    _ => {
                        if node.is_null() {
                            current_block = 6451473480150109090;
                        } else if (unsafe { (*node).type_0 }) as u32 == XML_DOCUMENT_NODE as i32 as u32
                            || (unsafe { (*node).type_0 }) as u32 == XML_HTML_DOCUMENT_NODE as i32 as u32
                            || (unsafe { (*node).type_0 }) as u32 == XML_NAMESPACE_DECL as i32 as u32
                        {
                            current_block = 6451473480150109090;
                        } else {
                            node = unsafe { (*node).parent };
                            while !node.is_null() {
                                if (unsafe { (*node).type_0 }) as u32 == XML_ELEMENT_NODE as i32 as u32
                                    && (unsafe { *((*(borrow(&step)).unwrap()).value)
                                        .offset(0 as i32 as isize) })
                                        as i32
                                        == (unsafe { *((*node).name).offset(0 as i32 as isize) }) as i32
                                    && (unsafe { xmlStrEqual((*(borrow(&step)).unwrap()).value, (*node).name) })
                                        != 0
                                {
                                    if (unsafe { (*node).ns }).is_null() {
                                        if ((*(borrow(&step)).unwrap()).value2).is_null() {
                                            break;
                                        }
                                    } else if !(unsafe { (*(*node).ns).href }).is_null() {
                                        if !((*(borrow(&step)).unwrap()).value2).is_null()
                                            && (unsafe { xmlStrEqual(
                                                (*(borrow(&step)).unwrap()).value2,
                                                (*(*node).ns).href,
                                            ) }) != 0
                                        {
                                            break;
                                        }
                                    }
                                }
                                node = unsafe { (*node).parent };
                            }
                            if node.is_null() {
                                current_block = 6451473480150109090;
                            } else {
                                if (*(borrow(&step)).unwrap()).op as u32
                                    == XML_OP_ANCESTOR as i32 as u32
                                {
                                    xmlPatPushState(Some(&mut states), i, node);
                                } else {
                                    xmlPatPushState(Some(&mut states), i - 1 as i32, node);
                                }
                                current_block = 820271813250567934;
                            }
                        }
                    },
                }
            },
            7 => {
                if (unsafe { (*node).type_0 }) as u32 != XML_ELEMENT_NODE as i32 as u32 {
                    current_block = 6451473480150109090;
                } else if (unsafe { (*node).ns }).is_null() {
                    if !((*(borrow(&step)).unwrap()).value).is_null() {
                        current_block = 6451473480150109090;
                    } else {
                        current_block = 820271813250567934;
                    }
                } else if !(unsafe { (*(*node).ns).href }).is_null() {
                    if ((*(borrow(&step)).unwrap()).value).is_null() {
                        current_block = 6451473480150109090;
                    } else if (unsafe { xmlStrEqual((*(borrow(&step)).unwrap()).value, (*(*node).ns).href) })
                        == 0
                    {
                        current_block = 6451473480150109090;
                    } else {
                        current_block = 820271813250567934;
                    }
                } else {
                    current_block = 820271813250567934;
                }
            },
            8 => {
                if (unsafe { (*node).type_0 }) as u32 != XML_ELEMENT_NODE as i32 as u32 {
                    current_block = 6451473480150109090;
                } else {
                    current_block = 820271813250567934;
                }
            },
            _ => {
                current_block = 820271813250567934;
            },
        }
        match current_block {
            820271813250567934 => {
                i += 1;
            },
            _ => {
                if (states.states).is_null() {
                    return 0 as i32;
                }
                if states.nbstates <= 0 as i32 {
                    (unsafe { xmlFree.expect("non-null function pointer")(states.states as *mut libc::c_void) });
                    return 0 as i32;
                }
                states.nbstates -= 1;
                i = unsafe { (*(states.states).offset(states.nbstates as isize)).step };
                node = unsafe { (*(states.states).offset(states.nbstates as isize)).node };
            },
        }
    }
    if !(states.states).is_null() {
        (unsafe { xmlFree.expect("non-null function pointer")(states.states as *mut libc::c_void) });
    }
    return 1 as i32;
}
extern "C" fn xmlPatScanName<'a1>(
    mut ctxt: *mut crate::src::pattern::_xmlPatParserContext<'a1>,
) -> *mut u8 {
    let mut q: *const u8 = 0 as *const xmlChar;
    let mut cur: *const u8 = 0 as *const xmlChar;
    let mut ret: *mut u8 = 0 as *mut xmlChar;
    let mut val: i32 = 0;
    let mut len: i32 = 0;
    while (unsafe { *(*ctxt).cur }) as i32 == 0x20 as i32
        || 0x9 as i32 <= (unsafe { *(*ctxt).cur }) as i32 && (unsafe { *(*ctxt).cur }) as i32 <= 0xa as i32
        || (unsafe { *(*ctxt).cur }) as i32 == 0xd as i32
    {
        if (unsafe { *(*ctxt).cur }) as i32 != 0 {
            let fresh27 = unsafe { &mut ((*ctxt).cur) };
            *fresh27 = unsafe { (*fresh27).offset(1) };
        } else {
        };
    }
    q = unsafe { (*ctxt).cur };
    cur = q;
    val = xmlStringCurrentChar(0 as xmlParserCtxtPtr, cur, Some(&mut len));
    if !((if val < 0x100 as i32 {
        (0x41 as i32 <= val && val <= 0x5a as i32
            || 0x61 as i32 <= val && val <= 0x7a as i32
            || 0xc0 as i32 <= val && val <= 0xd6 as i32
            || 0xd8 as i32 <= val && val <= 0xf6 as i32
            || 0xf8 as i32 <= val) as i32
    } else {
        xmlCharInRange(val as u32, (Some(unsafe { &xmlIsBaseCharGroup })).clone())
    }) != 0
        || (if val < 0x100 as i32 {
            0 as i32
        } else {
            (0x4e00 as i32 <= val && val <= 0x9fa5 as i32
                || val == 0x3007 as i32
                || 0x3021 as i32 <= val && val <= 0x3029 as i32) as i32
        }) != 0)
        && val != '_' as i32
        && val != ':' as i32
    {
        return 0 as *mut xmlChar;
    }
    while (if val < 0x100 as i32 {
        (0x41 as i32 <= val && val <= 0x5a as i32
            || 0x61 as i32 <= val && val <= 0x7a as i32
            || 0xc0 as i32 <= val && val <= 0xd6 as i32
            || 0xd8 as i32 <= val && val <= 0xf6 as i32
            || 0xf8 as i32 <= val) as i32
    } else {
        xmlCharInRange(val as u32, (Some(unsafe { &xmlIsBaseCharGroup })).clone())
    }) != 0
        || (if val < 0x100 as i32 {
            0 as i32
        } else {
            (0x4e00 as i32 <= val && val <= 0x9fa5 as i32
                || val == 0x3007 as i32
                || 0x3021 as i32 <= val && val <= 0x3029 as i32) as i32
        }) != 0
        || (if val < 0x100 as i32 {
            (0x30 as i32 <= val && val <= 0x39 as i32) as i32
        } else {
            xmlCharInRange(val as u32, (Some(unsafe { &xmlIsDigitGroup })).clone())
        }) != 0
        || val == '.' as i32
        || val == '-' as i32
        || val == '_' as i32
        || (if val < 0x100 as i32 {
            0 as i32
        } else {
            xmlCharInRange(val as u32, (Some(unsafe { &xmlIsCombiningGroup })).clone())
        }) != 0
        || (if val < 0x100 as i32 {
            (val == 0xb7 as i32) as i32
        } else {
            xmlCharInRange(val as u32, (Some(unsafe { &xmlIsExtenderGroup })).clone())
        }) != 0
    {
        cur = unsafe { cur.offset(len as isize) };
        val = xmlStringCurrentChar(0 as xmlParserCtxtPtr, cur, Some(&mut len));
    }
    if !(unsafe { (*ctxt).dict }).is_null() {
        ret = xmlDictLookup(unsafe { (*ctxt).dict }, q, (unsafe { cur.offset_from(q) }) as i64 as i32) as *mut xmlChar;
    } else {
        ret = unsafe { xmlStrndup(q, cur.offset_from(q) as i64 as i32) };
    }
    let fresh28 = unsafe { &mut ((*ctxt).cur) };
    *fresh28 = cur;
    return ret;
}
extern "C" fn xmlPatScanNCName<'a1>(
    mut ctxt: *mut crate::src::pattern::_xmlPatParserContext<'a1>,
) -> *mut u8 {
    let mut q: *const u8 = 0 as *const xmlChar;
    let mut cur: *const u8 = 0 as *const xmlChar;
    let mut ret: *mut u8 = 0 as *mut xmlChar;
    let mut val: i32 = 0;
    let mut len: i32 = 0;
    while (unsafe { *(*ctxt).cur }) as i32 == 0x20 as i32
        || 0x9 as i32 <= (unsafe { *(*ctxt).cur }) as i32 && (unsafe { *(*ctxt).cur }) as i32 <= 0xa as i32
        || (unsafe { *(*ctxt).cur }) as i32 == 0xd as i32
    {
        if (unsafe { *(*ctxt).cur }) as i32 != 0 {
            let fresh29 = unsafe { &mut ((*ctxt).cur) };
            *fresh29 = unsafe { (*fresh29).offset(1) };
        } else {
        };
    }
    q = unsafe { (*ctxt).cur };
    cur = q;
    val = xmlStringCurrentChar(0 as xmlParserCtxtPtr, cur, Some(&mut len));
    if !((if val < 0x100 as i32 {
        (0x41 as i32 <= val && val <= 0x5a as i32
            || 0x61 as i32 <= val && val <= 0x7a as i32
            || 0xc0 as i32 <= val && val <= 0xd6 as i32
            || 0xd8 as i32 <= val && val <= 0xf6 as i32
            || 0xf8 as i32 <= val) as i32
    } else {
        xmlCharInRange(val as u32, (Some(unsafe { &xmlIsBaseCharGroup })).clone())
    }) != 0
        || (if val < 0x100 as i32 {
            0 as i32
        } else {
            (0x4e00 as i32 <= val && val <= 0x9fa5 as i32
                || val == 0x3007 as i32
                || 0x3021 as i32 <= val && val <= 0x3029 as i32) as i32
        }) != 0)
        && val != '_' as i32
    {
        return 0 as *mut xmlChar;
    }
    while (if val < 0x100 as i32 {
        (0x41 as i32 <= val && val <= 0x5a as i32
            || 0x61 as i32 <= val && val <= 0x7a as i32
            || 0xc0 as i32 <= val && val <= 0xd6 as i32
            || 0xd8 as i32 <= val && val <= 0xf6 as i32
            || 0xf8 as i32 <= val) as i32
    } else {
        xmlCharInRange(val as u32, (Some(unsafe { &xmlIsBaseCharGroup })).clone())
    }) != 0
        || (if val < 0x100 as i32 {
            0 as i32
        } else {
            (0x4e00 as i32 <= val && val <= 0x9fa5 as i32
                || val == 0x3007 as i32
                || 0x3021 as i32 <= val && val <= 0x3029 as i32) as i32
        }) != 0
        || (if val < 0x100 as i32 {
            (0x30 as i32 <= val && val <= 0x39 as i32) as i32
        } else {
            xmlCharInRange(val as u32, (Some(unsafe { &xmlIsDigitGroup })).clone())
        }) != 0
        || val == '.' as i32
        || val == '-' as i32
        || val == '_' as i32
        || (if val < 0x100 as i32 {
            0 as i32
        } else {
            xmlCharInRange(val as u32, (Some(unsafe { &xmlIsCombiningGroup })).clone())
        }) != 0
        || (if val < 0x100 as i32 {
            (val == 0xb7 as i32) as i32
        } else {
            xmlCharInRange(val as u32, (Some(unsafe { &xmlIsExtenderGroup })).clone())
        }) != 0
    {
        cur = unsafe { cur.offset(len as isize) };
        val = xmlStringCurrentChar(0 as xmlParserCtxtPtr, cur, Some(&mut len));
    }
    if !(unsafe { (*ctxt).dict }).is_null() {
        ret = xmlDictLookup(unsafe { (*ctxt).dict }, q, (unsafe { cur.offset_from(q) }) as i64 as i32) as *mut xmlChar;
    } else {
        ret = unsafe { xmlStrndup(q, cur.offset_from(q) as i64 as i32) };
    }
    let fresh30 = unsafe { &mut ((*ctxt).cur) };
    *fresh30 = cur;
    return ret;
}
extern "C" fn xmlCompileAttributeTest<'a1>(
    mut ctxt: *mut crate::src::pattern::_xmlPatParserContext<'a1>,
) {
    let mut current_block: u64;
    let mut token: *mut u8 = 0 as *mut xmlChar;
    let mut name: *mut u8 = 0 as *mut xmlChar;
    let mut URL: *mut u8 = 0 as *mut xmlChar;
    while (unsafe { *(*ctxt).cur }) as i32 == 0x20 as i32
        || 0x9 as i32 <= (unsafe { *(*ctxt).cur }) as i32 && (unsafe { *(*ctxt).cur }) as i32 <= 0xa as i32
        || (unsafe { *(*ctxt).cur }) as i32 == 0xd as i32
    {
        if (unsafe { *(*ctxt).cur }) as i32 != 0 {
            let fresh31 = unsafe { &mut ((*ctxt).cur) };
            *fresh31 = unsafe { (*fresh31).offset(1) };
        } else {
        };
    }
    name = xmlPatScanNCName(ctxt);
    if name.is_null() {
        if (unsafe { *(*ctxt).cur }) as i32 == '*' as i32 {
            if xmlPatternAdd(
                ctxt,
                unsafe { (*ctxt).comp },
                XML_OP_ATTR,
                0 as *mut xmlChar,
                0 as *mut xmlChar,
            ) != 0
            {
                current_block = 11331548824878167032;
            } else {
                if (unsafe { *(*ctxt).cur }) as i32 != 0 {
                    let fresh32 = unsafe { &mut ((*ctxt).cur) };
                    *fresh32 = unsafe { (*fresh32).offset(1) };
                } else {
                };
                current_block = 5399440093318478209;
            }
        } else {
            (unsafe { (*ctxt).error = 1 as i32 });
            current_block = 5399440093318478209;
        }
        match current_block {
            11331548824878167032 => {},
            _ => return,
        }
    } else {
        if (unsafe { *(*ctxt).cur }) as i32 == ':' as i32 {
            let mut i: i32 = 0;
            let mut prefix: *mut u8 = name;
            if (unsafe { *(*ctxt).cur }) as i32 != 0 {
                let fresh33 = unsafe { &mut ((*ctxt).cur) };
                *fresh33 = unsafe { (*fresh33).offset(1) };
            } else {
            };
            if (unsafe { *(*ctxt).cur }) as i32 == 0x20 as i32
                || 0x9 as i32 <= (unsafe { *(*ctxt).cur }) as i32 && (unsafe { *(*ctxt).cur }) as i32 <= 0xa as i32
                || (unsafe { *(*ctxt).cur }) as i32 == 0xd as i32
            {
                if (unsafe { (*(*ctxt).comp).dict }).is_null() {
                    (unsafe { xmlFree.expect("non-null function pointer")(prefix as *mut libc::c_void) });
                }
                (unsafe { (*ctxt).error = 1 as i32 });
                current_block = 11331548824878167032;
            } else {
                token = xmlPatScanName(ctxt);
                if (unsafe { *prefix.offset(0 as i32 as isize) }) as i32 == 'x' as i32
                    && (unsafe { *prefix.offset(1 as i32 as isize) }) as i32 == 'm' as i32
                    && (unsafe { *prefix.offset(2 as i32 as isize) }) as i32 == 'l' as i32
                    && (unsafe { *prefix.offset(3 as i32 as isize) }) as i32 == 0 as i32
                {
                    if !(unsafe { (*(*ctxt).comp).dict }).is_null() {
                        URL = xmlDictLookup(
                            unsafe { (*(*ctxt).comp).dict },
                            b"http://www.w3.org/XML/1998/namespace\0" as *const u8 as *const i8
                                as *const xmlChar as *mut xmlChar,
                            -(1 as i32),
                        ) as *mut xmlChar;
                    } else {
                        URL = unsafe { xmlStrdup(
                            b"http://www.w3.org/XML/1998/namespace\0" as *const u8 as *const i8
                                as *const xmlChar as *mut xmlChar,
                        ) };
                    }
                    current_block = 15512526488502093901;
                } else {
                    i = 0 as i32;
                    while i < (unsafe { (*ctxt).nb_namespaces }) {
                        if (unsafe { xmlStrEqual(
                            *((*ctxt).namespaces).offset((2 as i32 * i + 1 as i32) as isize),
                            prefix,
                        ) }) != 0
                        {
                            if !(unsafe { (*(*ctxt).comp).dict }).is_null() {
                                URL = xmlDictLookup(
                                    unsafe { (*(*ctxt).comp).dict },
                                    (unsafe { *((*ctxt).namespaces).offset((2 as i32 * i) as isize) })
                                        as *mut xmlChar,
                                    -(1 as i32),
                                ) as *mut xmlChar;
                            } else {
                                URL = unsafe { xmlStrdup(
                                    *((*ctxt).namespaces).offset((2 as i32 * i) as isize)
                                        as *mut xmlChar,
                                ) };
                            }
                            break;
                        } else {
                            i += 1;
                        }
                    }
                    if i >= (unsafe { (*ctxt).nb_namespaces }) {
                        if (unsafe { (*(*ctxt).comp).dict }).is_null() {
                            (unsafe { xmlFree.expect("non-null function pointer")(
                                prefix as *mut libc::c_void,
                            ) });
                        }
                        (unsafe { (*ctxt).error = 1 as i32 });
                        current_block = 11331548824878167032;
                    } else {
                        current_block = 15512526488502093901;
                    }
                }
                match current_block {
                    11331548824878167032 => {},
                    _ => {
                        if (unsafe { (*(*ctxt).comp).dict }).is_null() {
                            (unsafe { xmlFree.expect("non-null function pointer")(
                                prefix as *mut libc::c_void,
                            ) });
                        }
                        if token.is_null() {
                            if (unsafe { *(*ctxt).cur }) as i32 == '*' as i32 {
                                if (unsafe { *(*ctxt).cur }) as i32 != 0 {
                                    let fresh34 = unsafe { &mut ((*ctxt).cur) };
                                    *fresh34 = unsafe { (*fresh34).offset(1) };
                                } else {
                                };
                                if xmlPatternAdd(
                                    ctxt,
                                    unsafe { (*ctxt).comp },
                                    XML_OP_ATTR,
                                    0 as *mut xmlChar,
                                    URL,
                                ) != 0
                                {
                                    current_block = 11331548824878167032;
                                } else {
                                    current_block = 9512719473022792396;
                                }
                            } else {
                                (unsafe { (*ctxt).error = 1 as i32 });
                                current_block = 11331548824878167032;
                            }
                        } else if xmlPatternAdd(ctxt, unsafe { (*ctxt).comp }, XML_OP_ATTR, token, URL) != 0 {
                            current_block = 11331548824878167032;
                        } else {
                            current_block = 9512719473022792396;
                        }
                    },
                }
            }
        } else if xmlPatternAdd(ctxt, unsafe { (*ctxt).comp }, XML_OP_ATTR, name, 0 as *mut xmlChar) != 0 {
            current_block = 11331548824878167032;
        } else {
            current_block = 9512719473022792396;
        }
        match current_block {
            11331548824878167032 => {},
            _ => return,
        }
    }
    if !URL.is_null() {
        if (unsafe { (*(*ctxt).comp).dict }).is_null() {
            (unsafe { xmlFree.expect("non-null function pointer")(URL as *mut libc::c_void) });
        }
    }
    if !token.is_null() {
        if (unsafe { (*(*ctxt).comp).dict }).is_null() {
            (unsafe { xmlFree.expect("non-null function pointer")(token as *mut libc::c_void) });
        }
    }
}
extern "C" fn xmlCompileStepPattern<'a1>(
    mut ctxt: *mut crate::src::pattern::_xmlPatParserContext<'a1>,
) {
    let mut current_block: u64;
    let mut token: *mut u8 = 0 as *mut xmlChar;
    let mut name: *mut u8 = 0 as *mut xmlChar;
    let mut URL: *mut u8 = 0 as *mut xmlChar;
    let mut hasBlanks: i32 = 0 as i32;
    while (unsafe { *(*ctxt).cur }) as i32 == 0x20 as i32
        || 0x9 as i32 <= (unsafe { *(*ctxt).cur }) as i32 && (unsafe { *(*ctxt).cur }) as i32 <= 0xa as i32
        || (unsafe { *(*ctxt).cur }) as i32 == 0xd as i32
    {
        if (unsafe { *(*ctxt).cur }) as i32 != 0 {
            let fresh35 = unsafe { &mut ((*ctxt).cur) };
            *fresh35 = unsafe { (*fresh35).offset(1) };
        } else {
        };
    }
    if (unsafe { *(*ctxt).cur }) as i32 == '.' as i32 {
        if (unsafe { *(*ctxt).cur }) as i32 != 0 {
            let fresh36 = unsafe { &mut ((*ctxt).cur) };
            *fresh36 = unsafe { (*fresh36).offset(1) };
        } else {
        };
        if !(xmlPatternAdd(
            ctxt,
            unsafe { (*ctxt).comp },
            XML_OP_ELEM,
            0 as *mut xmlChar,
            0 as *mut xmlChar,
        ) != 0)
        {
            return;
        }
    } else if (unsafe { *(*ctxt).cur }) as i32 == '@' as i32 {
        if (unsafe { (*(*ctxt).comp).flags }) & XML_PATTERN_XSSEL as i32 != 0 {
            (unsafe { (*ctxt).error = 1 as i32 });
            return;
        }
        if (unsafe { *(*ctxt).cur }) as i32 != 0 {
            let fresh37 = unsafe { &mut ((*ctxt).cur) };
            *fresh37 = unsafe { (*fresh37).offset(1) };
        } else {
        };
        xmlCompileAttributeTest(ctxt);
        if !((unsafe { (*ctxt).error }) != 0 as i32) {
            return;
        }
    } else {
        name = xmlPatScanNCName(ctxt);
        if name.is_null() {
            if (unsafe { *(*ctxt).cur }) as i32 == '*' as i32 {
                if (unsafe { *(*ctxt).cur }) as i32 != 0 {
                    let fresh38 = unsafe { &mut ((*ctxt).cur) };
                    *fresh38 = unsafe { (*fresh38).offset(1) };
                } else {
                };
                if !(xmlPatternAdd(
                    ctxt,
                    unsafe { (*ctxt).comp },
                    XML_OP_ALL,
                    0 as *mut xmlChar,
                    0 as *mut xmlChar,
                ) != 0)
                {
                    return;
                }
            } else {
                (unsafe { (*ctxt).error = 1 as i32 });
                return;
            }
        } else {
            if (unsafe { *(*ctxt).cur }) as i32 == 0x20 as i32
                || 0x9 as i32 <= (unsafe { *(*ctxt).cur }) as i32 && (unsafe { *(*ctxt).cur }) as i32 <= 0xa as i32
                || (unsafe { *(*ctxt).cur }) as i32 == 0xd as i32
            {
                hasBlanks = 1 as i32;
                while (unsafe { *(*ctxt).cur }) as i32 == 0x20 as i32
                    || 0x9 as i32 <= (unsafe { *(*ctxt).cur }) as i32 && (unsafe { *(*ctxt).cur }) as i32 <= 0xa as i32
                    || (unsafe { *(*ctxt).cur }) as i32 == 0xd as i32
                {
                    if (unsafe { *(*ctxt).cur }) as i32 != 0 {
                        let fresh39 = unsafe { &mut ((*ctxt).cur) };
                        *fresh39 = unsafe { (*fresh39).offset(1) };
                    } else {
                    };
                }
            }
            if (unsafe { *(*ctxt).cur }) as i32 == ':' as i32 {
                if (unsafe { *(*ctxt).cur }) as i32 != 0 {
                    let fresh40 = unsafe { &mut ((*ctxt).cur) };
                    *fresh40 = unsafe { (*fresh40).offset(1) };
                } else {
                };
                if (unsafe { *(*ctxt).cur }) as i32 != ':' as i32 {
                    let mut prefix: *mut u8 = name;
                    let mut i: i32 = 0;
                    if hasBlanks != 0
                        || ((unsafe { *(*ctxt).cur }) as i32 == 0x20 as i32
                            || 0x9 as i32 <= (unsafe { *(*ctxt).cur }) as i32
                                && (unsafe { *(*ctxt).cur }) as i32 <= 0xa as i32
                            || (unsafe { *(*ctxt).cur }) as i32 == 0xd as i32)
                    {
                        (unsafe { (*ctxt).error = 1 as i32 });
                        current_block = 15904406811757377787;
                    } else {
                        token = xmlPatScanName(ctxt);
                        if (unsafe { *prefix.offset(0 as i32 as isize) }) as i32 == 'x' as i32
                            && (unsafe { *prefix.offset(1 as i32 as isize) }) as i32 == 'm' as i32
                            && (unsafe { *prefix.offset(2 as i32 as isize) }) as i32 == 'l' as i32
                            && (unsafe { *prefix.offset(3 as i32 as isize) }) as i32 == 0 as i32
                        {
                            if !(unsafe { (*(*ctxt).comp).dict }).is_null() {
                                URL = xmlDictLookup(
                                    unsafe { (*(*ctxt).comp).dict },
                                    b"http://www.w3.org/XML/1998/namespace\0" as *const u8
                                        as *const i8
                                        as *const xmlChar
                                        as *mut xmlChar,
                                    -(1 as i32),
                                ) as *mut xmlChar;
                            } else {
                                URL = unsafe { xmlStrdup(
                                    b"http://www.w3.org/XML/1998/namespace\0" as *const u8
                                        as *const i8
                                        as *const xmlChar
                                        as *mut xmlChar,
                                ) };
                            }
                            current_block = 13325891313334703151;
                        } else {
                            i = 0 as i32;
                            while i < (unsafe { (*ctxt).nb_namespaces }) {
                                if (unsafe { xmlStrEqual(
                                    *((*ctxt).namespaces)
                                        .offset((2 as i32 * i + 1 as i32) as isize),
                                    prefix,
                                ) }) != 0
                                {
                                    if !(unsafe { (*(*ctxt).comp).dict }).is_null() {
                                        URL = xmlDictLookup(
                                            unsafe { (*(*ctxt).comp).dict },
                                            (unsafe { *((*ctxt).namespaces).offset((2 as i32 * i) as isize) })
                                                as *mut xmlChar,
                                            -(1 as i32),
                                        )
                                            as *mut xmlChar;
                                    } else {
                                        URL = unsafe { xmlStrdup(
                                            *((*ctxt).namespaces).offset((2 as i32 * i) as isize)
                                                as *mut xmlChar,
                                        ) };
                                    }
                                    break;
                                } else {
                                    i += 1;
                                }
                            }
                            if i >= (unsafe { (*ctxt).nb_namespaces }) {
                                (unsafe { (*ctxt).error = 1 as i32 });
                                current_block = 15904406811757377787;
                            } else {
                                current_block = 13325891313334703151;
                            }
                        }
                        match current_block {
                            15904406811757377787 => {},
                            _ => {
                                if (unsafe { (*(*ctxt).comp).dict }).is_null() {
                                    (unsafe { xmlFree.expect("non-null function pointer")(
                                        prefix as *mut libc::c_void,
                                    ) });
                                }
                                name = 0 as *mut xmlChar;
                                if token.is_null() {
                                    if (unsafe { *(*ctxt).cur }) as i32 == '*' as i32 {
                                        if (unsafe { *(*ctxt).cur }) as i32 != 0 {
                                            let fresh41 = unsafe { &mut ((*ctxt).cur) };
                                            *fresh41 = unsafe { (*fresh41).offset(1) };
                                        } else {
                                        };
                                        if xmlPatternAdd(
                                            ctxt,
                                            unsafe { (*ctxt).comp },
                                            XML_OP_NS,
                                            URL,
                                            0 as *mut xmlChar,
                                        ) != 0
                                        {
                                            current_block = 15904406811757377787;
                                        } else {
                                            current_block = 8880031775101799352;
                                        }
                                    } else {
                                        (unsafe { (*ctxt).error = 1 as i32 });
                                        current_block = 15904406811757377787;
                                    }
                                } else if xmlPatternAdd(ctxt, unsafe { (*ctxt).comp }, XML_OP_ELEM, token, URL)
                                    != 0
                                {
                                    current_block = 15904406811757377787;
                                } else {
                                    current_block = 8880031775101799352;
                                }
                            },
                        }
                    }
                } else {
                    if (unsafe { *(*ctxt).cur }) as i32 != 0 {
                        let fresh42 = unsafe { &mut ((*ctxt).cur) };
                        *fresh42 = unsafe { (*fresh42).offset(1) };
                    } else {
                    };
                    if (unsafe { xmlStrEqual(name, b"child\0" as *const u8 as *const i8 as *const xmlChar) })
                        != 0
                    {
                        if (unsafe { (*(*ctxt).comp).dict }).is_null() {
                            (unsafe { xmlFree.expect("non-null function pointer")(name as *mut libc::c_void) });
                        }
                        name = xmlPatScanName(ctxt);
                        if name.is_null() {
                            if (unsafe { *(*ctxt).cur }) as i32 == '*' as i32 {
                                if (unsafe { *(*ctxt).cur }) as i32 != 0 {
                                    let fresh43 = unsafe { &mut ((*ctxt).cur) };
                                    *fresh43 = unsafe { (*fresh43).offset(1) };
                                } else {
                                };
                                if !(xmlPatternAdd(
                                    ctxt,
                                    unsafe { (*ctxt).comp },
                                    XML_OP_ALL,
                                    0 as *mut xmlChar,
                                    0 as *mut xmlChar,
                                ) != 0)
                                {
                                    return;
                                }
                            } else {
                                (unsafe { (*ctxt).error = 1 as i32 });
                            }
                        } else {
                            if (unsafe { *(*ctxt).cur }) as i32 == ':' as i32 {
                                let mut prefix_0: *mut u8 = name;
                                let mut i_0: i32 = 0;
                                if (unsafe { *(*ctxt).cur }) as i32 != 0 {
                                    let fresh44 = unsafe { &mut ((*ctxt).cur) };
                                    *fresh44 = unsafe { (*fresh44).offset(1) };
                                } else {
                                };
                                if (unsafe { *(*ctxt).cur }) as i32 == 0x20 as i32
                                    || 0x9 as i32 <= (unsafe { *(*ctxt).cur }) as i32
                                        && (unsafe { *(*ctxt).cur }) as i32 <= 0xa as i32
                                    || (unsafe { *(*ctxt).cur }) as i32 == 0xd as i32
                                {
                                    (unsafe { (*ctxt).error = 1 as i32 });
                                    current_block = 15904406811757377787;
                                } else {
                                    token = xmlPatScanName(ctxt);
                                    if (unsafe { *prefix_0.offset(0 as i32 as isize) }) as i32 == 'x' as i32
                                        && (unsafe { *prefix_0.offset(1 as i32 as isize) }) as i32 == 'm' as i32
                                        && (unsafe { *prefix_0.offset(2 as i32 as isize) }) as i32 == 'l' as i32
                                        && (unsafe { *prefix_0.offset(3 as i32 as isize) }) as i32 == 0 as i32
                                    {
                                        if !(unsafe { (*(*ctxt).comp).dict }).is_null() {
                                            URL = xmlDictLookup(
                                                unsafe { (*(*ctxt).comp).dict },
                                                b"http://www.w3.org/XML/1998/namespace\0"
                                                    as *const u8
                                                    as *const i8
                                                    as *const xmlChar
                                                    as *mut xmlChar,
                                                -(1 as i32),
                                            )
                                                as *mut xmlChar;
                                        } else {
                                            URL = unsafe { xmlStrdup(
                                                b"http://www.w3.org/XML/1998/namespace\0"
                                                    as *const u8
                                                    as *const i8
                                                    as *const xmlChar
                                                    as *mut xmlChar,
                                            ) };
                                        }
                                        current_block = 5706227035632243100;
                                    } else {
                                        i_0 = 0 as i32;
                                        while i_0 < (unsafe { (*ctxt).nb_namespaces }) {
                                            if (unsafe { xmlStrEqual(
                                                *((*ctxt).namespaces)
                                                    .offset((2 as i32 * i_0 + 1 as i32) as isize),
                                                prefix_0,
                                            ) }) != 0
                                            {
                                                if !(unsafe { (*(*ctxt).comp).dict }).is_null() {
                                                    URL = xmlDictLookup(
                                                        unsafe { (*(*ctxt).comp).dict },
                                                        (unsafe { *((*ctxt).namespaces)
                                                            .offset((2 as i32 * i_0) as isize) })
                                                            as *mut xmlChar,
                                                        -(1 as i32),
                                                    )
                                                        as *mut xmlChar;
                                                } else {
                                                    URL = unsafe { xmlStrdup(
                                                        *((*ctxt).namespaces)
                                                            .offset((2 as i32 * i_0) as isize)
                                                            as *mut xmlChar,
                                                    ) };
                                                }
                                                break;
                                            } else {
                                                i_0 += 1;
                                            }
                                        }
                                        if i_0 >= (unsafe { (*ctxt).nb_namespaces }) {
                                            (unsafe { (*ctxt).error = 1 as i32 });
                                            current_block = 15904406811757377787;
                                        } else {
                                            current_block = 5706227035632243100;
                                        }
                                    }
                                    match current_block {
                                        15904406811757377787 => {},
                                        _ => {
                                            if (unsafe { (*(*ctxt).comp).dict }).is_null() {
                                                (unsafe { xmlFree.expect("non-null function pointer")(
                                                    prefix_0 as *mut libc::c_void,
                                                ) });
                                            }
                                            name = 0 as *mut xmlChar;
                                            if token.is_null() {
                                                if (unsafe { *(*ctxt).cur }) as i32 == '*' as i32 {
                                                    if (unsafe { *(*ctxt).cur }) as i32 != 0 {
                                                        let fresh45 = unsafe { &mut ((*ctxt).cur) };
                                                        *fresh45 = unsafe { (*fresh45).offset(1) };
                                                    } else {
                                                    };
                                                    if xmlPatternAdd(
                                                        ctxt,
                                                        unsafe { (*ctxt).comp },
                                                        XML_OP_NS,
                                                        URL,
                                                        0 as *mut xmlChar,
                                                    ) != 0
                                                    {
                                                        current_block = 15904406811757377787;
                                                    } else {
                                                        current_block = 7337917895049117968;
                                                    }
                                                } else {
                                                    (unsafe { (*ctxt).error = 1 as i32 });
                                                    current_block = 15904406811757377787;
                                                }
                                            } else if xmlPatternAdd(
                                                ctxt,
                                                unsafe { (*ctxt).comp },
                                                XML_OP_CHILD,
                                                token,
                                                URL,
                                            ) != 0
                                            {
                                                current_block = 15904406811757377787;
                                            } else {
                                                current_block = 7337917895049117968;
                                            }
                                        },
                                    }
                                }
                            } else if xmlPatternAdd(
                                ctxt,
                                unsafe { (*ctxt).comp },
                                XML_OP_CHILD,
                                name,
                                0 as *mut xmlChar,
                            ) != 0
                            {
                                current_block = 15904406811757377787;
                            } else {
                                current_block = 7337917895049117968;
                            }
                            match current_block {
                                15904406811757377787 => {},
                                _ => return,
                            }
                        }
                    } else if (unsafe { xmlStrEqual(
                        name,
                        b"attribute\0" as *const u8 as *const i8 as *const xmlChar,
                    ) }) != 0
                    {
                        if (unsafe { (*(*ctxt).comp).dict }).is_null() {
                            (unsafe { xmlFree.expect("non-null function pointer")(name as *mut libc::c_void) });
                        }
                        name = 0 as *mut xmlChar;
                        if (unsafe { (*(*ctxt).comp).flags }) & XML_PATTERN_XSSEL as i32 != 0 {
                            (unsafe { (*ctxt).error = 1 as i32 });
                        } else {
                            xmlCompileAttributeTest(ctxt);
                            if !((unsafe { (*ctxt).error }) != 0 as i32) {
                                return;
                            }
                        }
                    } else {
                        (unsafe { (*ctxt).error = 1 as i32 });
                    }
                    current_block = 15904406811757377787;
                }
            } else if (unsafe { *(*ctxt).cur }) as i32 == '*' as i32 {
                if !name.is_null() {
                    (unsafe { (*ctxt).error = 1 as i32 });
                    current_block = 15904406811757377787;
                } else {
                    if (unsafe { *(*ctxt).cur }) as i32 != 0 {
                        let fresh46 = unsafe { &mut ((*ctxt).cur) };
                        *fresh46 = unsafe { (*fresh46).offset(1) };
                    } else {
                    };
                    if xmlPatternAdd(ctxt, unsafe { (*ctxt).comp }, XML_OP_ALL, token, 0 as *mut xmlChar) != 0
                    {
                        current_block = 15904406811757377787;
                    } else {
                        current_block = 8880031775101799352;
                    }
                }
            } else if xmlPatternAdd(ctxt, unsafe { (*ctxt).comp }, XML_OP_ELEM, name, 0 as *mut xmlChar) != 0 {
                current_block = 15904406811757377787;
            } else {
                current_block = 8880031775101799352;
            }
            match current_block {
                15904406811757377787 => {},
                _ => return,
            }
        }
    }
    if !URL.is_null() {
        if (unsafe { (*(*ctxt).comp).dict }).is_null() {
            (unsafe { xmlFree.expect("non-null function pointer")(URL as *mut libc::c_void) });
        }
    }
    if !token.is_null() {
        if (unsafe { (*(*ctxt).comp).dict }).is_null() {
            (unsafe { xmlFree.expect("non-null function pointer")(token as *mut libc::c_void) });
        }
    }
    if !name.is_null() {
        if (unsafe { (*(*ctxt).comp).dict }).is_null() {
            (unsafe { xmlFree.expect("non-null function pointer")(name as *mut libc::c_void) });
        }
    }
}
extern "C" fn xmlCompilePathPattern<'a1>(
    mut ctxt: *mut crate::src::pattern::_xmlPatParserContext<'a1>,
) {
    let mut current_block: u64;
    while (unsafe { *(*ctxt).cur }) as i32 == 0x20 as i32
        || 0x9 as i32 <= (unsafe { *(*ctxt).cur }) as i32 && (unsafe { *(*ctxt).cur }) as i32 <= 0xa as i32
        || (unsafe { *(*ctxt).cur }) as i32 == 0xd as i32
    {
        if (unsafe { *(*ctxt).cur }) as i32 != 0 {
            let fresh47 = unsafe { &mut ((*ctxt).cur) };
            *fresh47 = unsafe { (*fresh47).offset(1) };
        } else {
        };
    }
    if (unsafe { *(*ctxt).cur }) as i32 == '/' as i32 {
        (unsafe { (*(*ctxt).comp).flags |= (1 as i32) << 8 as i32 });
    } else if (unsafe { *(*ctxt).cur }) as i32 == '.' as i32
        || (unsafe { (*(*ctxt).comp).flags })
            & (XML_PATTERN_XPATH as i32 | XML_PATTERN_XSSEL as i32 | XML_PATTERN_XSFIELD as i32)
            != 0
    {
        (unsafe { (*(*ctxt).comp).flags |= (1 as i32) << 9 as i32 });
    }
    if (unsafe { *(*ctxt).cur }) as i32 == '/' as i32
        && (unsafe { *((*ctxt).cur).offset(1 as i32 as isize) }) as i32 == '/' as i32
    {
        if xmlPatternAdd(
            ctxt,
            unsafe { (*ctxt).comp },
            XML_OP_ANCESTOR,
            0 as *mut xmlChar,
            0 as *mut xmlChar,
        ) != 0
        {
            current_block = 17489734837053406682;
        } else {
            if (unsafe { *(*ctxt).cur }) as i32 != 0 {
                let fresh48 = unsafe { &mut ((*ctxt).cur) };
                *fresh48 = unsafe { (*fresh48).offset(1) };
            } else {
            };
            if (unsafe { *(*ctxt).cur }) as i32 != 0 {
                let fresh49 = unsafe { &mut ((*ctxt).cur) };
                *fresh49 = unsafe { (*fresh49).offset(1) };
            } else {
            };
            current_block = 11194104282611034094;
        }
    } else if (unsafe { *(*ctxt).cur }) as i32 == '.' as i32
        && (unsafe { *((*ctxt).cur).offset(1 as i32 as isize) }) as i32 == '/' as i32
        && (unsafe { *((*ctxt).cur).offset(2 as i32 as isize) }) as i32 == '/' as i32
    {
        if xmlPatternAdd(
            ctxt,
            unsafe { (*ctxt).comp },
            XML_OP_ANCESTOR,
            0 as *mut xmlChar,
            0 as *mut xmlChar,
        ) != 0
        {
            current_block = 17489734837053406682;
        } else {
            if (unsafe { *(*ctxt).cur }) as i32 != 0 {
                let fresh50 = unsafe { &mut ((*ctxt).cur) };
                *fresh50 = unsafe { (*fresh50).offset(1) };
            } else {
            };
            if (unsafe { *(*ctxt).cur }) as i32 != 0 {
                let fresh51 = unsafe { &mut ((*ctxt).cur) };
                *fresh51 = unsafe { (*fresh51).offset(1) };
            } else {
            };
            if (unsafe { *(*ctxt).cur }) as i32 != 0 {
                let fresh52 = unsafe { &mut ((*ctxt).cur) };
                *fresh52 = unsafe { (*fresh52).offset(1) };
            } else {
            };
            while (unsafe { *(*ctxt).cur }) as i32 == 0x20 as i32
                || 0x9 as i32 <= (unsafe { *(*ctxt).cur }) as i32 && (unsafe { *(*ctxt).cur }) as i32 <= 0xa as i32
                || (unsafe { *(*ctxt).cur }) as i32 == 0xd as i32
            {
                if (unsafe { *(*ctxt).cur }) as i32 != 0 {
                    let fresh53 = unsafe { &mut ((*ctxt).cur) };
                    *fresh53 = unsafe { (*fresh53).offset(1) };
                } else {
                };
            }
            if (unsafe { *(*ctxt).cur }) as i32 == 0 as i32 {
                (unsafe { (*ctxt).error = 1 as i32 });
                current_block = 17489734837053406682;
            } else {
                current_block = 11194104282611034094;
            }
        }
    } else {
        current_block = 11194104282611034094;
    }
    match current_block {
        11194104282611034094 => {
            if (unsafe { *(*ctxt).cur }) as i32 == '@' as i32 {
                if (unsafe { *(*ctxt).cur }) as i32 != 0 {
                    let fresh54 = unsafe { &mut ((*ctxt).cur) };
                    *fresh54 = unsafe { (*fresh54).offset(1) };
                } else {
                };
                xmlCompileAttributeTest(ctxt);
                while (unsafe { *(*ctxt).cur }) as i32 == 0x20 as i32
                    || 0x9 as i32 <= (unsafe { *(*ctxt).cur }) as i32 && (unsafe { *(*ctxt).cur }) as i32 <= 0xa as i32
                    || (unsafe { *(*ctxt).cur }) as i32 == 0xd as i32
                {
                    if (unsafe { *(*ctxt).cur }) as i32 != 0 {
                        let fresh55 = unsafe { &mut ((*ctxt).cur) };
                        *fresh55 = unsafe { (*fresh55).offset(1) };
                    } else {
                    };
                }
                if (unsafe { *(*ctxt).cur }) as i32 != 0 as i32 {
                    xmlCompileStepPattern(ctxt);
                    if (unsafe { (*ctxt).error }) != 0 as i32 {
                        current_block = 17489734837053406682;
                    } else {
                        current_block = 7189308829251266000;
                    }
                } else {
                    current_block = 7189308829251266000;
                }
            } else {
                if (unsafe { *(*ctxt).cur }) as i32 == '/' as i32 {
                    if xmlPatternAdd(
                        ctxt,
                        unsafe { (*ctxt).comp },
                        XML_OP_ROOT,
                        0 as *mut xmlChar,
                        0 as *mut xmlChar,
                    ) != 0
                    {
                        current_block = 17489734837053406682;
                    } else {
                        if (unsafe { *(*ctxt).cur }) as i32 != 0 {
                            let fresh56 = unsafe { &mut ((*ctxt).cur) };
                            *fresh56 = unsafe { (*fresh56).offset(1) };
                        } else {
                        };
                        while (unsafe { *(*ctxt).cur }) as i32 == 0x20 as i32
                            || 0x9 as i32 <= (unsafe { *(*ctxt).cur }) as i32
                                && (unsafe { *(*ctxt).cur }) as i32 <= 0xa as i32
                            || (unsafe { *(*ctxt).cur }) as i32 == 0xd as i32
                        {
                            if (unsafe { *(*ctxt).cur }) as i32 != 0 {
                                let fresh57 = unsafe { &mut ((*ctxt).cur) };
                                *fresh57 = unsafe { (*fresh57).offset(1) };
                            } else {
                            };
                        }
                        if (unsafe { *(*ctxt).cur }) as i32 == 0 as i32 {
                            (unsafe { (*ctxt).error = 1 as i32 });
                            current_block = 17489734837053406682;
                        } else {
                            current_block = 15512526488502093901;
                        }
                    }
                } else {
                    current_block = 15512526488502093901;
                }
                match current_block {
                    17489734837053406682 => {},
                    _ => {
                        xmlCompileStepPattern(ctxt);
                        if (unsafe { (*ctxt).error }) != 0 as i32 {
                            current_block = 17489734837053406682;
                        } else {
                            while (unsafe { *(*ctxt).cur }) as i32 == 0x20 as i32
                                || 0x9 as i32 <= (unsafe { *(*ctxt).cur }) as i32
                                    && (unsafe { *(*ctxt).cur }) as i32 <= 0xa as i32
                                || (unsafe { *(*ctxt).cur }) as i32 == 0xd as i32
                            {
                                if (unsafe { *(*ctxt).cur }) as i32 != 0 {
                                    let fresh58 = unsafe { &mut ((*ctxt).cur) };
                                    *fresh58 = unsafe { (*fresh58).offset(1) };
                                } else {
                                };
                            }
                            loop {
                                if !((unsafe { *(*ctxt).cur }) as i32 == '/' as i32) {
                                    current_block = 7189308829251266000;
                                    break;
                                }
                                if (unsafe { *((*ctxt).cur).offset(1 as i32 as isize) }) as i32 == '/' as i32 {
                                    if xmlPatternAdd(
                                        ctxt,
                                        unsafe { (*ctxt).comp },
                                        XML_OP_ANCESTOR,
                                        0 as *mut xmlChar,
                                        0 as *mut xmlChar,
                                    ) != 0
                                    {
                                        current_block = 17489734837053406682;
                                        break;
                                    }
                                    if (unsafe { *(*ctxt).cur }) as i32 != 0 {
                                        let fresh59 = unsafe { &mut ((*ctxt).cur) };
                                        *fresh59 = unsafe { (*fresh59).offset(1) };
                                    } else {
                                    };
                                    if (unsafe { *(*ctxt).cur }) as i32 != 0 {
                                        let fresh60 = unsafe { &mut ((*ctxt).cur) };
                                        *fresh60 = unsafe { (*fresh60).offset(1) };
                                    } else {
                                    };
                                    while (unsafe { *(*ctxt).cur }) as i32 == 0x20 as i32
                                        || 0x9 as i32 <= (unsafe { *(*ctxt).cur }) as i32
                                            && (unsafe { *(*ctxt).cur }) as i32 <= 0xa as i32
                                        || (unsafe { *(*ctxt).cur }) as i32 == 0xd as i32
                                    {
                                        if (unsafe { *(*ctxt).cur }) as i32 != 0 {
                                            let fresh61 = unsafe { &mut ((*ctxt).cur) };
                                            *fresh61 = unsafe { (*fresh61).offset(1) };
                                        } else {
                                        };
                                    }
                                    xmlCompileStepPattern(ctxt);
                                    if (unsafe { (*ctxt).error }) != 0 as i32 {
                                        current_block = 17489734837053406682;
                                        break;
                                    }
                                } else {
                                    if xmlPatternAdd(
                                        ctxt,
                                        unsafe { (*ctxt).comp },
                                        XML_OP_PARENT,
                                        0 as *mut xmlChar,
                                        0 as *mut xmlChar,
                                    ) != 0
                                    {
                                        current_block = 17489734837053406682;
                                        break;
                                    }
                                    if (unsafe { *(*ctxt).cur }) as i32 != 0 {
                                        let fresh62 = unsafe { &mut ((*ctxt).cur) };
                                        *fresh62 = unsafe { (*fresh62).offset(1) };
                                    } else {
                                    };
                                    while (unsafe { *(*ctxt).cur }) as i32 == 0x20 as i32
                                        || 0x9 as i32 <= (unsafe { *(*ctxt).cur }) as i32
                                            && (unsafe { *(*ctxt).cur }) as i32 <= 0xa as i32
                                        || (unsafe { *(*ctxt).cur }) as i32 == 0xd as i32
                                    {
                                        if (unsafe { *(*ctxt).cur }) as i32 != 0 {
                                            let fresh63 = unsafe { &mut ((*ctxt).cur) };
                                            *fresh63 = unsafe { (*fresh63).offset(1) };
                                        } else {
                                        };
                                    }
                                    if (unsafe { *(*ctxt).cur }) as i32 == 0 as i32 {
                                        (unsafe { (*ctxt).error = 1 as i32 });
                                        current_block = 17489734837053406682;
                                        break;
                                    } else {
                                        xmlCompileStepPattern(ctxt);
                                        if (unsafe { (*ctxt).error }) != 0 as i32 {
                                            current_block = 17489734837053406682;
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    },
                }
            }
            match current_block {
                17489734837053406682 => {},
                _ => {
                    if (unsafe { *(*ctxt).cur }) as i32 != 0 as i32 {
                        (unsafe { (*ctxt).error = 1 as i32 });
                    }
                },
            }
        },
        _ => {},
    };
}
extern "C" fn xmlCompileIDCXPathPath<'a1>(
    mut ctxt: *mut crate::src::pattern::_xmlPatParserContext<'a1>,
) {
    let mut current_block: u64;
    while (unsafe { *(*ctxt).cur }) as i32 == 0x20 as i32
        || 0x9 as i32 <= (unsafe { *(*ctxt).cur }) as i32 && (unsafe { *(*ctxt).cur }) as i32 <= 0xa as i32
        || (unsafe { *(*ctxt).cur }) as i32 == 0xd as i32
    {
        if (unsafe { *(*ctxt).cur }) as i32 != 0 {
            let fresh64 = unsafe { &mut ((*ctxt).cur) };
            *fresh64 = unsafe { (*fresh64).offset(1) };
        } else {
        };
    }
    if !((unsafe { *(*ctxt).cur }) as i32 == '/' as i32) {
        (unsafe { (*(*ctxt).comp).flags |= (1 as i32) << 9 as i32 });
        if (unsafe { *(*ctxt).cur }) as i32 == '.' as i32 {
            if (unsafe { *(*ctxt).cur }) as i32 != 0 {
                let fresh65 = unsafe { &mut ((*ctxt).cur) };
                *fresh65 = unsafe { (*fresh65).offset(1) };
            } else {
            };
            while (unsafe { *(*ctxt).cur }) as i32 == 0x20 as i32
                || 0x9 as i32 <= (unsafe { *(*ctxt).cur }) as i32 && (unsafe { *(*ctxt).cur }) as i32 <= 0xa as i32
                || (unsafe { *(*ctxt).cur }) as i32 == 0xd as i32
            {
                if (unsafe { *(*ctxt).cur }) as i32 != 0 {
                    let fresh66 = unsafe { &mut ((*ctxt).cur) };
                    *fresh66 = unsafe { (*fresh66).offset(1) };
                } else {
                };
            }
            if (unsafe { *(*ctxt).cur }) as i32 == 0 as i32 {
                if xmlPatternAdd(
                    ctxt,
                    unsafe { (*ctxt).comp },
                    XML_OP_ELEM,
                    0 as *mut xmlChar,
                    0 as *mut xmlChar,
                ) != 0
                {
                    current_block = 10466991778982128886;
                } else {
                    return;
                }
            } else if (unsafe { *(*ctxt).cur }) as i32 != '/' as i32 {
                current_block = 10466991778982128886;
            } else {
                if (unsafe { *(*ctxt).cur }) as i32 != 0 {
                    let fresh67 = unsafe { &mut ((*ctxt).cur) };
                    *fresh67 = unsafe { (*fresh67).offset(1) };
                } else {
                };
                while (unsafe { *(*ctxt).cur }) as i32 == 0x20 as i32
                    || 0x9 as i32 <= (unsafe { *(*ctxt).cur }) as i32 && (unsafe { *(*ctxt).cur }) as i32 <= 0xa as i32
                    || (unsafe { *(*ctxt).cur }) as i32 == 0xd as i32
                {
                    if (unsafe { *(*ctxt).cur }) as i32 != 0 {
                        let fresh68 = unsafe { &mut ((*ctxt).cur) };
                        *fresh68 = unsafe { (*fresh68).offset(1) };
                    } else {
                    };
                }
                if (unsafe { *(*ctxt).cur }) as i32 == '/' as i32 {
                    if (unsafe { *((*ctxt).cur).offset(-(1 as i32) as isize) }) as i32 == 0x20 as i32
                        || 0x9 as i32 <= (unsafe { *((*ctxt).cur).offset(-(1 as i32) as isize) }) as i32
                            && (unsafe { *((*ctxt).cur).offset(-(1 as i32) as isize) }) as i32 <= 0xa as i32
                        || (unsafe { *((*ctxt).cur).offset(-(1 as i32) as isize) }) as i32 == 0xd as i32
                    {
                        current_block = 10466991778982128886;
                    } else if xmlPatternAdd(
                        ctxt,
                        unsafe { (*ctxt).comp },
                        XML_OP_ANCESTOR,
                        0 as *mut xmlChar,
                        0 as *mut xmlChar,
                    ) != 0
                    {
                        current_block = 10466991778982128886;
                    } else {
                        if (unsafe { *(*ctxt).cur }) as i32 != 0 {
                            let fresh69 = unsafe { &mut ((*ctxt).cur) };
                            *fresh69 = unsafe { (*fresh69).offset(1) };
                        } else {
                        };
                        while (unsafe { *(*ctxt).cur }) as i32 == 0x20 as i32
                            || 0x9 as i32 <= (unsafe { *(*ctxt).cur }) as i32
                                && (unsafe { *(*ctxt).cur }) as i32 <= 0xa as i32
                            || (unsafe { *(*ctxt).cur }) as i32 == 0xd as i32
                        {
                            if (unsafe { *(*ctxt).cur }) as i32 != 0 {
                                let fresh70 = unsafe { &mut ((*ctxt).cur) };
                                *fresh70 = unsafe { (*fresh70).offset(1) };
                            } else {
                            };
                        }
                        current_block = 14818589718467733107;
                    }
                } else {
                    current_block = 14818589718467733107;
                }
                match current_block {
                    10466991778982128886 => {},
                    _ => {
                        if (unsafe { *(*ctxt).cur }) as i32 == 0 as i32 {
                            current_block = 11874738112936171638;
                        } else {
                            current_block = 11932355480408055363;
                        }
                    },
                }
            }
        } else {
            current_block = 11932355480408055363;
        }
        match current_block {
            10466991778982128886 => {},
            _ => {
                loop {
                    match current_block {
                        11874738112936171638 => {
                            (unsafe { (*ctxt).error = 1 as i32 });
                            return;
                        },
                        _ => {
                            xmlCompileStepPattern(ctxt);
                            if (unsafe { (*ctxt).error }) != 0 as i32 {
                                current_block = 10466991778982128886;
                                break;
                            }
                            while (unsafe { *(*ctxt).cur }) as i32 == 0x20 as i32
                                || 0x9 as i32 <= (unsafe { *(*ctxt).cur }) as i32
                                    && (unsafe { *(*ctxt).cur }) as i32 <= 0xa as i32
                                || (unsafe { *(*ctxt).cur }) as i32 == 0xd as i32
                            {
                                if (unsafe { *(*ctxt).cur }) as i32 != 0 {
                                    let fresh71 = unsafe { &mut ((*ctxt).cur) };
                                    *fresh71 = unsafe { (*fresh71).offset(1) };
                                } else {
                                };
                            }
                            if (unsafe { *(*ctxt).cur }) as i32 != '/' as i32 {
                                current_block = 15004371738079956865;
                                break;
                            }
                            if xmlPatternAdd(
                                ctxt,
                                unsafe { (*ctxt).comp },
                                XML_OP_PARENT,
                                0 as *mut xmlChar,
                                0 as *mut xmlChar,
                            ) != 0
                            {
                                current_block = 10466991778982128886;
                                break;
                            }
                            if (unsafe { *(*ctxt).cur }) as i32 != 0 {
                                let fresh72 = unsafe { &mut ((*ctxt).cur) };
                                *fresh72 = unsafe { (*fresh72).offset(1) };
                            } else {
                            };
                            while (unsafe { *(*ctxt).cur }) as i32 == 0x20 as i32
                                || 0x9 as i32 <= (unsafe { *(*ctxt).cur }) as i32
                                    && (unsafe { *(*ctxt).cur }) as i32 <= 0xa as i32
                                || (unsafe { *(*ctxt).cur }) as i32 == 0xd as i32
                            {
                                if (unsafe { *(*ctxt).cur }) as i32 != 0 {
                                    let fresh73 = unsafe { &mut ((*ctxt).cur) };
                                    *fresh73 = unsafe { (*fresh73).offset(1) };
                                } else {
                                };
                            }
                            if (unsafe { *(*ctxt).cur }) as i32 == '/' as i32 {
                                current_block = 10466991778982128886;
                                break;
                            }
                            if (unsafe { *(*ctxt).cur }) as i32 == 0 as i32 {
                                current_block = 11874738112936171638;
                                continue;
                            }
                            if (unsafe { *(*ctxt).cur }) as i32 != 0 as i32 {
                                current_block = 11932355480408055363;
                            } else {
                                current_block = 15004371738079956865;
                                break;
                            }
                        },
                    }
                }
                match current_block {
                    10466991778982128886 => {},
                    _ => {
                        if (unsafe { *(*ctxt).cur }) as i32 != 0 as i32 {
                            (unsafe { (*ctxt).error = 1 as i32 });
                        }
                        return;
                    },
                }
            },
        }
    }
    (unsafe { (*ctxt).error = 1 as i32 });
}
extern "C" fn xmlNewStreamComp(mut size: i32) -> *mut crate::src::pattern::_xmlStreamComp {
    let mut cur: *mut crate::src::pattern::_xmlStreamComp = 0 as *mut xmlStreamComp;
    if size < 4 as i32 {
        size = 4 as i32;
    }
    cur = (unsafe { xmlMalloc.expect("non-null function pointer")(
        ::std::mem::size_of::<xmlStreamComp>() as u64
    ) }) as xmlStreamCompPtr;
    if cur.is_null() {
        return 0 as xmlStreamCompPtr;
    }
    (unsafe { memset(
        cur as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<xmlStreamComp>() as u64,
    ) });
    let fresh74 = unsafe { &mut ((*cur).steps) };
    *fresh74 = (unsafe { xmlMalloc.expect("non-null function pointer")(
        (size as u64).wrapping_mul(::std::mem::size_of::<xmlStreamStep>() as u64),
    ) }) as xmlStreamStepPtr;
    if (unsafe { (*cur).steps }).is_null() {
        (unsafe { xmlFree.expect("non-null function pointer")(cur as *mut libc::c_void) });
        return 0 as xmlStreamCompPtr;
    }
    (unsafe { (*cur).nbStep = 0 as i32 });
    (unsafe { (*cur).maxStep = size });
    return cur;
}
extern "C" fn xmlFreeStreamComp(mut comp: *mut crate::src::pattern::_xmlStreamComp) {
    if !comp.is_null() {
        if !(unsafe { (*comp).steps }).is_null() {
            (unsafe { xmlFree.expect("non-null function pointer")((*comp).steps as *mut libc::c_void) });
        }
        if !(unsafe { (*comp).dict }).is_null() {
            xmlDictFree(unsafe { (*comp).dict });
        }
        (unsafe { xmlFree.expect("non-null function pointer")(comp as *mut libc::c_void) });
    }
}
extern "C" fn xmlStreamCompAddStep(
    mut comp: *mut crate::src::pattern::_xmlStreamComp,
    mut name: *const u8,
    mut ns: *const u8,
    mut nodeType: i32,
    mut flags: i32,
) -> i32 {
    let mut cur: *mut crate::src::pattern::_xmlStreamStep = 0 as *mut xmlStreamStep;
    if (unsafe { (*comp).nbStep }) >= (unsafe { (*comp).maxStep }) {
        cur = (unsafe { xmlRealloc.expect("non-null function pointer")(
            (*comp).steps as *mut libc::c_void,
            (((*comp).maxStep * 2 as i32) as u64)
                .wrapping_mul(::std::mem::size_of::<xmlStreamStep>() as u64),
        ) }) as xmlStreamStepPtr;
        if cur.is_null() {
            return -(1 as i32);
        }
        let fresh75 = unsafe { &mut ((*comp).steps) };
        *fresh75 = cur;
        (unsafe { (*comp).maxStep *= 2 as i32 });
    }
    let fresh76 = unsafe { &mut ((*comp).nbStep) };
    let mut fresh77 = *fresh76;
    *fresh76 = *fresh76 + 1;
    cur = (unsafe { &mut *((*comp).steps).offset(fresh77 as isize) }) as *mut xmlStreamStep;
    (unsafe { (*cur).flags = flags });
    let fresh78 = unsafe { &mut ((*cur).name) };
    *fresh78 = name;
    let fresh79 = unsafe { &mut ((*cur).ns) };
    *fresh79 = ns;
    (unsafe { (*cur).nodeType = nodeType });
    return (unsafe { (*comp).nbStep }) - 1 as i32;
}
extern "C" fn xmlStreamCompile<'a1>(mut comp: *mut crate::src::pattern::_xmlPattern<'a1>) -> i32 {
    let mut current_block: u64;
    let mut stream: *mut crate::src::pattern::_xmlStreamComp = 0 as *mut xmlStreamComp;
    let mut i: i32 = 0;
    let mut s: i32 = 0 as i32;
    let mut root: i32 = 0 as i32;
    let mut flags: i32 = 0 as i32;
    let mut prevs: i32 = -(1 as i32);
    let mut step: crate::src::pattern::_xmlStepOp = xmlStepOp {
        op: XML_OP_END,
        value: 0 as *const xmlChar,
        value2: 0 as *const xmlChar,
    };
    if comp.is_null() || (unsafe { (*comp).steps }).is_null() {
        return -(1 as i32);
    }
    if (unsafe { (*comp).nbStep }) == 1 as i32
        && (unsafe { (*((*comp).steps).offset(0 as i32 as isize)).op }) as u32 == XML_OP_ELEM as i32 as u32
        && (unsafe { (*((*comp).steps).offset(0 as i32 as isize)).value }).is_null()
        && (unsafe { (*((*comp).steps).offset(0 as i32 as isize)).value2 }).is_null()
    {
        stream = xmlNewStreamComp(0 as i32);
        if stream.is_null() {
            return -(1 as i32);
        }
        (unsafe { (*stream).flags |= (1 as i32) << 14 as i32 });
        let fresh80 = unsafe { &mut ((*comp).stream) };
        *fresh80 = stream;
        return 0 as i32;
    }
    stream = xmlNewStreamComp((unsafe { (*comp).nbStep }) / 2 as i32 + 1 as i32);
    if stream.is_null() {
        return -(1 as i32);
    }
    if !(unsafe { (*comp).dict }).is_null() {
        let fresh81 = unsafe { &mut ((*stream).dict) };
        *fresh81 = unsafe { (*comp).dict };
        xmlDictReference(unsafe { (*stream).dict });
    }
    i = 0 as i32;
    if (unsafe { (*comp).flags }) & (1 as i32) << 8 as i32 != 0 {
        (unsafe { (*stream).flags |= (1 as i32) << 15 as i32 });
    }
    loop {
        if !(i < (unsafe { (*comp).nbStep })) {
            current_block = 12264624100856317061;
            break;
        }
        step = unsafe { *((*comp).steps).offset(i as isize) };
        match step.op as u32 {
            1 => {
                if i != 0 as i32 {
                    current_block = 9045084312945070449;
                    break;
                }
                root = 1 as i32;
            },
            7 => {
                s = xmlStreamCompAddStep(
                    stream,
                    0 as *const xmlChar,
                    step.value,
                    XML_ELEMENT_NODE as i32,
                    flags,
                );
                if s < 0 as i32 {
                    current_block = 9045084312945070449;
                    break;
                }
                prevs = s;
                flags = 0 as i32;
            },
            4 => {
                flags |= 8 as i32;
                prevs = -(1 as i32);
                s = xmlStreamCompAddStep(
                    stream,
                    step.value,
                    step.value2,
                    XML_ATTRIBUTE_NODE as i32,
                    flags,
                );
                flags = 0 as i32;
                if s < 0 as i32 {
                    current_block = 9045084312945070449;
                    break;
                }
            },
            2 => {
                if (step.value).is_null() && (step.value2).is_null() {
                    if (unsafe { (*comp).nbStep }) == i + 1 as i32 && flags & 1 as i32 != 0 {
                        if (unsafe { (*comp).nbStep }) == i + 1 as i32 {
                            (unsafe { (*stream).flags |= (1 as i32) << 14 as i32 });
                        }
                        flags |= 16 as i32;
                        s = xmlStreamCompAddStep(
                            stream,
                            0 as *const xmlChar,
                            0 as *const xmlChar,
                            100 as i32,
                            flags,
                        );
                        if s < 0 as i32 {
                            current_block = 9045084312945070449;
                            break;
                        }
                        flags = 0 as i32;
                        if prevs != -(1 as i32) {
                            (unsafe { (*((*stream).steps).offset(prevs as isize)).flags |= 32 as i32 });
                            prevs = -(1 as i32);
                        }
                    }
                } else {
                    s = xmlStreamCompAddStep(
                        stream,
                        step.value,
                        step.value2,
                        XML_ELEMENT_NODE as i32,
                        flags,
                    );
                    if s < 0 as i32 {
                        current_block = 9045084312945070449;
                        break;
                    }
                    prevs = s;
                    flags = 0 as i32;
                }
            },
            3 => {
                s = xmlStreamCompAddStep(
                    stream,
                    step.value,
                    step.value2,
                    XML_ELEMENT_NODE as i32,
                    flags,
                );
                if s < 0 as i32 {
                    current_block = 9045084312945070449;
                    break;
                }
                prevs = s;
                flags = 0 as i32;
            },
            8 => {
                s = xmlStreamCompAddStep(
                    stream,
                    0 as *const xmlChar,
                    0 as *const xmlChar,
                    XML_ELEMENT_NODE as i32,
                    flags,
                );
                if s < 0 as i32 {
                    current_block = 9045084312945070449;
                    break;
                }
                prevs = s;
                flags = 0 as i32;
            },
            6 => {
                if !(flags & 1 as i32 != 0) {
                    flags |= 1 as i32;
                    if (unsafe { (*stream).flags }) & (1 as i32) << 16 as i32 == 0 as i32 {
                        (unsafe { (*stream).flags |= (1 as i32) << 16 as i32 });
                    }
                }
            },
            0 | 5 | _ => {},
        }
        i += 1;
    }
    match current_block {
        12264624100856317061 => {
            if root == 0
                && (unsafe { (*comp).flags })
                    & (XML_PATTERN_XPATH as i32
                        | XML_PATTERN_XSSEL as i32
                        | XML_PATTERN_XSFIELD as i32)
                    == 0 as i32
            {
                if (unsafe { (*stream).flags }) & (1 as i32) << 16 as i32 == 0 as i32 {
                    (unsafe { (*stream).flags |= (1 as i32) << 16 as i32 });
                }
                if (unsafe { (*stream).nbStep }) > 0 as i32 {
                    if (unsafe { (*((*stream).steps).offset(0 as i32 as isize)).flags }) & 1 as i32 == 0 as i32 {
                        (unsafe { (*((*stream).steps).offset(0 as i32 as isize)).flags |= 1 as i32 });
                    }
                }
            }
            if !((unsafe { (*stream).nbStep }) <= s) {
                (unsafe { (*((*stream).steps).offset(s as isize)).flags |= 2 as i32 });
                if root != 0 {
                    (unsafe { (*((*stream).steps).offset(0 as i32 as isize)).flags |= 4 as i32 });
                }
                let fresh82 = unsafe { &mut ((*comp).stream) };
                *fresh82 = stream;
                return 0 as i32;
            }
        },
        _ => {},
    }
    xmlFreeStreamComp(stream);
    return 0 as i32;
}
extern "C" fn xmlNewStreamCtxt(
    mut stream: *mut crate::src::pattern::_xmlStreamComp,
) -> *mut crate::src::pattern::_xmlStreamCtxt {
    let mut cur: *mut crate::src::pattern::_xmlStreamCtxt = 0 as *mut xmlStreamCtxt;
    cur = (unsafe { xmlMalloc.expect("non-null function pointer")(
        ::std::mem::size_of::<xmlStreamCtxt>() as u64
    ) }) as xmlStreamCtxtPtr;
    if cur.is_null() {
        return 0 as xmlStreamCtxtPtr;
    }
    (unsafe { memset(
        cur as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<xmlStreamCtxt>() as u64,
    ) });
    let fresh83 = unsafe { &mut ((*cur).states) };
    *fresh83 = (unsafe { xmlMalloc.expect("non-null function pointer")(
        ((4 as i32 * 2 as i32) as u64).wrapping_mul(::std::mem::size_of::<i32>() as u64),
    ) }) as *mut i32;
    if (unsafe { (*cur).states }).is_null() {
        (unsafe { xmlFree.expect("non-null function pointer")(cur as *mut libc::c_void) });
        return 0 as xmlStreamCtxtPtr;
    }
    (unsafe { (*cur).nbState = 0 as i32 });
    (unsafe { (*cur).maxState = 4 as i32 });
    (unsafe { (*cur).level = 0 as i32 });
    let fresh84 = unsafe { &mut ((*cur).comp) };
    *fresh84 = stream;
    (unsafe { (*cur).blockLevel = -(1 as i32) });
    return cur;
}
#[no_mangle]
pub extern "C" fn xmlFreeStreamCtxt(mut stream: *mut crate::src::pattern::_xmlStreamCtxt) {
    let mut next: *mut crate::src::pattern::_xmlStreamCtxt = 0 as *mut xmlStreamCtxt;
    while !stream.is_null() {
        next = unsafe { (*stream).next };
        if !(unsafe { (*stream).states }).is_null() {
            (unsafe { xmlFree.expect("non-null function pointer")((*stream).states as *mut libc::c_void) });
        }
        (unsafe { xmlFree.expect("non-null function pointer")(stream as *mut libc::c_void) });
        stream = next;
    }
}
extern "C" fn xmlStreamCtxtAddState(
    mut comp: *mut crate::src::pattern::_xmlStreamCtxt,
    mut idx: i32,
    mut level: i32,
) -> i32 {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < (unsafe { (*comp).nbState }) {
        if (unsafe { *((*comp).states).offset((2 as i32 * i) as isize) }) < 0 as i32 {
            (unsafe { *((*comp).states).offset((2 as i32 * i) as isize) = idx });
            (unsafe { *((*comp).states).offset((2 as i32 * i + 1 as i32) as isize) = level });
            return i;
        }
        i += 1;
    }
    if (unsafe { (*comp).nbState }) >= (unsafe { (*comp).maxState }) {
        let mut cur: *mut i32 = 0 as *mut i32;
        cur = (unsafe { xmlRealloc.expect("non-null function pointer")(
            (*comp).states as *mut libc::c_void,
            (((*comp).maxState * 4 as i32) as u64)
                .wrapping_mul(::std::mem::size_of::<i32>() as u64),
        ) }) as *mut i32;
        if cur.is_null() {
            return -(1 as i32);
        }
        let fresh85 = unsafe { &mut ((*comp).states) };
        *fresh85 = cur;
        (unsafe { (*comp).maxState *= 2 as i32 });
    }
    (unsafe { *((*comp).states).offset((2 as i32 * (*comp).nbState) as isize) = idx });
    let fresh86 = unsafe { &mut ((*comp).nbState) };
    let mut fresh87 = *fresh86;
    *fresh86 = *fresh86 + 1;
    (unsafe { *((*comp).states).offset((2 as i32 * fresh87 + 1 as i32) as isize) = level });
    return (unsafe { (*comp).nbState }) - 1 as i32;
}
extern "C" fn xmlStreamPushInternal(
    mut stream: *mut crate::src::pattern::_xmlStreamCtxt,
    mut name: *const u8,
    mut ns: *const u8,
    mut nodeType: i32,
) -> i32 {
    let mut current_block: u64;
    let mut ret: i32 = 0 as i32;
    let mut err: i32 = 0 as i32;
    let mut final_0: i32 = 0 as i32;
    let mut tmp: i32 = 0;
    let mut i: i32 = 0;
    let mut m: i32 = 0;
    let mut match_0: i32 = 0;
    let mut stepNr: i32 = 0;
    let mut desc: i32 = 0;
    let mut comp: *mut crate::src::pattern::_xmlStreamComp = 0 as *mut xmlStreamComp;
    let mut step: crate::src::pattern::_xmlStreamStep = xmlStreamStep {
        flags: 0,
        name: 0 as *const xmlChar,
        ns: 0 as *const xmlChar,
        nodeType: 0,
    };
    if stream.is_null() || (unsafe { (*stream).nbState }) < 0 as i32 {
        return -(1 as i32);
    }
    while !stream.is_null() {
        comp = unsafe { (*stream).comp };
        if nodeType == XML_ELEMENT_NODE as i32 && name.is_null() && ns.is_null() {
            (unsafe { (*stream).nbState = 0 as i32 });
            (unsafe { (*stream).level = 0 as i32 });
            (unsafe { (*stream).blockLevel = -(1 as i32) });
            if (unsafe { (*comp).flags }) & (1 as i32) << 15 as i32 != 0 {
                if (unsafe { (*comp).nbStep }) == 0 as i32 {
                    ret = 1 as i32;
                } else if (unsafe { (*comp).nbStep }) == 1 as i32
                    && (unsafe { (*((*comp).steps).offset(0 as i32 as isize)).nodeType }) == 100 as i32
                    && (unsafe { (*((*comp).steps).offset(0 as i32 as isize)).flags }) & 1 as i32 != 0
                {
                    ret = 1 as i32;
                } else if (unsafe { (*((*comp).steps).offset(0 as i32 as isize)).flags }) & 4 as i32 != 0 {
                    tmp = xmlStreamCtxtAddState(stream, 0 as i32, 0 as i32);
                    if tmp < 0 as i32 {
                        err += 1;
                    }
                }
            }
            stream = unsafe { (*stream).next };
        } else {
            if (unsafe { (*comp).nbStep }) == 0 as i32 {
                if (unsafe { (*stream).flags }) & XML_PATTERN_XPATH as i32 != 0 {
                    stream = unsafe { (*stream).next };
                    continue;
                } else {
                    if nodeType != XML_ATTRIBUTE_NODE as i32
                        && ((unsafe { (*stream).flags })
                            & (XML_PATTERN_XPATH as i32
                                | XML_PATTERN_XSSEL as i32
                                | XML_PATTERN_XSFIELD as i32)
                            == 0 as i32
                            || (unsafe { (*stream).level }) == 0 as i32)
                    {
                        ret = 1 as i32;
                    }
                    let fresh88 = unsafe { &mut ((*stream).level) };
                    *fresh88 += 1;
                }
            } else if (unsafe { (*stream).blockLevel }) != -(1 as i32) {
                let fresh89 = unsafe { &mut ((*stream).level) };
                *fresh89 += 1;
            } else if nodeType != XML_ELEMENT_NODE as i32
                && nodeType != XML_ATTRIBUTE_NODE as i32
                && (unsafe { (*comp).flags }) & (1 as i32) << 14 as i32 == 0 as i32
            {
                let fresh90 = unsafe { &mut ((*stream).level) };
                *fresh90 += 1;
            } else {
                i = 0 as i32;
                m = unsafe { (*stream).nbState };
                while i < m {
                    if (unsafe { (*comp).flags }) & (1 as i32) << 16 as i32 == 0 as i32 {
                        stepNr = unsafe { *((*stream).states)
                            .offset((2 as i32 * ((*stream).nbState - 1 as i32)) as isize) };
                        if (unsafe { *((*stream).states)
                            .offset((2 as i32 * ((*stream).nbState - 1 as i32) + 1 as i32) as isize) })
                            < (unsafe { (*stream).level })
                        {
                            return -(1 as i32);
                        }
                        desc = 0 as i32;
                        i = m;
                        current_block = 2516253395664191498;
                    } else {
                        stepNr = unsafe { *((*stream).states).offset((2 as i32 * i) as isize) };
                        if stepNr < 0 as i32 {
                            current_block = 11581334008138293573;
                        } else {
                            tmp = unsafe { *((*stream).states).offset((2 as i32 * i + 1 as i32) as isize) };
                            if tmp > (unsafe { (*stream).level }) {
                                current_block = 11581334008138293573;
                            } else {
                                desc = (unsafe { (*((*comp).steps).offset(stepNr as isize)).flags }) & 1 as i32;
                                if tmp < (unsafe { (*stream).level }) && desc == 0 {
                                    current_block = 11581334008138293573;
                                } else {
                                    current_block = 2516253395664191498;
                                }
                            }
                        }
                    }
                    match current_block {
                        2516253395664191498 => {
                            step = unsafe { *((*comp).steps).offset(stepNr as isize) };
                            if step.nodeType != nodeType {
                                if step.nodeType == XML_ATTRIBUTE_NODE as i32 {
                                    if (unsafe { (*comp).flags }) & (1 as i32) << 16 as i32 == 0 as i32 {
                                        (unsafe { (*stream).blockLevel = (*stream).level + 1 as i32 });
                                    }
                                    current_block = 11581334008138293573;
                                } else if step.nodeType != 100 as i32 {
                                    current_block = 11581334008138293573;
                                } else {
                                    current_block = 4741994311446740739;
                                }
                            } else {
                                current_block = 4741994311446740739;
                            }
                            match current_block {
                                11581334008138293573 => {},
                                _ => {
                                    match_0 = 0 as i32;
                                    if step.nodeType == 100 as i32 {
                                        match_0 = 1 as i32;
                                    } else if (step.name).is_null() {
                                        if (step.ns).is_null() {
                                            match_0 = 1 as i32;
                                        } else if !ns.is_null() {
                                            match_0 = unsafe { xmlStrEqual(step.ns, ns) };
                                        }
                                    } else if (step.ns != 0 as *mut libc::c_void as *const xmlChar)
                                        as i32
                                        == (ns != 0 as *mut libc::c_void as *const xmlChar) as i32
                                        && !name.is_null()
                                        && (unsafe { *(step.name).offset(0 as i32 as isize) }) as i32
                                            == (unsafe { *name.offset(0 as i32 as isize) }) as i32
                                        && (unsafe { xmlStrEqual(step.name, name) }) != 0
                                        && (step.ns == ns || (unsafe { xmlStrEqual(step.ns, ns) }) != 0)
                                    {
                                        match_0 = 1 as i32;
                                    }
                                    if match_0 != 0 {
                                        final_0 = step.flags & 2 as i32;
                                        if desc != 0 {
                                            if final_0 != 0 {
                                                ret = 1 as i32;
                                            } else {
                                                xmlStreamCtxtAddState(
                                                    stream,
                                                    stepNr + 1 as i32,
                                                    (unsafe { (*stream).level }) + 1 as i32,
                                                );
                                            }
                                        } else if final_0 != 0 {
                                            ret = 1 as i32;
                                        } else {
                                            xmlStreamCtxtAddState(
                                                stream,
                                                stepNr + 1 as i32,
                                                (unsafe { (*stream).level }) + 1 as i32,
                                            );
                                        }
                                        if ret != 1 as i32 && step.flags & 32 as i32 != 0 {
                                            ret = 1 as i32;
                                        }
                                    }
                                    if (unsafe { (*comp).flags }) & (1 as i32) << 16 as i32 == 0 as i32
                                        && (match_0 == 0 || final_0 != 0)
                                    {
                                        (unsafe { (*stream).blockLevel = (*stream).level + 1 as i32 });
                                    }
                                },
                            }
                        },
                        _ => {},
                    }
                    i += 1;
                }
                let fresh91 = unsafe { &mut ((*stream).level) };
                *fresh91 += 1;
                step = unsafe { *((*comp).steps).offset(0 as i32 as isize) };
                if !(step.flags & 4 as i32 != 0) {
                    desc = step.flags & 1 as i32;
                    if (unsafe { (*stream).flags })
                        & (XML_PATTERN_XPATH as i32
                            | XML_PATTERN_XSSEL as i32
                            | XML_PATTERN_XSFIELD as i32)
                        != 0
                    {
                        if (unsafe { (*stream).level }) == 1 as i32 {
                            if (unsafe { (*stream).flags })
                                & (XML_PATTERN_XSSEL as i32 | XML_PATTERN_XSFIELD as i32)
                                != 0
                            {
                                current_block = 9048011128714838703;
                            } else {
                                current_block = 14442360071374423104;
                            }
                        } else if desc != 0 {
                            current_block = 14442360071374423104;
                        } else if (unsafe { (*stream).level }) == 2 as i32
                            && (unsafe { (*stream).flags })
                                & (XML_PATTERN_XSSEL as i32 | XML_PATTERN_XSFIELD as i32)
                                != 0
                        {
                            current_block = 14442360071374423104;
                        } else {
                            current_block = 9048011128714838703;
                        }
                    } else {
                        current_block = 14442360071374423104;
                    }
                    match current_block {
                        9048011128714838703 => {},
                        _ => {
                            if step.nodeType != nodeType {
                                if nodeType == XML_ATTRIBUTE_NODE as i32 {
                                    current_block = 9048011128714838703;
                                } else if step.nodeType != 100 as i32 {
                                    current_block = 9048011128714838703;
                                } else {
                                    current_block = 11674240781755647963;
                                }
                            } else {
                                current_block = 11674240781755647963;
                            }
                            match current_block {
                                9048011128714838703 => {},
                                _ => {
                                    match_0 = 0 as i32;
                                    if step.nodeType == 100 as i32 {
                                        match_0 = 1 as i32;
                                    } else if (step.name).is_null() {
                                        if (step.ns).is_null() {
                                            match_0 = 1 as i32;
                                        } else if !ns.is_null() {
                                            match_0 = unsafe { xmlStrEqual(step.ns, ns) };
                                        }
                                    } else if (step.ns != 0 as *mut libc::c_void as *const xmlChar)
                                        as i32
                                        == (ns != 0 as *mut libc::c_void as *const xmlChar) as i32
                                        && !name.is_null()
                                        && (unsafe { *(step.name).offset(0 as i32 as isize) }) as i32
                                            == (unsafe { *name.offset(0 as i32 as isize) }) as i32
                                        && (unsafe { xmlStrEqual(step.name, name) }) != 0
                                        && (step.ns == ns || (unsafe { xmlStrEqual(step.ns, ns) }) != 0)
                                    {
                                        match_0 = 1 as i32;
                                    }
                                    final_0 = step.flags & 2 as i32;
                                    if match_0 != 0 {
                                        if final_0 != 0 {
                                            ret = 1 as i32;
                                        } else {
                                            xmlStreamCtxtAddState(
                                                stream,
                                                1 as i32,
                                                unsafe { (*stream).level },
                                            );
                                        }
                                        if ret != 1 as i32 && step.flags & 32 as i32 != 0 {
                                            ret = 1 as i32;
                                        }
                                    }
                                    if (unsafe { (*comp).flags }) & (1 as i32) << 16 as i32 == 0 as i32
                                        && (match_0 == 0 || final_0 != 0)
                                    {
                                        (unsafe { (*stream).blockLevel = (*stream).level });
                                    }
                                },
                            }
                        },
                    }
                }
            }
            stream = unsafe { (*stream).next };
        }
    }
    if err > 0 as i32 {
        ret = -(1 as i32);
    }
    return ret;
}
#[no_mangle]
pub extern "C" fn xmlStreamPush(
    mut stream: *mut crate::src::pattern::_xmlStreamCtxt,
    mut name: *const u8,
    mut ns: *const u8,
) -> i32 {
    return xmlStreamPushInternal(stream, name, ns, XML_ELEMENT_NODE as i32);
}
#[no_mangle]
pub extern "C" fn xmlStreamPushNode(
    mut stream: *mut crate::src::pattern::_xmlStreamCtxt,
    mut name: *const u8,
    mut ns: *const u8,
    mut nodeType: i32,
) -> i32 {
    return xmlStreamPushInternal(stream, name, ns, nodeType);
}
#[no_mangle]
pub extern "C" fn xmlStreamPushAttr(
    mut stream: *mut crate::src::pattern::_xmlStreamCtxt,
    mut name: *const u8,
    mut ns: *const u8,
) -> i32 {
    return xmlStreamPushInternal(stream, name, ns, XML_ATTRIBUTE_NODE as i32);
}
#[no_mangle]
pub extern "C" fn xmlStreamPop(mut stream: *mut crate::src::pattern::_xmlStreamCtxt) -> i32 {
    let mut i: i32 = 0;
    let mut lev: i32 = 0;
    if stream.is_null() {
        return -(1 as i32);
    }
    while !stream.is_null() {
        if (unsafe { (*stream).blockLevel }) == (unsafe { (*stream).level }) {
            (unsafe { (*stream).blockLevel = -(1 as i32) });
        }
        if (unsafe { (*stream).level }) != 0 {
            let fresh92 = unsafe { &mut ((*stream).level) };
            *fresh92 -= 1;
        }
        i = (unsafe { (*stream).nbState }) - 1 as i32;
        while i >= 0 as i32 {
            lev = unsafe { *((*stream).states).offset((2 as i32 * i + 1 as i32) as isize) };
            if lev > (unsafe { (*stream).level }) {
                let fresh93 = unsafe { &mut ((*stream).nbState) };
                *fresh93 -= 1;
            }
            if lev <= (unsafe { (*stream).level }) {
                break;
            }
            i -= 1;
        }
        stream = unsafe { (*stream).next };
    }
    return 0 as i32;
}
#[no_mangle]
pub extern "C" fn xmlStreamWantsAnyNode(
    mut streamCtxt: *mut crate::src::pattern::_xmlStreamCtxt,
) -> i32 {
    if streamCtxt.is_null() {
        return -(1 as i32);
    }
    while !streamCtxt.is_null() {
        if (unsafe { (*(*streamCtxt).comp).flags }) & (1 as i32) << 14 as i32 != 0 {
            return 1 as i32;
        }
        streamCtxt = unsafe { (*streamCtxt).next };
    }
    return 0 as i32;
}
#[no_mangle]
pub extern "C" fn xmlPatterncompile<'a1>(
    mut pattern: *const u8,
    mut dict: *mut crate::src::dict::_xmlDict,
    mut flags: i32,
    mut namespaces: *mut *const u8,
) -> *mut crate::src::pattern::_xmlPattern<'a1> {
    let mut current_block: u64;
    let mut ret: *mut crate::src::pattern::_xmlPattern<'_> = 0 as xmlPatternPtr;
    let mut cur: *mut crate::src::pattern::_xmlPattern<'_> = 0 as *mut xmlPattern;
    let mut ctxt: *mut crate::src::pattern::_xmlPatParserContext<'_> = 0 as xmlPatParserContextPtr;
    let mut or: *const u8 = 0 as *const xmlChar;
    let mut start: *const u8 = 0 as *const xmlChar;
    let mut tmp: *mut u8 = 0 as *mut xmlChar;
    let mut type_0: i32 = 0 as i32;
    let mut streamable: i32 = 1 as i32;
    if pattern.is_null() {
        return 0 as xmlPatternPtr;
    }
    start = pattern;
    or = start;
    loop {
        if !((unsafe { *or }) as i32 != 0 as i32) {
            current_block = 10380409671385728102;
            break;
        }
        tmp = 0 as *mut xmlChar;
        while (unsafe { *or }) as i32 != 0 as i32 && (unsafe { *or }) as i32 != '|' as i32 {
            or = unsafe { or.offset(1) };
        }
        if (unsafe { *or }) as i32 == 0 as i32 {
            ctxt = xmlNewPatParserContext(start, dict, namespaces);
        } else {
            tmp = unsafe { xmlStrndup(start, or.offset_from(start) as i64 as i32) };
            if !tmp.is_null() {
                ctxt = xmlNewPatParserContext(tmp, dict, namespaces);
            }
            or = unsafe { or.offset(1) };
        }
        if ctxt.is_null() {
            current_block = 13522574393598791978;
            break;
        }
        cur = xmlNewPattern();
        if cur.is_null() {
            current_block = 13522574393598791978;
            break;
        }
        if !dict.is_null() {
            let fresh94 = unsafe { &mut ((*cur).dict) };
            *fresh94 = dict;
            xmlDictReference(dict);
        }
        if ret.is_null() {
            ret = cur;
        } else {
            let fresh95 = unsafe { &mut ((*cur).next) };
            *fresh95 = unsafe { (*ret).next };
            let fresh96 = unsafe { &mut ((*ret).next) };
            *fresh96 = cur;
        }
        (unsafe { (*cur).flags = flags });
        let fresh97 = unsafe { &mut ((*ctxt).comp) };
        *fresh97 = cur;
        if (unsafe { (*cur).flags }) & (XML_PATTERN_XSSEL as i32 | XML_PATTERN_XSFIELD as i32) != 0 {
            xmlCompileIDCXPathPath(ctxt);
        } else {
            xmlCompilePathPattern(ctxt);
        }
        if (unsafe { (*ctxt).error }) != 0 as i32 {
            current_block = 13522574393598791978;
            break;
        }
        xmlFreePatParserContext(ctxt);
        ctxt = 0 as xmlPatParserContextPtr;
        if streamable != 0 {
            if type_0 == 0 as i32 {
                type_0 = (unsafe { (*cur).flags }) & ((1 as i32) << 8 as i32 | (1 as i32) << 9 as i32);
            } else if type_0 == (1 as i32) << 8 as i32 {
                if (unsafe { (*cur).flags }) & (1 as i32) << 9 as i32 != 0 {
                    streamable = 0 as i32;
                }
            } else if type_0 == (1 as i32) << 9 as i32 {
                if (unsafe { (*cur).flags }) & (1 as i32) << 8 as i32 != 0 {
                    streamable = 0 as i32;
                }
            }
        }
        if streamable != 0 {
            xmlStreamCompile(cur);
        }
        if xmlReversePattern(cur) < 0 as i32 {
            current_block = 13522574393598791978;
            break;
        }
        if !tmp.is_null() {
            (unsafe { xmlFree.expect("non-null function pointer")(tmp as *mut libc::c_void) });
            tmp = 0 as *mut xmlChar;
        }
        start = or;
    }
    match current_block {
        13522574393598791978 => {
            if !ctxt.is_null() {
                xmlFreePatParserContext(ctxt);
            }
            if !ret.is_null() {
                xmlFreePattern(ret);
            }
            if !tmp.is_null() {
                (unsafe { xmlFree.expect("non-null function pointer")(tmp as *mut libc::c_void) });
            }
            return 0 as xmlPatternPtr;
        },
        _ => {
            if streamable == 0 as i32 {
                cur = ret;
                while !cur.is_null() {
                    if !(unsafe { (*cur).stream }).is_null() {
                        xmlFreeStreamComp(unsafe { (*cur).stream });
                        let fresh98 = unsafe { &mut ((*cur).stream) };
                        *fresh98 = 0 as xmlStreamCompPtr;
                    }
                    cur = unsafe { (*cur).next };
                }
            }
            return ret;
        },
    };
}
#[no_mangle]
pub extern "C" fn xmlPatternMatch<'a1>(
    mut comp: *mut crate::src::pattern::_xmlPattern<'a1>,
    mut node: *mut crate::src::HTMLparser::_xmlNode,
) -> i32 {
    let mut ret: i32 = 0 as i32;
    if comp.is_null() || node.is_null() {
        return -(1 as i32);
    }
    while !comp.is_null() {
        ret = xmlPatMatch(comp, node);
        if ret != 0 as i32 {
            return ret;
        }
        comp = unsafe { (*comp).next };
    }
    return ret;
}
#[no_mangle]
pub extern "C" fn xmlPatternGetStreamCtxt<'a1>(
    mut comp: *mut crate::src::pattern::_xmlPattern<'a1>,
) -> *mut crate::src::pattern::_xmlStreamCtxt {
    let mut current_block: u64;
    let mut ret: *mut crate::src::pattern::_xmlStreamCtxt = 0 as xmlStreamCtxtPtr;
    let mut cur: *mut crate::src::pattern::_xmlStreamCtxt = 0 as *mut xmlStreamCtxt;
    if comp.is_null() || (unsafe { (*comp).stream }).is_null() {
        return 0 as xmlStreamCtxtPtr;
    }
    loop {
        if comp.is_null() {
            current_block = 11050875288958768710;
            break;
        }
        if (unsafe { (*comp).stream }).is_null() {
            current_block = 11925268974377416611;
            break;
        }
        cur = xmlNewStreamCtxt(unsafe { (*comp).stream });
        if cur.is_null() {
            current_block = 11925268974377416611;
            break;
        }
        if ret.is_null() {
            ret = cur;
        } else {
            let fresh99 = unsafe { &mut ((*cur).next) };
            *fresh99 = unsafe { (*ret).next };
            let fresh100 = unsafe { &mut ((*ret).next) };
            *fresh100 = cur;
        }
        (unsafe { (*cur).flags = (*comp).flags });
        comp = unsafe { (*comp).next };
    }
    match current_block {
        11050875288958768710 => return ret,
        _ => {
            xmlFreeStreamCtxt(ret);
            return 0 as xmlStreamCtxtPtr;
        },
    };
}
#[no_mangle]
pub extern "C" fn xmlPatternStreamable<'a1>(
    mut comp: *mut crate::src::pattern::_xmlPattern<'a1>,
) -> i32 {
    if comp.is_null() {
        return -(1 as i32);
    }
    while !comp.is_null() {
        if (unsafe { (*comp).stream }).is_null() {
            return 0 as i32;
        }
        comp = unsafe { (*comp).next };
    }
    return 1 as i32;
}
#[no_mangle]
pub extern "C" fn xmlPatternMaxDepth<'a1>(
    mut comp: *mut crate::src::pattern::_xmlPattern<'a1>,
) -> i32 {
    let mut ret: i32 = 0 as i32;
    let mut i: i32 = 0;
    if comp.is_null() {
        return -(1 as i32);
    }
    while !comp.is_null() {
        if (unsafe { (*comp).stream }).is_null() {
            return -(1 as i32);
        }
        i = 0 as i32;
        while i < (unsafe { (*(*comp).stream).nbStep }) {
            if (unsafe { (*((*(*comp).stream).steps).offset(i as isize)).flags }) & 1 as i32 != 0 {
                return -(2 as i32);
            }
            i += 1;
        }
        if (unsafe { (*(*comp).stream).nbStep }) > ret {
            ret = unsafe { (*(*comp).stream).nbStep };
        }
        comp = unsafe { (*comp).next };
    }
    return ret;
}
#[no_mangle]
pub extern "C" fn xmlPatternMinDepth<'a1>(
    mut comp: *mut crate::src::pattern::_xmlPattern<'a1>,
) -> i32 {
    let mut ret: i32 = 12345678 as i32;
    if comp.is_null() {
        return -(1 as i32);
    }
    while !comp.is_null() {
        if (unsafe { (*comp).stream }).is_null() {
            return -(1 as i32);
        }
        if (unsafe { (*(*comp).stream).nbStep }) < ret {
            ret = unsafe { (*(*comp).stream).nbStep };
        }
        if ret == 0 as i32 {
            return 0 as i32;
        }
        comp = unsafe { (*comp).next };
    }
    return ret;
}
#[no_mangle]
pub extern "C" fn xmlPatternFromRoot<'a1>(
    mut comp: *mut crate::src::pattern::_xmlPattern<'a1>,
) -> i32 {
    if comp.is_null() {
        return -(1 as i32);
    }
    while !comp.is_null() {
        if (unsafe { (*comp).stream }).is_null() {
            return -(1 as i32);
        }
        if (unsafe { (*comp).flags }) & (1 as i32) << 8 as i32 != 0 {
            return 1 as i32;
        }
        comp = unsafe { (*comp).next };
    }
    return 0 as i32;
}
use crate::laertes_rt::*;
