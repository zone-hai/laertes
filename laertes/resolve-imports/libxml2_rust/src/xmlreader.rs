use ::libc;
extern "C" {
<<<<<<< HEAD
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
=======
    pub type _xmlBuf;
    pub type _xmlDict;
    pub type _xmlHashTable;
    pub type _xmlStartTag;
    pub type _xmlAutomataState;
    pub type _xmlAutomata;
    pub type _xmlValidState;
    pub type _xmlRelaxNG;
    pub type _xmlRelaxNGParserCtxt;
    pub type _xmlRelaxNGValidCtxt;
    pub type _xmlSchema;
    pub type _xmlSchemaParserCtxt;
    pub type _xmlSchemaValidCtxt;
    pub type _xmlSchemaSAXPlug;
    pub type _xmlPattern;
    pub type _xmlXIncludeCtxt;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
    fn vsnprintf(
        _: *mut i8,
        _: u64,
        _: *const i8,
        _: ::std::ffi::VaList,
    ) -> i32;
<<<<<<< HEAD
    
    
    
    
    
=======
    fn xmlStrcat(cur: *mut xmlChar, add: *const xmlChar) -> *mut xmlChar;
    fn xmlStrlen(str: *const xmlChar) -> i32;
    fn xmlStrEqual(str1: *const xmlChar, str2: *const xmlChar) -> i32;
    fn xmlStrdup(cur: *const xmlChar) -> *mut xmlChar;
    static mut __xmlRegisterCallbacks: i32;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
    fn memset(
        _: *mut libc::c_void,
        _: i32,
        _: u64,
    ) -> *mut libc::c_void;
<<<<<<< HEAD
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
}
pub use crate::src::SAX2::xmlSAXVersion;
pub use crate::src::buf::xmlBufContent;
pub use crate::src::buf::xmlBufCreateSize;
pub use crate::src::buf::xmlBufEmpty;
pub use crate::src::buf::xmlBufFree;
pub use crate::src::buf::xmlBufGetAllocationScheme;
pub use crate::src::buf::xmlBufResetInput;
pub use crate::src::buf::xmlBufSetAllocationScheme;
pub use crate::src::buf::xmlBufShrink;
pub use crate::src::buf::xmlBufUse;
pub use crate::src::dict::xmlDictCreate;
pub use crate::src::dict::xmlDictFree;
pub use crate::src::dict::xmlDictLookup;
pub use crate::src::dict::xmlDictOwns;
pub use crate::src::dict::xmlDictQLookup;
pub use crate::src::encoding::xmlByteConsumed;
pub use crate::src::encoding::xmlFindCharEncodingHandler;
pub use crate::src::error::xmlParserError;
pub use crate::src::error::xmlParserValidityError;
pub use crate::src::error::xmlParserValidityWarning;
pub use crate::src::error::xmlParserWarning;
pub use crate::src::globals::__xmlDeregisterNodeDefaultValue;
pub use crate::src::globals::__xmlGenericError;
pub use crate::src::globals::__xmlGenericErrorContext;
pub use crate::src::parser::inputPush;
pub use crate::src::parser::xmlCreatePushParserCtxt;
pub use crate::src::parser::xmlCtxtReset;
pub use crate::src::parser::xmlCtxtUseOptions;
pub use crate::src::parser::xmlParseChunk;
pub use crate::src::parser::xmlStopParser;
pub use crate::src::parserInternals::xmlFreeParserCtxt;
pub use crate::src::parserInternals::xmlNewInputStream;
pub use crate::src::parserInternals::xmlSwitchToEncoding;
pub use crate::src::pattern::xmlFreePattern;
pub use crate::src::pattern::xmlPatternMatch;
pub use crate::src::pattern::xmlPatterncompile;
pub use crate::src::relaxng::xmlRelaxNGFree;
pub use crate::src::relaxng::xmlRelaxNGFreeParserCtxt;
pub use crate::src::relaxng::xmlRelaxNGFreeValidCtxt;
pub use crate::src::relaxng::xmlRelaxNGNewParserCtxt;
pub use crate::src::relaxng::xmlRelaxNGNewValidCtxt;
pub use crate::src::relaxng::xmlRelaxNGParse;
pub use crate::src::relaxng::xmlRelaxNGSetParserErrors;
pub use crate::src::relaxng::xmlRelaxNGSetValidErrors;
pub use crate::src::relaxng::xmlRelaxNGSetValidStructuredErrors;
pub use crate::src::relaxng::xmlRelaxNGValidateFullElement;
pub use crate::src::relaxng::xmlRelaxNGValidatePopElement;
pub use crate::src::relaxng::xmlRelaxNGValidatePushCData;
pub use crate::src::relaxng::xmlRelaxNGValidatePushElement;
pub use crate::src::tree::xmlBufGetNodeContent;
pub use crate::src::tree::xmlBufferCat;
pub use crate::src::tree::xmlBufferCreate;
pub use crate::src::tree::xmlBufferFree;
pub use crate::src::tree::xmlBufferSetAllocationScheme;
pub use crate::src::tree::xmlCopyDtd;
pub use crate::src::tree::xmlDocCopyNode;
pub use crate::src::tree::xmlFreeDoc;
pub use crate::src::tree::xmlFreeDtd;
pub use crate::src::tree::xmlFreeNode;
pub use crate::src::tree::xmlFreeNs;
pub use crate::src::tree::xmlFreeNsList;
pub use crate::src::tree::xmlGetLineNo;
pub use crate::src::tree::xmlGetNoNsProp;
pub use crate::src::tree::xmlGetNsProp;
pub use crate::src::tree::xmlIsBlankNode;
pub use crate::src::tree::xmlNewDocText;
pub use crate::src::tree::xmlNodeGetBase;
pub use crate::src::tree::xmlNodeGetLang;
pub use crate::src::tree::xmlNodeGetSpacePreserve;
pub use crate::src::tree::xmlNodeListGetString;
pub use crate::src::tree::xmlSearchNs;
pub use crate::src::tree::xmlSplitQName2;
pub use crate::src::tree::xmlUnlinkNode;
pub use crate::src::uri::xmlCanonicPath;
pub use crate::src::valid::xmlFreeIDTable;
pub use crate::src::valid::xmlFreeRefTable;
pub use crate::src::valid::xmlValidatePopElement;
pub use crate::src::valid::xmlValidatePushCData;
pub use crate::src::valid::xmlValidatePushElement;
pub use crate::src::xinclude::xmlXIncludeFreeContext;
pub use crate::src::xinclude::xmlXIncludeNewContext;
pub use crate::src::xinclude::xmlXIncludeProcessNode;
pub use crate::src::xinclude::xmlXIncludeSetFlags;
pub use crate::src::xmlIO::xmlAllocParserInputBuffer;
pub use crate::src::xmlIO::xmlFreeParserInputBuffer;
pub use crate::src::xmlIO::xmlParserGetDirectory;
pub use crate::src::xmlIO::xmlParserInputBufferCreateFd;
pub use crate::src::xmlIO::xmlParserInputBufferCreateFilename;
pub use crate::src::xmlIO::xmlParserInputBufferCreateIO;
pub use crate::src::xmlIO::xmlParserInputBufferCreateStatic;
pub use crate::src::xmlIO::xmlParserInputBufferRead;
pub use crate::src::xmlsave::xmlNodeDump;
pub use crate::src::xmlschemas::xmlSchemaFree;
pub use crate::src::xmlschemas::xmlSchemaFreeParserCtxt;
pub use crate::src::xmlschemas::xmlSchemaFreeValidCtxt;
pub use crate::src::xmlschemas::xmlSchemaIsValid;
pub use crate::src::xmlschemas::xmlSchemaNewParserCtxt;
pub use crate::src::xmlschemas::xmlSchemaNewValidCtxt;
pub use crate::src::xmlschemas::xmlSchemaParse;
pub use crate::src::xmlschemas::xmlSchemaSAXPlug;
pub use crate::src::xmlschemas::xmlSchemaSAXUnplug;
pub use crate::src::xmlschemas::xmlSchemaSetParserErrors;
pub use crate::src::xmlschemas::xmlSchemaSetValidErrors;
pub use crate::src::xmlschemas::xmlSchemaSetValidStructuredErrors;
pub use crate::src::xmlschemas::xmlSchemaValidateSetLocator;
pub use crate::src::xmlstring::xmlStrEqual;
pub use crate::src::xmlstring::xmlStrcat;
pub use crate::src::xmlstring::xmlStrdup;
pub use crate::src::xmlstring::xmlStrlen;
pub use crate::src::buf::_xmlBuf;
pub use crate::src::dict::_xmlDict;
pub use crate::src::hash::_xmlHashTable;
pub use crate::src::parser::_xmlStartTag;
pub use crate::src::pattern::_xmlPattern;
pub use crate::src::relaxng::_xmlRelaxNG;
pub use crate::src::relaxng::_xmlRelaxNGParserCtxt;
pub use crate::src::relaxng::_xmlRelaxNGValidCtxt;
pub use crate::src::valid::_xmlValidState;
pub use crate::src::xinclude::_xmlXIncludeCtxt;
pub use crate::src::globals::xmlFree;
pub use crate::src::globals::xmlMalloc;
pub use crate::src::globals::xmlRealloc;
pub use crate::src::tree::__xmlRegisterCallbacks;
pub use crate::src::xmlregexp::_xmlAutomata;
pub use crate::src::xmlregexp::_xmlAutomataState;
pub use crate::src::xmlschemas::_xmlSchema;
pub use crate::src::xmlschemas::_xmlSchemaParserCtxt;
pub use crate::src::xmlschemas::_xmlSchemaSAXPlug;
pub use crate::src::xmlschemas::_xmlSchemaValidCtxt;
pub type __builtin_va_list = crate::src::error::__builtin_va_list;
// #[derive(Copy, Clone)]

pub type __va_list_tag = crate::src::error::__va_list_tag;
pub type va_list = crate::src::error::va_list;
pub type xmlChar = crate::src::HTMLparser::xmlChar;
pub type size_t = crate::src::HTMLparser::size_t;
pub type xmlFreeFunc = crate::src::HTMLparser::xmlFreeFunc;
pub type xmlMallocFunc = crate::src::HTMLparser::xmlMallocFunc;
pub type xmlReallocFunc = crate::src::HTMLparser::xmlReallocFunc;
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
    fn xmlBufContent(buf: *const xmlBuf) -> *mut xmlChar;
    fn xmlBufUse(buf: xmlBufPtr) -> size_t;
    fn xmlBufShrink(buf: xmlBufPtr, len: size_t) -> size_t;
    fn xmlDictCreate() -> xmlDictPtr;
    fn xmlDictFree(dict: xmlDictPtr);
    fn xmlDictLookup(
        dict: xmlDictPtr,
        name: *const xmlChar,
        len: i32,
    ) -> *const xmlChar;
    fn xmlDictQLookup(
        dict: xmlDictPtr,
        prefix: *const xmlChar,
        name: *const xmlChar,
    ) -> *const xmlChar;
    fn xmlDictOwns(dict: xmlDictPtr, str: *const xmlChar) -> i32;
    fn xmlSplitQName2(name: *const xmlChar, prefix: *mut *mut xmlChar) -> *mut xmlChar;
    fn xmlBufferCreate() -> xmlBufferPtr;
    fn xmlBufferFree(buf: xmlBufferPtr);
    fn xmlBufferCat(buf: xmlBufferPtr, str: *const xmlChar) -> i32;
    fn xmlBufferSetAllocationScheme(
        buf: xmlBufferPtr,
        scheme: xmlBufferAllocationScheme,
    );
    fn xmlFreeDtd(cur: xmlDtdPtr);
    fn xmlFreeNs(cur: xmlNsPtr);
    fn xmlFreeNsList(cur: xmlNsPtr);
    fn xmlFreeDoc(cur: xmlDocPtr);
    fn xmlCopyDtd(dtd: xmlDtdPtr) -> xmlDtdPtr;
    fn xmlNewDocText(doc: *const xmlDoc, content: *const xmlChar) -> xmlNodePtr;
    fn xmlDocCopyNode(
        node: xmlNodePtr,
        doc: xmlDocPtr,
        recursive: i32,
    ) -> xmlNodePtr;
    fn xmlGetLineNo(node: *const xmlNode) -> i64;
    fn xmlIsBlankNode(node: *const xmlNode) -> i32;
    fn xmlUnlinkNode(cur: xmlNodePtr);
    fn xmlFreeNode(cur: xmlNodePtr);
    fn xmlSearchNs(
        doc: xmlDocPtr,
        node: xmlNodePtr,
        nameSpace: *const xmlChar,
    ) -> xmlNsPtr;
    fn xmlGetNoNsProp(node: *const xmlNode, name: *const xmlChar) -> *mut xmlChar;
    fn xmlGetNsProp(
        node: *const xmlNode,
        name: *const xmlChar,
        nameSpace: *const xmlChar,
    ) -> *mut xmlChar;
    fn xmlNodeListGetString(
        doc: xmlDocPtr,
        list: *const xmlNode,
        inLine: i32,
    ) -> *mut xmlChar;
    fn xmlBufGetNodeContent(buf: xmlBufPtr, cur: *const xmlNode) -> i32;
    fn xmlNodeGetLang(cur: *const xmlNode) -> *mut xmlChar;
    fn xmlNodeGetSpacePreserve(cur: *const xmlNode) -> i32;
    fn xmlNodeGetBase(doc: *const xmlDoc, cur: *const xmlNode) -> *mut xmlChar;
    fn xmlNodeDump(
        buf: xmlBufferPtr,
        doc: xmlDocPtr,
        cur: xmlNodePtr,
        level: i32,
        format: i32,
    ) -> i32;
    fn xmlParserError(ctx: *mut libc::c_void, msg: *const i8, _: ...);
    fn xmlParserWarning(ctx: *mut libc::c_void, msg: *const i8, _: ...);
    fn xmlParserValidityError(ctx: *mut libc::c_void, msg: *const i8, _: ...);
    fn xmlParserValidityWarning(
        ctx: *mut libc::c_void,
        msg: *const i8,
        _: ...
    );
    fn xmlFreeIDTable(table: xmlIDTablePtr);
    fn xmlFreeRefTable(table: xmlRefTablePtr);
    fn xmlValidatePushElement(
        ctxt: xmlValidCtxtPtr,
        doc: xmlDocPtr,
        elem: xmlNodePtr,
        qname: *const xmlChar,
    ) -> i32;
    fn xmlValidatePushCData(
        ctxt: xmlValidCtxtPtr,
        data: *const xmlChar,
        len: i32,
    ) -> i32;
    fn xmlValidatePopElement(
        ctxt: xmlValidCtxtPtr,
        doc: xmlDocPtr,
        elem: xmlNodePtr,
        qname: *const xmlChar,
    ) -> i32;
    fn xmlFindCharEncodingHandler(
        name: *const i8,
    ) -> xmlCharEncodingHandlerPtr;
    fn xmlAllocParserInputBuffer(enc: xmlCharEncoding) -> xmlParserInputBufferPtr;
    fn xmlParserInputBufferCreateFilename(
        URI: *const i8,
        enc: xmlCharEncoding,
    ) -> xmlParserInputBufferPtr;
    fn xmlParserInputBufferCreateFd(
        fd: i32,
        enc: xmlCharEncoding,
    ) -> xmlParserInputBufferPtr;
    fn xmlParserInputBufferCreateStatic(
        mem: *const i8,
        size: i32,
        enc: xmlCharEncoding,
    ) -> xmlParserInputBufferPtr;
    fn xmlParserInputBufferCreateIO(
        ioread: xmlInputReadCallback,
        ioclose: xmlInputCloseCallback,
        ioctx: *mut libc::c_void,
        enc: xmlCharEncoding,
    ) -> xmlParserInputBufferPtr;
    fn xmlParserInputBufferRead(
        in_0: xmlParserInputBufferPtr,
        len: i32,
    ) -> i32;
    fn xmlFreeParserInputBuffer(in_0: xmlParserInputBufferPtr);
    fn xmlParserGetDirectory(filename: *const i8) -> *mut i8;
    fn xmlStopParser(ctxt: xmlParserCtxtPtr);
    fn xmlFreeParserCtxt(ctxt: xmlParserCtxtPtr);
    fn xmlCreatePushParserCtxt(
        sax: xmlSAXHandlerPtr,
        user_data: *mut libc::c_void,
        chunk: *const i8,
        size: i32,
        filename: *const i8,
    ) -> xmlParserCtxtPtr;
    fn xmlParseChunk(
        ctxt: xmlParserCtxtPtr,
        chunk: *const i8,
        size: i32,
        terminate: i32,
    ) -> i32;
    fn xmlSAXVersion(hdlr: *mut xmlSAXHandler, version: i32) -> i32;
    static mut xmlMalloc: xmlMallocFunc;
    static mut xmlRealloc: xmlReallocFunc;
    static mut xmlFree: xmlFreeFunc;
    fn __xmlGenericError() -> *mut xmlGenericErrorFunc;
    fn __xmlGenericErrorContext() -> *mut *mut libc::c_void;
    fn __xmlDeregisterNodeDefaultValue() -> *mut xmlDeregisterNodeFunc;
    fn xmlCtxtUseOptions(ctxt: xmlParserCtxtPtr, options: i32) -> i32;
    fn xmlCtxtReset(ctxt: xmlParserCtxtPtr);
    fn xmlByteConsumed(ctxt: xmlParserCtxtPtr) -> i64;
    fn xmlRelaxNGFree(schema: xmlRelaxNGPtr);
    fn xmlRelaxNGSetValidErrors(
        ctxt: xmlRelaxNGValidCtxtPtr,
        err: xmlRelaxNGValidityErrorFunc,
        warn: xmlRelaxNGValidityWarningFunc,
        ctx: *mut libc::c_void,
    );
    fn xmlRelaxNGParse(ctxt: xmlRelaxNGParserCtxtPtr) -> xmlRelaxNGPtr;
    fn xmlRelaxNGSetValidStructuredErrors(
        ctxt: xmlRelaxNGValidCtxtPtr,
        serror: xmlStructuredErrorFunc,
        ctx: *mut libc::c_void,
    );
    fn xmlRelaxNGNewValidCtxt(schema: xmlRelaxNGPtr) -> xmlRelaxNGValidCtxtPtr;
    fn xmlRelaxNGFreeValidCtxt(ctxt: xmlRelaxNGValidCtxtPtr);
    fn xmlRelaxNGValidatePushElement(
        ctxt: xmlRelaxNGValidCtxtPtr,
        doc: xmlDocPtr,
        elem: xmlNodePtr,
    ) -> i32;
    fn xmlRelaxNGValidatePushCData(
        ctxt: xmlRelaxNGValidCtxtPtr,
        data: *const xmlChar,
        len: i32,
    ) -> i32;
    fn xmlRelaxNGValidatePopElement(
        ctxt: xmlRelaxNGValidCtxtPtr,
        doc: xmlDocPtr,
        elem: xmlNodePtr,
    ) -> i32;
    fn xmlRelaxNGValidateFullElement(
        ctxt: xmlRelaxNGValidCtxtPtr,
        doc: xmlDocPtr,
        elem: xmlNodePtr,
    ) -> i32;
    fn xmlRelaxNGNewParserCtxt(URL: *const i8) -> xmlRelaxNGParserCtxtPtr;
    fn xmlRelaxNGFreeParserCtxt(ctxt: xmlRelaxNGParserCtxtPtr);
    fn xmlRelaxNGSetParserErrors(
        ctxt: xmlRelaxNGParserCtxtPtr,
        err: xmlRelaxNGValidityErrorFunc,
        warn: xmlRelaxNGValidityWarningFunc,
        ctx: *mut libc::c_void,
    );
    fn xmlSchemaIsValid(ctxt: xmlSchemaValidCtxtPtr) -> i32;
    fn xmlSchemaFreeParserCtxt(ctxt: xmlSchemaParserCtxtPtr);
    fn xmlSchemaSetParserErrors(
        ctxt: xmlSchemaParserCtxtPtr,
        err: xmlSchemaValidityErrorFunc,
        warn: xmlSchemaValidityWarningFunc,
        ctx: *mut libc::c_void,
    );
    fn xmlSchemaNewParserCtxt(URL: *const i8) -> xmlSchemaParserCtxtPtr;
    fn xmlSchemaFreeValidCtxt(ctxt: xmlSchemaValidCtxtPtr);
    fn xmlSchemaParse(ctxt: xmlSchemaParserCtxtPtr) -> xmlSchemaPtr;
    fn xmlSchemaFree(schema: xmlSchemaPtr);
    fn xmlSchemaSetValidErrors(
        ctxt: xmlSchemaValidCtxtPtr,
        err: xmlSchemaValidityErrorFunc,
        warn: xmlSchemaValidityWarningFunc,
        ctx: *mut libc::c_void,
    );
    fn xmlSchemaNewValidCtxt(schema: xmlSchemaPtr) -> xmlSchemaValidCtxtPtr;
    fn xmlSchemaSetValidStructuredErrors(
        ctxt: xmlSchemaValidCtxtPtr,
        serror: xmlStructuredErrorFunc,
        ctx: *mut libc::c_void,
    );
    fn xmlSchemaSAXPlug(
        ctxt: xmlSchemaValidCtxtPtr,
        sax: *mut xmlSAXHandlerPtr,
        user_data: *mut *mut libc::c_void,
    ) -> xmlSchemaSAXPlugPtr;
    fn xmlSchemaSAXUnplug(plug: xmlSchemaSAXPlugPtr) -> i32;
    fn xmlSchemaValidateSetLocator(
        vctxt: xmlSchemaValidCtxtPtr,
        f: xmlSchemaValidityLocatorFunc,
        ctxt: *mut libc::c_void,
    );
    fn xmlSwitchToEncoding(
        ctxt: xmlParserCtxtPtr,
        handler: xmlCharEncodingHandlerPtr,
    ) -> i32;
    fn xmlNewInputStream(ctxt: xmlParserCtxtPtr) -> xmlParserInputPtr;
    fn inputPush(ctxt: xmlParserCtxtPtr, value: xmlParserInputPtr) -> i32;
    fn xmlCanonicPath(path: *const xmlChar) -> *mut xmlChar;
    fn xmlXIncludeNewContext(doc: xmlDocPtr) -> xmlXIncludeCtxtPtr;
    fn xmlXIncludeSetFlags(ctxt: xmlXIncludeCtxtPtr, flags: i32) -> i32;
    fn xmlXIncludeFreeContext(ctxt: xmlXIncludeCtxtPtr);
    fn xmlXIncludeProcessNode(ctxt: xmlXIncludeCtxtPtr, tree: xmlNodePtr) -> i32;
    fn xmlFreePattern(comp: xmlPatternPtr);
    fn xmlPatterncompile(
        pattern: *const xmlChar,
        dict: *mut xmlDict,
        flags: i32,
        namespaces: *mut *const xmlChar,
    ) -> xmlPatternPtr;
    fn xmlPatternMatch(comp: xmlPatternPtr, node: xmlNodePtr) -> i32;
    fn xmlBufCreateSize(size: size_t) -> xmlBufPtr;
    fn xmlBufSetAllocationScheme(
        buf: xmlBufPtr,
        scheme: xmlBufferAllocationScheme,
    ) -> i32;
    fn xmlBufGetAllocationScheme(buf: xmlBufPtr) -> i32;
    fn xmlBufFree(buf: xmlBufPtr);
    fn xmlBufEmpty(buf: xmlBufPtr);
    fn xmlBufResetInput(buf: xmlBufPtr, input: xmlParserInputPtr) -> i32;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: u32,
    pub fp_offset: u32,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type va_list = __builtin_va_list;
pub type xmlChar = u8;
pub type size_t = u64;
pub type xmlFreeFunc = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type xmlMallocFunc = Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>;
pub type xmlReallocFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
>;
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
pub type xmlParserCtxt = _xmlParserCtxt;
pub type xmlParserCtxtPtr = *mut xmlParserCtxt;
pub type xmlSAXHandler = _xmlSAXHandler;
pub type xmlSAXHandlerPtr = *mut xmlSAXHandler;
pub type xmlBufferAllocationScheme = u32;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
pub const XML_BUFFER_ALLOC_BOUNDED: xmlBufferAllocationScheme = 5;
pub const XML_BUFFER_ALLOC_HYBRID: xmlBufferAllocationScheme = 4;
pub const XML_BUFFER_ALLOC_IO: xmlBufferAllocationScheme = 3;
pub const XML_BUFFER_ALLOC_IMMUTABLE: xmlBufferAllocationScheme = 2;
pub const XML_BUFFER_ALLOC_EXACT: xmlBufferAllocationScheme = 1;
pub const XML_BUFFER_ALLOC_DOUBLEIT: xmlBufferAllocationScheme = 0;
<<<<<<< HEAD
// #[derive(Copy, Clone)]

pub type _xmlBuffer = crate::src::HTMLtree::_xmlBuffer;
pub type xmlBuffer = crate::src::HTMLtree::xmlBuffer;
pub type xmlBufferPtr = crate::src::HTMLtree::xmlBufferPtr;
pub type xmlNsPtr = crate::src::HTMLtree::xmlNsPtr;
pub type xmlDtd = crate::src::HTMLparser::xmlDtd;
pub type xmlDtdPtr = crate::src::HTMLparser::xmlDtdPtr;
pub type xmlGenericErrorFunc = crate::src::HTMLparser::xmlGenericErrorFunc;
pub type xmlValidCtxtPtr = crate::src::SAX2::xmlValidCtxtPtr;
pub type xmlIDTable = crate::src::tree::xmlIDTable;
pub type xmlIDTablePtr = crate::src::tree::xmlIDTablePtr;
pub type xmlRefTable = crate::src::tree::xmlRefTable;
pub type xmlRefTablePtr = crate::src::tree::xmlRefTablePtr;
pub type xmlCharEncoding = crate::src::HTMLparser::xmlCharEncoding;
=======
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlBuffer {
    pub content: *mut xmlChar,
    pub use_0: u32,
    pub size: u32,
    pub alloc: xmlBufferAllocationScheme,
    pub contentIO: *mut xmlChar,
}
pub type xmlBuffer = _xmlBuffer;
pub type xmlBufferPtr = *mut xmlBuffer;
pub type xmlNsPtr = *mut xmlNs;
pub type xmlDtd = _xmlDtd;
pub type xmlDtdPtr = *mut xmlDtd;
pub type xmlGenericErrorFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const i8, ...) -> (),
>;
pub type xmlValidCtxtPtr = *mut xmlValidCtxt;
pub type xmlIDTable = _xmlHashTable;
pub type xmlIDTablePtr = *mut xmlIDTable;
pub type xmlRefTable = _xmlHashTable;
pub type xmlRefTablePtr = *mut xmlRefTable;
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
pub type C2RustUnnamed = u32;
pub const XML_PARSE_BIG_LINES: C2RustUnnamed = 4194304;
pub const XML_PARSE_IGNORE_ENC: C2RustUnnamed = 2097152;
pub const XML_PARSE_OLDSAX: C2RustUnnamed = 1048576;
pub const XML_PARSE_HUGE: C2RustUnnamed = 524288;
pub const XML_PARSE_NOBASEFIX: C2RustUnnamed = 262144;
pub const XML_PARSE_OLD10: C2RustUnnamed = 131072;
pub const XML_PARSE_COMPACT: C2RustUnnamed = 65536;
pub const XML_PARSE_NOXINCNODE: C2RustUnnamed = 32768;
pub const XML_PARSE_NOCDATA: C2RustUnnamed = 16384;
pub const XML_PARSE_NSCLEAN: C2RustUnnamed = 8192;
pub const XML_PARSE_NODICT: C2RustUnnamed = 4096;
pub const XML_PARSE_NONET: C2RustUnnamed = 2048;
pub const XML_PARSE_XINCLUDE: C2RustUnnamed = 1024;
pub const XML_PARSE_SAX1: C2RustUnnamed = 512;
pub const XML_PARSE_NOBLANKS: C2RustUnnamed = 256;
pub const XML_PARSE_PEDANTIC: C2RustUnnamed = 128;
pub const XML_PARSE_NOWARNING: C2RustUnnamed = 64;
pub const XML_PARSE_NOERROR: C2RustUnnamed = 32;
pub const XML_PARSE_DTDVALID: C2RustUnnamed = 16;
pub const XML_PARSE_DTDATTR: C2RustUnnamed = 8;
pub const XML_PARSE_DTDLOAD: C2RustUnnamed = 4;
pub const XML_PARSE_NOENT: C2RustUnnamed = 2;
pub const XML_PARSE_RECOVER: C2RustUnnamed = 1;
<<<<<<< HEAD
pub type xmlDeregisterNodeFunc = crate::src::globals::xmlDeregisterNodeFunc;
pub type xmlRelaxNG = crate::src::debugXML::xmlRelaxNG;
pub type xmlRelaxNGPtr = crate::src::debugXML::xmlRelaxNGPtr;
pub type xmlRelaxNGValidityErrorFunc = crate::src::debugXML::xmlRelaxNGValidityErrorFunc;
pub type xmlRelaxNGValidityWarningFunc = crate::src::debugXML::xmlRelaxNGValidityWarningFunc;
pub type xmlRelaxNGParserCtxt = crate::src::debugXML::xmlRelaxNGParserCtxt;
pub type xmlRelaxNGParserCtxtPtr = crate::src::debugXML::xmlRelaxNGParserCtxtPtr;
pub type xmlRelaxNGValidCtxt = crate::src::debugXML::xmlRelaxNGValidCtxt;
pub type xmlRelaxNGValidCtxtPtr = crate::src::debugXML::xmlRelaxNGValidCtxtPtr;
pub type xmlSchema = crate::src::xmllint::xmlSchema;
pub type xmlSchemaPtr = crate::src::xmllint::xmlSchemaPtr;
pub type xmlSchemaValidityErrorFunc = crate::src::xmllint::xmlSchemaValidityErrorFunc;
pub type xmlSchemaValidityWarningFunc = crate::src::xmllint::xmlSchemaValidityWarningFunc;
pub type xmlSchemaParserCtxt = crate::src::relaxng::xmlSchemaParserCtxt;
pub type xmlSchemaParserCtxtPtr = crate::src::relaxng::xmlSchemaParserCtxtPtr;
pub type xmlSchemaValidCtxt = crate::src::xmllint::xmlSchemaValidCtxt;
pub type xmlSchemaValidCtxtPtr = crate::src::xmllint::xmlSchemaValidCtxtPtr;
=======
pub type xmlDeregisterNodeFunc = Option::<unsafe extern "C" fn(xmlNodePtr) -> ()>;
pub type xmlRelaxNG = _xmlRelaxNG;
pub type xmlRelaxNGPtr = *mut xmlRelaxNG;
pub type xmlRelaxNGValidityErrorFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const i8, ...) -> (),
>;
pub type xmlRelaxNGValidityWarningFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const i8, ...) -> (),
>;
pub type xmlRelaxNGParserCtxt = _xmlRelaxNGParserCtxt;
pub type xmlRelaxNGParserCtxtPtr = *mut xmlRelaxNGParserCtxt;
pub type xmlRelaxNGValidCtxt = _xmlRelaxNGValidCtxt;
pub type xmlRelaxNGValidCtxtPtr = *mut xmlRelaxNGValidCtxt;
pub type xmlSchema = _xmlSchema;
pub type xmlSchemaPtr = *mut xmlSchema;
pub type xmlSchemaValidityErrorFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const i8, ...) -> (),
>;
pub type xmlSchemaValidityWarningFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const i8, ...) -> (),
>;
pub type xmlSchemaParserCtxt = _xmlSchemaParserCtxt;
pub type xmlSchemaParserCtxtPtr = *mut xmlSchemaParserCtxt;
pub type xmlSchemaValidCtxt = _xmlSchemaValidCtxt;
pub type xmlSchemaValidCtxtPtr = *mut xmlSchemaValidCtxt;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
pub type xmlSchemaValidityLocatorFunc = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *mut *const i8,
        *mut u64,
    ) -> i32,
