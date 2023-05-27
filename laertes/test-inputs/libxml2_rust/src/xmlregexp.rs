use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _xmlBuf;
    pub type _xmlDict;
    pub type _xmlHashTable;
    pub type _xmlStartTag;
    pub type _xmlValidState;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn xmlStrEqual(str1: *const xmlChar, str2: *const xmlChar) -> libc::c_int;
    fn xmlStrchr(str: *const xmlChar, val: xmlChar) -> *const xmlChar;
    fn xmlStrndup(cur: *const xmlChar, len: libc::c_int) -> *mut xmlChar;
    fn xmlStrdup(cur: *const xmlChar) -> *mut xmlChar;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static mut xmlFree: xmlFreeFunc;
    fn __xmlRaiseError(
        schannel: xmlStructuredErrorFunc,
        channel: xmlGenericErrorFunc,
        data: *mut libc::c_void,
        ctx: *mut libc::c_void,
        node: *mut libc::c_void,
        domain: libc::c_int,
        code: libc::c_int,
        level: xmlErrorLevel,
        file: *const libc::c_char,
        line: libc::c_int,
        str1: *const libc::c_char,
        str2: *const libc::c_char,
        str3: *const libc::c_char,
        int1: libc::c_int,
        col: libc::c_int,
        msg: *const libc::c_char,
        _: ...
    );
    static mut xmlMalloc: xmlMallocFunc;
    static mut xmlRealloc: xmlReallocFunc;
    static mut xmlMallocAtomic: xmlMallocFunc;
    fn __xmlGenericErrorContext() -> *mut *mut libc::c_void;
    fn __xmlGenericError() -> *mut xmlGenericErrorFunc;
    fn xmlCharInRange(val: libc::c_uint, group: *const xmlChRangeGroup) -> libc::c_int;
    static xmlIsBaseCharGroup: xmlChRangeGroup;
    static xmlIsCombiningGroup: xmlChRangeGroup;
    static xmlIsDigitGroup: xmlChRangeGroup;
    static xmlIsExtenderGroup: xmlChRangeGroup;
    fn xmlStringCurrentChar(
        ctxt: xmlParserCtxtPtr,
        cur: *const xmlChar,
        len: *mut libc::c_int,
    ) -> libc::c_int;
    fn xmlUCSIsBlock(code: libc::c_int, block: *const libc::c_char) -> libc::c_int;
    fn xmlUCSIsCatC(code: libc::c_int) -> libc::c_int;
    fn xmlUCSIsCatCc(code: libc::c_int) -> libc::c_int;
    fn xmlUCSIsCatCf(code: libc::c_int) -> libc::c_int;
    fn xmlUCSIsCatCo(code: libc::c_int) -> libc::c_int;
    fn xmlUCSIsCatL(code: libc::c_int) -> libc::c_int;
    fn xmlUCSIsCatLl(code: libc::c_int) -> libc::c_int;
    fn xmlUCSIsCatLm(code: libc::c_int) -> libc::c_int;
    fn xmlUCSIsCatLo(code: libc::c_int) -> libc::c_int;
    fn xmlUCSIsCatLt(code: libc::c_int) -> libc::c_int;
    fn xmlUCSIsCatLu(code: libc::c_int) -> libc::c_int;
    fn xmlUCSIsCatM(code: libc::c_int) -> libc::c_int;
    fn xmlUCSIsCatMc(code: libc::c_int) -> libc::c_int;
    fn xmlUCSIsCatMe(code: libc::c_int) -> libc::c_int;
    fn xmlUCSIsCatMn(code: libc::c_int) -> libc::c_int;
    fn xmlUCSIsCatN(code: libc::c_int) -> libc::c_int;
    fn xmlUCSIsCatNd(code: libc::c_int) -> libc::c_int;
    fn xmlUCSIsCatNl(code: libc::c_int) -> libc::c_int;
    fn xmlUCSIsCatNo(code: libc::c_int) -> libc::c_int;
    fn xmlUCSIsCatP(code: libc::c_int) -> libc::c_int;
    fn xmlUCSIsCatPc(code: libc::c_int) -> libc::c_int;
    fn xmlUCSIsCatPd(code: libc::c_int) -> libc::c_int;
    fn xmlUCSIsCatPe(code: libc::c_int) -> libc::c_int;
    fn xmlUCSIsCatPf(code: libc::c_int) -> libc::c_int;
    fn xmlUCSIsCatPi(code: libc::c_int) -> libc::c_int;
    fn xmlUCSIsCatPo(code: libc::c_int) -> libc::c_int;
    fn xmlUCSIsCatPs(code: libc::c_int) -> libc::c_int;
    fn xmlUCSIsCatS(code: libc::c_int) -> libc::c_int;
    fn xmlUCSIsCatSc(code: libc::c_int) -> libc::c_int;
    fn xmlUCSIsCatSk(code: libc::c_int) -> libc::c_int;
    fn xmlUCSIsCatSm(code: libc::c_int) -> libc::c_int;
    fn xmlUCSIsCatSo(code: libc::c_int) -> libc::c_int;
    fn xmlUCSIsCatZ(code: libc::c_int) -> libc::c_int;
    fn xmlUCSIsCatZl(code: libc::c_int) -> libc::c_int;
    fn xmlUCSIsCatZp(code: libc::c_int) -> libc::c_int;
    fn xmlUCSIsCatZs(code: libc::c_int) -> libc::c_int;
}
pub type xmlChar = libc::c_uchar;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlParserInputBuffer {
    pub context: *mut libc::c_void,
    pub readcallback: xmlInputReadCallback,
    pub closecallback: xmlInputCloseCallback,
    pub encoder: xmlCharEncodingHandlerPtr,
    pub buffer: xmlBufPtr,
    pub raw: xmlBufPtr,
    pub compressed: libc::c_int,
    pub error: libc::c_int,
    pub rawconsumed: libc::c_ulong,
}
pub type xmlBufPtr = *mut xmlBuf;
pub type xmlBuf = _xmlBuf;
pub type xmlCharEncodingHandlerPtr = *mut xmlCharEncodingHandler;
pub type xmlCharEncodingHandler = _xmlCharEncodingHandler;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlCharEncodingHandler {
    pub name: *mut libc::c_char,
    pub input: xmlCharEncodingInputFunc,
    pub output: xmlCharEncodingOutputFunc,
    pub iconv_in: iconv_t,
    pub iconv_out: iconv_t,
}
pub type iconv_t = *mut libc::c_void;
pub type xmlCharEncodingOutputFunc = Option::<
    unsafe extern "C" fn(
        *mut libc::c_uchar,
        *mut libc::c_int,
        *const libc::c_uchar,
        *mut libc::c_int,
    ) -> libc::c_int,
>;
pub type xmlCharEncodingInputFunc = Option::<
    unsafe extern "C" fn(
        *mut libc::c_uchar,
        *mut libc::c_int,
        *const libc::c_uchar,
        *mut libc::c_int,
    ) -> libc::c_int,
>;
pub type xmlInputCloseCallback = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
>;
pub type xmlInputReadCallback = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *mut libc::c_char,
        libc::c_int,
    ) -> libc::c_int,
>;
pub type xmlParserInputBuffer = _xmlParserInputBuffer;
pub type xmlParserInputBufferPtr = *mut xmlParserInputBuffer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlParserInput {
    pub buf: xmlParserInputBufferPtr,
    pub filename: *const libc::c_char,
    pub directory: *const libc::c_char,
    pub base: *const xmlChar,
    pub cur: *const xmlChar,
    pub end: *const xmlChar,
    pub length: libc::c_int,
    pub line: libc::c_int,
    pub col: libc::c_int,
    pub consumed: libc::c_ulong,
    pub free: xmlParserInputDeallocate,
    pub encoding: *const xmlChar,
    pub version: *const xmlChar,
    pub standalone: libc::c_int,
    pub id: libc::c_int,
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
    pub wellFormed: libc::c_int,
    pub replaceEntities: libc::c_int,
    pub version: *const xmlChar,
    pub encoding: *const xmlChar,
    pub standalone: libc::c_int,
    pub html: libc::c_int,
    pub input: xmlParserInputPtr,
    pub inputNr: libc::c_int,
    pub inputMax: libc::c_int,
    pub inputTab: *mut xmlParserInputPtr,
    pub node: xmlNodePtr,
    pub nodeNr: libc::c_int,
    pub nodeMax: libc::c_int,
    pub nodeTab: *mut xmlNodePtr,
    pub record_info: libc::c_int,
    pub node_seq: xmlParserNodeInfoSeq,
    pub errNo: libc::c_int,
    pub hasExternalSubset: libc::c_int,
    pub hasPErefs: libc::c_int,
    pub external: libc::c_int,
    pub valid: libc::c_int,
    pub validate: libc::c_int,
    pub vctxt: xmlValidCtxt,
    pub instate: xmlParserInputState,
    pub token: libc::c_int,
    pub directory: *mut libc::c_char,
    pub name: *const xmlChar,
    pub nameNr: libc::c_int,
    pub nameMax: libc::c_int,
    pub nameTab: *mut *const xmlChar,
    pub nbChars: libc::c_long,
    pub checkIndex: libc::c_long,
    pub keepBlanks: libc::c_int,
    pub disableSAX: libc::c_int,
    pub inSubset: libc::c_int,
    pub intSubName: *const xmlChar,
    pub extSubURI: *mut xmlChar,
    pub extSubSystem: *mut xmlChar,
    pub space: *mut libc::c_int,
    pub spaceNr: libc::c_int,
    pub spaceMax: libc::c_int,
    pub spaceTab: *mut libc::c_int,
    pub depth: libc::c_int,
    pub entity: xmlParserInputPtr,
    pub charset: libc::c_int,
    pub nodelen: libc::c_int,
    pub nodemem: libc::c_int,
    pub pedantic: libc::c_int,
    pub _private: *mut libc::c_void,
    pub loadsubset: libc::c_int,
    pub linenumbers: libc::c_int,
    pub catalogs: *mut libc::c_void,
    pub recovery: libc::c_int,
    pub progressive: libc::c_int,
    pub dict: xmlDictPtr,
    pub atts: *mut *const xmlChar,
    pub maxatts: libc::c_int,
    pub docdict: libc::c_int,
    pub str_xml: *const xmlChar,
    pub str_xmlns: *const xmlChar,
    pub str_xml_ns: *const xmlChar,
    pub sax2: libc::c_int,
    pub nsNr: libc::c_int,
    pub nsMax: libc::c_int,
    pub nsTab: *mut *const xmlChar,
    pub attallocs: *mut libc::c_int,
    pub pushTab: *mut xmlStartTag,
    pub attsDefault: xmlHashTablePtr,
    pub attsSpecial: xmlHashTablePtr,
    pub nsWellFormed: libc::c_int,
    pub options: libc::c_int,
    pub dictNames: libc::c_int,
    pub freeElemsNr: libc::c_int,
    pub freeElems: xmlNodePtr,
    pub freeAttrsNr: libc::c_int,
    pub freeAttrs: xmlAttrPtr,
    pub lastError: xmlError,
    pub parseMode: xmlParserMode,
    pub nbentities: libc::c_ulong,
    pub sizeentities: libc::c_ulong,
    pub nodeInfo: *mut xmlParserNodeInfo,
    pub nodeInfoNr: libc::c_int,
    pub nodeInfoMax: libc::c_int,
    pub nodeInfoTab: *mut xmlParserNodeInfo,
    pub input_id: libc::c_int,
    pub sizeentcopy: libc::c_ulong,
}
pub type xmlParserNodeInfo = _xmlParserNodeInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlParserNodeInfo {
    pub node: *const _xmlNode,
    pub begin_pos: libc::c_ulong,
    pub begin_line: libc::c_ulong,
    pub end_pos: libc::c_ulong,
    pub end_line: libc::c_ulong,
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
    pub line: libc::c_ushort,
    pub extra: libc::c_ushort,
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
    pub name: *mut libc::c_char,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlNode,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub compression: libc::c_int,
    pub standalone: libc::c_int,
    pub intSubset: *mut _xmlDtd,
    pub extSubset: *mut _xmlDtd,
    pub oldNs: *mut _xmlNs,
    pub version: *const xmlChar,
    pub encoding: *const xmlChar,
    pub ids: *mut libc::c_void,
    pub refs: *mut libc::c_void,
    pub URL: *const xmlChar,
    pub charset: libc::c_int,
    pub dict: *mut _xmlDict,
    pub psvi: *mut libc::c_void,
    pub parseFlags: libc::c_int,
    pub properties: libc::c_int,
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
pub type xmlElementType = libc::c_uint;
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
pub type xmlAttributeType = libc::c_uint;
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
pub type xmlParserMode = libc::c_uint;
pub const XML_PARSE_READER: xmlParserMode = 5;
pub const XML_PARSE_PUSH_SAX: xmlParserMode = 4;
pub const XML_PARSE_PUSH_DOM: xmlParserMode = 3;
pub const XML_PARSE_SAX: xmlParserMode = 2;
pub const XML_PARSE_DOM: xmlParserMode = 1;
pub const XML_PARSE_UNKNOWN: xmlParserMode = 0;
pub type xmlError = _xmlError;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlError {
    pub domain: libc::c_int,
    pub code: libc::c_int,
    pub message: *mut libc::c_char,
    pub level: xmlErrorLevel,
    pub file: *mut libc::c_char,
    pub line: libc::c_int,
    pub str1: *mut libc::c_char,
    pub str2: *mut libc::c_char,
    pub str3: *mut libc::c_char,
    pub int1: libc::c_int,
    pub int2: libc::c_int,
    pub ctxt: *mut libc::c_void,
    pub node: *mut libc::c_void,
}
pub type xmlErrorLevel = libc::c_uint;
pub const XML_ERR_FATAL: xmlErrorLevel = 3;
pub const XML_ERR_ERROR: xmlErrorLevel = 2;
pub const XML_ERR_WARNING: xmlErrorLevel = 1;
pub const XML_ERR_NONE: xmlErrorLevel = 0;
pub type xmlAttrPtr = *mut xmlAttr;
pub type xmlAttr = _xmlAttr;
pub type xmlNodePtr = *mut xmlNode;
pub type xmlNode = _xmlNode;
pub type xmlHashTablePtr = *mut xmlHashTable;
pub type xmlHashTable = _xmlHashTable;
pub type xmlStartTag = _xmlStartTag;
pub type xmlDictPtr = *mut xmlDict;
pub type xmlDict = _xmlDict;
pub type xmlParserInputState = libc::c_int;
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
pub type xmlValidCtxt = _xmlValidCtxt;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlValidCtxt {
    pub userData: *mut libc::c_void,
    pub error: xmlValidityErrorFunc,
    pub warning: xmlValidityWarningFunc,
    pub node: xmlNodePtr,
    pub nodeNr: libc::c_int,
    pub nodeMax: libc::c_int,
    pub nodeTab: *mut xmlNodePtr,
    pub flags: libc::c_uint,
    pub doc: xmlDocPtr,
    pub valid: libc::c_int,
    pub vstate: *mut xmlValidState,
    pub vstateNr: libc::c_int,
    pub vstateMax: libc::c_int,
    pub vstateTab: *mut xmlValidState,
    pub am: xmlAutomataPtr,
    pub state: xmlAutomataStatePtr,
}
pub type xmlAutomataStatePtr = *mut xmlAutomataState;
pub type xmlAutomataState = _xmlAutomataState;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlAutomataState {
    pub type_0: xmlRegStateType,
    pub mark: xmlRegMarkedType,
    pub markd: xmlRegMarkedType,
    pub reached: xmlRegMarkedType,
    pub no: libc::c_int,
    pub maxTrans: libc::c_int,
    pub nbTrans: libc::c_int,
    pub trans: *mut xmlRegTrans,
    pub maxTransTo: libc::c_int,
    pub nbTransTo: libc::c_int,
    pub transTo: *mut libc::c_int,
}
pub type xmlRegTrans = _xmlRegTrans;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlRegTrans {
    pub atom: xmlRegAtomPtr,
    pub to: libc::c_int,
    pub counter: libc::c_int,
    pub count: libc::c_int,
    pub nd: libc::c_int,
}
pub type xmlRegAtomPtr = *mut xmlRegAtom;
pub type xmlRegAtom = _xmlRegAtom;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlRegAtom {
    pub no: libc::c_int,
    pub type_0: xmlRegAtomType,
    pub quant: xmlRegQuantType,
    pub min: libc::c_int,
    pub max: libc::c_int,
    pub valuep: *mut libc::c_void,
    pub valuep2: *mut libc::c_void,
    pub neg: libc::c_int,
    pub codepoint: libc::c_int,
    pub start: xmlRegStatePtr,
    pub start0: xmlRegStatePtr,
    pub stop: xmlRegStatePtr,
    pub maxRanges: libc::c_int,
    pub nbRanges: libc::c_int,
    pub ranges: *mut xmlRegRangePtr,
    pub data: *mut libc::c_void,
}
pub type xmlRegRangePtr = *mut xmlRegRange;
pub type xmlRegRange = _xmlRegRange;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlRegRange {
    pub neg: libc::c_int,
    pub type_0: xmlRegAtomType,
    pub start: libc::c_int,
    pub end: libc::c_int,
    pub blockName: *mut xmlChar,
}
pub type xmlRegAtomType = libc::c_uint;
pub const XML_REGEXP_BLOCK_NAME: xmlRegAtomType = 136;
pub const XML_REGEXP_OTHER_NA: xmlRegAtomType = 135;
pub const XML_REGEXP_OTHER_PRIVATE: xmlRegAtomType = 134;
pub const XML_REGEXP_OTHER_FORMAT: xmlRegAtomType = 133;
pub const XML_REGEXP_OTHER_CONTROL: xmlRegAtomType = 132;
pub const XML_REGEXP_OTHER: xmlRegAtomType = 131;
pub const XML_REGEXP_SYMBOL_OTHERS: xmlRegAtomType = 130;
pub const XML_REGEXP_SYMBOL_MODIFIER: xmlRegAtomType = 129;
pub const XML_REGEXP_SYMBOL_CURRENCY: xmlRegAtomType = 128;
pub const XML_REGEXP_SYMBOL_MATH: xmlRegAtomType = 127;
pub const XML_REGEXP_SYMBOL: xmlRegAtomType = 126;
pub const XML_REGEXP_SEPAR_PARA: xmlRegAtomType = 125;
pub const XML_REGEXP_SEPAR_LINE: xmlRegAtomType = 124;
pub const XML_REGEXP_SEPAR_SPACE: xmlRegAtomType = 123;
pub const XML_REGEXP_SEPAR: xmlRegAtomType = 122;
pub const XML_REGEXP_PUNCT_OTHERS: xmlRegAtomType = 121;
pub const XML_REGEXP_PUNCT_FINQUOTE: xmlRegAtomType = 120;
pub const XML_REGEXP_PUNCT_INITQUOTE: xmlRegAtomType = 119;
pub const XML_REGEXP_PUNCT_CLOSE: xmlRegAtomType = 118;
pub const XML_REGEXP_PUNCT_OPEN: xmlRegAtomType = 117;
pub const XML_REGEXP_PUNCT_DASH: xmlRegAtomType = 116;
pub const XML_REGEXP_PUNCT_CONNECTOR: xmlRegAtomType = 115;
pub const XML_REGEXP_PUNCT: xmlRegAtomType = 114;
pub const XML_REGEXP_NUMBER_OTHERS: xmlRegAtomType = 113;
pub const XML_REGEXP_NUMBER_LETTER: xmlRegAtomType = 112;
pub const XML_REGEXP_NUMBER_DECIMAL: xmlRegAtomType = 111;
pub const XML_REGEXP_NUMBER: xmlRegAtomType = 110;
pub const XML_REGEXP_MARK_ENCLOSING: xmlRegAtomType = 109;
pub const XML_REGEXP_MARK_SPACECOMBINING: xmlRegAtomType = 108;
pub const XML_REGEXP_MARK_NONSPACING: xmlRegAtomType = 107;
pub const XML_REGEXP_MARK: xmlRegAtomType = 106;
pub const XML_REGEXP_LETTER_OTHERS: xmlRegAtomType = 105;
pub const XML_REGEXP_LETTER_MODIFIER: xmlRegAtomType = 104;
pub const XML_REGEXP_LETTER_TITLECASE: xmlRegAtomType = 103;
pub const XML_REGEXP_LETTER_LOWERCASE: xmlRegAtomType = 102;
pub const XML_REGEXP_LETTER_UPPERCASE: xmlRegAtomType = 101;
pub const XML_REGEXP_LETTER: xmlRegAtomType = 100;
pub const XML_REGEXP_NOTREALCHAR: xmlRegAtomType = 16;
pub const XML_REGEXP_REALCHAR: xmlRegAtomType = 15;
pub const XML_REGEXP_NOTDECIMAL: xmlRegAtomType = 14;
pub const XML_REGEXP_DECIMAL: xmlRegAtomType = 13;
pub const XML_REGEXP_NOTNAMECHAR: xmlRegAtomType = 12;
pub const XML_REGEXP_NAMECHAR: xmlRegAtomType = 11;
pub const XML_REGEXP_NOTINITNAME: xmlRegAtomType = 10;
pub const XML_REGEXP_INITNAME: xmlRegAtomType = 9;
pub const XML_REGEXP_NOTSPACE: xmlRegAtomType = 8;
pub const XML_REGEXP_ANYSPACE: xmlRegAtomType = 7;
pub const XML_REGEXP_ANYCHAR: xmlRegAtomType = 6;
pub const XML_REGEXP_STRING: xmlRegAtomType = 5;
pub const XML_REGEXP_SUBREG: xmlRegAtomType = 4;
pub const XML_REGEXP_RANGES: xmlRegAtomType = 3;
pub const XML_REGEXP_CHARVAL: xmlRegAtomType = 2;
pub const XML_REGEXP_EPSILON: xmlRegAtomType = 1;
pub type xmlRegStatePtr = *mut xmlRegState;
pub type xmlRegState = _xmlAutomataState;
pub type xmlRegQuantType = libc::c_uint;
pub const XML_REGEXP_QUANT_RANGE: xmlRegQuantType = 8;
pub const XML_REGEXP_QUANT_ALL: xmlRegQuantType = 7;
pub const XML_REGEXP_QUANT_ONCEONLY: xmlRegQuantType = 6;
pub const XML_REGEXP_QUANT_PLUS: xmlRegQuantType = 5;
pub const XML_REGEXP_QUANT_MULT: xmlRegQuantType = 4;
pub const XML_REGEXP_QUANT_OPT: xmlRegQuantType = 3;
pub const XML_REGEXP_QUANT_ONCE: xmlRegQuantType = 2;
pub const XML_REGEXP_QUANT_EPSILON: xmlRegQuantType = 1;
pub type xmlRegMarkedType = libc::c_uint;
pub const XML_REGEXP_MARK_VISITED: xmlRegMarkedType = 2;
pub const XML_REGEXP_MARK_START: xmlRegMarkedType = 1;
pub const XML_REGEXP_MARK_NORMAL: xmlRegMarkedType = 0;
pub type xmlRegStateType = libc::c_uint;
pub const XML_REGEXP_UNREACH_STATE: xmlRegStateType = 5;
pub const XML_REGEXP_SINK_STATE: xmlRegStateType = 4;
pub const XML_REGEXP_TRANS_STATE: xmlRegStateType = 3;
pub const XML_REGEXP_FINAL_STATE: xmlRegStateType = 2;
pub const XML_REGEXP_START_STATE: xmlRegStateType = 1;
pub type xmlAutomataPtr = *mut xmlAutomata;
pub type xmlAutomata = _xmlAutomata;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlAutomata {
    pub string: *mut xmlChar,
    pub cur: *mut xmlChar,
    pub error: libc::c_int,
    pub neg: libc::c_int,
    pub start: xmlRegStatePtr,
    pub end: xmlRegStatePtr,
    pub state: xmlRegStatePtr,
    pub atom: xmlRegAtomPtr,
    pub maxAtoms: libc::c_int,
    pub nbAtoms: libc::c_int,
    pub atoms: *mut xmlRegAtomPtr,
    pub maxStates: libc::c_int,
    pub nbStates: libc::c_int,
    pub states: *mut xmlRegStatePtr,
    pub maxCounters: libc::c_int,
    pub nbCounters: libc::c_int,
    pub counters: *mut xmlRegCounter,
    pub determinist: libc::c_int,
    pub negs: libc::c_int,
    pub flags: libc::c_int,
    pub depth: libc::c_int,
}
pub type xmlRegCounter = _xmlRegCounter;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlRegCounter {
    pub min: libc::c_int,
    pub max: libc::c_int,
}
pub type xmlValidState = _xmlValidState;
pub type xmlDocPtr = *mut xmlDoc;
pub type xmlDoc = _xmlDoc;
pub type xmlValidityWarningFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, ...) -> (),
>;
pub type xmlValidityErrorFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, ...) -> (),
>;
pub type xmlParserNodeInfoSeq = _xmlParserNodeInfoSeq;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlParserNodeInfoSeq {
    pub maximum: libc::c_ulong,
    pub length: libc::c_ulong,
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
    pub initialized: libc::c_uint,
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
        libc::c_int,
        *mut *const xmlChar,
        libc::c_int,
        libc::c_int,
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
    unsafe extern "C" fn(*mut libc::c_void, *const xmlChar, libc::c_int) -> (),
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
    pub length: libc::c_int,
    pub etype: xmlEntityType,
    pub ExternalID: *const xmlChar,
    pub SystemID: *const xmlChar,
    pub nexte: *mut _xmlEntity,
    pub URI: *const xmlChar,
    pub owner: libc::c_int,
    pub checked: libc::c_int,
}
pub type xmlEntityType = libc::c_uint;
pub const XML_INTERNAL_PREDEFINED_ENTITY: xmlEntityType = 6;
pub const XML_EXTERNAL_PARAMETER_ENTITY: xmlEntityType = 5;
pub const XML_INTERNAL_PARAMETER_ENTITY: xmlEntityType = 4;
pub const XML_EXTERNAL_GENERAL_UNPARSED_ENTITY: xmlEntityType = 3;
pub const XML_EXTERNAL_GENERAL_PARSED_ENTITY: xmlEntityType = 2;
pub const XML_INTERNAL_GENERAL_ENTITY: xmlEntityType = 1;
pub type fatalErrorSAXFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, ...) -> (),
>;
pub type errorSAXFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, ...) -> (),
>;
pub type warningSAXFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, ...) -> (),
>;
pub type commentSAXFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const xmlChar) -> (),
>;
pub type processingInstructionSAXFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const xmlChar, *const xmlChar) -> (),
>;
pub type ignorableWhitespaceSAXFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const xmlChar, libc::c_int) -> (),
>;
pub type charactersSAXFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const xmlChar, libc::c_int) -> (),
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
    pub getLineNumber: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    pub getColumnNumber: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
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
        libc::c_int,
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
pub type xmlElementContentOccur = libc::c_uint;
pub const XML_ELEMENT_CONTENT_PLUS: xmlElementContentOccur = 4;
pub const XML_ELEMENT_CONTENT_MULT: xmlElementContentOccur = 3;
pub const XML_ELEMENT_CONTENT_OPT: xmlElementContentOccur = 2;
pub const XML_ELEMENT_CONTENT_ONCE: xmlElementContentOccur = 1;
pub type xmlElementContentType = libc::c_uint;
pub const XML_ELEMENT_CONTENT_OR: xmlElementContentType = 4;
pub const XML_ELEMENT_CONTENT_SEQ: xmlElementContentType = 3;
pub const XML_ELEMENT_CONTENT_ELEMENT: xmlElementContentType = 2;
pub const XML_ELEMENT_CONTENT_PCDATA: xmlElementContentType = 1;
pub type attributeDeclSAXFunc = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const xmlChar,
        *const xmlChar,
        libc::c_int,
        libc::c_int,
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
        libc::c_int,
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
    unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
>;
pub type hasInternalSubsetSAXFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
>;
pub type isStandaloneSAXFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlRegexp {
    pub string: *mut xmlChar,
    pub nbStates: libc::c_int,
    pub states: *mut xmlRegStatePtr,
    pub nbAtoms: libc::c_int,
    pub atoms: *mut xmlRegAtomPtr,
    pub nbCounters: libc::c_int,
    pub counters: *mut xmlRegCounter,
    pub determinist: libc::c_int,
    pub flags: libc::c_int,
    pub nbstates: libc::c_int,
    pub compact: *mut libc::c_int,
    pub transdata: *mut *mut libc::c_void,
    pub nbstrings: libc::c_int,
    pub stringMap: *mut *mut xmlChar,
}
pub type xmlRegexp = _xmlRegexp;
pub type xmlRegexpPtr = *mut xmlRegexp;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlRegExecCtxt {
    pub status: libc::c_int,
    pub determinist: libc::c_int,
    pub comp: xmlRegexpPtr,
    pub callback: xmlRegExecCallbacks,
    pub data: *mut libc::c_void,
    pub state: xmlRegStatePtr,
    pub transno: libc::c_int,
    pub transcount: libc::c_int,
    pub maxRollbacks: libc::c_int,
    pub nbRollbacks: libc::c_int,
    pub rollbacks: *mut xmlRegExecRollback,
    pub counts: *mut libc::c_int,
    pub inputStackMax: libc::c_int,
    pub inputStackNr: libc::c_int,
    pub index: libc::c_int,
    pub charStack: *mut libc::c_int,
    pub inputString: *const xmlChar,
    pub inputStack: xmlRegInputTokenPtr,
    pub errStateNo: libc::c_int,
    pub errState: xmlRegStatePtr,
    pub errString: *mut xmlChar,
    pub errCounts: *mut libc::c_int,
    pub nbPush: libc::c_int,
}
pub type xmlRegInputTokenPtr = *mut xmlRegInputToken;
pub type xmlRegInputToken = _xmlRegInputToken;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlRegInputToken {
    pub value: *mut xmlChar,
    pub data: *mut libc::c_void,
}
pub type xmlRegExecRollback = _xmlRegExecRollback;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlRegExecRollback {
    pub state: xmlRegStatePtr,
    pub index: libc::c_int,
    pub nextbranch: libc::c_int,
    pub counts: *mut libc::c_int,
}
pub type xmlRegExecCallbacks = Option::<
    unsafe extern "C" fn(
        xmlRegExecCtxtPtr,
        *const xmlChar,
        *mut libc::c_void,
        *mut libc::c_void,
    ) -> (),
>;
pub type xmlRegExecCtxtPtr = *mut xmlRegExecCtxt;
pub type xmlRegExecCtxt = _xmlRegExecCtxt;
pub type xmlRegParserCtxtPtr = *mut xmlRegParserCtxt;
pub type xmlRegParserCtxt = _xmlAutomata;
pub type xmlFreeFunc = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type xmlRegTransPtr = *mut xmlRegTrans;
pub const XML_ERR_NO_MEMORY: C2RustUnnamed_0 = 2;
pub const XML_FROM_REGEXP: C2RustUnnamed = 14;
pub type xmlGenericErrorFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, ...) -> (),
>;
pub type xmlMallocFunc = Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>;
pub type xmlChRangeGroup = _xmlChRangeGroup;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlChRangeGroup {
    pub nbShortRange: libc::c_int,
    pub nbLongRange: libc::c_int,
    pub shortRange: *const xmlChSRange,
    pub longRange: *const xmlChLRange,
}
pub type xmlChLRange = _xmlChLRange;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlChLRange {
    pub low: libc::c_uint,
    pub high: libc::c_uint,
}
pub type xmlChSRange = _xmlChSRange;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlChSRange {
    pub low: libc::c_ushort,
    pub high: libc::c_ushort,
}
pub type xmlReallocFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
>;
pub const XML_REGEXP_COMPILE_ERROR: C2RustUnnamed_0 = 1450;
pub type xmlRegCounterPtr = *mut xmlRegCounter;
pub type C2RustUnnamed = libc::c_uint;
pub const XML_FROM_URI: C2RustUnnamed = 30;
pub const XML_FROM_BUFFER: C2RustUnnamed = 29;
pub const XML_FROM_SCHEMATRONV: C2RustUnnamed = 28;
pub const XML_FROM_I18N: C2RustUnnamed = 27;
pub const XML_FROM_MODULE: C2RustUnnamed = 26;
pub const XML_FROM_WRITER: C2RustUnnamed = 25;
pub const XML_FROM_CHECK: C2RustUnnamed = 24;
pub const XML_FROM_VALID: C2RustUnnamed = 23;
pub const XML_FROM_XSLT: C2RustUnnamed = 22;
pub const XML_FROM_C14N: C2RustUnnamed = 21;
pub const XML_FROM_CATALOG: C2RustUnnamed = 20;
pub const XML_FROM_RELAXNGV: C2RustUnnamed = 19;
pub const XML_FROM_RELAXNGP: C2RustUnnamed = 18;
pub const XML_FROM_SCHEMASV: C2RustUnnamed = 17;
pub const XML_FROM_SCHEMASP: C2RustUnnamed = 16;
pub const XML_FROM_DATATYPE: C2RustUnnamed = 15;
pub const XML_FROM_XPOINTER: C2RustUnnamed = 13;
pub const XML_FROM_XPATH: C2RustUnnamed = 12;
pub const XML_FROM_XINCLUDE: C2RustUnnamed = 11;
pub const XML_FROM_HTTP: C2RustUnnamed = 10;
pub const XML_FROM_FTP: C2RustUnnamed = 9;
pub const XML_FROM_IO: C2RustUnnamed = 8;
pub const XML_FROM_OUTPUT: C2RustUnnamed = 7;
pub const XML_FROM_MEMORY: C2RustUnnamed = 6;
pub const XML_FROM_HTML: C2RustUnnamed = 5;
pub const XML_FROM_DTD: C2RustUnnamed = 4;
pub const XML_FROM_NAMESPACE: C2RustUnnamed = 3;
pub const XML_FROM_TREE: C2RustUnnamed = 2;
pub const XML_FROM_PARSER: C2RustUnnamed = 1;
pub const XML_FROM_NONE: C2RustUnnamed = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const XML_BUF_OVERFLOW: C2RustUnnamed_0 = 7000;
pub const XML_I18N_NO_OUTPUT: C2RustUnnamed_0 = 6004;
pub const XML_I18N_CONV_FAILED: C2RustUnnamed_0 = 6003;
pub const XML_I18N_EXCESS_HANDLER: C2RustUnnamed_0 = 6002;
pub const XML_I18N_NO_HANDLER: C2RustUnnamed_0 = 6001;
pub const XML_I18N_NO_NAME: C2RustUnnamed_0 = 6000;
pub const XML_CHECK_NAME_NOT_NULL: C2RustUnnamed_0 = 5037;
pub const XML_CHECK_WRONG_NAME: C2RustUnnamed_0 = 5036;
pub const XML_CHECK_OUTSIDE_DICT: C2RustUnnamed_0 = 5035;
pub const XML_CHECK_NOT_NCNAME: C2RustUnnamed_0 = 5034;
pub const XML_CHECK_NO_DICT: C2RustUnnamed_0 = 5033;
pub const XML_CHECK_NOT_UTF8: C2RustUnnamed_0 = 5032;
pub const XML_CHECK_NS_ANCESTOR: C2RustUnnamed_0 = 5031;
pub const XML_CHECK_NS_SCOPE: C2RustUnnamed_0 = 5030;
pub const XML_CHECK_WRONG_PARENT: C2RustUnnamed_0 = 5029;
pub const XML_CHECK_NO_HREF: C2RustUnnamed_0 = 5028;
pub const XML_CHECK_NOT_NS_DECL: C2RustUnnamed_0 = 5027;
pub const XML_CHECK_NOT_ENTITY_DECL: C2RustUnnamed_0 = 5026;
pub const XML_CHECK_NOT_ELEM_DECL: C2RustUnnamed_0 = 5025;
pub const XML_CHECK_NOT_ATTR_DECL: C2RustUnnamed_0 = 5024;
pub const XML_CHECK_NOT_ATTR: C2RustUnnamed_0 = 5023;
pub const XML_CHECK_NOT_DTD: C2RustUnnamed_0 = 5022;
pub const XML_CHECK_WRONG_NEXT: C2RustUnnamed_0 = 5021;
pub const XML_CHECK_NO_NEXT: C2RustUnnamed_0 = 5020;
pub const XML_CHECK_WRONG_PREV: C2RustUnnamed_0 = 5019;
pub const XML_CHECK_NO_PREV: C2RustUnnamed_0 = 5018;
pub const XML_CHECK_WRONG_DOC: C2RustUnnamed_0 = 5017;
pub const XML_CHECK_NO_ELEM: C2RustUnnamed_0 = 5016;
pub const XML_CHECK_NO_NAME: C2RustUnnamed_0 = 5015;
pub const XML_CHECK_NO_DOC: C2RustUnnamed_0 = 5014;
pub const XML_CHECK_NO_PARENT: C2RustUnnamed_0 = 5013;
pub const XML_CHECK_ENTITY_TYPE: C2RustUnnamed_0 = 5012;
pub const XML_CHECK_UNKNOWN_NODE: C2RustUnnamed_0 = 5011;
pub const XML_CHECK_FOUND_NOTATION: C2RustUnnamed_0 = 5010;
pub const XML_CHECK_FOUND_FRAGMENT: C2RustUnnamed_0 = 5009;
pub const XML_CHECK_FOUND_DOCTYPE: C2RustUnnamed_0 = 5008;
pub const XML_CHECK_FOUND_COMMENT: C2RustUnnamed_0 = 5007;
pub const XML_CHECK_FOUND_PI: C2RustUnnamed_0 = 5006;
pub const XML_CHECK_FOUND_ENTITY: C2RustUnnamed_0 = 5005;
pub const XML_CHECK_FOUND_ENTITYREF: C2RustUnnamed_0 = 5004;
pub const XML_CHECK_FOUND_CDATA: C2RustUnnamed_0 = 5003;
pub const XML_CHECK_FOUND_TEXT: C2RustUnnamed_0 = 5002;
pub const XML_CHECK_FOUND_ATTRIBUTE: C2RustUnnamed_0 = 5001;
pub const XML_CHECK_FOUND_ELEMENT: C2RustUnnamed_0 = 5000;
pub const XML_MODULE_CLOSE: C2RustUnnamed_0 = 4901;
pub const XML_MODULE_OPEN: C2RustUnnamed_0 = 4900;
pub const XML_SCHEMATRONV_REPORT: C2RustUnnamed_0 = 4001;
pub const XML_SCHEMATRONV_ASSERT: C2RustUnnamed_0 = 4000;
pub const XML_SCHEMAP_COS_ALL_LIMITED: C2RustUnnamed_0 = 3091;
pub const XML_SCHEMAP_A_PROPS_CORRECT_3: C2RustUnnamed_0 = 3090;
pub const XML_SCHEMAP_AU_PROPS_CORRECT: C2RustUnnamed_0 = 3089;
pub const XML_SCHEMAP_COS_CT_EXTENDS_1_2: C2RustUnnamed_0 = 3088;
pub const XML_SCHEMAP_AG_PROPS_CORRECT: C2RustUnnamed_0 = 3087;
pub const XML_SCHEMAP_WARN_ATTR_POINTLESS_PROH: C2RustUnnamed_0 = 3086;
pub const XML_SCHEMAP_WARN_ATTR_REDECL_PROH: C2RustUnnamed_0 = 3085;
pub const XML_SCHEMAP_WARN_UNLOCATED_SCHEMA: C2RustUnnamed_0 = 3084;
pub const XML_SCHEMAP_WARN_SKIP_SCHEMA: C2RustUnnamed_0 = 3083;
pub const XML_SCHEMAP_SRC_IMPORT: C2RustUnnamed_0 = 3082;
pub const XML_SCHEMAP_SRC_REDEFINE: C2RustUnnamed_0 = 3081;
pub const XML_SCHEMAP_C_PROPS_CORRECT: C2RustUnnamed_0 = 3080;
pub const XML_SCHEMAP_A_PROPS_CORRECT_2: C2RustUnnamed_0 = 3079;
pub const XML_SCHEMAP_AU_PROPS_CORRECT_2: C2RustUnnamed_0 = 3078;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_3: C2RustUnnamed_0 = 3077;
pub const XML_SCHEMAP_SRC_CT_1: C2RustUnnamed_0 = 3076;
pub const XML_SCHEMAP_MG_PROPS_CORRECT_2: C2RustUnnamed_0 = 3075;
pub const XML_SCHEMAP_MG_PROPS_CORRECT_1: C2RustUnnamed_0 = 3074;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_3: C2RustUnnamed_0 = 3073;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_2: C2RustUnnamed_0 = 3072;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_1: C2RustUnnamed_0 = 3071;
pub const XML_SCHEMAP_NOT_DETERMINISTIC: C2RustUnnamed_0 = 3070;
pub const XML_SCHEMAP_INTERNAL: C2RustUnnamed_0 = 3069;
pub const XML_SCHEMAP_SRC_IMPORT_2_2: C2RustUnnamed_0 = 3068;
pub const XML_SCHEMAP_SRC_IMPORT_2_1: C2RustUnnamed_0 = 3067;
pub const XML_SCHEMAP_SRC_IMPORT_2: C2RustUnnamed_0 = 3066;
pub const XML_SCHEMAP_SRC_IMPORT_1_2: C2RustUnnamed_0 = 3065;
pub const XML_SCHEMAP_SRC_IMPORT_1_1: C2RustUnnamed_0 = 3064;
pub const XML_SCHEMAP_COS_CT_EXTENDS_1_1: C2RustUnnamed_0 = 3063;
pub const XML_SCHEMAP_CVC_SIMPLE_TYPE: C2RustUnnamed_0 = 3062;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_2_2_2: C2RustUnnamed_0 = 3061;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_2_2_1: C2RustUnnamed_0 = 3060;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_2_1: C2RustUnnamed_0 = 3059;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_1: C2RustUnnamed_0 = 3058;
pub const XML_SCHEMAP_NO_XSI: C2RustUnnamed_0 = 3057;
pub const XML_SCHEMAP_NO_XMLNS: C2RustUnnamed_0 = 3056;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_4: C2RustUnnamed_0 = 3055;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_3_2: C2RustUnnamed_0 = 3054;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_3_1: C2RustUnnamed_0 = 3053;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_2: C2RustUnnamed_0 = 3052;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_1: C2RustUnnamed_0 = 3051;
pub const XML_SCHEMAP_SRC_INCLUDE: C2RustUnnamed_0 = 3050;
pub const XML_SCHEMAP_E_PROPS_CORRECT_6: C2RustUnnamed_0 = 3049;
pub const XML_SCHEMAP_E_PROPS_CORRECT_5: C2RustUnnamed_0 = 3048;
pub const XML_SCHEMAP_E_PROPS_CORRECT_4: C2RustUnnamed_0 = 3047;
pub const XML_SCHEMAP_E_PROPS_CORRECT_3: C2RustUnnamed_0 = 3046;
pub const XML_SCHEMAP_E_PROPS_CORRECT_2: C2RustUnnamed_0 = 3045;
pub const XML_SCHEMAP_P_PROPS_CORRECT_2_2: C2RustUnnamed_0 = 3044;
pub const XML_SCHEMAP_P_PROPS_CORRECT_2_1: C2RustUnnamed_0 = 3043;
pub const XML_SCHEMAP_P_PROPS_CORRECT_1: C2RustUnnamed_0 = 3042;
pub const XML_SCHEMAP_SRC_ELEMENT_3: C2RustUnnamed_0 = 3041;
pub const XML_SCHEMAP_SRC_ELEMENT_2_2: C2RustUnnamed_0 = 3040;
pub const XML_SCHEMAP_SRC_ELEMENT_2_1: C2RustUnnamed_0 = 3039;
pub const XML_SCHEMAP_SRC_ELEMENT_1: C2RustUnnamed_0 = 3038;
pub const XML_SCHEMAP_S4S_ATTR_INVALID_VALUE: C2RustUnnamed_0 = 3037;
pub const XML_SCHEMAP_S4S_ATTR_MISSING: C2RustUnnamed_0 = 3036;
pub const XML_SCHEMAP_S4S_ATTR_NOT_ALLOWED: C2RustUnnamed_0 = 3035;
pub const XML_SCHEMAP_S4S_ELEM_MISSING: C2RustUnnamed_0 = 3034;
pub const XML_SCHEMAP_S4S_ELEM_NOT_ALLOWED: C2RustUnnamed_0 = 3033;
pub const XML_SCHEMAP_COS_ST_DERIVED_OK_2_2: C2RustUnnamed_0 = 3032;
pub const XML_SCHEMAP_COS_ST_DERIVED_OK_2_1: C2RustUnnamed_0 = 3031;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_5: C2RustUnnamed_0 = 3030;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_4: C2RustUnnamed_0 = 3029;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_3: C2RustUnnamed_0 = 3028;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_1: C2RustUnnamed_0 = 3027;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_2: C2RustUnnamed_0 = 3026;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_1_2: C2RustUnnamed_0 = 3025;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_1: C2RustUnnamed_0 = 3024;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_1: C2RustUnnamed_0 = 3023;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_5: C2RustUnnamed_0 = 3022;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_4: C2RustUnnamed_0 = 3021;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_3: C2RustUnnamed_0 = 3020;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_2: C2RustUnnamed_0 = 3019;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_1: C2RustUnnamed_0 = 3018;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_1_2: C2RustUnnamed_0 = 3017;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_1_1: C2RustUnnamed_0 = 3016;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_1: C2RustUnnamed_0 = 3015;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_3_2: C2RustUnnamed_0 = 3014;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_3_1: C2RustUnnamed_0 = 3013;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_2: C2RustUnnamed_0 = 3012;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_1: C2RustUnnamed_0 = 3011;
pub const XML_SCHEMAP_ST_PROPS_CORRECT_3: C2RustUnnamed_0 = 3010;
pub const XML_SCHEMAP_ST_PROPS_CORRECT_2: C2RustUnnamed_0 = 3009;
pub const XML_SCHEMAP_ST_PROPS_CORRECT_1: C2RustUnnamed_0 = 3008;
pub const XML_SCHEMAP_SRC_UNION_MEMBERTYPES_OR_SIMPLETYPES: C2RustUnnamed_0 = 3007;
pub const XML_SCHEMAP_SRC_LIST_ITEMTYPE_OR_SIMPLETYPE: C2RustUnnamed_0 = 3006;
pub const XML_SCHEMAP_SRC_RESTRICTION_BASE_OR_SIMPLETYPE: C2RustUnnamed_0 = 3005;
pub const XML_SCHEMAP_SRC_RESOLVE: C2RustUnnamed_0 = 3004;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_4: C2RustUnnamed_0 = 3003;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_3: C2RustUnnamed_0 = 3002;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_2: C2RustUnnamed_0 = 3001;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_1: C2RustUnnamed_0 = 3000;
pub const XML_HTTP_UNKNOWN_HOST: C2RustUnnamed_0 = 2022;
pub const XML_HTTP_USE_IP: C2RustUnnamed_0 = 2021;
pub const XML_HTTP_URL_SYNTAX: C2RustUnnamed_0 = 2020;
pub const XML_FTP_URL_SYNTAX: C2RustUnnamed_0 = 2003;
pub const XML_FTP_ACCNT: C2RustUnnamed_0 = 2002;
pub const XML_FTP_EPSV_ANSWER: C2RustUnnamed_0 = 2001;
pub const XML_FTP_PASV_ANSWER: C2RustUnnamed_0 = 2000;
pub const XML_C14N_RELATIVE_NAMESPACE: C2RustUnnamed_0 = 1955;
pub const XML_C14N_UNKNOW_NODE: C2RustUnnamed_0 = 1954;
pub const XML_C14N_INVALID_NODE: C2RustUnnamed_0 = 1953;
pub const XML_C14N_CREATE_STACK: C2RustUnnamed_0 = 1952;
pub const XML_C14N_REQUIRES_UTF8: C2RustUnnamed_0 = 1951;
pub const XML_C14N_CREATE_CTXT: C2RustUnnamed_0 = 1950;
pub const XML_XPTR_EXTRA_OBJECTS: C2RustUnnamed_0 = 1903;
pub const XML_XPTR_EVAL_FAILED: C2RustUnnamed_0 = 1902;
pub const XML_XPTR_CHILDSEQ_START: C2RustUnnamed_0 = 1901;
pub const XML_XPTR_UNKNOWN_SCHEME: C2RustUnnamed_0 = 1900;
pub const XML_SCHEMAV_MISC: C2RustUnnamed_0 = 1879;
pub const XML_SCHEMAV_CVC_WILDCARD: C2RustUnnamed_0 = 1878;
pub const XML_SCHEMAV_CVC_IDC: C2RustUnnamed_0 = 1877;
pub const XML_SCHEMAV_CVC_TYPE_2: C2RustUnnamed_0 = 1876;
pub const XML_SCHEMAV_CVC_TYPE_1: C2RustUnnamed_0 = 1875;
pub const XML_SCHEMAV_CVC_AU: C2RustUnnamed_0 = 1874;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_1: C2RustUnnamed_0 = 1873;
pub const XML_SCHEMAV_DOCUMENT_ELEMENT_MISSING: C2RustUnnamed_0 = 1872;
pub const XML_SCHEMAV_ELEMENT_CONTENT: C2RustUnnamed_0 = 1871;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_5_2: C2RustUnnamed_0 = 1870;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_5_1: C2RustUnnamed_0 = 1869;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_4: C2RustUnnamed_0 = 1868;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_3_2_2: C2RustUnnamed_0 = 1867;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_3_2_1: C2RustUnnamed_0 = 1866;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_3_1: C2RustUnnamed_0 = 1865;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_4: C2RustUnnamed_0 = 1864;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_3: C2RustUnnamed_0 = 1863;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_2: C2RustUnnamed_0 = 1862;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_1: C2RustUnnamed_0 = 1861;
pub const XML_SCHEMAV_CVC_ELT_7: C2RustUnnamed_0 = 1860;
pub const XML_SCHEMAV_CVC_ELT_6: C2RustUnnamed_0 = 1859;
pub const XML_SCHEMAV_CVC_ELT_5_2_2_2_2: C2RustUnnamed_0 = 1858;
pub const XML_SCHEMAV_CVC_ELT_5_2_2_2_1: C2RustUnnamed_0 = 1857;
pub const XML_SCHEMAV_CVC_ELT_5_2_2_1: C2RustUnnamed_0 = 1856;
pub const XML_SCHEMAV_CVC_ELT_5_2_1: C2RustUnnamed_0 = 1855;
pub const XML_SCHEMAV_CVC_ELT_5_1_2: C2RustUnnamed_0 = 1854;
pub const XML_SCHEMAV_CVC_ELT_5_1_1: C2RustUnnamed_0 = 1853;
pub const XML_SCHEMAV_CVC_ELT_4_3: C2RustUnnamed_0 = 1852;
pub const XML_SCHEMAV_CVC_ELT_4_2: C2RustUnnamed_0 = 1851;
pub const XML_SCHEMAV_CVC_ELT_4_1: C2RustUnnamed_0 = 1850;
pub const XML_SCHEMAV_CVC_ELT_3_2_2: C2RustUnnamed_0 = 1849;
pub const XML_SCHEMAV_CVC_ELT_3_2_1: C2RustUnnamed_0 = 1848;
pub const XML_SCHEMAV_CVC_ELT_3_1: C2RustUnnamed_0 = 1847;
pub const XML_SCHEMAV_CVC_ELT_2: C2RustUnnamed_0 = 1846;
pub const XML_SCHEMAV_CVC_ELT_1: C2RustUnnamed_0 = 1845;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_4: C2RustUnnamed_0 = 1844;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_3: C2RustUnnamed_0 = 1843;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_2: C2RustUnnamed_0 = 1842;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_1: C2RustUnnamed_0 = 1841;
pub const XML_SCHEMAV_CVC_ENUMERATION_VALID: C2RustUnnamed_0 = 1840;
pub const XML_SCHEMAV_CVC_PATTERN_VALID: C2RustUnnamed_0 = 1839;
pub const XML_SCHEMAV_CVC_FRACTIONDIGITS_VALID: C2RustUnnamed_0 = 1838;
pub const XML_SCHEMAV_CVC_TOTALDIGITS_VALID: C2RustUnnamed_0 = 1837;
pub const XML_SCHEMAV_CVC_MAXEXCLUSIVE_VALID: C2RustUnnamed_0 = 1836;
pub const XML_SCHEMAV_CVC_MINEXCLUSIVE_VALID: C2RustUnnamed_0 = 1835;
pub const XML_SCHEMAV_CVC_MAXINCLUSIVE_VALID: C2RustUnnamed_0 = 1834;
pub const XML_SCHEMAV_CVC_MININCLUSIVE_VALID: C2RustUnnamed_0 = 1833;
pub const XML_SCHEMAV_CVC_MAXLENGTH_VALID: C2RustUnnamed_0 = 1832;
pub const XML_SCHEMAV_CVC_MINLENGTH_VALID: C2RustUnnamed_0 = 1831;
pub const XML_SCHEMAV_CVC_LENGTH_VALID: C2RustUnnamed_0 = 1830;
pub const XML_SCHEMAV_CVC_FACET_VALID: C2RustUnnamed_0 = 1829;
pub const XML_SCHEMAV_CVC_TYPE_3_1_2: C2RustUnnamed_0 = 1828;
pub const XML_SCHEMAV_CVC_TYPE_3_1_1: C2RustUnnamed_0 = 1827;
pub const XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_3: C2RustUnnamed_0 = 1826;
pub const XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_2: C2RustUnnamed_0 = 1825;
pub const XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_1: C2RustUnnamed_0 = 1824;
pub const XML_SCHEMAV_FACET: C2RustUnnamed_0 = 1823;
pub const XML_SCHEMAV_VALUE: C2RustUnnamed_0 = 1822;
pub const XML_SCHEMAV_ATTRINVALID: C2RustUnnamed_0 = 1821;
pub const XML_SCHEMAV_ATTRUNKNOWN: C2RustUnnamed_0 = 1820;
pub const XML_SCHEMAV_NOTSIMPLE: C2RustUnnamed_0 = 1819;
pub const XML_SCHEMAV_INTERNAL: C2RustUnnamed_0 = 1818;
pub const XML_SCHEMAV_CONSTRUCT: C2RustUnnamed_0 = 1817;
pub const XML_SCHEMAV_NOTDETERMINIST: C2RustUnnamed_0 = 1816;
pub const XML_SCHEMAV_INVALIDELEM: C2RustUnnamed_0 = 1815;
pub const XML_SCHEMAV_INVALIDATTR: C2RustUnnamed_0 = 1814;
pub const XML_SCHEMAV_EXTRACONTENT: C2RustUnnamed_0 = 1813;
pub const XML_SCHEMAV_NOTNILLABLE: C2RustUnnamed_0 = 1812;
pub const XML_SCHEMAV_HAVEDEFAULT: C2RustUnnamed_0 = 1811;
pub const XML_SCHEMAV_ELEMCONT: C2RustUnnamed_0 = 1810;
pub const XML_SCHEMAV_NOTEMPTY: C2RustUnnamed_0 = 1809;
pub const XML_SCHEMAV_ISABSTRACT: C2RustUnnamed_0 = 1808;
pub const XML_SCHEMAV_NOROLLBACK: C2RustUnnamed_0 = 1807;
pub const XML_SCHEMAV_NOTYPE: C2RustUnnamed_0 = 1806;
pub const XML_SCHEMAV_WRONGELEM: C2RustUnnamed_0 = 1805;
pub const XML_SCHEMAV_MISSING: C2RustUnnamed_0 = 1804;
pub const XML_SCHEMAV_NOTTOPLEVEL: C2RustUnnamed_0 = 1803;
pub const XML_SCHEMAV_UNDECLAREDELEM: C2RustUnnamed_0 = 1802;
pub const XML_SCHEMAV_NOROOT: C2RustUnnamed_0 = 1801;
pub const XML_SCHEMAP_COS_CT_EXTENDS_1_3: C2RustUnnamed_0 = 1800;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_3: C2RustUnnamed_0 = 1799;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_2: C2RustUnnamed_0 = 1798;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_1: C2RustUnnamed_0 = 1797;
pub const XML_SCHEMAP_SRC_IMPORT_3_2: C2RustUnnamed_0 = 1796;
pub const XML_SCHEMAP_SRC_IMPORT_3_1: C2RustUnnamed_0 = 1795;
pub const XML_SCHEMAP_UNION_NOT_EXPRESSIBLE: C2RustUnnamed_0 = 1794;
pub const XML_SCHEMAP_INTERSECTION_NOT_EXPRESSIBLE: C2RustUnnamed_0 = 1793;
pub const XML_SCHEMAP_WILDCARD_INVALID_NS_MEMBER: C2RustUnnamed_0 = 1792;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_3: C2RustUnnamed_0 = 1791;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_2: C2RustUnnamed_0 = 1790;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_2: C2RustUnnamed_0 = 1789;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_1: C2RustUnnamed_0 = 1788;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_1: C2RustUnnamed_0 = 1787;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_5: C2RustUnnamed_0 = 1786;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_4: C2RustUnnamed_0 = 1785;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_3: C2RustUnnamed_0 = 1784;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_2: C2RustUnnamed_0 = 1783;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_1: C2RustUnnamed_0 = 1782;
pub const XML_SCHEMAP_REF_AND_CONTENT: C2RustUnnamed_0 = 1781;
pub const XML_SCHEMAP_INVALID_ATTR_NAME: C2RustUnnamed_0 = 1780;
pub const XML_SCHEMAP_MISSING_SIMPLETYPE_CHILD: C2RustUnnamed_0 = 1779;
pub const XML_SCHEMAP_INVALID_ATTR_INLINE_COMBINATION: C2RustUnnamed_0 = 1778;
pub const XML_SCHEMAP_INVALID_ATTR_COMBINATION: C2RustUnnamed_0 = 1777;
pub const XML_SCHEMAP_SUPERNUMEROUS_LIST_ITEM_TYPE: C2RustUnnamed_0 = 1776;
pub const XML_SCHEMAP_RECURSIVE: C2RustUnnamed_0 = 1775;
pub const XML_SCHEMAP_INVALID_ATTR_USE: C2RustUnnamed_0 = 1774;
pub const XML_SCHEMAP_UNKNOWN_MEMBER_TYPE: C2RustUnnamed_0 = 1773;
pub const XML_SCHEMAP_NOT_SCHEMA: C2RustUnnamed_0 = 1772;
pub const XML_SCHEMAP_INCLUDE_SCHEMA_NO_URI: C2RustUnnamed_0 = 1771;
pub const XML_SCHEMAP_INCLUDE_SCHEMA_NOT_URI: C2RustUnnamed_0 = 1770;
pub const XML_SCHEMAP_UNKNOWN_INCLUDE_CHILD: C2RustUnnamed_0 = 1769;
pub const XML_SCHEMAP_DEF_AND_PREFIX: C2RustUnnamed_0 = 1768;
pub const XML_SCHEMAP_UNKNOWN_PREFIX: C2RustUnnamed_0 = 1767;
pub const XML_SCHEMAP_FAILED_PARSE: C2RustUnnamed_0 = 1766;
pub const XML_SCHEMAP_REDEFINED_NOTATION: C2RustUnnamed_0 = 1765;
pub const XML_SCHEMAP_REDEFINED_ATTR: C2RustUnnamed_0 = 1764;
pub const XML_SCHEMAP_REDEFINED_ATTRGROUP: C2RustUnnamed_0 = 1763;
pub const XML_SCHEMAP_REDEFINED_ELEMENT: C2RustUnnamed_0 = 1762;
pub const XML_SCHEMAP_REDEFINED_TYPE: C2RustUnnamed_0 = 1761;
pub const XML_SCHEMAP_REDEFINED_GROUP: C2RustUnnamed_0 = 1760;
pub const XML_SCHEMAP_NOROOT: C2RustUnnamed_0 = 1759;
pub const XML_SCHEMAP_NOTHING_TO_PARSE: C2RustUnnamed_0 = 1758;
pub const XML_SCHEMAP_FAILED_LOAD: C2RustUnnamed_0 = 1757;
pub const XML_SCHEMAP_REGEXP_INVALID: C2RustUnnamed_0 = 1756;
pub const XML_SCHEMAP_ELEM_DEFAULT_FIXED: C2RustUnnamed_0 = 1755;
pub const XML_SCHEMAP_UNKNOWN_UNION_CHILD: C2RustUnnamed_0 = 1754;
pub const XML_SCHEMAP_UNKNOWN_TYPE: C2RustUnnamed_0 = 1753;
pub const XML_SCHEMAP_UNKNOWN_SIMPLETYPE_CHILD: C2RustUnnamed_0 = 1752;
pub const XML_SCHEMAP_UNKNOWN_SIMPLECONTENT_CHILD: C2RustUnnamed_0 = 1751;
pub const XML_SCHEMAP_UNKNOWN_SEQUENCE_CHILD: C2RustUnnamed_0 = 1750;
pub const XML_SCHEMAP_UNKNOWN_SCHEMAS_CHILD: C2RustUnnamed_0 = 1749;
pub const XML_SCHEMAP_UNKNOWN_RESTRICTION_CHILD: C2RustUnnamed_0 = 1748;
pub const XML_SCHEMAP_UNKNOWN_REF: C2RustUnnamed_0 = 1747;
pub const XML_SCHEMAP_UNKNOWN_PROCESSCONTENT_CHILD: C2RustUnnamed_0 = 1746;
pub const XML_SCHEMAP_UNKNOWN_NOTATION_CHILD: C2RustUnnamed_0 = 1745;
pub const XML_SCHEMAP_UNKNOWN_LIST_CHILD: C2RustUnnamed_0 = 1744;
pub const XML_SCHEMAP_UNKNOWN_IMPORT_CHILD: C2RustUnnamed_0 = 1743;
pub const XML_SCHEMAP_UNKNOWN_GROUP_CHILD: C2RustUnnamed_0 = 1742;
pub const XML_SCHEMAP_UNKNOWN_FACET_TYPE: C2RustUnnamed_0 = 1741;
pub const XML_SCHEMAP_UNKNOWN_FACET_CHILD: C2RustUnnamed_0 = 1740;
pub const XML_SCHEMAP_UNKNOWN_EXTENSION_CHILD: C2RustUnnamed_0 = 1739;
pub const XML_SCHEMAP_UNKNOWN_ELEM_CHILD: C2RustUnnamed_0 = 1738;
pub const XML_SCHEMAP_UNKNOWN_COMPLEXTYPE_CHILD: C2RustUnnamed_0 = 1737;
pub const XML_SCHEMAP_UNKNOWN_COMPLEXCONTENT_CHILD: C2RustUnnamed_0 = 1736;
pub const XML_SCHEMAP_UNKNOWN_CHOICE_CHILD: C2RustUnnamed_0 = 1735;
pub const XML_SCHEMAP_UNKNOWN_BASE_TYPE: C2RustUnnamed_0 = 1734;
pub const XML_SCHEMAP_UNKNOWN_ATTRIBUTE_GROUP: C2RustUnnamed_0 = 1733;
pub const XML_SCHEMAP_UNKNOWN_ATTRGRP_CHILD: C2RustUnnamed_0 = 1732;
pub const XML_SCHEMAP_UNKNOWN_ATTR_CHILD: C2RustUnnamed_0 = 1731;
pub const XML_SCHEMAP_UNKNOWN_ANYATTRIBUTE_CHILD: C2RustUnnamed_0 = 1730;
pub const XML_SCHEMAP_UNKNOWN_ALL_CHILD: C2RustUnnamed_0 = 1729;
pub const XML_SCHEMAP_TYPE_AND_SUBTYPE: C2RustUnnamed_0 = 1728;
pub const XML_SCHEMAP_SIMPLETYPE_NONAME: C2RustUnnamed_0 = 1727;
pub const XML_SCHEMAP_RESTRICTION_NONAME_NOREF: C2RustUnnamed_0 = 1726;
pub const XML_SCHEMAP_REF_AND_SUBTYPE: C2RustUnnamed_0 = 1725;
pub const XML_SCHEMAP_NOTYPE_NOREF: C2RustUnnamed_0 = 1724;
pub const XML_SCHEMAP_NOTATION_NO_NAME: C2RustUnnamed_0 = 1723;
pub const XML_SCHEMAP_NOATTR_NOREF: C2RustUnnamed_0 = 1722;
pub const XML_SCHEMAP_INVALID_WHITE_SPACE: C2RustUnnamed_0 = 1721;
pub const XML_SCHEMAP_INVALID_REF_AND_SUBTYPE: C2RustUnnamed_0 = 1720;
pub const XML_SCHEMAP_INVALID_MINOCCURS: C2RustUnnamed_0 = 1719;
pub const XML_SCHEMAP_INVALID_MAXOCCURS: C2RustUnnamed_0 = 1718;
pub const XML_SCHEMAP_INVALID_FACET_VALUE: C2RustUnnamed_0 = 1717;
pub const XML_SCHEMAP_INVALID_FACET: C2RustUnnamed_0 = 1716;
pub const XML_SCHEMAP_INVALID_ENUM: C2RustUnnamed_0 = 1715;
pub const XML_SCHEMAP_INVALID_BOOLEAN: C2RustUnnamed_0 = 1714;
pub const XML_SCHEMAP_IMPORT_SCHEMA_NOT_URI: C2RustUnnamed_0 = 1713;
pub const XML_SCHEMAP_IMPORT_REDEFINE_NSNAME: C2RustUnnamed_0 = 1712;
pub const XML_SCHEMAP_IMPORT_NAMESPACE_NOT_URI: C2RustUnnamed_0 = 1711;
pub const XML_SCHEMAP_GROUP_NONAME_NOREF: C2RustUnnamed_0 = 1710;
pub const XML_SCHEMAP_FAILED_BUILD_IMPORT: C2RustUnnamed_0 = 1709;
pub const XML_SCHEMAP_FACET_NO_VALUE: C2RustUnnamed_0 = 1708;
pub const XML_SCHEMAP_EXTENSION_NO_BASE: C2RustUnnamed_0 = 1707;
pub const XML_SCHEMAP_ELEM_NONAME_NOREF: C2RustUnnamed_0 = 1706;
pub const XML_SCHEMAP_ELEMFORMDEFAULT_VALUE: C2RustUnnamed_0 = 1705;
pub const XML_SCHEMAP_COMPLEXTYPE_NONAME_NOREF: C2RustUnnamed_0 = 1704;
pub const XML_SCHEMAP_ATTR_NONAME_NOREF: C2RustUnnamed_0 = 1703;
pub const XML_SCHEMAP_ATTRGRP_NONAME_NOREF: C2RustUnnamed_0 = 1702;
pub const XML_SCHEMAP_ATTRFORMDEFAULT_VALUE: C2RustUnnamed_0 = 1701;
pub const XML_SCHEMAP_PREFIX_UNDEFINED: C2RustUnnamed_0 = 1700;
pub const XML_CATALOG_RECURSION: C2RustUnnamed_0 = 1654;
pub const XML_CATALOG_NOT_CATALOG: C2RustUnnamed_0 = 1653;
pub const XML_CATALOG_PREFER_VALUE: C2RustUnnamed_0 = 1652;
pub const XML_CATALOG_ENTRY_BROKEN: C2RustUnnamed_0 = 1651;
pub const XML_CATALOG_MISSING_ATTR: C2RustUnnamed_0 = 1650;
pub const XML_XINCLUDE_FRAGMENT_ID: C2RustUnnamed_0 = 1618;
pub const XML_XINCLUDE_DEPRECATED_NS: C2RustUnnamed_0 = 1617;
pub const XML_XINCLUDE_FALLBACK_NOT_IN_INCLUDE: C2RustUnnamed_0 = 1616;
pub const XML_XINCLUDE_FALLBACKS_IN_INCLUDE: C2RustUnnamed_0 = 1615;
pub const XML_XINCLUDE_INCLUDE_IN_INCLUDE: C2RustUnnamed_0 = 1614;
pub const XML_XINCLUDE_XPTR_RESULT: C2RustUnnamed_0 = 1613;
pub const XML_XINCLUDE_XPTR_FAILED: C2RustUnnamed_0 = 1612;
pub const XML_XINCLUDE_MULTIPLE_ROOT: C2RustUnnamed_0 = 1611;
pub const XML_XINCLUDE_UNKNOWN_ENCODING: C2RustUnnamed_0 = 1610;
pub const XML_XINCLUDE_BUILD_FAILED: C2RustUnnamed_0 = 1609;
pub const XML_XINCLUDE_INVALID_CHAR: C2RustUnnamed_0 = 1608;
pub const XML_XINCLUDE_TEXT_DOCUMENT: C2RustUnnamed_0 = 1607;
pub const XML_XINCLUDE_TEXT_FRAGMENT: C2RustUnnamed_0 = 1606;
pub const XML_XINCLUDE_HREF_URI: C2RustUnnamed_0 = 1605;
pub const XML_XINCLUDE_NO_FALLBACK: C2RustUnnamed_0 = 1604;
pub const XML_XINCLUDE_NO_HREF: C2RustUnnamed_0 = 1603;
pub const XML_XINCLUDE_ENTITY_DEF_MISMATCH: C2RustUnnamed_0 = 1602;
pub const XML_XINCLUDE_PARSE_VALUE: C2RustUnnamed_0 = 1601;
pub const XML_XINCLUDE_RECURSION: C2RustUnnamed_0 = 1600;
pub const XML_IO_EAFNOSUPPORT: C2RustUnnamed_0 = 1556;
pub const XML_IO_EALREADY: C2RustUnnamed_0 = 1555;
pub const XML_IO_EADDRINUSE: C2RustUnnamed_0 = 1554;
pub const XML_IO_ENETUNREACH: C2RustUnnamed_0 = 1553;
pub const XML_IO_ECONNREFUSED: C2RustUnnamed_0 = 1552;
pub const XML_IO_EISCONN: C2RustUnnamed_0 = 1551;
pub const XML_IO_ENOTSOCK: C2RustUnnamed_0 = 1550;
pub const XML_IO_LOAD_ERROR: C2RustUnnamed_0 = 1549;
pub const XML_IO_BUFFER_FULL: C2RustUnnamed_0 = 1548;
pub const XML_IO_NO_INPUT: C2RustUnnamed_0 = 1547;
pub const XML_IO_WRITE: C2RustUnnamed_0 = 1546;
pub const XML_IO_FLUSH: C2RustUnnamed_0 = 1545;
pub const XML_IO_ENCODER: C2RustUnnamed_0 = 1544;
pub const XML_IO_NETWORK_ATTEMPT: C2RustUnnamed_0 = 1543;
pub const XML_IO_EXDEV: C2RustUnnamed_0 = 1542;
pub const XML_IO_ETIMEDOUT: C2RustUnnamed_0 = 1541;
pub const XML_IO_ESRCH: C2RustUnnamed_0 = 1540;
pub const XML_IO_ESPIPE: C2RustUnnamed_0 = 1539;
pub const XML_IO_EROFS: C2RustUnnamed_0 = 1538;
pub const XML_IO_ERANGE: C2RustUnnamed_0 = 1537;
pub const XML_IO_EPIPE: C2RustUnnamed_0 = 1536;
pub const XML_IO_EPERM: C2RustUnnamed_0 = 1535;
pub const XML_IO_ENXIO: C2RustUnnamed_0 = 1534;
pub const XML_IO_ENOTTY: C2RustUnnamed_0 = 1533;
pub const XML_IO_ENOTSUP: C2RustUnnamed_0 = 1532;
pub const XML_IO_ENOTEMPTY: C2RustUnnamed_0 = 1531;
pub const XML_IO_ENOTDIR: C2RustUnnamed_0 = 1530;
pub const XML_IO_ENOSYS: C2RustUnnamed_0 = 1529;
pub const XML_IO_ENOSPC: C2RustUnnamed_0 = 1528;
pub const XML_IO_ENOMEM: C2RustUnnamed_0 = 1527;
pub const XML_IO_ENOLCK: C2RustUnnamed_0 = 1526;
pub const XML_IO_ENOEXEC: C2RustUnnamed_0 = 1525;
pub const XML_IO_ENOENT: C2RustUnnamed_0 = 1524;
pub const XML_IO_ENODEV: C2RustUnnamed_0 = 1523;
pub const XML_IO_ENFILE: C2RustUnnamed_0 = 1522;
pub const XML_IO_ENAMETOOLONG: C2RustUnnamed_0 = 1521;
pub const XML_IO_EMSGSIZE: C2RustUnnamed_0 = 1520;
pub const XML_IO_EMLINK: C2RustUnnamed_0 = 1519;
pub const XML_IO_EMFILE: C2RustUnnamed_0 = 1518;
pub const XML_IO_EISDIR: C2RustUnnamed_0 = 1517;
pub const XML_IO_EIO: C2RustUnnamed_0 = 1516;
pub const XML_IO_EINVAL: C2RustUnnamed_0 = 1515;
pub const XML_IO_EINTR: C2RustUnnamed_0 = 1514;
pub const XML_IO_EINPROGRESS: C2RustUnnamed_0 = 1513;
pub const XML_IO_EFBIG: C2RustUnnamed_0 = 1512;
pub const XML_IO_EFAULT: C2RustUnnamed_0 = 1511;
pub const XML_IO_EEXIST: C2RustUnnamed_0 = 1510;
pub const XML_IO_EDOM: C2RustUnnamed_0 = 1509;
pub const XML_IO_EDEADLK: C2RustUnnamed_0 = 1508;
pub const XML_IO_ECHILD: C2RustUnnamed_0 = 1507;
pub const XML_IO_ECANCELED: C2RustUnnamed_0 = 1506;
pub const XML_IO_EBUSY: C2RustUnnamed_0 = 1505;
pub const XML_IO_EBADMSG: C2RustUnnamed_0 = 1504;
pub const XML_IO_EBADF: C2RustUnnamed_0 = 1503;
pub const XML_IO_EAGAIN: C2RustUnnamed_0 = 1502;
pub const XML_IO_EACCES: C2RustUnnamed_0 = 1501;
pub const XML_IO_UNKNOWN: C2RustUnnamed_0 = 1500;
pub const XML_SAVE_UNKNOWN_ENCODING: C2RustUnnamed_0 = 1403;
pub const XML_SAVE_NO_DOCTYPE: C2RustUnnamed_0 = 1402;
pub const XML_SAVE_CHAR_INVALID: C2RustUnnamed_0 = 1401;
pub const XML_SAVE_NOT_UTF8: C2RustUnnamed_0 = 1400;
pub const XML_TREE_NOT_UTF8: C2RustUnnamed_0 = 1303;
pub const XML_TREE_UNTERMINATED_ENTITY: C2RustUnnamed_0 = 1302;
pub const XML_TREE_INVALID_DEC: C2RustUnnamed_0 = 1301;
pub const XML_TREE_INVALID_HEX: C2RustUnnamed_0 = 1300;
pub const XML_XPATH_INVALID_CHAR_ERROR: C2RustUnnamed_0 = 1221;
pub const XML_XPATH_ENCODING_ERROR: C2RustUnnamed_0 = 1220;
pub const XML_XPATH_UNDEF_PREFIX_ERROR: C2RustUnnamed_0 = 1219;
pub const XML_XPTR_SUB_RESOURCE_ERROR: C2RustUnnamed_0 = 1218;
pub const XML_XPTR_RESOURCE_ERROR: C2RustUnnamed_0 = 1217;
pub const XML_XPTR_SYNTAX_ERROR: C2RustUnnamed_0 = 1216;
pub const XML_XPATH_MEMORY_ERROR: C2RustUnnamed_0 = 1215;
pub const XML_XPATH_INVALID_CTXT_POSITION: C2RustUnnamed_0 = 1214;
pub const XML_XPATH_INVALID_CTXT_SIZE: C2RustUnnamed_0 = 1213;
pub const XML_XPATH_INVALID_ARITY: C2RustUnnamed_0 = 1212;
pub const XML_XPATH_INVALID_TYPE: C2RustUnnamed_0 = 1211;
pub const XML_XPATH_INVALID_OPERAND: C2RustUnnamed_0 = 1210;
pub const XML_XPATH_UNKNOWN_FUNC_ERROR: C2RustUnnamed_0 = 1209;
pub const XML_XPATH_UNCLOSED_ERROR: C2RustUnnamed_0 = 1208;
pub const XML_XPATH_EXPR_ERROR: C2RustUnnamed_0 = 1207;
pub const XML_XPATH_INVALID_PREDICATE_ERROR: C2RustUnnamed_0 = 1206;
pub const XML_XPATH_UNDEF_VARIABLE_ERROR: C2RustUnnamed_0 = 1205;
pub const XML_XPATH_VARIABLE_REF_ERROR: C2RustUnnamed_0 = 1204;
pub const XML_XPATH_START_LITERAL_ERROR: C2RustUnnamed_0 = 1203;
pub const XML_XPATH_UNFINISHED_LITERAL_ERROR: C2RustUnnamed_0 = 1202;
pub const XML_XPATH_NUMBER_ERROR: C2RustUnnamed_0 = 1201;
pub const XML_XPATH_EXPRESSION_OK: C2RustUnnamed_0 = 1200;
pub const XML_RNGP_XML_NS: C2RustUnnamed_0 = 1122;
pub const XML_RNGP_XMLNS_NAME: C2RustUnnamed_0 = 1121;
pub const XML_RNGP_VALUE_NO_CONTENT: C2RustUnnamed_0 = 1120;
pub const XML_RNGP_VALUE_EMPTY: C2RustUnnamed_0 = 1119;
pub const XML_RNGP_URI_NOT_ABSOLUTE: C2RustUnnamed_0 = 1118;
pub const XML_RNGP_URI_FRAGMENT: C2RustUnnamed_0 = 1117;
pub const XML_RNGP_UNKNOWN_TYPE_LIB: C2RustUnnamed_0 = 1116;
pub const XML_RNGP_UNKNOWN_CONSTRUCT: C2RustUnnamed_0 = 1115;
pub const XML_RNGP_UNKNOWN_COMBINE: C2RustUnnamed_0 = 1114;
pub const XML_RNGP_UNKNOWN_ATTRIBUTE: C2RustUnnamed_0 = 1113;
pub const XML_RNGP_TYPE_VALUE: C2RustUnnamed_0 = 1112;
pub const XML_RNGP_TYPE_NOT_FOUND: C2RustUnnamed_0 = 1111;
pub const XML_RNGP_TYPE_MISSING: C2RustUnnamed_0 = 1110;
pub const XML_RNGP_TEXT_HAS_CHILD: C2RustUnnamed_0 = 1109;
pub const XML_RNGP_TEXT_EXPECTED: C2RustUnnamed_0 = 1108;
pub const XML_RNGP_START_MISSING: C2RustUnnamed_0 = 1107;
pub const XML_RNGP_START_EMPTY: C2RustUnnamed_0 = 1106;
pub const XML_RNGP_START_CONTENT: C2RustUnnamed_0 = 1105;
pub const XML_RNGP_START_CHOICE_AND_INTERLEAVE: C2RustUnnamed_0 = 1104;
pub const XML_RNGP_REF_NOT_EMPTY: C2RustUnnamed_0 = 1103;
pub const XML_RNGP_REF_NO_NAME: C2RustUnnamed_0 = 1102;
pub const XML_RNGP_REF_NO_DEF: C2RustUnnamed_0 = 1101;
pub const XML_RNGP_REF_NAME_INVALID: C2RustUnnamed_0 = 1100;
pub const XML_RNGP_REF_CYCLE: C2RustUnnamed_0 = 1099;
pub const XML_RNGP_REF_CREATE_FAILED: C2RustUnnamed_0 = 1098;
pub const XML_RNGP_PREFIX_UNDEFINED: C2RustUnnamed_0 = 1097;
pub const XML_RNGP_PAT_START_VALUE: C2RustUnnamed_0 = 1096;
pub const XML_RNGP_PAT_START_TEXT: C2RustUnnamed_0 = 1095;
pub const XML_RNGP_PAT_START_ONEMORE: C2RustUnnamed_0 = 1094;
pub const XML_RNGP_PAT_START_LIST: C2RustUnnamed_0 = 1093;
pub const XML_RNGP_PAT_START_INTERLEAVE: C2RustUnnamed_0 = 1092;
pub const XML_RNGP_PAT_START_GROUP: C2RustUnnamed_0 = 1091;
pub const XML_RNGP_PAT_START_EMPTY: C2RustUnnamed_0 = 1090;
pub const XML_RNGP_PAT_START_DATA: C2RustUnnamed_0 = 1089;
pub const XML_RNGP_PAT_START_ATTR: C2RustUnnamed_0 = 1088;
pub const XML_RNGP_PAT_ONEMORE_INTERLEAVE_ATTR: C2RustUnnamed_0 = 1087;
pub const XML_RNGP_PAT_ONEMORE_GROUP_ATTR: C2RustUnnamed_0 = 1086;
pub const XML_RNGP_PAT_NSNAME_EXCEPT_NSNAME: C2RustUnnamed_0 = 1085;
pub const XML_RNGP_PAT_NSNAME_EXCEPT_ANYNAME: C2RustUnnamed_0 = 1084;
pub const XML_RNGP_PAT_LIST_TEXT: C2RustUnnamed_0 = 1083;
pub const XML_RNGP_PAT_LIST_REF: C2RustUnnamed_0 = 1082;
pub const XML_RNGP_PAT_LIST_LIST: C2RustUnnamed_0 = 1081;
pub const XML_RNGP_PAT_LIST_INTERLEAVE: C2RustUnnamed_0 = 1080;
pub const XML_RNGP_PAT_LIST_ELEM: C2RustUnnamed_0 = 1079;
pub const XML_RNGP_PAT_LIST_ATTR: C2RustUnnamed_0 = 1078;
pub const XML_RNGP_PAT_DATA_EXCEPT_TEXT: C2RustUnnamed_0 = 1077;
pub const XML_RNGP_PAT_DATA_EXCEPT_REF: C2RustUnnamed_0 = 1076;
pub const XML_RNGP_PAT_DATA_EXCEPT_ONEMORE: C2RustUnnamed_0 = 1075;
pub const XML_RNGP_PAT_DATA_EXCEPT_LIST: C2RustUnnamed_0 = 1074;
pub const XML_RNGP_PAT_DATA_EXCEPT_INTERLEAVE: C2RustUnnamed_0 = 1073;
pub const XML_RNGP_PAT_DATA_EXCEPT_GROUP: C2RustUnnamed_0 = 1072;
pub const XML_RNGP_PAT_DATA_EXCEPT_EMPTY: C2RustUnnamed_0 = 1071;
pub const XML_RNGP_PAT_DATA_EXCEPT_ELEM: C2RustUnnamed_0 = 1070;
pub const XML_RNGP_PAT_DATA_EXCEPT_ATTR: C2RustUnnamed_0 = 1069;
pub const XML_RNGP_PAT_ATTR_ELEM: C2RustUnnamed_0 = 1068;
pub const XML_RNGP_PAT_ATTR_ATTR: C2RustUnnamed_0 = 1067;
pub const XML_RNGP_PAT_ANYNAME_EXCEPT_ANYNAME: C2RustUnnamed_0 = 1066;
pub const XML_RNGP_PARSE_ERROR: C2RustUnnamed_0 = 1065;
pub const XML_RNGP_PARENTREF_NOT_EMPTY: C2RustUnnamed_0 = 1064;
pub const XML_RNGP_PARENTREF_NO_PARENT: C2RustUnnamed_0 = 1063;
pub const XML_RNGP_PARENTREF_NO_NAME: C2RustUnnamed_0 = 1062;
pub const XML_RNGP_PARENTREF_NAME_INVALID: C2RustUnnamed_0 = 1061;
pub const XML_RNGP_PARENTREF_CREATE_FAILED: C2RustUnnamed_0 = 1060;
pub const XML_RNGP_PARAM_NAME_MISSING: C2RustUnnamed_0 = 1059;
pub const XML_RNGP_PARAM_FORBIDDEN: C2RustUnnamed_0 = 1058;
pub const XML_RNGP_NSNAME_NO_NS: C2RustUnnamed_0 = 1057;
pub const XML_RNGP_NSNAME_ATTR_ANCESTOR: C2RustUnnamed_0 = 1056;
pub const XML_RNGP_NOTALLOWED_NOT_EMPTY: C2RustUnnamed_0 = 1055;
pub const XML_RNGP_NEED_COMBINE: C2RustUnnamed_0 = 1054;
pub const XML_RNGP_NAME_MISSING: C2RustUnnamed_0 = 1053;
pub const XML_RNGP_MISSING_HREF: C2RustUnnamed_0 = 1052;
pub const XML_RNGP_INVALID_VALUE: C2RustUnnamed_0 = 1051;
pub const XML_RNGP_INVALID_URI: C2RustUnnamed_0 = 1050;
pub const XML_RNGP_INVALID_DEFINE_NAME: C2RustUnnamed_0 = 1049;
pub const XML_RNGP_INTERLEAVE_NO_CONTENT: C2RustUnnamed_0 = 1048;
pub const XML_RNGP_INTERLEAVE_EMPTY: C2RustUnnamed_0 = 1047;
pub const XML_RNGP_INTERLEAVE_CREATE_FAILED: C2RustUnnamed_0 = 1046;
pub const XML_RNGP_INTERLEAVE_ADD: C2RustUnnamed_0 = 1045;
pub const XML_RNGP_INCLUDE_RECURSE: C2RustUnnamed_0 = 1044;
pub const XML_RNGP_INCLUDE_FAILURE: C2RustUnnamed_0 = 1043;
pub const XML_RNGP_INCLUDE_EMPTY: C2RustUnnamed_0 = 1042;
pub const XML_RNGP_HREF_ERROR: C2RustUnnamed_0 = 1041;
pub const XML_RNGP_GROUP_ATTR_CONFLICT: C2RustUnnamed_0 = 1040;
pub const XML_RNGP_GRAMMAR_NO_START: C2RustUnnamed_0 = 1039;
pub const XML_RNGP_GRAMMAR_MISSING: C2RustUnnamed_0 = 1038;
pub const XML_RNGP_GRAMMAR_EMPTY: C2RustUnnamed_0 = 1037;
pub const XML_RNGP_GRAMMAR_CONTENT: C2RustUnnamed_0 = 1036;
pub const XML_RNGP_FOREIGN_ELEMENT: C2RustUnnamed_0 = 1035;
pub const XML_RNGP_FORBIDDEN_ATTRIBUTE: C2RustUnnamed_0 = 1034;
pub const XML_RNGP_EXTERNALREF_RECURSE: C2RustUnnamed_0 = 1033;
pub const XML_RNGP_EXTERNAL_REF_FAILURE: C2RustUnnamed_0 = 1032;
pub const XML_RNGP_EXTERNALREF_EMTPY: C2RustUnnamed_0 = 1031;
pub const XML_RNGP_EXCEPT_NO_CONTENT: C2RustUnnamed_0 = 1030;
pub const XML_RNGP_EXCEPT_MULTIPLE: C2RustUnnamed_0 = 1029;
pub const XML_RNGP_EXCEPT_MISSING: C2RustUnnamed_0 = 1028;
pub const XML_RNGP_EXCEPT_EMPTY: C2RustUnnamed_0 = 1027;
pub const XML_RNGP_ERROR_TYPE_LIB: C2RustUnnamed_0 = 1026;
pub const XML_RNGP_EMPTY_NOT_EMPTY: C2RustUnnamed_0 = 1025;
pub const XML_RNGP_EMPTY_CONTENT: C2RustUnnamed_0 = 1024;
pub const XML_RNGP_EMPTY_CONSTRUCT: C2RustUnnamed_0 = 1023;
pub const XML_RNGP_EMPTY: C2RustUnnamed_0 = 1022;
pub const XML_RNGP_ELEM_TEXT_CONFLICT: C2RustUnnamed_0 = 1021;
pub const XML_RNGP_ELEMENT_NO_CONTENT: C2RustUnnamed_0 = 1020;
pub const XML_RNGP_ELEMENT_NAME: C2RustUnnamed_0 = 1019;
pub const XML_RNGP_ELEMENT_CONTENT: C2RustUnnamed_0 = 1018;
pub const XML_RNGP_ELEMENT_EMPTY: C2RustUnnamed_0 = 1017;
pub const XML_RNGP_ELEM_CONTENT_ERROR: C2RustUnnamed_0 = 1016;
pub const XML_RNGP_ELEM_CONTENT_EMPTY: C2RustUnnamed_0 = 1015;
pub const XML_RNGP_DEFINE_NAME_MISSING: C2RustUnnamed_0 = 1014;
pub const XML_RNGP_DEFINE_MISSING: C2RustUnnamed_0 = 1013;
pub const XML_RNGP_DEFINE_EMPTY: C2RustUnnamed_0 = 1012;
pub const XML_RNGP_DEFINE_CREATE_FAILED: C2RustUnnamed_0 = 1011;
pub const XML_RNGP_DEF_CHOICE_AND_INTERLEAVE: C2RustUnnamed_0 = 1010;
pub const XML_RNGP_DATA_CONTENT: C2RustUnnamed_0 = 1009;
pub const XML_RNGP_CREATE_FAILURE: C2RustUnnamed_0 = 1008;
pub const XML_RNGP_CHOICE_EMPTY: C2RustUnnamed_0 = 1007;
pub const XML_RNGP_CHOICE_CONTENT: C2RustUnnamed_0 = 1006;
pub const XML_RNGP_ATTRIBUTE_NOOP: C2RustUnnamed_0 = 1005;
pub const XML_RNGP_ATTRIBUTE_EMPTY: C2RustUnnamed_0 = 1004;
pub const XML_RNGP_ATTRIBUTE_CONTENT: C2RustUnnamed_0 = 1003;
pub const XML_RNGP_ATTRIBUTE_CHILDREN: C2RustUnnamed_0 = 1002;
pub const XML_RNGP_ATTR_CONFLICT: C2RustUnnamed_0 = 1001;
pub const XML_RNGP_ANYNAME_ATTR_ANCESTOR: C2RustUnnamed_0 = 1000;
pub const XML_HTML_INCORRECTLY_OPENED_COMMENT: C2RustUnnamed_0 = 802;
pub const XML_HTML_UNKNOWN_TAG: C2RustUnnamed_0 = 801;
pub const XML_HTML_STRUCURE_ERROR: C2RustUnnamed_0 = 800;
pub const XML_DTD_DUP_TOKEN: C2RustUnnamed_0 = 541;
pub const XML_DTD_XMLID_TYPE: C2RustUnnamed_0 = 540;
pub const XML_DTD_XMLID_VALUE: C2RustUnnamed_0 = 539;
pub const XML_DTD_STANDALONE_DEFAULTED: C2RustUnnamed_0 = 538;
pub const XML_DTD_UNKNOWN_NOTATION: C2RustUnnamed_0 = 537;
pub const XML_DTD_UNKNOWN_ID: C2RustUnnamed_0 = 536;
pub const XML_DTD_UNKNOWN_ENTITY: C2RustUnnamed_0 = 535;
pub const XML_DTD_UNKNOWN_ELEM: C2RustUnnamed_0 = 534;
pub const XML_DTD_UNKNOWN_ATTRIBUTE: C2RustUnnamed_0 = 533;
pub const XML_DTD_STANDALONE_WHITE_SPACE: C2RustUnnamed_0 = 532;
pub const XML_DTD_ROOT_NAME: C2RustUnnamed_0 = 531;
pub const XML_DTD_NOT_STANDALONE: C2RustUnnamed_0 = 530;
pub const XML_DTD_NOT_PCDATA: C2RustUnnamed_0 = 529;
pub const XML_DTD_NOT_EMPTY: C2RustUnnamed_0 = 528;
pub const XML_DTD_NOTATION_VALUE: C2RustUnnamed_0 = 527;
pub const XML_DTD_NOTATION_REDEFINED: C2RustUnnamed_0 = 526;
pub const XML_DTD_NO_ROOT: C2RustUnnamed_0 = 525;
pub const XML_DTD_NO_PREFIX: C2RustUnnamed_0 = 524;
pub const XML_DTD_NO_ELEM_NAME: C2RustUnnamed_0 = 523;
pub const XML_DTD_NO_DTD: C2RustUnnamed_0 = 522;
pub const XML_DTD_NO_DOC: C2RustUnnamed_0 = 521;
pub const XML_DTD_MULTIPLE_ID: C2RustUnnamed_0 = 520;
pub const XML_DTD_MIXED_CORRUPT: C2RustUnnamed_0 = 519;
pub const XML_DTD_MISSING_ATTRIBUTE: C2RustUnnamed_0 = 518;
pub const XML_DTD_LOAD_ERROR: C2RustUnnamed_0 = 517;
pub const XML_DTD_INVALID_DEFAULT: C2RustUnnamed_0 = 516;
pub const XML_DTD_INVALID_CHILD: C2RustUnnamed_0 = 515;
pub const XML_DTD_ID_SUBSET: C2RustUnnamed_0 = 514;
pub const XML_DTD_ID_REDEFINED: C2RustUnnamed_0 = 513;
pub const XML_DTD_ID_FIXED: C2RustUnnamed_0 = 512;
pub const XML_DTD_ENTITY_TYPE: C2RustUnnamed_0 = 511;
pub const XML_DTD_EMPTY_NOTATION: C2RustUnnamed_0 = 510;
pub const XML_DTD_ELEM_REDEFINED: C2RustUnnamed_0 = 509;
pub const XML_DTD_ELEM_NAMESPACE: C2RustUnnamed_0 = 508;
pub const XML_DTD_ELEM_DEFAULT_NAMESPACE: C2RustUnnamed_0 = 507;
pub const XML_DTD_DIFFERENT_PREFIX: C2RustUnnamed_0 = 506;
pub const XML_DTD_CONTENT_NOT_DETERMINIST: C2RustUnnamed_0 = 505;
pub const XML_DTD_CONTENT_MODEL: C2RustUnnamed_0 = 504;
pub const XML_DTD_CONTENT_ERROR: C2RustUnnamed_0 = 503;
pub const XML_DTD_ATTRIBUTE_VALUE: C2RustUnnamed_0 = 502;
pub const XML_DTD_ATTRIBUTE_REDEFINED: C2RustUnnamed_0 = 501;
pub const XML_DTD_ATTRIBUTE_DEFAULT: C2RustUnnamed_0 = 500;
pub const XML_NS_ERR_COLON: C2RustUnnamed_0 = 205;
pub const XML_NS_ERR_EMPTY: C2RustUnnamed_0 = 204;
pub const XML_NS_ERR_ATTRIBUTE_REDEFINED: C2RustUnnamed_0 = 203;
pub const XML_NS_ERR_QNAME: C2RustUnnamed_0 = 202;
pub const XML_NS_ERR_UNDEFINED_NAMESPACE: C2RustUnnamed_0 = 201;
pub const XML_NS_ERR_XML_NAMESPACE: C2RustUnnamed_0 = 200;
pub const XML_ERR_COMMENT_ABRUPTLY_ENDED: C2RustUnnamed_0 = 112;
pub const XML_ERR_USER_STOP: C2RustUnnamed_0 = 111;
pub const XML_ERR_NAME_TOO_LONG: C2RustUnnamed_0 = 110;
pub const XML_ERR_VERSION_MISMATCH: C2RustUnnamed_0 = 109;
pub const XML_ERR_UNKNOWN_VERSION: C2RustUnnamed_0 = 108;
pub const XML_WAR_ENTITY_REDEFINED: C2RustUnnamed_0 = 107;
pub const XML_WAR_NS_COLUMN: C2RustUnnamed_0 = 106;
pub const XML_ERR_NOTATION_PROCESSING: C2RustUnnamed_0 = 105;
pub const XML_ERR_ENTITY_PROCESSING: C2RustUnnamed_0 = 104;
pub const XML_ERR_NOT_STANDALONE: C2RustUnnamed_0 = 103;
pub const XML_WAR_SPACE_VALUE: C2RustUnnamed_0 = 102;
pub const XML_ERR_MISSING_ENCODING: C2RustUnnamed_0 = 101;
pub const XML_WAR_NS_URI_RELATIVE: C2RustUnnamed_0 = 100;
pub const XML_WAR_NS_URI: C2RustUnnamed_0 = 99;
pub const XML_WAR_LANG_VALUE: C2RustUnnamed_0 = 98;
pub const XML_WAR_UNKNOWN_VERSION: C2RustUnnamed_0 = 97;
pub const XML_ERR_VERSION_MISSING: C2RustUnnamed_0 = 96;
pub const XML_ERR_CONDSEC_INVALID_KEYWORD: C2RustUnnamed_0 = 95;
pub const XML_ERR_NO_DTD: C2RustUnnamed_0 = 94;
pub const XML_WAR_CATALOG_PI: C2RustUnnamed_0 = 93;
pub const XML_ERR_URI_FRAGMENT: C2RustUnnamed_0 = 92;
pub const XML_ERR_INVALID_URI: C2RustUnnamed_0 = 91;
pub const XML_ERR_ENTITY_BOUNDARY: C2RustUnnamed_0 = 90;
pub const XML_ERR_ENTITY_LOOP: C2RustUnnamed_0 = 89;
pub const XML_ERR_ENTITY_PE_INTERNAL: C2RustUnnamed_0 = 88;
pub const XML_ERR_ENTITY_CHAR_ERROR: C2RustUnnamed_0 = 87;
pub const XML_ERR_EXTRA_CONTENT: C2RustUnnamed_0 = 86;
pub const XML_ERR_NOT_WELL_BALANCED: C2RustUnnamed_0 = 85;
pub const XML_ERR_VALUE_REQUIRED: C2RustUnnamed_0 = 84;
pub const XML_ERR_CONDSEC_INVALID: C2RustUnnamed_0 = 83;
pub const XML_ERR_EXT_ENTITY_STANDALONE: C2RustUnnamed_0 = 82;
pub const XML_ERR_INVALID_ENCODING: C2RustUnnamed_0 = 81;
pub const XML_ERR_HYPHEN_IN_COMMENT: C2RustUnnamed_0 = 80;
pub const XML_ERR_ENCODING_NAME: C2RustUnnamed_0 = 79;
pub const XML_ERR_STANDALONE_VALUE: C2RustUnnamed_0 = 78;
pub const XML_ERR_TAG_NOT_FINISHED: C2RustUnnamed_0 = 77;
pub const XML_ERR_TAG_NAME_MISMATCH: C2RustUnnamed_0 = 76;
pub const XML_ERR_EQUAL_REQUIRED: C2RustUnnamed_0 = 75;
pub const XML_ERR_LTSLASH_REQUIRED: C2RustUnnamed_0 = 74;
pub const XML_ERR_GT_REQUIRED: C2RustUnnamed_0 = 73;
pub const XML_ERR_LT_REQUIRED: C2RustUnnamed_0 = 72;
pub const XML_ERR_PUBID_REQUIRED: C2RustUnnamed_0 = 71;
pub const XML_ERR_URI_REQUIRED: C2RustUnnamed_0 = 70;
pub const XML_ERR_PCDATA_REQUIRED: C2RustUnnamed_0 = 69;
pub const XML_ERR_NAME_REQUIRED: C2RustUnnamed_0 = 68;
pub const XML_ERR_NMTOKEN_REQUIRED: C2RustUnnamed_0 = 67;
pub const XML_ERR_SEPARATOR_REQUIRED: C2RustUnnamed_0 = 66;
pub const XML_ERR_SPACE_REQUIRED: C2RustUnnamed_0 = 65;
pub const XML_ERR_RESERVED_XML_NAME: C2RustUnnamed_0 = 64;
pub const XML_ERR_CDATA_NOT_FINISHED: C2RustUnnamed_0 = 63;
pub const XML_ERR_MISPLACED_CDATA_END: C2RustUnnamed_0 = 62;
pub const XML_ERR_DOCTYPE_NOT_FINISHED: C2RustUnnamed_0 = 61;
pub const XML_ERR_EXT_SUBSET_NOT_FINISHED: C2RustUnnamed_0 = 60;
pub const XML_ERR_CONDSEC_NOT_FINISHED: C2RustUnnamed_0 = 59;
pub const XML_ERR_CONDSEC_NOT_STARTED: C2RustUnnamed_0 = 58;
pub const XML_ERR_XMLDECL_NOT_FINISHED: C2RustUnnamed_0 = 57;
pub const XML_ERR_XMLDECL_NOT_STARTED: C2RustUnnamed_0 = 56;
pub const XML_ERR_ELEMCONTENT_NOT_FINISHED: C2RustUnnamed_0 = 55;
pub const XML_ERR_ELEMCONTENT_NOT_STARTED: C2RustUnnamed_0 = 54;
pub const XML_ERR_MIXED_NOT_FINISHED: C2RustUnnamed_0 = 53;
pub const XML_ERR_MIXED_NOT_STARTED: C2RustUnnamed_0 = 52;
pub const XML_ERR_ATTLIST_NOT_FINISHED: C2RustUnnamed_0 = 51;
pub const XML_ERR_ATTLIST_NOT_STARTED: C2RustUnnamed_0 = 50;
pub const XML_ERR_NOTATION_NOT_FINISHED: C2RustUnnamed_0 = 49;
pub const XML_ERR_NOTATION_NOT_STARTED: C2RustUnnamed_0 = 48;
pub const XML_ERR_PI_NOT_FINISHED: C2RustUnnamed_0 = 47;
pub const XML_ERR_PI_NOT_STARTED: C2RustUnnamed_0 = 46;
pub const XML_ERR_COMMENT_NOT_FINISHED: C2RustUnnamed_0 = 45;
pub const XML_ERR_LITERAL_NOT_FINISHED: C2RustUnnamed_0 = 44;
pub const XML_ERR_LITERAL_NOT_STARTED: C2RustUnnamed_0 = 43;
pub const XML_ERR_ATTRIBUTE_REDEFINED: C2RustUnnamed_0 = 42;
pub const XML_ERR_ATTRIBUTE_WITHOUT_VALUE: C2RustUnnamed_0 = 41;
pub const XML_ERR_ATTRIBUTE_NOT_FINISHED: C2RustUnnamed_0 = 40;
pub const XML_ERR_ATTRIBUTE_NOT_STARTED: C2RustUnnamed_0 = 39;
pub const XML_ERR_LT_IN_ATTRIBUTE: C2RustUnnamed_0 = 38;
pub const XML_ERR_ENTITY_NOT_FINISHED: C2RustUnnamed_0 = 37;
pub const XML_ERR_ENTITY_NOT_STARTED: C2RustUnnamed_0 = 36;
pub const XML_ERR_NS_DECL_ERROR: C2RustUnnamed_0 = 35;
pub const XML_ERR_STRING_NOT_CLOSED: C2RustUnnamed_0 = 34;
pub const XML_ERR_STRING_NOT_STARTED: C2RustUnnamed_0 = 33;
pub const XML_ERR_UNSUPPORTED_ENCODING: C2RustUnnamed_0 = 32;
pub const XML_ERR_UNKNOWN_ENCODING: C2RustUnnamed_0 = 31;
pub const XML_ERR_ENTITY_IS_PARAMETER: C2RustUnnamed_0 = 30;
pub const XML_ERR_ENTITY_IS_EXTERNAL: C2RustUnnamed_0 = 29;
pub const XML_ERR_UNPARSED_ENTITY: C2RustUnnamed_0 = 28;
pub const XML_WAR_UNDECLARED_ENTITY: C2RustUnnamed_0 = 27;
pub const XML_ERR_UNDECLARED_ENTITY: C2RustUnnamed_0 = 26;
pub const XML_ERR_PEREF_SEMICOL_MISSING: C2RustUnnamed_0 = 25;
pub const XML_ERR_PEREF_NO_NAME: C2RustUnnamed_0 = 24;
pub const XML_ERR_ENTITYREF_SEMICOL_MISSING: C2RustUnnamed_0 = 23;
pub const XML_ERR_ENTITYREF_NO_NAME: C2RustUnnamed_0 = 22;
pub const XML_ERR_PEREF_IN_INT_SUBSET: C2RustUnnamed_0 = 21;
pub const XML_ERR_PEREF_IN_EPILOG: C2RustUnnamed_0 = 20;
pub const XML_ERR_PEREF_IN_PROLOG: C2RustUnnamed_0 = 19;
pub const XML_ERR_PEREF_AT_EOF: C2RustUnnamed_0 = 18;
pub const XML_ERR_ENTITYREF_IN_DTD: C2RustUnnamed_0 = 17;
pub const XML_ERR_ENTITYREF_IN_EPILOG: C2RustUnnamed_0 = 16;
pub const XML_ERR_ENTITYREF_IN_PROLOG: C2RustUnnamed_0 = 15;
pub const XML_ERR_ENTITYREF_AT_EOF: C2RustUnnamed_0 = 14;
pub const XML_ERR_CHARREF_IN_DTD: C2RustUnnamed_0 = 13;
pub const XML_ERR_CHARREF_IN_EPILOG: C2RustUnnamed_0 = 12;
pub const XML_ERR_CHARREF_IN_PROLOG: C2RustUnnamed_0 = 11;
pub const XML_ERR_CHARREF_AT_EOF: C2RustUnnamed_0 = 10;
pub const XML_ERR_INVALID_CHAR: C2RustUnnamed_0 = 9;
pub const XML_ERR_INVALID_CHARREF: C2RustUnnamed_0 = 8;
pub const XML_ERR_INVALID_DEC_CHARREF: C2RustUnnamed_0 = 7;
pub const XML_ERR_INVALID_HEX_CHARREF: C2RustUnnamed_0 = 6;
pub const XML_ERR_DOCUMENT_END: C2RustUnnamed_0 = 5;
pub const XML_ERR_DOCUMENT_EMPTY: C2RustUnnamed_0 = 4;
pub const XML_ERR_DOCUMENT_START: C2RustUnnamed_0 = 3;
pub const XML_ERR_INTERNAL_ERROR: C2RustUnnamed_0 = 1;
pub const XML_ERR_OK: C2RustUnnamed_0 = 0;
unsafe extern "C" fn xmlRegexpErrMemory(
    mut ctxt: xmlRegParserCtxtPtr,
    mut extra: *const libc::c_char,
) {
    let mut regexp: *const libc::c_char = 0 as *const libc::c_char;
    if !ctxt.is_null() {
        regexp = (*ctxt).string as *const libc::c_char;
        (*ctxt).error = XML_ERR_NO_MEMORY as libc::c_int;
    }
    __xmlRaiseError(
        None,
        None,
        0 as *mut libc::c_void,
        0 as *mut libc::c_void,
        0 as *mut libc::c_void,
        XML_FROM_REGEXP as libc::c_int,
        XML_ERR_NO_MEMORY as libc::c_int,
        XML_ERR_FATAL,
        0 as *const libc::c_char,
        0 as libc::c_int,
        extra,
        regexp,
        0 as *const libc::c_char,
        0 as libc::c_int,
        0 as libc::c_int,
        b"Memory allocation failed : %s\n\0" as *const u8 as *const libc::c_char,
        extra,
    );
}
unsafe extern "C" fn xmlRegexpErrCompile(
    mut ctxt: xmlRegParserCtxtPtr,
    mut extra: *const libc::c_char,
) {
    let mut regexp: *const libc::c_char = 0 as *const libc::c_char;
    let mut idx: libc::c_int = 0 as libc::c_int;
    if !ctxt.is_null() {
        regexp = (*ctxt).string as *const libc::c_char;
        idx = ((*ctxt).cur).offset_from((*ctxt).string) as libc::c_long as libc::c_int;
        (*ctxt).error = XML_REGEXP_COMPILE_ERROR as libc::c_int;
    }
    __xmlRaiseError(
        None,
        None,
        0 as *mut libc::c_void,
        0 as *mut libc::c_void,
        0 as *mut libc::c_void,
        XML_FROM_REGEXP as libc::c_int,
        XML_REGEXP_COMPILE_ERROR as libc::c_int,
        XML_ERR_FATAL,
        0 as *const libc::c_char,
        0 as libc::c_int,
        extra,
        regexp,
        0 as *const libc::c_char,
        idx,
        0 as libc::c_int,
        b"failed to compile: %s\n\0" as *const u8 as *const libc::c_char,
        extra,
    );
}
unsafe extern "C" fn xmlRegCalloc2(
    mut dim1: size_t,
    mut dim2: size_t,
    mut elemSize: size_t,
) -> *mut libc::c_void {
    let mut totalSize: size_t = 0;
    let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
    if dim1 > (-(1 as libc::c_int) as size_t).wrapping_div(dim2).wrapping_div(elemSize) {
        return 0 as *mut libc::c_void;
    }
    totalSize = dim1.wrapping_mul(dim2).wrapping_mul(elemSize);
    ret = xmlMalloc.expect("non-null function pointer")(totalSize);
    if !ret.is_null() {
        memset(ret, 0 as libc::c_int, totalSize);
    }
    return ret;
}
unsafe extern "C" fn xmlRegEpxFromParse(mut ctxt: xmlRegParserCtxtPtr) -> xmlRegexpPtr {
    let mut current_block: u64;
    let mut ret: xmlRegexpPtr = 0 as *mut xmlRegexp;
    ret = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::std::mem::size_of::<xmlRegexp>() as libc::c_ulong) as xmlRegexpPtr;
    if ret.is_null() {
        xmlRegexpErrMemory(
            ctxt,
            b"compiling regexp\0" as *const u8 as *const libc::c_char,
        );
        return 0 as xmlRegexpPtr;
    }
    memset(
        ret as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<xmlRegexp>() as libc::c_ulong,
    );
    let ref mut fresh0 = (*ret).string;
    *fresh0 = (*ctxt).string;
    (*ret).nbStates = (*ctxt).nbStates;
    let ref mut fresh1 = (*ret).states;
    *fresh1 = (*ctxt).states;
    (*ret).nbAtoms = (*ctxt).nbAtoms;
    let ref mut fresh2 = (*ret).atoms;
    *fresh2 = (*ctxt).atoms;
    (*ret).nbCounters = (*ctxt).nbCounters;
    let ref mut fresh3 = (*ret).counters;
    *fresh3 = (*ctxt).counters;
    (*ret).determinist = (*ctxt).determinist;
    (*ret).flags = (*ctxt).flags;
    if (*ret).determinist == -(1 as libc::c_int) {
        xmlRegexpIsDeterminist(ret);
    }
    if (*ret).determinist != 0 as libc::c_int && (*ret).nbCounters == 0 as libc::c_int
        && (*ctxt).negs == 0 as libc::c_int && !((*ret).atoms).is_null()
        && !(*((*ret).atoms).offset(0 as libc::c_int as isize)).is_null()
        && (**((*ret).atoms).offset(0 as libc::c_int as isize)).type_0 as libc::c_uint
            == XML_REGEXP_STRING as libc::c_int as libc::c_uint
    {
        let mut i: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        let mut nbstates: libc::c_int = 0 as libc::c_int;
        let mut nbatoms: libc::c_int = 0 as libc::c_int;
        let mut stateRemap: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut stringRemap: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut transitions: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut transdata: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
        let mut stringMap: *mut *mut xmlChar = 0 as *mut *mut xmlChar;
        let mut value: *mut xmlChar = 0 as *mut xmlChar;
        stateRemap = xmlMalloc
            .expect(
                "non-null function pointer",
            )(
            ((*ret).nbStates as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        if stateRemap.is_null() {
            xmlRegexpErrMemory(
                ctxt,
                b"compiling regexp\0" as *const u8 as *const libc::c_char,
            );
            xmlFree.expect("non-null function pointer")(ret as *mut libc::c_void);
            return 0 as xmlRegexpPtr;
        }
        i = 0 as libc::c_int;
        while i < (*ret).nbStates {
            if !(*((*ret).states).offset(i as isize)).is_null() {
                *stateRemap.offset(i as isize) = nbstates;
                nbstates += 1;
            } else {
                *stateRemap.offset(i as isize) = -(1 as libc::c_int);
            }
            i += 1;
        }
        stringMap = xmlMalloc
            .expect(
                "non-null function pointer",
            )(
            ((*ret).nbAtoms as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                ),
        ) as *mut *mut xmlChar;
        if stringMap.is_null() {
            xmlRegexpErrMemory(
                ctxt,
                b"compiling regexp\0" as *const u8 as *const libc::c_char,
            );
            xmlFree.expect("non-null function pointer")(stateRemap as *mut libc::c_void);
            xmlFree.expect("non-null function pointer")(ret as *mut libc::c_void);
            return 0 as xmlRegexpPtr;
        }
        stringRemap = xmlMalloc
            .expect(
                "non-null function pointer",
            )(
            ((*ret).nbAtoms as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        if stringRemap.is_null() {
            xmlRegexpErrMemory(
                ctxt,
                b"compiling regexp\0" as *const u8 as *const libc::c_char,
            );
            xmlFree.expect("non-null function pointer")(stringMap as *mut libc::c_void);
            xmlFree.expect("non-null function pointer")(stateRemap as *mut libc::c_void);
            xmlFree.expect("non-null function pointer")(ret as *mut libc::c_void);
            return 0 as xmlRegexpPtr;
        }
        i = 0 as libc::c_int;
        while i < (*ret).nbAtoms {
            if (**((*ret).atoms).offset(i as isize)).type_0 as libc::c_uint
                == XML_REGEXP_STRING as libc::c_int as libc::c_uint
                && (**((*ret).atoms).offset(i as isize)).quant as libc::c_uint
                    == XML_REGEXP_QUANT_ONCE as libc::c_int as libc::c_uint
            {
                value = (**((*ret).atoms).offset(i as isize)).valuep as *mut xmlChar;
                j = 0 as libc::c_int;
                while j < nbatoms {
                    if xmlStrEqual(*stringMap.offset(j as isize), value) != 0 {
                        *stringRemap.offset(i as isize) = j;
                        break;
                    } else {
                        j += 1;
                    }
                }
                if j >= nbatoms {
                    *stringRemap.offset(i as isize) = nbatoms;
                    let ref mut fresh4 = *stringMap.offset(nbatoms as isize);
                    *fresh4 = xmlStrdup(value);
                    if (*stringMap.offset(nbatoms as isize)).is_null() {
                        i = 0 as libc::c_int;
                        while i < nbatoms {
                            xmlFree
                                .expect(
                                    "non-null function pointer",
                                )(*stringMap.offset(i as isize) as *mut libc::c_void);
                            i += 1;
                        }
                        xmlFree
                            .expect(
                                "non-null function pointer",
                            )(stringRemap as *mut libc::c_void);
                        xmlFree
                            .expect(
                                "non-null function pointer",
                            )(stringMap as *mut libc::c_void);
                        xmlFree
                            .expect(
                                "non-null function pointer",
                            )(stateRemap as *mut libc::c_void);
                        xmlFree
                            .expect(
                                "non-null function pointer",
                            )(ret as *mut libc::c_void);
                        return 0 as xmlRegexpPtr;
                    }
                    nbatoms += 1;
                }
            } else {
                xmlFree
                    .expect(
                        "non-null function pointer",
                    )(stateRemap as *mut libc::c_void);
                xmlFree
                    .expect(
                        "non-null function pointer",
                    )(stringRemap as *mut libc::c_void);
                i = 0 as libc::c_int;
                while i < nbatoms {
                    xmlFree
                        .expect(
                            "non-null function pointer",
                        )(*stringMap.offset(i as isize) as *mut libc::c_void);
                    i += 1;
                }
                xmlFree
                    .expect("non-null function pointer")(stringMap as *mut libc::c_void);
                xmlFree.expect("non-null function pointer")(ret as *mut libc::c_void);
                return 0 as xmlRegexpPtr;
            }
            i += 1;
        }
        transitions = xmlRegCalloc2(
            (nbstates + 1 as libc::c_int) as size_t,
            (nbatoms + 1 as libc::c_int) as size_t,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        ) as *mut libc::c_int;
        if transitions.is_null() {
            xmlFree.expect("non-null function pointer")(stateRemap as *mut libc::c_void);
            xmlFree
                .expect("non-null function pointer")(stringRemap as *mut libc::c_void);
            i = 0 as libc::c_int;
            while i < nbatoms {
                xmlFree
                    .expect(
                        "non-null function pointer",
                    )(*stringMap.offset(i as isize) as *mut libc::c_void);
                i += 1;
            }
            xmlFree.expect("non-null function pointer")(stringMap as *mut libc::c_void);
            xmlFree.expect("non-null function pointer")(ret as *mut libc::c_void);
            return 0 as xmlRegexpPtr;
        }
        transdata = 0 as *mut *mut libc::c_void;
        i = 0 as libc::c_int;
        's_444: loop {
            if !(i < (*ret).nbStates) {
                current_block = 2627991079881472758;
                break;
            }
            let mut stateno: libc::c_int = 0;
            let mut atomno: libc::c_int = 0;
            let mut targetno: libc::c_int = 0;
            let mut prev: libc::c_int = 0;
            let mut state: xmlRegStatePtr = 0 as *mut xmlRegState;
            let mut trans: xmlRegTransPtr = 0 as *mut xmlRegTrans;
            stateno = *stateRemap.offset(i as isize);
            if !(stateno == -(1 as libc::c_int)) {
                state = *((*ret).states).offset(i as isize);
                *transitions
                    .offset(
                        (stateno * (nbatoms + 1 as libc::c_int)) as isize,
                    ) = (*state).type_0 as libc::c_int;
                j = 0 as libc::c_int;
                while j < (*state).nbTrans {
                    trans = &mut *((*state).trans).offset(j as isize)
                        as *mut xmlRegTrans;
                    if !((*trans).to == -(1 as libc::c_int) || ((*trans).atom).is_null())
                    {
                        atomno = *stringRemap.offset((*(*trans).atom).no as isize);
                        if !((*(*trans).atom).data).is_null() && transdata.is_null() {
                            transdata = xmlRegCalloc2(
                                nbstates as size_t,
                                nbatoms as size_t,
                                ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                            ) as *mut *mut libc::c_void;
                            if transdata.is_null() {
                                xmlRegexpErrMemory(
                                    ctxt,
                                    b"compiling regexp\0" as *const u8 as *const libc::c_char,
                                );
                                break;
                            }
                        }
                        targetno = *stateRemap.offset((*trans).to as isize);
                        prev = *transitions
                            .offset(
                                (stateno * (nbatoms + 1 as libc::c_int) + atomno
                                    + 1 as libc::c_int) as isize,
                            );
                        if prev != 0 as libc::c_int {
                            if prev != targetno + 1 as libc::c_int {
                                (*ret).determinist = 0 as libc::c_int;
                                if !transdata.is_null() {
                                    xmlFree
                                        .expect(
                                            "non-null function pointer",
                                        )(transdata as *mut libc::c_void);
                                }
                                xmlFree
                                    .expect(
                                        "non-null function pointer",
                                    )(transitions as *mut libc::c_void);
                                xmlFree
                                    .expect(
                                        "non-null function pointer",
                                    )(stateRemap as *mut libc::c_void);
                                xmlFree
                                    .expect(
                                        "non-null function pointer",
                                    )(stringRemap as *mut libc::c_void);
                                i = 0 as libc::c_int;
                                while i < nbatoms {
                                    xmlFree
                                        .expect(
                                            "non-null function pointer",
                                        )(*stringMap.offset(i as isize) as *mut libc::c_void);
                                    i += 1;
                                }
                                xmlFree
                                    .expect(
                                        "non-null function pointer",
                                    )(stringMap as *mut libc::c_void);
                                current_block = 13635467803606088781;
                                break 's_444;
                            }
                        } else {
                            *transitions
                                .offset(
                                    (stateno * (nbatoms + 1 as libc::c_int) + atomno
                                        + 1 as libc::c_int) as isize,
                                ) = targetno + 1 as libc::c_int;
                            if !transdata.is_null() {
                                let ref mut fresh5 = *transdata
                                    .offset((stateno * nbatoms + atomno) as isize);
                                *fresh5 = (*(*trans).atom).data;
                            }
                        }
                    }
                    j += 1;
                }
            }
            i += 1;
        }
        match current_block {
            13635467803606088781 => {}
            _ => {
                (*ret).determinist = 1 as libc::c_int;
                if !((*ret).states).is_null() {
                    i = 0 as libc::c_int;
                    while i < (*ret).nbStates {
                        xmlRegFreeState(*((*ret).states).offset(i as isize));
                        i += 1;
                    }
                    xmlFree
                        .expect(
                            "non-null function pointer",
                        )((*ret).states as *mut libc::c_void);
                }
                let ref mut fresh6 = (*ret).states;
                *fresh6 = 0 as *mut xmlRegStatePtr;
                (*ret).nbStates = 0 as libc::c_int;
                if !((*ret).atoms).is_null() {
                    i = 0 as libc::c_int;
                    while i < (*ret).nbAtoms {
                        xmlRegFreeAtom(*((*ret).atoms).offset(i as isize));
                        i += 1;
                    }
                    xmlFree
                        .expect(
                            "non-null function pointer",
                        )((*ret).atoms as *mut libc::c_void);
                }
                let ref mut fresh7 = (*ret).atoms;
                *fresh7 = 0 as *mut xmlRegAtomPtr;
                (*ret).nbAtoms = 0 as libc::c_int;
                let ref mut fresh8 = (*ret).compact;
                *fresh8 = transitions;
                let ref mut fresh9 = (*ret).transdata;
                *fresh9 = transdata;
                let ref mut fresh10 = (*ret).stringMap;
                *fresh10 = stringMap;
                (*ret).nbstrings = nbatoms;
                (*ret).nbstates = nbstates;
                xmlFree
                    .expect(
                        "non-null function pointer",
                    )(stateRemap as *mut libc::c_void);
                xmlFree
                    .expect(
                        "non-null function pointer",
                    )(stringRemap as *mut libc::c_void);
            }
        }
    }
    let ref mut fresh11 = (*ctxt).string;
    *fresh11 = 0 as *mut xmlChar;
    (*ctxt).nbStates = 0 as libc::c_int;
    let ref mut fresh12 = (*ctxt).states;
    *fresh12 = 0 as *mut xmlRegStatePtr;
    (*ctxt).nbAtoms = 0 as libc::c_int;
    let ref mut fresh13 = (*ctxt).atoms;
    *fresh13 = 0 as *mut xmlRegAtomPtr;
    (*ctxt).nbCounters = 0 as libc::c_int;
    let ref mut fresh14 = (*ctxt).counters;
    *fresh14 = 0 as *mut xmlRegCounter;
    return ret;
}
unsafe extern "C" fn xmlRegNewParserCtxt(
    mut string: *const xmlChar,
) -> xmlRegParserCtxtPtr {
    let mut ret: xmlRegParserCtxtPtr = 0 as *mut xmlRegParserCtxt;
    ret = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::std::mem::size_of::<xmlRegParserCtxt>() as libc::c_ulong)
        as xmlRegParserCtxtPtr;
    if ret.is_null() {
        return 0 as xmlRegParserCtxtPtr;
    }
    memset(
        ret as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<xmlRegParserCtxt>() as libc::c_ulong,
    );
    if !string.is_null() {
        let ref mut fresh15 = (*ret).string;
        *fresh15 = xmlStrdup(string);
    }
    let ref mut fresh16 = (*ret).cur;
    *fresh16 = (*ret).string;
    (*ret).neg = 0 as libc::c_int;
    (*ret).negs = 0 as libc::c_int;
    (*ret).error = 0 as libc::c_int;
    (*ret).determinist = -(1 as libc::c_int);
    return ret;
}
unsafe extern "C" fn xmlRegNewRange(
    mut ctxt: xmlRegParserCtxtPtr,
    mut neg: libc::c_int,
    mut type_0: xmlRegAtomType,
    mut start: libc::c_int,
    mut end: libc::c_int,
) -> xmlRegRangePtr {
    let mut ret: xmlRegRangePtr = 0 as *mut xmlRegRange;
    ret = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::std::mem::size_of::<xmlRegRange>() as libc::c_ulong) as xmlRegRangePtr;
    if ret.is_null() {
        xmlRegexpErrMemory(
            ctxt,
            b"allocating range\0" as *const u8 as *const libc::c_char,
        );
        return 0 as xmlRegRangePtr;
    }
    (*ret).neg = neg;
    (*ret).type_0 = type_0;
    (*ret).start = start;
    (*ret).end = end;
    return ret;
}
unsafe extern "C" fn xmlRegFreeRange(mut range: xmlRegRangePtr) {
    if range.is_null() {
        return;
    }
    if !((*range).blockName).is_null() {
        xmlFree
            .expect(
                "non-null function pointer",
            )((*range).blockName as *mut libc::c_void);
    }
    xmlFree.expect("non-null function pointer")(range as *mut libc::c_void);
}
unsafe extern "C" fn xmlRegCopyRange(
    mut ctxt: xmlRegParserCtxtPtr,
    mut range: xmlRegRangePtr,
) -> xmlRegRangePtr {
    let mut ret: xmlRegRangePtr = 0 as *mut xmlRegRange;
    if range.is_null() {
        return 0 as xmlRegRangePtr;
    }
    ret = xmlRegNewRange(
        ctxt,
        (*range).neg,
        (*range).type_0,
        (*range).start,
        (*range).end,
    );
    if ret.is_null() {
        return 0 as xmlRegRangePtr;
    }
    if !((*range).blockName).is_null() {
        let ref mut fresh17 = (*ret).blockName;
        *fresh17 = xmlStrdup((*range).blockName);
        if ((*ret).blockName).is_null() {
            xmlRegexpErrMemory(
                ctxt,
                b"allocating range\0" as *const u8 as *const libc::c_char,
            );
            xmlRegFreeRange(ret);
            return 0 as xmlRegRangePtr;
        }
    }
    return ret;
}
unsafe extern "C" fn xmlRegNewAtom(
    mut ctxt: xmlRegParserCtxtPtr,
    mut type_0: xmlRegAtomType,
) -> xmlRegAtomPtr {
    let mut ret: xmlRegAtomPtr = 0 as *mut xmlRegAtom;
    ret = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::std::mem::size_of::<xmlRegAtom>() as libc::c_ulong) as xmlRegAtomPtr;
    if ret.is_null() {
        xmlRegexpErrMemory(
            ctxt,
            b"allocating atom\0" as *const u8 as *const libc::c_char,
        );
        return 0 as xmlRegAtomPtr;
    }
    memset(
        ret as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<xmlRegAtom>() as libc::c_ulong,
    );
    (*ret).type_0 = type_0;
    (*ret).quant = XML_REGEXP_QUANT_ONCE;
    (*ret).min = 0 as libc::c_int;
    (*ret).max = 0 as libc::c_int;
    return ret;
}
unsafe extern "C" fn xmlRegFreeAtom(mut atom: xmlRegAtomPtr) {
    let mut i: libc::c_int = 0;
    if atom.is_null() {
        return;
    }
    i = 0 as libc::c_int;
    while i < (*atom).nbRanges {
        xmlRegFreeRange(*((*atom).ranges).offset(i as isize));
        i += 1;
    }
    if !((*atom).ranges).is_null() {
        xmlFree.expect("non-null function pointer")((*atom).ranges as *mut libc::c_void);
    }
    if (*atom).type_0 as libc::c_uint == XML_REGEXP_STRING as libc::c_int as libc::c_uint
        && !((*atom).valuep).is_null()
    {
        xmlFree.expect("non-null function pointer")((*atom).valuep);
    }
    if (*atom).type_0 as libc::c_uint == XML_REGEXP_STRING as libc::c_int as libc::c_uint
        && !((*atom).valuep2).is_null()
    {
        xmlFree.expect("non-null function pointer")((*atom).valuep2);
    }
    if (*atom).type_0 as libc::c_uint
        == XML_REGEXP_BLOCK_NAME as libc::c_int as libc::c_uint
        && !((*atom).valuep).is_null()
    {
        xmlFree.expect("non-null function pointer")((*atom).valuep);
    }
    xmlFree.expect("non-null function pointer")(atom as *mut libc::c_void);
}
unsafe extern "C" fn xmlRegCopyAtom(
    mut ctxt: xmlRegParserCtxtPtr,
    mut atom: xmlRegAtomPtr,
) -> xmlRegAtomPtr {
    let mut current_block: u64;
    let mut ret: xmlRegAtomPtr = 0 as *mut xmlRegAtom;
    ret = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::std::mem::size_of::<xmlRegAtom>() as libc::c_ulong) as xmlRegAtomPtr;
    if ret.is_null() {
        xmlRegexpErrMemory(ctxt, b"copying atom\0" as *const u8 as *const libc::c_char);
        return 0 as xmlRegAtomPtr;
    }
    memset(
        ret as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<xmlRegAtom>() as libc::c_ulong,
    );
    (*ret).type_0 = (*atom).type_0;
    (*ret).quant = (*atom).quant;
    (*ret).min = (*atom).min;
    (*ret).max = (*atom).max;
    if (*atom).nbRanges > 0 as libc::c_int {
        let mut i: libc::c_int = 0;
        let ref mut fresh18 = (*ret).ranges;
        *fresh18 = xmlMalloc
            .expect(
                "non-null function pointer",
            )(
            (::std::mem::size_of::<xmlRegRangePtr>() as libc::c_ulong)
                .wrapping_mul((*atom).nbRanges as libc::c_ulong),
        ) as *mut xmlRegRangePtr;
        if ((*ret).ranges).is_null() {
            xmlRegexpErrMemory(
                ctxt,
                b"copying atom\0" as *const u8 as *const libc::c_char,
            );
            current_block = 16366640603525550618;
        } else {
            i = 0 as libc::c_int;
            loop {
                if !(i < (*atom).nbRanges) {
                    current_block = 12124785117276362961;
                    break;
                }
                let ref mut fresh19 = *((*ret).ranges).offset(i as isize);
                *fresh19 = xmlRegCopyRange(ctxt, *((*atom).ranges).offset(i as isize));
                if (*((*ret).ranges).offset(i as isize)).is_null() {
                    current_block = 16366640603525550618;
                    break;
                }
                (*ret).nbRanges = i + 1 as libc::c_int;
                i += 1;
            }
        }
        match current_block {
            12124785117276362961 => {}
            _ => {
                xmlRegFreeAtom(ret);
                return 0 as xmlRegAtomPtr;
            }
        }
    }
    return ret;
}
unsafe extern "C" fn xmlRegNewState(mut ctxt: xmlRegParserCtxtPtr) -> xmlRegStatePtr {
    let mut ret: xmlRegStatePtr = 0 as *mut xmlRegState;
    ret = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::std::mem::size_of::<xmlRegState>() as libc::c_ulong) as xmlRegStatePtr;
    if ret.is_null() {
        xmlRegexpErrMemory(
            ctxt,
            b"allocating state\0" as *const u8 as *const libc::c_char,
        );
        return 0 as xmlRegStatePtr;
    }
    memset(
        ret as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<xmlRegState>() as libc::c_ulong,
    );
    (*ret).type_0 = XML_REGEXP_TRANS_STATE;
    (*ret).mark = XML_REGEXP_MARK_NORMAL;
    return ret;
}
unsafe extern "C" fn xmlRegFreeState(mut state: xmlRegStatePtr) {
    if state.is_null() {
        return;
    }
    if !((*state).trans).is_null() {
        xmlFree.expect("non-null function pointer")((*state).trans as *mut libc::c_void);
    }
    if !((*state).transTo).is_null() {
        xmlFree
            .expect("non-null function pointer")((*state).transTo as *mut libc::c_void);
    }
    xmlFree.expect("non-null function pointer")(state as *mut libc::c_void);
}
unsafe extern "C" fn xmlRegFreeParserCtxt(mut ctxt: xmlRegParserCtxtPtr) {
    let mut i: libc::c_int = 0;
    if ctxt.is_null() {
        return;
    }
    if !((*ctxt).string).is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).string as *mut libc::c_void);
    }
    if !((*ctxt).states).is_null() {
        i = 0 as libc::c_int;
        while i < (*ctxt).nbStates {
            xmlRegFreeState(*((*ctxt).states).offset(i as isize));
            i += 1;
        }
        xmlFree.expect("non-null function pointer")((*ctxt).states as *mut libc::c_void);
    }
    if !((*ctxt).atoms).is_null() {
        i = 0 as libc::c_int;
        while i < (*ctxt).nbAtoms {
            xmlRegFreeAtom(*((*ctxt).atoms).offset(i as isize));
            i += 1;
        }
        xmlFree.expect("non-null function pointer")((*ctxt).atoms as *mut libc::c_void);
    }
    if !((*ctxt).counters).is_null() {
        xmlFree
            .expect("non-null function pointer")((*ctxt).counters as *mut libc::c_void);
    }
    xmlFree.expect("non-null function pointer")(ctxt as *mut libc::c_void);
}
unsafe extern "C" fn xmlRegPrintAtomType(
    mut output: *mut FILE,
    mut type_0: xmlRegAtomType,
) {
    match type_0 as libc::c_uint {
        1 => {
            fprintf(output, b"epsilon \0" as *const u8 as *const libc::c_char);
        }
        2 => {
            fprintf(output, b"charval \0" as *const u8 as *const libc::c_char);
        }
        3 => {
            fprintf(output, b"ranges \0" as *const u8 as *const libc::c_char);
        }
        4 => {
            fprintf(output, b"subexpr \0" as *const u8 as *const libc::c_char);
        }
        5 => {
            fprintf(output, b"string \0" as *const u8 as *const libc::c_char);
        }
        6 => {
            fprintf(output, b"anychar \0" as *const u8 as *const libc::c_char);
        }
        7 => {
            fprintf(output, b"anyspace \0" as *const u8 as *const libc::c_char);
        }
        8 => {
            fprintf(output, b"notspace \0" as *const u8 as *const libc::c_char);
        }
        9 => {
            fprintf(output, b"initname \0" as *const u8 as *const libc::c_char);
        }
        10 => {
            fprintf(output, b"notinitname \0" as *const u8 as *const libc::c_char);
        }
        11 => {
            fprintf(output, b"namechar \0" as *const u8 as *const libc::c_char);
        }
        12 => {
            fprintf(output, b"notnamechar \0" as *const u8 as *const libc::c_char);
        }
        13 => {
            fprintf(output, b"decimal \0" as *const u8 as *const libc::c_char);
        }
        14 => {
            fprintf(output, b"notdecimal \0" as *const u8 as *const libc::c_char);
        }
        15 => {
            fprintf(output, b"realchar \0" as *const u8 as *const libc::c_char);
        }
        16 => {
            fprintf(output, b"notrealchar \0" as *const u8 as *const libc::c_char);
        }
        100 => {
            fprintf(output, b"LETTER \0" as *const u8 as *const libc::c_char);
        }
        101 => {
            fprintf(output, b"LETTER_UPPERCASE \0" as *const u8 as *const libc::c_char);
        }
        102 => {
            fprintf(output, b"LETTER_LOWERCASE \0" as *const u8 as *const libc::c_char);
        }
        103 => {
            fprintf(output, b"LETTER_TITLECASE \0" as *const u8 as *const libc::c_char);
        }
        104 => {
            fprintf(output, b"LETTER_MODIFIER \0" as *const u8 as *const libc::c_char);
        }
        105 => {
            fprintf(output, b"LETTER_OTHERS \0" as *const u8 as *const libc::c_char);
        }
        106 => {
            fprintf(output, b"MARK \0" as *const u8 as *const libc::c_char);
        }
        107 => {
            fprintf(output, b"MARK_NONSPACING \0" as *const u8 as *const libc::c_char);
        }
        108 => {
            fprintf(
                output,
                b"MARK_SPACECOMBINING \0" as *const u8 as *const libc::c_char,
            );
        }
        109 => {
            fprintf(output, b"MARK_ENCLOSING \0" as *const u8 as *const libc::c_char);
        }
        110 => {
            fprintf(output, b"NUMBER \0" as *const u8 as *const libc::c_char);
        }
        111 => {
            fprintf(output, b"NUMBER_DECIMAL \0" as *const u8 as *const libc::c_char);
        }
        112 => {
            fprintf(output, b"NUMBER_LETTER \0" as *const u8 as *const libc::c_char);
        }
        113 => {
            fprintf(output, b"NUMBER_OTHERS \0" as *const u8 as *const libc::c_char);
        }
        114 => {
            fprintf(output, b"PUNCT \0" as *const u8 as *const libc::c_char);
        }
        115 => {
            fprintf(output, b"PUNCT_CONNECTOR \0" as *const u8 as *const libc::c_char);
        }
        116 => {
            fprintf(output, b"PUNCT_DASH \0" as *const u8 as *const libc::c_char);
        }
        117 => {
            fprintf(output, b"PUNCT_OPEN \0" as *const u8 as *const libc::c_char);
        }
        118 => {
            fprintf(output, b"PUNCT_CLOSE \0" as *const u8 as *const libc::c_char);
        }
        119 => {
            fprintf(output, b"PUNCT_INITQUOTE \0" as *const u8 as *const libc::c_char);
        }
        120 => {
            fprintf(output, b"PUNCT_FINQUOTE \0" as *const u8 as *const libc::c_char);
        }
        121 => {
            fprintf(output, b"PUNCT_OTHERS \0" as *const u8 as *const libc::c_char);
        }
        122 => {
            fprintf(output, b"SEPAR \0" as *const u8 as *const libc::c_char);
        }
        123 => {
            fprintf(output, b"SEPAR_SPACE \0" as *const u8 as *const libc::c_char);
        }
        124 => {
            fprintf(output, b"SEPAR_LINE \0" as *const u8 as *const libc::c_char);
        }
        125 => {
            fprintf(output, b"SEPAR_PARA \0" as *const u8 as *const libc::c_char);
        }
        126 => {
            fprintf(output, b"SYMBOL \0" as *const u8 as *const libc::c_char);
        }
        127 => {
            fprintf(output, b"SYMBOL_MATH \0" as *const u8 as *const libc::c_char);
        }
        128 => {
            fprintf(output, b"SYMBOL_CURRENCY \0" as *const u8 as *const libc::c_char);
        }
        129 => {
            fprintf(output, b"SYMBOL_MODIFIER \0" as *const u8 as *const libc::c_char);
        }
        130 => {
            fprintf(output, b"SYMBOL_OTHERS \0" as *const u8 as *const libc::c_char);
        }
        131 => {
            fprintf(output, b"OTHER \0" as *const u8 as *const libc::c_char);
        }
        132 => {
            fprintf(output, b"OTHER_CONTROL \0" as *const u8 as *const libc::c_char);
        }
        133 => {
            fprintf(output, b"OTHER_FORMAT \0" as *const u8 as *const libc::c_char);
        }
        134 => {
            fprintf(output, b"OTHER_PRIVATE \0" as *const u8 as *const libc::c_char);
        }
        135 => {
            fprintf(output, b"OTHER_NA \0" as *const u8 as *const libc::c_char);
        }
        136 => {
            fprintf(output, b"BLOCK \0" as *const u8 as *const libc::c_char);
        }
        _ => {}
    };
}
unsafe extern "C" fn xmlRegPrintQuantType(
    mut output: *mut FILE,
    mut type_0: xmlRegQuantType,
) {
    match type_0 as libc::c_uint {
        1 => {
            fprintf(output, b"epsilon \0" as *const u8 as *const libc::c_char);
        }
        2 => {
            fprintf(output, b"once \0" as *const u8 as *const libc::c_char);
        }
        3 => {
            fprintf(output, b"? \0" as *const u8 as *const libc::c_char);
        }
        4 => {
            fprintf(output, b"* \0" as *const u8 as *const libc::c_char);
        }
        5 => {
            fprintf(output, b"+ \0" as *const u8 as *const libc::c_char);
        }
        8 => {
            fprintf(output, b"range \0" as *const u8 as *const libc::c_char);
        }
        6 => {
            fprintf(output, b"onceonly \0" as *const u8 as *const libc::c_char);
        }
        7 => {
            fprintf(output, b"all \0" as *const u8 as *const libc::c_char);
        }
        _ => {}
    };
}
unsafe extern "C" fn xmlRegPrintRange(mut output: *mut FILE, mut range: xmlRegRangePtr) {
    fprintf(output, b"  range: \0" as *const u8 as *const libc::c_char);
    if (*range).neg != 0 {
        fprintf(output, b"negative \0" as *const u8 as *const libc::c_char);
    }
    xmlRegPrintAtomType(output, (*range).type_0);
    fprintf(
        output,
        b"%c - %c\n\0" as *const u8 as *const libc::c_char,
        (*range).start,
        (*range).end,
    );
}
unsafe extern "C" fn xmlRegPrintAtom(mut output: *mut FILE, mut atom: xmlRegAtomPtr) {
    fprintf(output, b" atom: \0" as *const u8 as *const libc::c_char);
    if atom.is_null() {
        fprintf(output, b"NULL\n\0" as *const u8 as *const libc::c_char);
        return;
    }
    if (*atom).neg != 0 {
        fprintf(output, b"not \0" as *const u8 as *const libc::c_char);
    }
    xmlRegPrintAtomType(output, (*atom).type_0);
    xmlRegPrintQuantType(output, (*atom).quant);
    if (*atom).quant as libc::c_uint
        == XML_REGEXP_QUANT_RANGE as libc::c_int as libc::c_uint
    {
        fprintf(
            output,
            b"%d-%d \0" as *const u8 as *const libc::c_char,
            (*atom).min,
            (*atom).max,
        );
    }
    if (*atom).type_0 as libc::c_uint == XML_REGEXP_STRING as libc::c_int as libc::c_uint
    {
        fprintf(
            output,
            b"'%s' \0" as *const u8 as *const libc::c_char,
            (*atom).valuep as *mut libc::c_char,
        );
    }
    if (*atom).type_0 as libc::c_uint
        == XML_REGEXP_CHARVAL as libc::c_int as libc::c_uint
    {
        fprintf(
            output,
            b"char %c\n\0" as *const u8 as *const libc::c_char,
            (*atom).codepoint,
        );
    } else if (*atom).type_0 as libc::c_uint
            == XML_REGEXP_RANGES as libc::c_int as libc::c_uint
        {
        let mut i: libc::c_int = 0;
        fprintf(
            output,
            b"%d entries\n\0" as *const u8 as *const libc::c_char,
            (*atom).nbRanges,
        );
        i = 0 as libc::c_int;
        while i < (*atom).nbRanges {
            xmlRegPrintRange(output, *((*atom).ranges).offset(i as isize));
            i += 1;
        }
    } else if (*atom).type_0 as libc::c_uint
            == XML_REGEXP_SUBREG as libc::c_int as libc::c_uint
        {
        fprintf(
            output,
            b"start %d end %d\n\0" as *const u8 as *const libc::c_char,
            (*(*atom).start).no,
            (*(*atom).stop).no,
        );
    } else {
        fprintf(output, b"\n\0" as *const u8 as *const libc::c_char);
    };
}
unsafe extern "C" fn xmlRegPrintTrans(mut output: *mut FILE, mut trans: xmlRegTransPtr) {
    fprintf(output, b"  trans: \0" as *const u8 as *const libc::c_char);
    if trans.is_null() {
        fprintf(output, b"NULL\n\0" as *const u8 as *const libc::c_char);
        return;
    }
    if (*trans).to < 0 as libc::c_int {
        fprintf(output, b"removed\n\0" as *const u8 as *const libc::c_char);
        return;
    }
    if (*trans).nd != 0 as libc::c_int {
        if (*trans).nd == 2 as libc::c_int {
            fprintf(
                output,
                b"last not determinist, \0" as *const u8 as *const libc::c_char,
            );
        } else {
            fprintf(output, b"not determinist, \0" as *const u8 as *const libc::c_char);
        }
    }
    if (*trans).counter >= 0 as libc::c_int {
        fprintf(
            output,
            b"counted %d, \0" as *const u8 as *const libc::c_char,
            (*trans).counter,
        );
    }
    if (*trans).count == 0x123456 as libc::c_int {
        fprintf(output, b"all transition, \0" as *const u8 as *const libc::c_char);
    } else if (*trans).count >= 0 as libc::c_int {
        fprintf(
            output,
            b"count based %d, \0" as *const u8 as *const libc::c_char,
            (*trans).count,
        );
    }
    if ((*trans).atom).is_null() {
        fprintf(
            output,
            b"epsilon to %d\n\0" as *const u8 as *const libc::c_char,
            (*trans).to,
        );
        return;
    }
    if (*(*trans).atom).type_0 as libc::c_uint
        == XML_REGEXP_CHARVAL as libc::c_int as libc::c_uint
    {
        fprintf(
            output,
            b"char %c \0" as *const u8 as *const libc::c_char,
            (*(*trans).atom).codepoint,
        );
    }
    fprintf(
        output,
        b"atom %d, to %d\n\0" as *const u8 as *const libc::c_char,
        (*(*trans).atom).no,
        (*trans).to,
    );
}
unsafe extern "C" fn xmlRegPrintState(mut output: *mut FILE, mut state: xmlRegStatePtr) {
    let mut i: libc::c_int = 0;
    fprintf(output, b" state: \0" as *const u8 as *const libc::c_char);
    if state.is_null() {
        fprintf(output, b"NULL\n\0" as *const u8 as *const libc::c_char);
        return;
    }
    if (*state).type_0 as libc::c_uint
        == XML_REGEXP_START_STATE as libc::c_int as libc::c_uint
    {
        fprintf(output, b"START \0" as *const u8 as *const libc::c_char);
    }
    if (*state).type_0 as libc::c_uint
        == XML_REGEXP_FINAL_STATE as libc::c_int as libc::c_uint
    {
        fprintf(output, b"FINAL \0" as *const u8 as *const libc::c_char);
    }
    fprintf(
        output,
        b"%d, %d transitions:\n\0" as *const u8 as *const libc::c_char,
        (*state).no,
        (*state).nbTrans,
    );
    i = 0 as libc::c_int;
    while i < (*state).nbTrans {
        xmlRegPrintTrans(output, &mut *((*state).trans).offset(i as isize));
        i += 1;
    }
}
unsafe extern "C" fn xmlRegAtomAddRange(
    mut ctxt: xmlRegParserCtxtPtr,
    mut atom: xmlRegAtomPtr,
    mut neg: libc::c_int,
    mut type_0: xmlRegAtomType,
    mut start: libc::c_int,
    mut end: libc::c_int,
    mut blockName: *mut xmlChar,
) {
    let mut range: xmlRegRangePtr = 0 as *mut xmlRegRange;
    if atom.is_null() {
        (*ctxt).error = XML_REGEXP_COMPILE_ERROR as libc::c_int;
        xmlRegexpErrCompile(
            ctxt,
            b"add range: atom is NULL\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if (*atom).type_0 as libc::c_uint != XML_REGEXP_RANGES as libc::c_int as libc::c_uint
    {
        (*ctxt).error = XML_REGEXP_COMPILE_ERROR as libc::c_int;
        xmlRegexpErrCompile(
            ctxt,
            b"add range: atom is not ranges\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if (*atom).maxRanges == 0 as libc::c_int {
        (*atom).maxRanges = 4 as libc::c_int;
        let ref mut fresh20 = (*atom).ranges;
        *fresh20 = xmlMalloc
            .expect(
                "non-null function pointer",
            )(
            ((*atom).maxRanges as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<xmlRegRangePtr>() as libc::c_ulong),
        ) as *mut xmlRegRangePtr;
        if ((*atom).ranges).is_null() {
            xmlRegexpErrMemory(
                ctxt,
                b"adding ranges\0" as *const u8 as *const libc::c_char,
            );
            (*atom).maxRanges = 0 as libc::c_int;
            return;
        }
    } else if (*atom).nbRanges >= (*atom).maxRanges {
        let mut tmp: *mut xmlRegRangePtr = 0 as *mut xmlRegRangePtr;
        (*atom).maxRanges *= 2 as libc::c_int;
        tmp = xmlRealloc
            .expect(
                "non-null function pointer",
            )(
            (*atom).ranges as *mut libc::c_void,
            ((*atom).maxRanges as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<xmlRegRangePtr>() as libc::c_ulong),
        ) as *mut xmlRegRangePtr;
        if tmp.is_null() {
            xmlRegexpErrMemory(
                ctxt,
                b"adding ranges\0" as *const u8 as *const libc::c_char,
            );
            (*atom).maxRanges /= 2 as libc::c_int;
            return;
        }
        let ref mut fresh21 = (*atom).ranges;
        *fresh21 = tmp;
    }
    range = xmlRegNewRange(ctxt, neg, type_0, start, end);
    if range.is_null() {
        return;
    }
    let ref mut fresh22 = (*range).blockName;
    *fresh22 = blockName;
    let ref mut fresh23 = (*atom).nbRanges;
    let fresh24 = *fresh23;
    *fresh23 = *fresh23 + 1;
    let ref mut fresh25 = *((*atom).ranges).offset(fresh24 as isize);
    *fresh25 = range;
}
unsafe extern "C" fn xmlRegGetCounter(mut ctxt: xmlRegParserCtxtPtr) -> libc::c_int {
    if (*ctxt).maxCounters == 0 as libc::c_int {
        (*ctxt).maxCounters = 4 as libc::c_int;
        let ref mut fresh26 = (*ctxt).counters;
        *fresh26 = xmlMalloc
            .expect(
                "non-null function pointer",
            )(
            ((*ctxt).maxCounters as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<xmlRegCounter>() as libc::c_ulong),
        ) as *mut xmlRegCounter;
        if ((*ctxt).counters).is_null() {
            xmlRegexpErrMemory(
                ctxt,
                b"allocating counter\0" as *const u8 as *const libc::c_char,
            );
            (*ctxt).maxCounters = 0 as libc::c_int;
            return -(1 as libc::c_int);
        }
    } else if (*ctxt).nbCounters >= (*ctxt).maxCounters {
        let mut tmp: *mut xmlRegCounter = 0 as *mut xmlRegCounter;
        (*ctxt).maxCounters *= 2 as libc::c_int;
        tmp = xmlRealloc
            .expect(
                "non-null function pointer",
            )(
            (*ctxt).counters as *mut libc::c_void,
            ((*ctxt).maxCounters as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<xmlRegCounter>() as libc::c_ulong),
        ) as *mut xmlRegCounter;
        if tmp.is_null() {
            xmlRegexpErrMemory(
                ctxt,
                b"allocating counter\0" as *const u8 as *const libc::c_char,
            );
            (*ctxt).maxCounters /= 2 as libc::c_int;
            return -(1 as libc::c_int);
        }
        let ref mut fresh27 = (*ctxt).counters;
        *fresh27 = tmp;
    }
    (*((*ctxt).counters).offset((*ctxt).nbCounters as isize)).min = -(1 as libc::c_int);
    (*((*ctxt).counters).offset((*ctxt).nbCounters as isize)).max = -(1 as libc::c_int);
    let ref mut fresh28 = (*ctxt).nbCounters;
    let fresh29 = *fresh28;
    *fresh28 = *fresh28 + 1;
    return fresh29;
}
unsafe extern "C" fn xmlRegAtomPush(
    mut ctxt: xmlRegParserCtxtPtr,
    mut atom: xmlRegAtomPtr,
) -> libc::c_int {
    if atom.is_null() {
        (*ctxt).error = XML_REGEXP_COMPILE_ERROR as libc::c_int;
        xmlRegexpErrCompile(
            ctxt,
            b"atom push: atom is NULL\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (*ctxt).maxAtoms == 0 as libc::c_int {
        (*ctxt).maxAtoms = 4 as libc::c_int;
        let ref mut fresh30 = (*ctxt).atoms;
        *fresh30 = xmlMalloc
            .expect(
                "non-null function pointer",
            )(
            ((*ctxt).maxAtoms as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<xmlRegAtomPtr>() as libc::c_ulong),
        ) as *mut xmlRegAtomPtr;
        if ((*ctxt).atoms).is_null() {
            xmlRegexpErrMemory(
                ctxt,
                b"pushing atom\0" as *const u8 as *const libc::c_char,
            );
            (*ctxt).maxAtoms = 0 as libc::c_int;
            return -(1 as libc::c_int);
        }
    } else if (*ctxt).nbAtoms >= (*ctxt).maxAtoms {
        let mut tmp: *mut xmlRegAtomPtr = 0 as *mut xmlRegAtomPtr;
        (*ctxt).maxAtoms *= 2 as libc::c_int;
        tmp = xmlRealloc
            .expect(
                "non-null function pointer",
            )(
            (*ctxt).atoms as *mut libc::c_void,
            ((*ctxt).maxAtoms as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<xmlRegAtomPtr>() as libc::c_ulong),
        ) as *mut xmlRegAtomPtr;
        if tmp.is_null() {
            xmlRegexpErrMemory(
                ctxt,
                b"allocating counter\0" as *const u8 as *const libc::c_char,
            );
            (*ctxt).maxAtoms /= 2 as libc::c_int;
            return -(1 as libc::c_int);
        }
        let ref mut fresh31 = (*ctxt).atoms;
        *fresh31 = tmp;
    }
    (*atom).no = (*ctxt).nbAtoms;
    let ref mut fresh32 = (*ctxt).nbAtoms;
    let fresh33 = *fresh32;
    *fresh32 = *fresh32 + 1;
    let ref mut fresh34 = *((*ctxt).atoms).offset(fresh33 as isize);
    *fresh34 = atom;
    return 0 as libc::c_int;
}
unsafe extern "C" fn xmlRegStateAddTransTo(
    mut ctxt: xmlRegParserCtxtPtr,
    mut target: xmlRegStatePtr,
    mut from: libc::c_int,
) {
    if (*target).maxTransTo == 0 as libc::c_int {
        (*target).maxTransTo = 8 as libc::c_int;
        let ref mut fresh35 = (*target).transTo;
        *fresh35 = xmlMalloc
            .expect(
                "non-null function pointer",
            )(
            ((*target).maxTransTo as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        if ((*target).transTo).is_null() {
            xmlRegexpErrMemory(
                ctxt,
                b"adding transition\0" as *const u8 as *const libc::c_char,
            );
            (*target).maxTransTo = 0 as libc::c_int;
            return;
        }
    } else if (*target).nbTransTo >= (*target).maxTransTo {
        let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
        (*target).maxTransTo *= 2 as libc::c_int;
        tmp = xmlRealloc
            .expect(
                "non-null function pointer",
            )(
            (*target).transTo as *mut libc::c_void,
            ((*target).maxTransTo as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        if tmp.is_null() {
            xmlRegexpErrMemory(
                ctxt,
                b"adding transition\0" as *const u8 as *const libc::c_char,
            );
            (*target).maxTransTo /= 2 as libc::c_int;
            return;
        }
        let ref mut fresh36 = (*target).transTo;
        *fresh36 = tmp;
    }
    *((*target).transTo).offset((*target).nbTransTo as isize) = from;
    let ref mut fresh37 = (*target).nbTransTo;
    *fresh37 += 1;
}
unsafe extern "C" fn xmlRegStateAddTrans(
    mut ctxt: xmlRegParserCtxtPtr,
    mut state: xmlRegStatePtr,
    mut atom: xmlRegAtomPtr,
    mut target: xmlRegStatePtr,
    mut counter: libc::c_int,
    mut count: libc::c_int,
) {
    let mut nrtrans: libc::c_int = 0;
    if state.is_null() {
        (*ctxt).error = XML_REGEXP_COMPILE_ERROR as libc::c_int;
        xmlRegexpErrCompile(
            ctxt,
            b"add state: state is NULL\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if target.is_null() {
        (*ctxt).error = XML_REGEXP_COMPILE_ERROR as libc::c_int;
        xmlRegexpErrCompile(
            ctxt,
            b"add state: target is NULL\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    nrtrans = (*state).nbTrans - 1 as libc::c_int;
    while nrtrans >= 0 as libc::c_int {
        let mut trans: xmlRegTransPtr = &mut *((*state).trans).offset(nrtrans as isize)
            as *mut xmlRegTrans;
        if (*trans).atom == atom && (*trans).to == (*target).no
            && (*trans).counter == counter && (*trans).count == count
        {
            return;
        }
        nrtrans -= 1;
    }
    if (*state).maxTrans == 0 as libc::c_int {
        (*state).maxTrans = 8 as libc::c_int;
        let ref mut fresh38 = (*state).trans;
        *fresh38 = xmlMalloc
            .expect(
                "non-null function pointer",
            )(
            ((*state).maxTrans as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<xmlRegTrans>() as libc::c_ulong),
        ) as *mut xmlRegTrans;
        if ((*state).trans).is_null() {
            xmlRegexpErrMemory(
                ctxt,
                b"adding transition\0" as *const u8 as *const libc::c_char,
            );
            (*state).maxTrans = 0 as libc::c_int;
            return;
        }
    } else if (*state).nbTrans >= (*state).maxTrans {
        let mut tmp: *mut xmlRegTrans = 0 as *mut xmlRegTrans;
        (*state).maxTrans *= 2 as libc::c_int;
        tmp = xmlRealloc
            .expect(
                "non-null function pointer",
            )(
            (*state).trans as *mut libc::c_void,
            ((*state).maxTrans as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<xmlRegTrans>() as libc::c_ulong),
        ) as *mut xmlRegTrans;
        if tmp.is_null() {
            xmlRegexpErrMemory(
                ctxt,
                b"adding transition\0" as *const u8 as *const libc::c_char,
            );
            (*state).maxTrans /= 2 as libc::c_int;
            return;
        }
        let ref mut fresh39 = (*state).trans;
        *fresh39 = tmp;
    }
    let ref mut fresh40 = (*((*state).trans).offset((*state).nbTrans as isize)).atom;
    *fresh40 = atom;
    (*((*state).trans).offset((*state).nbTrans as isize)).to = (*target).no;
    (*((*state).trans).offset((*state).nbTrans as isize)).counter = counter;
    (*((*state).trans).offset((*state).nbTrans as isize)).count = count;
    (*((*state).trans).offset((*state).nbTrans as isize)).nd = 0 as libc::c_int;
    let ref mut fresh41 = (*state).nbTrans;
    *fresh41 += 1;
    xmlRegStateAddTransTo(ctxt, target, (*state).no);
}
unsafe extern "C" fn xmlRegStatePush(
    mut ctxt: xmlRegParserCtxtPtr,
    mut state: xmlRegStatePtr,
) -> libc::c_int {
    if state.is_null() {
        return -(1 as libc::c_int);
    }
    if (*ctxt).maxStates == 0 as libc::c_int {
        (*ctxt).maxStates = 4 as libc::c_int;
        let ref mut fresh42 = (*ctxt).states;
        *fresh42 = xmlMalloc
            .expect(
                "non-null function pointer",
            )(
            ((*ctxt).maxStates as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<xmlRegStatePtr>() as libc::c_ulong),
        ) as *mut xmlRegStatePtr;
        if ((*ctxt).states).is_null() {
            xmlRegexpErrMemory(
                ctxt,
                b"adding state\0" as *const u8 as *const libc::c_char,
            );
            (*ctxt).maxStates = 0 as libc::c_int;
            return -(1 as libc::c_int);
        }
    } else if (*ctxt).nbStates >= (*ctxt).maxStates {
        let mut tmp: *mut xmlRegStatePtr = 0 as *mut xmlRegStatePtr;
        (*ctxt).maxStates *= 2 as libc::c_int;
        tmp = xmlRealloc
            .expect(
                "non-null function pointer",
            )(
            (*ctxt).states as *mut libc::c_void,
            ((*ctxt).maxStates as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<xmlRegStatePtr>() as libc::c_ulong),
        ) as *mut xmlRegStatePtr;
        if tmp.is_null() {
            xmlRegexpErrMemory(
                ctxt,
                b"adding state\0" as *const u8 as *const libc::c_char,
            );
            (*ctxt).maxStates /= 2 as libc::c_int;
            return -(1 as libc::c_int);
        }
        let ref mut fresh43 = (*ctxt).states;
        *fresh43 = tmp;
    }
    (*state).no = (*ctxt).nbStates;
    let ref mut fresh44 = (*ctxt).nbStates;
    let fresh45 = *fresh44;
    *fresh44 = *fresh44 + 1;
    let ref mut fresh46 = *((*ctxt).states).offset(fresh45 as isize);
    *fresh46 = state;
    return 0 as libc::c_int;
}
unsafe extern "C" fn xmlFAGenerateAllTransition(
    mut ctxt: xmlRegParserCtxtPtr,
    mut from: xmlRegStatePtr,
    mut to: xmlRegStatePtr,
    mut lax: libc::c_int,
) {
    if to.is_null() {
        to = xmlRegNewState(ctxt);
        xmlRegStatePush(ctxt, to);
        let ref mut fresh47 = (*ctxt).state;
        *fresh47 = to;
    }
    if lax != 0 {
        xmlRegStateAddTrans(
            ctxt,
            from,
            0 as xmlRegAtomPtr,
            to,
            -(1 as libc::c_int),
            0x123457 as libc::c_int,
        );
    } else {
        xmlRegStateAddTrans(
            ctxt,
            from,
            0 as xmlRegAtomPtr,
            to,
            -(1 as libc::c_int),
            0x123456 as libc::c_int,
        );
    };
}
unsafe extern "C" fn xmlFAGenerateEpsilonTransition(
    mut ctxt: xmlRegParserCtxtPtr,
    mut from: xmlRegStatePtr,
    mut to: xmlRegStatePtr,
) {
    if to.is_null() {
        to = xmlRegNewState(ctxt);
        xmlRegStatePush(ctxt, to);
        let ref mut fresh48 = (*ctxt).state;
        *fresh48 = to;
    }
    xmlRegStateAddTrans(
        ctxt,
        from,
        0 as xmlRegAtomPtr,
        to,
        -(1 as libc::c_int),
        -(1 as libc::c_int),
    );
}
unsafe extern "C" fn xmlFAGenerateCountedEpsilonTransition(
    mut ctxt: xmlRegParserCtxtPtr,
    mut from: xmlRegStatePtr,
    mut to: xmlRegStatePtr,
    mut counter: libc::c_int,
) {
    if to.is_null() {
        to = xmlRegNewState(ctxt);
        xmlRegStatePush(ctxt, to);
        let ref mut fresh49 = (*ctxt).state;
        *fresh49 = to;
    }
    xmlRegStateAddTrans(
        ctxt,
        from,
        0 as xmlRegAtomPtr,
        to,
        counter,
        -(1 as libc::c_int),
    );
}
unsafe extern "C" fn xmlFAGenerateCountedTransition(
    mut ctxt: xmlRegParserCtxtPtr,
    mut from: xmlRegStatePtr,
    mut to: xmlRegStatePtr,
    mut counter: libc::c_int,
) {
    if to.is_null() {
        to = xmlRegNewState(ctxt);
        xmlRegStatePush(ctxt, to);
        let ref mut fresh50 = (*ctxt).state;
        *fresh50 = to;
    }
    xmlRegStateAddTrans(
        ctxt,
        from,
        0 as xmlRegAtomPtr,
        to,
        -(1 as libc::c_int),
        counter,
    );
}
unsafe extern "C" fn xmlFAGenerateTransitions(
    mut ctxt: xmlRegParserCtxtPtr,
    mut from: xmlRegStatePtr,
    mut to: xmlRegStatePtr,
    mut atom: xmlRegAtomPtr,
) -> libc::c_int {
    let mut end: xmlRegStatePtr = 0 as *mut xmlRegState;
    let mut nullable: libc::c_int = 0 as libc::c_int;
    if atom.is_null() {
        (*ctxt).error = XML_REGEXP_COMPILE_ERROR as libc::c_int;
        xmlRegexpErrCompile(
            ctxt,
            b"generate transition: atom == NULL\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (*atom).type_0 as libc::c_uint == XML_REGEXP_SUBREG as libc::c_int as libc::c_uint
    {
        if xmlRegAtomPush(ctxt, atom) < 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        if !to.is_null() && (*atom).stop != to
            && (*atom).quant as libc::c_uint
                != XML_REGEXP_QUANT_RANGE as libc::c_int as libc::c_uint
        {
            xmlFAGenerateEpsilonTransition(ctxt, (*atom).stop, to);
        }
        match (*atom).quant as libc::c_uint {
            3 => {
                (*atom).quant = XML_REGEXP_QUANT_ONCE;
                if to.is_null() {
                    xmlFAGenerateEpsilonTransition(
                        ctxt,
                        (*atom).start,
                        0 as xmlRegStatePtr,
                    );
                    xmlFAGenerateEpsilonTransition(ctxt, (*atom).stop, (*ctxt).state);
                } else {
                    xmlFAGenerateEpsilonTransition(ctxt, (*atom).start, to);
                }
            }
            4 => {
                (*atom).quant = XML_REGEXP_QUANT_ONCE;
                xmlFAGenerateEpsilonTransition(ctxt, (*atom).start, (*atom).stop);
                xmlFAGenerateEpsilonTransition(ctxt, (*atom).stop, (*atom).start);
            }
            5 => {
                (*atom).quant = XML_REGEXP_QUANT_ONCE;
                xmlFAGenerateEpsilonTransition(ctxt, (*atom).stop, (*atom).start);
            }
            8 => {
                let mut counter: libc::c_int = 0;
                let mut inter: xmlRegStatePtr = 0 as *mut xmlRegState;
                let mut newstate: xmlRegStatePtr = 0 as *mut xmlRegState;
                if !to.is_null() {
                    newstate = to;
                } else {
                    newstate = xmlRegNewState(ctxt);
                    xmlRegStatePush(ctxt, newstate);
                }
                if (*atom).min == 0 as libc::c_int && ((*atom).start0).is_null() {
                    let mut copy: xmlRegAtomPtr = 0 as *mut xmlRegAtom;
                    copy = xmlRegCopyAtom(ctxt, atom);
                    if copy.is_null() {
                        return -(1 as libc::c_int);
                    }
                    (*copy).quant = XML_REGEXP_QUANT_ONCE;
                    (*copy).min = 0 as libc::c_int;
                    (*copy).max = 0 as libc::c_int;
                    if xmlFAGenerateTransitions(
                        ctxt,
                        (*atom).start,
                        0 as xmlRegStatePtr,
                        copy,
                    ) < 0 as libc::c_int
                    {
                        return -(1 as libc::c_int);
                    }
                    inter = (*ctxt).state;
                    counter = xmlRegGetCounter(ctxt);
                    (*((*ctxt).counters).offset(counter as isize))
                        .min = (*atom).min - 1 as libc::c_int;
                    (*((*ctxt).counters).offset(counter as isize))
                        .max = (*atom).max - 1 as libc::c_int;
                    xmlFAGenerateCountedEpsilonTransition(
                        ctxt,
                        inter,
                        (*atom).stop,
                        counter,
                    );
                    xmlFAGenerateCountedTransition(ctxt, inter, newstate, counter);
                    xmlFAGenerateEpsilonTransition(ctxt, (*atom).start, newstate);
                } else {
                    counter = xmlRegGetCounter(ctxt);
                    (*((*ctxt).counters).offset(counter as isize))
                        .min = (*atom).min - 1 as libc::c_int;
                    (*((*ctxt).counters).offset(counter as isize))
                        .max = (*atom).max - 1 as libc::c_int;
                    xmlFAGenerateCountedTransition(
                        ctxt,
                        (*atom).stop,
                        newstate,
                        counter,
                    );
                    xmlFAGenerateCountedEpsilonTransition(
                        ctxt,
                        (*atom).stop,
                        (*atom).start,
                        counter,
                    );
                    if (*atom).min == 0 as libc::c_int {
                        xmlFAGenerateEpsilonTransition(ctxt, (*atom).start0, newstate);
                    }
                }
                (*atom).min = 0 as libc::c_int;
                (*atom).max = 0 as libc::c_int;
                (*atom).quant = XML_REGEXP_QUANT_ONCE;
                let ref mut fresh51 = (*ctxt).state;
                *fresh51 = newstate;
            }
            _ => {}
        }
        return 0 as libc::c_int;
    }
    if (*atom).min == 0 as libc::c_int && (*atom).max == 0 as libc::c_int
        && (*atom).quant as libc::c_uint
            == XML_REGEXP_QUANT_RANGE as libc::c_int as libc::c_uint
    {
        if to.is_null() {
            to = xmlRegNewState(ctxt);
            if !to.is_null() {
                xmlRegStatePush(ctxt, to);
            } else {
                return -(1 as libc::c_int)
            }
        }
        xmlFAGenerateEpsilonTransition(ctxt, from, to);
        let ref mut fresh52 = (*ctxt).state;
        *fresh52 = to;
        xmlRegFreeAtom(atom);
        return 0 as libc::c_int;
    }
    if to.is_null() {
        to = xmlRegNewState(ctxt);
        if !to.is_null() {
            xmlRegStatePush(ctxt, to);
        } else {
            return -(1 as libc::c_int)
        }
    }
    end = to;
    if (*atom).quant as libc::c_uint
        == XML_REGEXP_QUANT_MULT as libc::c_int as libc::c_uint
        || (*atom).quant as libc::c_uint
            == XML_REGEXP_QUANT_PLUS as libc::c_int as libc::c_uint
    {
        let mut tmp: xmlRegStatePtr = 0 as *mut xmlRegState;
        tmp = xmlRegNewState(ctxt);
        if !tmp.is_null() {
            xmlRegStatePush(ctxt, tmp);
        } else {
            return -(1 as libc::c_int)
        }
        xmlFAGenerateEpsilonTransition(ctxt, tmp, to);
        to = tmp;
    }
    if xmlRegAtomPush(ctxt, atom) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if (*atom).quant as libc::c_uint
        == XML_REGEXP_QUANT_RANGE as libc::c_int as libc::c_uint
        && (*atom).min == 0 as libc::c_int && (*atom).max > 0 as libc::c_int
    {
        nullable = 1 as libc::c_int;
        (*atom).min = 1 as libc::c_int;
        if (*atom).max == 1 as libc::c_int {
            (*atom).quant = XML_REGEXP_QUANT_OPT;
        }
    }
    xmlRegStateAddTrans(ctxt, from, atom, to, -(1 as libc::c_int), -(1 as libc::c_int));
    let ref mut fresh53 = (*ctxt).state;
    *fresh53 = end;
    match (*atom).quant as libc::c_uint {
        3 => {
            (*atom).quant = XML_REGEXP_QUANT_ONCE;
            xmlFAGenerateEpsilonTransition(ctxt, from, to);
        }
        4 => {
            (*atom).quant = XML_REGEXP_QUANT_ONCE;
            xmlFAGenerateEpsilonTransition(ctxt, from, to);
            xmlRegStateAddTrans(
                ctxt,
                to,
                atom,
                to,
                -(1 as libc::c_int),
                -(1 as libc::c_int),
            );
        }
        5 => {
            (*atom).quant = XML_REGEXP_QUANT_ONCE;
            xmlRegStateAddTrans(
                ctxt,
                to,
                atom,
                to,
                -(1 as libc::c_int),
                -(1 as libc::c_int),
            );
        }
        8 => {
            if nullable != 0 {
                xmlFAGenerateEpsilonTransition(ctxt, from, to);
            }
        }
        _ => {}
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn xmlFAReduceEpsilonTransitions(
    mut ctxt: xmlRegParserCtxtPtr,
    mut fromnr: libc::c_int,
    mut tonr: libc::c_int,
    mut counter: libc::c_int,
) {
    let mut transnr: libc::c_int = 0;
    let mut from: xmlRegStatePtr = 0 as *mut xmlRegState;
    let mut to: xmlRegStatePtr = 0 as *mut xmlRegState;
    from = *((*ctxt).states).offset(fromnr as isize);
    if from.is_null() {
        return;
    }
    to = *((*ctxt).states).offset(tonr as isize);
    if to.is_null() {
        return;
    }
    if (*to).mark as libc::c_uint == XML_REGEXP_MARK_START as libc::c_int as libc::c_uint
        || (*to).mark as libc::c_uint
            == XML_REGEXP_MARK_VISITED as libc::c_int as libc::c_uint
    {
        return;
    }
    (*to).mark = XML_REGEXP_MARK_VISITED;
    if (*to).type_0 as libc::c_uint
        == XML_REGEXP_FINAL_STATE as libc::c_int as libc::c_uint
    {
        (*from).type_0 = XML_REGEXP_FINAL_STATE;
    }
    transnr = 0 as libc::c_int;
    while transnr < (*to).nbTrans {
        if !((*((*to).trans).offset(transnr as isize)).to < 0 as libc::c_int) {
            if ((*((*to).trans).offset(transnr as isize)).atom).is_null() {
                if (*((*to).trans).offset(transnr as isize)).to != fromnr {
                    if (*((*to).trans).offset(transnr as isize)).count
                        >= 0 as libc::c_int
                    {
                        let mut newto: libc::c_int = (*((*to).trans)
                            .offset(transnr as isize))
                            .to;
                        xmlRegStateAddTrans(
                            ctxt,
                            from,
                            0 as xmlRegAtomPtr,
                            *((*ctxt).states).offset(newto as isize),
                            -(1 as libc::c_int),
                            (*((*to).trans).offset(transnr as isize)).count,
                        );
                    } else if (*((*to).trans).offset(transnr as isize)).counter
                            >= 0 as libc::c_int
                        {
                        xmlFAReduceEpsilonTransitions(
                            ctxt,
                            fromnr,
                            (*((*to).trans).offset(transnr as isize)).to,
                            (*((*to).trans).offset(transnr as isize)).counter,
                        );
                    } else {
                        xmlFAReduceEpsilonTransitions(
                            ctxt,
                            fromnr,
                            (*((*to).trans).offset(transnr as isize)).to,
                            counter,
                        );
                    }
                }
            } else {
                let mut newto_0: libc::c_int = (*((*to).trans).offset(transnr as isize))
                    .to;
                if (*((*to).trans).offset(transnr as isize)).counter >= 0 as libc::c_int
                {
                    xmlRegStateAddTrans(
                        ctxt,
                        from,
                        (*((*to).trans).offset(transnr as isize)).atom,
                        *((*ctxt).states).offset(newto_0 as isize),
                        (*((*to).trans).offset(transnr as isize)).counter,
                        -(1 as libc::c_int),
                    );
                } else {
                    xmlRegStateAddTrans(
                        ctxt,
                        from,
                        (*((*to).trans).offset(transnr as isize)).atom,
                        *((*ctxt).states).offset(newto_0 as isize),
                        counter,
                        -(1 as libc::c_int),
                    );
                }
            }
        }
        transnr += 1;
    }
    (*to).mark = XML_REGEXP_MARK_NORMAL;
}
unsafe extern "C" fn xmlFAEliminateSimpleEpsilonTransitions(
    mut ctxt: xmlRegParserCtxtPtr,
) {
    let mut statenr: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut newto: libc::c_int = 0;
    let mut state: xmlRegStatePtr = 0 as *mut xmlRegState;
    let mut tmp: xmlRegStatePtr = 0 as *mut xmlRegState;
    statenr = 0 as libc::c_int;
    while statenr < (*ctxt).nbStates {
        state = *((*ctxt).states).offset(statenr as isize);
        if !state.is_null() {
            if !((*state).nbTrans != 1 as libc::c_int) {
                if !((*state).type_0 as libc::c_uint
                    == XML_REGEXP_UNREACH_STATE as libc::c_int as libc::c_uint
                    || (*state).type_0 as libc::c_uint
                        == XML_REGEXP_FINAL_STATE as libc::c_int as libc::c_uint)
                {
                    if ((*((*state).trans).offset(0 as libc::c_int as isize)).atom)
                        .is_null()
                        && (*((*state).trans).offset(0 as libc::c_int as isize)).to
                            >= 0 as libc::c_int
                        && (*((*state).trans).offset(0 as libc::c_int as isize)).to
                            != statenr
                        && (*((*state).trans).offset(0 as libc::c_int as isize)).counter
                            < 0 as libc::c_int
                        && (*((*state).trans).offset(0 as libc::c_int as isize)).count
                            < 0 as libc::c_int
                    {
                        newto = (*((*state).trans).offset(0 as libc::c_int as isize)).to;
                        if !((*state).type_0 as libc::c_uint
                            == XML_REGEXP_START_STATE as libc::c_int as libc::c_uint)
                        {
                            i = 0 as libc::c_int;
                            while i < (*state).nbTransTo {
                                tmp = *((*ctxt).states)
                                    .offset(*((*state).transTo).offset(i as isize) as isize);
                                j = 0 as libc::c_int;
                                while j < (*tmp).nbTrans {
                                    if (*((*tmp).trans).offset(j as isize)).to == statenr {
                                        (*((*tmp).trans).offset(j as isize))
                                            .to = -(1 as libc::c_int);
                                        xmlRegStateAddTrans(
                                            ctxt,
                                            tmp,
                                            (*((*tmp).trans).offset(j as isize)).atom,
                                            *((*ctxt).states).offset(newto as isize),
                                            (*((*tmp).trans).offset(j as isize)).counter,
                                            (*((*tmp).trans).offset(j as isize)).count,
                                        );
                                    }
                                    j += 1;
                                }
                                i += 1;
                            }
                            if (*state).type_0 as libc::c_uint
                                == XML_REGEXP_FINAL_STATE as libc::c_int as libc::c_uint
                            {
                                (**((*ctxt).states).offset(newto as isize))
                                    .type_0 = XML_REGEXP_FINAL_STATE;
                            }
                            (*state).nbTrans = 0 as libc::c_int;
                            (*state).type_0 = XML_REGEXP_UNREACH_STATE;
                        }
                    }
                }
            }
        }
        statenr += 1;
    }
}
unsafe extern "C" fn xmlFAEliminateEpsilonTransitions(mut ctxt: xmlRegParserCtxtPtr) {
    let mut statenr: libc::c_int = 0;
    let mut transnr: libc::c_int = 0;
    let mut state: xmlRegStatePtr = 0 as *mut xmlRegState;
    let mut has_epsilon: libc::c_int = 0;
    if ((*ctxt).states).is_null() {
        return;
    }
    xmlFAEliminateSimpleEpsilonTransitions(ctxt);
    statenr = 0 as libc::c_int;
    while statenr < (*ctxt).nbStates {
        state = *((*ctxt).states).offset(statenr as isize);
        if !state.is_null()
            && (*state).type_0 as libc::c_uint
                == XML_REGEXP_UNREACH_STATE as libc::c_int as libc::c_uint
        {
            xmlRegFreeState(state);
            let ref mut fresh54 = *((*ctxt).states).offset(statenr as isize);
            *fresh54 = 0 as xmlRegStatePtr;
        }
        statenr += 1;
    }
    has_epsilon = 0 as libc::c_int;
    statenr = (*ctxt).nbStates - 1 as libc::c_int;
    while statenr >= 0 as libc::c_int {
        state = *((*ctxt).states).offset(statenr as isize);
        if !state.is_null() {
            if (*state).nbTrans == 0 as libc::c_int
                && (*state).type_0 as libc::c_uint
                    != XML_REGEXP_FINAL_STATE as libc::c_int as libc::c_uint
            {
                (*state).type_0 = XML_REGEXP_SINK_STATE;
            }
            transnr = 0 as libc::c_int;
            while transnr < (*state).nbTrans {
                if ((*((*state).trans).offset(transnr as isize)).atom).is_null()
                    && (*((*state).trans).offset(transnr as isize)).to
                        >= 0 as libc::c_int
                {
                    if (*((*state).trans).offset(transnr as isize)).to == statenr {
                        (*((*state).trans).offset(transnr as isize))
                            .to = -(1 as libc::c_int);
                    } else if (*((*state).trans).offset(transnr as isize)).count
                            < 0 as libc::c_int
                        {
                        let mut newto: libc::c_int = (*((*state).trans)
                            .offset(transnr as isize))
                            .to;
                        has_epsilon = 1 as libc::c_int;
                        (*((*state).trans).offset(transnr as isize))
                            .to = -(2 as libc::c_int);
                        (*state).mark = XML_REGEXP_MARK_START;
                        xmlFAReduceEpsilonTransitions(
                            ctxt,
                            statenr,
                            newto,
                            (*((*state).trans).offset(transnr as isize)).counter,
                        );
                        (*state).mark = XML_REGEXP_MARK_NORMAL;
                    }
                }
                transnr += 1;
            }
        }
        statenr -= 1;
    }
    if has_epsilon != 0 {
        statenr = 0 as libc::c_int;
        while statenr < (*ctxt).nbStates {
            state = *((*ctxt).states).offset(statenr as isize);
            if !state.is_null() {
                transnr = 0 as libc::c_int;
                while transnr < (*state).nbTrans {
                    let mut trans: xmlRegTransPtr = &mut *((*state).trans)
                        .offset(transnr as isize) as *mut xmlRegTrans;
                    if ((*trans).atom).is_null() && (*trans).count < 0 as libc::c_int
                        && (*trans).to >= 0 as libc::c_int
                    {
                        (*trans).to = -(1 as libc::c_int);
                    }
                    transnr += 1;
                }
            }
            statenr += 1;
        }
    }
    statenr = 0 as libc::c_int;
    while statenr < (*ctxt).nbStates {
        state = *((*ctxt).states).offset(statenr as isize);
        if !state.is_null() {
            (*state).reached = XML_REGEXP_MARK_NORMAL;
        }
        statenr += 1;
    }
    state = *((*ctxt).states).offset(0 as libc::c_int as isize);
    if !state.is_null() {
        (*state).reached = XML_REGEXP_MARK_START;
    }
    while !state.is_null() {
        let mut target: xmlRegStatePtr = 0 as xmlRegStatePtr;
        (*state).reached = XML_REGEXP_MARK_VISITED;
        transnr = 0 as libc::c_int;
        while transnr < (*state).nbTrans {
            if (*((*state).trans).offset(transnr as isize)).to >= 0 as libc::c_int
                && (!((*((*state).trans).offset(transnr as isize)).atom).is_null()
                    || (*((*state).trans).offset(transnr as isize)).count
                        >= 0 as libc::c_int)
            {
                let mut newto_0: libc::c_int = (*((*state).trans)
                    .offset(transnr as isize))
                    .to;
                if !(*((*ctxt).states).offset(newto_0 as isize)).is_null() {
                    if (**((*ctxt).states).offset(newto_0 as isize)).reached
                        as libc::c_uint
                        == XML_REGEXP_MARK_NORMAL as libc::c_int as libc::c_uint
                    {
                        (**((*ctxt).states).offset(newto_0 as isize))
                            .reached = XML_REGEXP_MARK_START;
                        target = *((*ctxt).states).offset(newto_0 as isize);
                    }
                }
            }
            transnr += 1;
        }
        if target.is_null() {
            statenr = 1 as libc::c_int;
            while statenr < (*ctxt).nbStates {
                state = *((*ctxt).states).offset(statenr as isize);
                if !state.is_null()
                    && (*state).reached as libc::c_uint
                        == XML_REGEXP_MARK_START as libc::c_int as libc::c_uint
                {
                    target = state;
                    break;
                } else {
                    statenr += 1;
                }
            }
        }
        state = target;
    }
    statenr = 0 as libc::c_int;
    while statenr < (*ctxt).nbStates {
        state = *((*ctxt).states).offset(statenr as isize);
        if !state.is_null()
            && (*state).reached as libc::c_uint
                == XML_REGEXP_MARK_NORMAL as libc::c_int as libc::c_uint
        {
            xmlRegFreeState(state);
            let ref mut fresh55 = *((*ctxt).states).offset(statenr as isize);
            *fresh55 = 0 as xmlRegStatePtr;
        }
        statenr += 1;
    }
}
unsafe extern "C" fn xmlFACompareRanges(
    mut range1: xmlRegRangePtr,
    mut range2: xmlRegRangePtr,
) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    if (*range1).type_0 as libc::c_uint
        == XML_REGEXP_RANGES as libc::c_int as libc::c_uint
        || (*range2).type_0 as libc::c_uint
            == XML_REGEXP_RANGES as libc::c_int as libc::c_uint
        || (*range2).type_0 as libc::c_uint
            == XML_REGEXP_SUBREG as libc::c_int as libc::c_uint
        || (*range1).type_0 as libc::c_uint
            == XML_REGEXP_SUBREG as libc::c_int as libc::c_uint
        || (*range1).type_0 as libc::c_uint
            == XML_REGEXP_STRING as libc::c_int as libc::c_uint
        || (*range2).type_0 as libc::c_uint
            == XML_REGEXP_STRING as libc::c_int as libc::c_uint
    {
        return -(1 as libc::c_int);
    }
    if (*range1).type_0 as libc::c_uint > (*range2).type_0 as libc::c_uint {
        let mut tmp: xmlRegRangePtr = 0 as *mut xmlRegRange;
        tmp = range1;
        range1 = range2;
        range2 = tmp;
    }
    if (*range1).type_0 as libc::c_uint
        == XML_REGEXP_ANYCHAR as libc::c_int as libc::c_uint
        || (*range2).type_0 as libc::c_uint
            == XML_REGEXP_ANYCHAR as libc::c_int as libc::c_uint
    {
        ret = 1 as libc::c_int;
    } else if (*range1).type_0 as libc::c_uint
            == XML_REGEXP_EPSILON as libc::c_int as libc::c_uint
            || (*range2).type_0 as libc::c_uint
                == XML_REGEXP_EPSILON as libc::c_int as libc::c_uint
        {
        return 0 as libc::c_int
    } else {
        if (*range1).type_0 as libc::c_uint == (*range2).type_0 as libc::c_uint {
            if (*range1).type_0 as libc::c_uint
                != XML_REGEXP_CHARVAL as libc::c_int as libc::c_uint
            {
                ret = 1 as libc::c_int;
            } else if (*range1).end < (*range2).start || (*range2).end < (*range1).start
                {
                ret = 0 as libc::c_int;
            } else {
                ret = 1 as libc::c_int;
            }
        } else if (*range1).type_0 as libc::c_uint
                == XML_REGEXP_CHARVAL as libc::c_int as libc::c_uint
            {
            let mut codepoint: libc::c_int = 0;
            let mut neg: libc::c_int = 0 as libc::c_int;
            if (*range1).neg == 0 as libc::c_int && (*range2).neg != 0 as libc::c_int
                || (*range1).neg != 0 as libc::c_int && (*range2).neg == 0 as libc::c_int
            {
                neg = 1 as libc::c_int;
            }
            codepoint = (*range1).start;
            while codepoint <= (*range1).end {
                ret = xmlRegCheckCharacterRange(
                    (*range2).type_0,
                    codepoint,
                    0 as libc::c_int,
                    (*range2).start,
                    (*range2).end,
                    (*range2).blockName,
                );
                if ret < 0 as libc::c_int {
                    return -(1 as libc::c_int);
                }
                if neg == 1 as libc::c_int && ret == 0 as libc::c_int
                    || neg == 0 as libc::c_int && ret == 1 as libc::c_int
                {
                    return 1 as libc::c_int;
                }
                codepoint += 1;
            }
            return 0 as libc::c_int;
        } else {
            if (*range1).type_0 as libc::c_uint
                == XML_REGEXP_BLOCK_NAME as libc::c_int as libc::c_uint
                || (*range2).type_0 as libc::c_uint
                    == XML_REGEXP_BLOCK_NAME as libc::c_int as libc::c_uint
            {
                if (*range1).type_0 as libc::c_uint == (*range2).type_0 as libc::c_uint {
                    ret = xmlStrEqual((*range1).blockName, (*range2).blockName);
                } else {
                    return 1 as libc::c_int
                }
            } else if ((*range1).type_0 as libc::c_uint)
                    < XML_REGEXP_LETTER as libc::c_int as libc::c_uint
                    || ((*range2).type_0 as libc::c_uint)
                        < XML_REGEXP_LETTER as libc::c_int as libc::c_uint
                {
                if (*range1).type_0 as libc::c_uint
                    == XML_REGEXP_ANYSPACE as libc::c_int as libc::c_uint
                    && (*range2).type_0 as libc::c_uint
                        == XML_REGEXP_NOTSPACE as libc::c_int as libc::c_uint
                {
                    ret = 0 as libc::c_int;
                } else if (*range1).type_0 as libc::c_uint
                        == XML_REGEXP_INITNAME as libc::c_int as libc::c_uint
                        && (*range2).type_0 as libc::c_uint
                            == XML_REGEXP_NOTINITNAME as libc::c_int as libc::c_uint
                    {
                    ret = 0 as libc::c_int;
                } else if (*range1).type_0 as libc::c_uint
                        == XML_REGEXP_NAMECHAR as libc::c_int as libc::c_uint
                        && (*range2).type_0 as libc::c_uint
                            == XML_REGEXP_NOTNAMECHAR as libc::c_int as libc::c_uint
                    {
                    ret = 0 as libc::c_int;
                } else if (*range1).type_0 as libc::c_uint
                        == XML_REGEXP_DECIMAL as libc::c_int as libc::c_uint
                        && (*range2).type_0 as libc::c_uint
                            == XML_REGEXP_NOTDECIMAL as libc::c_int as libc::c_uint
                    {
                    ret = 0 as libc::c_int;
                } else if (*range1).type_0 as libc::c_uint
                        == XML_REGEXP_REALCHAR as libc::c_int as libc::c_uint
                        && (*range2).type_0 as libc::c_uint
                            == XML_REGEXP_NOTREALCHAR as libc::c_int as libc::c_uint
                    {
                    ret = 0 as libc::c_int;
                } else {
                    return 1 as libc::c_int
                }
            } else {
                ret = 0 as libc::c_int;
                match (*range1).type_0 as libc::c_uint {
                    100 => {
                        if (*range2).type_0 as libc::c_uint
                            == XML_REGEXP_LETTER_UPPERCASE as libc::c_int as libc::c_uint
                            || (*range2).type_0 as libc::c_uint
                                == XML_REGEXP_LETTER_LOWERCASE as libc::c_int
                                    as libc::c_uint
                            || (*range2).type_0 as libc::c_uint
                                == XML_REGEXP_LETTER_TITLECASE as libc::c_int
                                    as libc::c_uint
                            || (*range2).type_0 as libc::c_uint
                                == XML_REGEXP_LETTER_MODIFIER as libc::c_int as libc::c_uint
                            || (*range2).type_0 as libc::c_uint
                                == XML_REGEXP_LETTER_OTHERS as libc::c_int as libc::c_uint
                        {
                            ret = 1 as libc::c_int;
                        }
                    }
                    106 => {
                        if (*range2).type_0 as libc::c_uint
                            == XML_REGEXP_MARK_NONSPACING as libc::c_int as libc::c_uint
                            || (*range2).type_0 as libc::c_uint
                                == XML_REGEXP_MARK_SPACECOMBINING as libc::c_int
                                    as libc::c_uint
                            || (*range2).type_0 as libc::c_uint
                                == XML_REGEXP_MARK_ENCLOSING as libc::c_int as libc::c_uint
                        {
                            ret = 1 as libc::c_int;
                        }
                    }
                    110 => {
                        if (*range2).type_0 as libc::c_uint
                            == XML_REGEXP_NUMBER_DECIMAL as libc::c_int as libc::c_uint
                            || (*range2).type_0 as libc::c_uint
                                == XML_REGEXP_NUMBER_LETTER as libc::c_int as libc::c_uint
                            || (*range2).type_0 as libc::c_uint
                                == XML_REGEXP_NUMBER_OTHERS as libc::c_int as libc::c_uint
                        {
                            ret = 1 as libc::c_int;
                        }
                    }
                    114 => {
                        if (*range2).type_0 as libc::c_uint
                            == XML_REGEXP_PUNCT_CONNECTOR as libc::c_int as libc::c_uint
                            || (*range2).type_0 as libc::c_uint
                                == XML_REGEXP_PUNCT_DASH as libc::c_int as libc::c_uint
                            || (*range2).type_0 as libc::c_uint
                                == XML_REGEXP_PUNCT_OPEN as libc::c_int as libc::c_uint
                            || (*range2).type_0 as libc::c_uint
                                == XML_REGEXP_PUNCT_CLOSE as libc::c_int as libc::c_uint
                            || (*range2).type_0 as libc::c_uint
                                == XML_REGEXP_PUNCT_INITQUOTE as libc::c_int as libc::c_uint
                            || (*range2).type_0 as libc::c_uint
                                == XML_REGEXP_PUNCT_FINQUOTE as libc::c_int as libc::c_uint
                            || (*range2).type_0 as libc::c_uint
                                == XML_REGEXP_PUNCT_OTHERS as libc::c_int as libc::c_uint
                        {
                            ret = 1 as libc::c_int;
                        }
                    }
                    122 => {
                        if (*range2).type_0 as libc::c_uint
                            == XML_REGEXP_SEPAR_SPACE as libc::c_int as libc::c_uint
                            || (*range2).type_0 as libc::c_uint
                                == XML_REGEXP_SEPAR_LINE as libc::c_int as libc::c_uint
                            || (*range2).type_0 as libc::c_uint
                                == XML_REGEXP_SEPAR_PARA as libc::c_int as libc::c_uint
                        {
                            ret = 1 as libc::c_int;
                        }
                    }
                    126 => {
                        if (*range2).type_0 as libc::c_uint
                            == XML_REGEXP_SYMBOL_MATH as libc::c_int as libc::c_uint
                            || (*range2).type_0 as libc::c_uint
                                == XML_REGEXP_SYMBOL_CURRENCY as libc::c_int as libc::c_uint
                            || (*range2).type_0 as libc::c_uint
                                == XML_REGEXP_SYMBOL_MODIFIER as libc::c_int as libc::c_uint
                            || (*range2).type_0 as libc::c_uint
                                == XML_REGEXP_SYMBOL_OTHERS as libc::c_int as libc::c_uint
                        {
                            ret = 1 as libc::c_int;
                        }
                    }
                    131 => {
                        if (*range2).type_0 as libc::c_uint
                            == XML_REGEXP_OTHER_CONTROL as libc::c_int as libc::c_uint
                            || (*range2).type_0 as libc::c_uint
                                == XML_REGEXP_OTHER_FORMAT as libc::c_int as libc::c_uint
                            || (*range2).type_0 as libc::c_uint
                                == XML_REGEXP_OTHER_PRIVATE as libc::c_int as libc::c_uint
                        {
                            ret = 1 as libc::c_int;
                        }
                    }
                    _ => {
                        if (*range2).type_0 as libc::c_uint
                            >= XML_REGEXP_LETTER as libc::c_int as libc::c_uint
                            && ((*range2).type_0 as libc::c_uint)
                                < XML_REGEXP_BLOCK_NAME as libc::c_int as libc::c_uint
                        {
                            ret = 0 as libc::c_int;
                        } else {
                            return 1 as libc::c_int
                        }
                    }
                }
            }
        }
    }
    if (*range1).neg == 0 as libc::c_int && (*range2).neg != 0 as libc::c_int
        || (*range1).neg != 0 as libc::c_int && (*range2).neg == 0 as libc::c_int
    {
        ret = (ret == 0) as libc::c_int;
    }
    return ret;
}
unsafe extern "C" fn xmlFACompareAtomTypes(
    mut type1: xmlRegAtomType,
    mut type2: xmlRegAtomType,
) -> libc::c_int {
    if type1 as libc::c_uint == XML_REGEXP_EPSILON as libc::c_int as libc::c_uint
        || type1 as libc::c_uint == XML_REGEXP_CHARVAL as libc::c_int as libc::c_uint
        || type1 as libc::c_uint == XML_REGEXP_RANGES as libc::c_int as libc::c_uint
        || type1 as libc::c_uint == XML_REGEXP_SUBREG as libc::c_int as libc::c_uint
        || type1 as libc::c_uint == XML_REGEXP_STRING as libc::c_int as libc::c_uint
        || type1 as libc::c_uint == XML_REGEXP_ANYCHAR as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int;
    }
    if type2 as libc::c_uint == XML_REGEXP_EPSILON as libc::c_int as libc::c_uint
        || type2 as libc::c_uint == XML_REGEXP_CHARVAL as libc::c_int as libc::c_uint
        || type2 as libc::c_uint == XML_REGEXP_RANGES as libc::c_int as libc::c_uint
        || type2 as libc::c_uint == XML_REGEXP_SUBREG as libc::c_int as libc::c_uint
        || type2 as libc::c_uint == XML_REGEXP_STRING as libc::c_int as libc::c_uint
        || type2 as libc::c_uint == XML_REGEXP_ANYCHAR as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int;
    }
    if type1 as libc::c_uint == type2 as libc::c_uint {
        return 1 as libc::c_int;
    }
    if type1 as libc::c_uint > type2 as libc::c_uint {
        let mut tmp: xmlRegAtomType = type1;
        type1 = type2;
        type2 = tmp;
    }
    match type1 as libc::c_uint {
        7 => {
            if type2 as libc::c_uint
                == XML_REGEXP_NOTSPACE as libc::c_int as libc::c_uint
                || type2 as libc::c_uint
                    >= XML_REGEXP_LETTER as libc::c_int as libc::c_uint
                    && type2 as libc::c_uint
                        <= XML_REGEXP_LETTER_OTHERS as libc::c_int as libc::c_uint
                || type2 as libc::c_uint
                    >= XML_REGEXP_NUMBER as libc::c_int as libc::c_uint
                    && type2 as libc::c_uint
                        <= XML_REGEXP_NUMBER_OTHERS as libc::c_int as libc::c_uint
                || type2 as libc::c_uint
                    >= XML_REGEXP_MARK as libc::c_int as libc::c_uint
                    && type2 as libc::c_uint
                        <= XML_REGEXP_MARK_ENCLOSING as libc::c_int as libc::c_uint
                || type2 as libc::c_uint
                    >= XML_REGEXP_PUNCT as libc::c_int as libc::c_uint
                    && type2 as libc::c_uint
                        <= XML_REGEXP_PUNCT_OTHERS as libc::c_int as libc::c_uint
                || type2 as libc::c_uint
                    >= XML_REGEXP_SYMBOL as libc::c_int as libc::c_uint
                    && type2 as libc::c_uint
                        <= XML_REGEXP_SYMBOL_OTHERS as libc::c_int as libc::c_uint
            {
                return 0 as libc::c_int;
            }
        }
        8 => {}
        9 => {
            if type2 as libc::c_uint
                == XML_REGEXP_NOTINITNAME as libc::c_int as libc::c_uint
                || type2 as libc::c_uint
                    >= XML_REGEXP_NUMBER as libc::c_int as libc::c_uint
                    && type2 as libc::c_uint
                        <= XML_REGEXP_NUMBER_OTHERS as libc::c_int as libc::c_uint
                || type2 as libc::c_uint
                    >= XML_REGEXP_MARK as libc::c_int as libc::c_uint
                    && type2 as libc::c_uint
                        <= XML_REGEXP_MARK_ENCLOSING as libc::c_int as libc::c_uint
                || type2 as libc::c_uint
                    >= XML_REGEXP_SEPAR as libc::c_int as libc::c_uint
                    && type2 as libc::c_uint
                        <= XML_REGEXP_SEPAR_PARA as libc::c_int as libc::c_uint
                || type2 as libc::c_uint
                    >= XML_REGEXP_PUNCT as libc::c_int as libc::c_uint
                    && type2 as libc::c_uint
                        <= XML_REGEXP_PUNCT_OTHERS as libc::c_int as libc::c_uint
                || type2 as libc::c_uint
                    >= XML_REGEXP_SYMBOL as libc::c_int as libc::c_uint
                    && type2 as libc::c_uint
                        <= XML_REGEXP_SYMBOL_OTHERS as libc::c_int as libc::c_uint
                || type2 as libc::c_uint
                    >= XML_REGEXP_OTHER as libc::c_int as libc::c_uint
                    && type2 as libc::c_uint
                        <= XML_REGEXP_OTHER_NA as libc::c_int as libc::c_uint
            {
                return 0 as libc::c_int;
            }
        }
        10 => {}
        11 => {
            if type2 as libc::c_uint
                == XML_REGEXP_NOTNAMECHAR as libc::c_int as libc::c_uint
                || type2 as libc::c_uint
                    >= XML_REGEXP_MARK as libc::c_int as libc::c_uint
                    && type2 as libc::c_uint
                        <= XML_REGEXP_MARK_ENCLOSING as libc::c_int as libc::c_uint
                || type2 as libc::c_uint
                    >= XML_REGEXP_PUNCT as libc::c_int as libc::c_uint
                    && type2 as libc::c_uint
                        <= XML_REGEXP_PUNCT_OTHERS as libc::c_int as libc::c_uint
                || type2 as libc::c_uint
                    >= XML_REGEXP_SEPAR as libc::c_int as libc::c_uint
                    && type2 as libc::c_uint
                        <= XML_REGEXP_SEPAR_PARA as libc::c_int as libc::c_uint
                || type2 as libc::c_uint
                    >= XML_REGEXP_SYMBOL as libc::c_int as libc::c_uint
                    && type2 as libc::c_uint
                        <= XML_REGEXP_SYMBOL_OTHERS as libc::c_int as libc::c_uint
                || type2 as libc::c_uint
                    >= XML_REGEXP_OTHER as libc::c_int as libc::c_uint
                    && type2 as libc::c_uint
                        <= XML_REGEXP_OTHER_NA as libc::c_int as libc::c_uint
            {
                return 0 as libc::c_int;
            }
        }
        12 => {}
        13 => {
            if type2 as libc::c_uint
                == XML_REGEXP_NOTDECIMAL as libc::c_int as libc::c_uint
                || type2 as libc::c_uint
                    == XML_REGEXP_REALCHAR as libc::c_int as libc::c_uint
                || type2 as libc::c_uint
                    >= XML_REGEXP_LETTER as libc::c_int as libc::c_uint
                    && type2 as libc::c_uint
                        <= XML_REGEXP_LETTER_OTHERS as libc::c_int as libc::c_uint
                || type2 as libc::c_uint
                    >= XML_REGEXP_MARK as libc::c_int as libc::c_uint
                    && type2 as libc::c_uint
                        <= XML_REGEXP_MARK_ENCLOSING as libc::c_int as libc::c_uint
                || type2 as libc::c_uint
                    >= XML_REGEXP_PUNCT as libc::c_int as libc::c_uint
                    && type2 as libc::c_uint
                        <= XML_REGEXP_PUNCT_OTHERS as libc::c_int as libc::c_uint
                || type2 as libc::c_uint
                    >= XML_REGEXP_SEPAR as libc::c_int as libc::c_uint
                    && type2 as libc::c_uint
                        <= XML_REGEXP_SEPAR_PARA as libc::c_int as libc::c_uint
                || type2 as libc::c_uint
                    >= XML_REGEXP_SYMBOL as libc::c_int as libc::c_uint
                    && type2 as libc::c_uint
                        <= XML_REGEXP_SYMBOL_OTHERS as libc::c_int as libc::c_uint
                || type2 as libc::c_uint
                    >= XML_REGEXP_OTHER as libc::c_int as libc::c_uint
                    && type2 as libc::c_uint
                        <= XML_REGEXP_OTHER_NA as libc::c_int as libc::c_uint
            {
                return 0 as libc::c_int;
            }
        }
        14 => {}
        15 => {
            if type2 as libc::c_uint
                == XML_REGEXP_NOTDECIMAL as libc::c_int as libc::c_uint
                || type2 as libc::c_uint
                    >= XML_REGEXP_MARK as libc::c_int as libc::c_uint
                    && type2 as libc::c_uint
                        <= XML_REGEXP_MARK_ENCLOSING as libc::c_int as libc::c_uint
                || type2 as libc::c_uint
                    >= XML_REGEXP_PUNCT as libc::c_int as libc::c_uint
                    && type2 as libc::c_uint
                        <= XML_REGEXP_PUNCT_OTHERS as libc::c_int as libc::c_uint
                || type2 as libc::c_uint
                    >= XML_REGEXP_SEPAR as libc::c_int as libc::c_uint
                    && type2 as libc::c_uint
                        <= XML_REGEXP_SEPAR_PARA as libc::c_int as libc::c_uint
                || type2 as libc::c_uint
                    >= XML_REGEXP_SYMBOL as libc::c_int as libc::c_uint
                    && type2 as libc::c_uint
                        <= XML_REGEXP_SYMBOL_OTHERS as libc::c_int as libc::c_uint
                || type2 as libc::c_uint
                    >= XML_REGEXP_OTHER as libc::c_int as libc::c_uint
                    && type2 as libc::c_uint
                        <= XML_REGEXP_OTHER_NA as libc::c_int as libc::c_uint
            {
                return 0 as libc::c_int;
            }
        }
        100 => {
            if type2 as libc::c_uint
                <= XML_REGEXP_LETTER_OTHERS as libc::c_int as libc::c_uint
            {
                return 1 as libc::c_int;
            }
            return 0 as libc::c_int;
        }
        101 | 102 | 103 | 104 | 105 => return 0 as libc::c_int,
        106 => {
            if type2 as libc::c_uint
                <= XML_REGEXP_MARK_ENCLOSING as libc::c_int as libc::c_uint
            {
                return 1 as libc::c_int;
            }
            return 0 as libc::c_int;
        }
        107 | 108 | 109 => return 0 as libc::c_int,
        110 => {
            if type2 as libc::c_uint
                <= XML_REGEXP_NUMBER_OTHERS as libc::c_int as libc::c_uint
            {
                return 1 as libc::c_int;
            }
            return 0 as libc::c_int;
        }
        111 | 112 | 113 => return 0 as libc::c_int,
        114 => {
            if type2 as libc::c_uint
                <= XML_REGEXP_PUNCT_OTHERS as libc::c_int as libc::c_uint
            {
                return 1 as libc::c_int;
            }
            return 0 as libc::c_int;
        }
        115 | 116 | 117 | 118 | 119 | 120 | 121 => return 0 as libc::c_int,
        122 => {
            if type2 as libc::c_uint
                <= XML_REGEXP_SEPAR_PARA as libc::c_int as libc::c_uint
            {
                return 1 as libc::c_int;
            }
            return 0 as libc::c_int;
        }
        123 | 124 | 125 => return 0 as libc::c_int,
        126 => {
            if type2 as libc::c_uint
                <= XML_REGEXP_SYMBOL_OTHERS as libc::c_int as libc::c_uint
            {
                return 1 as libc::c_int;
            }
            return 0 as libc::c_int;
        }
        127 | 128 | 129 | 130 => return 0 as libc::c_int,
        131 => {
            if type2 as libc::c_uint
                <= XML_REGEXP_OTHER_NA as libc::c_int as libc::c_uint
            {
                return 1 as libc::c_int;
            }
            return 0 as libc::c_int;
        }
        132 | 133 | 134 | 135 => return 0 as libc::c_int,
        16 | _ => {}
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn xmlFAEqualAtoms(
    mut atom1: xmlRegAtomPtr,
    mut atom2: xmlRegAtomPtr,
    mut deep: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    if atom1 == atom2 {
        return 1 as libc::c_int;
    }
    if atom1.is_null() || atom2.is_null() {
        return 0 as libc::c_int;
    }
    if (*atom1).type_0 as libc::c_uint != (*atom2).type_0 as libc::c_uint {
        return 0 as libc::c_int;
    }
    match (*atom1).type_0 as libc::c_uint {
        1 => {
            ret = 0 as libc::c_int;
        }
        5 => {
            if deep == 0 {
                ret = ((*atom1).valuep == (*atom2).valuep) as libc::c_int;
            } else {
                ret = xmlStrEqual(
                    (*atom1).valuep as *mut xmlChar,
                    (*atom2).valuep as *mut xmlChar,
                );
            }
        }
        2 => {
            ret = ((*atom1).codepoint == (*atom2).codepoint) as libc::c_int;
        }
        3 => {
            ret = 0 as libc::c_int;
        }
        _ => {}
    }
    return ret;
}
unsafe extern "C" fn xmlFACompareAtoms(
    mut atom1: xmlRegAtomPtr,
    mut atom2: xmlRegAtomPtr,
    mut deep: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut ret: libc::c_int = 1 as libc::c_int;
    if atom1 == atom2 {
        return 1 as libc::c_int;
    }
    if atom1.is_null() || atom2.is_null() {
        return 0 as libc::c_int;
    }
    if (*atom1).type_0 as libc::c_uint
        == XML_REGEXP_ANYCHAR as libc::c_int as libc::c_uint
        || (*atom2).type_0 as libc::c_uint
            == XML_REGEXP_ANYCHAR as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int;
    }
    if (*atom1).type_0 as libc::c_uint > (*atom2).type_0 as libc::c_uint {
        let mut tmp: xmlRegAtomPtr = 0 as *mut xmlRegAtom;
        tmp = atom1;
        atom1 = atom2;
        atom2 = tmp;
    }
    if (*atom1).type_0 as libc::c_uint != (*atom2).type_0 as libc::c_uint {
        ret = xmlFACompareAtomTypes((*atom1).type_0, (*atom2).type_0);
        if ret == 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    match (*atom1).type_0 as libc::c_uint {
        5 => {
            if deep == 0 {
                ret = ((*atom1).valuep != (*atom2).valuep) as libc::c_int;
            } else {
                let mut val1: *mut xmlChar = (*atom1).valuep as *mut xmlChar;
                let mut val2: *mut xmlChar = (*atom2).valuep as *mut xmlChar;
                let mut compound1: libc::c_int = (xmlStrchr(val1, '|' as i32 as xmlChar)
                    != 0 as *mut libc::c_void as *const xmlChar) as libc::c_int;
                let mut compound2: libc::c_int = (xmlStrchr(val2, '|' as i32 as xmlChar)
                    != 0 as *mut libc::c_void as *const xmlChar) as libc::c_int;
                if compound1 != compound2 {
                    return 0 as libc::c_int;
                }
                ret = xmlRegStrEqualWildcard(val1, val2);
            }
            current_block = 3686881736565329710;
        }
        2 => {
            if (*atom2).type_0 as libc::c_uint
                == XML_REGEXP_CHARVAL as libc::c_int as libc::c_uint
            {
                ret = ((*atom1).codepoint == (*atom2).codepoint) as libc::c_int;
            } else {
                ret = xmlRegCheckCharacter(atom2, (*atom1).codepoint);
                if ret < 0 as libc::c_int {
                    ret = 1 as libc::c_int;
                }
            }
            current_block = 3686881736565329710;
        }
        3 => {
            if (*atom2).type_0 as libc::c_uint
                == XML_REGEXP_RANGES as libc::c_int as libc::c_uint
            {
                let mut i: libc::c_int = 0;
                let mut j: libc::c_int = 0;
                let mut res: libc::c_int = 0;
                let mut r1: xmlRegRangePtr = 0 as *mut xmlRegRange;
                let mut r2: xmlRegRangePtr = 0 as *mut xmlRegRange;
                i = 0 as libc::c_int;
                's_172: loop {
                    if !(i < (*atom1).nbRanges) {
                        current_block = 3938820862080741272;
                        break;
                    }
                    j = 0 as libc::c_int;
                    while j < (*atom2).nbRanges {
                        r1 = *((*atom1).ranges).offset(i as isize);
                        r2 = *((*atom2).ranges).offset(j as isize);
                        res = xmlFACompareRanges(r1, r2);
                        if res == 1 as libc::c_int {
                            ret = 1 as libc::c_int;
                            current_block = 3686881736565329710;
                            break 's_172;
                        } else {
                            j += 1;
                        }
                    }
                    i += 1;
                }
                match current_block {
                    3686881736565329710 => {}
                    _ => {
                        ret = 0 as libc::c_int;
                        current_block = 3686881736565329710;
                    }
                }
            } else {
                current_block = 3686881736565329710;
            }
        }
        1 | _ => {
            current_block = 10156241871818535325;
        }
    }
    match current_block {
        3686881736565329710 => {
            if (*atom1).neg != (*atom2).neg {
                ret = (ret == 0) as libc::c_int;
            }
            if ret == 0 as libc::c_int {
                return 0 as libc::c_int;
            }
        }
        _ => {}
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn xmlFARecurseDeterminism(
    mut ctxt: xmlRegParserCtxtPtr,
    mut state: xmlRegStatePtr,
    mut to: libc::c_int,
    mut atom: xmlRegAtomPtr,
) -> libc::c_int {
    let mut ret: libc::c_int = 1 as libc::c_int;
    let mut res: libc::c_int = 0;
    let mut transnr: libc::c_int = 0;
    let mut nbTrans: libc::c_int = 0;
    let mut t1: xmlRegTransPtr = 0 as *mut xmlRegTrans;
    let mut deep: libc::c_int = 1 as libc::c_int;
    if state.is_null() {
        return ret;
    }
    if (*state).markd as libc::c_uint
        == XML_REGEXP_MARK_VISITED as libc::c_int as libc::c_uint
    {
        return ret;
    }
    if (*ctxt).flags & 1 as libc::c_int != 0 {
        deep = 0 as libc::c_int;
    }
    nbTrans = (*state).nbTrans;
    transnr = 0 as libc::c_int;
    while transnr < nbTrans {
        t1 = &mut *((*state).trans).offset(transnr as isize) as *mut xmlRegTrans;
        if ((*t1).atom).is_null() {
            if !((*t1).to < 0 as libc::c_int) {
                (*state).markd = XML_REGEXP_MARK_VISITED;
                res = xmlFARecurseDeterminism(
                    ctxt,
                    *((*ctxt).states).offset((*t1).to as isize),
                    to,
                    atom,
                );
                if res == 0 as libc::c_int {
                    ret = 0 as libc::c_int;
                }
            }
        } else if !((*t1).to != to) {
            if xmlFACompareAtoms((*t1).atom, atom, deep) != 0 {
                ret = 0 as libc::c_int;
                (*t1).nd = 1 as libc::c_int;
            }
        }
        transnr += 1;
    }
    return ret;
}
unsafe extern "C" fn xmlFAFinishRecurseDeterminism(
    mut ctxt: xmlRegParserCtxtPtr,
    mut state: xmlRegStatePtr,
) {
    let mut transnr: libc::c_int = 0;
    let mut nbTrans: libc::c_int = 0;
    if state.is_null() {
        return;
    }
    if (*state).markd as libc::c_uint
        != XML_REGEXP_MARK_VISITED as libc::c_int as libc::c_uint
    {
        return;
    }
    (*state).markd = XML_REGEXP_MARK_NORMAL;
    nbTrans = (*state).nbTrans;
    transnr = 0 as libc::c_int;
    while transnr < nbTrans {
        let mut t1: xmlRegTransPtr = &mut *((*state).trans).offset(transnr as isize)
            as *mut xmlRegTrans;
        if ((*t1).atom).is_null() && (*t1).to >= 0 as libc::c_int {
            xmlFAFinishRecurseDeterminism(
                ctxt,
                *((*ctxt).states).offset((*t1).to as isize),
            );
        }
        transnr += 1;
    }
}
unsafe extern "C" fn xmlFAComputesDeterminism(
    mut ctxt: xmlRegParserCtxtPtr,
) -> libc::c_int {
    let mut statenr: libc::c_int = 0;
    let mut transnr: libc::c_int = 0;
    let mut state: xmlRegStatePtr = 0 as *mut xmlRegState;
    let mut t1: xmlRegTransPtr = 0 as *mut xmlRegTrans;
    let mut t2: xmlRegTransPtr = 0 as *mut xmlRegTrans;
    let mut last: xmlRegTransPtr = 0 as *mut xmlRegTrans;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 1 as libc::c_int;
    let mut deep: libc::c_int = 1 as libc::c_int;
    if (*ctxt).determinist != -(1 as libc::c_int) {
        return (*ctxt).determinist;
    }
    if (*ctxt).flags & 1 as libc::c_int != 0 {
        deep = 0 as libc::c_int;
    }
    statenr = 0 as libc::c_int;
    while statenr < (*ctxt).nbStates {
        state = *((*ctxt).states).offset(statenr as isize);
        if !state.is_null() {
            if !((*state).nbTrans < 2 as libc::c_int) {
                transnr = 0 as libc::c_int;
                while transnr < (*state).nbTrans {
                    t1 = &mut *((*state).trans).offset(transnr as isize)
                        as *mut xmlRegTrans;
                    if !((*t1).atom).is_null() {
                        if !((*t1).to == -(1 as libc::c_int)) {
                            i = 0 as libc::c_int;
                            while i < transnr {
                                t2 = &mut *((*state).trans).offset(i as isize)
                                    as *mut xmlRegTrans;
                                if !((*t2).to == -(1 as libc::c_int)) {
                                    if !((*t2).atom).is_null() {
                                        if (*t1).to == (*t2).to {
                                            if xmlFAEqualAtoms((*t1).atom, (*t2).atom, deep) != 0
                                                && (*t1).counter == (*t2).counter
                                                && (*t1).count == (*t2).count
                                            {
                                                (*t2).to = -(1 as libc::c_int);
                                            }
                                        }
                                    }
                                }
                                i += 1;
                            }
                        }
                    }
                    transnr += 1;
                }
            }
        }
        statenr += 1;
    }
    statenr = 0 as libc::c_int;
    while statenr < (*ctxt).nbStates {
        state = *((*ctxt).states).offset(statenr as isize);
        if !state.is_null() {
            if !((*state).nbTrans < 2 as libc::c_int) {
                last = 0 as xmlRegTransPtr;
                transnr = 0 as libc::c_int;
                while transnr < (*state).nbTrans {
                    t1 = &mut *((*state).trans).offset(transnr as isize)
                        as *mut xmlRegTrans;
                    if !((*t1).atom).is_null() {
                        if !((*t1).to == -(1 as libc::c_int)) {
                            i = 0 as libc::c_int;
                            while i < transnr {
                                t2 = &mut *((*state).trans).offset(i as isize)
                                    as *mut xmlRegTrans;
                                if !((*t2).to == -(1 as libc::c_int)) {
                                    if !((*t2).atom).is_null() {
                                        if xmlFACompareAtoms(
                                            (*t1).atom,
                                            (*t2).atom,
                                            1 as libc::c_int,
                                        ) != 0
                                        {
                                            ret = 0 as libc::c_int;
                                            (*t1).nd = 1 as libc::c_int;
                                            (*t2).nd = 1 as libc::c_int;
                                            last = t1;
                                        }
                                    } else if (*t1).to != -(1 as libc::c_int) {
                                        ret = xmlFARecurseDeterminism(
                                            ctxt,
                                            *((*ctxt).states).offset((*t1).to as isize),
                                            (*t2).to,
                                            (*t2).atom,
                                        );
                                        xmlFAFinishRecurseDeterminism(
                                            ctxt,
                                            *((*ctxt).states).offset((*t1).to as isize),
                                        );
                                        if ret == 0 as libc::c_int {
                                            (*t1).nd = 1 as libc::c_int;
                                            last = t1;
                                        }
                                    }
                                }
                                i += 1;
                            }
                        }
                    }
                    transnr += 1;
                }
                if !last.is_null() {
                    (*last).nd = 2 as libc::c_int;
                }
            }
        }
        statenr += 1;
    }
    (*ctxt).determinist = ret;
    return ret;
}
unsafe extern "C" fn xmlRegCheckCharacterRange(
    mut type_0: xmlRegAtomType,
    mut codepoint: libc::c_int,
    mut neg: libc::c_int,
    mut start: libc::c_int,
    mut end: libc::c_int,
    mut blockName: *const xmlChar,
) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut current_block_53: u64;
    match type_0 as libc::c_uint {
        5 | 4 | 3 | 1 => return -(1 as libc::c_int),
        6 => {
            ret = (codepoint != '\n' as i32 && codepoint != '\r' as i32) as libc::c_int;
            current_block_53 = 3879520548144599102;
        }
        2 => {
            ret = (codepoint >= start && codepoint <= end) as libc::c_int;
            current_block_53 = 3879520548144599102;
        }
        8 => {
            neg = (neg == 0) as libc::c_int;
            current_block_53 = 1336241090893225463;
        }
        7 => {
            current_block_53 = 1336241090893225463;
        }
        10 => {
            neg = (neg == 0) as libc::c_int;
            current_block_53 = 465810355216019241;
        }
        9 => {
            current_block_53 = 465810355216019241;
        }
        12 => {
            neg = (neg == 0) as libc::c_int;
            current_block_53 = 10310224489676203593;
        }
        11 => {
            current_block_53 = 10310224489676203593;
        }
        14 => {
            neg = (neg == 0) as libc::c_int;
            current_block_53 = 7867894563361037860;
        }
        13 => {
            current_block_53 = 7867894563361037860;
        }
        15 => {
            neg = (neg == 0) as libc::c_int;
            current_block_53 = 4570984857387965468;
        }
        16 => {
            current_block_53 = 4570984857387965468;
        }
        100 => {
            ret = xmlUCSIsCatL(codepoint);
            current_block_53 = 3879520548144599102;
        }
        101 => {
            ret = xmlUCSIsCatLu(codepoint);
            current_block_53 = 3879520548144599102;
        }
        102 => {
            ret = xmlUCSIsCatLl(codepoint);
            current_block_53 = 3879520548144599102;
        }
        103 => {
            ret = xmlUCSIsCatLt(codepoint);
            current_block_53 = 3879520548144599102;
        }
        104 => {
            ret = xmlUCSIsCatLm(codepoint);
            current_block_53 = 3879520548144599102;
        }
        105 => {
            ret = xmlUCSIsCatLo(codepoint);
            current_block_53 = 3879520548144599102;
        }
        106 => {
            ret = xmlUCSIsCatM(codepoint);
            current_block_53 = 3879520548144599102;
        }
        107 => {
            ret = xmlUCSIsCatMn(codepoint);
            current_block_53 = 3879520548144599102;
        }
        108 => {
            ret = xmlUCSIsCatMc(codepoint);
            current_block_53 = 3879520548144599102;
        }
        109 => {
            ret = xmlUCSIsCatMe(codepoint);
            current_block_53 = 3879520548144599102;
        }
        110 => {
            ret = xmlUCSIsCatN(codepoint);
            current_block_53 = 3879520548144599102;
        }
        111 => {
            ret = xmlUCSIsCatNd(codepoint);
            current_block_53 = 3879520548144599102;
        }
        112 => {
            ret = xmlUCSIsCatNl(codepoint);
            current_block_53 = 3879520548144599102;
        }
        113 => {
            ret = xmlUCSIsCatNo(codepoint);
            current_block_53 = 3879520548144599102;
        }
        114 => {
            ret = xmlUCSIsCatP(codepoint);
            current_block_53 = 3879520548144599102;
        }
        115 => {
            ret = xmlUCSIsCatPc(codepoint);
            current_block_53 = 3879520548144599102;
        }
        116 => {
            ret = xmlUCSIsCatPd(codepoint);
            current_block_53 = 3879520548144599102;
        }
        117 => {
            ret = xmlUCSIsCatPs(codepoint);
            current_block_53 = 3879520548144599102;
        }
        118 => {
            ret = xmlUCSIsCatPe(codepoint);
            current_block_53 = 3879520548144599102;
        }
        119 => {
            ret = xmlUCSIsCatPi(codepoint);
            current_block_53 = 3879520548144599102;
        }
        120 => {
            ret = xmlUCSIsCatPf(codepoint);
            current_block_53 = 3879520548144599102;
        }
        121 => {
            ret = xmlUCSIsCatPo(codepoint);
            current_block_53 = 3879520548144599102;
        }
        122 => {
            ret = xmlUCSIsCatZ(codepoint);
            current_block_53 = 3879520548144599102;
        }
        123 => {
            ret = xmlUCSIsCatZs(codepoint);
            current_block_53 = 3879520548144599102;
        }
        124 => {
            ret = xmlUCSIsCatZl(codepoint);
            current_block_53 = 3879520548144599102;
        }
        125 => {
            ret = xmlUCSIsCatZp(codepoint);
            current_block_53 = 3879520548144599102;
        }
        126 => {
            ret = xmlUCSIsCatS(codepoint);
            current_block_53 = 3879520548144599102;
        }
        127 => {
            ret = xmlUCSIsCatSm(codepoint);
            current_block_53 = 3879520548144599102;
        }
        128 => {
            ret = xmlUCSIsCatSc(codepoint);
            current_block_53 = 3879520548144599102;
        }
        129 => {
            ret = xmlUCSIsCatSk(codepoint);
            current_block_53 = 3879520548144599102;
        }
        130 => {
            ret = xmlUCSIsCatSo(codepoint);
            current_block_53 = 3879520548144599102;
        }
        131 => {
            ret = xmlUCSIsCatC(codepoint);
            current_block_53 = 3879520548144599102;
        }
        132 => {
            ret = xmlUCSIsCatCc(codepoint);
            current_block_53 = 3879520548144599102;
        }
        133 => {
            ret = xmlUCSIsCatCf(codepoint);
            current_block_53 = 3879520548144599102;
        }
        134 => {
            ret = xmlUCSIsCatCo(codepoint);
            current_block_53 = 3879520548144599102;
        }
        135 => {
            ret = 0 as libc::c_int;
            current_block_53 = 3879520548144599102;
        }
        136 => {
            ret = xmlUCSIsBlock(codepoint, blockName as *const libc::c_char);
            current_block_53 = 3879520548144599102;
        }
        _ => {
            current_block_53 = 3879520548144599102;
        }
    }
    match current_block_53 {
        4570984857387965468 => {
            ret = xmlUCSIsCatP(codepoint);
            if ret == 0 as libc::c_int {
                ret = xmlUCSIsCatZ(codepoint);
            }
            if ret == 0 as libc::c_int {
                ret = xmlUCSIsCatC(codepoint);
            }
        }
        1336241090893225463 => {
            ret = (codepoint == '\n' as i32 || codepoint == '\r' as i32
                || codepoint == '\t' as i32 || codepoint == ' ' as i32) as libc::c_int;
        }
        465810355216019241 => {
            ret = ((if codepoint < 0x100 as libc::c_int {
                (0x41 as libc::c_int <= codepoint && codepoint <= 0x5a as libc::c_int
                    || 0x61 as libc::c_int <= codepoint
                        && codepoint <= 0x7a as libc::c_int
                    || 0xc0 as libc::c_int <= codepoint
                        && codepoint <= 0xd6 as libc::c_int
                    || 0xd8 as libc::c_int <= codepoint
                        && codepoint <= 0xf6 as libc::c_int
                    || 0xf8 as libc::c_int <= codepoint) as libc::c_int
            } else {
                xmlCharInRange(codepoint as libc::c_uint, &xmlIsBaseCharGroup)
            }) != 0
                || (if codepoint < 0x100 as libc::c_int {
                    0 as libc::c_int
                } else {
                    (0x4e00 as libc::c_int <= codepoint
                        && codepoint <= 0x9fa5 as libc::c_int
                        || codepoint == 0x3007 as libc::c_int
                        || 0x3021 as libc::c_int <= codepoint
                            && codepoint <= 0x3029 as libc::c_int) as libc::c_int
                }) != 0 || codepoint == '_' as i32 || codepoint == ':' as i32)
                as libc::c_int;
        }
        10310224489676203593 => {
            ret = ((if codepoint < 0x100 as libc::c_int {
                (0x41 as libc::c_int <= codepoint && codepoint <= 0x5a as libc::c_int
                    || 0x61 as libc::c_int <= codepoint
                        && codepoint <= 0x7a as libc::c_int
                    || 0xc0 as libc::c_int <= codepoint
                        && codepoint <= 0xd6 as libc::c_int
                    || 0xd8 as libc::c_int <= codepoint
                        && codepoint <= 0xf6 as libc::c_int
                    || 0xf8 as libc::c_int <= codepoint) as libc::c_int
            } else {
                xmlCharInRange(codepoint as libc::c_uint, &xmlIsBaseCharGroup)
            }) != 0
                || (if codepoint < 0x100 as libc::c_int {
                    0 as libc::c_int
                } else {
                    (0x4e00 as libc::c_int <= codepoint
                        && codepoint <= 0x9fa5 as libc::c_int
                        || codepoint == 0x3007 as libc::c_int
                        || 0x3021 as libc::c_int <= codepoint
                            && codepoint <= 0x3029 as libc::c_int) as libc::c_int
                }) != 0
                || (if codepoint < 0x100 as libc::c_int {
                    (0x30 as libc::c_int <= codepoint
                        && codepoint <= 0x39 as libc::c_int) as libc::c_int
                } else {
                    xmlCharInRange(codepoint as libc::c_uint, &xmlIsDigitGroup)
                }) != 0 || codepoint == '.' as i32 || codepoint == '-' as i32
                || codepoint == '_' as i32 || codepoint == ':' as i32
                || (if codepoint < 0x100 as libc::c_int {
                    0 as libc::c_int
                } else {
                    xmlCharInRange(codepoint as libc::c_uint, &xmlIsCombiningGroup)
                }) != 0
                || (if codepoint < 0x100 as libc::c_int {
                    (codepoint == 0xb7 as libc::c_int) as libc::c_int
                } else {
                    xmlCharInRange(codepoint as libc::c_uint, &xmlIsExtenderGroup)
                }) != 0) as libc::c_int;
        }
        7867894563361037860 => {
            ret = xmlUCSIsCatNd(codepoint);
        }
        _ => {}
    }
    if neg != 0 {
        return (ret == 0) as libc::c_int;
    }
    return ret;
}
unsafe extern "C" fn xmlRegCheckCharacter(
    mut atom: xmlRegAtomPtr,
    mut codepoint: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut range: xmlRegRangePtr = 0 as *mut xmlRegRange;
    if atom.is_null()
        || (if codepoint < 0x100 as libc::c_int {
            (0x9 as libc::c_int <= codepoint && codepoint <= 0xa as libc::c_int
                || codepoint == 0xd as libc::c_int || 0x20 as libc::c_int <= codepoint)
                as libc::c_int
        } else {
            (0x100 as libc::c_int <= codepoint && codepoint <= 0xd7ff as libc::c_int
                || 0xe000 as libc::c_int <= codepoint
                    && codepoint <= 0xfffd as libc::c_int
                || 0x10000 as libc::c_int <= codepoint
                    && codepoint <= 0x10ffff as libc::c_int) as libc::c_int
        }) == 0
    {
        return -(1 as libc::c_int);
    }
    match (*atom).type_0 as libc::c_uint {
        4 | 1 => return -(1 as libc::c_int),
        2 => return (codepoint == (*atom).codepoint) as libc::c_int,
        3 => {
            let mut accept: libc::c_int = 0 as libc::c_int;
            i = 0 as libc::c_int;
            while i < (*atom).nbRanges {
                range = *((*atom).ranges).offset(i as isize);
                if (*range).neg == 2 as libc::c_int {
                    ret = xmlRegCheckCharacterRange(
                        (*range).type_0,
                        codepoint,
                        0 as libc::c_int,
                        (*range).start,
                        (*range).end,
                        (*range).blockName,
                    );
                    if ret != 0 as libc::c_int {
                        return 0 as libc::c_int;
                    }
                } else if (*range).neg != 0 {
                    ret = xmlRegCheckCharacterRange(
                        (*range).type_0,
                        codepoint,
                        0 as libc::c_int,
                        (*range).start,
                        (*range).end,
                        (*range).blockName,
                    );
                    if ret == 0 as libc::c_int {
                        accept = 1 as libc::c_int;
                    } else {
                        return 0 as libc::c_int
                    }
                } else {
                    ret = xmlRegCheckCharacterRange(
                        (*range).type_0,
                        codepoint,
                        0 as libc::c_int,
                        (*range).start,
                        (*range).end,
                        (*range).blockName,
                    );
                    if ret != 0 as libc::c_int {
                        accept = 1 as libc::c_int;
                    }
                }
                i += 1;
            }
            return accept;
        }
        5 => {
            printf(b"TODO: XML_REGEXP_STRING\n\0" as *const u8 as *const libc::c_char);
            return -(1 as libc::c_int);
        }
        6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 100 | 101 | 102 | 103 | 104
        | 105 | 106 | 107 | 108 | 109 | 110 | 111 | 112 | 113 | 114 | 115 | 116 | 117
        | 118 | 119 | 120 | 121 | 122 | 123 | 124 | 125 | 126 | 127 | 128 | 129 | 130
        | 131 | 132 | 133 | 134 | 135 | 136 => {
            ret = xmlRegCheckCharacterRange(
                (*atom).type_0,
                codepoint,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                (*atom).valuep as *const xmlChar,
            );
            if (*atom).neg != 0 {
                ret = (ret == 0) as libc::c_int;
            }
        }
        _ => {}
    }
    return ret;
}
unsafe extern "C" fn xmlFARegExecSave(mut exec: xmlRegExecCtxtPtr) {
    if (*exec).nbPush > 10000000 as libc::c_int {
        return;
    }
    let ref mut fresh56 = (*exec).nbPush;
    *fresh56 += 1;
    if (*exec).maxRollbacks == 0 as libc::c_int {
        (*exec).maxRollbacks = 4 as libc::c_int;
        let ref mut fresh57 = (*exec).rollbacks;
        *fresh57 = xmlMalloc
            .expect(
                "non-null function pointer",
            )(
            ((*exec).maxRollbacks as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<xmlRegExecRollback>() as libc::c_ulong,
                ),
        ) as *mut xmlRegExecRollback;
        if ((*exec).rollbacks).is_null() {
            xmlRegexpErrMemory(
                0 as xmlRegParserCtxtPtr,
                b"saving regexp\0" as *const u8 as *const libc::c_char,
            );
            (*exec).maxRollbacks = 0 as libc::c_int;
            return;
        }
        memset(
            (*exec).rollbacks as *mut libc::c_void,
            0 as libc::c_int,
            ((*exec).maxRollbacks as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<xmlRegExecRollback>() as libc::c_ulong,
                ),
        );
    } else if (*exec).nbRollbacks >= (*exec).maxRollbacks {
        let mut tmp: *mut xmlRegExecRollback = 0 as *mut xmlRegExecRollback;
        let mut len: libc::c_int = (*exec).maxRollbacks;
        (*exec).maxRollbacks *= 2 as libc::c_int;
        tmp = xmlRealloc
            .expect(
                "non-null function pointer",
            )(
            (*exec).rollbacks as *mut libc::c_void,
            ((*exec).maxRollbacks as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<xmlRegExecRollback>() as libc::c_ulong,
                ),
        ) as *mut xmlRegExecRollback;
        if tmp.is_null() {
            xmlRegexpErrMemory(
                0 as xmlRegParserCtxtPtr,
                b"saving regexp\0" as *const u8 as *const libc::c_char,
            );
            (*exec).maxRollbacks /= 2 as libc::c_int;
            return;
        }
        let ref mut fresh58 = (*exec).rollbacks;
        *fresh58 = tmp;
        tmp = &mut *((*exec).rollbacks).offset(len as isize) as *mut xmlRegExecRollback;
        memset(
            tmp as *mut libc::c_void,
            0 as libc::c_int,
            (((*exec).maxRollbacks - len) as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<xmlRegExecRollback>() as libc::c_ulong,
                ),
        );
    }
    let ref mut fresh59 = (*((*exec).rollbacks).offset((*exec).nbRollbacks as isize))
        .state;
    *fresh59 = (*exec).state;
    (*((*exec).rollbacks).offset((*exec).nbRollbacks as isize)).index = (*exec).index;
    (*((*exec).rollbacks).offset((*exec).nbRollbacks as isize))
        .nextbranch = (*exec).transno + 1 as libc::c_int;
    if (*(*exec).comp).nbCounters > 0 as libc::c_int {
        if ((*((*exec).rollbacks).offset((*exec).nbRollbacks as isize)).counts).is_null()
        {
            let ref mut fresh60 = (*((*exec).rollbacks)
                .offset((*exec).nbRollbacks as isize))
                .counts;
            *fresh60 = xmlMalloc
                .expect(
                    "non-null function pointer",
                )(
                ((*(*exec).comp).nbCounters as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
            ) as *mut libc::c_int;
            if ((*((*exec).rollbacks).offset((*exec).nbRollbacks as isize)).counts)
                .is_null()
            {
                xmlRegexpErrMemory(
                    0 as xmlRegParserCtxtPtr,
                    b"saving regexp\0" as *const u8 as *const libc::c_char,
                );
                (*exec).status = -(5 as libc::c_int);
                return;
            }
        }
        memcpy(
            (*((*exec).rollbacks).offset((*exec).nbRollbacks as isize)).counts
                as *mut libc::c_void,
            (*exec).counts as *const libc::c_void,
            ((*(*exec).comp).nbCounters as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
    }
    let ref mut fresh61 = (*exec).nbRollbacks;
    *fresh61 += 1;
}
unsafe extern "C" fn xmlFARegExecRollBack(mut exec: xmlRegExecCtxtPtr) {
    if (*exec).nbRollbacks <= 0 as libc::c_int {
        (*exec).status = -(1 as libc::c_int);
        return;
    }
    let ref mut fresh62 = (*exec).nbRollbacks;
    *fresh62 -= 1;
    let ref mut fresh63 = (*exec).state;
    *fresh63 = (*((*exec).rollbacks).offset((*exec).nbRollbacks as isize)).state;
    (*exec).index = (*((*exec).rollbacks).offset((*exec).nbRollbacks as isize)).index;
    (*exec)
        .transno = (*((*exec).rollbacks).offset((*exec).nbRollbacks as isize))
        .nextbranch;
    if (*(*exec).comp).nbCounters > 0 as libc::c_int {
        if ((*((*exec).rollbacks).offset((*exec).nbRollbacks as isize)).counts).is_null()
        {
            fprintf(
                stderr,
                b"exec save: allocation failed\0" as *const u8 as *const libc::c_char,
            );
            (*exec).status = -(6 as libc::c_int);
            return;
        }
        if !((*exec).counts).is_null() {
            memcpy(
                (*exec).counts as *mut libc::c_void,
                (*((*exec).rollbacks).offset((*exec).nbRollbacks as isize)).counts
                    as *const libc::c_void,
                ((*(*exec).comp).nbCounters as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
            );
        }
    }
}
unsafe extern "C" fn xmlFARegExec(
    mut comp: xmlRegexpPtr,
    mut content: *const xmlChar,
) -> libc::c_int {
    let mut current_block: u64;
    let mut execval: xmlRegExecCtxt = xmlRegExecCtxt {
        status: 0,
        determinist: 0,
        comp: 0 as *mut xmlRegexp,
        callback: None,
        data: 0 as *mut libc::c_void,
        state: 0 as *mut xmlRegState,
        transno: 0,
        transcount: 0,
        maxRollbacks: 0,
        nbRollbacks: 0,
        rollbacks: 0 as *mut xmlRegExecRollback,
        counts: 0 as *mut libc::c_int,
        inputStackMax: 0,
        inputStackNr: 0,
        index: 0,
        charStack: 0 as *mut libc::c_int,
        inputString: 0 as *const xmlChar,
        inputStack: 0 as *mut xmlRegInputToken,
        errStateNo: 0,
        errState: 0 as *mut xmlRegState,
        errString: 0 as *mut xmlChar,
        errCounts: 0 as *mut libc::c_int,
        nbPush: 0,
    };
    let mut exec: xmlRegExecCtxtPtr = &mut execval;
    let mut ret: libc::c_int = 0;
    let mut codepoint: libc::c_int = 0 as libc::c_int;
    let mut len: libc::c_int = 0;
    let mut deter: libc::c_int = 0;
    let ref mut fresh64 = (*exec).inputString;
    *fresh64 = content;
    (*exec).index = 0 as libc::c_int;
    (*exec).nbPush = 0 as libc::c_int;
    (*exec).determinist = 1 as libc::c_int;
    (*exec).maxRollbacks = 0 as libc::c_int;
    (*exec).nbRollbacks = 0 as libc::c_int;
    let ref mut fresh65 = (*exec).rollbacks;
    *fresh65 = 0 as *mut xmlRegExecRollback;
    (*exec).status = 0 as libc::c_int;
    let ref mut fresh66 = (*exec).comp;
    *fresh66 = comp;
    let ref mut fresh67 = (*exec).state;
    *fresh67 = *((*comp).states).offset(0 as libc::c_int as isize);
    (*exec).transno = 0 as libc::c_int;
    (*exec).transcount = 0 as libc::c_int;
    let ref mut fresh68 = (*exec).inputStack;
    *fresh68 = 0 as xmlRegInputTokenPtr;
    (*exec).inputStackMax = 0 as libc::c_int;
    if (*comp).nbCounters > 0 as libc::c_int {
        let ref mut fresh69 = (*exec).counts;
        *fresh69 = xmlMalloc
            .expect(
                "non-null function pointer",
            )(
            ((*comp).nbCounters as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        if ((*exec).counts).is_null() {
            xmlRegexpErrMemory(
                0 as xmlRegParserCtxtPtr,
                b"running regexp\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        memset(
            (*exec).counts as *mut libc::c_void,
            0 as libc::c_int,
            ((*comp).nbCounters as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
    } else {
        let ref mut fresh70 = (*exec).counts;
        *fresh70 = 0 as *mut libc::c_int;
    }
    's_105: while (*exec).status == 0 as libc::c_int && !((*exec).state).is_null()
        && (*((*exec).inputString).offset((*exec).index as isize) as libc::c_int
            != 0 as libc::c_int
            || !((*exec).state).is_null()
                && (*(*exec).state).type_0 as libc::c_uint
                    != XML_REGEXP_FINAL_STATE as libc::c_int as libc::c_uint)
    {
        let mut trans: xmlRegTransPtr = 0 as *mut xmlRegTrans;
        let mut atom: xmlRegAtomPtr = 0 as *mut xmlRegAtom;
        len = 1 as libc::c_int;
        if *((*exec).inputString).offset((*exec).index as isize) as libc::c_int
            == 0 as libc::c_int && ((*exec).counts).is_null()
        {
            if (*exec).transno < (*(*exec).state).nbTrans {
                trans = &mut *((*(*exec).state).trans).offset((*exec).transno as isize)
                    as *mut xmlRegTrans;
                if (*trans).to >= 0 as libc::c_int {
                    atom = (*trans).atom;
                    if !((*atom).min == 0 as libc::c_int
                        && (*atom).max > 0 as libc::c_int)
                    {
                        current_block = 17840493588698777309;
                    } else {
                        current_block = 17281240262373992796;
                    }
                } else {
                    current_block = 17281240262373992796;
                }
            } else {
                current_block = 17840493588698777309;
            }
        } else {
            current_block = 17281240262373992796;
        }
        match current_block {
            17281240262373992796 => {
                (*exec).transcount = 0 as libc::c_int;
                loop {
                    if !((*exec).transno < (*(*exec).state).nbTrans) {
                        current_block = 2872334340672008580;
                        break;
                    }
                    trans = &mut *((*(*exec).state).trans)
                        .offset((*exec).transno as isize) as *mut xmlRegTrans;
                    if !((*trans).to < 0 as libc::c_int) {
                        atom = (*trans).atom;
                        ret = 0 as libc::c_int;
                        deter = 1 as libc::c_int;
                        if (*trans).count >= 0 as libc::c_int {
                            let mut count: libc::c_int = 0;
                            let mut counter: xmlRegCounterPtr = 0 as *mut xmlRegCounter;
                            if ((*exec).counts).is_null() {
                                (*exec).status = -(1 as libc::c_int);
                                break 's_105;
                            } else {
                                count = *((*exec).counts).offset((*trans).count as isize);
                                counter = &mut *((*(*exec).comp).counters)
                                    .offset((*trans).count as isize) as *mut xmlRegCounter;
                                ret = (count >= (*counter).min && count <= (*counter).max)
                                    as libc::c_int;
                                if ret != 0 && (*counter).min != (*counter).max {
                                    deter = 0 as libc::c_int;
                                }
                            }
                            current_block = 14155412868135599428;
                        } else if atom.is_null() {
                            fprintf(
                                stderr,
                                b"epsilon transition left at runtime\n\0" as *const u8
                                    as *const libc::c_char,
                            );
                            (*exec).status = -(2 as libc::c_int);
                            current_block = 2872334340672008580;
                            break;
                        } else if *((*exec).inputString).offset((*exec).index as isize)
                                as libc::c_int != 0 as libc::c_int
                            {
                            codepoint = xmlStringCurrentChar(
                                0 as xmlParserCtxtPtr,
                                &*((*exec).inputString).offset((*exec).index as isize),
                                &mut len,
                            );
                            ret = xmlRegCheckCharacter(atom, codepoint);
                            if ret == 1 as libc::c_int && (*atom).min >= 0 as libc::c_int
                                && (*atom).max > 0 as libc::c_int
                            {
                                let mut to: xmlRegStatePtr = *((*comp).states)
                                    .offset((*trans).to as isize);
                                if (*trans).counter >= 0 as libc::c_int {
                                    let mut counter_0: xmlRegCounterPtr = 0
                                        as *mut xmlRegCounter;
                                    if ((*exec).counts).is_null() || ((*exec).comp).is_null()
                                        || ((*(*exec).comp).counters).is_null()
                                    {
                                        (*exec).status = -(1 as libc::c_int);
                                        break 's_105;
                                    } else {
                                        counter_0 = &mut *((*(*exec).comp).counters)
                                            .offset((*trans).counter as isize) as *mut xmlRegCounter;
                                        if *((*exec).counts).offset((*trans).counter as isize)
                                            >= (*counter_0).max
                                        {
                                            current_block = 17500079516916021833;
                                        } else {
                                            current_block = 3546145585875536353;
                                        }
                                    }
                                } else {
                                    current_block = 3546145585875536353;
                                }
                                match current_block {
                                    17500079516916021833 => {}
                                    _ => {
                                        if (*(*exec).state).nbTrans
                                            > (*exec).transno + 1 as libc::c_int
                                        {
                                            xmlFARegExecSave(exec);
                                        }
                                        if (*trans).counter >= 0 as libc::c_int {
                                            let ref mut fresh71 = *((*exec).counts)
                                                .offset((*trans).counter as isize);
                                            *fresh71 += 1;
                                        }
                                        (*exec).transcount = 1 as libc::c_int;
                                        while !((*exec).transcount == (*atom).max) {
                                            (*exec).index += len;
                                            if *((*exec).inputString).offset((*exec).index as isize)
                                                as libc::c_int == 0 as libc::c_int
                                            {
                                                (*exec).index -= len;
                                                break;
                                            } else {
                                                if (*exec).transcount >= (*atom).min {
                                                    let mut transno: libc::c_int = (*exec).transno;
                                                    let mut state: xmlRegStatePtr = (*exec).state;
                                                    (*exec).transno = -(1 as libc::c_int);
                                                    let ref mut fresh72 = (*exec).state;
                                                    *fresh72 = to;
                                                    xmlFARegExecSave(exec);
                                                    (*exec).transno = transno;
                                                    let ref mut fresh73 = (*exec).state;
                                                    *fresh73 = state;
                                                }
                                                codepoint = xmlStringCurrentChar(
                                                    0 as xmlParserCtxtPtr,
                                                    &*((*exec).inputString).offset((*exec).index as isize),
                                                    &mut len,
                                                );
                                                ret = xmlRegCheckCharacter(atom, codepoint);
                                                let ref mut fresh74 = (*exec).transcount;
                                                *fresh74 += 1;
                                                if !(ret == 1 as libc::c_int) {
                                                    break;
                                                }
                                            }
                                        }
                                        if (*exec).transcount < (*atom).min {
                                            ret = 0 as libc::c_int;
                                        }
                                        if ret < 0 as libc::c_int {
                                            ret = 0 as libc::c_int;
                                        }
                                        if ret == 0 as libc::c_int {
                                            current_block = 17840493588698777309;
                                            break;
                                        }
                                        if (*trans).counter >= 0 as libc::c_int {
                                            if ((*exec).counts).is_null() {
                                                (*exec).status = -(1 as libc::c_int);
                                                break 's_105;
                                            } else {
                                                let ref mut fresh75 = *((*exec).counts)
                                                    .offset((*trans).counter as isize);
                                                *fresh75 -= 1;
                                            }
                                            current_block = 14155412868135599428;
                                        } else {
                                            current_block = 14155412868135599428;
                                        }
                                    }
                                }
                            } else {
                                if ret == 0 as libc::c_int
                                    && (*atom).min == 0 as libc::c_int
                                    && (*atom).max > 0 as libc::c_int
                                {
                                    (*exec).transcount = 1 as libc::c_int;
                                    len = 0 as libc::c_int;
                                    ret = 1 as libc::c_int;
                                }
                                current_block = 14155412868135599428;
                            }
                        } else {
                            if (*atom).min == 0 as libc::c_int
                                && (*atom).max > 0 as libc::c_int
                            {
                                (*exec).transcount = 1 as libc::c_int;
                                len = 0 as libc::c_int;
                                ret = 1 as libc::c_int;
                            }
                            current_block = 14155412868135599428;
                        }
                        match current_block {
                            17500079516916021833 => {}
                            _ => {
                                if ret == 1 as libc::c_int {
                                    if (*trans).nd == 1 as libc::c_int
                                        || (*trans).count >= 0 as libc::c_int
                                            && deter == 0 as libc::c_int
                                            && (*(*exec).state).nbTrans
                                                > (*exec).transno + 1 as libc::c_int
                                    {
                                        xmlFARegExecSave(exec);
                                    }
                                    if (*trans).counter >= 0 as libc::c_int {
                                        let mut counter_1: xmlRegCounterPtr = 0
                                            as *mut xmlRegCounter;
                                        if ((*exec).counts).is_null() || ((*exec).comp).is_null()
                                            || ((*(*exec).comp).counters).is_null()
                                        {
                                            (*exec).status = -(1 as libc::c_int);
                                            break 's_105;
                                        } else {
                                            counter_1 = &mut *((*(*exec).comp).counters)
                                                .offset((*trans).counter as isize) as *mut xmlRegCounter;
                                            if *((*exec).counts).offset((*trans).counter as isize)
                                                >= (*counter_1).max
                                            {
                                                current_block = 17500079516916021833;
                                            } else {
                                                let ref mut fresh76 = *((*exec).counts)
                                                    .offset((*trans).counter as isize);
                                                *fresh76 += 1;
                                                current_block = 11718254377427810743;
                                            }
                                        }
                                    } else {
                                        current_block = 11718254377427810743;
                                    }
                                    match current_block {
                                        17500079516916021833 => {}
                                        _ => {
                                            if (*trans).count >= 0 as libc::c_int
                                                && (*trans).count < 0x123456 as libc::c_int
                                            {
                                                if ((*exec).counts).is_null() {
                                                    (*exec).status = -(1 as libc::c_int);
                                                    break 's_105;
                                                } else {
                                                    *((*exec).counts)
                                                        .offset((*trans).count as isize) = 0 as libc::c_int;
                                                }
                                            }
                                            let ref mut fresh77 = (*exec).state;
                                            *fresh77 = *((*comp).states).offset((*trans).to as isize);
                                            (*exec).transno = 0 as libc::c_int;
                                            if !((*trans).atom).is_null() {
                                                (*exec).index += len;
                                            }
                                            continue 's_105;
                                        }
                                    }
                                } else if ret < 0 as libc::c_int {
                                    (*exec).status = -(4 as libc::c_int);
                                    current_block = 2872334340672008580;
                                    break;
                                }
                            }
                        }
                    }
                    let ref mut fresh78 = (*exec).transno;
                    *fresh78 += 1;
                }
                match current_block {
                    17840493588698777309 => {}
                    _ => {
                        if !((*exec).transno != 0 as libc::c_int
                            || (*(*exec).state).nbTrans == 0 as libc::c_int)
                        {
                            continue;
                        }
                    }
                }
            }
            _ => {}
        }
        (*exec).determinist = 0 as libc::c_int;
        xmlFARegExecRollBack(exec);
    }
    if !((*exec).rollbacks).is_null() {
        if !((*exec).counts).is_null() {
            let mut i: libc::c_int = 0;
            i = 0 as libc::c_int;
            while i < (*exec).maxRollbacks {
                if !((*((*exec).rollbacks).offset(i as isize)).counts).is_null() {
                    xmlFree
                        .expect(
                            "non-null function pointer",
                        )(
                        (*((*exec).rollbacks).offset(i as isize)).counts
                            as *mut libc::c_void,
                    );
                }
                i += 1;
            }
        }
        xmlFree
            .expect("non-null function pointer")((*exec).rollbacks as *mut libc::c_void);
    }
    if ((*exec).state).is_null() {
        return -(1 as libc::c_int);
    }
    if !((*exec).counts).is_null() {
        xmlFree.expect("non-null function pointer")((*exec).counts as *mut libc::c_void);
    }
    if (*exec).status == 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    if (*exec).status == -(1 as libc::c_int) {
        if (*exec).nbPush > 10000000 as libc::c_int {
            return -(1 as libc::c_int);
        }
        return 0 as libc::c_int;
    }
    return (*exec).status;
}
#[no_mangle]
pub unsafe extern "C" fn xmlRegNewExecCtxt(
    mut comp: xmlRegexpPtr,
    mut callback: xmlRegExecCallbacks,
    mut data: *mut libc::c_void,
) -> xmlRegExecCtxtPtr {
    let mut exec: xmlRegExecCtxtPtr = 0 as *mut xmlRegExecCtxt;
    if comp.is_null() {
        return 0 as xmlRegExecCtxtPtr;
    }
    if ((*comp).compact).is_null() && ((*comp).states).is_null() {
        return 0 as xmlRegExecCtxtPtr;
    }
    exec = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::std::mem::size_of::<xmlRegExecCtxt>() as libc::c_ulong) as xmlRegExecCtxtPtr;
    if exec.is_null() {
        xmlRegexpErrMemory(
            0 as xmlRegParserCtxtPtr,
            b"creating execution context\0" as *const u8 as *const libc::c_char,
        );
        return 0 as xmlRegExecCtxtPtr;
    }
    memset(
        exec as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<xmlRegExecCtxt>() as libc::c_ulong,
    );
    let ref mut fresh79 = (*exec).inputString;
    *fresh79 = 0 as *const xmlChar;
    (*exec).index = 0 as libc::c_int;
    (*exec).determinist = 1 as libc::c_int;
    (*exec).maxRollbacks = 0 as libc::c_int;
    (*exec).nbRollbacks = 0 as libc::c_int;
    let ref mut fresh80 = (*exec).rollbacks;
    *fresh80 = 0 as *mut xmlRegExecRollback;
    (*exec).status = 0 as libc::c_int;
    let ref mut fresh81 = (*exec).comp;
    *fresh81 = comp;
    if ((*comp).compact).is_null() {
        let ref mut fresh82 = (*exec).state;
        *fresh82 = *((*comp).states).offset(0 as libc::c_int as isize);
    }
    (*exec).transno = 0 as libc::c_int;
    (*exec).transcount = 0 as libc::c_int;
    let ref mut fresh83 = (*exec).callback;
    *fresh83 = callback;
    let ref mut fresh84 = (*exec).data;
    *fresh84 = data;
    if (*comp).nbCounters > 0 as libc::c_int {
        let ref mut fresh85 = (*exec).counts;
        *fresh85 = xmlMalloc
            .expect(
                "non-null function pointer",
            )(
            ((*comp).nbCounters as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_int;
        if ((*exec).counts).is_null() {
            xmlRegexpErrMemory(
                0 as xmlRegParserCtxtPtr,
                b"creating execution context\0" as *const u8 as *const libc::c_char,
            );
            xmlFree.expect("non-null function pointer")(exec as *mut libc::c_void);
            return 0 as xmlRegExecCtxtPtr;
        }
        memset(
            (*exec).counts as *mut libc::c_void,
            0 as libc::c_int,
            ((*comp).nbCounters as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong),
        );
        let ref mut fresh86 = (*exec).errCounts;
        *fresh86 = &mut *((*exec).counts).offset((*comp).nbCounters as isize)
            as *mut libc::c_int;
    } else {
        let ref mut fresh87 = (*exec).counts;
        *fresh87 = 0 as *mut libc::c_int;
        let ref mut fresh88 = (*exec).errCounts;
        *fresh88 = 0 as *mut libc::c_int;
    }
    (*exec).inputStackMax = 0 as libc::c_int;
    (*exec).inputStackNr = 0 as libc::c_int;
    let ref mut fresh89 = (*exec).inputStack;
    *fresh89 = 0 as xmlRegInputTokenPtr;
    (*exec).errStateNo = -(1 as libc::c_int);
    let ref mut fresh90 = (*exec).errString;
    *fresh90 = 0 as *mut xmlChar;
    (*exec).nbPush = 0 as libc::c_int;
    return exec;
}
#[no_mangle]
pub unsafe extern "C" fn xmlRegFreeExecCtxt(mut exec: xmlRegExecCtxtPtr) {
    if exec.is_null() {
        return;
    }
    if !((*exec).rollbacks).is_null() {
        if !((*exec).counts).is_null() {
            let mut i: libc::c_int = 0;
            i = 0 as libc::c_int;
            while i < (*exec).maxRollbacks {
                if !((*((*exec).rollbacks).offset(i as isize)).counts).is_null() {
                    xmlFree
                        .expect(
                            "non-null function pointer",
                        )(
                        (*((*exec).rollbacks).offset(i as isize)).counts
                            as *mut libc::c_void,
                    );
                }
                i += 1;
            }
        }
        xmlFree
            .expect("non-null function pointer")((*exec).rollbacks as *mut libc::c_void);
    }
    if !((*exec).counts).is_null() {
        xmlFree.expect("non-null function pointer")((*exec).counts as *mut libc::c_void);
    }
    if !((*exec).inputStack).is_null() {
        let mut i_0: libc::c_int = 0;
        i_0 = 0 as libc::c_int;
        while i_0 < (*exec).inputStackNr {
            if !((*((*exec).inputStack).offset(i_0 as isize)).value).is_null() {
                xmlFree
                    .expect(
                        "non-null function pointer",
                    )(
                    (*((*exec).inputStack).offset(i_0 as isize)).value
                        as *mut libc::c_void,
                );
            }
            i_0 += 1;
        }
        xmlFree
            .expect(
                "non-null function pointer",
            )((*exec).inputStack as *mut libc::c_void);
    }
    if !((*exec).errString).is_null() {
        xmlFree
            .expect("non-null function pointer")((*exec).errString as *mut libc::c_void);
    }
    xmlFree.expect("non-null function pointer")(exec as *mut libc::c_void);
}
unsafe extern "C" fn xmlFARegExecSaveInputString(
    mut exec: xmlRegExecCtxtPtr,
    mut value: *const xmlChar,
    mut data: *mut libc::c_void,
) {
    if (*exec).inputStackMax == 0 as libc::c_int {
        (*exec).inputStackMax = 4 as libc::c_int;
        let ref mut fresh91 = (*exec).inputStack;
        *fresh91 = xmlMalloc
            .expect(
                "non-null function pointer",
            )(
            ((*exec).inputStackMax as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<xmlRegInputToken>() as libc::c_ulong),
        ) as xmlRegInputTokenPtr;
        if ((*exec).inputStack).is_null() {
            xmlRegexpErrMemory(
                0 as xmlRegParserCtxtPtr,
                b"pushing input string\0" as *const u8 as *const libc::c_char,
            );
            (*exec).inputStackMax = 0 as libc::c_int;
            return;
        }
    } else if (*exec).inputStackNr + 1 as libc::c_int >= (*exec).inputStackMax {
        let mut tmp: xmlRegInputTokenPtr = 0 as *mut xmlRegInputToken;
        (*exec).inputStackMax *= 2 as libc::c_int;
        tmp = xmlRealloc
            .expect(
                "non-null function pointer",
            )(
            (*exec).inputStack as *mut libc::c_void,
            ((*exec).inputStackMax as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<xmlRegInputToken>() as libc::c_ulong),
        ) as xmlRegInputTokenPtr;
        if tmp.is_null() {
            xmlRegexpErrMemory(
                0 as xmlRegParserCtxtPtr,
                b"pushing input string\0" as *const u8 as *const libc::c_char,
            );
            (*exec).inputStackMax /= 2 as libc::c_int;
            return;
        }
        let ref mut fresh92 = (*exec).inputStack;
        *fresh92 = tmp;
    }
    let ref mut fresh93 = (*((*exec).inputStack).offset((*exec).inputStackNr as isize))
        .value;
    *fresh93 = xmlStrdup(value);
    let ref mut fresh94 = (*((*exec).inputStack).offset((*exec).inputStackNr as isize))
        .data;
    *fresh94 = data;
    let ref mut fresh95 = (*exec).inputStackNr;
    *fresh95 += 1;
    let ref mut fresh96 = (*((*exec).inputStack).offset((*exec).inputStackNr as isize))
        .value;
    *fresh96 = 0 as *mut xmlChar;
    let ref mut fresh97 = (*((*exec).inputStack).offset((*exec).inputStackNr as isize))
        .data;
    *fresh97 = 0 as *mut libc::c_void;
}
unsafe extern "C" fn xmlRegStrEqualWildcard(
    mut expStr: *const xmlChar,
    mut valStr: *const xmlChar,
) -> libc::c_int {
    if expStr == valStr {
        return 1 as libc::c_int;
    }
    if expStr.is_null() {
        return 0 as libc::c_int;
    }
    if valStr.is_null() {
        return 0 as libc::c_int;
    }
    loop {
        if *expStr as libc::c_int != *valStr as libc::c_int {
            if *valStr as libc::c_int == '*' as i32 {
                let mut tmp: *const xmlChar = 0 as *const xmlChar;
                tmp = valStr;
                valStr = expStr;
                expStr = tmp;
            }
            if *valStr as libc::c_int != 0 as libc::c_int
                && *expStr as libc::c_int != 0 as libc::c_int
                && {
                    let fresh98 = expStr;
                    expStr = expStr.offset(1);
                    *fresh98 as libc::c_int == '*' as i32
                }
            {
                while !(*valStr as libc::c_int == '|' as i32) {
                    valStr = valStr.offset(1);
                    if !(*valStr as libc::c_int != 0 as libc::c_int) {
                        break;
                    }
                }
            } else {
                return 0 as libc::c_int
            }
        } else {
            expStr = expStr.offset(1);
            valStr = valStr.offset(1);
        }
        if !(*valStr as libc::c_int != 0 as libc::c_int) {
            break;
        }
    }
    if *expStr as libc::c_int != 0 as libc::c_int {
        return 0 as libc::c_int
    } else {
        return 1 as libc::c_int
    };
}
unsafe extern "C" fn xmlRegCompactPushString(
    mut exec: xmlRegExecCtxtPtr,
    mut comp: xmlRegexpPtr,
    mut value: *const xmlChar,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut state: libc::c_int = (*exec).index;
    let mut i: libc::c_int = 0;
    let mut target: libc::c_int = 0;
    if comp.is_null() || ((*comp).compact).is_null() || ((*comp).stringMap).is_null() {
        return -(1 as libc::c_int);
    }
    if value.is_null() {
        if *((*comp).compact)
            .offset((state * ((*comp).nbstrings + 1 as libc::c_int)) as isize)
            == XML_REGEXP_FINAL_STATE as libc::c_int
        {
            return 1 as libc::c_int;
        }
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < (*comp).nbstrings {
        target = *((*comp).compact)
            .offset(
                (state * ((*comp).nbstrings + 1 as libc::c_int) + i + 1 as libc::c_int)
                    as isize,
            );
        if target > 0 as libc::c_int && target <= (*comp).nbstates {
            target -= 1;
            if xmlRegStrEqualWildcard(*((*comp).stringMap).offset(i as isize), value)
                != 0
            {
                (*exec).index = target;
                if ((*exec).callback).is_some() && !((*comp).transdata).is_null() {
                    ((*exec).callback)
                        .expect(
                            "non-null function pointer",
                        )(
                        (*exec).data as xmlRegExecCtxtPtr,
                        value,
                        *((*comp).transdata)
                            .offset((state * (*comp).nbstrings + i) as isize),
                        data,
                    );
                }
                if *((*comp).compact)
                    .offset((target * ((*comp).nbstrings + 1 as libc::c_int)) as isize)
                    == XML_REGEXP_SINK_STATE as libc::c_int
                {
                    break;
                }
                if *((*comp).compact)
                    .offset((target * ((*comp).nbstrings + 1 as libc::c_int)) as isize)
                    == XML_REGEXP_FINAL_STATE as libc::c_int
                {
                    return 1 as libc::c_int;
                }
                return 0 as libc::c_int;
            }
        }
        i += 1;
    }
    if !((*exec).errString).is_null() {
        xmlFree
            .expect("non-null function pointer")((*exec).errString as *mut libc::c_void);
    }
    let ref mut fresh99 = (*exec).errString;
    *fresh99 = xmlStrdup(value);
    (*exec).errStateNo = state;
    (*exec).status = -(1 as libc::c_int);
    return -(1 as libc::c_int);
}
unsafe extern "C" fn xmlRegExecPushStringInternal(
    mut exec: xmlRegExecCtxtPtr,
    mut value: *const xmlChar,
    mut data: *mut libc::c_void,
    mut compound: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut trans: xmlRegTransPtr = 0 as *mut xmlRegTrans;
    let mut atom: xmlRegAtomPtr = 0 as *mut xmlRegAtom;
    let mut ret: libc::c_int = 0;
    let mut final_0: libc::c_int = 0 as libc::c_int;
    let mut progress: libc::c_int = 1 as libc::c_int;
    if exec.is_null() {
        return -(1 as libc::c_int);
    }
    if ((*exec).comp).is_null() {
        return -(1 as libc::c_int);
    }
    if (*exec).status != 0 as libc::c_int {
        return (*exec).status;
    }
    if !((*(*exec).comp).compact).is_null() {
        return xmlRegCompactPushString(exec, (*exec).comp, value, data);
    }
    if value.is_null() {
        if (*(*exec).state).type_0 as libc::c_uint
            == XML_REGEXP_FINAL_STATE as libc::c_int as libc::c_uint
        {
            return 1 as libc::c_int;
        }
        final_0 = 1 as libc::c_int;
    }
    if !value.is_null() && (*exec).inputStackNr > 0 as libc::c_int {
        xmlFARegExecSaveInputString(exec, value, data);
        value = (*((*exec).inputStack).offset((*exec).index as isize)).value;
        data = (*((*exec).inputStack).offset((*exec).index as isize)).data;
    }
    while (*exec).status == 0 as libc::c_int
        && (!value.is_null()
            || final_0 == 1 as libc::c_int
                && (*(*exec).state).type_0 as libc::c_uint
                    != XML_REGEXP_FINAL_STATE as libc::c_int as libc::c_uint)
    {
        if !(value.is_null() && ((*exec).counts).is_null()) {
            (*exec).transcount = 0 as libc::c_int;
            loop {
                if !((*exec).transno < (*(*exec).state).nbTrans) {
                    current_block = 5511877782510663281;
                    break;
                }
                trans = &mut *((*(*exec).state).trans).offset((*exec).transno as isize)
                    as *mut xmlRegTrans;
                if !((*trans).to < 0 as libc::c_int) {
                    atom = (*trans).atom;
                    ret = 0 as libc::c_int;
                    if (*trans).count == 0x123457 as libc::c_int {
                        let mut i: libc::c_int = 0;
                        let mut count: libc::c_int = 0;
                        let mut t: xmlRegTransPtr = 0 as *mut xmlRegTrans;
                        let mut counter: xmlRegCounterPtr = 0 as *mut xmlRegCounter;
                        ret = 0 as libc::c_int;
                        if value.is_null() && final_0 != 0 {
                            ret = 1 as libc::c_int;
                        } else if !value.is_null() {
                            i = 0 as libc::c_int;
                            while i < (*(*exec).state).nbTrans {
                                t = &mut *((*(*exec).state).trans).offset(i as isize)
                                    as *mut xmlRegTrans;
                                if !((*t).counter < 0 as libc::c_int || t == trans) {
                                    counter = &mut *((*(*exec).comp).counters)
                                        .offset((*t).counter as isize) as *mut xmlRegCounter;
                                    count = *((*exec).counts).offset((*t).counter as isize);
                                    if count < (*counter).max && !((*t).atom).is_null()
                                        && xmlStrEqual(value, (*(*t).atom).valuep as *const xmlChar)
                                            != 0
                                    {
                                        ret = 0 as libc::c_int;
                                        break;
                                    } else if count >= (*counter).min && count < (*counter).max
                                            && !((*t).atom).is_null()
                                            && xmlStrEqual(value, (*(*t).atom).valuep as *const xmlChar)
                                                != 0
                                        {
                                        ret = 1 as libc::c_int;
                                        break;
                                    }
                                }
                                i += 1;
                            }
                        }
                    } else if (*trans).count == 0x123456 as libc::c_int {
                        let mut i_0: libc::c_int = 0;
                        let mut count_0: libc::c_int = 0;
                        let mut t_0: xmlRegTransPtr = 0 as *mut xmlRegTrans;
                        let mut counter_0: xmlRegCounterPtr = 0 as *mut xmlRegCounter;
                        ret = 1 as libc::c_int;
                        i_0 = 0 as libc::c_int;
                        while i_0 < (*(*exec).state).nbTrans {
                            t_0 = &mut *((*(*exec).state).trans).offset(i_0 as isize)
                                as *mut xmlRegTrans;
                            if !((*t_0).counter < 0 as libc::c_int || t_0 == trans) {
                                counter_0 = &mut *((*(*exec).comp).counters)
                                    .offset((*t_0).counter as isize) as *mut xmlRegCounter;
                                count_0 = *((*exec).counts).offset((*t_0).counter as isize);
                                if count_0 < (*counter_0).min || count_0 > (*counter_0).max
                                {
                                    ret = 0 as libc::c_int;
                                    break;
                                }
                            }
                            i_0 += 1;
                        }
                    } else if (*trans).count >= 0 as libc::c_int {
                        let mut count_1: libc::c_int = 0;
                        let mut counter_1: xmlRegCounterPtr = 0 as *mut xmlRegCounter;
                        count_1 = *((*exec).counts).offset((*trans).count as isize);
                        counter_1 = &mut *((*(*exec).comp).counters)
                            .offset((*trans).count as isize) as *mut xmlRegCounter;
                        ret = (count_1 >= (*counter_1).min
                            && count_1 <= (*counter_1).max) as libc::c_int;
                    } else if atom.is_null() {
                        fprintf(
                            stderr,
                            b"epsilon transition left at runtime\n\0" as *const u8
                                as *const libc::c_char,
                        );
                        (*exec).status = -(2 as libc::c_int);
                        current_block = 5511877782510663281;
                        break;
                    } else if !value.is_null() {
                        ret = xmlRegStrEqualWildcard(
                            (*atom).valuep as *const xmlChar,
                            value,
                        );
                        if (*atom).neg != 0 {
                            ret = (ret == 0) as libc::c_int;
                            if compound == 0 {
                                ret = 0 as libc::c_int;
                            }
                        }
                        if ret == 1 as libc::c_int
                            && (*trans).counter >= 0 as libc::c_int
                        {
                            let mut counter_2: xmlRegCounterPtr = 0
                                as *mut xmlRegCounter;
                            let mut count_2: libc::c_int = 0;
                            count_2 = *((*exec).counts)
                                .offset((*trans).counter as isize);
                            counter_2 = &mut *((*(*exec).comp).counters)
                                .offset((*trans).counter as isize) as *mut xmlRegCounter;
                            if count_2 >= (*counter_2).max {
                                ret = 0 as libc::c_int;
                            }
                        }
                        if ret == 1 as libc::c_int && (*atom).min > 0 as libc::c_int
                            && (*atom).max > 0 as libc::c_int
                        {
                            let mut to: xmlRegStatePtr = *((*(*exec).comp).states)
                                .offset((*trans).to as isize);
                            if (*(*exec).state).nbTrans
                                > (*exec).transno + 1 as libc::c_int
                            {
                                if (*exec).inputStackNr <= 0 as libc::c_int {
                                    xmlFARegExecSaveInputString(exec, value, data);
                                }
                                xmlFARegExecSave(exec);
                            }
                            (*exec).transcount = 1 as libc::c_int;
                            while !((*exec).transcount == (*atom).max) {
                                let ref mut fresh100 = (*exec).index;
                                *fresh100 += 1;
                                value = (*((*exec).inputStack)
                                    .offset((*exec).index as isize))
                                    .value;
                                data = (*((*exec).inputStack)
                                    .offset((*exec).index as isize))
                                    .data;
                                if value.is_null() {
                                    let ref mut fresh101 = (*exec).index;
                                    *fresh101 -= 1;
                                    break;
                                } else {
                                    if (*exec).transcount >= (*atom).min {
                                        let mut transno: libc::c_int = (*exec).transno;
                                        let mut state: xmlRegStatePtr = (*exec).state;
                                        (*exec).transno = -(1 as libc::c_int);
                                        let ref mut fresh102 = (*exec).state;
                                        *fresh102 = to;
                                        if (*exec).inputStackNr <= 0 as libc::c_int {
                                            xmlFARegExecSaveInputString(exec, value, data);
                                        }
                                        xmlFARegExecSave(exec);
                                        (*exec).transno = transno;
                                        let ref mut fresh103 = (*exec).state;
                                        *fresh103 = state;
                                    }
                                    ret = xmlStrEqual(value, (*atom).valuep as *const xmlChar);
                                    let ref mut fresh104 = (*exec).transcount;
                                    *fresh104 += 1;
                                    if !(ret == 1 as libc::c_int) {
                                        break;
                                    }
                                }
                            }
                            if (*exec).transcount < (*atom).min {
                                ret = 0 as libc::c_int;
                            }
                            if ret < 0 as libc::c_int {
                                ret = 0 as libc::c_int;
                            }
                            if ret == 0 as libc::c_int {
                                current_block = 10868385912253950213;
                                break;
                            }
                        }
                    }
                    if ret == 1 as libc::c_int {
                        if ((*exec).callback).is_some() && !atom.is_null()
                            && !data.is_null()
                        {
                            ((*exec).callback)
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*exec).data as xmlRegExecCtxtPtr,
                                (*atom).valuep as *const xmlChar,
                                (*atom).data,
                                data,
                            );
                        }
                        if (*(*exec).state).nbTrans > (*exec).transno + 1 as libc::c_int
                        {
                            if (*exec).inputStackNr <= 0 as libc::c_int {
                                xmlFARegExecSaveInputString(exec, value, data);
                            }
                            xmlFARegExecSave(exec);
                        }
                        if (*trans).counter >= 0 as libc::c_int {
                            let ref mut fresh105 = *((*exec).counts)
                                .offset((*trans).counter as isize);
                            *fresh105 += 1;
                        }
                        if (*trans).count >= 0 as libc::c_int
                            && (*trans).count < 0x123456 as libc::c_int
                        {
                            *((*exec).counts)
                                .offset((*trans).count as isize) = 0 as libc::c_int;
                        }
                        if !(*((*(*exec).comp).states).offset((*trans).to as isize))
                            .is_null()
                            && (**((*(*exec).comp).states).offset((*trans).to as isize))
                                .type_0 as libc::c_uint
                                == XML_REGEXP_SINK_STATE as libc::c_int as libc::c_uint
                        {
                            if !((*exec).errString).is_null() {
                                xmlFree
                                    .expect(
                                        "non-null function pointer",
                                    )((*exec).errString as *mut libc::c_void);
                            }
                            let ref mut fresh106 = (*exec).errString;
                            *fresh106 = xmlStrdup(value);
                            let ref mut fresh107 = (*exec).errState;
                            *fresh107 = (*exec).state;
                            memcpy(
                                (*exec).errCounts as *mut libc::c_void,
                                (*exec).counts as *const libc::c_void,
                                ((*(*exec).comp).nbCounters as libc::c_ulong)
                                    .wrapping_mul(
                                        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                                    ),
                            );
                        }
                        let ref mut fresh108 = (*exec).state;
                        *fresh108 = *((*(*exec).comp).states)
                            .offset((*trans).to as isize);
                        (*exec).transno = 0 as libc::c_int;
                        if !((*trans).atom).is_null() {
                            if !((*exec).inputStack).is_null() {
                                let ref mut fresh109 = (*exec).index;
                                *fresh109 += 1;
                                if (*exec).index < (*exec).inputStackNr {
                                    value = (*((*exec).inputStack)
                                        .offset((*exec).index as isize))
                                        .value;
                                    data = (*((*exec).inputStack)
                                        .offset((*exec).index as isize))
                                        .data;
                                } else {
                                    value = 0 as *const xmlChar;
                                    data = 0 as *mut libc::c_void;
                                }
                            } else {
                                value = 0 as *const xmlChar;
                                data = 0 as *mut libc::c_void;
                            }
                        }
                        current_block = 8483073061411596610;
                        break;
                    } else if ret < 0 as libc::c_int {
                        (*exec).status = -(4 as libc::c_int);
                        current_block = 5511877782510663281;
                        break;
                    }
                }
                let ref mut fresh110 = (*exec).transno;
                *fresh110 += 1;
            }
            match current_block {
                10868385912253950213 => {}
                _ => {
                    match current_block {
                        8483073061411596610 => {
                            progress = 1 as libc::c_int;
                            continue;
                        }
                        _ => {
                            if !((*exec).transno != 0 as libc::c_int
                                || (*(*exec).state).nbTrans == 0 as libc::c_int)
                            {
                                continue;
                            }
                        }
                    }
                }
            }
        }
        if progress != 0 && !((*exec).state).is_null()
            && (*(*exec).state).type_0 as libc::c_uint
                != XML_REGEXP_SINK_STATE as libc::c_int as libc::c_uint
        {
            progress = 0 as libc::c_int;
            if !((*exec).errString).is_null() {
                xmlFree
                    .expect(
                        "non-null function pointer",
                    )((*exec).errString as *mut libc::c_void);
            }
            let ref mut fresh111 = (*exec).errString;
            *fresh111 = xmlStrdup(value);
            let ref mut fresh112 = (*exec).errState;
            *fresh112 = (*exec).state;
            if (*(*exec).comp).nbCounters != 0 {
                memcpy(
                    (*exec).errCounts as *mut libc::c_void,
                    (*exec).counts as *const libc::c_void,
                    ((*(*exec).comp).nbCounters as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        ),
                );
            }
        }
        (*exec).determinist = 0 as libc::c_int;
        xmlFARegExecRollBack(exec);
        if !((*exec).inputStack).is_null() && (*exec).status == 0 as libc::c_int {
            value = (*((*exec).inputStack).offset((*exec).index as isize)).value;
            data = (*((*exec).inputStack).offset((*exec).index as isize)).data;
        }
    }
    if (*exec).status == 0 as libc::c_int {
        return ((*(*exec).state).type_0 as libc::c_uint
            == XML_REGEXP_FINAL_STATE as libc::c_int as libc::c_uint) as libc::c_int;
    }
    return (*exec).status;
}
#[no_mangle]
pub unsafe extern "C" fn xmlRegExecPushString(
    mut exec: xmlRegExecCtxtPtr,
    mut value: *const xmlChar,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    return xmlRegExecPushStringInternal(exec, value, data, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn xmlRegExecPushString2(
    mut exec: xmlRegExecCtxtPtr,
    mut value: *const xmlChar,
    mut value2: *const xmlChar,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut buf: [xmlChar; 150] = [0; 150];
    let mut lenn: libc::c_int = 0;
    let mut lenp: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut str: *mut xmlChar = 0 as *mut xmlChar;
    if exec.is_null() {
        return -(1 as libc::c_int);
    }
    if ((*exec).comp).is_null() {
        return -(1 as libc::c_int);
    }
    if (*exec).status != 0 as libc::c_int {
        return (*exec).status;
    }
    if value2.is_null() {
        return xmlRegExecPushString(exec, value, data);
    }
    lenn = strlen(value2 as *mut libc::c_char) as libc::c_int;
    lenp = strlen(value as *mut libc::c_char) as libc::c_int;
    if (150 as libc::c_int) < lenn + lenp + 2 as libc::c_int {
        str = xmlMallocAtomic
            .expect(
                "non-null function pointer",
            )((lenn + lenp + 2 as libc::c_int) as size_t) as *mut xmlChar;
        if str.is_null() {
            (*exec).status = -(1 as libc::c_int);
            return -(1 as libc::c_int);
        }
    } else {
        str = buf.as_mut_ptr();
    }
    memcpy(
        &mut *str.offset(0 as libc::c_int as isize) as *mut xmlChar as *mut libc::c_void,
        value as *const libc::c_void,
        lenp as libc::c_ulong,
    );
    *str.offset(lenp as isize) = '|' as i32 as xmlChar;
    memcpy(
        &mut *str.offset((lenp + 1 as libc::c_int) as isize) as *mut xmlChar
            as *mut libc::c_void,
        value2 as *const libc::c_void,
        lenn as libc::c_ulong,
    );
    *str.offset((lenn + lenp + 1 as libc::c_int) as isize) = 0 as libc::c_int as xmlChar;
    if !((*(*exec).comp).compact).is_null() {
        ret = xmlRegCompactPushString(exec, (*exec).comp, str, data);
    } else {
        ret = xmlRegExecPushStringInternal(exec, str, data, 1 as libc::c_int);
    }
    if str != buf.as_mut_ptr() {
        xmlFree.expect("non-null function pointer")(str as *mut libc::c_void);
    }
    return ret;
}
unsafe extern "C" fn xmlRegExecGetValues(
    mut exec: xmlRegExecCtxtPtr,
    mut err: libc::c_int,
    mut nbval: *mut libc::c_int,
    mut nbneg: *mut libc::c_int,
    mut values: *mut *mut xmlChar,
    mut terminal: *mut libc::c_int,
) -> libc::c_int {
    let mut maxval: libc::c_int = 0;
    let mut nb: libc::c_int = 0 as libc::c_int;
    if exec.is_null() || nbval.is_null() || nbneg.is_null() || values.is_null()
        || *nbval <= 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    maxval = *nbval;
    *nbval = 0 as libc::c_int;
    *nbneg = 0 as libc::c_int;
    if !((*exec).comp).is_null() && !((*(*exec).comp).compact).is_null() {
        let mut comp: xmlRegexpPtr = 0 as *mut xmlRegexp;
        let mut target: libc::c_int = 0;
        let mut i: libc::c_int = 0;
        let mut state: libc::c_int = 0;
        comp = (*exec).comp;
        if err != 0 {
            if (*exec).errStateNo == -(1 as libc::c_int) {
                return -(1 as libc::c_int);
            }
            state = (*exec).errStateNo;
        } else {
            state = (*exec).index;
        }
        if !terminal.is_null() {
            if *((*comp).compact)
                .offset((state * ((*comp).nbstrings + 1 as libc::c_int)) as isize)
                == XML_REGEXP_FINAL_STATE as libc::c_int
            {
                *terminal = 1 as libc::c_int;
            } else {
                *terminal = 0 as libc::c_int;
            }
        }
        i = 0 as libc::c_int;
        while i < (*comp).nbstrings && nb < maxval {
            target = *((*comp).compact)
                .offset(
                    (state * ((*comp).nbstrings + 1 as libc::c_int) + i
                        + 1 as libc::c_int) as isize,
                );
            if target > 0 as libc::c_int && target <= (*comp).nbstates
                && *((*comp).compact)
                    .offset(
                        ((target - 1 as libc::c_int)
                            * ((*comp).nbstrings + 1 as libc::c_int)) as isize,
                    ) != XML_REGEXP_SINK_STATE as libc::c_int
            {
                let fresh113 = nb;
                nb = nb + 1;
                let ref mut fresh114 = *values.offset(fresh113 as isize);
                *fresh114 = *((*comp).stringMap).offset(i as isize);
                *nbval += 1;
            }
            i += 1;
        }
        i = 0 as libc::c_int;
        while i < (*comp).nbstrings && nb < maxval {
            target = *((*comp).compact)
                .offset(
                    (state * ((*comp).nbstrings + 1 as libc::c_int) + i
                        + 1 as libc::c_int) as isize,
                );
            if target > 0 as libc::c_int && target <= (*comp).nbstates
                && *((*comp).compact)
                    .offset(
                        ((target - 1 as libc::c_int)
                            * ((*comp).nbstrings + 1 as libc::c_int)) as isize,
                    ) == XML_REGEXP_SINK_STATE as libc::c_int
            {
                let fresh115 = nb;
                nb = nb + 1;
                let ref mut fresh116 = *values.offset(fresh115 as isize);
                *fresh116 = *((*comp).stringMap).offset(i as isize);
                *nbneg += 1;
            }
            i += 1;
        }
    } else {
        let mut transno: libc::c_int = 0;
        let mut trans: xmlRegTransPtr = 0 as *mut xmlRegTrans;
        let mut atom: xmlRegAtomPtr = 0 as *mut xmlRegAtom;
        let mut state_0: xmlRegStatePtr = 0 as *mut xmlRegState;
        if !terminal.is_null() {
            if (*(*exec).state).type_0 as libc::c_uint
                == XML_REGEXP_FINAL_STATE as libc::c_int as libc::c_uint
            {
                *terminal = 1 as libc::c_int;
            } else {
                *terminal = 0 as libc::c_int;
            }
        }
        if err != 0 {
            if ((*exec).errState).is_null() {
                return -(1 as libc::c_int);
            }
            state_0 = (*exec).errState;
        } else {
            if ((*exec).state).is_null() {
                return -(1 as libc::c_int);
            }
            state_0 = (*exec).state;
        }
        transno = 0 as libc::c_int;
        while transno < (*state_0).nbTrans && nb < maxval {
            trans = &mut *((*state_0).trans).offset(transno as isize)
                as *mut xmlRegTrans;
            if !((*trans).to < 0 as libc::c_int) {
                atom = (*trans).atom;
                if !(atom.is_null() || ((*atom).valuep).is_null()) {
                    if (*trans).count == 0x123457 as libc::c_int {
                        (*__xmlGenericError())
                            .expect(
                                "non-null function pointer",
                            )(
                            *__xmlGenericErrorContext(),
                            b"Unimplemented block at %s:%d\n\0" as *const u8
                                as *const libc::c_char,
                            b"xmlregexp.c\0" as *const u8 as *const libc::c_char,
                            4365 as libc::c_int,
                        );
                    } else if (*trans).count == 0x123456 as libc::c_int {
                        (*__xmlGenericError())
                            .expect(
                                "non-null function pointer",
                            )(
                            *__xmlGenericErrorContext(),
                            b"Unimplemented block at %s:%d\n\0" as *const u8
                                as *const libc::c_char,
                            b"xmlregexp.c\0" as *const u8 as *const libc::c_char,
                            4368 as libc::c_int,
                        );
                    } else if (*trans).counter >= 0 as libc::c_int {
                        let mut counter: xmlRegCounterPtr = 0 as xmlRegCounterPtr;
                        let mut count: libc::c_int = 0;
                        if err != 0 {
                            count = *((*exec).errCounts)
                                .offset((*trans).counter as isize);
                        } else {
                            count = *((*exec).counts).offset((*trans).counter as isize);
                        }
                        if !((*exec).comp).is_null() {
                            counter = &mut *((*(*exec).comp).counters)
                                .offset((*trans).counter as isize) as *mut xmlRegCounter;
                        }
                        if counter.is_null() || count < (*counter).max {
                            if (*atom).neg != 0 {
                                let fresh117 = nb;
                                nb = nb + 1;
                                let ref mut fresh118 = *values.offset(fresh117 as isize);
                                *fresh118 = (*atom).valuep2 as *mut xmlChar;
                            } else {
                                let fresh119 = nb;
                                nb = nb + 1;
                                let ref mut fresh120 = *values.offset(fresh119 as isize);
                                *fresh120 = (*atom).valuep as *mut xmlChar;
                            }
                            *nbval += 1;
                        }
                    } else if !((*exec).comp).is_null()
                            && !(*((*(*exec).comp).states).offset((*trans).to as isize))
                                .is_null()
                            && (**((*(*exec).comp).states).offset((*trans).to as isize))
                                .type_0 as libc::c_uint
                                != XML_REGEXP_SINK_STATE as libc::c_int as libc::c_uint
                        {
                        if (*atom).neg != 0 {
                            let fresh121 = nb;
                            nb = nb + 1;
                            let ref mut fresh122 = *values.offset(fresh121 as isize);
                            *fresh122 = (*atom).valuep2 as *mut xmlChar;
                        } else {
                            let fresh123 = nb;
                            nb = nb + 1;
                            let ref mut fresh124 = *values.offset(fresh123 as isize);
                            *fresh124 = (*atom).valuep as *mut xmlChar;
                        }
                        *nbval += 1;
                    }
                }
            }
            transno += 1;
        }
        transno = 0 as libc::c_int;
        while transno < (*state_0).nbTrans && nb < maxval {
            trans = &mut *((*state_0).trans).offset(transno as isize)
                as *mut xmlRegTrans;
            if !((*trans).to < 0 as libc::c_int) {
                atom = (*trans).atom;
                if !(atom.is_null() || ((*atom).valuep).is_null()) {
                    if !((*trans).count == 0x123457 as libc::c_int) {
                        if !((*trans).count == 0x123456 as libc::c_int) {
                            if !((*trans).counter >= 0 as libc::c_int) {
                                if !(*((*(*exec).comp).states).offset((*trans).to as isize))
                                    .is_null()
                                    && (**((*(*exec).comp).states).offset((*trans).to as isize))
                                        .type_0 as libc::c_uint
                                        == XML_REGEXP_SINK_STATE as libc::c_int as libc::c_uint
                                {
                                    if (*atom).neg != 0 {
                                        let fresh125 = nb;
                                        nb = nb + 1;
                                        let ref mut fresh126 = *values.offset(fresh125 as isize);
                                        *fresh126 = (*atom).valuep2 as *mut xmlChar;
                                    } else {
                                        let fresh127 = nb;
                                        nb = nb + 1;
                                        let ref mut fresh128 = *values.offset(fresh127 as isize);
                                        *fresh128 = (*atom).valuep as *mut xmlChar;
                                    }
                                    *nbneg += 1;
                                }
                            }
                        }
                    }
                }
            }
            transno += 1;
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlRegExecNextValues(
    mut exec: xmlRegExecCtxtPtr,
    mut nbval: *mut libc::c_int,
    mut nbneg: *mut libc::c_int,
    mut values: *mut *mut xmlChar,
    mut terminal: *mut libc::c_int,
) -> libc::c_int {
    return xmlRegExecGetValues(exec, 0 as libc::c_int, nbval, nbneg, values, terminal);
}
#[no_mangle]
pub unsafe extern "C" fn xmlRegExecErrInfo(
    mut exec: xmlRegExecCtxtPtr,
    mut string: *mut *const xmlChar,
    mut nbval: *mut libc::c_int,
    mut nbneg: *mut libc::c_int,
    mut values: *mut *mut xmlChar,
    mut terminal: *mut libc::c_int,
) -> libc::c_int {
    if exec.is_null() {
        return -(1 as libc::c_int);
    }
    if !string.is_null() {
        if (*exec).status != 0 as libc::c_int {
            *string = (*exec).errString;
        } else {
            *string = 0 as *const xmlChar;
        }
    }
    return xmlRegExecGetValues(exec, 1 as libc::c_int, nbval, nbneg, values, terminal);
}
unsafe extern "C" fn xmlFAIsChar(mut ctxt: xmlRegParserCtxtPtr) -> libc::c_int {
    let mut cur: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    cur = xmlStringCurrentChar(0 as xmlParserCtxtPtr, (*ctxt).cur, &mut len);
    if cur == '.' as i32 || cur == '\\' as i32 || cur == '?' as i32 || cur == '*' as i32
        || cur == '+' as i32 || cur == '(' as i32 || cur == ')' as i32
        || cur == '|' as i32 || cur == 0x5b as libc::c_int || cur == 0x5d as libc::c_int
        || cur == 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    return cur;
}
unsafe extern "C" fn xmlFAParseCharProp(mut ctxt: xmlRegParserCtxtPtr) {
    let mut cur: libc::c_int = 0;
    let mut type_0: xmlRegAtomType = 0 as xmlRegAtomType;
    let mut blockName: *mut xmlChar = 0 as *mut xmlChar;
    cur = *(*ctxt).cur as libc::c_int;
    if cur == 'L' as i32 {
        let ref mut fresh129 = (*ctxt).cur;
        *fresh129 = (*fresh129).offset(1);
        cur = *(*ctxt).cur as libc::c_int;
        if cur == 'u' as i32 {
            let ref mut fresh130 = (*ctxt).cur;
            *fresh130 = (*fresh130).offset(1);
            type_0 = XML_REGEXP_LETTER_UPPERCASE;
        } else if cur == 'l' as i32 {
            let ref mut fresh131 = (*ctxt).cur;
            *fresh131 = (*fresh131).offset(1);
            type_0 = XML_REGEXP_LETTER_LOWERCASE;
        } else if cur == 't' as i32 {
            let ref mut fresh132 = (*ctxt).cur;
            *fresh132 = (*fresh132).offset(1);
            type_0 = XML_REGEXP_LETTER_TITLECASE;
        } else if cur == 'm' as i32 {
            let ref mut fresh133 = (*ctxt).cur;
            *fresh133 = (*fresh133).offset(1);
            type_0 = XML_REGEXP_LETTER_MODIFIER;
        } else if cur == 'o' as i32 {
            let ref mut fresh134 = (*ctxt).cur;
            *fresh134 = (*fresh134).offset(1);
            type_0 = XML_REGEXP_LETTER_OTHERS;
        } else {
            type_0 = XML_REGEXP_LETTER;
        }
    } else if cur == 'M' as i32 {
        let ref mut fresh135 = (*ctxt).cur;
        *fresh135 = (*fresh135).offset(1);
        cur = *(*ctxt).cur as libc::c_int;
        if cur == 'n' as i32 {
            let ref mut fresh136 = (*ctxt).cur;
            *fresh136 = (*fresh136).offset(1);
            type_0 = XML_REGEXP_MARK_NONSPACING;
        } else if cur == 'c' as i32 {
            let ref mut fresh137 = (*ctxt).cur;
            *fresh137 = (*fresh137).offset(1);
            type_0 = XML_REGEXP_MARK_SPACECOMBINING;
        } else if cur == 'e' as i32 {
            let ref mut fresh138 = (*ctxt).cur;
            *fresh138 = (*fresh138).offset(1);
            type_0 = XML_REGEXP_MARK_ENCLOSING;
        } else {
            type_0 = XML_REGEXP_MARK;
        }
    } else if cur == 'N' as i32 {
        let ref mut fresh139 = (*ctxt).cur;
        *fresh139 = (*fresh139).offset(1);
        cur = *(*ctxt).cur as libc::c_int;
        if cur == 'd' as i32 {
            let ref mut fresh140 = (*ctxt).cur;
            *fresh140 = (*fresh140).offset(1);
            type_0 = XML_REGEXP_NUMBER_DECIMAL;
        } else if cur == 'l' as i32 {
            let ref mut fresh141 = (*ctxt).cur;
            *fresh141 = (*fresh141).offset(1);
            type_0 = XML_REGEXP_NUMBER_LETTER;
        } else if cur == 'o' as i32 {
            let ref mut fresh142 = (*ctxt).cur;
            *fresh142 = (*fresh142).offset(1);
            type_0 = XML_REGEXP_NUMBER_OTHERS;
        } else {
            type_0 = XML_REGEXP_NUMBER;
        }
    } else if cur == 'P' as i32 {
        let ref mut fresh143 = (*ctxt).cur;
        *fresh143 = (*fresh143).offset(1);
        cur = *(*ctxt).cur as libc::c_int;
        if cur == 'c' as i32 {
            let ref mut fresh144 = (*ctxt).cur;
            *fresh144 = (*fresh144).offset(1);
            type_0 = XML_REGEXP_PUNCT_CONNECTOR;
        } else if cur == 'd' as i32 {
            let ref mut fresh145 = (*ctxt).cur;
            *fresh145 = (*fresh145).offset(1);
            type_0 = XML_REGEXP_PUNCT_DASH;
        } else if cur == 's' as i32 {
            let ref mut fresh146 = (*ctxt).cur;
            *fresh146 = (*fresh146).offset(1);
            type_0 = XML_REGEXP_PUNCT_OPEN;
        } else if cur == 'e' as i32 {
            let ref mut fresh147 = (*ctxt).cur;
            *fresh147 = (*fresh147).offset(1);
            type_0 = XML_REGEXP_PUNCT_CLOSE;
        } else if cur == 'i' as i32 {
            let ref mut fresh148 = (*ctxt).cur;
            *fresh148 = (*fresh148).offset(1);
            type_0 = XML_REGEXP_PUNCT_INITQUOTE;
        } else if cur == 'f' as i32 {
            let ref mut fresh149 = (*ctxt).cur;
            *fresh149 = (*fresh149).offset(1);
            type_0 = XML_REGEXP_PUNCT_FINQUOTE;
        } else if cur == 'o' as i32 {
            let ref mut fresh150 = (*ctxt).cur;
            *fresh150 = (*fresh150).offset(1);
            type_0 = XML_REGEXP_PUNCT_OTHERS;
        } else {
            type_0 = XML_REGEXP_PUNCT;
        }
    } else if cur == 'Z' as i32 {
        let ref mut fresh151 = (*ctxt).cur;
        *fresh151 = (*fresh151).offset(1);
        cur = *(*ctxt).cur as libc::c_int;
        if cur == 's' as i32 {
            let ref mut fresh152 = (*ctxt).cur;
            *fresh152 = (*fresh152).offset(1);
            type_0 = XML_REGEXP_SEPAR_SPACE;
        } else if cur == 'l' as i32 {
            let ref mut fresh153 = (*ctxt).cur;
            *fresh153 = (*fresh153).offset(1);
            type_0 = XML_REGEXP_SEPAR_LINE;
        } else if cur == 'p' as i32 {
            let ref mut fresh154 = (*ctxt).cur;
            *fresh154 = (*fresh154).offset(1);
            type_0 = XML_REGEXP_SEPAR_PARA;
        } else {
            type_0 = XML_REGEXP_SEPAR;
        }
    } else if cur == 'S' as i32 {
        let ref mut fresh155 = (*ctxt).cur;
        *fresh155 = (*fresh155).offset(1);
        cur = *(*ctxt).cur as libc::c_int;
        if cur == 'm' as i32 {
            let ref mut fresh156 = (*ctxt).cur;
            *fresh156 = (*fresh156).offset(1);
            type_0 = XML_REGEXP_SYMBOL_MATH;
        } else if cur == 'c' as i32 {
            let ref mut fresh157 = (*ctxt).cur;
            *fresh157 = (*fresh157).offset(1);
            type_0 = XML_REGEXP_SYMBOL_CURRENCY;
        } else if cur == 'k' as i32 {
            let ref mut fresh158 = (*ctxt).cur;
            *fresh158 = (*fresh158).offset(1);
            type_0 = XML_REGEXP_SYMBOL_MODIFIER;
        } else if cur == 'o' as i32 {
            let ref mut fresh159 = (*ctxt).cur;
            *fresh159 = (*fresh159).offset(1);
            type_0 = XML_REGEXP_SYMBOL_OTHERS;
        } else {
            type_0 = XML_REGEXP_SYMBOL;
        }
    } else if cur == 'C' as i32 {
        let ref mut fresh160 = (*ctxt).cur;
        *fresh160 = (*fresh160).offset(1);
        cur = *(*ctxt).cur as libc::c_int;
        if cur == 'c' as i32 {
            let ref mut fresh161 = (*ctxt).cur;
            *fresh161 = (*fresh161).offset(1);
            type_0 = XML_REGEXP_OTHER_CONTROL;
        } else if cur == 'f' as i32 {
            let ref mut fresh162 = (*ctxt).cur;
            *fresh162 = (*fresh162).offset(1);
            type_0 = XML_REGEXP_OTHER_FORMAT;
        } else if cur == 'o' as i32 {
            let ref mut fresh163 = (*ctxt).cur;
            *fresh163 = (*fresh163).offset(1);
            type_0 = XML_REGEXP_OTHER_PRIVATE;
        } else if cur == 'n' as i32 {
            let ref mut fresh164 = (*ctxt).cur;
            *fresh164 = (*fresh164).offset(1);
            type_0 = XML_REGEXP_OTHER_NA;
        } else {
            type_0 = XML_REGEXP_OTHER;
        }
    } else if cur == 'I' as i32 {
        let mut start: *const xmlChar = 0 as *const xmlChar;
        let ref mut fresh165 = (*ctxt).cur;
        *fresh165 = (*fresh165).offset(1);
        cur = *(*ctxt).cur as libc::c_int;
        if cur != 's' as i32 {
            (*ctxt).error = XML_REGEXP_COMPILE_ERROR as libc::c_int;
            xmlRegexpErrCompile(
                ctxt,
                b"IsXXXX expected\0" as *const u8 as *const libc::c_char,
            );
            return;
        }
        let ref mut fresh166 = (*ctxt).cur;
        *fresh166 = (*fresh166).offset(1);
        start = (*ctxt).cur;
        cur = *(*ctxt).cur as libc::c_int;
        if cur >= 'a' as i32 && cur <= 'z' as i32
            || cur >= 'A' as i32 && cur <= 'Z' as i32
            || cur >= '0' as i32 && cur <= '9' as i32 || cur == 0x2d as libc::c_int
        {
            let ref mut fresh167 = (*ctxt).cur;
            *fresh167 = (*fresh167).offset(1);
            cur = *(*ctxt).cur as libc::c_int;
            while cur >= 'a' as i32 && cur <= 'z' as i32
                || cur >= 'A' as i32 && cur <= 'Z' as i32
                || cur >= '0' as i32 && cur <= '9' as i32 || cur == 0x2d as libc::c_int
            {
                let ref mut fresh168 = (*ctxt).cur;
                *fresh168 = (*fresh168).offset(1);
                cur = *(*ctxt).cur as libc::c_int;
            }
        }
        type_0 = XML_REGEXP_BLOCK_NAME;
        blockName = xmlStrndup(
            start,
            ((*ctxt).cur).offset_from(start) as libc::c_long as libc::c_int,
        );
    } else {
        (*ctxt).error = XML_REGEXP_COMPILE_ERROR as libc::c_int;
        xmlRegexpErrCompile(
            ctxt,
            b"Unknown char property\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if ((*ctxt).atom).is_null() {
        let ref mut fresh169 = (*ctxt).atom;
        *fresh169 = xmlRegNewAtom(ctxt, type_0);
        if !((*ctxt).atom).is_null() {
            let ref mut fresh170 = (*(*ctxt).atom).valuep;
            *fresh170 = blockName as *mut libc::c_void;
        }
    } else if (*(*ctxt).atom).type_0 as libc::c_uint
            == XML_REGEXP_RANGES as libc::c_int as libc::c_uint
        {
        xmlRegAtomAddRange(
            ctxt,
            (*ctxt).atom,
            (*ctxt).neg,
            type_0,
            0 as libc::c_int,
            0 as libc::c_int,
            blockName,
        );
    }
}
unsafe extern "C" fn parse_escaped_codeunit(
    mut ctxt: xmlRegParserCtxtPtr,
) -> libc::c_int {
    let mut val: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut cur: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        let ref mut fresh171 = (*ctxt).cur;
        *fresh171 = (*fresh171).offset(1);
        val *= 16 as libc::c_int;
        cur = *(*ctxt).cur as libc::c_int;
        if cur >= '0' as i32 && cur <= '9' as i32 {
            val += cur - '0' as i32;
        } else if cur >= 'A' as i32 && cur <= 'F' as i32 {
            val += cur - 'A' as i32 + 10 as libc::c_int;
        } else if cur >= 'a' as i32 && cur <= 'f' as i32 {
            val += cur - 'a' as i32 + 10 as libc::c_int;
        } else {
            (*ctxt).error = XML_REGEXP_COMPILE_ERROR as libc::c_int;
            xmlRegexpErrCompile(
                ctxt,
                b"Expecting hex digit\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        i += 1;
    }
    return val;
}
unsafe extern "C" fn parse_escaped_codepoint(
    mut ctxt: xmlRegParserCtxtPtr,
) -> libc::c_int {
    let mut val: libc::c_int = parse_escaped_codeunit(ctxt);
    if 0xd800 as libc::c_int <= val && val <= 0xdbff as libc::c_int {
        let ref mut fresh172 = (*ctxt).cur;
        *fresh172 = (*fresh172).offset(1);
        if *(*ctxt).cur as libc::c_int == '\\' as i32 {
            let ref mut fresh173 = (*ctxt).cur;
            *fresh173 = (*fresh173).offset(1);
            if *(*ctxt).cur as libc::c_int == 'u' as i32 {
                let mut low: libc::c_int = parse_escaped_codeunit(ctxt);
                if 0xdc00 as libc::c_int <= low && low <= 0xdfff as libc::c_int {
                    return (val - 0xd800 as libc::c_int) * 0x400 as libc::c_int
                        + (low - 0xdc00 as libc::c_int) + 0x10000 as libc::c_int;
                }
            }
        }
        (*ctxt).error = XML_REGEXP_COMPILE_ERROR as libc::c_int;
        xmlRegexpErrCompile(
            ctxt,
            b"Invalid low surrogate pair code unit\0" as *const u8 as *const libc::c_char,
        );
        val = -(1 as libc::c_int);
    }
    return val;
}
unsafe extern "C" fn xmlFAParseCharClassEsc(mut ctxt: xmlRegParserCtxtPtr) {
    let mut cur: libc::c_int = 0;
    if *(*ctxt).cur as libc::c_int == '.' as i32 {
        if ((*ctxt).atom).is_null() {
            let ref mut fresh174 = (*ctxt).atom;
            *fresh174 = xmlRegNewAtom(ctxt, XML_REGEXP_ANYCHAR);
        } else if (*(*ctxt).atom).type_0 as libc::c_uint
                == XML_REGEXP_RANGES as libc::c_int as libc::c_uint
            {
            xmlRegAtomAddRange(
                ctxt,
                (*ctxt).atom,
                (*ctxt).neg,
                XML_REGEXP_ANYCHAR,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as *mut xmlChar,
            );
        }
        let ref mut fresh175 = (*ctxt).cur;
        *fresh175 = (*fresh175).offset(1);
        return;
    }
    if *(*ctxt).cur as libc::c_int != '\\' as i32 {
        (*ctxt).error = XML_REGEXP_COMPILE_ERROR as libc::c_int;
        xmlRegexpErrCompile(
            ctxt,
            b"Escaped sequence: expecting \\\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    let ref mut fresh176 = (*ctxt).cur;
    *fresh176 = (*fresh176).offset(1);
    cur = *(*ctxt).cur as libc::c_int;
    if cur == 'p' as i32 {
        let ref mut fresh177 = (*ctxt).cur;
        *fresh177 = (*fresh177).offset(1);
        if *(*ctxt).cur as libc::c_int != '{' as i32 {
            (*ctxt).error = XML_REGEXP_COMPILE_ERROR as libc::c_int;
            xmlRegexpErrCompile(
                ctxt,
                b"Expecting '{'\0" as *const u8 as *const libc::c_char,
            );
            return;
        }
        let ref mut fresh178 = (*ctxt).cur;
        *fresh178 = (*fresh178).offset(1);
        xmlFAParseCharProp(ctxt);
        if *(*ctxt).cur as libc::c_int != '}' as i32 {
            (*ctxt).error = XML_REGEXP_COMPILE_ERROR as libc::c_int;
            xmlRegexpErrCompile(
                ctxt,
                b"Expecting '}'\0" as *const u8 as *const libc::c_char,
            );
            return;
        }
        let ref mut fresh179 = (*ctxt).cur;
        *fresh179 = (*fresh179).offset(1);
    } else if cur == 'P' as i32 {
        let ref mut fresh180 = (*ctxt).cur;
        *fresh180 = (*fresh180).offset(1);
        if *(*ctxt).cur as libc::c_int != '{' as i32 {
            (*ctxt).error = XML_REGEXP_COMPILE_ERROR as libc::c_int;
            xmlRegexpErrCompile(
                ctxt,
                b"Expecting '{'\0" as *const u8 as *const libc::c_char,
            );
            return;
        }
        let ref mut fresh181 = (*ctxt).cur;
        *fresh181 = (*fresh181).offset(1);
        xmlFAParseCharProp(ctxt);
        if !((*ctxt).atom).is_null() {
            (*(*ctxt).atom).neg = 1 as libc::c_int;
        }
        if *(*ctxt).cur as libc::c_int != '}' as i32 {
            (*ctxt).error = XML_REGEXP_COMPILE_ERROR as libc::c_int;
            xmlRegexpErrCompile(
                ctxt,
                b"Expecting '}'\0" as *const u8 as *const libc::c_char,
            );
            return;
        }
        let ref mut fresh182 = (*ctxt).cur;
        *fresh182 = (*fresh182).offset(1);
    } else if cur == 'n' as i32 || cur == 'r' as i32 || cur == 't' as i32
            || cur == '\\' as i32 || cur == '|' as i32 || cur == '.' as i32
            || cur == '?' as i32 || cur == '*' as i32 || cur == '+' as i32
            || cur == '(' as i32 || cur == ')' as i32 || cur == '{' as i32
            || cur == '}' as i32 || cur == 0x2d as libc::c_int
            || cur == 0x5b as libc::c_int || cur == 0x5d as libc::c_int
            || cur == 0x5e as libc::c_int || cur == '!' as i32 || cur == '"' as i32
            || cur == '#' as i32 || cur == '$' as i32 || cur == '%' as i32
            || cur == ',' as i32 || cur == '/' as i32 || cur == ':' as i32
            || cur == ';' as i32 || cur == '=' as i32 || cur == '>' as i32
            || cur == '@' as i32 || cur == '`' as i32 || cur == '~' as i32
            || cur == 'u' as i32
        {
        if ((*ctxt).atom).is_null() {
            let ref mut fresh183 = (*ctxt).atom;
            *fresh183 = xmlRegNewAtom(ctxt, XML_REGEXP_CHARVAL);
            if !((*ctxt).atom).is_null() {
                match cur {
                    110 => {
                        (*(*ctxt).atom).codepoint = '\n' as i32;
                    }
                    114 => {
                        (*(*ctxt).atom).codepoint = '\r' as i32;
                    }
                    116 => {
                        (*(*ctxt).atom).codepoint = '\t' as i32;
                    }
                    117 => {
                        cur = parse_escaped_codepoint(ctxt);
                        if cur < 0 as libc::c_int {
                            return;
                        }
                        (*(*ctxt).atom).codepoint = cur;
                    }
                    _ => {
                        (*(*ctxt).atom).codepoint = cur;
                    }
                }
            }
        } else if (*(*ctxt).atom).type_0 as libc::c_uint
                == XML_REGEXP_RANGES as libc::c_int as libc::c_uint
            {
            match cur {
                110 => {
                    cur = '\n' as i32;
                }
                114 => {
                    cur = '\r' as i32;
                }
                116 => {
                    cur = '\t' as i32;
                }
                _ => {}
            }
            xmlRegAtomAddRange(
                ctxt,
                (*ctxt).atom,
                (*ctxt).neg,
                XML_REGEXP_CHARVAL,
                cur,
                cur,
                0 as *mut xmlChar,
            );
        }
        let ref mut fresh184 = (*ctxt).cur;
        *fresh184 = (*fresh184).offset(1);
    } else if cur == 's' as i32 || cur == 'S' as i32 || cur == 'i' as i32
            || cur == 'I' as i32 || cur == 'c' as i32 || cur == 'C' as i32
            || cur == 'd' as i32 || cur == 'D' as i32 || cur == 'w' as i32
            || cur == 'W' as i32
        {
        let mut type_0: xmlRegAtomType = XML_REGEXP_ANYSPACE;
        match cur {
            115 => {
                type_0 = XML_REGEXP_ANYSPACE;
            }
            83 => {
                type_0 = XML_REGEXP_NOTSPACE;
            }
            105 => {
                type_0 = XML_REGEXP_INITNAME;
            }
            73 => {
                type_0 = XML_REGEXP_NOTINITNAME;
            }
            99 => {
                type_0 = XML_REGEXP_NAMECHAR;
            }
            67 => {
                type_0 = XML_REGEXP_NOTNAMECHAR;
            }
            100 => {
                type_0 = XML_REGEXP_DECIMAL;
            }
            68 => {
                type_0 = XML_REGEXP_NOTDECIMAL;
            }
            119 => {
                type_0 = XML_REGEXP_REALCHAR;
            }
            87 => {
                type_0 = XML_REGEXP_NOTREALCHAR;
            }
            _ => {}
        }
        let ref mut fresh185 = (*ctxt).cur;
        *fresh185 = (*fresh185).offset(1);
        if ((*ctxt).atom).is_null() {
            let ref mut fresh186 = (*ctxt).atom;
            *fresh186 = xmlRegNewAtom(ctxt, type_0);
        } else if (*(*ctxt).atom).type_0 as libc::c_uint
                == XML_REGEXP_RANGES as libc::c_int as libc::c_uint
            {
            xmlRegAtomAddRange(
                ctxt,
                (*ctxt).atom,
                (*ctxt).neg,
                type_0,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as *mut xmlChar,
            );
        }
    } else {
        (*ctxt).error = XML_REGEXP_COMPILE_ERROR as libc::c_int;
        xmlRegexpErrCompile(
            ctxt,
            b"Wrong escape sequence, misuse of character '\\'\0" as *const u8
                as *const libc::c_char,
        );
    };
}
unsafe extern "C" fn xmlFAParseCharRange(mut ctxt: xmlRegParserCtxtPtr) {
    let mut cur: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut start: libc::c_int = -(1 as libc::c_int);
    let mut end: libc::c_int = -(1 as libc::c_int);
    if *(*ctxt).cur as libc::c_int == '\u{0}' as i32 {
        (*ctxt).error = XML_REGEXP_COMPILE_ERROR as libc::c_int;
        xmlRegexpErrCompile(
            ctxt,
            b"Expecting ']'\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    cur = *(*ctxt).cur as libc::c_int;
    if cur == '\\' as i32 {
        let ref mut fresh187 = (*ctxt).cur;
        *fresh187 = (*fresh187).offset(1);
        cur = *(*ctxt).cur as libc::c_int;
        match cur {
            110 => {
                start = 0xa as libc::c_int;
            }
            114 => {
                start = 0xd as libc::c_int;
            }
            116 => {
                start = 0x9 as libc::c_int;
            }
            92 | 124 | 46 | 45 | 94 | 63 | 42 | 43 | 123 | 125 | 40 | 41 | 91 | 93 => {
                start = cur;
            }
            _ => {
                (*ctxt).error = XML_REGEXP_COMPILE_ERROR as libc::c_int;
                xmlRegexpErrCompile(
                    ctxt,
                    b"Invalid escape value\0" as *const u8 as *const libc::c_char,
                );
                return;
            }
        }
        end = start;
        len = 1 as libc::c_int;
    } else if cur != 0x5b as libc::c_int && cur != 0x5d as libc::c_int {
        start = xmlStringCurrentChar(0 as xmlParserCtxtPtr, (*ctxt).cur, &mut len);
        end = start;
    } else {
        (*ctxt).error = XML_REGEXP_COMPILE_ERROR as libc::c_int;
        xmlRegexpErrCompile(
            ctxt,
            b"Expecting a char range\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if start == '-' as i32
        && *((*ctxt).cur).offset(1 as libc::c_int as isize) as libc::c_int != ']' as i32
        && *((*ctxt).cur).offset(-(1 as libc::c_int) as isize) as libc::c_int
            != '[' as i32
        && *((*ctxt).cur).offset(-(1 as libc::c_int) as isize) as libc::c_int
            != '^' as i32
    {
        let ref mut fresh188 = (*ctxt).cur;
        *fresh188 = (*fresh188).offset(len as isize);
        return;
    }
    let ref mut fresh189 = (*ctxt).cur;
    *fresh189 = (*fresh189).offset(len as isize);
    cur = *(*ctxt).cur as libc::c_int;
    if cur != '-' as i32
        || *((*ctxt).cur).offset(1 as libc::c_int as isize) as libc::c_int == '[' as i32
        || *((*ctxt).cur).offset(1 as libc::c_int as isize) as libc::c_int == ']' as i32
    {
        xmlRegAtomAddRange(
            ctxt,
            (*ctxt).atom,
            (*ctxt).neg,
            XML_REGEXP_CHARVAL,
            start,
            end,
            0 as *mut xmlChar,
        );
        return;
    }
    let ref mut fresh190 = (*ctxt).cur;
    *fresh190 = (*fresh190).offset(1);
    cur = *(*ctxt).cur as libc::c_int;
    if cur == '\\' as i32 {
        let ref mut fresh191 = (*ctxt).cur;
        *fresh191 = (*fresh191).offset(1);
        cur = *(*ctxt).cur as libc::c_int;
        match cur {
            110 => {
                end = 0xa as libc::c_int;
            }
            114 => {
                end = 0xd as libc::c_int;
            }
            116 => {
                end = 0x9 as libc::c_int;
            }
            92 | 124 | 46 | 45 | 94 | 63 | 42 | 43 | 123 | 125 | 40 | 41 | 91 | 93 => {
                end = cur;
            }
            _ => {
                (*ctxt).error = XML_REGEXP_COMPILE_ERROR as libc::c_int;
                xmlRegexpErrCompile(
                    ctxt,
                    b"Invalid escape value\0" as *const u8 as *const libc::c_char,
                );
                return;
            }
        }
        len = 1 as libc::c_int;
    } else if cur != '\u{0}' as i32 && cur != 0x5b as libc::c_int
            && cur != 0x5d as libc::c_int
        {
        end = xmlStringCurrentChar(0 as xmlParserCtxtPtr, (*ctxt).cur, &mut len);
    } else {
        (*ctxt).error = XML_REGEXP_COMPILE_ERROR as libc::c_int;
        xmlRegexpErrCompile(
            ctxt,
            b"Expecting the end of a char range\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if end < start {
        (*ctxt).error = XML_REGEXP_COMPILE_ERROR as libc::c_int;
        xmlRegexpErrCompile(
            ctxt,
            b"End of range is before start of range\0" as *const u8
                as *const libc::c_char,
        );
    } else {
        let ref mut fresh192 = (*ctxt).cur;
        *fresh192 = (*fresh192).offset(len as isize);
        xmlRegAtomAddRange(
            ctxt,
            (*ctxt).atom,
            (*ctxt).neg,
            XML_REGEXP_CHARVAL,
            start,
            end,
            0 as *mut xmlChar,
        );
    };
}
unsafe extern "C" fn xmlFAParsePosCharGroup(mut ctxt: xmlRegParserCtxtPtr) {
    loop {
        if *(*ctxt).cur as libc::c_int == '\\' as i32 {
            xmlFAParseCharClassEsc(ctxt);
        } else {
            xmlFAParseCharRange(ctxt);
        }
        if !(*(*ctxt).cur as libc::c_int != ']' as i32
            && *(*ctxt).cur as libc::c_int != '-' as i32
            && *(*ctxt).cur as libc::c_int != 0 as libc::c_int
            && (*ctxt).error == 0 as libc::c_int)
        {
            break;
        }
    };
}
unsafe extern "C" fn xmlFAParseCharGroup(mut ctxt: xmlRegParserCtxtPtr) {
    let mut neg: libc::c_int = (*ctxt).neg;
    if *(*ctxt).cur as libc::c_int == '^' as i32 {
        let ref mut fresh193 = (*ctxt).cur;
        *fresh193 = (*fresh193).offset(1);
        (*ctxt).neg = ((*ctxt).neg == 0) as libc::c_int;
        xmlFAParsePosCharGroup(ctxt);
        (*ctxt).neg = neg;
    }
    while *(*ctxt).cur as libc::c_int != ']' as i32 && (*ctxt).error == 0 as libc::c_int
    {
        if *(*ctxt).cur as libc::c_int == '-' as i32
            && *((*ctxt).cur).offset(1 as libc::c_int as isize) as libc::c_int
                == '[' as i32
        {
            let ref mut fresh194 = (*ctxt).cur;
            *fresh194 = (*fresh194).offset(1);
            let ref mut fresh195 = (*ctxt).cur;
            *fresh195 = (*fresh195).offset(1);
            (*ctxt).neg = 2 as libc::c_int;
            xmlFAParseCharGroup(ctxt);
            (*ctxt).neg = neg;
            if *(*ctxt).cur as libc::c_int == ']' as i32 {
                let ref mut fresh196 = (*ctxt).cur;
                *fresh196 = (*fresh196).offset(1);
            } else {
                (*ctxt).error = XML_REGEXP_COMPILE_ERROR as libc::c_int;
                xmlRegexpErrCompile(
                    ctxt,
                    b"charClassExpr: ']' expected\0" as *const u8 as *const libc::c_char,
                );
            }
            break;
        } else {
            xmlFAParsePosCharGroup(ctxt);
        }
    }
}
unsafe extern "C" fn xmlFAParseCharClass(mut ctxt: xmlRegParserCtxtPtr) {
    if *(*ctxt).cur as libc::c_int == '[' as i32 {
        let ref mut fresh197 = (*ctxt).cur;
        *fresh197 = (*fresh197).offset(1);
        let ref mut fresh198 = (*ctxt).atom;
        *fresh198 = xmlRegNewAtom(ctxt, XML_REGEXP_RANGES);
        if ((*ctxt).atom).is_null() {
            return;
        }
        xmlFAParseCharGroup(ctxt);
        if *(*ctxt).cur as libc::c_int == ']' as i32 {
            let ref mut fresh199 = (*ctxt).cur;
            *fresh199 = (*fresh199).offset(1);
        } else {
            (*ctxt).error = XML_REGEXP_COMPILE_ERROR as libc::c_int;
            xmlRegexpErrCompile(
                ctxt,
                b"xmlFAParseCharClass: ']' expected\0" as *const u8
                    as *const libc::c_char,
            );
        }
    } else {
        xmlFAParseCharClassEsc(ctxt);
    };
}
unsafe extern "C" fn xmlFAParseQuantExact(mut ctxt: xmlRegParserCtxtPtr) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut ok: libc::c_int = 0 as libc::c_int;
    let mut overflow: libc::c_int = 0 as libc::c_int;
    while *(*ctxt).cur as libc::c_int >= '0' as i32
        && *(*ctxt).cur as libc::c_int <= '9' as i32
    {
        if ret > 2147483647 as libc::c_int / 10 as libc::c_int {
            overflow = 1 as libc::c_int;
        } else {
            let mut digit: libc::c_int = *(*ctxt).cur as libc::c_int - '0' as i32;
            ret *= 10 as libc::c_int;
            if ret > 2147483647 as libc::c_int - digit {
                overflow = 1 as libc::c_int;
            } else {
                ret += digit;
            }
        }
        ok = 1 as libc::c_int;
        let ref mut fresh200 = (*ctxt).cur;
        *fresh200 = (*fresh200).offset(1);
    }
    if ok != 1 as libc::c_int || overflow == 1 as libc::c_int {
        return -(1 as libc::c_int);
    }
    return ret;
}
unsafe extern "C" fn xmlFAParseQuantifier(mut ctxt: xmlRegParserCtxtPtr) -> libc::c_int {
    let mut cur: libc::c_int = 0;
    cur = *(*ctxt).cur as libc::c_int;
    if cur == '?' as i32 || cur == '*' as i32 || cur == '+' as i32 {
        if !((*ctxt).atom).is_null() {
            if cur == '?' as i32 {
                (*(*ctxt).atom).quant = XML_REGEXP_QUANT_OPT;
            } else if cur == '*' as i32 {
                (*(*ctxt).atom).quant = XML_REGEXP_QUANT_MULT;
            } else if cur == '+' as i32 {
                (*(*ctxt).atom).quant = XML_REGEXP_QUANT_PLUS;
            }
        }
        let ref mut fresh201 = (*ctxt).cur;
        *fresh201 = (*fresh201).offset(1);
        return 1 as libc::c_int;
    }
    if cur == '{' as i32 {
        let mut min: libc::c_int = 0 as libc::c_int;
        let mut max: libc::c_int = 0 as libc::c_int;
        let ref mut fresh202 = (*ctxt).cur;
        *fresh202 = (*fresh202).offset(1);
        cur = xmlFAParseQuantExact(ctxt);
        if cur >= 0 as libc::c_int {
            min = cur;
        } else {
            (*ctxt).error = XML_REGEXP_COMPILE_ERROR as libc::c_int;
            xmlRegexpErrCompile(
                ctxt,
                b"Improper quantifier\0" as *const u8 as *const libc::c_char,
            );
        }
        if *(*ctxt).cur as libc::c_int == ',' as i32 {
            let ref mut fresh203 = (*ctxt).cur;
            *fresh203 = (*fresh203).offset(1);
            if *(*ctxt).cur as libc::c_int == '}' as i32 {
                max = 2147483647 as libc::c_int;
            } else {
                cur = xmlFAParseQuantExact(ctxt);
                if cur >= 0 as libc::c_int {
                    max = cur;
                } else {
                    (*ctxt).error = XML_REGEXP_COMPILE_ERROR as libc::c_int;
                    xmlRegexpErrCompile(
                        ctxt,
                        b"Improper quantifier\0" as *const u8 as *const libc::c_char,
                    );
                }
            }
        }
        if *(*ctxt).cur as libc::c_int == '}' as i32 {
            let ref mut fresh204 = (*ctxt).cur;
            *fresh204 = (*fresh204).offset(1);
        } else {
            (*ctxt).error = XML_REGEXP_COMPILE_ERROR as libc::c_int;
            xmlRegexpErrCompile(
                ctxt,
                b"Unterminated quantifier\0" as *const u8 as *const libc::c_char,
            );
        }
        if max == 0 as libc::c_int {
            max = min;
        }
        if !((*ctxt).atom).is_null() {
            (*(*ctxt).atom).quant = XML_REGEXP_QUANT_RANGE;
            (*(*ctxt).atom).min = min;
            (*(*ctxt).atom).max = max;
        }
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn xmlFAParseAtom(mut ctxt: xmlRegParserCtxtPtr) -> libc::c_int {
    let mut codepoint: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    codepoint = xmlFAIsChar(ctxt);
    if codepoint > 0 as libc::c_int {
        let ref mut fresh205 = (*ctxt).atom;
        *fresh205 = xmlRegNewAtom(ctxt, XML_REGEXP_CHARVAL);
        if ((*ctxt).atom).is_null() {
            return -(1 as libc::c_int);
        }
        codepoint = xmlStringCurrentChar(0 as xmlParserCtxtPtr, (*ctxt).cur, &mut len);
        (*(*ctxt).atom).codepoint = codepoint;
        let ref mut fresh206 = (*ctxt).cur;
        *fresh206 = (*fresh206).offset(len as isize);
        return 1 as libc::c_int;
    } else {
        if *(*ctxt).cur as libc::c_int == '|' as i32 {
            return 0 as libc::c_int
        } else {
            if *(*ctxt).cur as libc::c_int == 0 as libc::c_int {
                return 0 as libc::c_int
            } else {
                if *(*ctxt).cur as libc::c_int == ')' as i32 {
                    return 0 as libc::c_int
                } else {
                    if *(*ctxt).cur as libc::c_int == '(' as i32 {
                        let mut start: xmlRegStatePtr = 0 as *mut xmlRegState;
                        let mut oldend: xmlRegStatePtr = 0 as *mut xmlRegState;
                        let mut start0: xmlRegStatePtr = 0 as *mut xmlRegState;
                        let ref mut fresh207 = (*ctxt).cur;
                        *fresh207 = (*fresh207).offset(1);
                        if (*ctxt).depth >= 50 as libc::c_int {
                            (*ctxt).error = XML_REGEXP_COMPILE_ERROR as libc::c_int;
                            xmlRegexpErrCompile(
                                ctxt,
                                b"xmlFAParseAtom: maximum nesting depth exceeded\0"
                                    as *const u8 as *const libc::c_char,
                            );
                            return -(1 as libc::c_int);
                        }
                        xmlFAGenerateEpsilonTransition(
                            ctxt,
                            (*ctxt).state,
                            0 as xmlRegStatePtr,
                        );
                        start0 = (*ctxt).state;
                        xmlFAGenerateEpsilonTransition(
                            ctxt,
                            (*ctxt).state,
                            0 as xmlRegStatePtr,
                        );
                        start = (*ctxt).state;
                        oldend = (*ctxt).end;
                        let ref mut fresh208 = (*ctxt).end;
                        *fresh208 = 0 as xmlRegStatePtr;
                        let ref mut fresh209 = (*ctxt).atom;
                        *fresh209 = 0 as xmlRegAtomPtr;
                        let ref mut fresh210 = (*ctxt).depth;
                        *fresh210 += 1;
                        xmlFAParseRegExp(ctxt, 0 as libc::c_int);
                        let ref mut fresh211 = (*ctxt).depth;
                        *fresh211 -= 1;
                        if *(*ctxt).cur as libc::c_int == ')' as i32 {
                            let ref mut fresh212 = (*ctxt).cur;
                            *fresh212 = (*fresh212).offset(1);
                        } else {
                            (*ctxt).error = XML_REGEXP_COMPILE_ERROR as libc::c_int;
                            xmlRegexpErrCompile(
                                ctxt,
                                b"xmlFAParseAtom: expecting ')'\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                        let ref mut fresh213 = (*ctxt).atom;
                        *fresh213 = xmlRegNewAtom(ctxt, XML_REGEXP_SUBREG);
                        if ((*ctxt).atom).is_null() {
                            return -(1 as libc::c_int);
                        }
                        let ref mut fresh214 = (*(*ctxt).atom).start;
                        *fresh214 = start;
                        let ref mut fresh215 = (*(*ctxt).atom).start0;
                        *fresh215 = start0;
                        let ref mut fresh216 = (*(*ctxt).atom).stop;
                        *fresh216 = (*ctxt).state;
                        let ref mut fresh217 = (*ctxt).end;
                        *fresh217 = oldend;
                        return 1 as libc::c_int;
                    } else {
                        if *(*ctxt).cur as libc::c_int == '[' as i32
                            || *(*ctxt).cur as libc::c_int == '\\' as i32
                            || *(*ctxt).cur as libc::c_int == '.' as i32
                        {
                            xmlFAParseCharClass(ctxt);
                            return 1 as libc::c_int;
                        }
                    }
                }
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn xmlFAParsePiece(mut ctxt: xmlRegParserCtxtPtr) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let ref mut fresh218 = (*ctxt).atom;
    *fresh218 = 0 as xmlRegAtomPtr;
    ret = xmlFAParseAtom(ctxt);
    if ret == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if ((*ctxt).atom).is_null() {
        (*ctxt).error = XML_REGEXP_COMPILE_ERROR as libc::c_int;
        xmlRegexpErrCompile(
            ctxt,
            b"internal: no atom generated\0" as *const u8 as *const libc::c_char,
        );
    }
    xmlFAParseQuantifier(ctxt);
    return 1 as libc::c_int;
}
unsafe extern "C" fn xmlFAParseBranch(
    mut ctxt: xmlRegParserCtxtPtr,
    mut to: xmlRegStatePtr,
) -> libc::c_int {
    let mut previous: xmlRegStatePtr = 0 as *mut xmlRegState;
    let mut ret: libc::c_int = 0;
    previous = (*ctxt).state;
    ret = xmlFAParsePiece(ctxt);
    if ret == 0 as libc::c_int {
        xmlFAGenerateEpsilonTransition(ctxt, previous, to);
    } else {
        if xmlFAGenerateTransitions(
            ctxt,
            previous,
            (if *(*ctxt).cur as libc::c_int == '|' as i32
                || *(*ctxt).cur as libc::c_int == ')' as i32
                || *(*ctxt).cur as libc::c_int == 0 as libc::c_int
            {
                to
            } else {
                0 as xmlRegStatePtr
            }),
            (*ctxt).atom,
        ) < 0 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
        previous = (*ctxt).state;
        let ref mut fresh219 = (*ctxt).atom;
        *fresh219 = 0 as xmlRegAtomPtr;
    }
    while ret != 0 as libc::c_int && (*ctxt).error == 0 as libc::c_int {
        ret = xmlFAParsePiece(ctxt);
        if ret != 0 as libc::c_int {
            if xmlFAGenerateTransitions(
                ctxt,
                previous,
                (if *(*ctxt).cur as libc::c_int == '|' as i32
                    || *(*ctxt).cur as libc::c_int == ')' as i32
                    || *(*ctxt).cur as libc::c_int == 0 as libc::c_int
                {
                    to
                } else {
                    0 as xmlRegStatePtr
                }),
                (*ctxt).atom,
            ) < 0 as libc::c_int
            {
                return -(1 as libc::c_int);
            }
            previous = (*ctxt).state;
            let ref mut fresh220 = (*ctxt).atom;
            *fresh220 = 0 as xmlRegAtomPtr;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn xmlFAParseRegExp(
    mut ctxt: xmlRegParserCtxtPtr,
    mut top: libc::c_int,
) {
    let mut start: xmlRegStatePtr = 0 as *mut xmlRegState;
    let mut end: xmlRegStatePtr = 0 as *mut xmlRegState;
    start = (*ctxt).state;
    let ref mut fresh221 = (*ctxt).end;
    *fresh221 = 0 as xmlRegStatePtr;
    xmlFAParseBranch(ctxt, 0 as xmlRegStatePtr);
    if top != 0 {
        (*(*ctxt).state).type_0 = XML_REGEXP_FINAL_STATE;
    }
    if *(*ctxt).cur as libc::c_int != '|' as i32 {
        let ref mut fresh222 = (*ctxt).end;
        *fresh222 = (*ctxt).state;
        return;
    }
    end = (*ctxt).state;
    while *(*ctxt).cur as libc::c_int == '|' as i32 && (*ctxt).error == 0 as libc::c_int
    {
        let ref mut fresh223 = (*ctxt).cur;
        *fresh223 = (*fresh223).offset(1);
        let ref mut fresh224 = (*ctxt).state;
        *fresh224 = start;
        let ref mut fresh225 = (*ctxt).end;
        *fresh225 = 0 as xmlRegStatePtr;
        xmlFAParseBranch(ctxt, end);
    }
    if top == 0 {
        let ref mut fresh226 = (*ctxt).state;
        *fresh226 = end;
        let ref mut fresh227 = (*ctxt).end;
        *fresh227 = end;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlRegexpPrint(
    mut output: *mut FILE,
    mut regexp: xmlRegexpPtr,
) {
    let mut i: libc::c_int = 0;
    if output.is_null() {
        return;
    }
    fprintf(output, b" regexp: \0" as *const u8 as *const libc::c_char);
    if regexp.is_null() {
        fprintf(output, b"NULL\n\0" as *const u8 as *const libc::c_char);
        return;
    }
    fprintf(output, b"'%s' \0" as *const u8 as *const libc::c_char, (*regexp).string);
    fprintf(output, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        output,
        b"%d atoms:\n\0" as *const u8 as *const libc::c_char,
        (*regexp).nbAtoms,
    );
    i = 0 as libc::c_int;
    while i < (*regexp).nbAtoms {
        fprintf(output, b" %02d \0" as *const u8 as *const libc::c_char, i);
        xmlRegPrintAtom(output, *((*regexp).atoms).offset(i as isize));
        i += 1;
    }
    fprintf(
        output,
        b"%d states:\0" as *const u8 as *const libc::c_char,
        (*regexp).nbStates,
    );
    fprintf(output, b"\n\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < (*regexp).nbStates {
        xmlRegPrintState(output, *((*regexp).states).offset(i as isize));
        i += 1;
    }
    fprintf(
        output,
        b"%d counters:\n\0" as *const u8 as *const libc::c_char,
        (*regexp).nbCounters,
    );
    i = 0 as libc::c_int;
    while i < (*regexp).nbCounters {
        fprintf(
            output,
            b" %d: min %d max %d\n\0" as *const u8 as *const libc::c_char,
            i,
            (*((*regexp).counters).offset(i as isize)).min,
            (*((*regexp).counters).offset(i as isize)).max,
        );
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlRegexpCompile(mut regexp: *const xmlChar) -> xmlRegexpPtr {
    let mut ret: xmlRegexpPtr = 0 as *mut xmlRegexp;
    let mut ctxt: xmlRegParserCtxtPtr = 0 as *mut xmlRegParserCtxt;
    ctxt = xmlRegNewParserCtxt(regexp);
    if ctxt.is_null() {
        return 0 as xmlRegexpPtr;
    }
    let ref mut fresh228 = (*ctxt).end;
    *fresh228 = 0 as xmlRegStatePtr;
    let ref mut fresh229 = (*ctxt).state;
    *fresh229 = xmlRegNewState(ctxt);
    let ref mut fresh230 = (*ctxt).start;
    *fresh230 = *fresh229;
    xmlRegStatePush(ctxt, (*ctxt).start);
    xmlFAParseRegExp(ctxt, 1 as libc::c_int);
    if *(*ctxt).cur as libc::c_int != 0 as libc::c_int {
        (*ctxt).error = XML_REGEXP_COMPILE_ERROR as libc::c_int;
        xmlRegexpErrCompile(
            ctxt,
            b"xmlFAParseRegExp: extra characters\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*ctxt).error != 0 as libc::c_int {
        xmlRegFreeParserCtxt(ctxt);
        return 0 as xmlRegexpPtr;
    }
    let ref mut fresh231 = (*ctxt).end;
    *fresh231 = (*ctxt).state;
    (*(*ctxt).start).type_0 = XML_REGEXP_START_STATE;
    (*(*ctxt).end).type_0 = XML_REGEXP_FINAL_STATE;
    xmlFAEliminateEpsilonTransitions(ctxt);
    if (*ctxt).error != 0 as libc::c_int {
        xmlRegFreeParserCtxt(ctxt);
        return 0 as xmlRegexpPtr;
    }
    ret = xmlRegEpxFromParse(ctxt);
    xmlRegFreeParserCtxt(ctxt);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlRegexpExec(
    mut comp: xmlRegexpPtr,
    mut content: *const xmlChar,
) -> libc::c_int {
    if comp.is_null() || content.is_null() {
        return -(1 as libc::c_int);
    }
    return xmlFARegExec(comp, content);
}
#[no_mangle]
pub unsafe extern "C" fn xmlRegexpIsDeterminist(mut comp: xmlRegexpPtr) -> libc::c_int {
    let mut am: xmlAutomataPtr = 0 as *mut xmlAutomata;
    let mut ret: libc::c_int = 0;
    if comp.is_null() {
        return -(1 as libc::c_int);
    }
    if (*comp).determinist != -(1 as libc::c_int) {
        return (*comp).determinist;
    }
    am = xmlNewAutomata();
    if am.is_null() {
        return -(1 as libc::c_int);
    }
    if !((*am).states).is_null() {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < (*am).nbStates {
            xmlRegFreeState(*((*am).states).offset(i as isize));
            i += 1;
        }
        xmlFree.expect("non-null function pointer")((*am).states as *mut libc::c_void);
    }
    (*am).nbAtoms = (*comp).nbAtoms;
    let ref mut fresh232 = (*am).atoms;
    *fresh232 = (*comp).atoms;
    (*am).nbStates = (*comp).nbStates;
    let ref mut fresh233 = (*am).states;
    *fresh233 = (*comp).states;
    (*am).determinist = -(1 as libc::c_int);
    (*am).flags = (*comp).flags;
    ret = xmlFAComputesDeterminism(am);
    let ref mut fresh234 = (*am).atoms;
    *fresh234 = 0 as *mut xmlRegAtomPtr;
    let ref mut fresh235 = (*am).states;
    *fresh235 = 0 as *mut xmlRegStatePtr;
    xmlFreeAutomata(am);
    (*comp).determinist = ret;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlRegFreeRegexp(mut regexp: xmlRegexpPtr) {
    let mut i: libc::c_int = 0;
    if regexp.is_null() {
        return;
    }
    if !((*regexp).string).is_null() {
        xmlFree
            .expect("non-null function pointer")((*regexp).string as *mut libc::c_void);
    }
    if !((*regexp).states).is_null() {
        i = 0 as libc::c_int;
        while i < (*regexp).nbStates {
            xmlRegFreeState(*((*regexp).states).offset(i as isize));
            i += 1;
        }
        xmlFree
            .expect("non-null function pointer")((*regexp).states as *mut libc::c_void);
    }
    if !((*regexp).atoms).is_null() {
        i = 0 as libc::c_int;
        while i < (*regexp).nbAtoms {
            xmlRegFreeAtom(*((*regexp).atoms).offset(i as isize));
            i += 1;
        }
        xmlFree
            .expect("non-null function pointer")((*regexp).atoms as *mut libc::c_void);
    }
    if !((*regexp).counters).is_null() {
        xmlFree
            .expect(
                "non-null function pointer",
            )((*regexp).counters as *mut libc::c_void);
    }
    if !((*regexp).compact).is_null() {
        xmlFree
            .expect("non-null function pointer")((*regexp).compact as *mut libc::c_void);
    }
    if !((*regexp).transdata).is_null() {
        xmlFree
            .expect(
                "non-null function pointer",
            )((*regexp).transdata as *mut libc::c_void);
    }
    if !((*regexp).stringMap).is_null() {
        i = 0 as libc::c_int;
        while i < (*regexp).nbstrings {
            xmlFree
                .expect(
                    "non-null function pointer",
                )(*((*regexp).stringMap).offset(i as isize) as *mut libc::c_void);
            i += 1;
        }
        xmlFree
            .expect(
                "non-null function pointer",
            )((*regexp).stringMap as *mut libc::c_void);
    }
    xmlFree.expect("non-null function pointer")(regexp as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn xmlNewAutomata() -> xmlAutomataPtr {
    let mut ctxt: xmlAutomataPtr = 0 as *mut xmlAutomata;
    ctxt = xmlRegNewParserCtxt(0 as *const xmlChar);
    if ctxt.is_null() {
        return 0 as xmlAutomataPtr;
    }
    let ref mut fresh236 = (*ctxt).end;
    *fresh236 = 0 as xmlRegStatePtr;
    let ref mut fresh237 = (*ctxt).state;
    *fresh237 = xmlRegNewState(ctxt);
    let ref mut fresh238 = (*ctxt).start;
    *fresh238 = *fresh237;
    if ((*ctxt).start).is_null() {
        xmlFreeAutomata(ctxt);
        return 0 as xmlAutomataPtr;
    }
    (*(*ctxt).start).type_0 = XML_REGEXP_START_STATE;
    if xmlRegStatePush(ctxt, (*ctxt).start) < 0 as libc::c_int {
        xmlRegFreeState((*ctxt).start);
        xmlFreeAutomata(ctxt);
        return 0 as xmlAutomataPtr;
    }
    (*ctxt).flags = 0 as libc::c_int;
    return ctxt;
}
#[no_mangle]
pub unsafe extern "C" fn xmlFreeAutomata(mut am: xmlAutomataPtr) {
    if am.is_null() {
        return;
    }
    xmlRegFreeParserCtxt(am);
}
#[no_mangle]
pub unsafe extern "C" fn xmlAutomataSetFlags(
    mut am: xmlAutomataPtr,
    mut flags: libc::c_int,
) {
    if am.is_null() {
        return;
    }
    (*am).flags |= flags;
}
#[no_mangle]
pub unsafe extern "C" fn xmlAutomataGetInitState(
    mut am: xmlAutomataPtr,
) -> xmlAutomataStatePtr {
    if am.is_null() {
        return 0 as xmlAutomataStatePtr;
    }
    return (*am).start;
}
#[no_mangle]
pub unsafe extern "C" fn xmlAutomataSetFinalState(
    mut am: xmlAutomataPtr,
    mut state: xmlAutomataStatePtr,
) -> libc::c_int {
    if am.is_null() || state.is_null() {
        return -(1 as libc::c_int);
    }
    (*state).type_0 = XML_REGEXP_FINAL_STATE;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlAutomataNewTransition(
    mut am: xmlAutomataPtr,
    mut from: xmlAutomataStatePtr,
    mut to: xmlAutomataStatePtr,
    mut token: *const xmlChar,
    mut data: *mut libc::c_void,
) -> xmlAutomataStatePtr {
    let mut atom: xmlRegAtomPtr = 0 as *mut xmlRegAtom;
    if am.is_null() || from.is_null() || token.is_null() {
        return 0 as xmlAutomataStatePtr;
    }
    atom = xmlRegNewAtom(am, XML_REGEXP_STRING);
    if atom.is_null() {
        return 0 as xmlAutomataStatePtr;
    }
    let ref mut fresh239 = (*atom).data;
    *fresh239 = data;
    let ref mut fresh240 = (*atom).valuep;
    *fresh240 = xmlStrdup(token) as *mut libc::c_void;
    if xmlFAGenerateTransitions(am, from, to, atom) < 0 as libc::c_int {
        xmlRegFreeAtom(atom);
        return 0 as xmlAutomataStatePtr;
    }
    if to.is_null() {
        return (*am).state;
    }
    return to;
}
#[no_mangle]
pub unsafe extern "C" fn xmlAutomataNewTransition2(
    mut am: xmlAutomataPtr,
    mut from: xmlAutomataStatePtr,
    mut to: xmlAutomataStatePtr,
    mut token: *const xmlChar,
    mut token2: *const xmlChar,
    mut data: *mut libc::c_void,
) -> xmlAutomataStatePtr {
    let mut atom: xmlRegAtomPtr = 0 as *mut xmlRegAtom;
    if am.is_null() || from.is_null() || token.is_null() {
        return 0 as xmlAutomataStatePtr;
    }
    atom = xmlRegNewAtom(am, XML_REGEXP_STRING);
    if atom.is_null() {
        return 0 as xmlAutomataStatePtr;
    }
    let ref mut fresh241 = (*atom).data;
    *fresh241 = data;
    if token2.is_null() || *token2 as libc::c_int == 0 as libc::c_int {
        let ref mut fresh242 = (*atom).valuep;
        *fresh242 = xmlStrdup(token) as *mut libc::c_void;
    } else {
        let mut lenn: libc::c_int = 0;
        let mut lenp: libc::c_int = 0;
        let mut str: *mut xmlChar = 0 as *mut xmlChar;
        lenn = strlen(token2 as *mut libc::c_char) as libc::c_int;
        lenp = strlen(token as *mut libc::c_char) as libc::c_int;
        str = xmlMallocAtomic
            .expect(
                "non-null function pointer",
            )((lenn + lenp + 2 as libc::c_int) as size_t) as *mut xmlChar;
        if str.is_null() {
            xmlRegFreeAtom(atom);
            return 0 as xmlAutomataStatePtr;
        }
        memcpy(
            &mut *str.offset(0 as libc::c_int as isize) as *mut xmlChar
                as *mut libc::c_void,
            token as *const libc::c_void,
            lenp as libc::c_ulong,
        );
        *str.offset(lenp as isize) = '|' as i32 as xmlChar;
        memcpy(
            &mut *str.offset((lenp + 1 as libc::c_int) as isize) as *mut xmlChar
                as *mut libc::c_void,
            token2 as *const libc::c_void,
            lenn as libc::c_ulong,
        );
        *str
            .offset(
                (lenn + lenp + 1 as libc::c_int) as isize,
            ) = 0 as libc::c_int as xmlChar;
        let ref mut fresh243 = (*atom).valuep;
        *fresh243 = str as *mut libc::c_void;
    }
    if xmlFAGenerateTransitions(am, from, to, atom) < 0 as libc::c_int {
        xmlRegFreeAtom(atom);
        return 0 as xmlAutomataStatePtr;
    }
    if to.is_null() {
        return (*am).state;
    }
    return to;
}
#[no_mangle]
pub unsafe extern "C" fn xmlAutomataNewNegTrans(
    mut am: xmlAutomataPtr,
    mut from: xmlAutomataStatePtr,
    mut to: xmlAutomataStatePtr,
    mut token: *const xmlChar,
    mut token2: *const xmlChar,
    mut data: *mut libc::c_void,
) -> xmlAutomataStatePtr {
    let mut atom: xmlRegAtomPtr = 0 as *mut xmlRegAtom;
    let mut err_msg: [xmlChar; 200] = [0; 200];
    if am.is_null() || from.is_null() || token.is_null() {
        return 0 as xmlAutomataStatePtr;
    }
    atom = xmlRegNewAtom(am, XML_REGEXP_STRING);
    if atom.is_null() {
        return 0 as xmlAutomataStatePtr;
    }
    let ref mut fresh244 = (*atom).data;
    *fresh244 = data;
    (*atom).neg = 1 as libc::c_int;
    if token2.is_null() || *token2 as libc::c_int == 0 as libc::c_int {
        let ref mut fresh245 = (*atom).valuep;
        *fresh245 = xmlStrdup(token) as *mut libc::c_void;
    } else {
        let mut lenn: libc::c_int = 0;
        let mut lenp: libc::c_int = 0;
        let mut str: *mut xmlChar = 0 as *mut xmlChar;
        lenn = strlen(token2 as *mut libc::c_char) as libc::c_int;
        lenp = strlen(token as *mut libc::c_char) as libc::c_int;
        str = xmlMallocAtomic
            .expect(
                "non-null function pointer",
            )((lenn + lenp + 2 as libc::c_int) as size_t) as *mut xmlChar;
        if str.is_null() {
            xmlRegFreeAtom(atom);
            return 0 as xmlAutomataStatePtr;
        }
        memcpy(
            &mut *str.offset(0 as libc::c_int as isize) as *mut xmlChar
                as *mut libc::c_void,
            token as *const libc::c_void,
            lenp as libc::c_ulong,
        );
        *str.offset(lenp as isize) = '|' as i32 as xmlChar;
        memcpy(
            &mut *str.offset((lenp + 1 as libc::c_int) as isize) as *mut xmlChar
                as *mut libc::c_void,
            token2 as *const libc::c_void,
            lenn as libc::c_ulong,
        );
        *str
            .offset(
                (lenn + lenp + 1 as libc::c_int) as isize,
            ) = 0 as libc::c_int as xmlChar;
        let ref mut fresh246 = (*atom).valuep;
        *fresh246 = str as *mut libc::c_void;
    }
    snprintf(
        err_msg.as_mut_ptr() as *mut libc::c_char,
        199 as libc::c_int as libc::c_ulong,
        b"not %s\0" as *const u8 as *const libc::c_char,
        (*atom).valuep as *const libc::c_char,
    );
    err_msg[199 as libc::c_int as usize] = 0 as libc::c_int as xmlChar;
    let ref mut fresh247 = (*atom).valuep2;
    *fresh247 = xmlStrdup(err_msg.as_mut_ptr()) as *mut libc::c_void;
    if xmlFAGenerateTransitions(am, from, to, atom) < 0 as libc::c_int {
        xmlRegFreeAtom(atom);
        return 0 as xmlAutomataStatePtr;
    }
    let ref mut fresh248 = (*am).negs;
    *fresh248 += 1;
    if to.is_null() {
        return (*am).state;
    }
    return to;
}
#[no_mangle]
pub unsafe extern "C" fn xmlAutomataNewCountTrans2(
    mut am: xmlAutomataPtr,
    mut from: xmlAutomataStatePtr,
    mut to: xmlAutomataStatePtr,
    mut token: *const xmlChar,
    mut token2: *const xmlChar,
    mut min: libc::c_int,
    mut max: libc::c_int,
    mut data: *mut libc::c_void,
) -> xmlAutomataStatePtr {
    let mut atom: xmlRegAtomPtr = 0 as *mut xmlRegAtom;
    let mut counter: libc::c_int = 0;
    if am.is_null() || from.is_null() || token.is_null() {
        return 0 as xmlAutomataStatePtr;
    }
    if min < 0 as libc::c_int {
        return 0 as xmlAutomataStatePtr;
    }
    if max < min || max < 1 as libc::c_int {
        return 0 as xmlAutomataStatePtr;
    }
    atom = xmlRegNewAtom(am, XML_REGEXP_STRING);
    if atom.is_null() {
        return 0 as xmlAutomataStatePtr;
    }
    if token2.is_null() || *token2 as libc::c_int == 0 as libc::c_int {
        let ref mut fresh249 = (*atom).valuep;
        *fresh249 = xmlStrdup(token) as *mut libc::c_void;
    } else {
        let mut lenn: libc::c_int = 0;
        let mut lenp: libc::c_int = 0;
        let mut str: *mut xmlChar = 0 as *mut xmlChar;
        lenn = strlen(token2 as *mut libc::c_char) as libc::c_int;
        lenp = strlen(token as *mut libc::c_char) as libc::c_int;
        str = xmlMallocAtomic
            .expect(
                "non-null function pointer",
            )((lenn + lenp + 2 as libc::c_int) as size_t) as *mut xmlChar;
        if str.is_null() {
            xmlRegFreeAtom(atom);
            return 0 as xmlAutomataStatePtr;
        }
        memcpy(
            &mut *str.offset(0 as libc::c_int as isize) as *mut xmlChar
                as *mut libc::c_void,
            token as *const libc::c_void,
            lenp as libc::c_ulong,
        );
        *str.offset(lenp as isize) = '|' as i32 as xmlChar;
        memcpy(
            &mut *str.offset((lenp + 1 as libc::c_int) as isize) as *mut xmlChar
                as *mut libc::c_void,
            token2 as *const libc::c_void,
            lenn as libc::c_ulong,
        );
        *str
            .offset(
                (lenn + lenp + 1 as libc::c_int) as isize,
            ) = 0 as libc::c_int as xmlChar;
        let ref mut fresh250 = (*atom).valuep;
        *fresh250 = str as *mut libc::c_void;
    }
    let ref mut fresh251 = (*atom).data;
    *fresh251 = data;
    if min == 0 as libc::c_int {
        (*atom).min = 1 as libc::c_int;
    } else {
        (*atom).min = min;
    }
    (*atom).max = max;
    counter = xmlRegGetCounter(am);
    (*((*am).counters).offset(counter as isize)).min = min;
    (*((*am).counters).offset(counter as isize)).max = max;
    if to.is_null() {
        to = xmlRegNewState(am);
        xmlRegStatePush(am, to);
    }
    xmlRegStateAddTrans(am, from, atom, to, counter, -(1 as libc::c_int));
    xmlRegAtomPush(am, atom);
    let ref mut fresh252 = (*am).state;
    *fresh252 = to;
    if to.is_null() {
        to = (*am).state;
    }
    if to.is_null() {
        return 0 as xmlAutomataStatePtr;
    }
    if min == 0 as libc::c_int {
        xmlFAGenerateEpsilonTransition(am, from, to);
    }
    return to;
}
#[no_mangle]
pub unsafe extern "C" fn xmlAutomataNewCountTrans(
    mut am: xmlAutomataPtr,
    mut from: xmlAutomataStatePtr,
    mut to: xmlAutomataStatePtr,
    mut token: *const xmlChar,
    mut min: libc::c_int,
    mut max: libc::c_int,
    mut data: *mut libc::c_void,
) -> xmlAutomataStatePtr {
    let mut atom: xmlRegAtomPtr = 0 as *mut xmlRegAtom;
    let mut counter: libc::c_int = 0;
    if am.is_null() || from.is_null() || token.is_null() {
        return 0 as xmlAutomataStatePtr;
    }
    if min < 0 as libc::c_int {
        return 0 as xmlAutomataStatePtr;
    }
    if max < min || max < 1 as libc::c_int {
        return 0 as xmlAutomataStatePtr;
    }
    atom = xmlRegNewAtom(am, XML_REGEXP_STRING);
    if atom.is_null() {
        return 0 as xmlAutomataStatePtr;
    }
    let ref mut fresh253 = (*atom).valuep;
    *fresh253 = xmlStrdup(token) as *mut libc::c_void;
    let ref mut fresh254 = (*atom).data;
    *fresh254 = data;
    if min == 0 as libc::c_int {
        (*atom).min = 1 as libc::c_int;
    } else {
        (*atom).min = min;
    }
    (*atom).max = max;
    counter = xmlRegGetCounter(am);
    (*((*am).counters).offset(counter as isize)).min = min;
    (*((*am).counters).offset(counter as isize)).max = max;
    if to.is_null() {
        to = xmlRegNewState(am);
        xmlRegStatePush(am, to);
    }
    xmlRegStateAddTrans(am, from, atom, to, counter, -(1 as libc::c_int));
    xmlRegAtomPush(am, atom);
    let ref mut fresh255 = (*am).state;
    *fresh255 = to;
    if to.is_null() {
        to = (*am).state;
    }
    if to.is_null() {
        return 0 as xmlAutomataStatePtr;
    }
    if min == 0 as libc::c_int {
        xmlFAGenerateEpsilonTransition(am, from, to);
    }
    return to;
}
#[no_mangle]
pub unsafe extern "C" fn xmlAutomataNewOnceTrans2(
    mut am: xmlAutomataPtr,
    mut from: xmlAutomataStatePtr,
    mut to: xmlAutomataStatePtr,
    mut token: *const xmlChar,
    mut token2: *const xmlChar,
    mut min: libc::c_int,
    mut max: libc::c_int,
    mut data: *mut libc::c_void,
) -> xmlAutomataStatePtr {
    let mut atom: xmlRegAtomPtr = 0 as *mut xmlRegAtom;
    let mut counter: libc::c_int = 0;
    if am.is_null() || from.is_null() || token.is_null() {
        return 0 as xmlAutomataStatePtr;
    }
    if min < 1 as libc::c_int {
        return 0 as xmlAutomataStatePtr;
    }
    if max < min {
        return 0 as xmlAutomataStatePtr;
    }
    atom = xmlRegNewAtom(am, XML_REGEXP_STRING);
    if atom.is_null() {
        return 0 as xmlAutomataStatePtr;
    }
    if token2.is_null() || *token2 as libc::c_int == 0 as libc::c_int {
        let ref mut fresh256 = (*atom).valuep;
        *fresh256 = xmlStrdup(token) as *mut libc::c_void;
    } else {
        let mut lenn: libc::c_int = 0;
        let mut lenp: libc::c_int = 0;
        let mut str: *mut xmlChar = 0 as *mut xmlChar;
        lenn = strlen(token2 as *mut libc::c_char) as libc::c_int;
        lenp = strlen(token as *mut libc::c_char) as libc::c_int;
        str = xmlMallocAtomic
            .expect(
                "non-null function pointer",
            )((lenn + lenp + 2 as libc::c_int) as size_t) as *mut xmlChar;
        if str.is_null() {
            xmlRegFreeAtom(atom);
            return 0 as xmlAutomataStatePtr;
        }
        memcpy(
            &mut *str.offset(0 as libc::c_int as isize) as *mut xmlChar
                as *mut libc::c_void,
            token as *const libc::c_void,
            lenp as libc::c_ulong,
        );
        *str.offset(lenp as isize) = '|' as i32 as xmlChar;
        memcpy(
            &mut *str.offset((lenp + 1 as libc::c_int) as isize) as *mut xmlChar
                as *mut libc::c_void,
            token2 as *const libc::c_void,
            lenn as libc::c_ulong,
        );
        *str
            .offset(
                (lenn + lenp + 1 as libc::c_int) as isize,
            ) = 0 as libc::c_int as xmlChar;
        let ref mut fresh257 = (*atom).valuep;
        *fresh257 = str as *mut libc::c_void;
    }
    let ref mut fresh258 = (*atom).data;
    *fresh258 = data;
    (*atom).quant = XML_REGEXP_QUANT_ONCEONLY;
    (*atom).min = min;
    (*atom).max = max;
    counter = xmlRegGetCounter(am);
    (*((*am).counters).offset(counter as isize)).min = 1 as libc::c_int;
    (*((*am).counters).offset(counter as isize)).max = 1 as libc::c_int;
    if to.is_null() {
        to = xmlRegNewState(am);
        xmlRegStatePush(am, to);
    }
    xmlRegStateAddTrans(am, from, atom, to, counter, -(1 as libc::c_int));
    xmlRegAtomPush(am, atom);
    let ref mut fresh259 = (*am).state;
    *fresh259 = to;
    return to;
}
#[no_mangle]
pub unsafe extern "C" fn xmlAutomataNewOnceTrans(
    mut am: xmlAutomataPtr,
    mut from: xmlAutomataStatePtr,
    mut to: xmlAutomataStatePtr,
    mut token: *const xmlChar,
    mut min: libc::c_int,
    mut max: libc::c_int,
    mut data: *mut libc::c_void,
) -> xmlAutomataStatePtr {
    let mut atom: xmlRegAtomPtr = 0 as *mut xmlRegAtom;
    let mut counter: libc::c_int = 0;
    if am.is_null() || from.is_null() || token.is_null() {
        return 0 as xmlAutomataStatePtr;
    }
    if min < 1 as libc::c_int {
        return 0 as xmlAutomataStatePtr;
    }
    if max < min {
        return 0 as xmlAutomataStatePtr;
    }
    atom = xmlRegNewAtom(am, XML_REGEXP_STRING);
    if atom.is_null() {
        return 0 as xmlAutomataStatePtr;
    }
    let ref mut fresh260 = (*atom).valuep;
    *fresh260 = xmlStrdup(token) as *mut libc::c_void;
    let ref mut fresh261 = (*atom).data;
    *fresh261 = data;
    (*atom).quant = XML_REGEXP_QUANT_ONCEONLY;
    (*atom).min = min;
    (*atom).max = max;
    counter = xmlRegGetCounter(am);
    (*((*am).counters).offset(counter as isize)).min = 1 as libc::c_int;
    (*((*am).counters).offset(counter as isize)).max = 1 as libc::c_int;
    if to.is_null() {
        to = xmlRegNewState(am);
        xmlRegStatePush(am, to);
    }
    xmlRegStateAddTrans(am, from, atom, to, counter, -(1 as libc::c_int));
    xmlRegAtomPush(am, atom);
    let ref mut fresh262 = (*am).state;
    *fresh262 = to;
    return to;
}
#[no_mangle]
pub unsafe extern "C" fn xmlAutomataNewState(
    mut am: xmlAutomataPtr,
) -> xmlAutomataStatePtr {
    let mut to: xmlAutomataStatePtr = 0 as *mut xmlAutomataState;
    if am.is_null() {
        return 0 as xmlAutomataStatePtr;
    }
    to = xmlRegNewState(am);
    xmlRegStatePush(am, to);
    return to;
}
#[no_mangle]
pub unsafe extern "C" fn xmlAutomataNewEpsilon(
    mut am: xmlAutomataPtr,
    mut from: xmlAutomataStatePtr,
    mut to: xmlAutomataStatePtr,
) -> xmlAutomataStatePtr {
    if am.is_null() || from.is_null() {
        return 0 as xmlAutomataStatePtr;
    }
    xmlFAGenerateEpsilonTransition(am, from, to);
    if to.is_null() {
        return (*am).state;
    }
    return to;
}
#[no_mangle]
pub unsafe extern "C" fn xmlAutomataNewAllTrans(
    mut am: xmlAutomataPtr,
    mut from: xmlAutomataStatePtr,
    mut to: xmlAutomataStatePtr,
    mut lax: libc::c_int,
) -> xmlAutomataStatePtr {
    if am.is_null() || from.is_null() {
        return 0 as xmlAutomataStatePtr;
    }
    xmlFAGenerateAllTransition(am, from, to, lax);
    if to.is_null() {
        return (*am).state;
    }
    return to;
}
#[no_mangle]
pub unsafe extern "C" fn xmlAutomataNewCounter(
    mut am: xmlAutomataPtr,
    mut min: libc::c_int,
    mut max: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    if am.is_null() {
        return -(1 as libc::c_int);
    }
    ret = xmlRegGetCounter(am);
    if ret < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    (*((*am).counters).offset(ret as isize)).min = min;
    (*((*am).counters).offset(ret as isize)).max = max;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlAutomataNewCountedTrans(
    mut am: xmlAutomataPtr,
    mut from: xmlAutomataStatePtr,
    mut to: xmlAutomataStatePtr,
    mut counter: libc::c_int,
) -> xmlAutomataStatePtr {
    if am.is_null() || from.is_null() || counter < 0 as libc::c_int {
        return 0 as xmlAutomataStatePtr;
    }
    xmlFAGenerateCountedEpsilonTransition(am, from, to, counter);
    if to.is_null() {
        return (*am).state;
    }
    return to;
}
#[no_mangle]
pub unsafe extern "C" fn xmlAutomataNewCounterTrans(
    mut am: xmlAutomataPtr,
    mut from: xmlAutomataStatePtr,
    mut to: xmlAutomataStatePtr,
    mut counter: libc::c_int,
) -> xmlAutomataStatePtr {
    if am.is_null() || from.is_null() || counter < 0 as libc::c_int {
        return 0 as xmlAutomataStatePtr;
    }
    xmlFAGenerateCountedTransition(am, from, to, counter);
    if to.is_null() {
        return (*am).state;
    }
    return to;
}
#[no_mangle]
pub unsafe extern "C" fn xmlAutomataCompile(mut am: xmlAutomataPtr) -> xmlRegexpPtr {
    let mut ret: xmlRegexpPtr = 0 as *mut xmlRegexp;
    if am.is_null() || (*am).error != 0 as libc::c_int {
        return 0 as xmlRegexpPtr;
    }
    xmlFAEliminateEpsilonTransitions(am);
    ret = xmlRegEpxFromParse(am);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlAutomataIsDeterminist(
    mut am: xmlAutomataPtr,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    if am.is_null() {
        return -(1 as libc::c_int);
    }
    ret = xmlFAComputesDeterminism(am);
    return ret;
}