>;
pub type xmlSchemaSAXPlugStruct = _xmlSchemaSAXPlug;
pub type xmlSchemaSAXPlugPtr = *mut xmlSchemaSAXPlugStruct;
pub type xmlParserSeverities = u32;
pub const XML_PARSER_SEVERITY_ERROR: xmlParserSeverities = 4;
pub const XML_PARSER_SEVERITY_WARNING: xmlParserSeverities = 3;
pub const XML_PARSER_SEVERITY_VALIDITY_ERROR: xmlParserSeverities = 2;
pub const XML_PARSER_SEVERITY_VALIDITY_WARNING: xmlParserSeverities = 1;
pub type C2RustUnnamed_0 = u32;
pub const XML_TEXTREADER_MODE_READING: C2RustUnnamed_0 = 5;
pub const XML_TEXTREADER_MODE_CLOSED: C2RustUnnamed_0 = 4;
pub const XML_TEXTREADER_MODE_EOF: C2RustUnnamed_0 = 3;
pub const XML_TEXTREADER_MODE_ERROR: C2RustUnnamed_0 = 2;
pub const XML_TEXTREADER_MODE_INTERACTIVE: C2RustUnnamed_0 = 1;
pub const XML_TEXTREADER_MODE_INITIAL: C2RustUnnamed_0 = 0;
pub type xmlParserProperties = u32;
pub const XML_PARSER_SUBST_ENTITIES: xmlParserProperties = 4;
pub const XML_PARSER_VALIDATE: xmlParserProperties = 3;
pub const XML_PARSER_DEFAULTATTRS: xmlParserProperties = 2;
pub const XML_PARSER_LOADDTD: xmlParserProperties = 1;
pub type C2RustUnnamed_1 = u32;
pub const XML_READER_TYPE_XML_DECLARATION: C2RustUnnamed_1 = 17;
pub const XML_READER_TYPE_END_ENTITY: C2RustUnnamed_1 = 16;
pub const XML_READER_TYPE_END_ELEMENT: C2RustUnnamed_1 = 15;
pub const XML_READER_TYPE_SIGNIFICANT_WHITESPACE: C2RustUnnamed_1 = 14;
pub const XML_READER_TYPE_WHITESPACE: C2RustUnnamed_1 = 13;
pub const XML_READER_TYPE_NOTATION: C2RustUnnamed_1 = 12;
pub const XML_READER_TYPE_DOCUMENT_FRAGMENT: C2RustUnnamed_1 = 11;
pub const XML_READER_TYPE_DOCUMENT_TYPE: C2RustUnnamed_1 = 10;
pub const XML_READER_TYPE_DOCUMENT: C2RustUnnamed_1 = 9;
pub const XML_READER_TYPE_COMMENT: C2RustUnnamed_1 = 8;
pub const XML_READER_TYPE_PROCESSING_INSTRUCTION: C2RustUnnamed_1 = 7;
pub const XML_READER_TYPE_ENTITY: C2RustUnnamed_1 = 6;
pub const XML_READER_TYPE_ENTITY_REFERENCE: C2RustUnnamed_1 = 5;
pub const XML_READER_TYPE_CDATA: C2RustUnnamed_1 = 4;
pub const XML_READER_TYPE_TEXT: C2RustUnnamed_1 = 3;
pub const XML_READER_TYPE_ATTRIBUTE: C2RustUnnamed_1 = 2;
pub const XML_READER_TYPE_ELEMENT: C2RustUnnamed_1 = 1;
pub const XML_READER_TYPE_NONE: C2RustUnnamed_1 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlTextReader {
    pub mode: i32,
    pub doc: xmlDocPtr,
    pub validate: xmlTextReaderValidate,
    pub allocs: i32,
    pub state: xmlTextReaderState,
    pub ctxt: xmlParserCtxtPtr,
    pub sax: xmlSAXHandlerPtr,
    pub input: xmlParserInputBufferPtr,
    pub startElement: startElementSAXFunc,
    pub endElement: endElementSAXFunc,
    pub startElementNs: startElementNsSAX2Func,
    pub endElementNs: endElementNsSAX2Func,
    pub characters: charactersSAXFunc,
    pub cdataBlock: cdataBlockSAXFunc,
    pub base: u32,
    pub cur: u32,
    pub node: xmlNodePtr,
    pub curnode: xmlNodePtr,
    pub depth: i32,
    pub faketext: xmlNodePtr,
    pub preserve: i32,
    pub buffer: xmlBufPtr,
    pub dict: xmlDictPtr,
    pub ent: xmlNodePtr,
    pub entNr: i32,
    pub entMax: i32,
    pub entTab: *mut xmlNodePtr,
    pub errorFunc: xmlTextReaderErrorFunc,
    pub errorFuncArg: *mut libc::c_void,
    pub rngSchemas: xmlRelaxNGPtr,
    pub rngValidCtxt: xmlRelaxNGValidCtxtPtr,
    pub rngPreserveCtxt: i32,
    pub rngValidErrors: i32,
    pub rngFullNode: xmlNodePtr,
    pub xsdSchemas: xmlSchemaPtr,
    pub xsdValidCtxt: xmlSchemaValidCtxtPtr,
    pub xsdPreserveCtxt: i32,
    pub xsdValidErrors: i32,
    pub xsdPlug: xmlSchemaSAXPlugPtr,
    pub xinclude: i32,
    pub xinclude_name: *const xmlChar,
    pub xincctxt: xmlXIncludeCtxtPtr,
    pub in_xinclude: i32,
    pub patternNr: i32,
    pub patternMax: i32,
    pub patternTab: *mut xmlPatternPtr,
    pub preserves: i32,
    pub parserFlags: i32,
    pub sErrorFunc: xmlStructuredErrorFunc,
}
<<<<<<< HEAD
pub type xmlPatternPtr = crate::src::pattern::xmlPatternPtr;
pub type xmlPattern = crate::src::pattern::xmlPattern;
pub type xmlXIncludeCtxtPtr = crate::src::xinclude::xmlXIncludeCtxtPtr;
pub type xmlXIncludeCtxt = crate::src::xinclude::xmlXIncludeCtxt;
=======
pub type xmlPatternPtr = *mut xmlPattern;
pub type xmlPattern = _xmlPattern;
pub type xmlXIncludeCtxtPtr = *mut xmlXIncludeCtxt;
pub type xmlXIncludeCtxt = _xmlXIncludeCtxt;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
pub type xmlTextReaderErrorFunc = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const i8,
        xmlParserSeverities,
        xmlTextReaderLocatorPtr,
    ) -> (),
>;
pub type xmlTextReaderLocatorPtr = *mut libc::c_void;
pub type xmlTextReaderState = i32;
pub const XML_TEXTREADER_ERROR: xmlTextReaderState = 6;
pub const XML_TEXTREADER_DONE: xmlTextReaderState = 5;
pub const XML_TEXTREADER_BACKTRACK: xmlTextReaderState = 4;
pub const XML_TEXTREADER_EMPTY: xmlTextReaderState = 3;
pub const XML_TEXTREADER_END: xmlTextReaderState = 2;
pub const XML_TEXTREADER_ELEMENT: xmlTextReaderState = 1;
pub const XML_TEXTREADER_START: xmlTextReaderState = 0;
pub const XML_TEXTREADER_NONE: xmlTextReaderState = -1;
pub type xmlTextReaderValidate = u32;
pub const XML_TEXTREADER_VALIDATE_XSD: xmlTextReaderValidate = 4;
pub const XML_TEXTREADER_VALIDATE_RNG: xmlTextReaderValidate = 2;
pub const XML_TEXTREADER_VALIDATE_DTD: xmlTextReaderValidate = 1;
pub const XML_TEXTREADER_NOT_VALIDATE: xmlTextReaderValidate = 0;
<<<<<<< HEAD
pub type xmlTextReader = crate::src::xmllint::xmlTextReader;
pub type xmlTextReaderPtr = crate::src::xmllint::xmlTextReaderPtr;
=======
pub type xmlTextReader = _xmlTextReader;
pub type xmlTextReaderPtr = *mut xmlTextReader;
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
unsafe extern "C" fn xmlTextReaderFreeProp(
    mut reader: xmlTextReaderPtr,
    mut cur: xmlAttrPtr,
) {
    let mut dict: xmlDictPtr = 0 as *mut xmlDict;
    if !reader.is_null() && !((*reader).ctxt).is_null() {
        dict = (*(*reader).ctxt).dict;
    } else {
        dict = 0 as xmlDictPtr;
    }
    if cur.is_null() {
        return;
    }
    if __xmlRegisterCallbacks != 0 && (*__xmlDeregisterNodeDefaultValue()).is_some() {
        (*__xmlDeregisterNodeDefaultValue())
            .expect("non-null function pointer")(cur as xmlNodePtr);
    }
    if !((*cur).children).is_null() {
        xmlTextReaderFreeNodeList(reader, (*cur).children);
    }
    if !((*cur).name).is_null()
        && (dict.is_null() || xmlDictOwns(dict, (*cur).name) == 0 as i32)
    {
        xmlFree
            .expect(
                "non-null function pointer",
            )((*cur).name as *mut i8 as *mut libc::c_void);
    }
    if !reader.is_null() && !((*reader).ctxt).is_null()
        && (*(*reader).ctxt).freeAttrsNr < 100 as i32
    {
        let ref mut fresh0 = (*cur).next;
        *fresh0 = (*(*reader).ctxt).freeAttrs;
        let ref mut fresh1 = (*(*reader).ctxt).freeAttrs;
        *fresh1 = cur;
        let ref mut fresh2 = (*(*reader).ctxt).freeAttrsNr;
        *fresh2 += 1;
    } else {
        xmlFree.expect("non-null function pointer")(cur as *mut libc::c_void);
    };
}
unsafe extern "C" fn xmlTextReaderFreePropList(
    mut reader: xmlTextReaderPtr,
    mut cur: xmlAttrPtr,
) {
    let mut next: xmlAttrPtr = 0 as *mut xmlAttr;
    while !cur.is_null() {
        next = (*cur).next;
        xmlTextReaderFreeProp(reader, cur);
        cur = next;
    }
}
unsafe extern "C" fn xmlTextReaderFreeNodeList(
    mut reader: xmlTextReaderPtr,
    mut cur: xmlNodePtr,
) {
    let mut next: xmlNodePtr = 0 as *mut xmlNode;
    let mut parent: xmlNodePtr = 0 as *mut xmlNode;
    let mut dict: xmlDictPtr = 0 as *mut xmlDict;
    let mut depth: size_t = 0 as i32 as size_t;
    if !reader.is_null() && !((*reader).ctxt).is_null() {
        dict = (*(*reader).ctxt).dict;
    } else {
        dict = 0 as xmlDictPtr;
    }
    if cur.is_null() {
        return;
    }
    if (*cur).type_0 as u32 == XML_NAMESPACE_DECL as i32 as u32
    {
        xmlFreeNsList(cur as xmlNsPtr);
        return;
    }
    if (*cur).type_0 as u32 == XML_DOCUMENT_NODE as i32 as u32
        || (*cur).type_0 as u32
            == XML_HTML_DOCUMENT_NODE as i32 as u32
    {
        xmlFreeDoc(cur as xmlDocPtr);
        return;
    }
    loop {
        while (*cur).type_0 as u32
            != XML_DTD_NODE as i32 as u32
            && (*cur).type_0 as u32
                != XML_ENTITY_REF_NODE as i32 as u32
            && !((*cur).children).is_null() && (*(*cur).children).parent == cur
        {
            cur = (*cur).children;
            depth = (depth as u64)
                .wrapping_add(1 as i32 as u64) as size_t as size_t;
        }
        next = (*cur).next;
        parent = (*cur).parent;
        if (*cur).type_0 as u32 != XML_DTD_NODE as i32 as u32 {
            if __xmlRegisterCallbacks != 0
                && (*__xmlDeregisterNodeDefaultValue()).is_some()
            {
                (*__xmlDeregisterNodeDefaultValue())
                    .expect("non-null function pointer")(cur);
            }
            if ((*cur).type_0 as u32
                == XML_ELEMENT_NODE as i32 as u32
                || (*cur).type_0 as u32
                    == XML_XINCLUDE_START as i32 as u32
                || (*cur).type_0 as u32
                    == XML_XINCLUDE_END as i32 as u32)
                && !((*cur).properties).is_null()
            {
                xmlTextReaderFreePropList(reader, (*cur).properties);
            }
            if (*cur).content
                != &mut (*cur).properties as *mut *mut _xmlAttr as *mut xmlChar
                && (*cur).type_0 as u32
                    != XML_ELEMENT_NODE as i32 as u32
                && (*cur).type_0 as u32
                    != XML_XINCLUDE_START as i32 as u32
                && (*cur).type_0 as u32
                    != XML_XINCLUDE_END as i32 as u32
                && (*cur).type_0 as u32
                    != XML_ENTITY_REF_NODE as i32 as u32
            {
                if !((*cur).content).is_null()
                    && (dict.is_null()
                        || xmlDictOwns(dict, (*cur).content as *const xmlChar)
                            == 0 as i32)
                {
                    xmlFree
                        .expect(
                            "non-null function pointer",
                        )((*cur).content as *mut i8 as *mut libc::c_void);
                }
            }
            if ((*cur).type_0 as u32
                == XML_ELEMENT_NODE as i32 as u32
                || (*cur).type_0 as u32
                    == XML_XINCLUDE_START as i32 as u32
                || (*cur).type_0 as u32
                    == XML_XINCLUDE_END as i32 as u32)
                && !((*cur).nsDef).is_null()
            {
                xmlFreeNsList((*cur).nsDef);
            }
            if (*cur).type_0 as u32
                != XML_TEXT_NODE as i32 as u32
                && (*cur).type_0 as u32
                    != XML_COMMENT_NODE as i32 as u32
            {
                if !((*cur).name).is_null()
                    && (dict.is_null()
                        || xmlDictOwns(dict, (*cur).name) == 0 as i32)
                {
                    xmlFree
                        .expect(
                            "non-null function pointer",
                        )((*cur).name as *mut i8 as *mut libc::c_void);
                }
            }
            if ((*cur).type_0 as u32
                == XML_ELEMENT_NODE as i32 as u32
                || (*cur).type_0 as u32
                    == XML_TEXT_NODE as i32 as u32) && !reader.is_null()
                && !((*reader).ctxt).is_null()
                && (*(*reader).ctxt).freeElemsNr < 100 as i32
            {
                let ref mut fresh3 = (*cur).next;
                *fresh3 = (*(*reader).ctxt).freeElems;
                let ref mut fresh4 = (*(*reader).ctxt).freeElems;
                *fresh4 = cur;
                let ref mut fresh5 = (*(*reader).ctxt).freeElemsNr;
                *fresh5 += 1;
            } else {
                xmlFree.expect("non-null function pointer")(cur as *mut libc::c_void);
            }
        }
        if !next.is_null() {
            cur = next;
        } else {
            if depth == 0 as i32 as u64 || parent.is_null() {
                break;
            }
            depth = (depth as u64)
                .wrapping_sub(1 as i32 as u64) as size_t as size_t;
            cur = parent;
            let ref mut fresh6 = (*cur).children;
            *fresh6 = 0 as *mut _xmlNode;
        }
    };
}
unsafe extern "C" fn xmlTextReaderFreeNode(
    mut reader: xmlTextReaderPtr,
    mut cur: xmlNodePtr,
) {
    let mut dict: xmlDictPtr = 0 as *mut xmlDict;
    if !reader.is_null() && !((*reader).ctxt).is_null() {
        dict = (*(*reader).ctxt).dict;
    } else {
        dict = 0 as xmlDictPtr;
    }
    if (*cur).type_0 as u32 == XML_DTD_NODE as i32 as u32 {
        xmlFreeDtd(cur as xmlDtdPtr);
        return;
    }
    if (*cur).type_0 as u32 == XML_NAMESPACE_DECL as i32 as u32
    {
        xmlFreeNs(cur as xmlNsPtr);
        return;
    }
    if (*cur).type_0 as u32 == XML_ATTRIBUTE_NODE as i32 as u32
    {
        xmlTextReaderFreeProp(reader, cur as xmlAttrPtr);
        return;
    }
    if !((*cur).children).is_null()
        && (*cur).type_0 as u32
            != XML_ENTITY_REF_NODE as i32 as u32
    {
        if (*(*cur).children).parent == cur {
            xmlTextReaderFreeNodeList(reader, (*cur).children);
        }
        let ref mut fresh7 = (*cur).children;
        *fresh7 = 0 as *mut _xmlNode;
    }
    if __xmlRegisterCallbacks != 0 && (*__xmlDeregisterNodeDefaultValue()).is_some() {
        (*__xmlDeregisterNodeDefaultValue()).expect("non-null function pointer")(cur);
    }
    if ((*cur).type_0 as u32 == XML_ELEMENT_NODE as i32 as u32
        || (*cur).type_0 as u32
            == XML_XINCLUDE_START as i32 as u32
        || (*cur).type_0 as u32
            == XML_XINCLUDE_END as i32 as u32)
        && !((*cur).properties).is_null()
    {
        xmlTextReaderFreePropList(reader, (*cur).properties);
    }
    if (*cur).content != &mut (*cur).properties as *mut *mut _xmlAttr as *mut xmlChar
        && (*cur).type_0 as u32
            != XML_ELEMENT_NODE as i32 as u32
        && (*cur).type_0 as u32
            != XML_XINCLUDE_START as i32 as u32
        && (*cur).type_0 as u32
            != XML_XINCLUDE_END as i32 as u32
        && (*cur).type_0 as u32
            != XML_ENTITY_REF_NODE as i32 as u32
    {
        if !((*cur).content).is_null()
            && (dict.is_null()
                || xmlDictOwns(dict, (*cur).content as *const xmlChar)
                    == 0 as i32)
        {
            xmlFree
                .expect(
                    "non-null function pointer",
                )((*cur).content as *mut i8 as *mut libc::c_void);
        }
    }
    if ((*cur).type_0 as u32 == XML_ELEMENT_NODE as i32 as u32
        || (*cur).type_0 as u32
            == XML_XINCLUDE_START as i32 as u32
        || (*cur).type_0 as u32
            == XML_XINCLUDE_END as i32 as u32)
        && !((*cur).nsDef).is_null()
    {
        xmlFreeNsList((*cur).nsDef);
    }
    if (*cur).type_0 as u32 != XML_TEXT_NODE as i32 as u32
        && (*cur).type_0 as u32
            != XML_COMMENT_NODE as i32 as u32
    {
        if !((*cur).name).is_null()
            && (dict.is_null() || xmlDictOwns(dict, (*cur).name) == 0 as i32)
        {
            xmlFree
                .expect(
                    "non-null function pointer",
                )((*cur).name as *mut i8 as *mut libc::c_void);
        }
    }
    if ((*cur).type_0 as u32 == XML_ELEMENT_NODE as i32 as u32
        || (*cur).type_0 as u32 == XML_TEXT_NODE as i32 as u32)
        && !reader.is_null() && !((*reader).ctxt).is_null()
        && (*(*reader).ctxt).freeElemsNr < 100 as i32
    {
        let ref mut fresh8 = (*cur).next;
        *fresh8 = (*(*reader).ctxt).freeElems;
        let ref mut fresh9 = (*(*reader).ctxt).freeElems;
        *fresh9 = cur;
        let ref mut fresh10 = (*(*reader).ctxt).freeElemsNr;
        *fresh10 += 1;
    } else {
        xmlFree.expect("non-null function pointer")(cur as *mut libc::c_void);
    };
}
unsafe extern "C" fn xmlTextReaderFreeDoc(
    mut reader: xmlTextReaderPtr,
    mut cur: xmlDocPtr,
) {
    let mut extSubset: xmlDtdPtr = 0 as *mut xmlDtd;
    let mut intSubset: xmlDtdPtr = 0 as *mut xmlDtd;
    if cur.is_null() {
        return;
    }
    if __xmlRegisterCallbacks != 0 && (*__xmlDeregisterNodeDefaultValue()).is_some() {
        (*__xmlDeregisterNodeDefaultValue())
            .expect("non-null function pointer")(cur as xmlNodePtr);
    }
    if !((*cur).ids).is_null() {
        xmlFreeIDTable((*cur).ids as xmlIDTablePtr);
    }
    let ref mut fresh11 = (*cur).ids;
    *fresh11 = 0 as *mut libc::c_void;
    if !((*cur).refs).is_null() {
        xmlFreeRefTable((*cur).refs as xmlRefTablePtr);
    }
    let ref mut fresh12 = (*cur).refs;
    *fresh12 = 0 as *mut libc::c_void;
    extSubset = (*cur).extSubset;
    intSubset = (*cur).intSubset;
    if intSubset == extSubset {
        extSubset = 0 as xmlDtdPtr;
    }
    if !extSubset.is_null() {
        xmlUnlinkNode((*cur).extSubset as xmlNodePtr);
        let ref mut fresh13 = (*cur).extSubset;
        *fresh13 = 0 as *mut _xmlDtd;
        xmlFreeDtd(extSubset);
    }
    if !intSubset.is_null() {
        xmlUnlinkNode((*cur).intSubset as xmlNodePtr);
        let ref mut fresh14 = (*cur).intSubset;
        *fresh14 = 0 as *mut _xmlDtd;
        xmlFreeDtd(intSubset);
    }
    if !((*cur).children).is_null() {
        xmlTextReaderFreeNodeList(reader, (*cur).children);
    }
    if !((*cur).version).is_null() {
        xmlFree
            .expect(
                "non-null function pointer",
            )((*cur).version as *mut i8 as *mut libc::c_void);
    }
    if !((*cur).name).is_null() {
        xmlFree.expect("non-null function pointer")((*cur).name as *mut libc::c_void);
    }
    if !((*cur).encoding).is_null() {
        xmlFree
            .expect(
                "non-null function pointer",
            )((*cur).encoding as *mut i8 as *mut libc::c_void);
    }
    if !((*cur).oldNs).is_null() {
        xmlFreeNsList((*cur).oldNs);
    }
    if !((*cur).URL).is_null() {
        xmlFree
            .expect(
                "non-null function pointer",
            )((*cur).URL as *mut i8 as *mut libc::c_void);
    }
    if !((*cur).dict).is_null() {
        xmlDictFree((*cur).dict);
    }
    xmlFree.expect("non-null function pointer")(cur as *mut libc::c_void);
}
unsafe extern "C" fn xmlTextReaderEntPush(
    mut reader: xmlTextReaderPtr,
    mut value: xmlNodePtr,
) -> i32 {
    if (*reader).entMax <= 0 as i32 {
        (*reader).entMax = 10 as i32;
        let ref mut fresh15 = (*reader).entTab;
        *fresh15 = xmlMalloc
            .expect(
                "non-null function pointer",
            )(
            ((*reader).entMax as u64)
                .wrapping_mul(::std::mem::size_of::<xmlNodePtr>() as u64),
        ) as *mut xmlNodePtr;
        if ((*reader).entTab).is_null() {
            (*__xmlGenericError())
                .expect(
                    "non-null function pointer",
                )(
                *__xmlGenericErrorContext(),
                b"xmlMalloc failed !\n\0" as *const u8 as *const i8,
            );
            return 0 as i32;
        }
    }
    if (*reader).entNr >= (*reader).entMax {
        (*reader).entMax *= 2 as i32;
        let ref mut fresh16 = (*reader).entTab;
        *fresh16 = xmlRealloc
            .expect(
                "non-null function pointer",
            )(
            (*reader).entTab as *mut libc::c_void,
            ((*reader).entMax as u64)
                .wrapping_mul(::std::mem::size_of::<xmlNodePtr>() as u64),
        ) as *mut xmlNodePtr;
        if ((*reader).entTab).is_null() {
            (*__xmlGenericError())
                .expect(
                    "non-null function pointer",
                )(
                *__xmlGenericErrorContext(),
                b"xmlRealloc failed !\n\0" as *const u8 as *const i8,
            );
            return 0 as i32;
        }
    }
    let ref mut fresh17 = *((*reader).entTab).offset((*reader).entNr as isize);
    *fresh17 = value;
    let ref mut fresh18 = (*reader).ent;
    *fresh18 = value;
    let ref mut fresh19 = (*reader).entNr;
    let fresh20 = *fresh19;
    *fresh19 = *fresh19 + 1;
    return fresh20;
}
unsafe extern "C" fn xmlTextReaderEntPop(mut reader: xmlTextReaderPtr) -> xmlNodePtr {
    let mut ret: xmlNodePtr = 0 as *mut xmlNode;
    if (*reader).entNr <= 0 as i32 {
        return 0 as xmlNodePtr;
    }
    let ref mut fresh21 = (*reader).entNr;
    *fresh21 -= 1;
    if (*reader).entNr > 0 as i32 {
        let ref mut fresh22 = (*reader).ent;
        *fresh22 = *((*reader).entTab)
            .offset(((*reader).entNr - 1 as i32) as isize);
    } else {
        let ref mut fresh23 = (*reader).ent;
        *fresh23 = 0 as xmlNodePtr;
    }
    ret = *((*reader).entTab).offset((*reader).entNr as isize);
    let ref mut fresh24 = *((*reader).entTab).offset((*reader).entNr as isize);
    *fresh24 = 0 as xmlNodePtr;
    return ret;
}
unsafe extern "C" fn xmlTextReaderStartElement(
    mut ctx: *mut libc::c_void,
    mut fullname: *const xmlChar,
    mut atts: *mut *const xmlChar,
) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut reader: xmlTextReaderPtr = (*ctxt)._private as xmlTextReaderPtr;
    if !reader.is_null() && ((*reader).startElement).is_some() {
        ((*reader).startElement)
            .expect("non-null function pointer")(ctx, fullname, atts);
        if !((*ctxt).node).is_null() && !((*ctxt).input).is_null()
            && !((*(*ctxt).input).cur).is_null()
            && *((*(*ctxt).input).cur).offset(0 as i32 as isize) as i32
                == '/' as i32
            && *((*(*ctxt).input).cur).offset(1 as i32 as isize) as i32
                == '>' as i32
        {
            (*(*ctxt).node).extra = 0x1 as i32 as u16;
        }
    }
    if !reader.is_null() {
        (*reader).state = XML_TEXTREADER_ELEMENT;
    }
}
unsafe extern "C" fn xmlTextReaderEndElement(
    mut ctx: *mut libc::c_void,
    mut fullname: *const xmlChar,
) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut reader: xmlTextReaderPtr = (*ctxt)._private as xmlTextReaderPtr;
    if !reader.is_null() && ((*reader).endElement).is_some() {
        ((*reader).endElement).expect("non-null function pointer")(ctx, fullname);
    }
}
unsafe extern "C" fn xmlTextReaderStartElementNs(
    mut ctx: *mut libc::c_void,
    mut localname: *const xmlChar,
    mut prefix: *const xmlChar,
    mut URI: *const xmlChar,
    mut nb_namespaces: i32,
    mut namespaces: *mut *const xmlChar,
    mut nb_attributes: i32,
    mut nb_defaulted: i32,
    mut attributes: *mut *const xmlChar,
) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut reader: xmlTextReaderPtr = (*ctxt)._private as xmlTextReaderPtr;
    if !reader.is_null() && ((*reader).startElementNs).is_some() {
        ((*reader).startElementNs)
            .expect(
                "non-null function pointer",
            )(
            ctx,
            localname,
            prefix,
            URI,
            nb_namespaces,
            namespaces,
            nb_attributes,
            nb_defaulted,
            attributes,
        );
        if !((*ctxt).node).is_null() && !((*ctxt).input).is_null()
            && !((*(*ctxt).input).cur).is_null()
            && *((*(*ctxt).input).cur).offset(0 as i32 as isize) as i32
                == '/' as i32
            && *((*(*ctxt).input).cur).offset(1 as i32 as isize) as i32
                == '>' as i32
        {
            (*(*ctxt).node).extra = 0x1 as i32 as u16;
        }
    }
    if !reader.is_null() {
        (*reader).state = XML_TEXTREADER_ELEMENT;
    }
}
unsafe extern "C" fn xmlTextReaderEndElementNs(
    mut ctx: *mut libc::c_void,
    mut localname: *const xmlChar,
    mut prefix: *const xmlChar,
    mut URI: *const xmlChar,
) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut reader: xmlTextReaderPtr = (*ctxt)._private as xmlTextReaderPtr;
    if !reader.is_null() && ((*reader).endElementNs).is_some() {
        ((*reader).endElementNs)
            .expect("non-null function pointer")(ctx, localname, prefix, URI);
    }
}
unsafe extern "C" fn xmlTextReaderCharacters(
    mut ctx: *mut libc::c_void,
    mut ch: *const xmlChar,
    mut len: i32,
) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut reader: xmlTextReaderPtr = (*ctxt)._private as xmlTextReaderPtr;
    if !reader.is_null() && ((*reader).characters).is_some() {
        ((*reader).characters).expect("non-null function pointer")(ctx, ch, len);
    }
}
unsafe extern "C" fn xmlTextReaderCDataBlock(
    mut ctx: *mut libc::c_void,
    mut ch: *const xmlChar,
    mut len: i32,
) {
    let mut ctxt: xmlParserCtxtPtr = ctx as xmlParserCtxtPtr;
    let mut reader: xmlTextReaderPtr = (*ctxt)._private as xmlTextReaderPtr;
    if !reader.is_null() && ((*reader).cdataBlock).is_some() {
        ((*reader).cdataBlock).expect("non-null function pointer")(ctx, ch, len);
    }
}
unsafe extern "C" fn xmlTextReaderPushData(mut reader: xmlTextReaderPtr) -> i32 {
    let mut inbuf: xmlBufPtr = 0 as *mut xmlBuf;
    let mut val: i32 = 0;
    let mut s: i32 = 0;
    let mut oldstate: xmlTextReaderState = XML_TEXTREADER_START;
    let mut alloc: i32 = 0;
    if ((*reader).input).is_null() || ((*(*reader).input).buffer).is_null() {
        return -(1 as i32);
    }
    oldstate = (*reader).state;
    (*reader).state = XML_TEXTREADER_NONE;
    inbuf = (*(*reader).input).buffer;
    alloc = xmlBufGetAllocationScheme(inbuf);
    while (*reader).state as i32 == XML_TEXTREADER_NONE as i32 {
        if xmlBufUse(inbuf)
            < ((*reader).cur).wrapping_add(512 as i32 as u32)
                as u64
        {
            if !((*reader).mode != XML_TEXTREADER_MODE_EOF as i32) {
                break;
            }
            val = xmlParserInputBufferRead((*reader).input, 4096 as i32);
            if val == 0 as i32
                && alloc == XML_BUFFER_ALLOC_IMMUTABLE as i32
            {
                if xmlBufUse(inbuf) == (*reader).cur as u64 {
                    (*reader).mode = XML_TEXTREADER_MODE_EOF as i32;
                    (*reader).state = oldstate;
                }
            } else if val < 0 as i32 {
                (*reader).mode = XML_TEXTREADER_MODE_EOF as i32;
                (*reader).state = oldstate;
                if oldstate as i32 != XML_TEXTREADER_START as i32
                    || !((*(*reader).ctxt).myDoc).is_null()
                {
                    return val;
                }
            } else if val == 0 as i32 {
                (*reader).mode = XML_TEXTREADER_MODE_EOF as i32;
                break;
            }
        }
        if xmlBufUse(inbuf)
            >= ((*reader).cur).wrapping_add(512 as i32 as u32)
                as u64
        {
            val = xmlParseChunk(
                (*reader).ctxt,
                (xmlBufContent(inbuf as *const xmlBuf) as *const i8)
                    .offset((*reader).cur as isize),
                512 as i32,
                0 as i32,
            );
            let ref mut fresh25 = (*reader).cur;
            *fresh25 = (*fresh25).wrapping_add(512 as i32 as u32);
            if val != 0 as i32 {
                (*(*reader).ctxt).wellFormed = 0 as i32;
            }
            if (*(*reader).ctxt).wellFormed == 0 as i32 {
                break;
            }
        } else {
            s = (xmlBufUse(inbuf)).wrapping_sub((*reader).cur as u64)
                as i32;
            val = xmlParseChunk(
                (*reader).ctxt,
                (xmlBufContent(inbuf as *const xmlBuf) as *const i8)
                    .offset((*reader).cur as isize),
                s,
                0 as i32,
            );
            let ref mut fresh26 = (*reader).cur;
            *fresh26 = (*fresh26).wrapping_add(s as u32);
            if val != 0 as i32 {
                (*(*reader).ctxt).wellFormed = 0 as i32;
            }
            break;
        }
    }
    if (*reader).mode == XML_TEXTREADER_MODE_INTERACTIVE as i32 {
        if alloc != XML_BUFFER_ALLOC_IMMUTABLE as i32 {
            if (*reader).cur >= 4096 as i32 as u32
                && (xmlBufUse(inbuf)).wrapping_sub((*reader).cur as u64)
                    <= 512 as i32 as u64
            {
                val = xmlBufShrink(inbuf, (*reader).cur as size_t) as i32;
                if val >= 0 as i32 {
                    let ref mut fresh27 = (*reader).cur;
                    *fresh27 = (*fresh27).wrapping_sub(val as u32);
                }
            }
        }
    } else if (*reader).mode == XML_TEXTREADER_MODE_EOF as i32 {
        if (*reader).state as i32 != XML_TEXTREADER_DONE as i32 {
            s = (xmlBufUse(inbuf)).wrapping_sub((*reader).cur as u64)
                as i32;
            val = xmlParseChunk(
                (*reader).ctxt,
                (xmlBufContent(inbuf as *const xmlBuf) as *const i8)
                    .offset((*reader).cur as isize),
                s,
                1 as i32,
            );
            (*reader).cur = xmlBufUse(inbuf) as u32;
            (*reader).state = XML_TEXTREADER_DONE;
            if val != 0 as i32 {
                if (*(*reader).ctxt).wellFormed != 0 {
                    (*(*reader).ctxt).wellFormed = 0 as i32;
                } else {
                    return -(1 as i32)
                }
            }
        }
    }
    (*reader).state = oldstate;
    if (*(*reader).ctxt).wellFormed == 0 as i32 {
        (*reader).mode = XML_TEXTREADER_MODE_EOF as i32;
        return -(1 as i32);
    }
    return 0 as i32;
}
unsafe extern "C" fn xmlTextReaderValidatePush(mut reader: xmlTextReaderPtr) {
    let mut node: xmlNodePtr = (*reader).node;
    if (*reader).validate as u32
        == XML_TEXTREADER_VALIDATE_DTD as i32 as u32
        && !((*reader).ctxt).is_null() && (*(*reader).ctxt).validate == 1 as i32
    {
        if ((*node).ns).is_null() || ((*(*node).ns).prefix).is_null() {
            (*(*reader).ctxt).valid
                &= xmlValidatePushElement(
                    &mut (*(*reader).ctxt).vctxt,
                    (*(*reader).ctxt).myDoc,
                    node,
                    (*node).name,
                );
        } else {
            let mut qname: *mut xmlChar = 0 as *mut xmlChar;
            qname = xmlStrdup((*(*node).ns).prefix);
            qname = xmlStrcat(
                qname,
                b":\0" as *const u8 as *const i8 as *mut xmlChar,
            );
            qname = xmlStrcat(qname, (*node).name);
            (*(*reader).ctxt).valid
                &= xmlValidatePushElement(
                    &mut (*(*reader).ctxt).vctxt,
                    (*(*reader).ctxt).myDoc,
                    node,
                    qname,
                );
            if !qname.is_null() {
                xmlFree.expect("non-null function pointer")(qname as *mut libc::c_void);
            }
        }
    }
    if (*reader).validate as u32
        == XML_TEXTREADER_VALIDATE_RNG as i32 as u32
        && !((*reader).rngValidCtxt).is_null()
    {
        let mut ret: i32 = 0;
        if !((*reader).rngFullNode).is_null() {
            return;
        }
        ret = xmlRelaxNGValidatePushElement(
            (*reader).rngValidCtxt,
            (*(*reader).ctxt).myDoc,
            node,
        );
        if ret == 0 as i32 {
            node = xmlTextReaderExpand(reader);
            if node.is_null() {
                ret = -(1 as i32);
            } else {
                ret = xmlRelaxNGValidateFullElement(
                    (*reader).rngValidCtxt,
                    (*(*reader).ctxt).myDoc,
                    node,
                );
                let ref mut fresh28 = (*reader).rngFullNode;
                *fresh28 = node;
            }
        }
        if ret != 1 as i32 {
            let ref mut fresh29 = (*reader).rngValidErrors;
            *fresh29 += 1;
        }
    }
}
unsafe extern "C" fn xmlTextReaderValidateCData(
    mut reader: xmlTextReaderPtr,
    mut data: *const xmlChar,
    mut len: i32,
) {
    if (*reader).validate as u32
        == XML_TEXTREADER_VALIDATE_DTD as i32 as u32
        && !((*reader).ctxt).is_null() && (*(*reader).ctxt).validate == 1 as i32
    {
        (*(*reader).ctxt).valid
            &= xmlValidatePushCData(&mut (*(*reader).ctxt).vctxt, data, len);
    }
    if (*reader).validate as u32
        == XML_TEXTREADER_VALIDATE_RNG as i32 as u32
        && !((*reader).rngValidCtxt).is_null()
    {
        let mut ret: i32 = 0;
        if !((*reader).rngFullNode).is_null() {
            return;
        }
        ret = xmlRelaxNGValidatePushCData((*reader).rngValidCtxt, data, len);
        if ret != 1 as i32 {
            let ref mut fresh30 = (*reader).rngValidErrors;
            *fresh30 += 1;
        }
    }
}
unsafe extern "C" fn xmlTextReaderValidatePop(mut reader: xmlTextReaderPtr) {
    let mut node: xmlNodePtr = (*reader).node;
    if (*reader).validate as u32
        == XML_TEXTREADER_VALIDATE_DTD as i32 as u32
        && !((*reader).ctxt).is_null() && (*(*reader).ctxt).validate == 1 as i32
    {
        if ((*node).ns).is_null() || ((*(*node).ns).prefix).is_null() {
            (*(*reader).ctxt).valid
                &= xmlValidatePopElement(
                    &mut (*(*reader).ctxt).vctxt,
                    (*(*reader).ctxt).myDoc,
                    node,
                    (*node).name,
                );
        } else {
            let mut qname: *mut xmlChar = 0 as *mut xmlChar;
            qname = xmlStrdup((*(*node).ns).prefix);
            qname = xmlStrcat(
                qname,
                b":\0" as *const u8 as *const i8 as *mut xmlChar,
            );
            qname = xmlStrcat(qname, (*node).name);
            (*(*reader).ctxt).valid
                &= xmlValidatePopElement(
                    &mut (*(*reader).ctxt).vctxt,
                    (*(*reader).ctxt).myDoc,
                    node,
                    qname,
                );
            if !qname.is_null() {
                xmlFree.expect("non-null function pointer")(qname as *mut libc::c_void);
            }
        }
    }
    if (*reader).validate as u32
        == XML_TEXTREADER_VALIDATE_RNG as i32 as u32
        && !((*reader).rngValidCtxt).is_null()
    {
        let mut ret: i32 = 0;
        if !((*reader).rngFullNode).is_null() {
            if node == (*reader).rngFullNode {
                let ref mut fresh31 = (*reader).rngFullNode;
                *fresh31 = 0 as xmlNodePtr;
            }
            return;
        }
        ret = xmlRelaxNGValidatePopElement(
            (*reader).rngValidCtxt,
            (*(*reader).ctxt).myDoc,
            node,
        );
        if ret != 1 as i32 {
            let ref mut fresh32 = (*reader).rngValidErrors;
            *fresh32 += 1;
        }
    }
}
unsafe extern "C" fn xmlTextReaderValidateEntity(mut reader: xmlTextReaderPtr) {
    let mut oldnode: xmlNodePtr = (*reader).node;
    let mut node: xmlNodePtr = (*reader).node;
    let mut current_block_29: u64;
    loop {
        if (*node).type_0 as u32
            == XML_ENTITY_REF_NODE as i32 as u32
        {
            if !((*node).children).is_null()
                && (*(*node).children).type_0 as u32
                    == XML_ENTITY_DECL as i32 as u32
                && !((*(*node).children).children).is_null()
            {
                xmlTextReaderEntPush(reader, node);
                node = (*(*node).children).children;
                current_block_29 = 12237857397564741460;
            } else {
                if node == oldnode {
                    break;
                }
                current_block_29 = 13226217046118304493;
            }
        } else {
            if (*node).type_0 as u32
                == XML_ELEMENT_NODE as i32 as u32
            {
                let ref mut fresh33 = (*reader).node;
                *fresh33 = node;
                xmlTextReaderValidatePush(reader);
            } else if (*node).type_0 as u32
                    == XML_TEXT_NODE as i32 as u32
                    || (*node).type_0 as u32
                        == XML_CDATA_SECTION_NODE as i32 as u32
                {
                xmlTextReaderValidateCData(
                    reader,
                    (*node).content,
                    xmlStrlen((*node).content),
                );
            }
            if !((*node).children).is_null() {
                node = (*node).children;
                current_block_29 = 12237857397564741460;
            } else {
                if (*node).type_0 as u32
                    == XML_ELEMENT_NODE as i32 as u32
                {
                    xmlTextReaderValidatePop(reader);
                }
                current_block_29 = 13226217046118304493;
            }
        }
        match current_block_29 {
            13226217046118304493 => {
                if !((*node).next).is_null() {
                    node = (*node).next;
                } else {
                    loop {
                        node = (*node).parent;
                        if (*node).type_0 as u32
                            == XML_ELEMENT_NODE as i32 as u32
                        {
                            let mut tmp: xmlNodePtr = 0 as *mut xmlNode;
                            if (*reader).entNr == 0 as i32 {
                                loop {
                                    tmp = (*node).last;
                                    if tmp.is_null() {
                                        break;
                                    }
                                    if !((*tmp).extra as i32 & 0x2 as i32
                                        == 0 as i32)
                                    {
                                        break;
                                    }
                                    xmlUnlinkNode(tmp);
                                    xmlTextReaderFreeNode(reader, tmp);
                                }
                            }
                            let ref mut fresh34 = (*reader).node;
                            *fresh34 = node;
                            xmlTextReaderValidatePop(reader);
                        }
                        if (*node).type_0 as u32
                            == XML_ENTITY_DECL as i32 as u32
                            && !((*reader).ent).is_null()
                            && (*(*reader).ent).children == node
                        {
                            node = xmlTextReaderEntPop(reader);
                        }
                        if node == oldnode {
                            break;
                        }
                        if !((*node).next).is_null() {
                            node = (*node).next;
                            break;
                        } else if !(!node.is_null() && node != oldnode) {
                            break;
                        }
                    }
                }
            }
            _ => {}
        }
        if !(!node.is_null() && node != oldnode) {
            break;
        }
    }
    let ref mut fresh35 = (*reader).node;
    *fresh35 = oldnode;
}
unsafe extern "C" fn xmlTextReaderGetSuccessor(mut cur: xmlNodePtr) -> xmlNodePtr {
    if cur.is_null() {
        return 0 as xmlNodePtr;
    }
    if !((*cur).next).is_null() {
        return (*cur).next;
    }
    loop {
        cur = (*cur).parent;
        if cur.is_null() {
            break;
        }
        if !((*cur).next).is_null() {
            return (*cur).next;
        }
        if cur.is_null() {
            break;
        }
    }
    return cur;
}
unsafe extern "C" fn xmlTextReaderDoExpand(mut reader: xmlTextReaderPtr) -> i32 {
    let mut val: i32 = 0;
    if reader.is_null() || ((*reader).node).is_null() || ((*reader).ctxt).is_null() {
        return -(1 as i32);
    }
    loop {
        if (*(*reader).ctxt).instate as i32 == XML_PARSER_EOF as i32 {
            return 1 as i32;
        }
        if !(xmlTextReaderGetSuccessor((*reader).node)).is_null() {
            return 1 as i32;
        }
        if (*(*reader).ctxt).nodeNr < (*reader).depth {
            return 1 as i32;
        }
        if (*reader).mode == XML_TEXTREADER_MODE_EOF as i32 {
            return 1 as i32;
        }
        val = xmlTextReaderPushData(reader);
        if val < 0 as i32 {
            (*reader).mode = XML_TEXTREADER_MODE_ERROR as i32;
            return -(1 as i32);
        }
        if !((*reader).mode != XML_TEXTREADER_MODE_EOF as i32) {
            break;
        }
    }
    return 1 as i32;
}
unsafe extern "C" fn xmlTextReaderCollectSiblings(mut node: xmlNodePtr) -> *mut xmlChar {
    let mut buffer: xmlBufferPtr = 0 as *mut xmlBuffer;
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    if node.is_null()
        || (*node).type_0 as u32
            == XML_NAMESPACE_DECL as i32 as u32
    {
        return 0 as *mut xmlChar;
    }
    buffer = xmlBufferCreate();
    if buffer.is_null() {
        return 0 as *mut xmlChar;
    }
    xmlBufferSetAllocationScheme(buffer, XML_BUFFER_ALLOC_DOUBLEIT);
    while !node.is_null() {
        match (*node).type_0 as u32 {
            3 | 4 => {
                xmlBufferCat(buffer, (*node).content);
            }
            1 => {
                let mut tmp: *mut xmlChar = 0 as *mut xmlChar;
                tmp = xmlTextReaderCollectSiblings((*node).children);
                xmlBufferCat(buffer, tmp);
                xmlFree.expect("non-null function pointer")(tmp as *mut libc::c_void);
            }
            _ => {}
        }
        node = (*node).next;
    }
    ret = (*buffer).content;
    let ref mut fresh36 = (*buffer).content;
    *fresh36 = 0 as *mut xmlChar;
    xmlBufferFree(buffer);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderRead(mut reader: xmlTextReaderPtr) -> i32 {
    let mut current_block: u64;
    let mut val: i32 = 0;
    let mut olddepth: i32 = 0 as i32;
    let mut oldstate: xmlTextReaderState = XML_TEXTREADER_START;
    let mut oldnode: xmlNodePtr = 0 as xmlNodePtr;
    if reader.is_null() {
        return -(1 as i32);
    }
    let ref mut fresh37 = (*reader).curnode;
    *fresh37 = 0 as xmlNodePtr;
    if !((*reader).doc).is_null() {
        return xmlTextReaderReadTree(reader);
    }
    if ((*reader).ctxt).is_null() {
        return -(1 as i32);
    }
    if (*reader).mode == XML_TEXTREADER_MODE_INITIAL as i32 {
        (*reader).mode = XML_TEXTREADER_MODE_INTERACTIVE as i32;
        loop {
            val = xmlTextReaderPushData(reader);
            if val < 0 as i32 {
                (*reader).mode = XML_TEXTREADER_MODE_ERROR as i32;
                (*reader).state = XML_TEXTREADER_ERROR;
                return -(1 as i32);
            }
            if !(((*(*reader).ctxt).node).is_null()
                && ((*reader).mode != XML_TEXTREADER_MODE_EOF as i32
                    && (*reader).state as i32
                        != XML_TEXTREADER_DONE as i32))
            {
                break;
            }
        }
        if ((*(*reader).ctxt).node).is_null() {
            if !((*(*reader).ctxt).myDoc).is_null() {
                let ref mut fresh38 = (*reader).node;
                *fresh38 = (*(*(*reader).ctxt).myDoc).children;
            }
            if ((*reader).node).is_null() {
                (*reader).mode = XML_TEXTREADER_MODE_ERROR as i32;
                (*reader).state = XML_TEXTREADER_ERROR;
                return -(1 as i32);
            }
            (*reader).state = XML_TEXTREADER_ELEMENT;
        } else {
            if !((*(*reader).ctxt).myDoc).is_null() {
                let ref mut fresh39 = (*reader).node;
                *fresh39 = (*(*(*reader).ctxt).myDoc).children;
            }
            if ((*reader).node).is_null() {
                let ref mut fresh40 = (*reader).node;
                *fresh40 = *((*(*reader).ctxt).nodeTab)
                    .offset(0 as i32 as isize);
            }
            (*reader).state = XML_TEXTREADER_ELEMENT;
        }
        (*reader).depth = 0 as i32;
        (*(*reader).ctxt).parseMode = XML_PARSE_READER;
        current_block = 6684489022775130119;
    } else {
        oldstate = (*reader).state;
        olddepth = (*(*reader).ctxt).nodeNr;
        oldnode = (*reader).node;
        current_block = 11951279088167397802;
    }
    'c_11937: loop {
        match current_block {
            11951279088167397802 => {
                if ((*reader).node).is_null() {
                    if (*reader).mode == XML_TEXTREADER_MODE_EOF as i32 {
                        return 0 as i32
                    } else {
                        return -(1 as i32)
                    }
                }
                while !((*reader).node).is_null() && ((*(*reader).node).next).is_null()
                    && (*(*reader).ctxt).nodeNr == olddepth
                    && (oldstate as i32
                        == XML_TEXTREADER_BACKTRACK as i32
                        || ((*(*reader).node).children).is_null()
                        || (*(*reader).node).type_0 as u32
                            == XML_ENTITY_REF_NODE as i32 as u32
                        || !((*(*reader).node).children).is_null()
                            && (*(*(*reader).node).children).type_0 as u32
                                == XML_TEXT_NODE as i32 as u32
                            && ((*(*(*reader).node).children).next).is_null()
                        || (*(*reader).node).type_0 as u32
                            == XML_DTD_NODE as i32 as u32
                        || (*(*reader).node).type_0 as u32
                            == XML_DOCUMENT_NODE as i32 as u32
                        || (*(*reader).node).type_0 as u32
                            == XML_HTML_DOCUMENT_NODE as i32 as u32)
                    && (((*(*reader).ctxt).node).is_null()
                        || (*(*reader).ctxt).node == (*reader).node
                        || (*(*reader).ctxt).node == (*(*reader).node).parent)
                    && (*(*reader).ctxt).instate as i32
                        != XML_PARSER_EOF as i32
                {
                    val = xmlTextReaderPushData(reader);
                    if val < 0 as i32 {
                        (*reader).mode = XML_TEXTREADER_MODE_ERROR as i32;
                        (*reader).state = XML_TEXTREADER_ERROR;
                        return -(1 as i32);
                    }
                    if ((*reader).node).is_null() {
                        break 'c_11937;
                    }
                }
                if oldstate as i32 != XML_TEXTREADER_BACKTRACK as i32 {
                    if !((*(*reader).node).children).is_null()
                        && (*(*reader).node).type_0 as u32
                            != XML_ENTITY_REF_NODE as i32 as u32
                        && (*(*reader).node).type_0 as u32
                            != XML_XINCLUDE_START as i32 as u32
                        && (*(*reader).node).type_0 as u32
                            != XML_DTD_NODE as i32 as u32
                    {
                        let ref mut fresh41 = (*reader).node;
                        *fresh41 = (*(*reader).node).children;
                        let ref mut fresh42 = (*reader).depth;
                        *fresh42 += 1;
                        (*reader).state = XML_TEXTREADER_ELEMENT;
                        current_block = 6684489022775130119;
                        continue;
                    }
                }
                if !((*(*reader).node).next).is_null() {
                    if oldstate as i32 == XML_TEXTREADER_ELEMENT as i32
                        && (*(*reader).node).type_0 as u32
                            == XML_ELEMENT_NODE as i32 as u32
                        && ((*(*reader).node).children).is_null()
                        && (*(*reader).node).extra as i32 & 0x1 as i32
                            == 0 as i32
                        && (*reader).in_xinclude <= 0 as i32
                    {
                        (*reader).state = XML_TEXTREADER_END;
                        current_block = 6684489022775130119;
                    } else {
                        if (*reader).validate as u32 != 0
                            && (*(*reader).node).type_0 as u32
                                == XML_ELEMENT_NODE as i32 as u32
                        {
                            xmlTextReaderValidatePop(reader);
                        }
                        if (*reader).preserves > 0 as i32
                            && (*(*reader).node).extra as i32
                                & 0x4 as i32 != 0
                        {
                            let ref mut fresh43 = (*reader).preserves;
                            *fresh43 -= 1;
                        }
                        let ref mut fresh44 = (*reader).node;
                        *fresh44 = (*(*reader).node).next;
                        (*reader).state = XML_TEXTREADER_ELEMENT;
                        if (*reader).preserves == 0 as i32
                            && (*reader).in_xinclude == 0 as i32
                            && (*reader).entNr == 0 as i32
                            && !((*(*reader).node).prev).is_null()
                            && (*(*(*reader).node).prev).type_0 as u32
                                != XML_DTD_NODE as i32 as u32
                        {
                            let mut tmp: xmlNodePtr = (*(*reader).node).prev;
                            if (*tmp).extra as i32 & 0x2 as i32
                                == 0 as i32
                            {
                                if oldnode == tmp {
                                    oldnode = 0 as xmlNodePtr;
                                }
                                xmlUnlinkNode(tmp);
                                xmlTextReaderFreeNode(reader, tmp);
                            }
                        }
                        current_block = 6684489022775130119;
                    }
                } else if oldstate as i32
                        == XML_TEXTREADER_ELEMENT as i32
                        && (*(*reader).node).type_0 as u32
                            == XML_ELEMENT_NODE as i32 as u32
                        && ((*(*reader).node).children).is_null()
                        && (*(*reader).node).extra as i32 & 0x1 as i32
                            == 0 as i32
                    {
                    (*reader).state = XML_TEXTREADER_END;
                    current_block = 6684489022775130119;
                } else {
                    if (*reader).validate as u32
                        != XML_TEXTREADER_NOT_VALIDATE as i32 as u32
                        && (*(*reader).node).type_0 as u32
                            == XML_ELEMENT_NODE as i32 as u32
                    {
                        xmlTextReaderValidatePop(reader);
                    }
                    if (*reader).preserves > 0 as i32
                        && (*(*reader).node).extra as i32 & 0x4 as i32
                            != 0
                    {
                        let ref mut fresh45 = (*reader).preserves;
                        *fresh45 -= 1;
                    }
                    let ref mut fresh46 = (*reader).node;
                    *fresh46 = (*(*reader).node).parent;
                    if ((*reader).node).is_null()
                        || (*(*reader).node).type_0 as u32
                            == XML_DOCUMENT_NODE as i32 as u32
                        || (*(*reader).node).type_0 as u32
                            == XML_HTML_DOCUMENT_NODE as i32 as u32
                    {
                        if (*reader).mode != XML_TEXTREADER_MODE_EOF as i32 {
                            val = xmlParseChunk(
                                (*reader).ctxt,
                                b"\0" as *const u8 as *const i8,
                                0 as i32,
                                1 as i32,
                            );
                            (*reader).state = XML_TEXTREADER_DONE;
                            if val != 0 as i32 {
                                return -(1 as i32);
                            }
                        }
                        let ref mut fresh47 = (*reader).node;
                        *fresh47 = 0 as xmlNodePtr;
                        (*reader).depth = -(1 as i32);
                        if !oldnode.is_null() && (*reader).preserves == 0 as i32
                            && (*reader).in_xinclude == 0 as i32
                            && (*reader).entNr == 0 as i32
                            && (*oldnode).type_0 as u32
                                != XML_DTD_NODE as i32 as u32
                            && (*oldnode).extra as i32 & 0x2 as i32
                                == 0 as i32
                        {
                            xmlUnlinkNode(oldnode);
                            xmlTextReaderFreeNode(reader, oldnode);
                        }
                        break;
                    } else {
                        if (*reader).preserves == 0 as i32
                            && (*reader).in_xinclude == 0 as i32
                            && (*reader).entNr == 0 as i32
                            && !((*(*reader).node).last).is_null()
                            && (*(*(*reader).node).last).extra as i32
                                & 0x2 as i32 == 0 as i32
                        {
                            let mut tmp_0: xmlNodePtr = (*(*reader).node).last;
                            xmlUnlinkNode(tmp_0);
                            xmlTextReaderFreeNode(reader, tmp_0);
                        }
                        let ref mut fresh48 = (*reader).depth;
                        *fresh48 -= 1;
                        (*reader).state = XML_TEXTREADER_BACKTRACK;
                        current_block = 6684489022775130119;
                    }
                }
            }
            _ => {
                if !((*reader).node).is_null() && ((*(*reader).node).next).is_null()
                    && ((*(*reader).node).type_0 as u32
                        == XML_TEXT_NODE as i32 as u32
                        || (*(*reader).node).type_0 as u32
                            == XML_CDATA_SECTION_NODE as i32 as u32)
                {
                    if (xmlTextReaderExpand(reader)).is_null() {
                        return -(1 as i32);
                    }
                }
                if (*reader).xinclude != 0 && (*reader).in_xinclude == 0 as i32
                    && !((*reader).node).is_null()
                    && (*(*reader).node).type_0 as u32
                        == XML_ELEMENT_NODE as i32 as u32
                    && !((*(*reader).node).ns).is_null()
                    && (xmlStrEqual(
                        (*(*(*reader).node).ns).href,
                        b"http://www.w3.org/2003/XInclude\0" as *const u8
                            as *const i8 as *const xmlChar,
                    ) != 0
                        || xmlStrEqual(
                            (*(*(*reader).node).ns).href,
                            b"http://www.w3.org/2001/XInclude\0" as *const u8
                                as *const i8 as *const xmlChar,
                        ) != 0)
                {
                    if ((*reader).xincctxt).is_null() {
                        let ref mut fresh49 = (*reader).xincctxt;
                        *fresh49 = xmlXIncludeNewContext((*(*reader).ctxt).myDoc);
                        xmlXIncludeSetFlags(
                            (*reader).xincctxt,
                            (*reader).parserFlags
                                & !(XML_PARSE_NOXINCNODE as i32),
                        );
                    }
                    if (xmlTextReaderExpand(reader)).is_null() {
                        return -(1 as i32);
                    }
                    xmlXIncludeProcessNode((*reader).xincctxt, (*reader).node);
                }
                if !((*reader).node).is_null()
                    && (*(*reader).node).type_0 as u32
                        == XML_XINCLUDE_START as i32 as u32
                {
                    let ref mut fresh50 = (*reader).in_xinclude;
                    *fresh50 += 1;
                    current_block = 11951279088167397802;
                } else if !((*reader).node).is_null()
                        && (*(*reader).node).type_0 as u32
                            == XML_XINCLUDE_END as i32 as u32
                    {
                    let ref mut fresh51 = (*reader).in_xinclude;
                    *fresh51 -= 1;
                    current_block = 11951279088167397802;
                } else {
                    if !((*reader).node).is_null()
                        && (*(*reader).node).type_0 as u32
                            == XML_ENTITY_REF_NODE as i32 as u32
                        && !((*reader).ctxt).is_null()
                        && (*(*reader).ctxt).replaceEntities == 1 as i32
                    {
                        if !((*(*reader).node).children).is_null()
                            && (*(*(*reader).node).children).type_0 as u32
                                == XML_ENTITY_DECL as i32 as u32
                            && !((*(*(*reader).node).children).children).is_null()
                        {
                            xmlTextReaderEntPush(reader, (*reader).node);
                            let ref mut fresh52 = (*reader).node;
                            *fresh52 = (*(*(*reader).node).children).children;
                        }
                    } else if !((*reader).node).is_null()
                            && (*(*reader).node).type_0 as u32
                                == XML_ENTITY_REF_NODE as i32 as u32
                            && !((*reader).ctxt).is_null()
                            && (*reader).validate as u32 != 0
                        {
                        xmlTextReaderValidateEntity(reader);
                    }
                    if !((*reader).node).is_null()
                        && (*(*reader).node).type_0 as u32
                            == XML_ENTITY_DECL as i32 as u32
                        && !((*reader).ent).is_null()
                        && (*(*reader).ent).children == (*reader).node
                    {
                        let ref mut fresh53 = (*reader).node;
                        *fresh53 = xmlTextReaderEntPop(reader);
                        let ref mut fresh54 = (*reader).depth;
                        *fresh54 += 1;
                        current_block = 11951279088167397802;
                    } else {
                        if (*reader).validate as u32
                            != XML_TEXTREADER_NOT_VALIDATE as i32 as u32
                            && !((*reader).node).is_null()
                        {
                            let mut node: xmlNodePtr = (*reader).node;
                            if (*node).type_0 as u32
                                == XML_ELEMENT_NODE as i32 as u32
                                && ((*reader).state as i32
                                    != XML_TEXTREADER_END as i32
                                    && (*reader).state as i32
                                        != XML_TEXTREADER_BACKTRACK as i32)
                            {
                                xmlTextReaderValidatePush(reader);
                            } else if (*node).type_0 as u32
                                    == XML_TEXT_NODE as i32 as u32
                                    || (*node).type_0 as u32
                                        == XML_CDATA_SECTION_NODE as i32 as u32
                                {
                                xmlTextReaderValidateCData(
                                    reader,
                                    (*node).content,
                                    xmlStrlen((*node).content),
                                );
                            }
                        }
                        if (*reader).patternNr > 0 as i32
                            && (*reader).state as i32
                                != XML_TEXTREADER_END as i32
                            && (*reader).state as i32
                                != XML_TEXTREADER_BACKTRACK as i32
                        {
                            let mut i: i32 = 0;
                            i = 0 as i32;
                            while i < (*reader).patternNr {
                                if xmlPatternMatch(
                                    *((*reader).patternTab).offset(i as isize),
                                    (*reader).node,
                                ) == 1 as i32
                                {
                                    xmlTextReaderPreserve(reader);
                                    break;
                                } else {
                                    i += 1;
                                }
                            }
                        }
                        if (*reader).validate as u32
                            == XML_TEXTREADER_VALIDATE_XSD as i32 as u32
                            && (*reader).xsdValidErrors == 0 as i32
                            && !((*reader).xsdValidCtxt).is_null()
                        {
                            (*reader)
                                .xsdValidErrors = (xmlSchemaIsValid((*reader).xsdValidCtxt)
                                == 0) as i32;
                        }
                        return 1 as i32;
                    }
                }
            }
        }
    }
    (*reader).state = XML_TEXTREADER_DONE;
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderReadState(
    mut reader: xmlTextReaderPtr,
) -> i32 {
    if reader.is_null() {
        return -(1 as i32);
    }
    return (*reader).mode;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderExpand(
    mut reader: xmlTextReaderPtr,
) -> xmlNodePtr {
    if reader.is_null() || ((*reader).node).is_null() {
        return 0 as xmlNodePtr;
    }
    if !((*reader).doc).is_null() {
        return (*reader).node;
    }
    if ((*reader).ctxt).is_null() {
        return 0 as xmlNodePtr;
    }
    if xmlTextReaderDoExpand(reader) < 0 as i32 {
        return 0 as xmlNodePtr;
    }
    return (*reader).node;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderNext(mut reader: xmlTextReaderPtr) -> i32 {
    let mut ret: i32 = 0;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    if reader.is_null() {
        return -(1 as i32);
    }
    if !((*reader).doc).is_null() {
        return xmlTextReaderNextTree(reader);
    }
    cur = (*reader).node;
    if cur.is_null()
        || (*cur).type_0 as u32
            != XML_ELEMENT_NODE as i32 as u32
    {
        return xmlTextReaderRead(reader);
    }
    if (*reader).state as i32 == XML_TEXTREADER_END as i32
        || (*reader).state as i32 == XML_TEXTREADER_BACKTRACK as i32
    {
        return xmlTextReaderRead(reader);
    }
    if (*cur).extra as i32 & 0x1 as i32 != 0 {
        return xmlTextReaderRead(reader);
    }
    loop {
        ret = xmlTextReaderRead(reader);
        if ret != 1 as i32 {
            return ret;
        }
        if !((*reader).node != cur) {
            break;
        }
    }
    return xmlTextReaderRead(reader);
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderReadInnerXml(
    mut reader: xmlTextReaderPtr,
) -> *mut xmlChar {
    let mut resbuf: *mut xmlChar = 0 as *mut xmlChar;
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    let mut cur_node: xmlNodePtr = 0 as *mut xmlNode;
    let mut buff: xmlBufferPtr = 0 as *mut xmlBuffer;
    let mut buff2: xmlBufferPtr = 0 as *mut xmlBuffer;
    let mut doc: xmlDocPtr = 0 as *mut xmlDoc;
    if (xmlTextReaderExpand(reader)).is_null() {
        return 0 as *mut xmlChar;
    }
    doc = (*(*reader).node).doc;
    buff = xmlBufferCreate();
    if buff.is_null() {
        return 0 as *mut xmlChar;
    }
    xmlBufferSetAllocationScheme(buff, XML_BUFFER_ALLOC_DOUBLEIT);
    cur_node = (*(*reader).node).children;
    while !cur_node.is_null() {
        node = xmlDocCopyNode(cur_node, doc, 1 as i32);
        buff2 = xmlBufferCreate();
        xmlBufferSetAllocationScheme(buff2, XML_BUFFER_ALLOC_DOUBLEIT);
        if xmlNodeDump(buff2, doc, node, 0 as i32, 0 as i32)
            == -(1 as i32)
        {
            xmlFreeNode(node);
            xmlBufferFree(buff2);
            xmlBufferFree(buff);
            return 0 as *mut xmlChar;
        }
        xmlBufferCat(buff, (*buff2).content);
        xmlFreeNode(node);
        xmlBufferFree(buff2);
        cur_node = (*cur_node).next;
    }
    resbuf = (*buff).content;
    let ref mut fresh55 = (*buff).content;
    *fresh55 = 0 as *mut xmlChar;
    xmlBufferFree(buff);
    return resbuf;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderReadOuterXml(
    mut reader: xmlTextReaderPtr,
) -> *mut xmlChar {
    let mut resbuf: *mut xmlChar = 0 as *mut xmlChar;
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    let mut buff: xmlBufferPtr = 0 as *mut xmlBuffer;
    let mut doc: xmlDocPtr = 0 as *mut xmlDoc;
    if (xmlTextReaderExpand(reader)).is_null() {
        return 0 as *mut xmlChar;
    }
    node = (*reader).node;
    doc = (*node).doc;
    if (*node).type_0 as u32 == XML_DTD_NODE as i32 as u32 {
        node = xmlCopyDtd(node as xmlDtdPtr) as xmlNodePtr;
    } else {
        node = xmlDocCopyNode(node, doc, 1 as i32);
    }
    buff = xmlBufferCreate();
    xmlBufferSetAllocationScheme(buff, XML_BUFFER_ALLOC_DOUBLEIT);
    if xmlNodeDump(buff, doc, node, 0 as i32, 0 as i32)
        == -(1 as i32)
    {
        xmlFreeNode(node);
        xmlBufferFree(buff);
        return 0 as *mut xmlChar;
    }
    resbuf = (*buff).content;
    let ref mut fresh56 = (*buff).content;
    *fresh56 = 0 as *mut xmlChar;
    xmlFreeNode(node);
    xmlBufferFree(buff);
    return resbuf;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderReadString(
    mut reader: xmlTextReaderPtr,
) -> *mut xmlChar {
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    if reader.is_null() || ((*reader).node).is_null() {
        return 0 as *mut xmlChar;
    }
    node = if !((*reader).curnode).is_null() {
        (*reader).curnode
    } else {
        (*reader).node
    };
    match (*node).type_0 as u32 {
        3 => {
            if !((*node).content).is_null() {
                return xmlStrdup((*node).content);
            }
        }
        1 => {
            if xmlTextReaderDoExpand(reader) != -(1 as i32) {
                return xmlTextReaderCollectSiblings((*node).children);
            }
        }
        2 => {
            (*__xmlGenericError())
                .expect(
                    "non-null function pointer",
                )(
                *__xmlGenericErrorContext(),
                b"Unimplemented block at %s:%d\n\0" as *const u8 as *const i8,
                b"xmlreader.c\0" as *const u8 as *const i8,
                1740 as i32,
            );
        }
        _ => {}
    }
    return 0 as *mut xmlChar;
}
unsafe extern "C" fn xmlTextReaderNextTree(mut reader: xmlTextReaderPtr) -> i32 {
    if reader.is_null() {
        return -(1 as i32);
    }
    if (*reader).state as i32 == XML_TEXTREADER_END as i32 {
        return 0 as i32;
    }
    if ((*reader).node).is_null() {
        if ((*(*reader).doc).children).is_null() {
            (*reader).state = XML_TEXTREADER_END;
            return 0 as i32;
        }
        let ref mut fresh57 = (*reader).node;
        *fresh57 = (*(*reader).doc).children;
        (*reader).state = XML_TEXTREADER_START;
        return 1 as i32;
    }
    if (*reader).state as i32 != XML_TEXTREADER_BACKTRACK as i32 {
        if !((*(*reader).node).next).is_null() {
            let ref mut fresh58 = (*reader).node;
            *fresh58 = (*(*reader).node).next;
            (*reader).state = XML_TEXTREADER_START;
            return 1 as i32;
        }
        (*reader).state = XML_TEXTREADER_BACKTRACK;
        xmlTextReaderRead(reader);
    }
    if !((*(*reader).node).next).is_null() {
        let ref mut fresh59 = (*reader).node;
        *fresh59 = (*(*reader).node).next;
        (*reader).state = XML_TEXTREADER_START;
        return 1 as i32;
    }
    if !((*(*reader).node).parent).is_null() {
        if (*(*(*reader).node).parent).type_0 as u32
            == XML_DOCUMENT_NODE as i32 as u32
        {
            (*reader).state = XML_TEXTREADER_END;
            return 0 as i32;
        }
        let ref mut fresh60 = (*reader).node;
        *fresh60 = (*(*reader).node).parent;
        let ref mut fresh61 = (*reader).depth;
        *fresh61 -= 1;
        (*reader).state = XML_TEXTREADER_BACKTRACK;
        xmlTextReaderNextTree(reader);
    }
    (*reader).state = XML_TEXTREADER_END;
    return 1 as i32;
}
unsafe extern "C" fn xmlTextReaderReadTree(mut reader: xmlTextReaderPtr) -> i32 {
    let mut current_block: u64;
    if (*reader).state as i32 == XML_TEXTREADER_END as i32 {
        return 0 as i32;
    }
    loop {
        if ((*reader).node).is_null() {
            if ((*(*reader).doc).children).is_null() {
                (*reader).state = XML_TEXTREADER_END;
                return 0 as i32;
            }
            let ref mut fresh62 = (*reader).node;
            *fresh62 = (*(*reader).doc).children;
            (*reader).state = XML_TEXTREADER_START;
        } else {
            if (*reader).state as i32 != XML_TEXTREADER_BACKTRACK as i32
                && (*(*reader).node).type_0 as u32
                    != XML_DTD_NODE as i32 as u32
                && (*(*reader).node).type_0 as u32
                    != XML_XINCLUDE_START as i32 as u32
                && (*(*reader).node).type_0 as u32
                    != XML_ENTITY_REF_NODE as i32 as u32
            {
                if !((*(*reader).node).children).is_null() {
                    let ref mut fresh63 = (*reader).node;
                    *fresh63 = (*(*reader).node).children;
                    let ref mut fresh64 = (*reader).depth;
                    *fresh64 += 1;
                    (*reader).state = XML_TEXTREADER_START;
                    current_block = 4103914342875670315;
                } else if (*(*reader).node).type_0 as u32
                        == XML_ATTRIBUTE_NODE as i32 as u32
                    {
                    (*reader).state = XML_TEXTREADER_BACKTRACK;
                    current_block = 4103914342875670315;
                } else {
                    current_block = 5143058163439228106;
                }
            } else {
                current_block = 5143058163439228106;
            }
            match current_block {
                4103914342875670315 => {}
                _ => {
                    if !((*(*reader).node).next).is_null() {
                        let ref mut fresh65 = (*reader).node;
                        *fresh65 = (*(*reader).node).next;
                        (*reader).state = XML_TEXTREADER_START;
                    } else if !((*(*reader).node).parent).is_null() {
                        if (*(*(*reader).node).parent).type_0 as u32
                            == XML_DOCUMENT_NODE as i32 as u32
                            || (*(*(*reader).node).parent).type_0 as u32
                                == XML_HTML_DOCUMENT_NODE as i32 as u32
                        {
                            (*reader).state = XML_TEXTREADER_END;
                            return 0 as i32;
                        }
                        let ref mut fresh66 = (*reader).node;
                        *fresh66 = (*(*reader).node).parent;
                        let ref mut fresh67 = (*reader).depth;
                        *fresh67 -= 1;
                        (*reader).state = XML_TEXTREADER_BACKTRACK;
                    } else {
                        (*reader).state = XML_TEXTREADER_END;
                    }
                }
            }
        }
        if !((*(*reader).node).type_0 as u32
            == XML_XINCLUDE_START as i32 as u32
            || (*(*reader).node).type_0 as u32
                == XML_XINCLUDE_END as i32 as u32)
        {
            break;
        }
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderNextSibling(
    mut reader: xmlTextReaderPtr,
) -> i32 {
    if reader.is_null() {
        return -(1 as i32);
    }
    if ((*reader).doc).is_null() {
        return -(1 as i32);
    }
    if (*reader).state as i32 == XML_TEXTREADER_END as i32 {
        return 0 as i32;
    }
    if ((*reader).node).is_null() {
        return xmlTextReaderNextTree(reader);
    }
    if !((*(*reader).node).next).is_null() {
        let ref mut fresh68 = (*reader).node;
        *fresh68 = (*(*reader).node).next;
        (*reader).state = XML_TEXTREADER_START;
        return 1 as i32;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn xmlNewTextReader(
    mut input: xmlParserInputBufferPtr,
    mut URI: *const i8,
) -> xmlTextReaderPtr {
    let mut ret: xmlTextReaderPtr = 0 as *mut xmlTextReader;
    if input.is_null() {
        return 0 as xmlTextReaderPtr;
    }
    ret = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::std::mem::size_of::<xmlTextReader>() as u64) as xmlTextReaderPtr;
    if ret.is_null() {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"xmlNewTextReader : malloc failed\n\0" as *const u8 as *const i8,
        );
        return 0 as xmlTextReaderPtr;
    }
    memset(
        ret as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<xmlTextReader>() as u64,
    );
    let ref mut fresh69 = (*ret).doc;
    *fresh69 = 0 as xmlDocPtr;
    let ref mut fresh70 = (*ret).entTab;
    *fresh70 = 0 as *mut xmlNodePtr;
    (*ret).entMax = 0 as i32;
    (*ret).entNr = 0 as i32;
    let ref mut fresh71 = (*ret).input;
    *fresh71 = input;
    let ref mut fresh72 = (*ret).buffer;
    *fresh72 = xmlBufCreateSize(100 as i32 as size_t);
    if ((*ret).buffer).is_null() {
        xmlFree.expect("non-null function pointer")(ret as *mut libc::c_void);
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"xmlNewTextReader : malloc failed\n\0" as *const u8 as *const i8,
        );
        return 0 as xmlTextReaderPtr;
    }
    xmlBufSetAllocationScheme((*ret).buffer, XML_BUFFER_ALLOC_DOUBLEIT);
    let ref mut fresh73 = (*ret).sax;
    *fresh73 = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::std::mem::size_of::<xmlSAXHandler>() as u64) as *mut xmlSAXHandler;
    if ((*ret).sax).is_null() {
        xmlBufFree((*ret).buffer);
        xmlFree.expect("non-null function pointer")(ret as *mut libc::c_void);
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"xmlNewTextReader : malloc failed\n\0" as *const u8 as *const i8,
        );
        return 0 as xmlTextReaderPtr;
    }
    xmlSAXVersion((*ret).sax, 2 as i32);
    let ref mut fresh74 = (*ret).startElement;
    *fresh74 = (*(*ret).sax).startElement;
    let ref mut fresh75 = (*(*ret).sax).startElement;
    *fresh75 = Some(
        xmlTextReaderStartElement
            as unsafe extern "C" fn(
                *mut libc::c_void,
                *const xmlChar,
                *mut *const xmlChar,
            ) -> (),
    );
    let ref mut fresh76 = (*ret).endElement;
    *fresh76 = (*(*ret).sax).endElement;
    let ref mut fresh77 = (*(*ret).sax).endElement;
    *fresh77 = Some(
        xmlTextReaderEndElement
            as unsafe extern "C" fn(*mut libc::c_void, *const xmlChar) -> (),
    );
    if (*(*ret).sax).initialized == 0xdeedbeaf as u32 {
        let ref mut fresh78 = (*ret).startElementNs;
        *fresh78 = (*(*ret).sax).startElementNs;
        let ref mut fresh79 = (*(*ret).sax).startElementNs;
        *fresh79 = Some(
            xmlTextReaderStartElementNs
                as unsafe extern "C" fn(
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
        );
        let ref mut fresh80 = (*ret).endElementNs;
        *fresh80 = (*(*ret).sax).endElementNs;
        let ref mut fresh81 = (*(*ret).sax).endElementNs;
        *fresh81 = Some(
            xmlTextReaderEndElementNs
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const xmlChar,
                    *const xmlChar,
                    *const xmlChar,
                ) -> (),
        );
    } else {
        let ref mut fresh82 = (*ret).startElementNs;
        *fresh82 = None;
        let ref mut fresh83 = (*ret).endElementNs;
        *fresh83 = None;
    }
    let ref mut fresh84 = (*ret).characters;
    *fresh84 = (*(*ret).sax).characters;
    let ref mut fresh85 = (*(*ret).sax).characters;
    *fresh85 = Some(
        xmlTextReaderCharacters
            as unsafe extern "C" fn(*mut libc::c_void, *const xmlChar, i32) -> (),
    );
    let ref mut fresh86 = (*(*ret).sax).ignorableWhitespace;
    *fresh86 = Some(
        xmlTextReaderCharacters
            as unsafe extern "C" fn(*mut libc::c_void, *const xmlChar, i32) -> (),
    );
    let ref mut fresh87 = (*ret).cdataBlock;
    *fresh87 = (*(*ret).sax).cdataBlock;
    let ref mut fresh88 = (*(*ret).sax).cdataBlock;
    *fresh88 = Some(
        xmlTextReaderCDataBlock
            as unsafe extern "C" fn(*mut libc::c_void, *const xmlChar, i32) -> (),
    );
    (*ret).mode = XML_TEXTREADER_MODE_INITIAL as i32;
    let ref mut fresh89 = (*ret).node;
    *fresh89 = 0 as xmlNodePtr;
    let ref mut fresh90 = (*ret).curnode;
    *fresh90 = 0 as xmlNodePtr;
    if xmlBufUse((*(*ret).input).buffer) < 4 as i32 as u64 {
        xmlParserInputBufferRead(input, 4 as i32);
    }
    if xmlBufUse((*(*ret).input).buffer) >= 4 as i32 as u64 {
        let ref mut fresh91 = (*ret).ctxt;
        *fresh91 = xmlCreatePushParserCtxt(
            (*ret).sax,
            0 as *mut libc::c_void,
            xmlBufContent((*(*ret).input).buffer as *const xmlBuf)
                as *const i8,
            4 as i32,
            URI,
        );
        (*ret).base = 0 as i32 as u32;
        (*ret).cur = 4 as i32 as u32;
    } else {
        let ref mut fresh92 = (*ret).ctxt;
        *fresh92 = xmlCreatePushParserCtxt(
            (*ret).sax,
            0 as *mut libc::c_void,
            0 as *const i8,
            0 as i32,
            URI,
        );
        (*ret).base = 0 as i32 as u32;
        (*ret).cur = 0 as i32 as u32;
    }
    if ((*ret).ctxt).is_null() {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"xmlNewTextReader : malloc failed\n\0" as *const u8 as *const i8,
        );
        xmlBufFree((*ret).buffer);
        xmlFree.expect("non-null function pointer")((*ret).sax as *mut libc::c_void);
        xmlFree.expect("non-null function pointer")(ret as *mut libc::c_void);
        return 0 as xmlTextReaderPtr;
    }
    (*(*ret).ctxt).parseMode = XML_PARSE_READER;
    let ref mut fresh93 = (*(*ret).ctxt)._private;
    *fresh93 = ret as *mut libc::c_void;
    (*(*ret).ctxt).linenumbers = 1 as i32;
    (*(*ret).ctxt).dictNames = 1 as i32;
    (*ret).allocs = 2 as i32;
    (*(*ret).ctxt).docdict = 1 as i32;
    let ref mut fresh94 = (*ret).dict;
    *fresh94 = (*(*ret).ctxt).dict;
    (*ret).xinclude = 0 as i32;
    (*ret).patternMax = 0 as i32;
    let ref mut fresh95 = (*ret).patternTab;
    *fresh95 = 0 as *mut xmlPatternPtr;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlNewTextReaderFilename(
    mut URI: *const i8,
) -> xmlTextReaderPtr {
    let mut input: xmlParserInputBufferPtr = 0 as *mut xmlParserInputBuffer;
    let mut ret: xmlTextReaderPtr = 0 as *mut xmlTextReader;
    let mut directory: *mut i8 = 0 as *mut i8;
    input = xmlParserInputBufferCreateFilename(URI, XML_CHAR_ENCODING_NONE);
    if input.is_null() {
        return 0 as xmlTextReaderPtr;
    }
    ret = xmlNewTextReader(input, URI);
    if ret.is_null() {
        xmlFreeParserInputBuffer(input);
        return 0 as xmlTextReaderPtr;
    }
    (*ret).allocs |= 1 as i32;
    if ((*(*ret).ctxt).directory).is_null() {
        directory = xmlParserGetDirectory(URI);
    }
    if ((*(*ret).ctxt).directory).is_null() && !directory.is_null() {
        let ref mut fresh96 = (*(*ret).ctxt).directory;
        *fresh96 = xmlStrdup(directory as *mut xmlChar) as *mut i8;
    }
    if !directory.is_null() {
        xmlFree.expect("non-null function pointer")(directory as *mut libc::c_void);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlFreeTextReader(mut reader: xmlTextReaderPtr) {
    if reader.is_null() {
        return;
    }
    if !((*reader).rngSchemas).is_null() {
        xmlRelaxNGFree((*reader).rngSchemas);
        let ref mut fresh97 = (*reader).rngSchemas;
        *fresh97 = 0 as xmlRelaxNGPtr;
    }
    if !((*reader).rngValidCtxt).is_null() {
        if (*reader).rngPreserveCtxt == 0 {
            xmlRelaxNGFreeValidCtxt((*reader).rngValidCtxt);
        }
        let ref mut fresh98 = (*reader).rngValidCtxt;
        *fresh98 = 0 as xmlRelaxNGValidCtxtPtr;
    }
    if !((*reader).xsdPlug).is_null() {
        xmlSchemaSAXUnplug((*reader).xsdPlug);
        let ref mut fresh99 = (*reader).xsdPlug;
        *fresh99 = 0 as xmlSchemaSAXPlugPtr;
    }
    if !((*reader).xsdValidCtxt).is_null() {
        if (*reader).xsdPreserveCtxt == 0 {
            xmlSchemaFreeValidCtxt((*reader).xsdValidCtxt);
        }
        let ref mut fresh100 = (*reader).xsdValidCtxt;
        *fresh100 = 0 as xmlSchemaValidCtxtPtr;
    }
    if !((*reader).xsdSchemas).is_null() {
        xmlSchemaFree((*reader).xsdSchemas);
        let ref mut fresh101 = (*reader).xsdSchemas;
        *fresh101 = 0 as xmlSchemaPtr;
    }
    if !((*reader).xincctxt).is_null() {
        xmlXIncludeFreeContext((*reader).xincctxt);
    }
    if !((*reader).patternTab).is_null() {
        let mut i: i32 = 0;
        i = 0 as i32;
        while i < (*reader).patternNr {
            if !(*((*reader).patternTab).offset(i as isize)).is_null() {
                xmlFreePattern(*((*reader).patternTab).offset(i as isize));
            }
            i += 1;
        }
        xmlFree
            .expect(
                "non-null function pointer",
            )((*reader).patternTab as *mut libc::c_void);
    }
    if (*reader).mode != XML_TEXTREADER_MODE_CLOSED as i32 {
        xmlTextReaderClose(reader);
    }
    if !((*reader).ctxt).is_null() {
        if (*reader).dict == (*(*reader).ctxt).dict {
            let ref mut fresh102 = (*reader).dict;
            *fresh102 = 0 as xmlDictPtr;
        }
        if (*reader).allocs & 2 as i32 != 0 {
            xmlFreeParserCtxt((*reader).ctxt);
        }
    }
    if !((*reader).sax).is_null() {
        xmlFree.expect("non-null function pointer")((*reader).sax as *mut libc::c_void);
    }
    if !((*reader).buffer).is_null() {
        xmlBufFree((*reader).buffer);
    }
    if !((*reader).entTab).is_null() {
        xmlFree
            .expect("non-null function pointer")((*reader).entTab as *mut libc::c_void);
    }
    if !((*reader).dict).is_null() {
        xmlDictFree((*reader).dict);
    }
    xmlFree.expect("non-null function pointer")(reader as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderClose(
    mut reader: xmlTextReaderPtr,
) -> i32 {
    if reader.is_null() {
        return -(1 as i32);
    }
    let ref mut fresh103 = (*reader).node;
    *fresh103 = 0 as xmlNodePtr;
    let ref mut fresh104 = (*reader).curnode;
    *fresh104 = 0 as xmlNodePtr;
    (*reader).mode = XML_TEXTREADER_MODE_CLOSED as i32;
    if !((*reader).faketext).is_null() {
        xmlFreeNode((*reader).faketext);
        let ref mut fresh105 = (*reader).faketext;
        *fresh105 = 0 as xmlNodePtr;
    }
    if !((*reader).ctxt).is_null() {
        if !((*(*reader).ctxt).vctxt.vstateTab).is_null()
            && (*(*reader).ctxt).vctxt.vstateMax > 0 as i32
        {
            while (*(*reader).ctxt).vctxt.vstateNr > 0 as i32 {
                xmlValidatePopElement(
                    &mut (*(*reader).ctxt).vctxt,
                    0 as xmlDocPtr,
                    0 as xmlNodePtr,
                    0 as *const xmlChar,
                );
            }
            xmlFree
                .expect(
                    "non-null function pointer",
                )((*(*reader).ctxt).vctxt.vstateTab as *mut libc::c_void);
            let ref mut fresh106 = (*(*reader).ctxt).vctxt.vstateTab;
            *fresh106 = 0 as *mut xmlValidState;
            (*(*reader).ctxt).vctxt.vstateMax = 0 as i32;
        }
        xmlStopParser((*reader).ctxt);
        if !((*(*reader).ctxt).myDoc).is_null() {
            if (*reader).preserve == 0 as i32 {
                xmlTextReaderFreeDoc(reader, (*(*reader).ctxt).myDoc);
            }
            let ref mut fresh107 = (*(*reader).ctxt).myDoc;
            *fresh107 = 0 as xmlDocPtr;
        }
    }
    if !((*reader).input).is_null() && (*reader).allocs & 1 as i32 != 0 {
        xmlFreeParserInputBuffer((*reader).input);
        (*reader).allocs -= 1 as i32;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderGetAttributeNo(
    mut reader: xmlTextReaderPtr,
    mut no: i32,
) -> *mut xmlChar {
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    let mut i: i32 = 0;
    let mut cur: xmlAttrPtr = 0 as *mut xmlAttr;
    let mut ns: xmlNsPtr = 0 as *mut xmlNs;
    if reader.is_null() {
        return 0 as *mut xmlChar;
    }
    if ((*reader).node).is_null() {
        return 0 as *mut xmlChar;
    }
    if !((*reader).curnode).is_null() {
        return 0 as *mut xmlChar;
    }
    if (*(*reader).node).type_0 as u32
        != XML_ELEMENT_NODE as i32 as u32
    {
        return 0 as *mut xmlChar;
    }
    ns = (*(*reader).node).nsDef;
    i = 0 as i32;
    while i < no && !ns.is_null() {
        ns = (*ns).next;
        i += 1;
    }
    if !ns.is_null() {
        return xmlStrdup((*ns).href);
    }
    cur = (*(*reader).node).properties;
    if cur.is_null() {
        return 0 as *mut xmlChar;
    }
    while i < no {
        cur = (*cur).next;
        if cur.is_null() {
            return 0 as *mut xmlChar;
        }
        i += 1;
    }
    ret = xmlNodeListGetString((*(*reader).node).doc, (*cur).children, 1 as i32);
    if ret.is_null() {
        return xmlStrdup(b"\0" as *const u8 as *const i8 as *mut xmlChar);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderGetAttribute(
    mut reader: xmlTextReaderPtr,
    mut name: *const xmlChar,
) -> *mut xmlChar {
    let mut prefix: *mut xmlChar = 0 as *mut xmlChar;
    let mut localname: *mut xmlChar = 0 as *mut xmlChar;
    let mut ns: xmlNsPtr = 0 as *mut xmlNs;
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    if reader.is_null() || name.is_null() {
        return 0 as *mut xmlChar;
    }
    if ((*reader).node).is_null() {
        return 0 as *mut xmlChar;
    }
    if !((*reader).curnode).is_null() {
        return 0 as *mut xmlChar;
    }
    if (*(*reader).node).type_0 as u32
        != XML_ELEMENT_NODE as i32 as u32
    {
        return 0 as *mut xmlChar;
    }
    localname = xmlSplitQName2(name, &mut prefix);
    if localname.is_null() {
        if xmlStrEqual(
            name,
            b"xmlns\0" as *const u8 as *const i8 as *mut xmlChar,
        ) != 0
        {
            ns = (*(*reader).node).nsDef;
            while !ns.is_null() {
                if ((*ns).prefix).is_null() {
                    return xmlStrdup((*ns).href);
                }
                ns = (*ns).next;
            }
            return 0 as *mut xmlChar;
        }
        return xmlGetNoNsProp((*reader).node as *const xmlNode, name);
    }
    if xmlStrEqual(
        prefix,
        b"xmlns\0" as *const u8 as *const i8 as *mut xmlChar,
    ) != 0
    {
        ns = (*(*reader).node).nsDef;
        while !ns.is_null() {
            if !((*ns).prefix).is_null() && xmlStrEqual((*ns).prefix, localname) != 0 {
                ret = xmlStrdup((*ns).href);
                break;
            } else {
                ns = (*ns).next;
            }
        }
    } else {
        ns = xmlSearchNs((*(*reader).node).doc, (*reader).node, prefix);
        if !ns.is_null() {
            ret = xmlGetNsProp((*reader).node as *const xmlNode, localname, (*ns).href);
        }
    }
    xmlFree.expect("non-null function pointer")(localname as *mut libc::c_void);
    if !prefix.is_null() {
        xmlFree.expect("non-null function pointer")(prefix as *mut libc::c_void);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderGetAttributeNs(
    mut reader: xmlTextReaderPtr,
    mut localName: *const xmlChar,
    mut namespaceURI: *const xmlChar,
) -> *mut xmlChar {
    let mut prefix: *mut xmlChar = 0 as *mut xmlChar;
    let mut ns: xmlNsPtr = 0 as *mut xmlNs;
    if reader.is_null() || localName.is_null() {
        return 0 as *mut xmlChar;
    }
    if ((*reader).node).is_null() {
        return 0 as *mut xmlChar;
    }
    if !((*reader).curnode).is_null() {
        return 0 as *mut xmlChar;
    }
    if (*(*reader).node).type_0 as u32
        != XML_ELEMENT_NODE as i32 as u32
    {
        return 0 as *mut xmlChar;
    }
    if xmlStrEqual(
        namespaceURI,
        b"http://www.w3.org/2000/xmlns/\0" as *const u8 as *const i8
            as *mut xmlChar,
    ) != 0
    {
        if xmlStrEqual(
            localName,
            b"xmlns\0" as *const u8 as *const i8 as *mut xmlChar,
        ) == 0
        {
            prefix = localName as *mut xmlChar;
        }
        ns = (*(*reader).node).nsDef;
        while !ns.is_null() {
            if prefix.is_null() && ((*ns).prefix).is_null()
                || !((*ns).prefix).is_null() && xmlStrEqual((*ns).prefix, localName) != 0
            {
                return xmlStrdup((*ns).href);
            }
            ns = (*ns).next;
        }
        return 0 as *mut xmlChar;
    }
    return xmlGetNsProp((*reader).node as *const xmlNode, localName, namespaceURI);
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderGetRemainder(
    mut reader: xmlTextReaderPtr,
) -> xmlParserInputBufferPtr {
    let mut ret: xmlParserInputBufferPtr = 0 as xmlParserInputBufferPtr;
    if reader.is_null() {
        return 0 as xmlParserInputBufferPtr;
    }
    if ((*reader).node).is_null() {
        return 0 as xmlParserInputBufferPtr;
    }
    let ref mut fresh108 = (*reader).node;
    *fresh108 = 0 as xmlNodePtr;
    let ref mut fresh109 = (*reader).curnode;
    *fresh109 = 0 as xmlNodePtr;
    (*reader).mode = XML_TEXTREADER_MODE_EOF as i32;
    if !((*reader).ctxt).is_null() {
        xmlStopParser((*reader).ctxt);
        if !((*(*reader).ctxt).myDoc).is_null() {
            if (*reader).preserve == 0 as i32 {
                xmlTextReaderFreeDoc(reader, (*(*reader).ctxt).myDoc);
            }
            let ref mut fresh110 = (*(*reader).ctxt).myDoc;
            *fresh110 = 0 as xmlDocPtr;
        }
    }
    if (*reader).allocs & 1 as i32 != 0 {
        ret = (*reader).input;
        let ref mut fresh111 = (*reader).input;
        *fresh111 = 0 as xmlParserInputBufferPtr;
        (*reader).allocs -= 1 as i32;
    } else {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"Unimplemented block at %s:%d\n\0" as *const u8 as *const i8,
            b"xmlreader.c\0" as *const u8 as *const i8,
            2468 as i32,
        );
        return 0 as xmlParserInputBufferPtr;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderLookupNamespace(
    mut reader: xmlTextReaderPtr,
    mut prefix: *const xmlChar,
) -> *mut xmlChar {
    let mut ns: xmlNsPtr = 0 as *mut xmlNs;
    if reader.is_null() {
        return 0 as *mut xmlChar;
    }
    if ((*reader).node).is_null() {
        return 0 as *mut xmlChar;
    }
    ns = xmlSearchNs((*(*reader).node).doc, (*reader).node, prefix);
    if ns.is_null() {
        return 0 as *mut xmlChar;
    }
    return xmlStrdup((*ns).href);
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderMoveToAttributeNo(
    mut reader: xmlTextReaderPtr,
    mut no: i32,
) -> i32 {
    let mut i: i32 = 0;
    let mut cur: xmlAttrPtr = 0 as *mut xmlAttr;
    let mut ns: xmlNsPtr = 0 as *mut xmlNs;
    if reader.is_null() {
        return -(1 as i32);
    }
    if ((*reader).node).is_null() {
        return -(1 as i32);
    }
    if (*(*reader).node).type_0 as u32
        != XML_ELEMENT_NODE as i32 as u32
    {
        return -(1 as i32);
    }
    let ref mut fresh112 = (*reader).curnode;
    *fresh112 = 0 as xmlNodePtr;
    ns = (*(*reader).node).nsDef;
    i = 0 as i32;
    while i < no && !ns.is_null() {
        ns = (*ns).next;
        i += 1;
    }
    if !ns.is_null() {
        let ref mut fresh113 = (*reader).curnode;
        *fresh113 = ns as xmlNodePtr;
        return 1 as i32;
    }
    cur = (*(*reader).node).properties;
    if cur.is_null() {
        return 0 as i32;
    }
    while i < no {
        cur = (*cur).next;
        if cur.is_null() {
            return 0 as i32;
        }
        i += 1;
    }
    let ref mut fresh114 = (*reader).curnode;
    *fresh114 = cur as xmlNodePtr;
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderMoveToAttribute(
    mut reader: xmlTextReaderPtr,
    mut name: *const xmlChar,
) -> i32 {
    let mut current_block: u64;
    let mut prefix: *mut xmlChar = 0 as *mut xmlChar;
    let mut localname: *mut xmlChar = 0 as *mut xmlChar;
    let mut ns: xmlNsPtr = 0 as *mut xmlNs;
    let mut prop: xmlAttrPtr = 0 as *mut xmlAttr;
    if reader.is_null() || name.is_null() {
        return -(1 as i32);
    }
    if ((*reader).node).is_null() {
        return -(1 as i32);
    }
    if (*(*reader).node).type_0 as u32
        != XML_ELEMENT_NODE as i32 as u32
    {
        return 0 as i32;
    }
    localname = xmlSplitQName2(name, &mut prefix);
    if localname.is_null() {
        if xmlStrEqual(
            name,
            b"xmlns\0" as *const u8 as *const i8 as *mut xmlChar,
        ) != 0
        {
            ns = (*(*reader).node).nsDef;
            while !ns.is_null() {
                if ((*ns).prefix).is_null() {
                    let ref mut fresh115 = (*reader).curnode;
                    *fresh115 = ns as xmlNodePtr;
                    return 1 as i32;
                }
                ns = (*ns).next;
            }
            return 0 as i32;
        }
        prop = (*(*reader).node).properties;
        while !prop.is_null() {
            if xmlStrEqual((*prop).name, name) != 0
                && (((*prop).ns).is_null() || ((*(*prop).ns).prefix).is_null())
            {
                let ref mut fresh116 = (*reader).curnode;
                *fresh116 = prop as xmlNodePtr;
                return 1 as i32;
            }
            prop = (*prop).next;
        }
        return 0 as i32;
    }
    if xmlStrEqual(
        prefix,
        b"xmlns\0" as *const u8 as *const i8 as *mut xmlChar,
    ) != 0
    {
        ns = (*(*reader).node).nsDef;
        loop {
            if ns.is_null() {
                current_block = 7357153348284164863;
                break;
            }
            if !((*ns).prefix).is_null() && xmlStrEqual((*ns).prefix, localname) != 0 {
                let ref mut fresh117 = (*reader).curnode;
                *fresh117 = ns as xmlNodePtr;
                current_block = 17751903034314626763;
                break;
            } else {
                ns = (*ns).next;
            }
        }
    } else {
        prop = (*(*reader).node).properties;
        loop {
            if prop.is_null() {
                current_block = 7357153348284164863;
                break;
            }
            if xmlStrEqual((*prop).name, localname) != 0 && !((*prop).ns).is_null()
                && xmlStrEqual((*(*prop).ns).prefix, prefix) != 0
            {
                let ref mut fresh118 = (*reader).curnode;
                *fresh118 = prop as xmlNodePtr;
                current_block = 17751903034314626763;
                break;
            } else {
                prop = (*prop).next;
            }
        }
    }
    match current_block {
        17751903034314626763 => {
            if !localname.is_null() {
                xmlFree
                    .expect("non-null function pointer")(localname as *mut libc::c_void);
            }
            if !prefix.is_null() {
                xmlFree.expect("non-null function pointer")(prefix as *mut libc::c_void);
            }
            return 1 as i32;
        }
        _ => {
            if !localname.is_null() {
                xmlFree
                    .expect("non-null function pointer")(localname as *mut libc::c_void);
            }
            if !prefix.is_null() {
                xmlFree.expect("non-null function pointer")(prefix as *mut libc::c_void);
            }
            return 0 as i32;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderMoveToAttributeNs(
    mut reader: xmlTextReaderPtr,
    mut localName: *const xmlChar,
    mut namespaceURI: *const xmlChar,
) -> i32 {
    let mut prop: xmlAttrPtr = 0 as *mut xmlAttr;
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    let mut ns: xmlNsPtr = 0 as *mut xmlNs;
    let mut prefix: *mut xmlChar = 0 as *mut xmlChar;
    if reader.is_null() || localName.is_null() || namespaceURI.is_null() {
        return -(1 as i32);
    }
    if ((*reader).node).is_null() {
        return -(1 as i32);
    }
    if (*(*reader).node).type_0 as u32
        != XML_ELEMENT_NODE as i32 as u32
    {
        return 0 as i32;
    }
    node = (*reader).node;
    if xmlStrEqual(
        namespaceURI,
        b"http://www.w3.org/2000/xmlns/\0" as *const u8 as *const i8
            as *mut xmlChar,
    ) != 0
    {
        if xmlStrEqual(
            localName,
            b"xmlns\0" as *const u8 as *const i8 as *mut xmlChar,
        ) == 0
        {
            prefix = localName as *mut xmlChar;
        }
        ns = (*(*reader).node).nsDef;
        while !ns.is_null() {
            if prefix.is_null() && ((*ns).prefix).is_null()
                || !((*ns).prefix).is_null() && xmlStrEqual((*ns).prefix, localName) != 0
            {
                let ref mut fresh119 = (*reader).curnode;
                *fresh119 = ns as xmlNodePtr;
                return 1 as i32;
            }
            ns = (*ns).next;
        }
        return 0 as i32;
    }
    prop = (*node).properties;
    while !prop.is_null() {
        if xmlStrEqual((*prop).name, localName) != 0
            && (!((*prop).ns).is_null()
                && xmlStrEqual((*(*prop).ns).href, namespaceURI) != 0)
        {
            let ref mut fresh120 = (*reader).curnode;
            *fresh120 = prop as xmlNodePtr;
            return 1 as i32;
        }
        prop = (*prop).next;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderMoveToFirstAttribute(
    mut reader: xmlTextReaderPtr,
) -> i32 {
    if reader.is_null() {
        return -(1 as i32);
    }
    if ((*reader).node).is_null() {
        return -(1 as i32);
    }
    if (*(*reader).node).type_0 as u32
        != XML_ELEMENT_NODE as i32 as u32
    {
        return 0 as i32;
    }
    if !((*(*reader).node).nsDef).is_null() {
        let ref mut fresh121 = (*reader).curnode;
        *fresh121 = (*(*reader).node).nsDef as xmlNodePtr;
        return 1 as i32;
    }
    if !((*(*reader).node).properties).is_null() {
        let ref mut fresh122 = (*reader).curnode;
        *fresh122 = (*(*reader).node).properties as xmlNodePtr;
        return 1 as i32;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderMoveToNextAttribute(
    mut reader: xmlTextReaderPtr,
) -> i32 {
    if reader.is_null() {
        return -(1 as i32);
    }
    if ((*reader).node).is_null() {
        return -(1 as i32);
    }
    if (*(*reader).node).type_0 as u32
        != XML_ELEMENT_NODE as i32 as u32
    {
        return 0 as i32;
    }
    if ((*reader).curnode).is_null() {
        return xmlTextReaderMoveToFirstAttribute(reader);
    }
    if (*(*reader).curnode).type_0 as u32
        == XML_NAMESPACE_DECL as i32 as u32
    {
        let mut ns: xmlNsPtr = (*reader).curnode as xmlNsPtr;
        if !((*ns).next).is_null() {
            let ref mut fresh123 = (*reader).curnode;
            *fresh123 = (*ns).next as xmlNodePtr;
            return 1 as i32;
        }
        if !((*(*reader).node).properties).is_null() {
            let ref mut fresh124 = (*reader).curnode;
            *fresh124 = (*(*reader).node).properties as xmlNodePtr;
            return 1 as i32;
        }
        return 0 as i32;
    } else {
        if (*(*reader).curnode).type_0 as u32
            == XML_ATTRIBUTE_NODE as i32 as u32
            && !((*(*reader).curnode).next).is_null()
        {
            let ref mut fresh125 = (*reader).curnode;
            *fresh125 = (*(*reader).curnode).next;
            return 1 as i32;
        }
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderMoveToElement(
    mut reader: xmlTextReaderPtr,
) -> i32 {
    if reader.is_null() {
        return -(1 as i32);
    }
    if ((*reader).node).is_null() {
        return -(1 as i32);
    }
    if (*(*reader).node).type_0 as u32
        != XML_ELEMENT_NODE as i32 as u32
    {
        return 0 as i32;
    }
    if !((*reader).curnode).is_null() {
        let ref mut fresh126 = (*reader).curnode;
        *fresh126 = 0 as xmlNodePtr;
        return 1 as i32;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderReadAttributeValue(
    mut reader: xmlTextReaderPtr,
) -> i32 {
    if reader.is_null() {
        return -(1 as i32);
    }
    if ((*reader).node).is_null() {
        return -(1 as i32);
    }
    if ((*reader).curnode).is_null() {
        return 0 as i32;
    }
    if (*(*reader).curnode).type_0 as u32
        == XML_ATTRIBUTE_NODE as i32 as u32
    {
        if ((*(*reader).curnode).children).is_null() {
            return 0 as i32;
        }
        let ref mut fresh127 = (*reader).curnode;
        *fresh127 = (*(*reader).curnode).children;
    } else if (*(*reader).curnode).type_0 as u32
            == XML_NAMESPACE_DECL as i32 as u32
        {
        let mut ns: xmlNsPtr = (*reader).curnode as xmlNsPtr;
        if ((*reader).faketext).is_null() {
            let ref mut fresh128 = (*reader).faketext;
            *fresh128 = xmlNewDocText((*(*reader).node).doc, (*ns).href);
        } else {
            if !((*(*reader).faketext).content).is_null()
                && (*(*reader).faketext).content
                    != &mut (*(*reader).faketext).properties as *mut *mut _xmlAttr
                        as *mut xmlChar
            {
                xmlFree
                    .expect(
                        "non-null function pointer",
                    )((*(*reader).faketext).content as *mut libc::c_void);
            }
            let ref mut fresh129 = (*(*reader).faketext).content;
            *fresh129 = xmlStrdup((*ns).href);
        }
        let ref mut fresh130 = (*reader).curnode;
        *fresh130 = (*reader).faketext;
    } else {
        if ((*(*reader).curnode).next).is_null() {
            return 0 as i32;
        }
        let ref mut fresh131 = (*reader).curnode;
        *fresh131 = (*(*reader).curnode).next;
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderConstEncoding(
    mut reader: xmlTextReaderPtr,
) -> *const xmlChar {
    let mut doc: xmlDocPtr = 0 as xmlDocPtr;
    if reader.is_null() {
        return 0 as *const xmlChar;
    }
    if !((*reader).doc).is_null() {
        doc = (*reader).doc;
    } else if !((*reader).ctxt).is_null() {
        doc = (*(*reader).ctxt).myDoc;
    }
    if doc.is_null() {
        return 0 as *const xmlChar;
    }
    if ((*doc).encoding).is_null() {
        return 0 as *const xmlChar
    } else {
        return xmlDictLookup((*reader).dict, (*doc).encoding, -(1 as i32))
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderAttributeCount(
    mut reader: xmlTextReaderPtr,
) -> i32 {
    let mut ret: i32 = 0;
    let mut attr: xmlAttrPtr = 0 as *mut xmlAttr;
    let mut ns: xmlNsPtr = 0 as *mut xmlNs;
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    if reader.is_null() {
        return -(1 as i32);
    }
    if ((*reader).node).is_null() {
        return 0 as i32;
    }
    if !((*reader).curnode).is_null() {
        node = (*reader).curnode;
    } else {
        node = (*reader).node;
    }
    if (*node).type_0 as u32 != XML_ELEMENT_NODE as i32 as u32
    {
        return 0 as i32;
    }
    if (*reader).state as i32 == XML_TEXTREADER_END as i32
        || (*reader).state as i32 == XML_TEXTREADER_BACKTRACK as i32
    {
        return 0 as i32;
    }
    ret = 0 as i32;
    attr = (*node).properties;
    while !attr.is_null() {
        ret += 1;
        attr = (*attr).next;
    }
    ns = (*node).nsDef;
    while !ns.is_null() {
        ret += 1;
        ns = (*ns).next;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderNodeType(
    mut reader: xmlTextReaderPtr,
) -> i32 {
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    if reader.is_null() {
        return -(1 as i32);
    }
    if ((*reader).node).is_null() {
        return XML_READER_TYPE_NONE as i32;
    }
    if !((*reader).curnode).is_null() {
        node = (*reader).curnode;
    } else {
        node = (*reader).node;
    }
    match (*node).type_0 as u32 {
        1 => {
            if (*reader).state as i32 == XML_TEXTREADER_END as i32
                || (*reader).state as i32
                    == XML_TEXTREADER_BACKTRACK as i32
            {
                return XML_READER_TYPE_END_ELEMENT as i32;
            }
            return XML_READER_TYPE_ELEMENT as i32;
        }
        18 | 2 => return XML_READER_TYPE_ATTRIBUTE as i32,
        3 => {
            if xmlIsBlankNode((*reader).node as *const xmlNode) != 0 {
                if xmlNodeGetSpacePreserve((*reader).node as *const xmlNode) != 0 {
                    return XML_READER_TYPE_SIGNIFICANT_WHITESPACE as i32
                } else {
                    return XML_READER_TYPE_WHITESPACE as i32
                }
            } else {
                return XML_READER_TYPE_TEXT as i32
            }
        }
        4 => return XML_READER_TYPE_CDATA as i32,
        5 => return XML_READER_TYPE_ENTITY_REFERENCE as i32,
        6 => return XML_READER_TYPE_ENTITY as i32,
        7 => return XML_READER_TYPE_PROCESSING_INSTRUCTION as i32,
        8 => return XML_READER_TYPE_COMMENT as i32,
        9 | 13 => return XML_READER_TYPE_DOCUMENT as i32,
        11 => return XML_READER_TYPE_DOCUMENT_FRAGMENT as i32,
        12 => return XML_READER_TYPE_NOTATION as i32,
        10 | 14 => return XML_READER_TYPE_DOCUMENT_TYPE as i32,
        15 | 16 | 17 | 19 | 20 => return XML_READER_TYPE_NONE as i32,
        _ => {}
    }
    return -(1 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderIsEmptyElement(
    mut reader: xmlTextReaderPtr,
) -> i32 {
    if reader.is_null() || ((*reader).node).is_null() {
        return -(1 as i32);
    }
    if (*(*reader).node).type_0 as u32
        != XML_ELEMENT_NODE as i32 as u32
    {
        return 0 as i32;
    }
    if !((*reader).curnode).is_null() {
        return 0 as i32;
    }
    if !((*(*reader).node).children).is_null() {
        return 0 as i32;
    }
    if (*reader).state as i32 == XML_TEXTREADER_END as i32 {
        return 0 as i32;
    }
    if !((*reader).doc).is_null() {
        return 1 as i32;
    }
    if (*reader).in_xinclude > 0 as i32 {
        return 1 as i32;
    }
    return ((*(*reader).node).extra as i32 & 0x1 as i32
        != 0 as i32) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderLocalName(
    mut reader: xmlTextReaderPtr,
) -> *mut xmlChar {
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    if reader.is_null() || ((*reader).node).is_null() {
        return 0 as *mut xmlChar;
    }
    if !((*reader).curnode).is_null() {
        node = (*reader).curnode;
    } else {
        node = (*reader).node;
    }
    if (*node).type_0 as u32
        == XML_NAMESPACE_DECL as i32 as u32
    {
        let mut ns: xmlNsPtr = node as xmlNsPtr;
        if ((*ns).prefix).is_null() {
            return xmlStrdup(
                b"xmlns\0" as *const u8 as *const i8 as *mut xmlChar,
            )
        } else {
            return xmlStrdup((*ns).prefix)
        }
    }
    if (*node).type_0 as u32 != XML_ELEMENT_NODE as i32 as u32
        && (*node).type_0 as u32
            != XML_ATTRIBUTE_NODE as i32 as u32
    {
        return xmlTextReaderName(reader);
    }
    return xmlStrdup((*node).name);
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderConstLocalName(
    mut reader: xmlTextReaderPtr,
) -> *const xmlChar {
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    if reader.is_null() || ((*reader).node).is_null() {
        return 0 as *const xmlChar;
    }
    if !((*reader).curnode).is_null() {
        node = (*reader).curnode;
    } else {
        node = (*reader).node;
    }
    if (*node).type_0 as u32
        == XML_NAMESPACE_DECL as i32 as u32
    {
        let mut ns: xmlNsPtr = node as xmlNsPtr;
        if ((*ns).prefix).is_null() {
            return xmlDictLookup(
                (*reader).dict,
                b"xmlns\0" as *const u8 as *const i8 as *mut xmlChar,
                -(1 as i32),
            )
        } else {
            return (*ns).prefix
        }
    }
    if (*node).type_0 as u32 != XML_ELEMENT_NODE as i32 as u32
        && (*node).type_0 as u32
            != XML_ATTRIBUTE_NODE as i32 as u32
    {
        return xmlTextReaderConstName(reader);
    }
    return (*node).name;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderName(
    mut reader: xmlTextReaderPtr,
) -> *mut xmlChar {
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    if reader.is_null() || ((*reader).node).is_null() {
        return 0 as *mut xmlChar;
    }
    if !((*reader).curnode).is_null() {
        node = (*reader).curnode;
    } else {
        node = (*reader).node;
    }
    match (*node).type_0 as u32 {
        1 | 2 => {
            if ((*node).ns).is_null() || ((*(*node).ns).prefix).is_null() {
                return xmlStrdup((*node).name);
            }
            ret = xmlStrdup((*(*node).ns).prefix);
            ret = xmlStrcat(
                ret,
                b":\0" as *const u8 as *const i8 as *mut xmlChar,
            );
            ret = xmlStrcat(ret, (*node).name);
            return ret;
        }
        3 => {
            return xmlStrdup(
                b"#text\0" as *const u8 as *const i8 as *mut xmlChar,
            );
        }
        4 => {
            return xmlStrdup(
                b"#cdata-section\0" as *const u8 as *const i8 as *mut xmlChar,
            );
        }
        6 | 5 => return xmlStrdup((*node).name),
        7 => return xmlStrdup((*node).name),
        8 => {
            return xmlStrdup(
                b"#comment\0" as *const u8 as *const i8 as *mut xmlChar,
            );
        }
        9 | 13 => {
            return xmlStrdup(
                b"#document\0" as *const u8 as *const i8 as *mut xmlChar,
            );
        }
        11 => {
            return xmlStrdup(
                b"#document-fragment\0" as *const u8 as *const i8
                    as *mut xmlChar,
            );
        }
        12 => return xmlStrdup((*node).name),
        10 | 14 => return xmlStrdup((*node).name),
        18 => {
            let mut ns: xmlNsPtr = node as xmlNsPtr;
            ret = xmlStrdup(
                b"xmlns\0" as *const u8 as *const i8 as *mut xmlChar,
            );
            if ((*ns).prefix).is_null() {
                return ret;
            }
            ret = xmlStrcat(
                ret,
                b":\0" as *const u8 as *const i8 as *mut xmlChar,
            );
            ret = xmlStrcat(ret, (*ns).prefix);
            return ret;
        }
        15 | 16 | 17 | 19 | 20 => return 0 as *mut xmlChar,
        _ => {}
    }
    return 0 as *mut xmlChar;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderConstName(
    mut reader: xmlTextReaderPtr,
) -> *const xmlChar {
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    if reader.is_null() || ((*reader).node).is_null() {
        return 0 as *const xmlChar;
    }
    if !((*reader).curnode).is_null() {
        node = (*reader).curnode;
    } else {
        node = (*reader).node;
    }
    match (*node).type_0 as u32 {
        1 | 2 => {
            if ((*node).ns).is_null() || ((*(*node).ns).prefix).is_null() {
                return (*node).name;
            }
            return xmlDictQLookup((*reader).dict, (*(*node).ns).prefix, (*node).name);
        }
        3 => {
            return xmlDictLookup(
                (*reader).dict,
                b"#text\0" as *const u8 as *const i8 as *mut xmlChar,
                -(1 as i32),
            );
        }
        4 => {
            return xmlDictLookup(
                (*reader).dict,
                b"#cdata-section\0" as *const u8 as *const i8 as *mut xmlChar,
                -(1 as i32),
            );
        }
        6 | 5 => return xmlDictLookup((*reader).dict, (*node).name, -(1 as i32)),
        7 => return xmlDictLookup((*reader).dict, (*node).name, -(1 as i32)),
        8 => {
            return xmlDictLookup(
                (*reader).dict,
                b"#comment\0" as *const u8 as *const i8 as *mut xmlChar,
                -(1 as i32),
            );
        }
        9 | 13 => {
            return xmlDictLookup(
                (*reader).dict,
                b"#document\0" as *const u8 as *const i8 as *mut xmlChar,
                -(1 as i32),
            );
        }
        11 => {
            return xmlDictLookup(
                (*reader).dict,
                b"#document-fragment\0" as *const u8 as *const i8
                    as *mut xmlChar,
                -(1 as i32),
            );
        }
        12 => return xmlDictLookup((*reader).dict, (*node).name, -(1 as i32)),
        10 | 14 => {
            return xmlDictLookup((*reader).dict, (*node).name, -(1 as i32));
        }
        18 => {
            let mut ns: xmlNsPtr = node as xmlNsPtr;
            if ((*ns).prefix).is_null() {
                return xmlDictLookup(
                    (*reader).dict,
                    b"xmlns\0" as *const u8 as *const i8 as *mut xmlChar,
                    -(1 as i32),
                );
            }
            return xmlDictQLookup(
                (*reader).dict,
                b"xmlns\0" as *const u8 as *const i8 as *mut xmlChar,
                (*ns).prefix,
            );
        }
        15 | 16 | 17 | 19 | 20 => return 0 as *const xmlChar,
        _ => {}
    }
    return 0 as *const xmlChar;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderPrefix(
    mut reader: xmlTextReaderPtr,
) -> *mut xmlChar {
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    if reader.is_null() || ((*reader).node).is_null() {
        return 0 as *mut xmlChar;
    }
    if !((*reader).curnode).is_null() {
        node = (*reader).curnode;
    } else {
        node = (*reader).node;
    }
    if (*node).type_0 as u32
        == XML_NAMESPACE_DECL as i32 as u32
    {
        let mut ns: xmlNsPtr = node as xmlNsPtr;
        if ((*ns).prefix).is_null() {
            return 0 as *mut xmlChar;
        }
        return xmlStrdup(b"xmlns\0" as *const u8 as *const i8 as *mut xmlChar);
    }
    if (*node).type_0 as u32 != XML_ELEMENT_NODE as i32 as u32
        && (*node).type_0 as u32
            != XML_ATTRIBUTE_NODE as i32 as u32
    {
        return 0 as *mut xmlChar;
    }
    if !((*node).ns).is_null() && !((*(*node).ns).prefix).is_null() {
        return xmlStrdup((*(*node).ns).prefix);
    }
    return 0 as *mut xmlChar;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderConstPrefix(
    mut reader: xmlTextReaderPtr,
) -> *const xmlChar {
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    if reader.is_null() || ((*reader).node).is_null() {
        return 0 as *const xmlChar;
    }
    if !((*reader).curnode).is_null() {
        node = (*reader).curnode;
    } else {
        node = (*reader).node;
    }
    if (*node).type_0 as u32
        == XML_NAMESPACE_DECL as i32 as u32
    {
        let mut ns: xmlNsPtr = node as xmlNsPtr;
        if ((*ns).prefix).is_null() {
            return 0 as *const xmlChar;
        }
        return xmlDictLookup(
            (*reader).dict,
            b"xmlns\0" as *const u8 as *const i8 as *mut xmlChar,
            -(1 as i32),
        );
    }
    if (*node).type_0 as u32 != XML_ELEMENT_NODE as i32 as u32
        && (*node).type_0 as u32
            != XML_ATTRIBUTE_NODE as i32 as u32
    {
        return 0 as *const xmlChar;
    }
    if !((*node).ns).is_null() && !((*(*node).ns).prefix).is_null() {
        return xmlDictLookup((*reader).dict, (*(*node).ns).prefix, -(1 as i32));
    }
    return 0 as *const xmlChar;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderNamespaceUri(
    mut reader: xmlTextReaderPtr,
) -> *mut xmlChar {
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    if reader.is_null() || ((*reader).node).is_null() {
        return 0 as *mut xmlChar;
    }
    if !((*reader).curnode).is_null() {
        node = (*reader).curnode;
    } else {
        node = (*reader).node;
    }
    if (*node).type_0 as u32
        == XML_NAMESPACE_DECL as i32 as u32
    {
        return xmlStrdup(
            b"http://www.w3.org/2000/xmlns/\0" as *const u8 as *const i8
                as *mut xmlChar,
        );
    }
    if (*node).type_0 as u32 != XML_ELEMENT_NODE as i32 as u32
        && (*node).type_0 as u32
            != XML_ATTRIBUTE_NODE as i32 as u32
    {
        return 0 as *mut xmlChar;
    }
    if !((*node).ns).is_null() {
        return xmlStrdup((*(*node).ns).href);
    }
    return 0 as *mut xmlChar;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderConstNamespaceUri(
    mut reader: xmlTextReaderPtr,
) -> *const xmlChar {
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    if reader.is_null() || ((*reader).node).is_null() {
        return 0 as *const xmlChar;
    }
    if !((*reader).curnode).is_null() {
        node = (*reader).curnode;
    } else {
        node = (*reader).node;
    }
    if (*node).type_0 as u32
        == XML_NAMESPACE_DECL as i32 as u32
    {
        return xmlDictLookup(
            (*reader).dict,
            b"http://www.w3.org/2000/xmlns/\0" as *const u8 as *const i8
                as *mut xmlChar,
            -(1 as i32),
        );
    }
    if (*node).type_0 as u32 != XML_ELEMENT_NODE as i32 as u32
        && (*node).type_0 as u32
            != XML_ATTRIBUTE_NODE as i32 as u32
    {
        return 0 as *const xmlChar;
    }
    if !((*node).ns).is_null() {
        return xmlDictLookup((*reader).dict, (*(*node).ns).href, -(1 as i32));
    }
    return 0 as *const xmlChar;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderBaseUri(
    mut reader: xmlTextReaderPtr,
) -> *mut xmlChar {
    if reader.is_null() || ((*reader).node).is_null() {
        return 0 as *mut xmlChar;
    }
    return xmlNodeGetBase(0 as *const xmlDoc, (*reader).node as *const xmlNode);
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderConstBaseUri(
    mut reader: xmlTextReaderPtr,
) -> *const xmlChar {
    let mut tmp: *mut xmlChar = 0 as *mut xmlChar;
    let mut ret: *const xmlChar = 0 as *const xmlChar;
    if reader.is_null() || ((*reader).node).is_null() {
        return 0 as *const xmlChar;
    }
    tmp = xmlNodeGetBase(0 as *const xmlDoc, (*reader).node as *const xmlNode);
    if tmp.is_null() {
        return 0 as *const xmlChar;
    }
    ret = xmlDictLookup((*reader).dict, tmp, -(1 as i32));
    xmlFree.expect("non-null function pointer")(tmp as *mut libc::c_void);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderDepth(
    mut reader: xmlTextReaderPtr,
) -> i32 {
    if reader.is_null() {
        return -(1 as i32);
    }
    if ((*reader).node).is_null() {
        return 0 as i32;
    }
    if !((*reader).curnode).is_null() {
        if (*(*reader).curnode).type_0 as u32
            == XML_ATTRIBUTE_NODE as i32 as u32
            || (*(*reader).curnode).type_0 as u32
                == XML_NAMESPACE_DECL as i32 as u32
        {
            return (*reader).depth + 1 as i32;
        }
        return (*reader).depth + 2 as i32;
    }
    return (*reader).depth;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderHasAttributes(
    mut reader: xmlTextReaderPtr,
) -> i32 {
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    if reader.is_null() {
        return -(1 as i32);
    }
    if ((*reader).node).is_null() {
        return 0 as i32;
    }
    if !((*reader).curnode).is_null() {
        node = (*reader).curnode;
    } else {
        node = (*reader).node;
    }
    if (*node).type_0 as u32 == XML_ELEMENT_NODE as i32 as u32
        && (!((*node).properties).is_null() || !((*node).nsDef).is_null())
    {
        return 1 as i32;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderHasValue(
    mut reader: xmlTextReaderPtr,
) -> i32 {
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    if reader.is_null() {
        return -(1 as i32);
    }
    if ((*reader).node).is_null() {
        return 0 as i32;
    }
    if !((*reader).curnode).is_null() {
        node = (*reader).curnode;
    } else {
        node = (*reader).node;
    }
    match (*node).type_0 as u32 {
        2 | 3 | 4 | 7 | 8 | 18 => return 1 as i32,
        _ => {}
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderValue(
    mut reader: xmlTextReaderPtr,
) -> *mut xmlChar {
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    if reader.is_null() {
        return 0 as *mut xmlChar;
    }
    if ((*reader).node).is_null() {
        return 0 as *mut xmlChar;
    }
    if !((*reader).curnode).is_null() {
        node = (*reader).curnode;
    } else {
        node = (*reader).node;
    }
    match (*node).type_0 as u32 {
        18 => return xmlStrdup((*(node as xmlNsPtr)).href),
        2 => {
            let mut attr: xmlAttrPtr = node as xmlAttrPtr;
            if !((*attr).parent).is_null() {
                return xmlNodeListGetString(
                    (*(*attr).parent).doc,
                    (*attr).children,
                    1 as i32,
                )
            } else {
                return xmlNodeListGetString(
                    0 as xmlDocPtr,
                    (*attr).children,
                    1 as i32,
                )
            }
        }
        3 | 4 | 7 | 8 => {
            if !((*node).content).is_null() {
                return xmlStrdup((*node).content);
            }
        }
        _ => {}
    }
    return 0 as *mut xmlChar;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderConstValue(
    mut reader: xmlTextReaderPtr,
) -> *const xmlChar {
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    if reader.is_null() {
        return 0 as *const xmlChar;
    }
    if ((*reader).node).is_null() {
        return 0 as *const xmlChar;
    }
    if !((*reader).curnode).is_null() {
        node = (*reader).curnode;
    } else {
        node = (*reader).node;
    }
    match (*node).type_0 as u32 {
        18 => return (*(node as xmlNsPtr)).href,
        2 => {
            let mut attr: xmlAttrPtr = node as xmlAttrPtr;
            let mut ret: *const xmlChar = 0 as *const xmlChar;
            if !((*attr).children).is_null()
                && (*(*attr).children).type_0 as u32
                    == XML_TEXT_NODE as i32 as u32
                && ((*(*attr).children).next).is_null()
            {
                return (*(*attr).children).content
            } else {
                if ((*reader).buffer).is_null() {
                    let ref mut fresh132 = (*reader).buffer;
                    *fresh132 = xmlBufCreateSize(100 as i32 as size_t);
                    if ((*reader).buffer).is_null() {
                        (*__xmlGenericError())
                            .expect(
                                "non-null function pointer",
                            )(
                            *__xmlGenericErrorContext(),
                            b"xmlTextReaderSetup : malloc failed\n\0" as *const u8
                                as *const i8,
                        );
                        return 0 as *const xmlChar;
                    }
                    xmlBufSetAllocationScheme(
                        (*reader).buffer,
                        XML_BUFFER_ALLOC_DOUBLEIT,
                    );
                } else {
                    xmlBufEmpty((*reader).buffer);
                }
                xmlBufGetNodeContent((*reader).buffer, node as *const xmlNode);
                ret = xmlBufContent((*reader).buffer as *const xmlBuf);
                if ret.is_null() {
                    xmlBufFree((*reader).buffer);
                    let ref mut fresh133 = (*reader).buffer;
                    *fresh133 = xmlBufCreateSize(100 as i32 as size_t);
                    xmlBufSetAllocationScheme(
                        (*reader).buffer,
                        XML_BUFFER_ALLOC_DOUBLEIT,
                    );
                    ret = b"\0" as *const u8 as *const i8 as *mut xmlChar;
                }
                return ret;
            }
        }
        3 | 4 | 7 | 8 => return (*node).content,
        _ => {}
    }
    return 0 as *const xmlChar;
}
#[no_mangle]
<<<<<<< HEAD
pub extern "C" fn xmlTextReaderIsDefault(
=======
pub unsafe extern "C" fn xmlTextReaderIsDefault(
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
    mut reader: xmlTextReaderPtr,
) -> i32 {
    if reader.is_null() {
        return -(1 as i32);
    }
    return 0 as i32;
}
#[no_mangle]
<<<<<<< HEAD
pub extern "C" fn xmlTextReaderQuoteChar(
=======
pub unsafe extern "C" fn xmlTextReaderQuoteChar(
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
    mut reader: xmlTextReaderPtr,
) -> i32 {
    if reader.is_null() {
        return -(1 as i32);
    }
    return '"' as i32;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderXmlLang(
    mut reader: xmlTextReaderPtr,
) -> *mut xmlChar {
    if reader.is_null() {
        return 0 as *mut xmlChar;
    }
    if ((*reader).node).is_null() {
        return 0 as *mut xmlChar;
    }
    return xmlNodeGetLang((*reader).node as *const xmlNode);
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderConstXmlLang(
    mut reader: xmlTextReaderPtr,
) -> *const xmlChar {
    let mut tmp: *mut xmlChar = 0 as *mut xmlChar;
    let mut ret: *const xmlChar = 0 as *const xmlChar;
    if reader.is_null() {
        return 0 as *const xmlChar;
    }
    if ((*reader).node).is_null() {
        return 0 as *const xmlChar;
    }
    tmp = xmlNodeGetLang((*reader).node as *const xmlNode);
    if tmp.is_null() {
        return 0 as *const xmlChar;
    }
    ret = xmlDictLookup((*reader).dict, tmp, -(1 as i32));
    xmlFree.expect("non-null function pointer")(tmp as *mut libc::c_void);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderConstString(
    mut reader: xmlTextReaderPtr,
    mut str: *const xmlChar,
) -> *const xmlChar {
    if reader.is_null() {
        return 0 as *const xmlChar;
    }
    return xmlDictLookup((*reader).dict, str, -(1 as i32));
}
#[no_mangle]
<<<<<<< HEAD
pub extern "C" fn xmlTextReaderNormalization(
=======
pub unsafe extern "C" fn xmlTextReaderNormalization(
>>>>>>> bbfa64d0f7db1b5e94d853ac3a3845e65fc050fe
    mut reader: xmlTextReaderPtr,
) -> i32 {
    if reader.is_null() {
        return -(1 as i32);
    }
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderSetParserProp(
    mut reader: xmlTextReaderPtr,
    mut prop: i32,
    mut value: i32,
) -> i32 {
    let mut p: xmlParserProperties = prop as xmlParserProperties;
    let mut ctxt: xmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
    if reader.is_null() || ((*reader).ctxt).is_null() {
        return -(1 as i32);
    }
    ctxt = (*reader).ctxt;
    match p as u32 {
        1 => {
            if value != 0 as i32 {
                if (*ctxt).loadsubset == 0 as i32 {
                    if (*reader).mode != XML_TEXTREADER_MODE_INITIAL as i32 {
                        return -(1 as i32);
                    }
                    (*ctxt).loadsubset = 2 as i32;
                }
            } else {
                (*ctxt).loadsubset = 0 as i32;
            }
            return 0 as i32;
        }
        2 => {
            if value != 0 as i32 {
                (*ctxt).loadsubset |= 4 as i32;
            } else if (*ctxt).loadsubset & 4 as i32 != 0 {
                (*ctxt).loadsubset -= 4 as i32;
            }
            return 0 as i32;
        }
        3 => {
            if value != 0 as i32 {
                (*ctxt).options |= XML_PARSE_DTDVALID as i32;
                (*ctxt).validate = 1 as i32;
                (*reader).validate = XML_TEXTREADER_VALIDATE_DTD;
            } else {
                (*ctxt).options &= !(XML_PARSE_DTDVALID as i32);
                (*ctxt).validate = 0 as i32;
            }
            return 0 as i32;
        }
        4 => {
            if value != 0 as i32 {
                (*ctxt).options |= XML_PARSE_NOENT as i32;
                (*ctxt).replaceEntities = 1 as i32;
            } else {
                (*ctxt).options &= !(XML_PARSE_NOENT as i32);
                (*ctxt).replaceEntities = 0 as i32;
            }
            return 0 as i32;
        }
        _ => {}
    }
    return -(1 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderGetParserProp(
    mut reader: xmlTextReaderPtr,
    mut prop: i32,
) -> i32 {
    let mut p: xmlParserProperties = prop as xmlParserProperties;
    let mut ctxt: xmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
    if reader.is_null() || ((*reader).ctxt).is_null() {
        return -(1 as i32);
    }
    ctxt = (*reader).ctxt;
    match p as u32 {
        1 => {
            if (*ctxt).loadsubset != 0 as i32
                || (*ctxt).validate != 0 as i32
            {
                return 1 as i32;
            }
            return 0 as i32;
        }
        2 => {
            if (*ctxt).loadsubset & 4 as i32 != 0 {
                return 1 as i32;
            }
            return 0 as i32;
        }
        3 => return (*reader).validate as i32,
        4 => return (*ctxt).replaceEntities,
        _ => {}
    }
    return -(1 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderGetParserLineNumber(
    mut reader: xmlTextReaderPtr,
) -> i32 {
    if reader.is_null() || ((*reader).ctxt).is_null()
        || ((*(*reader).ctxt).input).is_null()
    {
        return 0 as i32;
    }
    return (*(*(*reader).ctxt).input).line;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderGetParserColumnNumber(
    mut reader: xmlTextReaderPtr,
) -> i32 {
    if reader.is_null() || ((*reader).ctxt).is_null()
        || ((*(*reader).ctxt).input).is_null()
    {
        return 0 as i32;
    }
    return (*(*(*reader).ctxt).input).col;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderCurrentNode(
    mut reader: xmlTextReaderPtr,
) -> xmlNodePtr {
    if reader.is_null() {
        return 0 as xmlNodePtr;
    }
    if !((*reader).curnode).is_null() {
        return (*reader).curnode;
    }
    return (*reader).node;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderPreserve(
    mut reader: xmlTextReaderPtr,
) -> xmlNodePtr {
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut parent: xmlNodePtr = 0 as *mut xmlNode;
    if reader.is_null() {
        return 0 as xmlNodePtr;
    }
    if !((*reader).curnode).is_null() {
        cur = (*reader).curnode;
    } else {
        cur = (*reader).node;
    }
    if cur.is_null() {
        return 0 as xmlNodePtr;
    }
    if (*cur).type_0 as u32 != XML_DOCUMENT_NODE as i32 as u32
        && (*cur).type_0 as u32 != XML_DTD_NODE as i32 as u32
    {
        let ref mut fresh134 = (*cur).extra;
        *fresh134 = (*fresh134 as i32 | 0x2 as i32) as u16;
        let ref mut fresh135 = (*cur).extra;
        *fresh135 = (*fresh135 as i32 | 0x4 as i32) as u16;
    }
    let ref mut fresh136 = (*reader).preserves;
    *fresh136 += 1;
    parent = (*cur).parent;
    while !parent.is_null() {
        if (*parent).type_0 as u32
            == XML_ELEMENT_NODE as i32 as u32
        {
            let ref mut fresh137 = (*parent).extra;
            *fresh137 = (*fresh137 as i32 | 0x2 as i32)
                as u16;
        }
        parent = (*parent).parent;
    }
    return cur;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderPreservePattern(
    mut reader: xmlTextReaderPtr,
    mut pattern: *const xmlChar,
    mut namespaces: *mut *const xmlChar,
) -> i32 {
    let mut comp: xmlPatternPtr = 0 as *mut xmlPattern;
    if reader.is_null() || pattern.is_null() {
        return -(1 as i32);
    }
    comp = xmlPatterncompile(pattern, (*reader).dict, 0 as i32, namespaces);
    if comp.is_null() {
        return -(1 as i32);
    }
    if (*reader).patternMax <= 0 as i32 {
        (*reader).patternMax = 4 as i32;
        let ref mut fresh138 = (*reader).patternTab;
        *fresh138 = xmlMalloc
            .expect(
                "non-null function pointer",
            )(
            ((*reader).patternMax as u64)
                .wrapping_mul(::std::mem::size_of::<xmlPatternPtr>() as u64),
        ) as *mut xmlPatternPtr;
        if ((*reader).patternTab).is_null() {
            (*__xmlGenericError())
                .expect(
                    "non-null function pointer",
                )(
                *__xmlGenericErrorContext(),
                b"xmlMalloc failed !\n\0" as *const u8 as *const i8,
            );
            return -(1 as i32);
        }
    }
    if (*reader).patternNr >= (*reader).patternMax {
        let mut tmp: *mut xmlPatternPtr = 0 as *mut xmlPatternPtr;
        (*reader).patternMax *= 2 as i32;
        tmp = xmlRealloc
            .expect(
                "non-null function pointer",
            )(
            (*reader).patternTab as *mut libc::c_void,
            ((*reader).patternMax as u64)
                .wrapping_mul(::std::mem::size_of::<xmlPatternPtr>() as u64),
        ) as *mut xmlPatternPtr;
        if tmp.is_null() {
            (*__xmlGenericError())
                .expect(
                    "non-null function pointer",
                )(
                *__xmlGenericErrorContext(),
                b"xmlRealloc failed !\n\0" as *const u8 as *const i8,
            );
            (*reader).patternMax /= 2 as i32;
            return -(1 as i32);
        }
        let ref mut fresh139 = (*reader).patternTab;
        *fresh139 = tmp;
    }
    let ref mut fresh140 = *((*reader).patternTab).offset((*reader).patternNr as isize);
    *fresh140 = comp;
    let ref mut fresh141 = (*reader).patternNr;
    let fresh142 = *fresh141;
    *fresh141 = *fresh141 + 1;
    return fresh142;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderCurrentDoc(
    mut reader: xmlTextReaderPtr,
) -> xmlDocPtr {
    if reader.is_null() {
        return 0 as xmlDocPtr;
    }
    if !((*reader).doc).is_null() {
        return (*reader).doc;
    }
    if ((*reader).ctxt).is_null() || ((*(*reader).ctxt).myDoc).is_null() {
        return 0 as xmlDocPtr;
    }
    (*reader).preserve = 1 as i32;
    return (*(*reader).ctxt).myDoc;
}
unsafe extern "C" fn xmlTextReaderValidityErrorRelay(
    mut ctx: *mut libc::c_void,
    mut msg: *const i8,
    mut args: ...
) {
    let mut reader: xmlTextReaderPtr = ctx as xmlTextReaderPtr;
    let mut str: *mut i8 = 0 as *mut i8;
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    str = xmlTextReaderBuildMessage(msg, ap.as_va_list());
    if ((*reader).errorFunc).is_none() {
        xmlTextReaderValidityError(
            ctx,
            b"%s\0" as *const u8 as *const i8,
            str,
        );
    } else {
        ((*reader).errorFunc)
            .expect(
                "non-null function pointer",
            )(
            (*reader).errorFuncArg,
            str,
            XML_PARSER_SEVERITY_VALIDITY_ERROR,
            0 as *mut libc::c_void,
        );
    }
    if !str.is_null() {
        xmlFree.expect("non-null function pointer")(str as *mut libc::c_void);
    }
}
unsafe extern "C" fn xmlTextReaderValidityWarningRelay(
    mut ctx: *mut libc::c_void,
    mut msg: *const i8,
    mut args: ...
) {
    let mut reader: xmlTextReaderPtr = ctx as xmlTextReaderPtr;
    let mut str: *mut i8 = 0 as *mut i8;
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    str = xmlTextReaderBuildMessage(msg, ap.as_va_list());
    if ((*reader).errorFunc).is_none() {
        xmlTextReaderValidityWarning(
            ctx,
            b"%s\0" as *const u8 as *const i8,
            str,
        );
    } else {
        ((*reader).errorFunc)
            .expect(
                "non-null function pointer",
            )(
            (*reader).errorFuncArg,
            str,
            XML_PARSER_SEVERITY_VALIDITY_WARNING,
            0 as *mut libc::c_void,
        );
    }
    if !str.is_null() {
        xmlFree.expect("non-null function pointer")(str as *mut libc::c_void);
    }
}
unsafe extern "C" fn xmlTextReaderValidityStructuredRelay(
    mut userData: *mut libc::c_void,
    mut error: xmlErrorPtr,
) {
    let mut reader: xmlTextReaderPtr = userData as xmlTextReaderPtr;
    if ((*reader).sErrorFunc).is_some() {
        ((*reader).sErrorFunc)
            .expect("non-null function pointer")((*reader).errorFuncArg, error);
    } else {
        xmlTextReaderStructuredError(reader as *mut libc::c_void, error);
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderRelaxNGSetSchema(
    mut reader: xmlTextReaderPtr,
    mut schema: xmlRelaxNGPtr,
) -> i32 {
    if reader.is_null() {
        return -(1 as i32);
    }
    if schema.is_null() {
        if !((*reader).rngSchemas).is_null() {
            xmlRelaxNGFree((*reader).rngSchemas);
            let ref mut fresh143 = (*reader).rngSchemas;
            *fresh143 = 0 as xmlRelaxNGPtr;
        }
        if !((*reader).rngValidCtxt).is_null() {
            if (*reader).rngPreserveCtxt == 0 {
                xmlRelaxNGFreeValidCtxt((*reader).rngValidCtxt);
            }
            let ref mut fresh144 = (*reader).rngValidCtxt;
            *fresh144 = 0 as xmlRelaxNGValidCtxtPtr;
        }
        (*reader).rngPreserveCtxt = 0 as i32;
        return 0 as i32;
    }
    if (*reader).mode != XML_TEXTREADER_MODE_INITIAL as i32 {
        return -(1 as i32);
    }
    if !((*reader).rngSchemas).is_null() {
        xmlRelaxNGFree((*reader).rngSchemas);
        let ref mut fresh145 = (*reader).rngSchemas;
        *fresh145 = 0 as xmlRelaxNGPtr;
    }
    if !((*reader).rngValidCtxt).is_null() {
        if (*reader).rngPreserveCtxt == 0 {
            xmlRelaxNGFreeValidCtxt((*reader).rngValidCtxt);
        }
        let ref mut fresh146 = (*reader).rngValidCtxt;
        *fresh146 = 0 as xmlRelaxNGValidCtxtPtr;
    }
    (*reader).rngPreserveCtxt = 0 as i32;
    let ref mut fresh147 = (*reader).rngValidCtxt;
    *fresh147 = xmlRelaxNGNewValidCtxt(schema);
    if ((*reader).rngValidCtxt).is_null() {
        return -(1 as i32);
    }
    if ((*reader).errorFunc).is_some() {
        xmlRelaxNGSetValidErrors(
            (*reader).rngValidCtxt,
            Some(
                xmlTextReaderValidityErrorRelay
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const i8,
                        ...
                    ) -> (),
            ),
            Some(
                xmlTextReaderValidityWarningRelay
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const i8,
                        ...
                    ) -> (),
            ),
            reader as *mut libc::c_void,
        );
    }
    if ((*reader).sErrorFunc).is_some() {
        xmlRelaxNGSetValidStructuredErrors(
            (*reader).rngValidCtxt,
            Some(
                xmlTextReaderValidityStructuredRelay
                    as unsafe extern "C" fn(*mut libc::c_void, xmlErrorPtr) -> (),
            ),
            reader as *mut libc::c_void,
        );
    }
    (*reader).rngValidErrors = 0 as i32;
    let ref mut fresh148 = (*reader).rngFullNode;
    *fresh148 = 0 as xmlNodePtr;
    (*reader).validate = XML_TEXTREADER_VALIDATE_RNG;
    return 0 as i32;
}
unsafe extern "C" fn xmlTextReaderLocator(
    mut ctx: *mut libc::c_void,
    mut file: *mut *const i8,
    mut line: *mut u64,
) -> i32 {
    let mut reader: xmlTextReaderPtr = 0 as *mut xmlTextReader;
    if ctx.is_null() || file.is_null() && line.is_null() {
        return -(1 as i32);
    }
    if !file.is_null() {
        *file = 0 as *const i8;
    }
    if !line.is_null() {
        *line = 0 as i32 as u64;
    }
    reader = ctx as xmlTextReaderPtr;
    if !((*reader).ctxt).is_null() && !((*(*reader).ctxt).input).is_null() {
        if !file.is_null() {
            *file = (*(*(*reader).ctxt).input).filename;
        }
        if !line.is_null() {
            *line = (*(*(*reader).ctxt).input).line as u64;
        }
        return 0 as i32;
    }
    if !((*reader).node).is_null() {
        let mut res: i64 = 0;
        let mut ret: i32 = 0 as i32;
        if !line.is_null() {
            res = xmlGetLineNo((*reader).node as *const xmlNode);
            if res > 0 as i32 as i64 {
                *line = res as u64;
            } else {
                ret = -(1 as i32);
            }
        }
        if !file.is_null() {
            let mut doc: xmlDocPtr = (*(*reader).node).doc;
            if !doc.is_null() && !((*doc).URL).is_null() {
                *file = (*doc).URL as *const i8;
            } else {
                ret = -(1 as i32);
            }
        }
        return ret;
    }
    return -(1 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderSetSchema(
    mut reader: xmlTextReaderPtr,
    mut schema: xmlSchemaPtr,
) -> i32 {
    if reader.is_null() {
        return -(1 as i32);
    }
    if schema.is_null() {
        if !((*reader).xsdPlug).is_null() {
            xmlSchemaSAXUnplug((*reader).xsdPlug);
            let ref mut fresh149 = (*reader).xsdPlug;
            *fresh149 = 0 as xmlSchemaSAXPlugPtr;
        }
        if !((*reader).xsdValidCtxt).is_null() {
            if (*reader).xsdPreserveCtxt == 0 {
                xmlSchemaFreeValidCtxt((*reader).xsdValidCtxt);
            }
            let ref mut fresh150 = (*reader).xsdValidCtxt;
            *fresh150 = 0 as xmlSchemaValidCtxtPtr;
        }
        (*reader).xsdPreserveCtxt = 0 as i32;
        if !((*reader).xsdSchemas).is_null() {
            xmlSchemaFree((*reader).xsdSchemas);
            let ref mut fresh151 = (*reader).xsdSchemas;
            *fresh151 = 0 as xmlSchemaPtr;
        }
        return 0 as i32;
    }
    if (*reader).mode != XML_TEXTREADER_MODE_INITIAL as i32 {
        return -(1 as i32);
    }
    if !((*reader).xsdPlug).is_null() {
        xmlSchemaSAXUnplug((*reader).xsdPlug);
        let ref mut fresh152 = (*reader).xsdPlug;
        *fresh152 = 0 as xmlSchemaSAXPlugPtr;
    }
    if !((*reader).xsdValidCtxt).is_null() {
        if (*reader).xsdPreserveCtxt == 0 {
            xmlSchemaFreeValidCtxt((*reader).xsdValidCtxt);
        }
        let ref mut fresh153 = (*reader).xsdValidCtxt;
        *fresh153 = 0 as xmlSchemaValidCtxtPtr;
    }
    (*reader).xsdPreserveCtxt = 0 as i32;
    if !((*reader).xsdSchemas).is_null() {
        xmlSchemaFree((*reader).xsdSchemas);
        let ref mut fresh154 = (*reader).xsdSchemas;
        *fresh154 = 0 as xmlSchemaPtr;
    }
    let ref mut fresh155 = (*reader).xsdValidCtxt;
    *fresh155 = xmlSchemaNewValidCtxt(schema);
    if ((*reader).xsdValidCtxt).is_null() {
        xmlSchemaFree((*reader).xsdSchemas);
        let ref mut fresh156 = (*reader).xsdSchemas;
        *fresh156 = 0 as xmlSchemaPtr;
        return -(1 as i32);
    }
    let ref mut fresh157 = (*reader).xsdPlug;
    *fresh157 = xmlSchemaSAXPlug(
        (*reader).xsdValidCtxt,
        &mut (*(*reader).ctxt).sax,
        &mut (*(*reader).ctxt).userData,
    );
    if ((*reader).xsdPlug).is_null() {
        xmlSchemaFree((*reader).xsdSchemas);
        let ref mut fresh158 = (*reader).xsdSchemas;
        *fresh158 = 0 as xmlSchemaPtr;
        xmlSchemaFreeValidCtxt((*reader).xsdValidCtxt);
        let ref mut fresh159 = (*reader).xsdValidCtxt;
        *fresh159 = 0 as xmlSchemaValidCtxtPtr;
        return -(1 as i32);
    }
    xmlSchemaValidateSetLocator(
        (*reader).xsdValidCtxt,
        Some(
            xmlTextReaderLocator
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut *const i8,
                    *mut u64,
                ) -> i32,
        ),
        reader as *mut libc::c_void,
    );
    if ((*reader).errorFunc).is_some() {
        xmlSchemaSetValidErrors(
            (*reader).xsdValidCtxt,
            Some(
                xmlTextReaderValidityErrorRelay
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const i8,
                        ...
                    ) -> (),
            ),
            Some(
                xmlTextReaderValidityWarningRelay
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const i8,
                        ...
                    ) -> (),
            ),
            reader as *mut libc::c_void,
        );
    }
    if ((*reader).sErrorFunc).is_some() {
        xmlSchemaSetValidStructuredErrors(
            (*reader).xsdValidCtxt,
            Some(
                xmlTextReaderValidityStructuredRelay
                    as unsafe extern "C" fn(*mut libc::c_void, xmlErrorPtr) -> (),
            ),
            reader as *mut libc::c_void,
        );
    }
    (*reader).xsdValidErrors = 0 as i32;
    (*reader).validate = XML_TEXTREADER_VALIDATE_XSD;
    return 0 as i32;
}
unsafe extern "C" fn xmlTextReaderRelaxNGValidateInternal(
    mut reader: xmlTextReaderPtr,
    mut rng: *const i8,
    mut ctxt: xmlRelaxNGValidCtxtPtr,
    mut options: i32,
) -> i32 {
    if reader.is_null() {
        return -(1 as i32);
    }
    if !rng.is_null() && !ctxt.is_null() {
        return -(1 as i32);
    }
    if (!rng.is_null() || !ctxt.is_null())
        && ((*reader).mode != XML_TEXTREADER_MODE_INITIAL as i32
            || ((*reader).ctxt).is_null())
    {
        return -(1 as i32);
    }
    if !((*reader).rngValidCtxt).is_null() {
        if (*reader).rngPreserveCtxt == 0 {
            xmlRelaxNGFreeValidCtxt((*reader).rngValidCtxt);
        }
        let ref mut fresh160 = (*reader).rngValidCtxt;
        *fresh160 = 0 as xmlRelaxNGValidCtxtPtr;
    }
    (*reader).rngPreserveCtxt = 0 as i32;
    if !((*reader).rngSchemas).is_null() {
        xmlRelaxNGFree((*reader).rngSchemas);
        let ref mut fresh161 = (*reader).rngSchemas;
        *fresh161 = 0 as xmlRelaxNGPtr;
    }
    if rng.is_null() && ctxt.is_null() {
        return 0 as i32;
    }
    if !rng.is_null() {
        let mut pctxt: xmlRelaxNGParserCtxtPtr = 0 as *mut xmlRelaxNGParserCtxt;
        pctxt = xmlRelaxNGNewParserCtxt(rng);
        if ((*reader).errorFunc).is_some() {
            xmlRelaxNGSetParserErrors(
                pctxt,
                Some(
                    xmlTextReaderValidityErrorRelay
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *const i8,
                            ...
                        ) -> (),
                ),
                Some(
                    xmlTextReaderValidityWarningRelay
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *const i8,
                            ...
                        ) -> (),
                ),
                reader as *mut libc::c_void,
            );
        }
        if ((*reader).sErrorFunc).is_some() {
            xmlRelaxNGSetValidStructuredErrors(
                (*reader).rngValidCtxt,
                Some(
                    xmlTextReaderValidityStructuredRelay
                        as unsafe extern "C" fn(*mut libc::c_void, xmlErrorPtr) -> (),
                ),
                reader as *mut libc::c_void,
            );
        }
        let ref mut fresh162 = (*reader).rngSchemas;
        *fresh162 = xmlRelaxNGParse(pctxt);
        xmlRelaxNGFreeParserCtxt(pctxt);
        if ((*reader).rngSchemas).is_null() {
            return -(1 as i32);
        }
        let ref mut fresh163 = (*reader).rngValidCtxt;
        *fresh163 = xmlRelaxNGNewValidCtxt((*reader).rngSchemas);
        if ((*reader).rngValidCtxt).is_null() {
            xmlRelaxNGFree((*reader).rngSchemas);
            let ref mut fresh164 = (*reader).rngSchemas;
            *fresh164 = 0 as xmlRelaxNGPtr;
            return -(1 as i32);
        }
    } else {
        let ref mut fresh165 = (*reader).rngValidCtxt;
        *fresh165 = ctxt;
        (*reader).rngPreserveCtxt = 1 as i32;
    }
    if ((*reader).errorFunc).is_some() {
        xmlRelaxNGSetValidErrors(
            (*reader).rngValidCtxt,
            Some(
                xmlTextReaderValidityErrorRelay
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const i8,
                        ...
                    ) -> (),
            ),
            Some(
                xmlTextReaderValidityWarningRelay
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const i8,
                        ...
                    ) -> (),
            ),
            reader as *mut libc::c_void,
        );
    }
    if ((*reader).sErrorFunc).is_some() {
        xmlRelaxNGSetValidStructuredErrors(
            (*reader).rngValidCtxt,
            Some(
                xmlTextReaderValidityStructuredRelay
                    as unsafe extern "C" fn(*mut libc::c_void, xmlErrorPtr) -> (),
            ),
            reader as *mut libc::c_void,
        );
    }
    (*reader).rngValidErrors = 0 as i32;
    let ref mut fresh166 = (*reader).rngFullNode;
    *fresh166 = 0 as xmlNodePtr;
    (*reader).validate = XML_TEXTREADER_VALIDATE_RNG;
    return 0 as i32;
}
unsafe extern "C" fn xmlTextReaderSchemaValidateInternal(
    mut reader: xmlTextReaderPtr,
    mut xsd: *const i8,
    mut ctxt: xmlSchemaValidCtxtPtr,
    mut options: i32,
) -> i32 {
    if reader.is_null() {
        return -(1 as i32);
    }
    if !xsd.is_null() && !ctxt.is_null() {
        return -(1 as i32);
    }
    if (!xsd.is_null() || !ctxt.is_null())
        && ((*reader).mode != XML_TEXTREADER_MODE_INITIAL as i32
            || ((*reader).ctxt).is_null())
    {
        return -(1 as i32);
    }
    if !((*reader).xsdPlug).is_null() {
        xmlSchemaSAXUnplug((*reader).xsdPlug);
        let ref mut fresh167 = (*reader).xsdPlug;
        *fresh167 = 0 as xmlSchemaSAXPlugPtr;
    }
    if !((*reader).xsdValidCtxt).is_null() {
        if (*reader).xsdPreserveCtxt == 0 {
            xmlSchemaFreeValidCtxt((*reader).xsdValidCtxt);
        }
        let ref mut fresh168 = (*reader).xsdValidCtxt;
        *fresh168 = 0 as xmlSchemaValidCtxtPtr;
    }
    (*reader).xsdPreserveCtxt = 0 as i32;
    if !((*reader).xsdSchemas).is_null() {
        xmlSchemaFree((*reader).xsdSchemas);
        let ref mut fresh169 = (*reader).xsdSchemas;
        *fresh169 = 0 as xmlSchemaPtr;
    }
    if xsd.is_null() && ctxt.is_null() {
        return 0 as i32;
    }
    if !xsd.is_null() {
        let mut pctxt: xmlSchemaParserCtxtPtr = 0 as *mut xmlSchemaParserCtxt;
        pctxt = xmlSchemaNewParserCtxt(xsd);
        if ((*reader).errorFunc).is_some() {
            xmlSchemaSetParserErrors(
                pctxt,
                Some(
                    xmlTextReaderValidityErrorRelay
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *const i8,
                            ...
                        ) -> (),
                ),
                Some(
                    xmlTextReaderValidityWarningRelay
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *const i8,
                            ...
                        ) -> (),
                ),
                reader as *mut libc::c_void,
            );
        }
        let ref mut fresh170 = (*reader).xsdSchemas;
        *fresh170 = xmlSchemaParse(pctxt);
        xmlSchemaFreeParserCtxt(pctxt);
        if ((*reader).xsdSchemas).is_null() {
            return -(1 as i32);
        }
        let ref mut fresh171 = (*reader).xsdValidCtxt;
        *fresh171 = xmlSchemaNewValidCtxt((*reader).xsdSchemas);
        if ((*reader).xsdValidCtxt).is_null() {
            xmlSchemaFree((*reader).xsdSchemas);
            let ref mut fresh172 = (*reader).xsdSchemas;
            *fresh172 = 0 as xmlSchemaPtr;
            return -(1 as i32);
        }
        let ref mut fresh173 = (*reader).xsdPlug;
        *fresh173 = xmlSchemaSAXPlug(
            (*reader).xsdValidCtxt,
            &mut (*(*reader).ctxt).sax,
            &mut (*(*reader).ctxt).userData,
        );
        if ((*reader).xsdPlug).is_null() {
            xmlSchemaFree((*reader).xsdSchemas);
            let ref mut fresh174 = (*reader).xsdSchemas;
            *fresh174 = 0 as xmlSchemaPtr;
            xmlSchemaFreeValidCtxt((*reader).xsdValidCtxt);
            let ref mut fresh175 = (*reader).xsdValidCtxt;
            *fresh175 = 0 as xmlSchemaValidCtxtPtr;
            return -(1 as i32);
        }
    } else {
        let ref mut fresh176 = (*reader).xsdValidCtxt;
        *fresh176 = ctxt;
        (*reader).xsdPreserveCtxt = 1 as i32;
        let ref mut fresh177 = (*reader).xsdPlug;
        *fresh177 = xmlSchemaSAXPlug(
            (*reader).xsdValidCtxt,
            &mut (*(*reader).ctxt).sax,
            &mut (*(*reader).ctxt).userData,
        );
        if ((*reader).xsdPlug).is_null() {
            let ref mut fresh178 = (*reader).xsdValidCtxt;
            *fresh178 = 0 as xmlSchemaValidCtxtPtr;
            (*reader).xsdPreserveCtxt = 0 as i32;
            return -(1 as i32);
        }
    }
    xmlSchemaValidateSetLocator(
        (*reader).xsdValidCtxt,
        Some(
            xmlTextReaderLocator
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut *const i8,
                    *mut u64,
                ) -> i32,
        ),
        reader as *mut libc::c_void,
    );
    if ((*reader).errorFunc).is_some() {
        xmlSchemaSetValidErrors(
            (*reader).xsdValidCtxt,
            Some(
                xmlTextReaderValidityErrorRelay
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const i8,
                        ...
                    ) -> (),
            ),
            Some(
                xmlTextReaderValidityWarningRelay
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const i8,
                        ...
                    ) -> (),
            ),
            reader as *mut libc::c_void,
        );
    }
    if ((*reader).sErrorFunc).is_some() {
        xmlSchemaSetValidStructuredErrors(
            (*reader).xsdValidCtxt,
            Some(
                xmlTextReaderValidityStructuredRelay
                    as unsafe extern "C" fn(*mut libc::c_void, xmlErrorPtr) -> (),
            ),
            reader as *mut libc::c_void,
        );
    }
    (*reader).xsdValidErrors = 0 as i32;
    (*reader).validate = XML_TEXTREADER_VALIDATE_XSD;
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderSchemaValidateCtxt(
    mut reader: xmlTextReaderPtr,
    mut ctxt: xmlSchemaValidCtxtPtr,
    mut options: i32,
) -> i32 {
    return xmlTextReaderSchemaValidateInternal(
        reader,
        0 as *const i8,
        ctxt,
        options,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderSchemaValidate(
    mut reader: xmlTextReaderPtr,
    mut xsd: *const i8,
) -> i32 {
    return xmlTextReaderSchemaValidateInternal(
        reader,
        xsd,
        0 as xmlSchemaValidCtxtPtr,
        0 as i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderRelaxNGValidateCtxt(
    mut reader: xmlTextReaderPtr,
    mut ctxt: xmlRelaxNGValidCtxtPtr,
    mut options: i32,
) -> i32 {
    return xmlTextReaderRelaxNGValidateInternal(
        reader,
        0 as *const i8,
        ctxt,
        options,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderRelaxNGValidate(
    mut reader: xmlTextReaderPtr,
    mut rng: *const i8,
) -> i32 {
    return xmlTextReaderRelaxNGValidateInternal(
        reader,
        rng,
        0 as xmlRelaxNGValidCtxtPtr,
        0 as i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderIsNamespaceDecl(
    mut reader: xmlTextReaderPtr,
) -> i32 {
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    if reader.is_null() {
        return -(1 as i32);
    }
    if ((*reader).node).is_null() {
        return -(1 as i32);
    }
    if !((*reader).curnode).is_null() {
        node = (*reader).curnode;
    } else {
        node = (*reader).node;
    }
    if XML_NAMESPACE_DECL as i32 as u32
        == (*node).type_0 as u32
    {
        return 1 as i32
    } else {
        return 0 as i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderConstXmlVersion(
    mut reader: xmlTextReaderPtr,
) -> *const xmlChar {
    let mut doc: xmlDocPtr = 0 as xmlDocPtr;
    if reader.is_null() {
        return 0 as *const xmlChar;
    }
    if !((*reader).doc).is_null() {
        doc = (*reader).doc;
    } else if !((*reader).ctxt).is_null() {
        doc = (*(*reader).ctxt).myDoc;
    }
    if doc.is_null() {
        return 0 as *const xmlChar;
    }
    if ((*doc).version).is_null() {
        return 0 as *const xmlChar
    } else {
        return xmlDictLookup((*reader).dict, (*doc).version, -(1 as i32))
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderStandalone(
    mut reader: xmlTextReaderPtr,
) -> i32 {
    let mut doc: xmlDocPtr = 0 as xmlDocPtr;
    if reader.is_null() {
        return -(1 as i32);
    }
    if !((*reader).doc).is_null() {
        doc = (*reader).doc;
    } else if !((*reader).ctxt).is_null() {
        doc = (*(*reader).ctxt).myDoc;
    }
    if doc.is_null() {
        return -(1 as i32);
    }
    return (*doc).standalone;
}
unsafe extern "C" fn xmlTextReaderBuildMessage(
    mut msg: *const i8,
    mut ap: ::std::ffi::VaList,
) -> *mut i8 {
    let mut size: i32 = 0 as i32;
    let mut chars: i32 = 0;
    let mut larger: *mut i8 = 0 as *mut i8;
    let mut str: *mut i8 = 0 as *mut i8;
    let mut aq: ::std::ffi::VaListImpl;
    loop {
        aq = ap.clone();
        chars = vsnprintf(str, size as u64, msg, aq.as_va_list());
        if chars < 0 as i32 {
            (*__xmlGenericError())
                .expect(
                    "non-null function pointer",
                )(
                *__xmlGenericErrorContext(),
                b"vsnprintf failed !\n\0" as *const u8 as *const i8,
            );
            if !str.is_null() {
                xmlFree.expect("non-null function pointer")(str as *mut libc::c_void);
            }
            return 0 as *mut i8;
        }
        if chars < size || size == 64000 as i32 {
            break;
        }
        if chars < 64000 as i32 {
            size = chars + 1 as i32;
        } else {
            size = 64000 as i32;
        }
        larger = xmlRealloc
            .expect(
                "non-null function pointer",
            )(str as *mut libc::c_void, size as size_t) as *mut i8;
        if larger.is_null() {
            (*__xmlGenericError())
                .expect(
                    "non-null function pointer",
                )(
                *__xmlGenericErrorContext(),
                b"xmlRealloc failed !\n\0" as *const u8 as *const i8,
            );
            if !str.is_null() {
                xmlFree.expect("non-null function pointer")(str as *mut libc::c_void);
            }
            return 0 as *mut i8;
        }
        str = larger;
    }
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderLocatorLineNumber(
    mut locator: xmlTextReaderLocatorPtr,
) -> i32 {
    let mut ctx: xmlParserCtxtPtr = locator as xmlParserCtxtPtr;
    let mut ret: i32 = -(1 as i32);
    if locator.is_null() {
        return -(1 as i32);
    }
    if !((*ctx).node).is_null() {
        ret = xmlGetLineNo((*ctx).node as *const xmlNode) as i32;
    } else {
        let mut input: xmlParserInputPtr = 0 as *mut xmlParserInput;
        input = (*ctx).input;
        if ((*input).filename).is_null() && (*ctx).inputNr > 1 as i32 {
            input = *((*ctx).inputTab)
                .offset(((*ctx).inputNr - 2 as i32) as isize);
        }
        if !input.is_null() {
            ret = (*input).line;
        } else {
            ret = -(1 as i32);
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderLocatorBaseURI(
    mut locator: xmlTextReaderLocatorPtr,
) -> *mut xmlChar {
    let mut ctx: xmlParserCtxtPtr = locator as xmlParserCtxtPtr;
    let mut ret: *mut xmlChar = 0 as *mut xmlChar;
    if locator.is_null() {
        return 0 as *mut xmlChar;
    }
    if !((*ctx).node).is_null() {
        ret = xmlNodeGetBase(0 as *const xmlDoc, (*ctx).node as *const xmlNode);
    } else {
        let mut input: xmlParserInputPtr = 0 as *mut xmlParserInput;
        input = (*ctx).input;
        if ((*input).filename).is_null() && (*ctx).inputNr > 1 as i32 {
            input = *((*ctx).inputTab)
                .offset(((*ctx).inputNr - 2 as i32) as isize);
        }
        if !input.is_null() {
            ret = xmlStrdup((*input).filename as *mut xmlChar);
        } else {
            ret = 0 as *mut xmlChar;
        }
    }
    return ret;
}
unsafe extern "C" fn xmlTextReaderGenericError(
    mut ctxt: *mut libc::c_void,
    mut severity: xmlParserSeverities,
    mut str: *mut i8,
) {
    let mut ctx: xmlParserCtxtPtr = ctxt as xmlParserCtxtPtr;
    let mut reader: xmlTextReaderPtr = (*ctx)._private as xmlTextReaderPtr;
    if !str.is_null() {
        if ((*reader).errorFunc).is_some() {
            ((*reader).errorFunc)
                .expect(
                    "non-null function pointer",
                )((*reader).errorFuncArg, str, severity, ctx as xmlTextReaderLocatorPtr);
        }
        xmlFree.expect("non-null function pointer")(str as *mut libc::c_void);
    }
}
unsafe extern "C" fn xmlTextReaderStructuredError(
    mut ctxt: *mut libc::c_void,
    mut error: xmlErrorPtr,
) {
    let mut ctx: xmlParserCtxtPtr = ctxt as xmlParserCtxtPtr;
    let mut reader: xmlTextReaderPtr = (*ctx)._private as xmlTextReaderPtr;
    if !error.is_null() && ((*reader).sErrorFunc).is_some() {
        ((*reader).sErrorFunc)
            .expect("non-null function pointer")((*reader).errorFuncArg, error);
    }
}
unsafe extern "C" fn xmlTextReaderError(
    mut ctxt: *mut libc::c_void,
    mut msg: *const i8,
    mut args: ...
) {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    xmlTextReaderGenericError(
        ctxt,
        XML_PARSER_SEVERITY_ERROR,
        xmlTextReaderBuildMessage(msg, ap.as_va_list()),
    );
}
unsafe extern "C" fn xmlTextReaderWarning(
    mut ctxt: *mut libc::c_void,
    mut msg: *const i8,
    mut args: ...
) {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    xmlTextReaderGenericError(
        ctxt,
        XML_PARSER_SEVERITY_WARNING,
        xmlTextReaderBuildMessage(msg, ap.as_va_list()),
    );
}
unsafe extern "C" fn xmlTextReaderValidityError(
    mut ctxt: *mut libc::c_void,
    mut msg: *const i8,
    mut args: ...
) {
    let mut ap: ::std::ffi::VaListImpl;
    let mut len: i32 = xmlStrlen(msg as *const xmlChar);
    if len > 1 as i32
        && *msg.offset((len - 2 as i32) as isize) as i32 != ':' as i32
    {
        ap = args.clone();
        xmlTextReaderGenericError(
            ctxt,
            XML_PARSER_SEVERITY_VALIDITY_ERROR,
            xmlTextReaderBuildMessage(msg, ap.as_va_list()),
        );
    }
}
unsafe extern "C" fn xmlTextReaderValidityWarning(
    mut ctxt: *mut libc::c_void,
    mut msg: *const i8,
    mut args: ...
) {
    let mut ap: ::std::ffi::VaListImpl;
    let mut len: i32 = xmlStrlen(msg as *const xmlChar);
    if len != 0 as i32
        && *msg.offset((len - 1 as i32) as isize) as i32 != ':' as i32
    {
        ap = args.clone();
        xmlTextReaderGenericError(
            ctxt,
            XML_PARSER_SEVERITY_VALIDITY_WARNING,
            xmlTextReaderBuildMessage(msg, ap.as_va_list()),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderSetErrorHandler(
    mut reader: xmlTextReaderPtr,
    mut f: xmlTextReaderErrorFunc,
    mut arg: *mut libc::c_void,
) {
    if f.is_some() {
        let ref mut fresh179 = (*(*(*reader).ctxt).sax).error;
        *fresh179 = Some(
            xmlTextReaderError
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const i8,
                    ...
                ) -> (),
        );
        let ref mut fresh180 = (*(*(*reader).ctxt).sax).serror;
        *fresh180 = None;
        let ref mut fresh181 = (*(*reader).ctxt).vctxt.error;
        *fresh181 = Some(
            xmlTextReaderValidityError
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const i8,
                    ...
                ) -> (),
        );
        let ref mut fresh182 = (*(*(*reader).ctxt).sax).warning;
        *fresh182 = Some(
            xmlTextReaderWarning
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const i8,
                    ...
                ) -> (),
        );
        let ref mut fresh183 = (*(*reader).ctxt).vctxt.warning;
        *fresh183 = Some(
            xmlTextReaderValidityWarning
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const i8,
                    ...
                ) -> (),
        );
        let ref mut fresh184 = (*reader).errorFunc;
        *fresh184 = f;
        let ref mut fresh185 = (*reader).sErrorFunc;
        *fresh185 = None;
        let ref mut fresh186 = (*reader).errorFuncArg;
        *fresh186 = arg;
        if !((*reader).rngValidCtxt).is_null() {
            xmlRelaxNGSetValidErrors(
                (*reader).rngValidCtxt,
                Some(
                    xmlTextReaderValidityErrorRelay
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *const i8,
                            ...
                        ) -> (),
                ),
                Some(
                    xmlTextReaderValidityWarningRelay
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *const i8,
                            ...
                        ) -> (),
                ),
                reader as *mut libc::c_void,
            );
            xmlRelaxNGSetValidStructuredErrors(
                (*reader).rngValidCtxt,
                None,
                reader as *mut libc::c_void,
            );
        }
        if !((*reader).xsdValidCtxt).is_null() {
            xmlSchemaSetValidErrors(
                (*reader).xsdValidCtxt,
                Some(
                    xmlTextReaderValidityErrorRelay
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *const i8,
                            ...
                        ) -> (),
                ),
                Some(
                    xmlTextReaderValidityWarningRelay
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *const i8,
                            ...
                        ) -> (),
                ),
                reader as *mut libc::c_void,
            );
            xmlSchemaSetValidStructuredErrors(
                (*reader).xsdValidCtxt,
                None,
                reader as *mut libc::c_void,
            );
        }
    } else {
        let ref mut fresh187 = (*(*(*reader).ctxt).sax).error;
        *fresh187 = Some(
            xmlParserError
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const i8,
                    ...
                ) -> (),
        );
        let ref mut fresh188 = (*(*reader).ctxt).vctxt.error;
        *fresh188 = Some(
            xmlParserValidityError
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const i8,
                    ...
                ) -> (),
        );
        let ref mut fresh189 = (*(*(*reader).ctxt).sax).warning;
        *fresh189 = Some(
            xmlParserWarning
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const i8,
                    ...
                ) -> (),
        );
        let ref mut fresh190 = (*(*reader).ctxt).vctxt.warning;
        *fresh190 = Some(
            xmlParserValidityWarning
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const i8,
                    ...
                ) -> (),
        );
        let ref mut fresh191 = (*reader).errorFunc;
        *fresh191 = None;
        let ref mut fresh192 = (*reader).sErrorFunc;
        *fresh192 = None;
        let ref mut fresh193 = (*reader).errorFuncArg;
        *fresh193 = 0 as *mut libc::c_void;
        if !((*reader).rngValidCtxt).is_null() {
            xmlRelaxNGSetValidErrors(
                (*reader).rngValidCtxt,
                None,
                None,
                reader as *mut libc::c_void,
            );
            xmlRelaxNGSetValidStructuredErrors(
                (*reader).rngValidCtxt,
                None,
                reader as *mut libc::c_void,
            );
        }
        if !((*reader).xsdValidCtxt).is_null() {
            xmlSchemaSetValidErrors(
                (*reader).xsdValidCtxt,
                None,
                None,
                reader as *mut libc::c_void,
            );
            xmlSchemaSetValidStructuredErrors(
                (*reader).xsdValidCtxt,
                None,
                reader as *mut libc::c_void,
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderSetStructuredErrorHandler(
    mut reader: xmlTextReaderPtr,
    mut f: xmlStructuredErrorFunc,
    mut arg: *mut libc::c_void,
) {
    if f.is_some() {
        let ref mut fresh194 = (*(*(*reader).ctxt).sax).error;
        *fresh194 = None;
        let ref mut fresh195 = (*(*(*reader).ctxt).sax).serror;
        *fresh195 = Some(
            xmlTextReaderStructuredError
                as unsafe extern "C" fn(*mut libc::c_void, xmlErrorPtr) -> (),
        );
        let ref mut fresh196 = (*(*reader).ctxt).vctxt.error;
        *fresh196 = Some(
            xmlTextReaderValidityError
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const i8,
                    ...
                ) -> (),
        );
        let ref mut fresh197 = (*(*(*reader).ctxt).sax).warning;
        *fresh197 = Some(
            xmlTextReaderWarning
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const i8,
                    ...
                ) -> (),
        );
        let ref mut fresh198 = (*(*reader).ctxt).vctxt.warning;
        *fresh198 = Some(
            xmlTextReaderValidityWarning
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const i8,
                    ...
                ) -> (),
        );
        let ref mut fresh199 = (*reader).sErrorFunc;
        *fresh199 = f;
        let ref mut fresh200 = (*reader).errorFunc;
        *fresh200 = None;
        let ref mut fresh201 = (*reader).errorFuncArg;
        *fresh201 = arg;
        if !((*reader).rngValidCtxt).is_null() {
            xmlRelaxNGSetValidErrors(
                (*reader).rngValidCtxt,
                None,
                None,
                reader as *mut libc::c_void,
            );
            xmlRelaxNGSetValidStructuredErrors(
                (*reader).rngValidCtxt,
                Some(
                    xmlTextReaderValidityStructuredRelay
                        as unsafe extern "C" fn(*mut libc::c_void, xmlErrorPtr) -> (),
                ),
                reader as *mut libc::c_void,
            );
        }
        if !((*reader).xsdValidCtxt).is_null() {
            xmlSchemaSetValidErrors(
                (*reader).xsdValidCtxt,
                None,
                None,
                reader as *mut libc::c_void,
            );
            xmlSchemaSetValidStructuredErrors(
                (*reader).xsdValidCtxt,
                Some(
                    xmlTextReaderValidityStructuredRelay
                        as unsafe extern "C" fn(*mut libc::c_void, xmlErrorPtr) -> (),
                ),
                reader as *mut libc::c_void,
            );
        }
    } else {
        let ref mut fresh202 = (*(*(*reader).ctxt).sax).error;
        *fresh202 = Some(
            xmlParserError
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const i8,
                    ...
                ) -> (),
        );
        let ref mut fresh203 = (*(*(*reader).ctxt).sax).serror;
        *fresh203 = None;
        let ref mut fresh204 = (*(*reader).ctxt).vctxt.error;
        *fresh204 = Some(
            xmlParserValidityError
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const i8,
                    ...
                ) -> (),
        );
        let ref mut fresh205 = (*(*(*reader).ctxt).sax).warning;
        *fresh205 = Some(
            xmlParserWarning
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const i8,
                    ...
                ) -> (),
        );
        let ref mut fresh206 = (*(*reader).ctxt).vctxt.warning;
        *fresh206 = Some(
            xmlParserValidityWarning
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const i8,
                    ...
                ) -> (),
        );
        let ref mut fresh207 = (*reader).errorFunc;
        *fresh207 = None;
        let ref mut fresh208 = (*reader).sErrorFunc;
        *fresh208 = None;
        let ref mut fresh209 = (*reader).errorFuncArg;
        *fresh209 = 0 as *mut libc::c_void;
        if !((*reader).rngValidCtxt).is_null() {
            xmlRelaxNGSetValidErrors(
                (*reader).rngValidCtxt,
                None,
                None,
                reader as *mut libc::c_void,
            );
            xmlRelaxNGSetValidStructuredErrors(
                (*reader).rngValidCtxt,
                None,
                reader as *mut libc::c_void,
            );
        }
        if !((*reader).xsdValidCtxt).is_null() {
            xmlSchemaSetValidErrors(
                (*reader).xsdValidCtxt,
                None,
                None,
                reader as *mut libc::c_void,
            );
            xmlSchemaSetValidStructuredErrors(
                (*reader).xsdValidCtxt,
                None,
                reader as *mut libc::c_void,
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderIsValid(
    mut reader: xmlTextReaderPtr,
) -> i32 {
    if reader.is_null() {
        return -(1 as i32);
    }
    if (*reader).validate as u32
        == XML_TEXTREADER_VALIDATE_RNG as i32 as u32
    {
        return ((*reader).rngValidErrors == 0 as i32) as i32;
    }
    if (*reader).validate as u32
        == XML_TEXTREADER_VALIDATE_XSD as i32 as u32
    {
        return ((*reader).xsdValidErrors == 0 as i32) as i32;
    }
    if !((*reader).ctxt).is_null() && (*(*reader).ctxt).validate == 1 as i32 {
        return (*(*reader).ctxt).valid;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderGetErrorHandler(
    mut reader: xmlTextReaderPtr,
    mut f: *mut xmlTextReaderErrorFunc,
    mut arg: *mut *mut libc::c_void,
) {
    if !f.is_null() {
        *f = (*reader).errorFunc;
    }
    if !arg.is_null() {
        *arg = (*reader).errorFuncArg;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderSetup(
    mut reader: xmlTextReaderPtr,
    mut input: xmlParserInputBufferPtr,
    mut URL: *const i8,
    mut encoding: *const i8,
    mut options: i32,
) -> i32 {
    if reader.is_null() {
        if !input.is_null() {
            xmlFreeParserInputBuffer(input);
        }
        return -(1 as i32);
    }
    options |= XML_PARSE_COMPACT as i32;
    let ref mut fresh210 = (*reader).doc;
    *fresh210 = 0 as xmlDocPtr;
    (*reader).entNr = 0 as i32;
    (*reader).parserFlags = options;
    (*reader).validate = XML_TEXTREADER_NOT_VALIDATE;
    if !input.is_null() && !((*reader).input).is_null()
        && (*reader).allocs & 1 as i32 != 0
    {
        xmlFreeParserInputBuffer((*reader).input);
        let ref mut fresh211 = (*reader).input;
        *fresh211 = 0 as xmlParserInputBufferPtr;
        (*reader).allocs -= 1 as i32;
    }
    if !input.is_null() {
        let ref mut fresh212 = (*reader).input;
        *fresh212 = input;
        (*reader).allocs |= 1 as i32;
    }
    if ((*reader).buffer).is_null() {
        let ref mut fresh213 = (*reader).buffer;
        *fresh213 = xmlBufCreateSize(100 as i32 as size_t);
    }
    if ((*reader).buffer).is_null() {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"xmlTextReaderSetup : malloc failed\n\0" as *const u8 as *const i8,
        );
        return -(1 as i32);
    }
    xmlBufSetAllocationScheme((*reader).buffer, XML_BUFFER_ALLOC_DOUBLEIT);
    if ((*reader).sax).is_null() {
        let ref mut fresh214 = (*reader).sax;
        *fresh214 = xmlMalloc
            .expect(
                "non-null function pointer",
            )(::std::mem::size_of::<xmlSAXHandler>() as u64)
            as *mut xmlSAXHandler;
    }
    if ((*reader).sax).is_null() {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"xmlTextReaderSetup : malloc failed\n\0" as *const u8 as *const i8,
        );
        return -(1 as i32);
    }
    xmlSAXVersion((*reader).sax, 2 as i32);
    let ref mut fresh215 = (*reader).startElement;
    *fresh215 = (*(*reader).sax).startElement;
    let ref mut fresh216 = (*(*reader).sax).startElement;
    *fresh216 = Some(
        xmlTextReaderStartElement
            as unsafe extern "C" fn(
                *mut libc::c_void,
                *const xmlChar,
                *mut *const xmlChar,
            ) -> (),
    );
    let ref mut fresh217 = (*reader).endElement;
    *fresh217 = (*(*reader).sax).endElement;
    let ref mut fresh218 = (*(*reader).sax).endElement;
    *fresh218 = Some(
        xmlTextReaderEndElement
            as unsafe extern "C" fn(*mut libc::c_void, *const xmlChar) -> (),
    );
    if (*(*reader).sax).initialized == 0xdeedbeaf as u32 {
        let ref mut fresh219 = (*reader).startElementNs;
        *fresh219 = (*(*reader).sax).startElementNs;
        let ref mut fresh220 = (*(*reader).sax).startElementNs;
        *fresh220 = Some(
            xmlTextReaderStartElementNs
                as unsafe extern "C" fn(
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
        );
        let ref mut fresh221 = (*reader).endElementNs;
        *fresh221 = (*(*reader).sax).endElementNs;
        let ref mut fresh222 = (*(*reader).sax).endElementNs;
        *fresh222 = Some(
            xmlTextReaderEndElementNs
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const xmlChar,
                    *const xmlChar,
                    *const xmlChar,
                ) -> (),
        );
    } else {
        let ref mut fresh223 = (*reader).startElementNs;
        *fresh223 = None;
        let ref mut fresh224 = (*reader).endElementNs;
        *fresh224 = None;
    }
    let ref mut fresh225 = (*reader).characters;
    *fresh225 = (*(*reader).sax).characters;
    let ref mut fresh226 = (*(*reader).sax).characters;
    *fresh226 = Some(
        xmlTextReaderCharacters
            as unsafe extern "C" fn(*mut libc::c_void, *const xmlChar, i32) -> (),
    );
    let ref mut fresh227 = (*(*reader).sax).ignorableWhitespace;
    *fresh227 = Some(
        xmlTextReaderCharacters
            as unsafe extern "C" fn(*mut libc::c_void, *const xmlChar, i32) -> (),
    );
    let ref mut fresh228 = (*reader).cdataBlock;
    *fresh228 = (*(*reader).sax).cdataBlock;
    let ref mut fresh229 = (*(*reader).sax).cdataBlock;
    *fresh229 = Some(
        xmlTextReaderCDataBlock
            as unsafe extern "C" fn(*mut libc::c_void, *const xmlChar, i32) -> (),
    );
    (*reader).mode = XML_TEXTREADER_MODE_INITIAL as i32;
    let ref mut fresh230 = (*reader).node;
    *fresh230 = 0 as xmlNodePtr;
    let ref mut fresh231 = (*reader).curnode;
    *fresh231 = 0 as xmlNodePtr;
    if !input.is_null() {
        if xmlBufUse((*(*reader).input).buffer) < 4 as i32 as u64 {
            xmlParserInputBufferRead(input, 4 as i32);
        }
        if ((*reader).ctxt).is_null() {
            if xmlBufUse((*(*reader).input).buffer) >= 4 as i32 as u64
            {
                let ref mut fresh232 = (*reader).ctxt;
                *fresh232 = xmlCreatePushParserCtxt(
                    (*reader).sax,
                    0 as *mut libc::c_void,
                    xmlBufContent((*(*reader).input).buffer as *const xmlBuf)
                        as *const i8,
                    4 as i32,
                    URL,
                );
                (*reader).base = 0 as i32 as u32;
                (*reader).cur = 4 as i32 as u32;
            } else {
                let ref mut fresh233 = (*reader).ctxt;
                *fresh233 = xmlCreatePushParserCtxt(
                    (*reader).sax,
                    0 as *mut libc::c_void,
                    0 as *const i8,
                    0 as i32,
                    URL,
                );
                (*reader).base = 0 as i32 as u32;
                (*reader).cur = 0 as i32 as u32;
            }
        } else {
            let mut inputStream: xmlParserInputPtr = 0 as *mut xmlParserInput;
            let mut buf: xmlParserInputBufferPtr = 0 as *mut xmlParserInputBuffer;
            let mut enc: xmlCharEncoding = XML_CHAR_ENCODING_NONE;
            xmlCtxtReset((*reader).ctxt);
            buf = xmlAllocParserInputBuffer(enc);
            if buf.is_null() {
                return -(1 as i32);
            }
            inputStream = xmlNewInputStream((*reader).ctxt);
            if inputStream.is_null() {
                xmlFreeParserInputBuffer(buf);
                return -(1 as i32);
            }
            if URL.is_null() {
                let ref mut fresh234 = (*inputStream).filename;
                *fresh234 = 0 as *const i8;
            } else {
                let ref mut fresh235 = (*inputStream).filename;
                *fresh235 = xmlCanonicPath(URL as *const xmlChar) as *mut i8;
            }
            let ref mut fresh236 = (*inputStream).buf;
            *fresh236 = buf;
            xmlBufResetInput((*buf).buffer, inputStream);
            inputPush((*reader).ctxt, inputStream);
            (*reader).cur = 0 as i32 as u32;
        }
        if ((*reader).ctxt).is_null() {
            (*__xmlGenericError())
                .expect(
                    "non-null function pointer",
                )(
                *__xmlGenericErrorContext(),
                b"xmlTextReaderSetup : malloc failed\n\0" as *const u8
                    as *const i8,
            );
            return -(1 as i32);
        }
    }
    if !((*reader).dict).is_null() {
        if !((*(*reader).ctxt).dict).is_null() {
            if (*reader).dict != (*(*reader).ctxt).dict {
                xmlDictFree((*reader).dict);
                let ref mut fresh237 = (*reader).dict;
                *fresh237 = (*(*reader).ctxt).dict;
            }
        } else {
            let ref mut fresh238 = (*(*reader).ctxt).dict;
            *fresh238 = (*reader).dict;
        }
    } else {
        if ((*(*reader).ctxt).dict).is_null() {
            let ref mut fresh239 = (*(*reader).ctxt).dict;
            *fresh239 = xmlDictCreate();
        }
        let ref mut fresh240 = (*reader).dict;
        *fresh240 = (*(*reader).ctxt).dict;
    }
    let ref mut fresh241 = (*(*reader).ctxt)._private;
    *fresh241 = reader as *mut libc::c_void;
    (*(*reader).ctxt).linenumbers = 1 as i32;
    (*(*reader).ctxt).dictNames = 1 as i32;
    (*(*reader).ctxt).docdict = 1 as i32;
    (*(*reader).ctxt).parseMode = XML_PARSE_READER;
    if !((*reader).xincctxt).is_null() {
        xmlXIncludeFreeContext((*reader).xincctxt);
        let ref mut fresh242 = (*reader).xincctxt;
        *fresh242 = 0 as xmlXIncludeCtxtPtr;
    }
    if options & XML_PARSE_XINCLUDE as i32 != 0 {
        (*reader).xinclude = 1 as i32;
        let ref mut fresh243 = (*reader).xinclude_name;
        *fresh243 = xmlDictLookup(
            (*reader).dict,
            b"include\0" as *const u8 as *const i8 as *const xmlChar,
            -(1 as i32),
        );
        options -= XML_PARSE_XINCLUDE as i32;
    } else {
        (*reader).xinclude = 0 as i32;
    }
    (*reader).in_xinclude = 0 as i32;
    if ((*reader).patternTab).is_null() {
        (*reader).patternNr = 0 as i32;
        (*reader).patternMax = 0 as i32;
    }
    while (*reader).patternNr > 0 as i32 {
        let ref mut fresh244 = (*reader).patternNr;
        *fresh244 -= 1;
        if !(*((*reader).patternTab).offset((*reader).patternNr as isize)).is_null() {
            xmlFreePattern(*((*reader).patternTab).offset((*reader).patternNr as isize));
            let ref mut fresh245 = *((*reader).patternTab)
                .offset((*reader).patternNr as isize);
            *fresh245 = 0 as xmlPatternPtr;
        }
    }
    if options & XML_PARSE_DTDVALID as i32 != 0 {
        (*reader).validate = XML_TEXTREADER_VALIDATE_DTD;
    }
    xmlCtxtUseOptions((*reader).ctxt, options);
    if !encoding.is_null() {
        let mut hdlr: xmlCharEncodingHandlerPtr = 0 as *mut xmlCharEncodingHandler;
        hdlr = xmlFindCharEncodingHandler(encoding);
        if !hdlr.is_null() {
            xmlSwitchToEncoding((*reader).ctxt, hdlr);
        }
    }
    if !URL.is_null() && !((*(*reader).ctxt).input).is_null()
        && ((*(*(*reader).ctxt).input).filename).is_null()
    {
        let ref mut fresh246 = (*(*(*reader).ctxt).input).filename;
        *fresh246 = xmlStrdup(URL as *const xmlChar) as *mut i8;
    }
    let ref mut fresh247 = (*reader).doc;
    *fresh247 = 0 as xmlDocPtr;
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn xmlTextReaderByteConsumed(
    mut reader: xmlTextReaderPtr,
) -> i64 {
    if reader.is_null() || ((*reader).ctxt).is_null() {
        return -(1 as i32) as i64;
    }
    return xmlByteConsumed((*reader).ctxt);
}
#[no_mangle]
pub unsafe extern "C" fn xmlReaderWalker(mut doc: xmlDocPtr) -> xmlTextReaderPtr {
    let mut ret: xmlTextReaderPtr = 0 as *mut xmlTextReader;
    if doc.is_null() {
        return 0 as xmlTextReaderPtr;
    }
    ret = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::std::mem::size_of::<xmlTextReader>() as u64) as xmlTextReaderPtr;
    if ret.is_null() {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"xmlNewTextReader : malloc failed\n\0" as *const u8 as *const i8,
        );
        return 0 as xmlTextReaderPtr;
    }
    memset(
        ret as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<xmlTextReader>() as u64,
    );
    (*ret).entNr = 0 as i32;
    let ref mut fresh248 = (*ret).input;
    *fresh248 = 0 as xmlParserInputBufferPtr;
    (*ret).mode = XML_TEXTREADER_MODE_INITIAL as i32;
    let ref mut fresh249 = (*ret).node;
    *fresh249 = 0 as xmlNodePtr;
    let ref mut fresh250 = (*ret).curnode;
    *fresh250 = 0 as xmlNodePtr;
    (*ret).base = 0 as i32 as u32;
    (*ret).cur = 0 as i32 as u32;
    (*ret).allocs = 2 as i32;
    let ref mut fresh251 = (*ret).doc;
    *fresh251 = doc;
    (*ret).state = XML_TEXTREADER_START;
    let ref mut fresh252 = (*ret).dict;
    *fresh252 = xmlDictCreate();
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlReaderForDoc(
    mut cur: *const xmlChar,
    mut URL: *const i8,
    mut encoding: *const i8,
    mut options: i32,
) -> xmlTextReaderPtr {
    let mut len: i32 = 0;
    if cur.is_null() {
        return 0 as xmlTextReaderPtr;
    }
    len = xmlStrlen(cur);
    return xmlReaderForMemory(cur as *const i8, len, URL, encoding, options);
}
#[no_mangle]
pub unsafe extern "C" fn xmlReaderForFile(
    mut filename: *const i8,
    mut encoding: *const i8,
    mut options: i32,
) -> xmlTextReaderPtr {
    let mut reader: xmlTextReaderPtr = 0 as *mut xmlTextReader;
    reader = xmlNewTextReaderFilename(filename);
    if reader.is_null() {
        return 0 as xmlTextReaderPtr;
    }
    xmlTextReaderSetup(
        reader,
        0 as xmlParserInputBufferPtr,
        0 as *const i8,
        encoding,
        options,
    );
    return reader;
}
#[no_mangle]
pub unsafe extern "C" fn xmlReaderForMemory(
    mut buffer: *const i8,
    mut size: i32,
    mut URL: *const i8,
    mut encoding: *const i8,
    mut options: i32,
) -> xmlTextReaderPtr {
    let mut reader: xmlTextReaderPtr = 0 as *mut xmlTextReader;
    let mut buf: xmlParserInputBufferPtr = 0 as *mut xmlParserInputBuffer;
    buf = xmlParserInputBufferCreateStatic(buffer, size, XML_CHAR_ENCODING_NONE);
    if buf.is_null() {
        return 0 as xmlTextReaderPtr;
    }
    reader = xmlNewTextReader(buf, URL);
    if reader.is_null() {
        xmlFreeParserInputBuffer(buf);
        return 0 as xmlTextReaderPtr;
    }
    (*reader).allocs |= 1 as i32;
    xmlTextReaderSetup(reader, 0 as xmlParserInputBufferPtr, URL, encoding, options);
    return reader;
}
#[no_mangle]
pub unsafe extern "C" fn xmlReaderForFd(
    mut fd: i32,
    mut URL: *const i8,
    mut encoding: *const i8,
    mut options: i32,
) -> xmlTextReaderPtr {
    let mut reader: xmlTextReaderPtr = 0 as *mut xmlTextReader;
    let mut input: xmlParserInputBufferPtr = 0 as *mut xmlParserInputBuffer;
    if fd < 0 as i32 {
        return 0 as xmlTextReaderPtr;
    }
    input = xmlParserInputBufferCreateFd(fd, XML_CHAR_ENCODING_NONE);
    if input.is_null() {
        return 0 as xmlTextReaderPtr;
    }
    let ref mut fresh253 = (*input).closecallback;
    *fresh253 = None;
    reader = xmlNewTextReader(input, URL);
    if reader.is_null() {
        xmlFreeParserInputBuffer(input);
        return 0 as xmlTextReaderPtr;
    }
    (*reader).allocs |= 1 as i32;
    xmlTextReaderSetup(reader, 0 as xmlParserInputBufferPtr, URL, encoding, options);
    return reader;
}
#[no_mangle]
pub unsafe extern "C" fn xmlReaderForIO(
    mut ioread: xmlInputReadCallback,
    mut ioclose: xmlInputCloseCallback,
    mut ioctx: *mut libc::c_void,
    mut URL: *const i8,
    mut encoding: *const i8,
    mut options: i32,
) -> xmlTextReaderPtr {
    let mut reader: xmlTextReaderPtr = 0 as *mut xmlTextReader;
    let mut input: xmlParserInputBufferPtr = 0 as *mut xmlParserInputBuffer;
    if ioread.is_none() {
        return 0 as xmlTextReaderPtr;
    }
    input = xmlParserInputBufferCreateIO(ioread, ioclose, ioctx, XML_CHAR_ENCODING_NONE);
    if input.is_null() {
        if ioclose.is_some() {
            ioclose.expect("non-null function pointer")(ioctx);
        }
        return 0 as xmlTextReaderPtr;
    }
    reader = xmlNewTextReader(input, URL);
    if reader.is_null() {
        xmlFreeParserInputBuffer(input);
        return 0 as xmlTextReaderPtr;
    }
    (*reader).allocs |= 1 as i32;
    xmlTextReaderSetup(reader, 0 as xmlParserInputBufferPtr, URL, encoding, options);
    return reader;
}
#[no_mangle]
pub unsafe extern "C" fn xmlReaderNewWalker(
    mut reader: xmlTextReaderPtr,
    mut doc: xmlDocPtr,
) -> i32 {
    if doc.is_null() {
        return -(1 as i32);
    }
    if reader.is_null() {
        return -(1 as i32);
    }
    if !((*reader).input).is_null() {
        xmlFreeParserInputBuffer((*reader).input);
    }
    if !((*reader).ctxt).is_null() {
        xmlCtxtReset((*reader).ctxt);
    }
    (*reader).entNr = 0 as i32;
    let ref mut fresh254 = (*reader).input;
    *fresh254 = 0 as xmlParserInputBufferPtr;
    (*reader).mode = XML_TEXTREADER_MODE_INITIAL as i32;
    let ref mut fresh255 = (*reader).node;
    *fresh255 = 0 as xmlNodePtr;
    let ref mut fresh256 = (*reader).curnode;
    *fresh256 = 0 as xmlNodePtr;
    (*reader).base = 0 as i32 as u32;
    (*reader).cur = 0 as i32 as u32;
    (*reader).allocs = 2 as i32;
    let ref mut fresh257 = (*reader).doc;
    *fresh257 = doc;
    (*reader).state = XML_TEXTREADER_START;
    if ((*reader).dict).is_null() {
        if !((*reader).ctxt).is_null() && !((*(*reader).ctxt).dict).is_null() {
            let ref mut fresh258 = (*reader).dict;
            *fresh258 = (*(*reader).ctxt).dict;
        } else {
            let ref mut fresh259 = (*reader).dict;
            *fresh259 = xmlDictCreate();
        }
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn xmlReaderNewDoc(
    mut reader: xmlTextReaderPtr,
    mut cur: *const xmlChar,
    mut URL: *const i8,
    mut encoding: *const i8,
    mut options: i32,
) -> i32 {
    let mut len: i32 = 0;
    if cur.is_null() {
        return -(1 as i32);
    }
    if reader.is_null() {
        return -(1 as i32);
    }
    len = xmlStrlen(cur);
    return xmlReaderNewMemory(
        reader,
        cur as *const i8,
        len,
        URL,
        encoding,
        options,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlReaderNewFile(
    mut reader: xmlTextReaderPtr,
    mut filename: *const i8,
    mut encoding: *const i8,
    mut options: i32,
) -> i32 {
    let mut input: xmlParserInputBufferPtr = 0 as *mut xmlParserInputBuffer;
    if filename.is_null() {
        return -(1 as i32);
    }
    if reader.is_null() {
        return -(1 as i32);
    }
    input = xmlParserInputBufferCreateFilename(filename, XML_CHAR_ENCODING_NONE);
    if input.is_null() {
        return -(1 as i32);
    }
    return xmlTextReaderSetup(reader, input, filename, encoding, options);
}
#[no_mangle]
pub unsafe extern "C" fn xmlReaderNewMemory(
    mut reader: xmlTextReaderPtr,
    mut buffer: *const i8,
    mut size: i32,
    mut URL: *const i8,
    mut encoding: *const i8,
    mut options: i32,
) -> i32 {
    let mut input: xmlParserInputBufferPtr = 0 as *mut xmlParserInputBuffer;
    if reader.is_null() {
        return -(1 as i32);
    }
    if buffer.is_null() {
        return -(1 as i32);
    }
    input = xmlParserInputBufferCreateStatic(buffer, size, XML_CHAR_ENCODING_NONE);
    if input.is_null() {
        return -(1 as i32);
    }
    return xmlTextReaderSetup(reader, input, URL, encoding, options);
}
#[no_mangle]
pub unsafe extern "C" fn xmlReaderNewFd(
    mut reader: xmlTextReaderPtr,
    mut fd: i32,
    mut URL: *const i8,
    mut encoding: *const i8,
    mut options: i32,
) -> i32 {
    let mut input: xmlParserInputBufferPtr = 0 as *mut xmlParserInputBuffer;
    if fd < 0 as i32 {
        return -(1 as i32);
    }
    if reader.is_null() {
        return -(1 as i32);
    }
    input = xmlParserInputBufferCreateFd(fd, XML_CHAR_ENCODING_NONE);
    if input.is_null() {
        return -(1 as i32);
    }
    let ref mut fresh260 = (*input).closecallback;
    *fresh260 = None;
    return xmlTextReaderSetup(reader, input, URL, encoding, options);
}
#[no_mangle]
pub unsafe extern "C" fn xmlReaderNewIO(
    mut reader: xmlTextReaderPtr,
    mut ioread: xmlInputReadCallback,
    mut ioclose: xmlInputCloseCallback,
    mut ioctx: *mut libc::c_void,
    mut URL: *const i8,
    mut encoding: *const i8,
    mut options: i32,
) -> i32 {
    let mut input: xmlParserInputBufferPtr = 0 as *mut xmlParserInputBuffer;
    if ioread.is_none() {
        return -(1 as i32);
    }
    if reader.is_null() {
        return -(1 as i32);
    }
    input = xmlParserInputBufferCreateIO(ioread, ioclose, ioctx, XML_CHAR_ENCODING_NONE);
    if input.is_null() {
        if ioclose.is_some() {
            ioclose.expect("non-null function pointer")(ioctx);
        }
        return -(1 as i32);
    }
    return xmlTextReaderSetup(reader, input, URL, encoding, options);
}
