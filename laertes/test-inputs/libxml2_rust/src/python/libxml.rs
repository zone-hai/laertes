use ::libc;
extern "C" {
    pub type _IO_wide_data;
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
    pub type _xmlXPathCompExpr;
    pub type _xmlRelaxNGValidCtxt;
    pub type _xmlSchemaValidCtxt;
    pub type _xmlTextReader;
    pub type _xmlSaveCtxt;
    fn PyType_IsSubtype(_: *mut PyTypeObject, _: *mut PyTypeObject) -> libc::c_int;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
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
    fn PyUnicodeUCS4_AsUTF8String(unicode: *mut PyObject) -> *mut PyObject;
    fn PyLong_FromLong(_: libc::c_long) -> *mut PyObject;
    fn PyLong_AsLong(_: *mut PyObject) -> libc::c_long;
    fn PyLong_FromVoidPtr(_: *mut libc::c_void) -> *mut PyObject;
    fn PyString_FromStringAndSize(
        _: *const libc::c_char,
        _: Py_ssize_t,
    ) -> *mut PyObject;
    fn PyString_FromString(_: *const libc::c_char) -> *mut PyObject;
    fn PyString_Size(_: *mut PyObject) -> Py_ssize_t;
    fn PyString_AsString(_: *mut PyObject) -> *mut libc::c_char;
    fn PyTuple_New(size: Py_ssize_t) -> *mut PyObject;
    fn PyTuple_SetItem(_: *mut PyObject, _: Py_ssize_t, _: *mut PyObject) -> libc::c_int;
    fn PyList_New(size: Py_ssize_t) -> *mut PyObject;
    fn PyList_SetItem(_: *mut PyObject, _: Py_ssize_t, _: *mut PyObject) -> libc::c_int;
    fn PyDict_New() -> *mut PyObject;
    fn PyDict_SetItem(
        mp: *mut PyObject,
        key: *mut PyObject,
        item: *mut PyObject,
    ) -> libc::c_int;
    static mut PyFile_Type: PyTypeObject;
    fn PyFile_AsFile(_: *mut PyObject) -> *mut FILE;
    static mut PyCapsule_Type: PyTypeObject;
    fn PyCapsule_GetPointer(
        capsule: *mut PyObject,
        name: *const libc::c_char,
    ) -> *mut libc::c_void;
    fn PyCapsule_GetName(capsule: *mut PyObject) -> *const libc::c_char;
    fn PyErr_WarnEx(
        _: *mut PyObject,
        _: *const libc::c_char,
        _: Py_ssize_t,
    ) -> libc::c_int;
    fn PyErr_SetString(_: *mut PyObject, _: *const libc::c_char);
    fn PyErr_Occurred() -> *mut PyObject;
    static mut PyExc_Exception: *mut PyObject;
    static mut PyExc_AssertionError: *mut PyObject;
    static mut PyExc_IndexError: *mut PyObject;
    static mut PyExc_MemoryError: *mut PyObject;
    static mut PyExc_TypeError: *mut PyObject;
    static mut PyExc_ValueError: *mut PyObject;
    static mut PyExc_PendingDeprecationWarning: *mut PyObject;
    fn PyErr_NoMemory() -> *mut PyObject;
    fn _PyArg_ParseTuple_SizeT(
        _: *mut PyObject,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn _Py_BuildValue_SizeT(_: *const libc::c_char, _: ...) -> *mut PyObject;
    fn Py_InitModule4_64(
        name: *const libc::c_char,
        methods: *mut PyMethodDef,
        doc: *const libc::c_char,
        self_0: *mut PyObject,
        apiver: libc::c_int,
    ) -> *mut PyObject;
    fn PyErr_Print();
    fn PyEval_CallObjectWithKeywords(
        _: *mut PyObject,
        _: *mut PyObject,
        _: *mut PyObject,
    ) -> *mut PyObject;
    fn PyEval_CallMethod(
        obj: *mut PyObject,
        methodname: *const libc::c_char,
        format: *const libc::c_char,
        _: ...
    ) -> *mut PyObject;
    fn _PyObject_CallFunction_SizeT(
        callable_object: *mut PyObject,
        format: *mut libc::c_char,
        _: ...
    ) -> *mut PyObject;
    fn _PyObject_CallMethod_SizeT(
        o: *mut PyObject,
        m: *mut libc::c_char,
        format: *mut libc::c_char,
        _: ...
    ) -> *mut PyObject;
    fn PyObject_HasAttrString(_: *mut PyObject, _: *const libc::c_char) -> libc::c_int;
    fn PyCallable_Check(_: *mut PyObject) -> libc::c_int;
    static mut _Py_NoneStruct: PyObject;
    fn xmlBufferCreate() -> xmlBufferPtr;
    fn xmlBufferFree(buf: xmlBufferPtr);
    fn xmlNewNode(ns: xmlNsPtr, name: *const xmlChar) -> xmlNodePtr;
    fn xmlMemSetup(
        freeFunc_0: xmlFreeFunc,
        mallocFunc_0: xmlMallocFunc,
        reallocFunc_0: xmlReallocFunc,
        strdupFunc_0: xmlStrdupFunc,
    ) -> libc::c_int;
    fn xmlMemGet(
        freeFunc_0: *mut xmlFreeFunc,
        mallocFunc_0: *mut xmlMallocFunc,
        reallocFunc_0: *mut xmlReallocFunc,
        strdupFunc_0: *mut xmlStrdupFunc,
    ) -> libc::c_int;
    fn xmlMemUsed() -> libc::c_int;
    fn xmlMemoryDump();
    fn xmlMemMalloc(size: size_t) -> *mut libc::c_void;
    fn xmlMemRealloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn xmlMemFree(ptr: *mut libc::c_void);
    fn xmlMemoryStrdup(str: *const libc::c_char) -> *mut libc::c_char;
    static mut xmlFree: xmlFreeFunc;
    static mut xmlRealloc: xmlReallocFunc;
    static mut xmlMalloc: xmlMallocFunc;
    fn xmlThrDefSetGenericErrorFunc(
        ctx: *mut libc::c_void,
        handler: xmlGenericErrorFunc,
    );
    fn xmlSetGenericErrorFunc(ctx: *mut libc::c_void, handler: xmlGenericErrorFunc);
    fn xmlParserError(ctx: *mut libc::c_void, msg: *const libc::c_char, _: ...);
    fn xmlParserWarning(ctx: *mut libc::c_void, msg: *const libc::c_char, _: ...);
    fn xmlParserValidityError(ctx: *mut libc::c_void, msg: *const libc::c_char, _: ...);
    fn xmlParserValidityWarning(
        ctx: *mut libc::c_void,
        msg: *const libc::c_char,
        _: ...
    );
    fn xmlStrEqual(str1: *const xmlChar, str2: *const xmlChar) -> libc::c_int;
    fn xmlStrdup(cur: *const xmlChar) -> *mut xmlChar;
    fn xmlFreeValidCtxt(_: xmlValidCtxtPtr);
    fn xmlFindCharEncodingHandler(
        name: *const libc::c_char,
    ) -> xmlCharEncodingHandlerPtr;
    fn xmlParseCharEncoding(name: *const libc::c_char) -> xmlCharEncoding;
    fn xmlPopInputCallbacks() -> libc::c_int;
    fn xmlAllocParserInputBuffer(enc: xmlCharEncoding) -> xmlParserInputBufferPtr;
    fn xmlParserGetDirectory(filename: *const libc::c_char) -> *mut libc::c_char;
    fn xmlRegisterInputCallbacks(
        matchFunc: xmlInputMatchCallback,
        openFunc: xmlInputOpenCallback,
        readFunc: xmlInputReadCallback,
        closeFunc: xmlInputCloseCallback,
    ) -> libc::c_int;
    fn xmlAllocOutputBuffer(encoder: xmlCharEncodingHandlerPtr) -> xmlOutputBufferPtr;
    fn xmlOutputBufferCreateFile(
        file: *mut FILE,
        encoder: xmlCharEncodingHandlerPtr,
    ) -> xmlOutputBufferPtr;
    fn xmlOutputBufferFlush(out: xmlOutputBufferPtr) -> libc::c_int;
    fn xmlOutputBufferClose(out: xmlOutputBufferPtr) -> libc::c_int;
    fn xmlInitParser();
    fn xmlCleanupParser();
    fn xmlSAXUserParseFile(
        sax: xmlSAXHandlerPtr,
        user_data: *mut libc::c_void,
        filename: *const libc::c_char,
    ) -> libc::c_int;
    fn xmlFreeParserCtxt(ctxt: xmlParserCtxtPtr);
    fn xmlCreatePushParserCtxt(
        sax: xmlSAXHandlerPtr,
        user_data: *mut libc::c_void,
        chunk: *const libc::c_char,
        size: libc::c_int,
        filename: *const libc::c_char,
    ) -> xmlParserCtxtPtr;
    fn xmlNewIOInputStream(
        ctxt: xmlParserCtxtPtr,
        input: xmlParserInputBufferPtr,
        enc: xmlCharEncoding,
    ) -> xmlParserInputPtr;
    fn xmlSetExternalEntityLoader(f: xmlExternalEntityLoader);
    fn xmlGetExternalEntityLoader() -> xmlExternalEntityLoader;
    fn xmlSaveFormatFileTo(
        buf: xmlOutputBufferPtr,
        cur: xmlDocPtr,
        encoding: *const libc::c_char,
        format: libc::c_int,
    ) -> libc::c_int;
    fn xmlNodeDumpOutput(
        buf: xmlOutputBufferPtr,
        doc: xmlDocPtr,
        cur: xmlNodePtr,
        level: libc::c_int,
        format: libc::c_int,
        encoding: *const libc::c_char,
    );
    fn xmlSaveFileTo(
        buf: xmlOutputBufferPtr,
        cur: xmlDocPtr,
        encoding: *const libc::c_char,
    ) -> libc::c_int;
    fn xmlXPathRegisterFuncLookup(
        ctxt: xmlXPathContextPtr,
        f: xmlXPathFuncLookupFunc,
        funcCtxt: *mut libc::c_void,
    );
    fn xmlXPathRegisterVariableNS(
        ctxt: xmlXPathContextPtr,
        name: *const xmlChar,
        ns_uri: *const xmlChar,
        value: xmlXPathObjectPtr,
    ) -> libc::c_int;
    fn valuePop(ctxt: xmlXPathParserContextPtr) -> xmlXPathObjectPtr;
    fn valuePush(
        ctxt: xmlXPathParserContextPtr,
        value: xmlXPathObjectPtr,
    ) -> libc::c_int;
    fn xmlC14NDocSaveTo(
        doc: xmlDocPtr,
        nodes: xmlNodeSetPtr,
        mode: libc::c_int,
        inclusive_ns_prefixes: *mut *mut xmlChar,
        with_comments: libc::c_int,
        buf: xmlOutputBufferPtr,
    ) -> libc::c_int;
    fn xmlC14NDocDumpMemory(
        doc: xmlDocPtr,
        nodes: xmlNodeSetPtr,
        mode: libc::c_int,
        inclusive_ns_prefixes: *mut *mut xmlChar,
        with_comments: libc::c_int,
        doc_txt_ptr: *mut *mut xmlChar,
    ) -> libc::c_int;
    fn xmlFreeTextReader(reader: xmlTextReaderPtr);
    fn xmlRelaxNGFreeValidCtxt(ctxt: xmlRelaxNGValidCtxtPtr);
    fn xmlRelaxNGSetValidErrors(
        ctxt: xmlRelaxNGValidCtxtPtr,
        err: xmlRelaxNGValidityErrorFunc,
        warn: xmlRelaxNGValidityWarningFunc,
        ctx: *mut libc::c_void,
    );
    fn xmlTextReaderSetErrorHandler(
        reader: xmlTextReaderPtr,
        f: xmlTextReaderErrorFunc,
        arg: *mut libc::c_void,
    );
    fn xmlTextReaderGetErrorHandler(
        reader: xmlTextReaderPtr,
        f: *mut xmlTextReaderErrorFunc,
        arg: *mut *mut libc::c_void,
    );
    fn xmlRelaxNGGetValidErrors(
        ctxt: xmlRelaxNGValidCtxtPtr,
        err: *mut xmlRelaxNGValidityErrorFunc,
        warn: *mut xmlRelaxNGValidityWarningFunc,
        ctx: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn xmlSchemaSetValidErrors(
        ctxt: xmlSchemaValidCtxtPtr,
        err: xmlSchemaValidityErrorFunc,
        warn: xmlSchemaValidityWarningFunc,
        ctx: *mut libc::c_void,
    );
    fn xmlSchemaFreeValidCtxt(ctxt: xmlSchemaValidCtxtPtr);
    fn xmlSchemaGetValidErrors(
        ctxt: xmlSchemaValidCtxtPtr,
        err: *mut xmlSchemaValidityErrorFunc,
        warn: *mut xmlSchemaValidityWarningFunc,
        ctx: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn xmlSaveToBuffer(
        buffer: xmlBufferPtr,
        encoding: *const libc::c_char,
        options: libc::c_int,
    ) -> xmlSaveCtxtPtr;
    fn xmlSaveDoc(ctxt: xmlSaveCtxtPtr, doc: xmlDocPtr) -> libc::c_long;
    fn xmlSaveTree(ctxt: xmlSaveCtxtPtr, node: xmlNodePtr) -> libc::c_long;
    fn xmlSaveClose(ctxt: xmlSaveCtxtPtr) -> libc::c_int;
    fn htmlCreatePushParserCtxt(
        sax: htmlSAXHandlerPtr,
        user_data: *mut libc::c_void,
        chunk: *const libc::c_char,
        size: libc::c_int,
        filename: *const libc::c_char,
        enc: xmlCharEncoding,
    ) -> htmlParserCtxtPtr;
    fn xmlCatalogAddLocal(
        catalogs: *mut libc::c_void,
        URL: *const xmlChar,
    ) -> *mut libc::c_void;
    fn htmlGetMetaEncoding(doc: htmlDocPtr) -> *const xmlChar;
    fn htmlSAXParseFile(
        filename: *const libc::c_char,
        encoding: *const libc::c_char,
        sax: htmlSAXHandlerPtr,
        userData: *mut libc::c_void,
    ) -> htmlDocPtr;
    fn htmlNodeDumpFormatOutput(
        buf: xmlOutputBufferPtr,
        doc: xmlDocPtr,
        cur: xmlNodePtr,
        encoding: *const libc::c_char,
        format: libc::c_int,
    );
    fn htmlDocContentDumpFormatOutput(
        buf: xmlOutputBufferPtr,
        cur: xmlDocPtr,
        encoding: *const libc::c_char,
        format: libc::c_int,
    );
    fn libxml_xmlXPathObjectPtrConvert(obj: *mut PyObject) -> xmlXPathObjectPtr;
    fn libxml_xmlTextReaderLocatorPtrWrap(
        locator: xmlTextReaderLocatorPtr,
    ) -> *mut PyObject;
    fn libxml_xmlParserInputBufferPtrWrap(
        buffer: xmlParserInputBufferPtr,
    ) -> *mut PyObject;
    fn libxml_xmlOutputBufferPtrWrap(buffer: xmlOutputBufferPtr) -> *mut PyObject;
    fn libxml_xmlXPathObjectPtrWrap(obj: xmlXPathObjectPtr) -> *mut PyObject;
    fn libxml_xmlXPathParserContextPtrWrap(
        ctxt: xmlXPathParserContextPtr,
    ) -> *mut PyObject;
    fn libxml_xmlParserCtxtPtrWrap(ctxt: xmlParserCtxtPtr) -> *mut PyObject;
    fn libxml_xmlNsPtrWrap(ns: xmlNsPtr) -> *mut PyObject;
    fn libxml_xmlAttrPtrWrap(attr: xmlAttrPtr) -> *mut PyObject;
    fn libxml_xmlNodePtrWrap(node: xmlNodePtr) -> *mut PyObject;
    fn libxml_xmlDocPtrWrap(doc: xmlDocPtr) -> *mut PyObject;
    fn libxml_charPtrConstWrap(str: *const libc::c_char) -> *mut PyObject;
    fn libxml_charPtrWrap(str: *mut libc::c_char) -> *mut PyObject;
    fn libxml_constxmlCharPtrWrap(str: *const xmlChar) -> *mut PyObject;
    fn libxml_longWrap(val: libc::c_long) -> *mut PyObject;
    fn libxml_intWrap(val: libc::c_int) -> *mut PyObject;
    fn libxml_htmlAutoCloseTag(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_htmlCreateFileParserCtxt(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_htmlCreateMemoryParserCtxt(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_htmlCtxtReadDoc(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_htmlCtxtReadFd(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_htmlCtxtReadFile(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_htmlCtxtReadMemory(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_htmlCtxtReset(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_htmlCtxtUseOptions(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_htmlDefaultSAXHandlerInit(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_htmlDocContentDumpFormatOutput(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_htmlDocContentDumpOutput(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_htmlDocDump(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_htmlFreeParserCtxt(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_htmlGetMetaEncoding(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_htmlHandleOmittedElem(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_htmlInitAutoClose(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_htmlIsAutoClosed(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_htmlIsBooleanAttr(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_htmlIsScriptAttribute(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_htmlNewDoc(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_htmlNewDocNoDtD(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_htmlNewParserCtxt(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_htmlNodeDumpFile(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_htmlNodeDumpFileFormat(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_htmlNodeDumpFormatOutput(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_htmlNodeDumpOutput(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_htmlParseCharRef(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_htmlParseChunk(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_htmlParseDoc(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_htmlParseDocument(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_htmlParseElement(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_htmlParseFile(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_htmlReadDoc(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_htmlReadFd(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_htmlReadFile(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_htmlReadMemory(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_htmlSaveFile(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_htmlSaveFileEnc(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_htmlSaveFileFormat(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_htmlSetMetaEncoding(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_namePop(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_namePush(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_nodePop(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_nodePush(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_valuePop(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlACatalogAdd(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlACatalogDump(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlACatalogRemove(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlACatalogResolve(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlACatalogResolvePublic(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlACatalogResolveSystem(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlACatalogResolveURI(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlAddChild(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlAddChildList(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlAddDocEntity(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlAddDtdEntity(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlAddEncodingAlias(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlAddNextSibling(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlAddPrevSibling(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlAddSibling(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlBoolToText(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlBuildQName(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlBuildRelativeURI(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlBuildURI(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlByteConsumed(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlCanonicPath(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlCatalogAdd(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlCatalogCleanup(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlCatalogConvert(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlCatalogDump(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlCatalogGetPublic(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlCatalogGetSystem(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlCatalogIsEmpty(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlCatalogRemove(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlCatalogResolve(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlCatalogResolvePublic(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlCatalogResolveSystem(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlCatalogResolveURI(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlCatalogSetDebug(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlCharStrdup(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlCharStrndup(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlCheckFilename(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlCheckLanguageID(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlCheckUTF8(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlCheckVersion(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlCleanupCharEncodingHandlers(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlCleanupEncodingAliases(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlCleanupGlobals(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlCleanupInputCallbacks(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlCleanupOutputCallbacks(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlCleanupPredefinedEntities(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlClearParserCtxt(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlConvertSGMLCatalog(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlCopyChar(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlCopyCharMultiByte(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlCopyDoc(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlCopyDtd(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlCopyError(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlCopyNamespace(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlCopyNamespaceList(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlCopyNode(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlCopyNodeList(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlCopyProp(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlCopyPropList(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlCreateDocParserCtxt(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlCreateEntityParserCtxt(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlCreateFileParserCtxt(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlCreateIntSubset(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlCreateMemoryParserCtxt(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlCreateURI(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlCreateURLParserCtxt(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlCtxtReadDoc(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlCtxtReadFd(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlCtxtReadFile(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlCtxtReadMemory(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlCtxtReset(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlCtxtResetPush(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlCtxtUseOptions(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlDebugCheckDocument(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlDebugDumpAttr(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlDebugDumpAttrList(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlDebugDumpDTD(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlDebugDumpDocument(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlDebugDumpDocumentHead(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlDebugDumpEntities(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlDebugDumpNode(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlDebugDumpNodeList(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlDebugDumpOneNode(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlDebugDumpString(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlDecodeEntities(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlDefaultSAXHandlerInit(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlDelEncodingAlias(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlDictCleanup(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlDocCopyNode(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlDocCopyNodeList(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlDocDump(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlDocFormatDump(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlDocGetRootElement(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlDocSetRootElement(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlElemDump(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlEncodeEntities(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlEncodeEntitiesReentrant(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlEncodeSpecialChars(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlErrorGetCode(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlErrorGetDomain(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlErrorGetFile(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlErrorGetLevel(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlErrorGetLine(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlErrorGetMessage(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlFileMatch(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlFirstElementChild(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlFreeCatalog(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlFreeDoc(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlFreeDtd(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlFreeNode(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlFreeNodeList(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlFreeNs(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlFreeNsList(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlFreeParserInputBuffer(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlFreeProp(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlFreePropList(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlFreeURI(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlGetCompressMode(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlGetDocCompressMode(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlGetDocEntity(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlGetDtdAttrDesc(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlGetDtdElementDesc(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlGetDtdEntity(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlGetDtdQAttrDesc(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlGetDtdQElementDesc(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlGetEncodingAlias(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlGetID(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlGetIntSubset(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlGetLastChild(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlGetLastError(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlGetLineNo(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlGetNoNsProp(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlGetNodePath(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlGetNsProp(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlGetParameterEntity(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlGetPredefinedEntity(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlGetProp(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlHandleEntity(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlHasNsProp(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlHasProp(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlIOFTPMatch(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlIOHTTPMatch(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlInitCharEncodingHandlers(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlInitGlobals(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlInitParser(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlInitParserCtxt(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlInitializeCatalog(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlInitializeDict(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlInitializePredefinedEntities(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlIsBaseChar(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlIsBlank(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlIsBlankNode(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlIsChar(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlIsCombining(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlIsDigit(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlIsExtender(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlIsID(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlIsIdeographic(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlIsLetter(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlIsMixedElement(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlIsPubidChar(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlIsRef(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlIsXHTML(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlKeepBlanksDefault(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlLastElementChild(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlLineNumbersDefault(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlLoadACatalog(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlLoadCatalog(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlLoadCatalogs(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlLoadSGMLSuperCatalog(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlLsCountNode(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlLsOneNode(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlNamespaceParseNCName(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlNamespaceParseNSDef(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlNanoFTPCleanup(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlNanoFTPInit(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlNanoFTPProxy(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlNanoFTPScanProxy(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlNanoHTTPCleanup(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlNanoHTTPInit(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlNanoHTTPScanProxy(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlNewCDataBlock(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlNewCatalog(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlNewCharRef(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlNewChild(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlNewComment(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlNewDoc(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlNewDocComment(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlNewDocFragment(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlNewDocNode(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlNewDocNodeEatName(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlNewDocPI(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlNewDocProp(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlNewDocRawNode(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlNewDocText(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlNewDocTextLen(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlNewDtd(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlNewEntity(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlNewGlobalNs(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlNewNodeEatName(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlNewNs(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlNewNsProp(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlNewNsPropEatName(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlNewPI(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlNewParserCtxt(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlNewProp(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlNewReference(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlNewText(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlNewTextChild(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlNewTextLen(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlNewTextReader(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlNewTextReaderFilename(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlNewValidCtxt(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlNextChar(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlNextElementSibling(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlNodeAddContent(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlNodeAddContentLen(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlNodeDumpOutput(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlNodeGetBase(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlNodeGetContent(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlNodeGetLang(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlNodeGetSpacePreserve(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlNodeIsText(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlNodeListGetRawString(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlNodeListGetString(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlNodeSetBase(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlNodeSetContent(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlNodeSetContentLen(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlNodeSetLang(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlNodeSetName(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlNodeSetSpacePreserve(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlNormalizeURIPath(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlNormalizeWindowsPath(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlOutputBufferGetContent(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlOutputBufferWrite(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlOutputBufferWriteString(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlParseAttValue(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlParseAttributeListDecl(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlParseCDSect(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlParseCatalogFile(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlParseCharData(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlParseCharRef(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlParseChunk(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlParseComment(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlParseContent(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlParseDTD(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlParseDoc(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlParseDocTypeDecl(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlParseDocument(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlParseElement(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlParseElementDecl(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlParseEncName(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlParseEncodingDecl(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlParseEndTag(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlParseEntity(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlParseEntityDecl(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlParseEntityRef(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlParseExtParsedEnt(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlParseExternalSubset(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlParseFile(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlParseMarkupDecl(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlParseMemory(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlParseMisc(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlParseName(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlParseNamespace(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlParseNmtoken(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlParseNotationDecl(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlParsePEReference(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlParsePI(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlParsePITarget(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlParsePubidLiteral(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlParseQuotedString(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlParseReference(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlParseSDDecl(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlParseStartTag(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlParseSystemLiteral(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlParseTextDecl(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlParseURI(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlParseURIRaw(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlParseURIReference(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlParseVersionInfo(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlParseVersionNum(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlParseXMLDecl(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlParserGetDirectory(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlParserGetDoc(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlParserGetIsValid(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlParserGetWellFormed(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlParserHandlePEReference(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlParserHandleReference(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlParserInputBufferGrow(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlParserInputBufferPush(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlParserInputBufferRead(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlParserSetLineNumbers(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlParserSetLoadSubset(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlParserSetPedantic(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlParserSetReplaceEntities(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlParserSetValidate(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlPathToURI(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlPedanticParserDefault(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlPopInput(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlPopOutputCallbacks(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlPreviousElementSibling(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlPrintURI(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlReadDoc(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlReadFd(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlReadFile(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlReadMemory(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlReaderForDoc(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlReaderForFd(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlReaderForFile(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlReaderForMemory(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlReaderNewDoc(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlReaderNewFd(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlReaderNewFile(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlReaderNewMemory(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlReaderNewWalker(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlReaderWalker(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlReconciliateNs(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlRecoverDoc(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlRecoverFile(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlRecoverMemory(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlRegFreeRegexp(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlRegexpCompile(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlRegexpExec(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlRegexpIsDeterminist(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlRegexpPrint(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlRegisterDefaultInputCallbacks(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlRegisterDefaultOutputCallbacks(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlRegisterHTTPPostCallbacks(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlRelaxNGCleanupTypes(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlRelaxNGDump(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlRelaxNGDumpTree(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlRelaxNGFree(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlRelaxNGFreeParserCtxt(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlRelaxNGInitTypes(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlRelaxNGNewDocParserCtxt(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlRelaxNGNewMemParserCtxt(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlRelaxNGNewParserCtxt(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlRelaxNGNewValidCtxt(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlRelaxNGParse(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlRelaxNGValidateDoc(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlRelaxNGValidateFullElement(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlRelaxNGValidatePopElement(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlRelaxNGValidatePushCData(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlRelaxNGValidatePushElement(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlRelaxParserSetFlag(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlRemoveID(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlRemoveProp(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlRemoveRef(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlReplaceNode(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlResetError(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlResetLastError(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlSAXDefaultVersion(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlSaveFile(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlSaveFileEnc(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlSaveFormatFile(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlSaveFormatFileEnc(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlSaveUri(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlScanName(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlSchemaCleanupTypes(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlSchemaCollapseString(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlSchemaDump(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlSchemaFree(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlSchemaFreeParserCtxt(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlSchemaInitTypes(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlSchemaIsValid(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlSchemaNewDocParserCtxt(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlSchemaNewMemParserCtxt(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlSchemaNewParserCtxt(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlSchemaNewValidCtxt(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlSchemaParse(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlSchemaSetValidOptions(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlSchemaValidCtxtGetOptions(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlSchemaValidCtxtGetParserCtxt(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlSchemaValidateDoc(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlSchemaValidateFile(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlSchemaValidateOneElement(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlSchemaValidateSetFilename(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlSchemaWhiteSpaceReplace(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlSearchNs(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlSearchNsByHref(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlSetCompressMode(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlSetDocCompressMode(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlSetListDoc(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlSetNs(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlSetNsProp(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlSetProp(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlSetTreeDoc(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlSetupParserForBuffer(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlShellPrintNode(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlShellPrintXPathError(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlSkipBlankChars(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlStopParser(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlStrEqual(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlStrQEqual(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlStrcasecmp(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlStrcasestr(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlStrcat(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlStrchr(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlStrcmp(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlStrdup(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlStringDecodeEntities(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlStringGetNodeList(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlStringLenDecodeEntities(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlStringLenGetNodeList(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlStrlen(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlStrncasecmp(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlStrncat(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlStrncatNew(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlStrncmp(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlStrndup(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlStrstr(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlStrsub(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlSubstituteEntitiesDefault(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextConcat(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlTextMerge(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlTextReaderAttributeCount(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderByteConsumed(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderClose(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderConstBaseUri(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderConstEncoding(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderConstLocalName(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderConstName(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderConstNamespaceUri(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderConstPrefix(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderConstString(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderConstValue(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderConstXmlLang(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderConstXmlVersion(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderCurrentDoc(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderCurrentNode(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderDepth(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderExpand(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderGetAttribute(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderGetAttributeNo(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderGetAttributeNs(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderGetParserColumnNumber(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderGetParserLineNumber(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderGetParserProp(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderGetRemainder(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderHasAttributes(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderHasValue(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderIsDefault(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderIsEmptyElement(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderIsNamespaceDecl(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderIsValid(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderLocatorBaseURI(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderLocatorLineNumber(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderLookupNamespace(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderMoveToAttribute(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderMoveToAttributeNo(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderMoveToAttributeNs(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderMoveToElement(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderMoveToFirstAttribute(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderMoveToNextAttribute(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderNext(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderNextSibling(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderNodeType(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderNormalization(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderPreserve(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderQuoteChar(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderRead(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderReadAttributeValue(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderReadInnerXml(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderReadOuterXml(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderReadState(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderReadString(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderRelaxNGSetSchema(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderRelaxNGValidate(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderRelaxNGValidateCtxt(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderSchemaValidate(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderSchemaValidateCtxt(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderSetParserProp(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderSetSchema(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderSetup(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlTextReaderStandalone(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlThrDefDefaultBufferSize(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlThrDefDoValidityCheckingDefaultValue(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlThrDefGetWarningsDefaultValue(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlThrDefIndentTreeOutput(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlThrDefKeepBlanksDefaultValue(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlThrDefLineNumbersDefaultValue(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlThrDefLoadExtDtdDefaultValue(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlThrDefParserDebugEntities(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlThrDefPedanticParserDefaultValue(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlThrDefSaveNoEmptyTags(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlThrDefSubstituteEntitiesDefaultValue(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlThrDefTreeIndentString(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsAegeanNumbers(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsAlphabeticPresentationForms(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsArabic(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsArabicPresentationFormsA(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsArabicPresentationFormsB(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsArmenian(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsArrows(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsBasicLatin(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsBengali(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsBlock(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlUCSIsBlockElements(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsBopomofo(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsBopomofoExtended(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsBoxDrawing(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsBraillePatterns(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsBuhid(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlUCSIsByzantineMusicalSymbols(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsCJKCompatibility(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsCJKCompatibilityForms(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsCJKCompatibilityIdeographs(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsCJKCompatibilityIdeographsSupplement(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsCJKRadicalsSupplement(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsCJKSymbolsandPunctuation(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsCJKUnifiedIdeographs(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsCJKUnifiedIdeographsExtensionA(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsCJKUnifiedIdeographsExtensionB(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsCat(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlUCSIsCatC(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlUCSIsCatCc(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlUCSIsCatCf(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlUCSIsCatCo(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlUCSIsCatCs(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlUCSIsCatL(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlUCSIsCatLl(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlUCSIsCatLm(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlUCSIsCatLo(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlUCSIsCatLt(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlUCSIsCatLu(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlUCSIsCatM(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlUCSIsCatMc(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlUCSIsCatMe(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlUCSIsCatMn(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlUCSIsCatN(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlUCSIsCatNd(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlUCSIsCatNl(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlUCSIsCatNo(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlUCSIsCatP(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlUCSIsCatPc(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlUCSIsCatPd(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlUCSIsCatPe(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlUCSIsCatPf(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlUCSIsCatPi(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlUCSIsCatPo(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlUCSIsCatPs(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlUCSIsCatS(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlUCSIsCatSc(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlUCSIsCatSk(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlUCSIsCatSm(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlUCSIsCatSo(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlUCSIsCatZ(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlUCSIsCatZl(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlUCSIsCatZp(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlUCSIsCatZs(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlUCSIsCherokee(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsCombiningDiacriticalMarks(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsCombiningDiacriticalMarksforSymbols(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsCombiningHalfMarks(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsCombiningMarksforSymbols(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsControlPictures(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsCurrencySymbols(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsCypriotSyllabary(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsCyrillic(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsCyrillicSupplement(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsDeseret(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsDevanagari(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsDingbats(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsEnclosedAlphanumerics(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsEnclosedCJKLettersandMonths(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsEthiopic(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsGeneralPunctuation(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsGeometricShapes(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsGeorgian(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsGothic(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsGreek(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlUCSIsGreekExtended(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsGreekandCoptic(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsGujarati(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsGurmukhi(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsHalfwidthandFullwidthForms(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsHangulCompatibilityJamo(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsHangulJamo(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsHangulSyllables(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsHanunoo(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsHebrew(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsHighPrivateUseSurrogates(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsHighSurrogates(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsHiragana(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsIPAExtensions(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsIdeographicDescriptionCharacters(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsKanbun(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsKangxiRadicals(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsKannada(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsKatakana(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsKatakanaPhoneticExtensions(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsKhmer(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlUCSIsKhmerSymbols(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsLao(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlUCSIsLatin1Supplement(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsLatinExtendedA(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsLatinExtendedAdditional(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsLatinExtendedB(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsLetterlikeSymbols(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsLimbu(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlUCSIsLinearBIdeograms(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsLinearBSyllabary(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsLowSurrogates(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsMalayalam(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsMathematicalAlphanumericSymbols(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsMathematicalOperators(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsMiscellaneousMathematicalSymbolsA(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsMiscellaneousMathematicalSymbolsB(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsMiscellaneousSymbols(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsMiscellaneousSymbolsandArrows(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsMiscellaneousTechnical(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsMongolian(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsMusicalSymbols(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsMyanmar(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsNumberForms(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsOgham(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlUCSIsOldItalic(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsOpticalCharacterRecognition(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsOriya(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlUCSIsOsmanya(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsPhoneticExtensions(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsPrivateUse(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsPrivateUseArea(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsRunic(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlUCSIsShavian(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsSinhala(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsSmallFormVariants(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsSpacingModifierLetters(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsSpecials(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsSuperscriptsandSubscripts(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsSupplementalArrowsA(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsSupplementalArrowsB(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsSupplementalMathematicalOperators(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsSupplementaryPrivateUseAreaA(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsSupplementaryPrivateUseAreaB(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsSyriac(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsTagalog(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsTagbanwa(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsTags(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlUCSIsTaiLe(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlUCSIsTaiXuanJingSymbols(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsTamil(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlUCSIsTelugu(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsThaana(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsThai(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlUCSIsTibetan(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsUgaritic(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsUnifiedCanadianAboriginalSyllabics(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsVariationSelectors(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsVariationSelectorsSupplement(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsYiRadicals(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsYiSyllables(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUCSIsYijingHexagramSymbols(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlURIEscape(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlURIEscapeStr(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlURIGetAuthority(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlURIGetFragment(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlURIGetOpaque(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlURIGetPath(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlURIGetPort(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlURIGetQuery(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlURIGetQueryRaw(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlURIGetScheme(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlURIGetServer(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlURIGetUser(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlURISetAuthority(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlURISetFragment(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlURISetOpaque(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlURISetPath(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlURISetPort(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlURISetQuery(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlURISetQueryRaw(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlURISetScheme(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlURISetServer(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlURISetUser(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlURIUnescapeString(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUTF8Charcmp(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUTF8Size(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlUTF8Strlen(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlUTF8Strloc(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlUTF8Strndup(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUTF8Strpos(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlUTF8Strsize(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUTF8Strsub(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlUnlinkNode(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlUnsetNsProp(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlUnsetProp(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlValidCtxtNormalizeAttributeValue(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlValidNormalizeAttributeValue(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlValidateDocument(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlValidateDocumentFinal(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlValidateDtd(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlValidateDtdFinal(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlValidateElement(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlValidateNCName(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlValidateNMToken(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlValidateName(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlValidateNameValue(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlValidateNamesValue(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlValidateNmtokenValue(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlValidateNmtokensValue(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlValidateNotationUse(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlValidateOneAttribute(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlValidateOneElement(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlValidateOneNamespace(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlValidatePopElement(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlValidatePushCData(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlValidatePushElement(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlValidateQName(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlValidateRoot(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXIncludeProcess(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXIncludeProcessFlags(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXIncludeProcessTree(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXIncludeProcessTreeFlags(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathAddValues(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathBooleanFunction(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathCastBooleanToNumber(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathCastBooleanToString(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathCastNodeToNumber(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathCastNodeToString(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathCastNumberToBoolean(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathCastNumberToString(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathCastStringToBoolean(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathCastStringToNumber(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathCeilingFunction(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathCmpNodes(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathCompareValues(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathConcatFunction(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathContainsFunction(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathContextSetCache(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathCountFunction(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathDivValues(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathEqualValues(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathErr(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlXPathEval(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlXPathEvalExpr(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathEvalExpression(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathFalseFunction(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathFloorFunction(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathFreeContext(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathFreeParserContext(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathGetContextDoc(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathGetContextNode(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathGetContextPosition(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathGetContextSize(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathGetFunction(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathGetFunctionURI(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathIdFunction(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathInit(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlXPathIsInf(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlXPathIsNaN(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlXPathIsNodeType(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathLangFunction(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathLastFunction(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathLocalNameFunction(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathModValues(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathMultValues(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathNamespaceURIFunction(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathNewBoolean(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathNewCString(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathNewContext(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathNewFloat(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathNewNodeSet(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathNewParserContext(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathNewString(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathNewValueTree(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathNextAncestor(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathNextAncestorOrSelf(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathNextAttribute(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathNextChild(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathNextDescendant(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathNextDescendantOrSelf(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathNextFollowing(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathNextFollowingSibling(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathNextNamespace(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathNextParent(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathNextPreceding(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathNextPrecedingSibling(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathNextSelf(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathNodeEval(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathNodeSetFreeNs(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathNormalizeFunction(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathNotEqualValues(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathNotFunction(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathNsLookup(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathNumberFunction(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathOrderDocElems(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathParseNCName(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathParseName(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathParserGetContext(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathPopBoolean(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathPopNumber(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathPopString(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathPositionFunction(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathRegisterAllFunctions(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathRegisterNs(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathRegisteredFuncsCleanup(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathRegisteredNsCleanup(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathRegisteredVariablesCleanup(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathRoot(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlXPathRoundFunction(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathSetContextDoc(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathSetContextNode(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathStartsWithFunction(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathStringEvalNumber(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathStringFunction(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathStringLengthFunction(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathSubValues(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathSubstringAfterFunction(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathSubstringBeforeFunction(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathSubstringFunction(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathSumFunction(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathTranslateFunction(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathTrueFunction(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathValueFlipSign(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathVariableLookup(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPathVariableLookupNS(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
    fn libxml_xmlXPatherror(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlXPtrEval(self_0: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    fn libxml_xmlXPtrNewContext(
        self_0: *mut PyObject,
        args: *mut PyObject,
    ) -> *mut PyObject;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
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
    pub tp_name: *const libc::c_char,
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
    pub tp_flags: libc::c_long,
    pub tp_doc: *const libc::c_char,
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
    pub tp_version_tag: libc::c_uint,
}
pub type destructor = Option::<unsafe extern "C" fn(*mut PyObject) -> ()>;
pub type PyObject = _object;
pub type inquiry = Option::<unsafe extern "C" fn(*mut PyObject) -> libc::c_int>;
pub type freefunc = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type newfunc = Option::<
    unsafe extern "C" fn(*mut _typeobject, *mut PyObject, *mut PyObject) -> *mut PyObject,
>;
pub type allocfunc = Option::<
    unsafe extern "C" fn(*mut _typeobject, Py_ssize_t) -> *mut PyObject,
>;
pub type initproc = Option::<
    unsafe extern "C" fn(*mut PyObject, *mut PyObject, *mut PyObject) -> libc::c_int,
>;
pub type descrsetfunc = Option::<
    unsafe extern "C" fn(*mut PyObject, *mut PyObject, *mut PyObject) -> libc::c_int,
>;
pub type descrgetfunc = Option::<
    unsafe extern "C" fn(*mut PyObject, *mut PyObject, *mut PyObject) -> *mut PyObject,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyGetSetDef {
    pub name: *mut libc::c_char,
    pub get: getter,
    pub set: setter,
    pub doc: *mut libc::c_char,
    pub closure: *mut libc::c_void,
}
pub type setter = Option::<
    unsafe extern "C" fn(*mut PyObject, *mut PyObject, *mut libc::c_void) -> libc::c_int,
>;
pub type getter = Option::<
    unsafe extern "C" fn(*mut PyObject, *mut libc::c_void) -> *mut PyObject,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyMethodDef {
    pub ml_name: *const libc::c_char,
    pub ml_meth: PyCFunction,
    pub ml_flags: libc::c_int,
    pub ml_doc: *const libc::c_char,
}
pub type PyCFunction = Option::<
    unsafe extern "C" fn(*mut PyObject, *mut PyObject) -> *mut PyObject,
>;
pub type iternextfunc = Option::<unsafe extern "C" fn(*mut PyObject) -> *mut PyObject>;
pub type getiterfunc = Option::<unsafe extern "C" fn(*mut PyObject) -> *mut PyObject>;
pub type richcmpfunc = Option::<
    unsafe extern "C" fn(*mut PyObject, *mut PyObject, libc::c_int) -> *mut PyObject,
>;
pub type traverseproc = Option::<
    unsafe extern "C" fn(*mut PyObject, visitproc, *mut libc::c_void) -> libc::c_int,
>;
pub type visitproc = Option::<
    unsafe extern "C" fn(*mut PyObject, *mut libc::c_void) -> libc::c_int,
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
    pub readonly: libc::c_int,
    pub ndim: libc::c_int,
    pub format: *mut libc::c_char,
    pub shape: *mut Py_ssize_t,
    pub strides: *mut Py_ssize_t,
    pub suboffsets: *mut Py_ssize_t,
    pub smalltable: [Py_ssize_t; 2],
    pub internal: *mut libc::c_void,
}
pub type getbufferproc = Option::<
    unsafe extern "C" fn(*mut PyObject, *mut Py_buffer, libc::c_int) -> libc::c_int,
>;
pub type charbufferproc = Option::<
    unsafe extern "C" fn(*mut PyObject, Py_ssize_t, *mut *mut libc::c_char) -> Py_ssize_t,
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
    unsafe extern "C" fn(*mut PyObject, *mut PyObject, *mut PyObject) -> libc::c_int,
>;
pub type getattrofunc = Option::<
    unsafe extern "C" fn(*mut PyObject, *mut PyObject) -> *mut PyObject,
>;
pub type reprfunc = Option::<unsafe extern "C" fn(*mut PyObject) -> *mut PyObject>;
pub type ternaryfunc = Option::<
    unsafe extern "C" fn(*mut PyObject, *mut PyObject, *mut PyObject) -> *mut PyObject,
>;
pub type hashfunc = Option::<unsafe extern "C" fn(*mut PyObject) -> libc::c_long>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyMappingMethods {
    pub mp_length: lenfunc,
    pub mp_subscript: binaryfunc,
    pub mp_ass_subscript: objobjargproc,
}
pub type objobjargproc = Option::<
    unsafe extern "C" fn(*mut PyObject, *mut PyObject, *mut PyObject) -> libc::c_int,
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
    unsafe extern "C" fn(*mut PyObject, *mut PyObject) -> libc::c_int,
>;
pub type ssizessizeobjargproc = Option::<
    unsafe extern "C" fn(
        *mut PyObject,
        Py_ssize_t,
        Py_ssize_t,
        *mut PyObject,
    ) -> libc::c_int,
>;
pub type ssizeobjargproc = Option::<
    unsafe extern "C" fn(*mut PyObject, Py_ssize_t, *mut PyObject) -> libc::c_int,
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
    unsafe extern "C" fn(*mut *mut PyObject, *mut *mut PyObject) -> libc::c_int,
>;
pub type cmpfunc = Option::<
    unsafe extern "C" fn(*mut PyObject, *mut PyObject) -> libc::c_int,
>;
pub type setattrfunc = Option::<
    unsafe extern "C" fn(*mut PyObject, *mut libc::c_char, *mut PyObject) -> libc::c_int,
>;
pub type getattrfunc = Option::<
    unsafe extern "C" fn(*mut PyObject, *mut libc::c_char) -> *mut PyObject,
>;
pub type printfunc = Option::<
    unsafe extern "C" fn(*mut PyObject, *mut FILE, libc::c_int) -> libc::c_int,
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
pub struct PyTupleObject {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut _typeobject,
    pub ob_size: Py_ssize_t,
    pub ob_item: [*mut PyObject; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyListObject {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut _typeobject,
    pub ob_size: Py_ssize_t,
    pub ob_item: *mut *mut PyObject,
    pub allocated: Py_ssize_t,
}
pub type xmlFreeFunc = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type xmlMallocFunc = Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>;
pub type xmlReallocFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
>;
pub type xmlStrdupFunc = Option::<
    unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_char,
>;
pub type xmlChar = libc::c_uchar;
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
pub struct _xmlOutputBuffer {
    pub context: *mut libc::c_void,
    pub writecallback: xmlOutputWriteCallback,
    pub closecallback: xmlOutputCloseCallback,
    pub encoder: xmlCharEncodingHandlerPtr,
    pub buffer: xmlBufPtr,
    pub conv: xmlBufPtr,
    pub written: libc::c_int,
    pub error: libc::c_int,
}
pub type xmlOutputCloseCallback = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
>;
pub type xmlOutputWriteCallback = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const libc::c_char,
        libc::c_int,
    ) -> libc::c_int,
>;
pub type xmlOutputBuffer = _xmlOutputBuffer;
pub type xmlOutputBufferPtr = *mut xmlOutputBuffer;
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
pub type xmlAutomataPtr = *mut xmlAutomata;
pub type xmlAutomata = _xmlAutomata;
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
pub type xmlSAXHandler = _xmlSAXHandler;
pub type xmlSAXHandlerPtr = *mut xmlSAXHandler;
pub type xmlBufferAllocationScheme = libc::c_uint;
pub const XML_BUFFER_ALLOC_BOUNDED: xmlBufferAllocationScheme = 5;
pub const XML_BUFFER_ALLOC_HYBRID: xmlBufferAllocationScheme = 4;
pub const XML_BUFFER_ALLOC_IO: xmlBufferAllocationScheme = 3;
pub const XML_BUFFER_ALLOC_IMMUTABLE: xmlBufferAllocationScheme = 2;
pub const XML_BUFFER_ALLOC_EXACT: xmlBufferAllocationScheme = 1;
pub const XML_BUFFER_ALLOC_DOUBLEIT: xmlBufferAllocationScheme = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlBuffer {
    pub content: *mut xmlChar,
    pub use_0: libc::c_uint,
    pub size: libc::c_uint,
    pub alloc: xmlBufferAllocationScheme,
    pub contentIO: *mut xmlChar,
}
pub type xmlBuffer = _xmlBuffer;
pub type xmlBufferPtr = *mut xmlBuffer;
pub type xmlNsPtr = *mut xmlNs;
pub type xmlGenericErrorFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, ...) -> (),
>;
pub type xmlValidCtxtPtr = *mut xmlValidCtxt;
pub type xmlExternalEntityLoader = Option::<
    unsafe extern "C" fn(
        *const libc::c_char,
        *const libc::c_char,
        xmlParserCtxtPtr,
    ) -> xmlParserInputPtr,
>;
pub type xmlCharEncoding = libc::c_int;
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
pub type xmlInputMatchCallback = Option::<
    unsafe extern "C" fn(*const libc::c_char) -> libc::c_int,
>;
pub type xmlInputOpenCallback = Option::<
    unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_void,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlXPathContext {
    pub doc: xmlDocPtr,
    pub node: xmlNodePtr,
    pub nb_variables_unused: libc::c_int,
    pub max_variables_unused: libc::c_int,
    pub varHash: xmlHashTablePtr,
    pub nb_types: libc::c_int,
    pub max_types: libc::c_int,
    pub types: xmlXPathTypePtr,
    pub nb_funcs_unused: libc::c_int,
    pub max_funcs_unused: libc::c_int,
    pub funcHash: xmlHashTablePtr,
    pub nb_axis: libc::c_int,
    pub max_axis: libc::c_int,
    pub axis: xmlXPathAxisPtr,
    pub namespaces: *mut xmlNsPtr,
    pub nsNr: libc::c_int,
    pub user: *mut libc::c_void,
    pub contextSize: libc::c_int,
    pub proximityPosition: libc::c_int,
    pub xptr: libc::c_int,
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
    pub tmpNsNr: libc::c_int,
    pub userData: *mut libc::c_void,
    pub error: xmlStructuredErrorFunc,
    pub lastError: xmlError,
    pub debugNode: xmlNodePtr,
    pub dict: xmlDictPtr,
    pub flags: libc::c_int,
    pub cache: *mut libc::c_void,
    pub opLimit: libc::c_ulong,
    pub opCount: libc::c_ulong,
    pub depth: libc::c_int,
}
pub type xmlXPathFuncLookupFunc = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const xmlChar,
        *const xmlChar,
    ) -> xmlXPathFunction,
>;
pub type xmlXPathFunction = Option::<
    unsafe extern "C" fn(xmlXPathParserContextPtr, libc::c_int) -> (),
>;
pub type xmlXPathParserContextPtr = *mut xmlXPathParserContext;
pub type xmlXPathParserContext = _xmlXPathParserContext;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlXPathParserContext {
    pub cur: *const xmlChar,
    pub base: *const xmlChar,
    pub error: libc::c_int,
    pub context: xmlXPathContextPtr,
    pub value: xmlXPathObjectPtr,
    pub valueNr: libc::c_int,
    pub valueMax: libc::c_int,
    pub valueTab: *mut xmlXPathObjectPtr,
    pub comp: xmlXPathCompExprPtr,
    pub xptr: libc::c_int,
    pub ancestor: xmlNodePtr,
    pub valueFrame: libc::c_int,
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
    pub boolval: libc::c_int,
    pub floatval: libc::c_double,
    pub stringval: *mut xmlChar,
    pub user: *mut libc::c_void,
    pub index: libc::c_int,
    pub user2: *mut libc::c_void,
    pub index2: libc::c_int,
}
pub type xmlNodeSetPtr = *mut xmlNodeSet;
pub type xmlNodeSet = _xmlNodeSet;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlNodeSet {
    pub nodeNr: libc::c_int,
    pub nodeMax: libc::c_int,
    pub nodeTab: *mut xmlNodePtr,
}
pub type xmlXPathObjectType = libc::c_uint;
pub const XPATH_XSLT_TREE: xmlXPathObjectType = 9;
pub const XPATH_USERS: xmlXPathObjectType = 8;
pub const XPATH_STRING: xmlXPathObjectType = 4;
pub const XPATH_NUMBER: xmlXPathObjectType = 3;
pub const XPATH_BOOLEAN: xmlXPathObjectType = 2;
pub const XPATH_NODESET: xmlXPathObjectType = 1;
pub const XPATH_UNDEFINED: xmlXPathObjectType = 0;
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
    unsafe extern "C" fn(xmlXPathObjectPtr, libc::c_int) -> libc::c_int,
>;
pub type xmlRelaxNGValidityErrorFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, ...) -> (),
>;
pub type xmlRelaxNGValidityWarningFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, ...) -> (),
>;
pub type xmlRelaxNGValidCtxt = _xmlRelaxNGValidCtxt;
pub type xmlRelaxNGValidCtxtPtr = *mut xmlRelaxNGValidCtxt;
pub type xmlSchemaValidityErrorFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, ...) -> (),
>;
pub type xmlSchemaValidityWarningFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, ...) -> (),
>;
pub type xmlSchemaValidCtxt = _xmlSchemaValidCtxt;
pub type xmlSchemaValidCtxtPtr = *mut xmlSchemaValidCtxt;
pub type xmlParserSeverities = libc::c_uint;
pub const XML_PARSER_SEVERITY_ERROR: xmlParserSeverities = 4;
pub const XML_PARSER_SEVERITY_WARNING: xmlParserSeverities = 3;
pub const XML_PARSER_SEVERITY_VALIDITY_ERROR: xmlParserSeverities = 2;
pub const XML_PARSER_SEVERITY_VALIDITY_WARNING: xmlParserSeverities = 1;
pub type xmlTextReader = _xmlTextReader;
pub type xmlTextReaderPtr = *mut xmlTextReader;
pub type xmlTextReaderLocatorPtr = *mut libc::c_void;
pub type xmlTextReaderErrorFunc = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const libc::c_char,
        xmlParserSeverities,
        xmlTextReaderLocatorPtr,
    ) -> (),
>;
pub type C2RustUnnamed = libc::c_uint;
pub const XML_SAVE_WSNONSIG: C2RustUnnamed = 128;
pub const XML_SAVE_AS_HTML: C2RustUnnamed = 64;
pub const XML_SAVE_AS_XML: C2RustUnnamed = 32;
pub const XML_SAVE_XHTML: C2RustUnnamed = 16;
pub const XML_SAVE_NO_XHTML: C2RustUnnamed = 8;
pub const XML_SAVE_NO_EMPTY: C2RustUnnamed = 4;
pub const XML_SAVE_NO_DECL: C2RustUnnamed = 2;
pub const XML_SAVE_FORMAT: C2RustUnnamed = 1;
pub type xmlSaveCtxt = _xmlSaveCtxt;
pub type xmlSaveCtxtPtr = *mut xmlSaveCtxt;
pub type htmlParserCtxtPtr = xmlParserCtxtPtr;
pub type htmlSAXHandlerPtr = xmlSAXHandlerPtr;
pub type htmlDocPtr = xmlDocPtr;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyxmlNode_Object {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut _typeobject,
    pub obj: xmlNodePtr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyxmlXPathContext_Object {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut _typeobject,
    pub obj: xmlXPathContextPtr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyparserCtxt_Object {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut _typeobject,
    pub obj: xmlParserCtxtPtr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyValidCtxt_Object {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut _typeobject,
    pub obj: xmlValidCtxtPtr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyxmlTextReader_Object {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut _typeobject,
    pub obj: xmlTextReaderPtr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyoutputBuffer_Object {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut _typeobject,
    pub obj: xmlOutputBufferPtr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyrelaxNgValidCtxt_Object {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut _typeobject,
    pub obj: xmlRelaxNGValidCtxtPtr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PySchemaValidCtxt_Object {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut _typeobject,
    pub obj: xmlSchemaValidCtxtPtr,
}
pub type xmlSchemaValidCtxtPyCtxtPtr = *mut xmlSchemaValidCtxtPyCtxt;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xmlSchemaValidCtxtPyCtxt {
    pub warn: *mut PyObject,
    pub error: *mut PyObject,
    pub arg: *mut PyObject,
}
pub type libxml_xpathCallbackArray = [libxml_xpathCallback; 0];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct libxml_xpathCallback {
    pub ctx: xmlXPathContextPtr,
    pub name: *mut xmlChar,
    pub ns_uri: *mut xmlChar,
    pub function: *mut PyObject,
}
pub type xmlRelaxNGValidCtxtPyCtxtPtr = *mut xmlRelaxNGValidCtxtPyCtxt;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xmlRelaxNGValidCtxtPyCtxt {
    pub warn: *mut PyObject,
    pub error: *mut PyObject,
    pub arg: *mut PyObject,
}
pub type xmlTextReaderPyCtxtPtr = *mut xmlTextReaderPyCtxt;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xmlTextReaderPyCtxt {
    pub f: *mut PyObject,
    pub arg: *mut PyObject,
}
pub type xmlParserCtxtPyCtxtPtr = *mut xmlParserCtxtPyCtxt;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xmlParserCtxtPyCtxt {
    pub f: *mut PyObject,
    pub arg: *mut PyObject,
}
pub type xmlValidCtxtPyCtxtPtr = *mut xmlValidCtxtPyCtxt;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xmlValidCtxtPyCtxt {
    pub warn: *mut PyObject,
    pub error: *mut PyObject,
    pub arg: *mut PyObject,
}
static mut libxml_xpathCallbacksInitialized: libc::c_int = 0 as libc::c_int;
static mut libxml_xpathCallbacksAllocd: libc::c_int = 10 as libc::c_int;
static mut libxml_xpathCallbacks: *mut libxml_xpathCallbackArray = 0
    as *const libxml_xpathCallbackArray as *mut libxml_xpathCallbackArray;
static mut libxml_xpathCallbacksNb: libc::c_int = 0 as libc::c_int;
static mut libxmlMemoryDebugActivated: libc::c_int = 0 as libc::c_int;
static mut libxmlMemoryAllocatedBase: libc::c_long = 0 as libc::c_int as libc::c_long;
static mut libxmlMemoryDebug: libc::c_int = 0 as libc::c_int;
static mut freeFunc: xmlFreeFunc = None;
static mut mallocFunc: xmlMallocFunc = None;
static mut reallocFunc: xmlReallocFunc = None;
static mut strdupFunc: xmlStrdupFunc = None;
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlMemoryUsed(
    mut self_0: *mut PyObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut ret: libc::c_long = 0;
    let mut py_retval: *mut PyObject = 0 as *mut PyObject;
    ret = xmlMemUsed() as libc::c_long;
    py_retval = libxml_longWrap(ret);
    return py_retval;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlDebugMemory(
    mut self_0: *mut PyObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut current_block: u64;
    let mut activate: libc::c_int = 0;
    let mut py_retval: *mut PyObject = 0 as *mut PyObject;
    let mut ret: libc::c_long = 0;
    if _PyArg_ParseTuple_SizeT(
        args,
        b"i:xmlDebugMemory\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut activate as *mut libc::c_int,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    if activate != 0 as libc::c_int {
        if libxmlMemoryDebug == 0 as libc::c_int {
            xmlMemGet(
                &mut freeFunc as *mut xmlFreeFunc,
                &mut mallocFunc as *mut xmlMallocFunc,
                &mut reallocFunc as *mut xmlReallocFunc,
                &mut strdupFunc as *mut xmlStrdupFunc,
            );
            if freeFunc
                == Some(xmlMemFree as unsafe extern "C" fn(*mut libc::c_void) -> ())
                && mallocFunc
                    == Some(
                        xmlMemMalloc as unsafe extern "C" fn(size_t) -> *mut libc::c_void,
                    )
                && reallocFunc
                    == Some(
                        xmlMemRealloc
                            as unsafe extern "C" fn(
                                *mut libc::c_void,
                                size_t,
                            ) -> *mut libc::c_void,
                    )
                && strdupFunc
                    == Some(
                        xmlMemoryStrdup
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                            ) -> *mut libc::c_char,
                    )
            {
                libxmlMemoryAllocatedBase = xmlMemUsed() as libc::c_long;
                current_block = 10048703153582371463;
            } else {
                xmlCleanupParser();
                ret = xmlMemSetup(
                    Some(xmlMemFree as unsafe extern "C" fn(*mut libc::c_void) -> ()),
                    Some(
                        xmlMemMalloc as unsafe extern "C" fn(size_t) -> *mut libc::c_void,
                    ),
                    Some(
                        xmlMemRealloc
                            as unsafe extern "C" fn(
                                *mut libc::c_void,
                                size_t,
                            ) -> *mut libc::c_void,
                    ),
                    Some(
                        xmlMemoryStrdup
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                            ) -> *mut libc::c_char,
                    ),
                ) as libc::c_long;
                if ret < 0 as libc::c_int as libc::c_long {
                    current_block = 15224037586932921579;
                } else {
                    libxmlMemoryAllocatedBase = xmlMemUsed() as libc::c_long;
                    xmlInitParser();
                    libxml_xmlErrorInitialize();
                    current_block = 10048703153582371463;
                }
            }
            match current_block {
                15224037586932921579 => {}
                _ => {
                    ret = 0 as libc::c_int as libc::c_long;
                    current_block = 4495394744059808450;
                }
            }
        } else {
            if libxmlMemoryDebugActivated == 0 as libc::c_int {
                libxmlMemoryAllocatedBase = xmlMemUsed() as libc::c_long;
                ret = 0 as libc::c_int as libc::c_long;
            } else {
                ret = xmlMemUsed() as libc::c_long - libxmlMemoryAllocatedBase;
            }
            current_block = 4495394744059808450;
        }
        match current_block {
            15224037586932921579 => {}
            _ => {
                libxmlMemoryDebug = 1 as libc::c_int;
                libxmlMemoryDebugActivated = 1 as libc::c_int;
            }
        }
    } else {
        if libxmlMemoryDebugActivated == 1 as libc::c_int {
            ret = xmlMemUsed() as libc::c_long - libxmlMemoryAllocatedBase;
        } else {
            ret = 0 as libc::c_int as libc::c_long;
        }
        libxmlMemoryDebugActivated = 0 as libc::c_int;
    }
    py_retval = libxml_longWrap(ret);
    return py_retval;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlPythonCleanupParser(
    mut self_0: *mut PyObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut ix: libc::c_int = 0;
    let mut freed: libc::c_long = -(1 as libc::c_int) as libc::c_long;
    if libxmlMemoryDebug != 0 {
        freed = xmlMemUsed() as libc::c_long;
    }
    xmlCleanupParser();
    if !libxml_xpathCallbacks.is_null() {
        ix = 0 as libc::c_int;
        while ix < libxml_xpathCallbacksNb {
            if !((*(*libxml_xpathCallbacks).as_mut_ptr().offset(ix as isize)).name)
                .is_null()
            {
                xmlFree
                    .expect(
                        "non-null function pointer",
                    )(
                    (*(*libxml_xpathCallbacks).as_mut_ptr().offset(ix as isize)).name
                        as *mut libc::c_void,
                );
            }
            if !((*(*libxml_xpathCallbacks).as_mut_ptr().offset(ix as isize)).ns_uri)
                .is_null()
            {
                xmlFree
                    .expect(
                        "non-null function pointer",
                    )(
                    (*(*libxml_xpathCallbacks).as_mut_ptr().offset(ix as isize)).ns_uri
                        as *mut libc::c_void,
                );
            }
            ix += 1;
        }
        libxml_xpathCallbacksNb = 0 as libc::c_int;
        xmlFree
            .expect(
                "non-null function pointer",
            )(libxml_xpathCallbacks as *mut libc::c_void);
        libxml_xpathCallbacks = 0 as *mut libxml_xpathCallbackArray;
    }
    if libxmlMemoryDebug != 0 {
        freed -= xmlMemUsed() as libc::c_long;
        libxmlMemoryAllocatedBase -= freed;
        if libxmlMemoryAllocatedBase < 0 as libc::c_int as libc::c_long {
            libxmlMemoryAllocatedBase = 0 as libc::c_int as libc::c_long;
        }
    }
    let ref mut fresh0 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
    *fresh0 += 1;
    return &mut _Py_NoneStruct;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlDumpMemory(
    mut self_0: *mut PyObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    if libxmlMemoryDebug != 0 as libc::c_int {
        xmlMemoryDump();
    }
    let ref mut fresh1 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
    *fresh1 += 1;
    return &mut _Py_NoneStruct;
}
unsafe extern "C" fn xmlPythonFileCloseRaw(
    mut context: *mut libc::c_void,
) -> libc::c_int {
    let mut file: *mut PyObject = 0 as *mut PyObject;
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    file = context as *mut PyObject;
    if file.is_null() {
        return -(1 as libc::c_int);
    }
    ret = PyEval_CallMethod(
        file,
        b"close\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"()\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !ret.is_null() {
        let ref mut fresh2 = (*ret).ob_refcnt;
        *fresh2 -= 1;
        if !(*fresh2 != 0 as libc::c_int as libc::c_long) {
            (Some(((*(*ret).ob_type).tp_dealloc).expect("non-null function pointer")))
                .expect("non-null function pointer")(ret);
        }
    }
    let ref mut fresh3 = (*file).ob_refcnt;
    *fresh3 -= 1;
    if !(*fresh3 != 0 as libc::c_int as libc::c_long) {
        (Some(((*(*file).ob_type).tp_dealloc).expect("non-null function pointer")))
            .expect("non-null function pointer")(file);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn xmlPythonFileReadRaw(
    mut context: *mut libc::c_void,
    mut buffer: *mut libc::c_char,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut file: *mut PyObject = 0 as *mut PyObject;
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    let mut lenread: libc::c_int = -(1 as libc::c_int);
    let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
    file = context as *mut PyObject;
    if file.is_null() {
        return -(1 as libc::c_int);
    }
    ret = PyEval_CallMethod(
        file,
        b"read\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"(i)\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        len,
    );
    if ret.is_null() {
        printf(
            b"xmlPythonFileReadRaw: result is NULL\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    } else {
        if (*(*ret).ob_type).tp_flags & (1 as libc::c_long) << 27 as libc::c_int
            != 0 as libc::c_int as libc::c_long
        {
            lenread = PyString_Size(ret) as libc::c_int;
            data = PyString_AsString(ret);
        } else if (*(*ret).ob_type).tp_flags & (1 as libc::c_long) << 28 as libc::c_int
                != 0 as libc::c_int as libc::c_long
            {
            let mut b: *mut PyObject = 0 as *mut PyObject;
            b = PyUnicodeUCS4_AsUTF8String(ret);
            if b.is_null() {
                printf(
                    b"xmlPythonFileReadRaw: failed to convert to UTF-8\n\0" as *const u8
                        as *const libc::c_char,
                );
                return -(1 as libc::c_int);
            }
            lenread = PyString_Size(b) as libc::c_int;
            data = PyString_AsString(b);
            let ref mut fresh4 = (*b).ob_refcnt;
            *fresh4 -= 1;
            if !(*fresh4 != 0 as libc::c_int as libc::c_long) {
                (Some(((*(*b).ob_type).tp_dealloc).expect("non-null function pointer")))
                    .expect("non-null function pointer")(b);
            }
        } else {
            printf(
                b"xmlPythonFileReadRaw: result is not a String\n\0" as *const u8
                    as *const libc::c_char,
            );
            let ref mut fresh5 = (*ret).ob_refcnt;
            *fresh5 -= 1;
            if !(*fresh5 != 0 as libc::c_int as libc::c_long) {
                (Some(
                    ((*(*ret).ob_type).tp_dealloc).expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(ret);
            }
            return -(1 as libc::c_int);
        }
    }
    if lenread > len {
        memcpy(
            buffer as *mut libc::c_void,
            data as *const libc::c_void,
            len as libc::c_ulong,
        );
    } else {
        memcpy(
            buffer as *mut libc::c_void,
            data as *const libc::c_void,
            lenread as libc::c_ulong,
        );
    }
    let ref mut fresh6 = (*ret).ob_refcnt;
    *fresh6 -= 1;
    if !(*fresh6 != 0 as libc::c_int as libc::c_long) {
        (Some(((*(*ret).ob_type).tp_dealloc).expect("non-null function pointer")))
            .expect("non-null function pointer")(ret);
    }
    return lenread;
}
unsafe extern "C" fn xmlPythonFileRead(
    mut context: *mut libc::c_void,
    mut buffer: *mut libc::c_char,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut file: *mut PyObject = 0 as *mut PyObject;
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    let mut lenread: libc::c_int = -(1 as libc::c_int);
    let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
    file = context as *mut PyObject;
    if file.is_null() {
        return -(1 as libc::c_int);
    }
    ret = PyEval_CallMethod(
        file,
        b"io_read\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"(i)\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        len,
    );
    if ret.is_null() {
        printf(
            b"xmlPythonFileRead: result is NULL\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    } else {
        if (*(*ret).ob_type).tp_flags & (1 as libc::c_long) << 27 as libc::c_int
            != 0 as libc::c_int as libc::c_long
        {
            lenread = PyString_Size(ret) as libc::c_int;
            data = PyString_AsString(ret);
        } else if (*(*ret).ob_type).tp_flags & (1 as libc::c_long) << 28 as libc::c_int
                != 0 as libc::c_int as libc::c_long
            {
            let mut b: *mut PyObject = 0 as *mut PyObject;
            b = PyUnicodeUCS4_AsUTF8String(ret);
            if b.is_null() {
                printf(
                    b"xmlPythonFileRead: failed to convert to UTF-8\n\0" as *const u8
                        as *const libc::c_char,
                );
                return -(1 as libc::c_int);
            }
            lenread = PyString_Size(b) as libc::c_int;
            data = PyString_AsString(b);
            let ref mut fresh7 = (*b).ob_refcnt;
            *fresh7 -= 1;
            if !(*fresh7 != 0 as libc::c_int as libc::c_long) {
                (Some(((*(*b).ob_type).tp_dealloc).expect("non-null function pointer")))
                    .expect("non-null function pointer")(b);
            }
        } else {
            printf(
                b"xmlPythonFileRead: result is not a String\n\0" as *const u8
                    as *const libc::c_char,
            );
            let ref mut fresh8 = (*ret).ob_refcnt;
            *fresh8 -= 1;
            if !(*fresh8 != 0 as libc::c_int as libc::c_long) {
                (Some(
                    ((*(*ret).ob_type).tp_dealloc).expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(ret);
            }
            return -(1 as libc::c_int);
        }
    }
    if lenread > len {
        memcpy(
            buffer as *mut libc::c_void,
            data as *const libc::c_void,
            len as libc::c_ulong,
        );
    } else {
        memcpy(
            buffer as *mut libc::c_void,
            data as *const libc::c_void,
            lenread as libc::c_ulong,
        );
    }
    let ref mut fresh9 = (*ret).ob_refcnt;
    *fresh9 -= 1;
    if !(*fresh9 != 0 as libc::c_int as libc::c_long) {
        (Some(((*(*ret).ob_type).tp_dealloc).expect("non-null function pointer")))
            .expect("non-null function pointer")(ret);
    }
    return lenread;
}
unsafe extern "C" fn xmlPythonFileWrite(
    mut context: *mut libc::c_void,
    mut buffer: *const libc::c_char,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut file: *mut PyObject = 0 as *mut PyObject;
    let mut string: *mut PyObject = 0 as *mut PyObject;
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    let mut written: libc::c_int = -(1 as libc::c_int);
    file = context as *mut PyObject;
    if file.is_null() {
        return -(1 as libc::c_int);
    }
    string = PyString_FromStringAndSize(buffer, len as Py_ssize_t);
    if string.is_null() {
        return -(1 as libc::c_int);
    }
    if PyObject_HasAttrString(
        file,
        b"io_write\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) != 0
    {
        ret = PyEval_CallMethod(
            file,
            b"io_write\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"(O)\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            string,
        );
    } else if PyObject_HasAttrString(
            file,
            b"write\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) != 0
        {
        ret = PyEval_CallMethod(
            file,
            b"write\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"(O)\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            string,
        );
    }
    let ref mut fresh10 = (*string).ob_refcnt;
    *fresh10 -= 1;
    if !(*fresh10 != 0 as libc::c_int as libc::c_long) {
        (Some(((*(*string).ob_type).tp_dealloc).expect("non-null function pointer")))
            .expect("non-null function pointer")(string);
    }
    if ret.is_null() {
        printf(
            b"xmlPythonFileWrite: result is NULL\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    } else {
        if (*(*ret).ob_type).tp_flags & (1 as libc::c_long) << 24 as libc::c_int
            != 0 as libc::c_int as libc::c_long
        {
            written = PyLong_AsLong(ret) as libc::c_int;
            let ref mut fresh11 = (*ret).ob_refcnt;
            *fresh11 -= 1;
            if !(*fresh11 != 0 as libc::c_int as libc::c_long) {
                (Some(
                    ((*(*ret).ob_type).tp_dealloc).expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(ret);
            }
        } else if ret == &mut _Py_NoneStruct as *mut PyObject {
            written = len;
            let ref mut fresh12 = (*ret).ob_refcnt;
            *fresh12 -= 1;
            if !(*fresh12 != 0 as libc::c_int as libc::c_long) {
                (Some(
                    ((*(*ret).ob_type).tp_dealloc).expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(ret);
            }
        } else {
            printf(
                b"xmlPythonFileWrite: result is not an Int nor None\n\0" as *const u8
                    as *const libc::c_char,
            );
            let ref mut fresh13 = (*ret).ob_refcnt;
            *fresh13 -= 1;
            if !(*fresh13 != 0 as libc::c_int as libc::c_long) {
                (Some(
                    ((*(*ret).ob_type).tp_dealloc).expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(ret);
            }
        }
    }
    return written;
}
unsafe extern "C" fn xmlPythonFileClose(mut context: *mut libc::c_void) -> libc::c_int {
    let mut file: *mut PyObject = 0 as *mut PyObject;
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    file = context as *mut PyObject;
    if file.is_null() {
        return -(1 as libc::c_int);
    }
    if PyObject_HasAttrString(
        file,
        b"io_close\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) != 0
    {
        ret = PyEval_CallMethod(
            file,
            b"io_close\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"()\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    } else if PyObject_HasAttrString(
            file,
            b"flush\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) != 0
        {
        ret = PyEval_CallMethod(
            file,
            b"flush\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"()\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    if !ret.is_null() {
        let ref mut fresh14 = (*ret).ob_refcnt;
        *fresh14 -= 1;
        if !(*fresh14 != 0 as libc::c_int as libc::c_long) {
            (Some(((*(*ret).ob_type).tp_dealloc).expect("non-null function pointer")))
                .expect("non-null function pointer")(ret);
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn xmlOutputBufferCreatePythonFile(
    mut file: *mut PyObject,
    mut encoder: xmlCharEncodingHandlerPtr,
) -> xmlOutputBufferPtr {
    let mut ret: xmlOutputBufferPtr = 0 as *mut xmlOutputBuffer;
    if file.is_null() {
        return 0 as xmlOutputBufferPtr;
    }
    ret = xmlAllocOutputBuffer(encoder);
    if !ret.is_null() {
        let ref mut fresh15 = (*ret).context;
        *fresh15 = file as *mut libc::c_void;
        let ref mut fresh16 = (*ret).writecallback;
        *fresh16 = Some(
            xmlPythonFileWrite
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                    libc::c_int,
                ) -> libc::c_int,
        );
        let ref mut fresh17 = (*ret).closecallback;
        *fresh17 = Some(
            xmlPythonFileClose as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
        );
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlCreateOutputBuffer(
    mut self_0: *mut PyObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut py_retval: *mut PyObject = 0 as *mut PyObject;
    let mut file: *mut PyObject = 0 as *mut PyObject;
    let mut encoding: *mut xmlChar = 0 as *mut xmlChar;
    let mut handler: xmlCharEncodingHandlerPtr = 0 as xmlCharEncodingHandlerPtr;
    let mut buffer: xmlOutputBufferPtr = 0 as *mut xmlOutputBuffer;
    if _PyArg_ParseTuple_SizeT(
        args,
        b"Oz:xmlOutputBufferCreate\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        &mut file as *mut *mut PyObject,
        &mut encoding as *mut *mut xmlChar,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    if !encoding.is_null()
        && *encoding.offset(0 as libc::c_int as isize) as libc::c_int != 0 as libc::c_int
    {
        handler = xmlFindCharEncodingHandler(encoding as *const libc::c_char);
    }
    buffer = xmlOutputBufferCreatePythonFile(file, handler);
    if buffer.is_null() {
        printf(
            b"libxml_xmlCreateOutputBuffer: buffer == NULL\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    py_retval = libxml_xmlOutputBufferPtrWrap(buffer);
    return py_retval;
}
unsafe extern "C" fn libxml_outputBufferGetPythonFile(
    mut self_0: *mut PyObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut buffer: *mut PyObject = 0 as *mut PyObject;
    let mut file: *mut PyObject = 0 as *mut PyObject;
    let mut obj: xmlOutputBufferPtr = 0 as *mut xmlOutputBuffer;
    if _PyArg_ParseTuple_SizeT(
        args,
        b"O:outputBufferGetPythonFile\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        &mut buffer as *mut *mut PyObject,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    obj = if buffer == &mut _Py_NoneStruct as *mut PyObject {
        0 as xmlOutputBufferPtr
    } else {
        (*(buffer as *mut PyoutputBuffer_Object)).obj
    };
    if obj.is_null() {
        fprintf(
            stderr,
            b"outputBufferGetPythonFile: obj == NULL\n\0" as *const u8
                as *const libc::c_char,
        );
        let ref mut fresh18 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
        *fresh18 += 1;
        return &mut _Py_NoneStruct;
    }
    if (*obj).closecallback
        != Some(
            xmlPythonFileClose as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
        )
    {
        fprintf(
            stderr,
            b"outputBufferGetPythonFile: not a python file wrapper\n\0" as *const u8
                as *const libc::c_char,
        );
        let ref mut fresh19 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
        *fresh19 += 1;
        return &mut _Py_NoneStruct;
    }
    file = (*obj).context as *mut PyObject;
    if file.is_null() {
        let ref mut fresh20 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
        *fresh20 += 1;
        return &mut _Py_NoneStruct;
    }
    let ref mut fresh21 = (*file).ob_refcnt;
    *fresh21 += 1;
    return file;
}
unsafe extern "C" fn libxml_xmlOutputBufferClose(
    mut self_0: *mut PyObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut py_retval: *mut PyObject = 0 as *mut PyObject;
    let mut c_retval: libc::c_int = 0;
    let mut out: xmlOutputBufferPtr = 0 as *mut xmlOutputBuffer;
    let mut pyobj_out: *mut PyObject = 0 as *mut PyObject;
    if _PyArg_ParseTuple_SizeT(
        args,
        b"O:xmlOutputBufferClose\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        &mut pyobj_out as *mut *mut PyObject,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    out = if pyobj_out == &mut _Py_NoneStruct as *mut PyObject {
        0 as xmlOutputBufferPtr
    } else {
        (*(pyobj_out as *mut PyoutputBuffer_Object)).obj
    };
    if out.is_null() {
        let ref mut fresh22 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
        *fresh22 += 1;
        return &mut _Py_NoneStruct;
    }
    c_retval = xmlOutputBufferClose(out);
    py_retval = libxml_intWrap(c_retval);
    return py_retval;
}
unsafe extern "C" fn libxml_xmlOutputBufferFlush(
    mut self_0: *mut PyObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut py_retval: *mut PyObject = 0 as *mut PyObject;
    let mut c_retval: libc::c_int = 0;
    let mut out: xmlOutputBufferPtr = 0 as *mut xmlOutputBuffer;
    let mut pyobj_out: *mut PyObject = 0 as *mut PyObject;
    if _PyArg_ParseTuple_SizeT(
        args,
        b"O:xmlOutputBufferFlush\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        &mut pyobj_out as *mut *mut PyObject,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    out = if pyobj_out == &mut _Py_NoneStruct as *mut PyObject {
        0 as xmlOutputBufferPtr
    } else {
        (*(pyobj_out as *mut PyoutputBuffer_Object)).obj
    };
    c_retval = xmlOutputBufferFlush(out);
    py_retval = libxml_intWrap(c_retval);
    return py_retval;
}
unsafe extern "C" fn libxml_xmlSaveFileTo(
    mut self_0: *mut PyObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut py_retval: *mut PyObject = 0 as *mut PyObject;
    let mut c_retval: libc::c_int = 0;
    let mut buf: xmlOutputBufferPtr = 0 as *mut xmlOutputBuffer;
    let mut pyobj_buf: *mut PyObject = 0 as *mut PyObject;
    let mut cur: xmlDocPtr = 0 as *mut xmlDoc;
    let mut pyobj_cur: *mut PyObject = 0 as *mut PyObject;
    let mut encoding: *mut libc::c_char = 0 as *mut libc::c_char;
    if _PyArg_ParseTuple_SizeT(
        args,
        b"OOz:xmlSaveFileTo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut pyobj_buf as *mut *mut PyObject,
        &mut pyobj_cur as *mut *mut PyObject,
        &mut encoding as *mut *mut libc::c_char,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    buf = if pyobj_buf == &mut _Py_NoneStruct as *mut PyObject {
        0 as xmlOutputBufferPtr
    } else {
        (*(pyobj_buf as *mut PyoutputBuffer_Object)).obj
    };
    cur = (if pyobj_cur == &mut _Py_NoneStruct as *mut PyObject {
        0 as xmlNodePtr
    } else {
        (*(pyobj_cur as *mut PyxmlNode_Object)).obj
    }) as xmlDocPtr;
    c_retval = xmlSaveFileTo(buf, cur, encoding);
    let ref mut fresh23 = (*(pyobj_buf as *mut PyoutputBuffer_Object)).obj;
    *fresh23 = 0 as xmlOutputBufferPtr;
    py_retval = libxml_intWrap(c_retval);
    return py_retval;
}
unsafe extern "C" fn libxml_xmlSaveFormatFileTo(
    mut self_0: *mut PyObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut py_retval: *mut PyObject = 0 as *mut PyObject;
    let mut c_retval: libc::c_int = 0;
    let mut buf: xmlOutputBufferPtr = 0 as *mut xmlOutputBuffer;
    let mut pyobj_buf: *mut PyObject = 0 as *mut PyObject;
    let mut cur: xmlDocPtr = 0 as *mut xmlDoc;
    let mut pyobj_cur: *mut PyObject = 0 as *mut PyObject;
    let mut encoding: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut format: libc::c_int = 0;
    if _PyArg_ParseTuple_SizeT(
        args,
        b"OOzi:xmlSaveFormatFileTo\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        &mut pyobj_buf as *mut *mut PyObject,
        &mut pyobj_cur as *mut *mut PyObject,
        &mut encoding as *mut *mut libc::c_char,
        &mut format as *mut libc::c_int,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    buf = if pyobj_buf == &mut _Py_NoneStruct as *mut PyObject {
        0 as xmlOutputBufferPtr
    } else {
        (*(pyobj_buf as *mut PyoutputBuffer_Object)).obj
    };
    cur = (if pyobj_cur == &mut _Py_NoneStruct as *mut PyObject {
        0 as xmlNodePtr
    } else {
        (*(pyobj_cur as *mut PyxmlNode_Object)).obj
    }) as xmlDocPtr;
    c_retval = xmlSaveFormatFileTo(buf, cur, encoding, format);
    let ref mut fresh24 = (*(pyobj_buf as *mut PyoutputBuffer_Object)).obj;
    *fresh24 = 0 as xmlOutputBufferPtr;
    py_retval = libxml_intWrap(c_retval);
    return py_retval;
}
unsafe extern "C" fn xmlParserInputBufferCreatePythonFile(
    mut file: *mut PyObject,
    mut encoding: xmlCharEncoding,
) -> xmlParserInputBufferPtr {
    let mut ret: xmlParserInputBufferPtr = 0 as *mut xmlParserInputBuffer;
    if file.is_null() {
        return 0 as xmlParserInputBufferPtr;
    }
    ret = xmlAllocParserInputBuffer(encoding);
    if !ret.is_null() {
        let ref mut fresh25 = (*ret).context;
        *fresh25 = file as *mut libc::c_void;
        let ref mut fresh26 = (*ret).readcallback;
        *fresh26 = Some(
            xmlPythonFileRead
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_char,
                    libc::c_int,
                ) -> libc::c_int,
        );
        let ref mut fresh27 = (*ret).closecallback;
        *fresh27 = Some(
            xmlPythonFileClose as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
        );
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlCreateInputBuffer(
    mut self_0: *mut PyObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut py_retval: *mut PyObject = 0 as *mut PyObject;
    let mut file: *mut PyObject = 0 as *mut PyObject;
    let mut encoding: *mut xmlChar = 0 as *mut xmlChar;
    let mut enc: xmlCharEncoding = XML_CHAR_ENCODING_NONE;
    let mut buffer: xmlParserInputBufferPtr = 0 as *mut xmlParserInputBuffer;
    if _PyArg_ParseTuple_SizeT(
        args,
        b"Oz:xmlParserInputBufferCreate\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        &mut file as *mut *mut PyObject,
        &mut encoding as *mut *mut xmlChar,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    if !encoding.is_null()
        && *encoding.offset(0 as libc::c_int as isize) as libc::c_int != 0 as libc::c_int
    {
        enc = xmlParseCharEncoding(encoding as *const libc::c_char);
    }
    buffer = xmlParserInputBufferCreatePythonFile(file, enc);
    if buffer.is_null() {
        printf(
            b"libxml_xmlParserInputBufferCreate: buffer == NULL\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    py_retval = libxml_xmlParserInputBufferPtrWrap(buffer);
    return py_retval;
}
static mut defaultExternalEntityLoader: xmlExternalEntityLoader = None;
static mut pythonExternalEntityLoaderObjext: *mut PyObject = 0 as *const PyObject
    as *mut PyObject;
unsafe extern "C" fn pythonExternalEntityLoader(
    mut URL: *const libc::c_char,
    mut ID: *const libc::c_char,
    mut ctxt: xmlParserCtxtPtr,
) -> xmlParserInputPtr {
    let mut result: xmlParserInputPtr = 0 as xmlParserInputPtr;
    if !pythonExternalEntityLoaderObjext.is_null() {
        let mut ret: *mut PyObject = 0 as *mut PyObject;
        let mut ctxtobj: *mut PyObject = 0 as *mut PyObject;
        ctxtobj = libxml_xmlParserCtxtPtrWrap(ctxt);
        ret = _PyObject_CallFunction_SizeT(
            pythonExternalEntityLoaderObjext,
            b"(ssO)\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            URL,
            ID,
            ctxtobj,
        );
        if !ctxtobj.is_null() {
            let ref mut fresh28 = (*ctxtobj).ob_refcnt;
            *fresh28 -= 1;
            if !(*fresh28 != 0 as libc::c_int as libc::c_long) {
                (Some(
                    ((*(*ctxtobj).ob_type).tp_dealloc)
                        .expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(ctxtobj);
            }
        }
        if !ret.is_null() {
            if PyObject_HasAttrString(
                ret,
                b"read\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) != 0
            {
                let mut buf: xmlParserInputBufferPtr = 0 as *mut xmlParserInputBuffer;
                buf = xmlAllocParserInputBuffer(XML_CHAR_ENCODING_NONE);
                if !buf.is_null() {
                    let ref mut fresh29 = (*buf).context;
                    *fresh29 = ret as *mut libc::c_void;
                    let ref mut fresh30 = (*buf).readcallback;
                    *fresh30 = Some(
                        xmlPythonFileReadRaw
                            as unsafe extern "C" fn(
                                *mut libc::c_void,
                                *mut libc::c_char,
                                libc::c_int,
                            ) -> libc::c_int,
                    );
                    let ref mut fresh31 = (*buf).closecallback;
                    *fresh31 = Some(
                        xmlPythonFileCloseRaw
                            as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
                    );
                    result = xmlNewIOInputStream(ctxt, buf, XML_CHAR_ENCODING_NONE);
                }
            }
            if result.is_null() {
                let ref mut fresh32 = (*ret).ob_refcnt;
                *fresh32 -= 1;
                if !(*fresh32 != 0 as libc::c_int as libc::c_long) {
                    (Some(
                        ((*(*ret).ob_type).tp_dealloc)
                            .expect("non-null function pointer"),
                    ))
                        .expect("non-null function pointer")(ret);
                }
            } else if !URL.is_null() {
                let ref mut fresh33 = (*result).filename;
                *fresh33 = xmlStrdup(URL as *const xmlChar) as *mut libc::c_char;
                let ref mut fresh34 = (*result).directory;
                *fresh34 = xmlParserGetDirectory(URL);
            }
        }
    }
    if result.is_null() && defaultExternalEntityLoader.is_some() {
        result = defaultExternalEntityLoader
            .expect("non-null function pointer")(URL, ID, ctxt);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlSetEntityLoader(
    mut self_0: *mut PyObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut py_retval: *mut PyObject = 0 as *mut PyObject;
    let mut loader: *mut PyObject = 0 as *mut PyObject;
    if _PyArg_ParseTuple_SizeT(
        args,
        b"O:libxml_xmlSetEntityLoader\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        &mut loader as *mut *mut PyObject,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    if PyCallable_Check(loader) == 0 {
        PyErr_SetString(
            PyExc_ValueError,
            b"entity loader is not callable\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut PyObject;
    }
    if defaultExternalEntityLoader.is_none() {
        defaultExternalEntityLoader = xmlGetExternalEntityLoader();
    }
    if !pythonExternalEntityLoaderObjext.is_null() {
        let ref mut fresh35 = (*pythonExternalEntityLoaderObjext).ob_refcnt;
        *fresh35 -= 1;
        if !(*fresh35 != 0 as libc::c_int as libc::c_long) {
            (Some(
                ((*(*pythonExternalEntityLoaderObjext).ob_type).tp_dealloc)
                    .expect("non-null function pointer"),
            ))
                .expect("non-null function pointer")(pythonExternalEntityLoaderObjext);
        }
    }
    pythonExternalEntityLoaderObjext = loader;
    if !pythonExternalEntityLoaderObjext.is_null() {
        let ref mut fresh36 = (*pythonExternalEntityLoaderObjext).ob_refcnt;
        *fresh36 += 1;
    }
    xmlSetExternalEntityLoader(
        Some(
            pythonExternalEntityLoader
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                    xmlParserCtxtPtr,
                ) -> xmlParserInputPtr,
        ),
    );
    py_retval = PyLong_FromLong(0 as libc::c_int as libc::c_long);
    return py_retval;
}
static mut pythonInputOpenCallbackObject: *mut PyObject = 0 as *const PyObject
    as *mut PyObject;
static mut pythonInputCallbackID: libc::c_int = -(1 as libc::c_int);
unsafe extern "C" fn pythonInputMatchCallback(
    mut URI: *const libc::c_char,
) -> libc::c_int {
    return 1 as libc::c_int;
}
unsafe extern "C" fn pythonInputOpenCallback(
    mut URI: *const libc::c_char,
) -> *mut libc::c_void {
    let mut ret: *mut PyObject = 0 as *mut PyObject;
    ret = _PyObject_CallFunction_SizeT(
        pythonInputOpenCallbackObject,
        b"s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        URI,
    );
    if ret == &mut _Py_NoneStruct as *mut PyObject {
        let ref mut fresh37 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
        *fresh37 -= 1;
        if !(*fresh37 != 0 as libc::c_int as libc::c_long) {
            (Some(
                ((*(*(&mut _Py_NoneStruct as *mut PyObject)).ob_type).tp_dealloc)
                    .expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(&mut _Py_NoneStruct as *mut PyObject);
        }
        return 0 as *mut libc::c_void;
    }
    return ret as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlRegisterInputCallback(
    mut self_0: *mut PyObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut cb: *mut PyObject = 0 as *mut PyObject;
    if _PyArg_ParseTuple_SizeT(
        args,
        b"O:libxml_xmlRegisterInputCallback\0" as *const u8 as *const libc::c_char,
        &mut cb as *mut *mut PyObject,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    if PyCallable_Check(cb) == 0 {
        PyErr_SetString(
            PyExc_ValueError,
            b"input callback is not callable\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut PyObject;
    }
    if pythonInputCallbackID == -(1 as libc::c_int) {
        pythonInputCallbackID = xmlRegisterInputCallbacks(
            Some(
                pythonInputMatchCallback
                    as unsafe extern "C" fn(*const libc::c_char) -> libc::c_int,
            ),
            Some(
                pythonInputOpenCallback
                    as unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_void,
            ),
            Some(
                xmlPythonFileReadRaw
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_char,
                        libc::c_int,
                    ) -> libc::c_int,
            ),
            Some(
                xmlPythonFileCloseRaw
                    as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
            ),
        );
        if pythonInputCallbackID == -(1 as libc::c_int) {
            return PyErr_NoMemory();
        }
        pythonInputOpenCallbackObject = cb;
        let ref mut fresh38 = (*pythonInputOpenCallbackObject).ob_refcnt;
        *fresh38 += 1;
    }
    let ref mut fresh39 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
    *fresh39 += 1;
    return &mut _Py_NoneStruct;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlUnregisterInputCallback(
    mut self_0: *mut PyObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut ret: libc::c_int = 0;
    ret = xmlPopInputCallbacks();
    if pythonInputCallbackID != -(1 as libc::c_int) {
        if pythonInputCallbackID == ret {
            pythonInputCallbackID = -(1 as libc::c_int);
            let ref mut fresh40 = (*pythonInputOpenCallbackObject).ob_refcnt;
            *fresh40 -= 1;
            if !(*fresh40 != 0 as libc::c_int as libc::c_long) {
                (Some(
                    ((*(*pythonInputOpenCallbackObject).ob_type).tp_dealloc)
                        .expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(pythonInputOpenCallbackObject);
            }
            pythonInputOpenCallbackObject = 0 as *mut PyObject;
        } else {
            PyErr_SetString(
                PyExc_AssertionError,
                b"popped non-python input callback\0" as *const u8 as *const libc::c_char,
            );
            return 0 as *mut PyObject;
        }
    } else if ret == -(1 as libc::c_int) {
        PyErr_SetString(
            PyExc_IndexError,
            b"no input callbacks to pop\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut PyObject;
    }
    let ref mut fresh41 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
    *fresh41 += 1;
    return &mut _Py_NoneStruct;
}
unsafe extern "C" fn pythonStartElement(
    mut user_data: *mut libc::c_void,
    mut name: *const xmlChar,
    mut attrs: *mut *const xmlChar,
) {
    let mut i: libc::c_int = 0;
    let mut handler: *mut PyObject = 0 as *mut PyObject;
    let mut dict: *mut PyObject = 0 as *mut PyObject;
    let mut attrname: *mut PyObject = 0 as *mut PyObject;
    let mut attrvalue: *mut PyObject = 0 as *mut PyObject;
    let mut result: *mut PyObject = 0 as *mut PyObject;
    let mut type_0: libc::c_int = 0 as libc::c_int;
    handler = user_data as *mut PyObject;
    if PyObject_HasAttrString(
        handler,
        b"startElement\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) != 0
    {
        type_0 = 1 as libc::c_int;
    } else if PyObject_HasAttrString(
            handler,
            b"start\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) != 0
        {
        type_0 = 2 as libc::c_int;
    }
    if type_0 != 0 as libc::c_int {
        if attrs.is_null() && type_0 == 1 as libc::c_int {
            if !(&mut _Py_NoneStruct as *mut PyObject).is_null() {
                let ref mut fresh42 = (*(&mut _Py_NoneStruct as *mut PyObject))
                    .ob_refcnt;
                *fresh42 += 1;
            }
            dict = &mut _Py_NoneStruct;
        } else if attrs.is_null() {
            dict = PyDict_New();
        } else {
            dict = PyDict_New();
            i = 0 as libc::c_int;
            while !(*attrs.offset(i as isize)).is_null() {
                attrname = PyString_FromString(
                    *attrs.offset(i as isize) as *mut libc::c_char,
                );
                i += 1;
                if !(*attrs.offset(i as isize)).is_null() {
                    attrvalue = PyString_FromString(
                        *attrs.offset(i as isize) as *mut libc::c_char,
                    );
                } else {
                    if !(&mut _Py_NoneStruct as *mut PyObject).is_null() {
                        let ref mut fresh43 = (*(&mut _Py_NoneStruct as *mut PyObject))
                            .ob_refcnt;
                        *fresh43 += 1;
                    }
                    attrvalue = &mut _Py_NoneStruct;
                }
                PyDict_SetItem(dict, attrname, attrvalue);
                let ref mut fresh44 = (*attrname).ob_refcnt;
                *fresh44 -= 1;
                if !(*fresh44 != 0 as libc::c_int as libc::c_long) {
                    (Some(
                        ((*(*attrname).ob_type).tp_dealloc)
                            .expect("non-null function pointer"),
                    ))
                        .expect("non-null function pointer")(attrname);
                }
                let ref mut fresh45 = (*attrvalue).ob_refcnt;
                *fresh45 -= 1;
                if !(*fresh45 != 0 as libc::c_int as libc::c_long) {
                    (Some(
                        ((*(*attrvalue).ob_type).tp_dealloc)
                            .expect("non-null function pointer"),
                    ))
                        .expect("non-null function pointer")(attrvalue);
                }
                i += 1;
            }
        }
        if type_0 == 1 as libc::c_int {
            result = _PyObject_CallMethod_SizeT(
                handler,
                b"startElement\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                b"sO\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                name,
                dict,
            );
        } else if type_0 == 2 as libc::c_int {
            result = _PyObject_CallMethod_SizeT(
                handler,
                b"start\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"sO\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                name,
                dict,
            );
        }
        if !(PyErr_Occurred()).is_null() {
            PyErr_Print();
        }
        if !dict.is_null() {
            let ref mut fresh46 = (*dict).ob_refcnt;
            *fresh46 -= 1;
            if !(*fresh46 != 0 as libc::c_int as libc::c_long) {
                (Some(
                    ((*(*dict).ob_type).tp_dealloc).expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(dict);
            }
        }
        if !result.is_null() {
            let ref mut fresh47 = (*result).ob_refcnt;
            *fresh47 -= 1;
            if !(*fresh47 != 0 as libc::c_int as libc::c_long) {
                (Some(
                    ((*(*result).ob_type).tp_dealloc).expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(result);
            }
        }
    }
}
unsafe extern "C" fn pythonStartDocument(mut user_data: *mut libc::c_void) {
    let mut handler: *mut PyObject = 0 as *mut PyObject;
    let mut result: *mut PyObject = 0 as *mut PyObject;
    handler = user_data as *mut PyObject;
    if PyObject_HasAttrString(
        handler,
        b"startDocument\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) != 0
    {
        result = _PyObject_CallMethod_SizeT(
            handler,
            b"startDocument\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *mut libc::c_char,
        );
        if !(PyErr_Occurred()).is_null() {
            PyErr_Print();
        }
        if !result.is_null() {
            let ref mut fresh48 = (*result).ob_refcnt;
            *fresh48 -= 1;
            if !(*fresh48 != 0 as libc::c_int as libc::c_long) {
                (Some(
                    ((*(*result).ob_type).tp_dealloc).expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(result);
            }
        }
    }
}
unsafe extern "C" fn pythonEndDocument(mut user_data: *mut libc::c_void) {
    let mut handler: *mut PyObject = 0 as *mut PyObject;
    let mut result: *mut PyObject = 0 as *mut PyObject;
    handler = user_data as *mut PyObject;
    if PyObject_HasAttrString(
        handler,
        b"endDocument\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) != 0
    {
        result = _PyObject_CallMethod_SizeT(
            handler,
            b"endDocument\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *mut libc::c_char,
        );
        if !(PyErr_Occurred()).is_null() {
            PyErr_Print();
        }
        if !result.is_null() {
            let ref mut fresh49 = (*result).ob_refcnt;
            *fresh49 -= 1;
            if !(*fresh49 != 0 as libc::c_int as libc::c_long) {
                (Some(
                    ((*(*result).ob_type).tp_dealloc).expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(result);
            }
        }
    }
    if !handler.is_null() {
        let ref mut fresh50 = (*handler).ob_refcnt;
        *fresh50 -= 1;
        if !(*fresh50 != 0 as libc::c_int as libc::c_long) {
            (Some(
                ((*(*handler).ob_type).tp_dealloc).expect("non-null function pointer"),
            ))
                .expect("non-null function pointer")(handler);
        }
    }
}
unsafe extern "C" fn pythonEndElement(
    mut user_data: *mut libc::c_void,
    mut name: *const xmlChar,
) {
    let mut handler: *mut PyObject = 0 as *mut PyObject;
    let mut result: *mut PyObject = 0 as *mut PyObject;
    handler = user_data as *mut PyObject;
    if PyObject_HasAttrString(
        handler,
        b"endElement\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) != 0
    {
        result = _PyObject_CallMethod_SizeT(
            handler,
            b"endElement\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            name,
        );
        if !(PyErr_Occurred()).is_null() {
            PyErr_Print();
        }
        if !result.is_null() {
            let ref mut fresh51 = (*result).ob_refcnt;
            *fresh51 -= 1;
            if !(*fresh51 != 0 as libc::c_int as libc::c_long) {
                (Some(
                    ((*(*result).ob_type).tp_dealloc).expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(result);
            }
        }
    } else if PyObject_HasAttrString(
            handler,
            b"end\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) != 0
        {
        result = _PyObject_CallMethod_SizeT(
            handler,
            b"end\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            name,
        );
        if !(PyErr_Occurred()).is_null() {
            PyErr_Print();
        }
        if !result.is_null() {
            let ref mut fresh52 = (*result).ob_refcnt;
            *fresh52 -= 1;
            if !(*fresh52 != 0 as libc::c_int as libc::c_long) {
                (Some(
                    ((*(*result).ob_type).tp_dealloc).expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(result);
            }
        }
    }
}
unsafe extern "C" fn pythonReference(
    mut user_data: *mut libc::c_void,
    mut name: *const xmlChar,
) {
    let mut handler: *mut PyObject = 0 as *mut PyObject;
    let mut result: *mut PyObject = 0 as *mut PyObject;
    handler = user_data as *mut PyObject;
    if PyObject_HasAttrString(
        handler,
        b"reference\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) != 0
    {
        result = _PyObject_CallMethod_SizeT(
            handler,
            b"reference\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            name,
        );
        if !(PyErr_Occurred()).is_null() {
            PyErr_Print();
        }
        if !result.is_null() {
            let ref mut fresh53 = (*result).ob_refcnt;
            *fresh53 -= 1;
            if !(*fresh53 != 0 as libc::c_int as libc::c_long) {
                (Some(
                    ((*(*result).ob_type).tp_dealloc).expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(result);
            }
        }
    }
}
unsafe extern "C" fn pythonCharacters(
    mut user_data: *mut libc::c_void,
    mut ch: *const xmlChar,
    mut len: libc::c_int,
) {
    let mut handler: *mut PyObject = 0 as *mut PyObject;
    let mut result: *mut PyObject = 0 as *mut PyObject;
    let mut type_0: libc::c_int = 0 as libc::c_int;
    handler = user_data as *mut PyObject;
    if PyObject_HasAttrString(
        handler,
        b"characters\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) != 0
    {
        type_0 = 1 as libc::c_int;
    } else if PyObject_HasAttrString(
            handler,
            b"data\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) != 0
        {
        type_0 = 2 as libc::c_int;
    }
    if type_0 != 0 as libc::c_int {
        if type_0 == 1 as libc::c_int {
            result = _PyObject_CallMethod_SizeT(
                handler,
                b"characters\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"s#\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ch,
                len as Py_ssize_t,
            );
        } else if type_0 == 2 as libc::c_int {
            result = _PyObject_CallMethod_SizeT(
                handler,
                b"data\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"s#\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ch,
                len as Py_ssize_t,
            );
        }
        if !(PyErr_Occurred()).is_null() {
            PyErr_Print();
        }
        if !result.is_null() {
            let ref mut fresh54 = (*result).ob_refcnt;
            *fresh54 -= 1;
            if !(*fresh54 != 0 as libc::c_int as libc::c_long) {
                (Some(
                    ((*(*result).ob_type).tp_dealloc).expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(result);
            }
        }
    }
}
unsafe extern "C" fn pythonIgnorableWhitespace(
    mut user_data: *mut libc::c_void,
    mut ch: *const xmlChar,
    mut len: libc::c_int,
) {
    let mut handler: *mut PyObject = 0 as *mut PyObject;
    let mut result: *mut PyObject = 0 as *mut PyObject;
    let mut type_0: libc::c_int = 0 as libc::c_int;
    handler = user_data as *mut PyObject;
    if PyObject_HasAttrString(
        handler,
        b"ignorableWhitespace\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) != 0
    {
        type_0 = 1 as libc::c_int;
    } else if PyObject_HasAttrString(
            handler,
            b"data\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) != 0
        {
        type_0 = 2 as libc::c_int;
    }
    if type_0 != 0 as libc::c_int {
        if type_0 == 1 as libc::c_int {
            result = _PyObject_CallMethod_SizeT(
                handler,
                b"ignorableWhitespace\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                b"s#\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ch,
                len as Py_ssize_t,
            );
        } else if type_0 == 2 as libc::c_int {
            result = _PyObject_CallMethod_SizeT(
                handler,
                b"data\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"s#\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ch,
                len as Py_ssize_t,
            );
        }
        if !result.is_null() {
            let ref mut fresh55 = (*result).ob_refcnt;
            *fresh55 -= 1;
            if !(*fresh55 != 0 as libc::c_int as libc::c_long) {
                (Some(
                    ((*(*result).ob_type).tp_dealloc).expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(result);
            }
        }
    }
}
unsafe extern "C" fn pythonProcessingInstruction(
    mut user_data: *mut libc::c_void,
    mut target: *const xmlChar,
    mut data: *const xmlChar,
) {
    let mut handler: *mut PyObject = 0 as *mut PyObject;
    let mut result: *mut PyObject = 0 as *mut PyObject;
    handler = user_data as *mut PyObject;
    if PyObject_HasAttrString(
        handler,
        b"processingInstruction\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    ) != 0
    {
        result = _PyObject_CallMethod_SizeT(
            handler,
            b"processingInstruction\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            b"ss\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            target,
            data,
        );
        if !result.is_null() {
            let ref mut fresh56 = (*result).ob_refcnt;
            *fresh56 -= 1;
            if !(*fresh56 != 0 as libc::c_int as libc::c_long) {
                (Some(
                    ((*(*result).ob_type).tp_dealloc).expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(result);
            }
        }
    }
}
unsafe extern "C" fn pythonComment(
    mut user_data: *mut libc::c_void,
    mut value: *const xmlChar,
) {
    let mut handler: *mut PyObject = 0 as *mut PyObject;
    let mut result: *mut PyObject = 0 as *mut PyObject;
    handler = user_data as *mut PyObject;
    if PyObject_HasAttrString(
        handler,
        b"comment\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) != 0
    {
        result = _PyObject_CallMethod_SizeT(
            handler,
            b"comment\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value,
        );
        if !(PyErr_Occurred()).is_null() {
            PyErr_Print();
        }
        if !result.is_null() {
            let ref mut fresh57 = (*result).ob_refcnt;
            *fresh57 -= 1;
            if !(*fresh57 != 0 as libc::c_int as libc::c_long) {
                (Some(
                    ((*(*result).ob_type).tp_dealloc).expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(result);
            }
        }
    }
}
unsafe extern "C" fn pythonWarning(
    mut user_data: *mut libc::c_void,
    mut msg: *const libc::c_char,
    mut args: ...
) {
    let mut handler: *mut PyObject = 0 as *mut PyObject;
    let mut result: *mut PyObject = 0 as *mut PyObject;
    let mut args_0: ::std::ffi::VaListImpl;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    handler = user_data as *mut PyObject;
    if PyObject_HasAttrString(
        handler,
        b"warning\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) != 0
    {
        args_0 = args.clone();
        vsnprintf(
            buf.as_mut_ptr(),
            1023 as libc::c_int as libc::c_ulong,
            msg,
            args_0.as_va_list(),
        );
        buf[1023 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        result = _PyObject_CallMethod_SizeT(
            handler,
            b"warning\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            buf.as_mut_ptr(),
        );
        if !(PyErr_Occurred()).is_null() {
            PyErr_Print();
        }
        if !result.is_null() {
            let ref mut fresh58 = (*result).ob_refcnt;
            *fresh58 -= 1;
            if !(*fresh58 != 0 as libc::c_int as libc::c_long) {
                (Some(
                    ((*(*result).ob_type).tp_dealloc).expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(result);
            }
        }
    }
}
unsafe extern "C" fn pythonError(
    mut user_data: *mut libc::c_void,
    mut msg: *const libc::c_char,
    mut args: ...
) {
    let mut handler: *mut PyObject = 0 as *mut PyObject;
    let mut result: *mut PyObject = 0 as *mut PyObject;
    let mut args_0: ::std::ffi::VaListImpl;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    handler = user_data as *mut PyObject;
    if PyObject_HasAttrString(
        handler,
        b"error\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) != 0
    {
        args_0 = args.clone();
        vsnprintf(
            buf.as_mut_ptr(),
            1023 as libc::c_int as libc::c_ulong,
            msg,
            args_0.as_va_list(),
        );
        buf[1023 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        result = _PyObject_CallMethod_SizeT(
            handler,
            b"error\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            buf.as_mut_ptr(),
        );
        if !(PyErr_Occurred()).is_null() {
            PyErr_Print();
        }
        if !result.is_null() {
            let ref mut fresh59 = (*result).ob_refcnt;
            *fresh59 -= 1;
            if !(*fresh59 != 0 as libc::c_int as libc::c_long) {
                (Some(
                    ((*(*result).ob_type).tp_dealloc).expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(result);
            }
        }
    }
}
unsafe extern "C" fn pythonFatalError(
    mut user_data: *mut libc::c_void,
    mut msg: *const libc::c_char,
    mut args: ...
) {
    let mut handler: *mut PyObject = 0 as *mut PyObject;
    let mut result: *mut PyObject = 0 as *mut PyObject;
    let mut args_0: ::std::ffi::VaListImpl;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    handler = user_data as *mut PyObject;
    if PyObject_HasAttrString(
        handler,
        b"fatalError\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) != 0
    {
        args_0 = args.clone();
        vsnprintf(
            buf.as_mut_ptr(),
            1023 as libc::c_int as libc::c_ulong,
            msg,
            args_0.as_va_list(),
        );
        buf[1023 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        result = _PyObject_CallMethod_SizeT(
            handler,
            b"fatalError\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            buf.as_mut_ptr(),
        );
        if !(PyErr_Occurred()).is_null() {
            PyErr_Print();
        }
        if !result.is_null() {
            let ref mut fresh60 = (*result).ob_refcnt;
            *fresh60 -= 1;
            if !(*fresh60 != 0 as libc::c_int as libc::c_long) {
                (Some(
                    ((*(*result).ob_type).tp_dealloc).expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(result);
            }
        }
    }
}
unsafe extern "C" fn pythonCdataBlock(
    mut user_data: *mut libc::c_void,
    mut ch: *const xmlChar,
    mut len: libc::c_int,
) {
    let mut handler: *mut PyObject = 0 as *mut PyObject;
    let mut result: *mut PyObject = 0 as *mut PyObject;
    let mut type_0: libc::c_int = 0 as libc::c_int;
    handler = user_data as *mut PyObject;
    if PyObject_HasAttrString(
        handler,
        b"cdataBlock\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) != 0
    {
        type_0 = 1 as libc::c_int;
    } else if PyObject_HasAttrString(
            handler,
            b"cdata\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) != 0
        {
        type_0 = 2 as libc::c_int;
    }
    if type_0 != 0 as libc::c_int {
        if type_0 == 1 as libc::c_int {
            result = _PyObject_CallMethod_SizeT(
                handler,
                b"cdataBlock\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"s#\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ch,
                len as Py_ssize_t,
            );
        } else if type_0 == 2 as libc::c_int {
            result = _PyObject_CallMethod_SizeT(
                handler,
                b"cdata\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"s#\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ch,
                len as Py_ssize_t,
            );
        }
        if !(PyErr_Occurred()).is_null() {
            PyErr_Print();
        }
        if !result.is_null() {
            let ref mut fresh61 = (*result).ob_refcnt;
            *fresh61 -= 1;
            if !(*fresh61 != 0 as libc::c_int as libc::c_long) {
                (Some(
                    ((*(*result).ob_type).tp_dealloc).expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(result);
            }
        }
    }
}
unsafe extern "C" fn pythonExternalSubset(
    mut user_data: *mut libc::c_void,
    mut name: *const xmlChar,
    mut externalID: *const xmlChar,
    mut systemID: *const xmlChar,
) {
    let mut handler: *mut PyObject = 0 as *mut PyObject;
    let mut result: *mut PyObject = 0 as *mut PyObject;
    handler = user_data as *mut PyObject;
    if PyObject_HasAttrString(
        handler,
        b"externalSubset\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) != 0
    {
        result = _PyObject_CallMethod_SizeT(
            handler,
            b"externalSubset\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"sss\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            name,
            externalID,
            systemID,
        );
        if !result.is_null() {
            let ref mut fresh62 = (*result).ob_refcnt;
            *fresh62 -= 1;
            if !(*fresh62 != 0 as libc::c_int as libc::c_long) {
                (Some(
                    ((*(*result).ob_type).tp_dealloc).expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(result);
            }
        }
    }
}
unsafe extern "C" fn pythonEntityDecl(
    mut user_data: *mut libc::c_void,
    mut name: *const xmlChar,
    mut type_0: libc::c_int,
    mut publicId: *const xmlChar,
    mut systemId: *const xmlChar,
    mut content: *mut xmlChar,
) {
    let mut handler: *mut PyObject = 0 as *mut PyObject;
    let mut result: *mut PyObject = 0 as *mut PyObject;
    handler = user_data as *mut PyObject;
    if PyObject_HasAttrString(
        handler,
        b"entityDecl\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) != 0
    {
        result = _PyObject_CallMethod_SizeT(
            handler,
            b"entityDecl\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"sisss\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            name,
            type_0,
            publicId,
            systemId,
            content,
        );
        if !(PyErr_Occurred()).is_null() {
            PyErr_Print();
        }
        if !result.is_null() {
            let ref mut fresh63 = (*result).ob_refcnt;
            *fresh63 -= 1;
            if !(*fresh63 != 0 as libc::c_int as libc::c_long) {
                (Some(
                    ((*(*result).ob_type).tp_dealloc).expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(result);
            }
        }
    }
}
unsafe extern "C" fn pythonNotationDecl(
    mut user_data: *mut libc::c_void,
    mut name: *const xmlChar,
    mut publicId: *const xmlChar,
    mut systemId: *const xmlChar,
) {
    let mut handler: *mut PyObject = 0 as *mut PyObject;
    let mut result: *mut PyObject = 0 as *mut PyObject;
    handler = user_data as *mut PyObject;
    if PyObject_HasAttrString(
        handler,
        b"notationDecl\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) != 0
    {
        result = _PyObject_CallMethod_SizeT(
            handler,
            b"notationDecl\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"sss\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            name,
            publicId,
            systemId,
        );
        if !(PyErr_Occurred()).is_null() {
            PyErr_Print();
        }
        if !result.is_null() {
            let ref mut fresh64 = (*result).ob_refcnt;
            *fresh64 -= 1;
            if !(*fresh64 != 0 as libc::c_int as libc::c_long) {
                (Some(
                    ((*(*result).ob_type).tp_dealloc).expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(result);
            }
        }
    }
}
unsafe extern "C" fn pythonAttributeDecl(
    mut user_data: *mut libc::c_void,
    mut elem: *const xmlChar,
    mut name: *const xmlChar,
    mut type_0: libc::c_int,
    mut def: libc::c_int,
    mut defaultValue: *const xmlChar,
    mut tree: xmlEnumerationPtr,
) {
    let mut handler: *mut PyObject = 0 as *mut PyObject;
    let mut nameList: *mut PyObject = 0 as *mut PyObject;
    let mut newName: *mut PyObject = 0 as *mut PyObject;
    let mut node: xmlEnumerationPtr = 0 as *mut xmlEnumeration;
    let mut result: *mut PyObject = 0 as *mut PyObject;
    let mut count: libc::c_int = 0;
    handler = user_data as *mut PyObject;
    if PyObject_HasAttrString(
        handler,
        b"attributeDecl\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) != 0
    {
        count = 0 as libc::c_int;
        node = tree;
        while !node.is_null() {
            count += 1;
            node = (*node).next;
        }
        nameList = PyList_New(count as Py_ssize_t);
        count = 0 as libc::c_int;
        node = tree;
        while !node.is_null() {
            newName = PyString_FromString((*node).name as *mut libc::c_char);
            PyList_SetItem(nameList, count as Py_ssize_t, newName);
            let ref mut fresh65 = (*newName).ob_refcnt;
            *fresh65 -= 1;
            if !(*fresh65 != 0 as libc::c_int as libc::c_long) {
                (Some(
                    ((*(*newName).ob_type).tp_dealloc)
                        .expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(newName);
            }
            count += 1;
            node = (*node).next;
        }
        result = _PyObject_CallMethod_SizeT(
            handler,
            b"attributeDecl\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"ssiisO\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            elem,
            name,
            type_0,
            def,
            defaultValue,
            nameList,
        );
        if !(PyErr_Occurred()).is_null() {
            PyErr_Print();
        }
        if !nameList.is_null() {
            let ref mut fresh66 = (*nameList).ob_refcnt;
            *fresh66 -= 1;
            if !(*fresh66 != 0 as libc::c_int as libc::c_long) {
                (Some(
                    ((*(*nameList).ob_type).tp_dealloc)
                        .expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(nameList);
            }
        }
        if !result.is_null() {
            let ref mut fresh67 = (*result).ob_refcnt;
            *fresh67 -= 1;
            if !(*fresh67 != 0 as libc::c_int as libc::c_long) {
                (Some(
                    ((*(*result).ob_type).tp_dealloc).expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(result);
            }
        }
    }
}
unsafe extern "C" fn pythonElementDecl(
    mut user_data: *mut libc::c_void,
    mut name: *const xmlChar,
    mut type_0: libc::c_int,
    mut content: xmlElementContentPtr,
) {
    let mut handler: *mut PyObject = 0 as *mut PyObject;
    let mut obj: *mut PyObject = 0 as *mut PyObject;
    let mut result: *mut PyObject = 0 as *mut PyObject;
    handler = user_data as *mut PyObject;
    if PyObject_HasAttrString(
        handler,
        b"elementDecl\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) != 0
    {
        printf(
            b"pythonElementDecl: xmlElementContentPtr wrapper missing !\n\0" as *const u8
                as *const libc::c_char,
        );
        obj = &mut _Py_NoneStruct;
        result = _PyObject_CallMethod_SizeT(
            handler,
            b"elementDecl\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"siO\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            name,
            type_0,
            obj,
        );
        if !(PyErr_Occurred()).is_null() {
            PyErr_Print();
        }
        if !result.is_null() {
            let ref mut fresh68 = (*result).ob_refcnt;
            *fresh68 -= 1;
            if !(*fresh68 != 0 as libc::c_int as libc::c_long) {
                (Some(
                    ((*(*result).ob_type).tp_dealloc).expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(result);
            }
        }
    }
}
unsafe extern "C" fn pythonUnparsedEntityDecl(
    mut user_data: *mut libc::c_void,
    mut name: *const xmlChar,
    mut publicId: *const xmlChar,
    mut systemId: *const xmlChar,
    mut notationName: *const xmlChar,
) {
    let mut handler: *mut PyObject = 0 as *mut PyObject;
    let mut result: *mut PyObject = 0 as *mut PyObject;
    handler = user_data as *mut PyObject;
    if PyObject_HasAttrString(
        handler,
        b"unparsedEntityDecl\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) != 0
    {
        result = _PyObject_CallMethod_SizeT(
            handler,
            b"unparsedEntityDecl\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            b"ssss\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            name,
            publicId,
            systemId,
            notationName,
        );
        if !(PyErr_Occurred()).is_null() {
            PyErr_Print();
        }
        if !result.is_null() {
            let ref mut fresh69 = (*result).ob_refcnt;
            *fresh69 -= 1;
            if !(*fresh69 != 0 as libc::c_int as libc::c_long) {
                (Some(
                    ((*(*result).ob_type).tp_dealloc).expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(result);
            }
        }
    }
}
unsafe extern "C" fn pythonInternalSubset(
    mut user_data: *mut libc::c_void,
    mut name: *const xmlChar,
    mut ExternalID: *const xmlChar,
    mut SystemID: *const xmlChar,
) {
    let mut handler: *mut PyObject = 0 as *mut PyObject;
    let mut result: *mut PyObject = 0 as *mut PyObject;
    handler = user_data as *mut PyObject;
    if PyObject_HasAttrString(
        handler,
        b"internalSubset\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) != 0
    {
        result = _PyObject_CallMethod_SizeT(
            handler,
            b"internalSubset\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"sss\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            name,
            ExternalID,
            SystemID,
        );
        if !(PyErr_Occurred()).is_null() {
            PyErr_Print();
        }
        if !result.is_null() {
            let ref mut fresh70 = (*result).ob_refcnt;
            *fresh70 -= 1;
            if !(*fresh70 != 0 as libc::c_int as libc::c_long) {
                (Some(
                    ((*(*result).ob_type).tp_dealloc).expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(result);
            }
        }
    }
}
static mut pythonSaxHandler: xmlSAXHandler = unsafe {
    {
        let mut init = _xmlSAXHandler {
            internalSubset: Some(
                pythonInternalSubset
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
            getEntity: None,
            entityDecl: Some(
                pythonEntityDecl
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                        libc::c_int,
                        *const xmlChar,
                        *const xmlChar,
                        *mut xmlChar,
                    ) -> (),
            ),
            notationDecl: Some(
                pythonNotationDecl
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                        *const xmlChar,
                        *const xmlChar,
                    ) -> (),
            ),
            attributeDecl: Some(
                pythonAttributeDecl
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                        *const xmlChar,
                        libc::c_int,
                        libc::c_int,
                        *const xmlChar,
                        xmlEnumerationPtr,
                    ) -> (),
            ),
            elementDecl: Some(
                pythonElementDecl
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                        libc::c_int,
                        xmlElementContentPtr,
                    ) -> (),
            ),
            unparsedEntityDecl: Some(
                pythonUnparsedEntityDecl
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                        *const xmlChar,
                        *const xmlChar,
                        *const xmlChar,
                    ) -> (),
            ),
            setDocumentLocator: None,
            startDocument: Some(
                pythonStartDocument as unsafe extern "C" fn(*mut libc::c_void) -> (),
            ),
            endDocument: Some(
                pythonEndDocument as unsafe extern "C" fn(*mut libc::c_void) -> (),
            ),
            startElement: Some(
                pythonStartElement
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                        *mut *const xmlChar,
                    ) -> (),
            ),
            endElement: Some(
                pythonEndElement
                    as unsafe extern "C" fn(*mut libc::c_void, *const xmlChar) -> (),
            ),
            reference: Some(
                pythonReference
                    as unsafe extern "C" fn(*mut libc::c_void, *const xmlChar) -> (),
            ),
            characters: Some(
                pythonCharacters
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                        libc::c_int,
                    ) -> (),
            ),
            ignorableWhitespace: Some(
                pythonIgnorableWhitespace
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                        libc::c_int,
                    ) -> (),
            ),
            processingInstruction: Some(
                pythonProcessingInstruction
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                        *const xmlChar,
                    ) -> (),
            ),
            comment: Some(
                pythonComment
                    as unsafe extern "C" fn(*mut libc::c_void, *const xmlChar) -> (),
            ),
            warning: Some(
                pythonWarning
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        ...
                    ) -> (),
            ),
            error: Some(
                pythonError
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        ...
                    ) -> (),
            ),
            fatalError: Some(
                pythonFatalError
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        ...
                    ) -> (),
            ),
            getParameterEntity: None,
            cdataBlock: Some(
                pythonCdataBlock
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                        libc::c_int,
                    ) -> (),
            ),
            externalSubset: Some(
                pythonExternalSubset
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const xmlChar,
                        *const xmlChar,
                        *const xmlChar,
                    ) -> (),
            ),
            initialized: 1 as libc::c_int as libc::c_uint,
            _private: 0 as *const libc::c_void as *mut libc::c_void,
            startElementNs: None,
            endElementNs: None,
            serror: None,
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlCreatePushParser(
    mut self_0: *mut PyObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut chunk: *const libc::c_char = 0 as *const libc::c_char;
    let mut size: libc::c_int = 0;
    let mut URI: *const libc::c_char = 0 as *const libc::c_char;
    let mut pyobj_SAX: *mut PyObject = 0 as *mut PyObject;
    let mut SAX: xmlSAXHandlerPtr = 0 as xmlSAXHandlerPtr;
    let mut ret: xmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
    let mut pyret: *mut PyObject = 0 as *mut PyObject;
    if _PyArg_ParseTuple_SizeT(
        args,
        b"Oziz:xmlCreatePushParser\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        &mut pyobj_SAX as *mut *mut PyObject,
        &mut chunk as *mut *const libc::c_char,
        &mut size as *mut libc::c_int,
        &mut URI as *mut *const libc::c_char,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    if pyobj_SAX != &mut _Py_NoneStruct as *mut PyObject {
        SAX = &mut pythonSaxHandler;
        let ref mut fresh71 = (*pyobj_SAX).ob_refcnt;
        *fresh71 += 1;
    }
    ret = xmlCreatePushParserCtxt(SAX, pyobj_SAX as *mut libc::c_void, chunk, size, URI);
    pyret = libxml_xmlParserCtxtPtrWrap(ret);
    return pyret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_htmlCreatePushParser(
    mut self_0: *mut PyObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut chunk: *const libc::c_char = 0 as *const libc::c_char;
    let mut size: libc::c_int = 0;
    let mut URI: *const libc::c_char = 0 as *const libc::c_char;
    let mut pyobj_SAX: *mut PyObject = 0 as *mut PyObject;
    let mut SAX: xmlSAXHandlerPtr = 0 as xmlSAXHandlerPtr;
    let mut ret: xmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
    let mut pyret: *mut PyObject = 0 as *mut PyObject;
    if _PyArg_ParseTuple_SizeT(
        args,
        b"Oziz:htmlCreatePushParser\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        &mut pyobj_SAX as *mut *mut PyObject,
        &mut chunk as *mut *const libc::c_char,
        &mut size as *mut libc::c_int,
        &mut URI as *mut *const libc::c_char,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    if pyobj_SAX != &mut _Py_NoneStruct as *mut PyObject {
        SAX = &mut pythonSaxHandler;
        let ref mut fresh72 = (*pyobj_SAX).ob_refcnt;
        *fresh72 += 1;
    }
    ret = htmlCreatePushParserCtxt(
        SAX,
        pyobj_SAX as *mut libc::c_void,
        chunk,
        size,
        URI,
        XML_CHAR_ENCODING_NONE,
    );
    pyret = libxml_xmlParserCtxtPtrWrap(ret);
    return pyret;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlSAXParseFile(
    mut self_0: *mut PyObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut recover: libc::c_int = 0;
    let mut URI: *const libc::c_char = 0 as *const libc::c_char;
    let mut pyobj_SAX: *mut PyObject = 0 as *mut PyObject;
    let mut SAX: xmlSAXHandlerPtr = 0 as xmlSAXHandlerPtr;
    if _PyArg_ParseTuple_SizeT(
        args,
        b"Osi:xmlSAXParseFile\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        &mut pyobj_SAX as *mut *mut PyObject,
        &mut URI as *mut *const libc::c_char,
        &mut recover as *mut libc::c_int,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    if pyobj_SAX == &mut _Py_NoneStruct as *mut PyObject {
        let ref mut fresh73 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
        *fresh73 += 1;
        return &mut _Py_NoneStruct;
    }
    SAX = &mut pythonSaxHandler;
    let ref mut fresh74 = (*pyobj_SAX).ob_refcnt;
    *fresh74 += 1;
    xmlSAXUserParseFile(SAX, pyobj_SAX as *mut libc::c_void, URI);
    let ref mut fresh75 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
    *fresh75 += 1;
    return &mut _Py_NoneStruct;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_htmlSAXParseFile(
    mut self_0: *mut PyObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut URI: *const libc::c_char = 0 as *const libc::c_char;
    let mut encoding: *const libc::c_char = 0 as *const libc::c_char;
    let mut pyobj_SAX: *mut PyObject = 0 as *mut PyObject;
    let mut SAX: xmlSAXHandlerPtr = 0 as xmlSAXHandlerPtr;
    if _PyArg_ParseTuple_SizeT(
        args,
        b"Osz:htmlSAXParseFile\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        &mut pyobj_SAX as *mut *mut PyObject,
        &mut URI as *mut *const libc::c_char,
        &mut encoding as *mut *const libc::c_char,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    if pyobj_SAX == &mut _Py_NoneStruct as *mut PyObject {
        let ref mut fresh76 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
        *fresh76 += 1;
        return &mut _Py_NoneStruct;
    }
    SAX = &mut pythonSaxHandler;
    let ref mut fresh77 = (*pyobj_SAX).ob_refcnt;
    *fresh77 += 1;
    htmlSAXParseFile(URI, encoding, SAX, pyobj_SAX as *mut libc::c_void);
    let ref mut fresh78 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
    *fresh78 += 1;
    return &mut _Py_NoneStruct;
}
static mut libxml_xmlPythonErrorFuncHandler: *mut PyObject = 0 as *const PyObject
    as *mut PyObject;
static mut libxml_xmlPythonErrorFuncCtxt: *mut PyObject = 0 as *const PyObject
    as *mut PyObject;
unsafe extern "C" fn libxml_buildMessage(
    mut msg: *const libc::c_char,
    mut ap: ::std::ffi::VaList,
) -> *mut libc::c_char {
    let mut chars: libc::c_int = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    str = xmlMalloc.expect("non-null function pointer")(1000 as libc::c_int as size_t)
        as *mut libc::c_char;
    if str.is_null() {
        return 0 as *mut libc::c_char;
    }
    chars = vsnprintf(str, 999 as libc::c_int as libc::c_ulong, msg, ap.as_va_list());
    if chars >= 998 as libc::c_int {
        *str.offset(999 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
    }
    return str;
}
unsafe extern "C" fn libxml_xmlErrorFuncHandler(
    mut ctx: *mut libc::c_void,
    mut msg: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::std::ffi::VaListImpl;
    let mut list: *mut PyObject = 0 as *mut PyObject;
    let mut message: *mut PyObject = 0 as *mut PyObject;
    let mut result: *mut PyObject = 0 as *mut PyObject;
    let mut str: [libc::c_char; 1000] = [0; 1000];
    if libxml_xmlPythonErrorFuncHandler.is_null() {
        ap = args.clone();
        vfprintf(stderr, msg, ap.as_va_list());
    } else {
        ap = args.clone();
        if vsnprintf(
            str.as_mut_ptr(),
            999 as libc::c_int as libc::c_ulong,
            msg,
            ap.as_va_list(),
        ) >= 998 as libc::c_int
        {
            str[999 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        }
        list = PyTuple_New(2 as libc::c_int as Py_ssize_t);
        PyTuple_SetItem(
            list,
            0 as libc::c_int as Py_ssize_t,
            libxml_xmlPythonErrorFuncCtxt,
        );
        if !libxml_xmlPythonErrorFuncCtxt.is_null() {
            let ref mut fresh79 = (*libxml_xmlPythonErrorFuncCtxt).ob_refcnt;
            *fresh79 += 1;
        }
        message = libxml_charPtrConstWrap(str.as_mut_ptr());
        PyTuple_SetItem(list, 1 as libc::c_int as Py_ssize_t, message);
        result = PyEval_CallObjectWithKeywords(
            libxml_xmlPythonErrorFuncHandler,
            list,
            0 as *mut libc::c_void as *mut PyObject,
        );
        if !list.is_null() {
            let ref mut fresh80 = (*list).ob_refcnt;
            *fresh80 -= 1;
            if !(*fresh80 != 0 as libc::c_int as libc::c_long) {
                (Some(
                    ((*(*list).ob_type).tp_dealloc).expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(list);
            }
        }
        if !result.is_null() {
            let ref mut fresh81 = (*result).ob_refcnt;
            *fresh81 -= 1;
            if !(*fresh81 != 0 as libc::c_int as libc::c_long) {
                (Some(
                    ((*(*result).ob_type).tp_dealloc).expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(result);
            }
        }
    };
}
unsafe extern "C" fn libxml_xmlErrorInitialize() {
    xmlSetGenericErrorFunc(
        0 as *mut libc::c_void,
        Some(
            libxml_xmlErrorFuncHandler
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                    ...
                ) -> (),
        ),
    );
    xmlThrDefSetGenericErrorFunc(
        0 as *mut libc::c_void,
        Some(
            libxml_xmlErrorFuncHandler
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                    ...
                ) -> (),
        ),
    );
}
unsafe extern "C" fn libxml_xmlRegisterErrorHandler(
    mut self_0: *mut PyObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut py_retval: *mut PyObject = 0 as *mut PyObject;
    let mut pyobj_f: *mut PyObject = 0 as *mut PyObject;
    let mut pyobj_ctx: *mut PyObject = 0 as *mut PyObject;
    if _PyArg_ParseTuple_SizeT(
        args,
        b"OO:xmlRegisterErrorHandler\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        &mut pyobj_f as *mut *mut PyObject,
        &mut pyobj_ctx as *mut *mut PyObject,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    if !libxml_xmlPythonErrorFuncHandler.is_null() {
        if !libxml_xmlPythonErrorFuncHandler.is_null() {
            let ref mut fresh82 = (*libxml_xmlPythonErrorFuncHandler).ob_refcnt;
            *fresh82 -= 1;
            if !(*fresh82 != 0 as libc::c_int as libc::c_long) {
                (Some(
                    ((*(*libxml_xmlPythonErrorFuncHandler).ob_type).tp_dealloc)
                        .expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(libxml_xmlPythonErrorFuncHandler);
            }
        }
    }
    if !libxml_xmlPythonErrorFuncCtxt.is_null() {
        if !libxml_xmlPythonErrorFuncCtxt.is_null() {
            let ref mut fresh83 = (*libxml_xmlPythonErrorFuncCtxt).ob_refcnt;
            *fresh83 -= 1;
            if !(*fresh83 != 0 as libc::c_int as libc::c_long) {
                (Some(
                    ((*(*libxml_xmlPythonErrorFuncCtxt).ob_type).tp_dealloc)
                        .expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(libxml_xmlPythonErrorFuncCtxt);
            }
        }
    }
    if !pyobj_ctx.is_null() {
        let ref mut fresh84 = (*pyobj_ctx).ob_refcnt;
        *fresh84 += 1;
    }
    if !pyobj_f.is_null() {
        let ref mut fresh85 = (*pyobj_f).ob_refcnt;
        *fresh85 += 1;
    }
    libxml_xmlPythonErrorFuncHandler = pyobj_f;
    libxml_xmlPythonErrorFuncCtxt = pyobj_ctx;
    py_retval = libxml_intWrap(1 as libc::c_int);
    return py_retval;
}
unsafe extern "C" fn libxml_xmlParserCtxtGenericErrorFuncHandler(
    mut ctx: *mut libc::c_void,
    mut severity: libc::c_int,
    mut str: *mut libc::c_char,
) {
    let mut list: *mut PyObject = 0 as *mut PyObject;
    let mut result: *mut PyObject = 0 as *mut PyObject;
    let mut ctxt: xmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
    let mut pyCtxt: xmlParserCtxtPyCtxtPtr = 0 as *mut xmlParserCtxtPyCtxt;
    ctxt = ctx as xmlParserCtxtPtr;
    pyCtxt = (*ctxt)._private as xmlParserCtxtPyCtxtPtr;
    list = PyTuple_New(4 as libc::c_int as Py_ssize_t);
    PyTuple_SetItem(list, 0 as libc::c_int as Py_ssize_t, (*pyCtxt).arg);
    if !((*pyCtxt).arg).is_null() {
        let ref mut fresh86 = (*(*pyCtxt).arg).ob_refcnt;
        *fresh86 += 1;
    }
    PyTuple_SetItem(list, 1 as libc::c_int as Py_ssize_t, libxml_charPtrWrap(str));
    PyTuple_SetItem(list, 2 as libc::c_int as Py_ssize_t, libxml_intWrap(severity));
    PyTuple_SetItem(list, 3 as libc::c_int as Py_ssize_t, &mut _Py_NoneStruct);
    let ref mut fresh87 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
    *fresh87 += 1;
    result = PyEval_CallObjectWithKeywords(
        (*pyCtxt).f,
        list,
        0 as *mut libc::c_void as *mut PyObject,
    );
    if result.is_null() {
        PyErr_Print();
    }
    if !list.is_null() {
        let ref mut fresh88 = (*list).ob_refcnt;
        *fresh88 -= 1;
        if !(*fresh88 != 0 as libc::c_int as libc::c_long) {
            (Some(((*(*list).ob_type).tp_dealloc).expect("non-null function pointer")))
                .expect("non-null function pointer")(list);
        }
    }
    if !result.is_null() {
        let ref mut fresh89 = (*result).ob_refcnt;
        *fresh89 -= 1;
        if !(*fresh89 != 0 as libc::c_int as libc::c_long) {
            (Some(((*(*result).ob_type).tp_dealloc).expect("non-null function pointer")))
                .expect("non-null function pointer")(result);
        }
    }
}
unsafe extern "C" fn libxml_xmlParserCtxtErrorFuncHandler(
    mut ctx: *mut libc::c_void,
    mut msg: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    libxml_xmlParserCtxtGenericErrorFuncHandler(
        ctx,
        XML_PARSER_SEVERITY_ERROR as libc::c_int,
        libxml_buildMessage(msg, ap.as_va_list()),
    );
}
unsafe extern "C" fn libxml_xmlParserCtxtWarningFuncHandler(
    mut ctx: *mut libc::c_void,
    mut msg: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    libxml_xmlParserCtxtGenericErrorFuncHandler(
        ctx,
        XML_PARSER_SEVERITY_WARNING as libc::c_int,
        libxml_buildMessage(msg, ap.as_va_list()),
    );
}
unsafe extern "C" fn libxml_xmlParserCtxtValidityErrorFuncHandler(
    mut ctx: *mut libc::c_void,
    mut msg: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    libxml_xmlParserCtxtGenericErrorFuncHandler(
        ctx,
        XML_PARSER_SEVERITY_VALIDITY_ERROR as libc::c_int,
        libxml_buildMessage(msg, ap.as_va_list()),
    );
}
unsafe extern "C" fn libxml_xmlParserCtxtValidityWarningFuncHandler(
    mut ctx: *mut libc::c_void,
    mut msg: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    libxml_xmlParserCtxtGenericErrorFuncHandler(
        ctx,
        XML_PARSER_SEVERITY_VALIDITY_WARNING as libc::c_int,
        libxml_buildMessage(msg, ap.as_va_list()),
    );
}
unsafe extern "C" fn libxml_xmlParserCtxtSetErrorHandler(
    mut self_0: *mut PyObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut py_retval: *mut PyObject = 0 as *mut PyObject;
    let mut ctxt: xmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
    let mut pyCtxt: xmlParserCtxtPyCtxtPtr = 0 as *mut xmlParserCtxtPyCtxt;
    let mut pyobj_ctxt: *mut PyObject = 0 as *mut PyObject;
    let mut pyobj_f: *mut PyObject = 0 as *mut PyObject;
    let mut pyobj_arg: *mut PyObject = 0 as *mut PyObject;
    if _PyArg_ParseTuple_SizeT(
        args,
        b"OOO:xmlParserCtxtSetErrorHandler\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        &mut pyobj_ctxt as *mut *mut PyObject,
        &mut pyobj_f as *mut *mut PyObject,
        &mut pyobj_arg as *mut *mut PyObject,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    ctxt = if pyobj_ctxt == &mut _Py_NoneStruct as *mut PyObject {
        0 as xmlParserCtxtPtr
    } else {
        (*(pyobj_ctxt as *mut PyparserCtxt_Object)).obj
    };
    if ((*ctxt)._private).is_null() {
        pyCtxt = xmlMalloc
            .expect(
                "non-null function pointer",
            )(::std::mem::size_of::<xmlParserCtxtPyCtxt>() as libc::c_ulong)
            as xmlParserCtxtPyCtxtPtr;
        if pyCtxt.is_null() {
            py_retval = libxml_intWrap(-(1 as libc::c_int));
            return py_retval;
        }
        memset(
            pyCtxt as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<xmlParserCtxtPyCtxt>() as libc::c_ulong,
        );
        let ref mut fresh90 = (*ctxt)._private;
        *fresh90 = pyCtxt as *mut libc::c_void;
    } else {
        pyCtxt = (*ctxt)._private as xmlParserCtxtPyCtxtPtr;
    }
    if !((*pyCtxt).f).is_null() {
        let ref mut fresh91 = (*(*pyCtxt).f).ob_refcnt;
        *fresh91 -= 1;
        if !(*fresh91 != 0 as libc::c_int as libc::c_long) {
            (Some(
                ((*(*(*pyCtxt).f).ob_type).tp_dealloc)
                    .expect("non-null function pointer"),
            ))
                .expect("non-null function pointer")((*pyCtxt).f);
        }
    }
    if !pyobj_f.is_null() {
        let ref mut fresh92 = (*pyobj_f).ob_refcnt;
        *fresh92 += 1;
    }
    let ref mut fresh93 = (*pyCtxt).f;
    *fresh93 = pyobj_f;
    if !((*pyCtxt).arg).is_null() {
        let ref mut fresh94 = (*(*pyCtxt).arg).ob_refcnt;
        *fresh94 -= 1;
        if !(*fresh94 != 0 as libc::c_int as libc::c_long) {
            (Some(
                ((*(*(*pyCtxt).arg).ob_type).tp_dealloc)
                    .expect("non-null function pointer"),
            ))
                .expect("non-null function pointer")((*pyCtxt).arg);
        }
    }
    if !pyobj_arg.is_null() {
        let ref mut fresh95 = (*pyobj_arg).ob_refcnt;
        *fresh95 += 1;
    }
    let ref mut fresh96 = (*pyCtxt).arg;
    *fresh96 = pyobj_arg;
    if pyobj_f != &mut _Py_NoneStruct as *mut PyObject {
        let ref mut fresh97 = (*(*ctxt).sax).error;
        *fresh97 = Some(
            libxml_xmlParserCtxtErrorFuncHandler
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                    ...
                ) -> (),
        );
        let ref mut fresh98 = (*(*ctxt).sax).warning;
        *fresh98 = Some(
            libxml_xmlParserCtxtWarningFuncHandler
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                    ...
                ) -> (),
        );
        let ref mut fresh99 = (*ctxt).vctxt.error;
        *fresh99 = Some(
            libxml_xmlParserCtxtValidityErrorFuncHandler
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                    ...
                ) -> (),
        );
        let ref mut fresh100 = (*ctxt).vctxt.warning;
        *fresh100 = Some(
            libxml_xmlParserCtxtValidityWarningFuncHandler
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                    ...
                ) -> (),
        );
    } else {
        let ref mut fresh101 = (*(*ctxt).sax).error;
        *fresh101 = Some(
            xmlParserError
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                    ...
                ) -> (),
        );
        let ref mut fresh102 = (*ctxt).vctxt.error;
        *fresh102 = Some(
            xmlParserValidityError
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                    ...
                ) -> (),
        );
        let ref mut fresh103 = (*(*ctxt).sax).warning;
        *fresh103 = Some(
            xmlParserWarning
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                    ...
                ) -> (),
        );
        let ref mut fresh104 = (*ctxt).vctxt.warning;
        *fresh104 = Some(
            xmlParserValidityWarning
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                    ...
                ) -> (),
        );
    }
    py_retval = libxml_intWrap(1 as libc::c_int);
    return py_retval;
}
unsafe extern "C" fn libxml_xmlParserCtxtGetErrorHandler(
    mut self_0: *mut PyObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut py_retval: *mut PyObject = 0 as *mut PyObject;
    let mut ctxt: xmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
    let mut pyCtxt: xmlParserCtxtPyCtxtPtr = 0 as *mut xmlParserCtxtPyCtxt;
    let mut pyobj_ctxt: *mut PyObject = 0 as *mut PyObject;
    if _PyArg_ParseTuple_SizeT(
        args,
        b"O:xmlParserCtxtGetErrorHandler\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        &mut pyobj_ctxt as *mut *mut PyObject,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    ctxt = if pyobj_ctxt == &mut _Py_NoneStruct as *mut PyObject {
        0 as xmlParserCtxtPtr
    } else {
        (*(pyobj_ctxt as *mut PyparserCtxt_Object)).obj
    };
    py_retval = PyTuple_New(2 as libc::c_int as Py_ssize_t);
    if !((*ctxt)._private).is_null() {
        pyCtxt = (*ctxt)._private as xmlParserCtxtPyCtxtPtr;
        PyTuple_SetItem(py_retval, 0 as libc::c_int as Py_ssize_t, (*pyCtxt).f);
        if !((*pyCtxt).f).is_null() {
            let ref mut fresh105 = (*(*pyCtxt).f).ob_refcnt;
            *fresh105 += 1;
        }
        PyTuple_SetItem(py_retval, 1 as libc::c_int as Py_ssize_t, (*pyCtxt).arg);
        if !((*pyCtxt).arg).is_null() {
            let ref mut fresh106 = (*(*pyCtxt).arg).ob_refcnt;
            *fresh106 += 1;
        }
    } else {
        PyTuple_SetItem(py_retval, 0 as libc::c_int as Py_ssize_t, &mut _Py_NoneStruct);
        if !(&mut _Py_NoneStruct as *mut PyObject).is_null() {
            let ref mut fresh107 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
            *fresh107 += 1;
        }
        PyTuple_SetItem(py_retval, 1 as libc::c_int as Py_ssize_t, &mut _Py_NoneStruct);
        if !(&mut _Py_NoneStruct as *mut PyObject).is_null() {
            let ref mut fresh108 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
            *fresh108 += 1;
        }
    }
    return py_retval;
}
unsafe extern "C" fn libxml_xmlFreeParserCtxt(
    mut self_0: *mut PyObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut ctxt: xmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
    let mut pyobj_ctxt: *mut PyObject = 0 as *mut PyObject;
    let mut pyCtxt: xmlParserCtxtPyCtxtPtr = 0 as *mut xmlParserCtxtPyCtxt;
    if _PyArg_ParseTuple_SizeT(
        args,
        b"O:xmlFreeParserCtxt\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        &mut pyobj_ctxt as *mut *mut PyObject,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    ctxt = if pyobj_ctxt == &mut _Py_NoneStruct as *mut PyObject {
        0 as xmlParserCtxtPtr
    } else {
        (*(pyobj_ctxt as *mut PyparserCtxt_Object)).obj
    };
    if !ctxt.is_null() {
        pyCtxt = (*ctxt)._private as xmlParserCtxtPyCtxtPtr;
        if !pyCtxt.is_null() {
            if !((*pyCtxt).f).is_null() {
                let ref mut fresh109 = (*(*pyCtxt).f).ob_refcnt;
                *fresh109 -= 1;
                if !(*fresh109 != 0 as libc::c_int as libc::c_long) {
                    (Some(
                        ((*(*(*pyCtxt).f).ob_type).tp_dealloc)
                            .expect("non-null function pointer"),
                    ))
                        .expect("non-null function pointer")((*pyCtxt).f);
                }
            }
            if !((*pyCtxt).arg).is_null() {
                let ref mut fresh110 = (*(*pyCtxt).arg).ob_refcnt;
                *fresh110 -= 1;
                if !(*fresh110 != 0 as libc::c_int as libc::c_long) {
                    (Some(
                        ((*(*(*pyCtxt).arg).ob_type).tp_dealloc)
                            .expect("non-null function pointer"),
                    ))
                        .expect("non-null function pointer")((*pyCtxt).arg);
                }
            }
            xmlFree.expect("non-null function pointer")(pyCtxt as *mut libc::c_void);
        }
        xmlFreeParserCtxt(ctxt);
    }
    let ref mut fresh111 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
    *fresh111 += 1;
    return &mut _Py_NoneStruct;
}
unsafe extern "C" fn libxml_xmlValidCtxtGenericErrorFuncHandler(
    mut ctx: *mut libc::c_void,
    mut severity: libc::c_int,
    mut str: *mut libc::c_char,
) {
    let mut list: *mut PyObject = 0 as *mut PyObject;
    let mut result: *mut PyObject = 0 as *mut PyObject;
    let mut pyCtxt: xmlValidCtxtPyCtxtPtr = 0 as *mut xmlValidCtxtPyCtxt;
    pyCtxt = ctx as xmlValidCtxtPyCtxtPtr;
    list = PyTuple_New(2 as libc::c_int as Py_ssize_t);
    PyTuple_SetItem(list, 0 as libc::c_int as Py_ssize_t, libxml_charPtrWrap(str));
    PyTuple_SetItem(list, 1 as libc::c_int as Py_ssize_t, (*pyCtxt).arg);
    if !((*pyCtxt).arg).is_null() {
        let ref mut fresh112 = (*(*pyCtxt).arg).ob_refcnt;
        *fresh112 += 1;
    }
    result = PyEval_CallObjectWithKeywords(
        (*pyCtxt).error,
        list,
        0 as *mut libc::c_void as *mut PyObject,
    );
    if result.is_null() {
        PyErr_Print();
    }
    if !list.is_null() {
        let ref mut fresh113 = (*list).ob_refcnt;
        *fresh113 -= 1;
        if !(*fresh113 != 0 as libc::c_int as libc::c_long) {
            (Some(((*(*list).ob_type).tp_dealloc).expect("non-null function pointer")))
                .expect("non-null function pointer")(list);
        }
    }
    if !result.is_null() {
        let ref mut fresh114 = (*result).ob_refcnt;
        *fresh114 -= 1;
        if !(*fresh114 != 0 as libc::c_int as libc::c_long) {
            (Some(((*(*result).ob_type).tp_dealloc).expect("non-null function pointer")))
                .expect("non-null function pointer")(result);
        }
    }
}
unsafe extern "C" fn libxml_xmlValidCtxtGenericWarningFuncHandler(
    mut ctx: *mut libc::c_void,
    mut severity: libc::c_int,
    mut str: *mut libc::c_char,
) {
    let mut list: *mut PyObject = 0 as *mut PyObject;
    let mut result: *mut PyObject = 0 as *mut PyObject;
    let mut pyCtxt: xmlValidCtxtPyCtxtPtr = 0 as *mut xmlValidCtxtPyCtxt;
    pyCtxt = ctx as xmlValidCtxtPyCtxtPtr;
    list = PyTuple_New(2 as libc::c_int as Py_ssize_t);
    PyTuple_SetItem(list, 0 as libc::c_int as Py_ssize_t, libxml_charPtrWrap(str));
    PyTuple_SetItem(list, 1 as libc::c_int as Py_ssize_t, (*pyCtxt).arg);
    if !((*pyCtxt).arg).is_null() {
        let ref mut fresh115 = (*(*pyCtxt).arg).ob_refcnt;
        *fresh115 += 1;
    }
    result = PyEval_CallObjectWithKeywords(
        (*pyCtxt).warn,
        list,
        0 as *mut libc::c_void as *mut PyObject,
    );
    if result.is_null() {
        PyErr_Print();
    }
    if !list.is_null() {
        let ref mut fresh116 = (*list).ob_refcnt;
        *fresh116 -= 1;
        if !(*fresh116 != 0 as libc::c_int as libc::c_long) {
            (Some(((*(*list).ob_type).tp_dealloc).expect("non-null function pointer")))
                .expect("non-null function pointer")(list);
        }
    }
    if !result.is_null() {
        let ref mut fresh117 = (*result).ob_refcnt;
        *fresh117 -= 1;
        if !(*fresh117 != 0 as libc::c_int as libc::c_long) {
            (Some(((*(*result).ob_type).tp_dealloc).expect("non-null function pointer")))
                .expect("non-null function pointer")(result);
        }
    }
}
unsafe extern "C" fn libxml_xmlValidCtxtErrorFuncHandler(
    mut ctx: *mut libc::c_void,
    mut msg: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    libxml_xmlValidCtxtGenericErrorFuncHandler(
        ctx,
        XML_PARSER_SEVERITY_VALIDITY_ERROR as libc::c_int,
        libxml_buildMessage(msg, ap.as_va_list()),
    );
}
unsafe extern "C" fn libxml_xmlValidCtxtWarningFuncHandler(
    mut ctx: *mut libc::c_void,
    mut msg: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    libxml_xmlValidCtxtGenericWarningFuncHandler(
        ctx,
        XML_PARSER_SEVERITY_VALIDITY_WARNING as libc::c_int,
        libxml_buildMessage(msg, ap.as_va_list()),
    );
}
unsafe extern "C" fn libxml_xmlSetValidErrors(
    mut self_0: *mut PyObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut py_retval: *mut PyObject = 0 as *mut PyObject;
    let mut pyobj_error: *mut PyObject = 0 as *mut PyObject;
    let mut pyobj_warn: *mut PyObject = 0 as *mut PyObject;
    let mut pyobj_ctx: *mut PyObject = 0 as *mut PyObject;
    let mut pyobj_arg: *mut PyObject = &mut _Py_NoneStruct;
    let mut ctxt: xmlValidCtxtPtr = 0 as *mut xmlValidCtxt;
    let mut pyCtxt: xmlValidCtxtPyCtxtPtr = 0 as *mut xmlValidCtxtPyCtxt;
    if _PyArg_ParseTuple_SizeT(
        args,
        b"OOO|O:xmlSetValidErrors\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        &mut pyobj_ctx as *mut *mut PyObject,
        &mut pyobj_error as *mut *mut PyObject,
        &mut pyobj_warn as *mut *mut PyObject,
        &mut pyobj_arg as *mut *mut PyObject,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    ctxt = if pyobj_ctx == &mut _Py_NoneStruct as *mut PyObject {
        0 as xmlValidCtxtPtr
    } else {
        (*(pyobj_ctx as *mut PyValidCtxt_Object)).obj
    };
    pyCtxt = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::std::mem::size_of::<xmlValidCtxtPyCtxt>() as libc::c_ulong)
        as xmlValidCtxtPyCtxtPtr;
    if pyCtxt.is_null() {
        py_retval = libxml_intWrap(-(1 as libc::c_int));
        return py_retval;
    }
    memset(
        pyCtxt as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<xmlValidCtxtPyCtxt>() as libc::c_ulong,
    );
    if !((*pyCtxt).error).is_null() {
        let ref mut fresh118 = (*(*pyCtxt).error).ob_refcnt;
        *fresh118 -= 1;
        if !(*fresh118 != 0 as libc::c_int as libc::c_long) {
            (Some(
                ((*(*(*pyCtxt).error).ob_type).tp_dealloc)
                    .expect("non-null function pointer"),
            ))
                .expect("non-null function pointer")((*pyCtxt).error);
        }
    }
    if !pyobj_error.is_null() {
        let ref mut fresh119 = (*pyobj_error).ob_refcnt;
        *fresh119 += 1;
    }
    let ref mut fresh120 = (*pyCtxt).error;
    *fresh120 = pyobj_error;
    if !((*pyCtxt).warn).is_null() {
        let ref mut fresh121 = (*(*pyCtxt).warn).ob_refcnt;
        *fresh121 -= 1;
        if !(*fresh121 != 0 as libc::c_int as libc::c_long) {
            (Some(
                ((*(*(*pyCtxt).warn).ob_type).tp_dealloc)
                    .expect("non-null function pointer"),
            ))
                .expect("non-null function pointer")((*pyCtxt).warn);
        }
    }
    if !pyobj_warn.is_null() {
        let ref mut fresh122 = (*pyobj_warn).ob_refcnt;
        *fresh122 += 1;
    }
    let ref mut fresh123 = (*pyCtxt).warn;
    *fresh123 = pyobj_warn;
    if !((*pyCtxt).arg).is_null() {
        let ref mut fresh124 = (*(*pyCtxt).arg).ob_refcnt;
        *fresh124 -= 1;
        if !(*fresh124 != 0 as libc::c_int as libc::c_long) {
            (Some(
                ((*(*(*pyCtxt).arg).ob_type).tp_dealloc)
                    .expect("non-null function pointer"),
            ))
                .expect("non-null function pointer")((*pyCtxt).arg);
        }
    }
    if !pyobj_arg.is_null() {
        let ref mut fresh125 = (*pyobj_arg).ob_refcnt;
        *fresh125 += 1;
    }
    let ref mut fresh126 = (*pyCtxt).arg;
    *fresh126 = pyobj_arg;
    let ref mut fresh127 = (*ctxt).error;
    *fresh127 = Some(
        libxml_xmlValidCtxtErrorFuncHandler
            as unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, ...) -> (),
    );
    let ref mut fresh128 = (*ctxt).warning;
    *fresh128 = Some(
        libxml_xmlValidCtxtWarningFuncHandler
            as unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, ...) -> (),
    );
    let ref mut fresh129 = (*ctxt).userData;
    *fresh129 = pyCtxt as *mut libc::c_void;
    py_retval = libxml_intWrap(1 as libc::c_int);
    return py_retval;
}
unsafe extern "C" fn libxml_xmlFreeValidCtxt(
    mut self_0: *mut PyObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut cur: xmlValidCtxtPtr = 0 as *mut xmlValidCtxt;
    let mut pyCtxt: xmlValidCtxtPyCtxtPtr = 0 as *mut xmlValidCtxtPyCtxt;
    let mut pyobj_cur: *mut PyObject = 0 as *mut PyObject;
    if _PyArg_ParseTuple_SizeT(
        args,
        b"O:xmlFreeValidCtxt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut pyobj_cur as *mut *mut PyObject,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    cur = if pyobj_cur == &mut _Py_NoneStruct as *mut PyObject {
        0 as xmlValidCtxtPtr
    } else {
        (*(pyobj_cur as *mut PyValidCtxt_Object)).obj
    };
    pyCtxt = (*cur).userData as xmlValidCtxtPyCtxtPtr;
    if !pyCtxt.is_null() {
        if !((*pyCtxt).error).is_null() {
            let ref mut fresh130 = (*(*pyCtxt).error).ob_refcnt;
            *fresh130 -= 1;
            if !(*fresh130 != 0 as libc::c_int as libc::c_long) {
                (Some(
                    ((*(*(*pyCtxt).error).ob_type).tp_dealloc)
                        .expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")((*pyCtxt).error);
            }
        }
        if !((*pyCtxt).warn).is_null() {
            let ref mut fresh131 = (*(*pyCtxt).warn).ob_refcnt;
            *fresh131 -= 1;
            if !(*fresh131 != 0 as libc::c_int as libc::c_long) {
                (Some(
                    ((*(*(*pyCtxt).warn).ob_type).tp_dealloc)
                        .expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")((*pyCtxt).warn);
            }
        }
        if !((*pyCtxt).arg).is_null() {
            let ref mut fresh132 = (*(*pyCtxt).arg).ob_refcnt;
            *fresh132 -= 1;
            if !(*fresh132 != 0 as libc::c_int as libc::c_long) {
                (Some(
                    ((*(*(*pyCtxt).arg).ob_type).tp_dealloc)
                        .expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")((*pyCtxt).arg);
            }
        }
        xmlFree.expect("non-null function pointer")(pyCtxt as *mut libc::c_void);
    }
    xmlFreeValidCtxt(cur);
    let ref mut fresh133 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
    *fresh133 += 1;
    return &mut _Py_NoneStruct;
}
unsafe extern "C" fn libxml_xmlTextReaderErrorCallback(
    mut arg: *mut libc::c_void,
    mut msg: *const libc::c_char,
    mut severity: libc::c_int,
    mut locator: xmlTextReaderLocatorPtr,
) {
    let mut pyCtxt: *mut xmlTextReaderPyCtxt = arg as *mut xmlTextReaderPyCtxt;
    let mut list: *mut PyObject = 0 as *mut PyObject;
    let mut result: *mut PyObject = 0 as *mut PyObject;
    list = PyTuple_New(4 as libc::c_int as Py_ssize_t);
    PyTuple_SetItem(list, 0 as libc::c_int as Py_ssize_t, (*pyCtxt).arg);
    if !((*pyCtxt).arg).is_null() {
        let ref mut fresh134 = (*(*pyCtxt).arg).ob_refcnt;
        *fresh134 += 1;
    }
    PyTuple_SetItem(list, 1 as libc::c_int as Py_ssize_t, libxml_charPtrConstWrap(msg));
    PyTuple_SetItem(list, 2 as libc::c_int as Py_ssize_t, libxml_intWrap(severity));
    PyTuple_SetItem(
        list,
        3 as libc::c_int as Py_ssize_t,
        libxml_xmlTextReaderLocatorPtrWrap(locator),
    );
    result = PyEval_CallObjectWithKeywords(
        (*pyCtxt).f,
        list,
        0 as *mut libc::c_void as *mut PyObject,
    );
    if result.is_null() {
        PyErr_Print();
    }
    if !list.is_null() {
        let ref mut fresh135 = (*list).ob_refcnt;
        *fresh135 -= 1;
        if !(*fresh135 != 0 as libc::c_int as libc::c_long) {
            (Some(((*(*list).ob_type).tp_dealloc).expect("non-null function pointer")))
                .expect("non-null function pointer")(list);
        }
    }
    if !result.is_null() {
        let ref mut fresh136 = (*result).ob_refcnt;
        *fresh136 -= 1;
        if !(*fresh136 != 0 as libc::c_int as libc::c_long) {
            (Some(((*(*result).ob_type).tp_dealloc).expect("non-null function pointer")))
                .expect("non-null function pointer")(result);
        }
    }
}
unsafe extern "C" fn libxml_xmlTextReaderSetErrorHandler(
    mut self_0: *mut PyObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut reader: xmlTextReaderPtr = 0 as *mut xmlTextReader;
    let mut pyCtxt: xmlTextReaderPyCtxtPtr = 0 as *mut xmlTextReaderPyCtxt;
    let mut f: xmlTextReaderErrorFunc = None;
    let mut arg: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut pyobj_reader: *mut PyObject = 0 as *mut PyObject;
    let mut pyobj_f: *mut PyObject = 0 as *mut PyObject;
    let mut pyobj_arg: *mut PyObject = 0 as *mut PyObject;
    let mut py_retval: *mut PyObject = 0 as *mut PyObject;
    if _PyArg_ParseTuple_SizeT(
        args,
        b"OOO:xmlTextReaderSetErrorHandler\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        &mut pyobj_reader as *mut *mut PyObject,
        &mut pyobj_f as *mut *mut PyObject,
        &mut pyobj_arg as *mut *mut PyObject,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    reader = if pyobj_reader == &mut _Py_NoneStruct as *mut PyObject {
        0 as xmlTextReaderPtr
    } else {
        (*(pyobj_reader as *mut PyxmlTextReader_Object)).obj
    };
    xmlTextReaderGetErrorHandler(reader, &mut f, &mut arg);
    if !arg.is_null() {
        if f
            == ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        libc::c_int,
                        xmlTextReaderLocatorPtr,
                    ) -> (),
                >,
                xmlTextReaderErrorFunc,
            >(
                Some(
                    libxml_xmlTextReaderErrorCallback
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *const libc::c_char,
                            libc::c_int,
                            xmlTextReaderLocatorPtr,
                        ) -> (),
                ),
            )
        {
            pyCtxt = arg as xmlTextReaderPyCtxtPtr;
            if !((*pyCtxt).f).is_null() {
                let ref mut fresh137 = (*(*pyCtxt).f).ob_refcnt;
                *fresh137 -= 1;
                if !(*fresh137 != 0 as libc::c_int as libc::c_long) {
                    (Some(
                        ((*(*(*pyCtxt).f).ob_type).tp_dealloc)
                            .expect("non-null function pointer"),
                    ))
                        .expect("non-null function pointer")((*pyCtxt).f);
                }
            }
            if !((*pyCtxt).arg).is_null() {
                let ref mut fresh138 = (*(*pyCtxt).arg).ob_refcnt;
                *fresh138 -= 1;
                if !(*fresh138 != 0 as libc::c_int as libc::c_long) {
                    (Some(
                        ((*(*(*pyCtxt).arg).ob_type).tp_dealloc)
                            .expect("non-null function pointer"),
                    ))
                        .expect("non-null function pointer")((*pyCtxt).arg);
                }
            }
            xmlFree.expect("non-null function pointer")(pyCtxt as *mut libc::c_void);
        } else {
            py_retval = libxml_intWrap(-(1 as libc::c_int));
            return py_retval;
        }
    }
    xmlTextReaderSetErrorHandler(reader, None, 0 as *mut libc::c_void);
    if pyobj_f != &mut _Py_NoneStruct as *mut PyObject {
        pyCtxt = xmlMalloc
            .expect(
                "non-null function pointer",
            )(::std::mem::size_of::<xmlTextReaderPyCtxt>() as libc::c_ulong)
            as xmlTextReaderPyCtxtPtr;
        if pyCtxt.is_null() {
            py_retval = libxml_intWrap(-(1 as libc::c_int));
            return py_retval;
        }
        if !pyobj_f.is_null() {
            let ref mut fresh139 = (*pyobj_f).ob_refcnt;
            *fresh139 += 1;
        }
        let ref mut fresh140 = (*pyCtxt).f;
        *fresh140 = pyobj_f;
        if !pyobj_arg.is_null() {
            let ref mut fresh141 = (*pyobj_arg).ob_refcnt;
            *fresh141 += 1;
        }
        let ref mut fresh142 = (*pyCtxt).arg;
        *fresh142 = pyobj_arg;
        xmlTextReaderSetErrorHandler(
            reader,
            ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        libc::c_int,
                        xmlTextReaderLocatorPtr,
                    ) -> (),
                >,
                xmlTextReaderErrorFunc,
            >(
                Some(
                    libxml_xmlTextReaderErrorCallback
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *const libc::c_char,
                            libc::c_int,
                            xmlTextReaderLocatorPtr,
                        ) -> (),
                ),
            ),
            pyCtxt as *mut libc::c_void,
        );
    }
    py_retval = libxml_intWrap(1 as libc::c_int);
    return py_retval;
}
unsafe extern "C" fn libxml_xmlTextReaderGetErrorHandler(
    mut self_0: *mut PyObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut reader: xmlTextReaderPtr = 0 as *mut xmlTextReader;
    let mut pyCtxt: xmlTextReaderPyCtxtPtr = 0 as *mut xmlTextReaderPyCtxt;
    let mut f: xmlTextReaderErrorFunc = None;
    let mut arg: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut pyobj_reader: *mut PyObject = 0 as *mut PyObject;
    let mut py_retval: *mut PyObject = 0 as *mut PyObject;
    if _PyArg_ParseTuple_SizeT(
        args,
        b"O:xmlTextReaderSetErrorHandler\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        &mut pyobj_reader as *mut *mut PyObject,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    reader = if pyobj_reader == &mut _Py_NoneStruct as *mut PyObject {
        0 as xmlTextReaderPtr
    } else {
        (*(pyobj_reader as *mut PyxmlTextReader_Object)).obj
    };
    xmlTextReaderGetErrorHandler(reader, &mut f, &mut arg);
    py_retval = PyTuple_New(2 as libc::c_int as Py_ssize_t);
    if f
        == ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                    libc::c_int,
                    xmlTextReaderLocatorPtr,
                ) -> (),
            >,
            xmlTextReaderErrorFunc,
        >(
            Some(
                libxml_xmlTextReaderErrorCallback
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        libc::c_int,
                        xmlTextReaderLocatorPtr,
                    ) -> (),
            ),
        )
    {
        pyCtxt = arg as xmlTextReaderPyCtxtPtr;
        PyTuple_SetItem(py_retval, 0 as libc::c_int as Py_ssize_t, (*pyCtxt).f);
        if !((*pyCtxt).f).is_null() {
            let ref mut fresh143 = (*(*pyCtxt).f).ob_refcnt;
            *fresh143 += 1;
        }
        PyTuple_SetItem(py_retval, 1 as libc::c_int as Py_ssize_t, (*pyCtxt).arg);
        if !((*pyCtxt).arg).is_null() {
            let ref mut fresh144 = (*(*pyCtxt).arg).ob_refcnt;
            *fresh144 += 1;
        }
    } else {
        PyTuple_SetItem(py_retval, 0 as libc::c_int as Py_ssize_t, &mut _Py_NoneStruct);
        if !(&mut _Py_NoneStruct as *mut PyObject).is_null() {
            let ref mut fresh145 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
            *fresh145 += 1;
        }
        PyTuple_SetItem(py_retval, 1 as libc::c_int as Py_ssize_t, &mut _Py_NoneStruct);
        if !(&mut _Py_NoneStruct as *mut PyObject).is_null() {
            let ref mut fresh146 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
            *fresh146 += 1;
        }
    }
    return py_retval;
}
unsafe extern "C" fn libxml_xmlFreeTextReader(
    mut self_0: *mut PyObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut reader: xmlTextReaderPtr = 0 as *mut xmlTextReader;
    let mut pyobj_reader: *mut PyObject = 0 as *mut PyObject;
    let mut pyCtxt: xmlTextReaderPyCtxtPtr = 0 as *mut xmlTextReaderPyCtxt;
    let mut f: xmlTextReaderErrorFunc = None;
    let mut arg: *mut libc::c_void = 0 as *mut libc::c_void;
    if _PyArg_ParseTuple_SizeT(
        args,
        b"O:xmlFreeTextReader\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        &mut pyobj_reader as *mut *mut PyObject,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    if !((*pyobj_reader).ob_type == &mut PyCapsule_Type as *mut PyTypeObject) {
        let ref mut fresh147 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
        *fresh147 += 1;
        return &mut _Py_NoneStruct;
    }
    reader = if pyobj_reader == &mut _Py_NoneStruct as *mut PyObject {
        0 as xmlTextReaderPtr
    } else {
        (*(pyobj_reader as *mut PyxmlTextReader_Object)).obj
    };
    if reader.is_null() {
        let ref mut fresh148 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
        *fresh148 += 1;
        return &mut _Py_NoneStruct;
    }
    xmlTextReaderGetErrorHandler(reader, &mut f, &mut arg);
    if !arg.is_null() {
        if f
            == ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        libc::c_int,
                        xmlTextReaderLocatorPtr,
                    ) -> (),
                >,
                xmlTextReaderErrorFunc,
            >(
                Some(
                    libxml_xmlTextReaderErrorCallback
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *const libc::c_char,
                            libc::c_int,
                            xmlTextReaderLocatorPtr,
                        ) -> (),
                ),
            )
        {
            pyCtxt = arg as xmlTextReaderPyCtxtPtr;
            if !((*pyCtxt).f).is_null() {
                let ref mut fresh149 = (*(*pyCtxt).f).ob_refcnt;
                *fresh149 -= 1;
                if !(*fresh149 != 0 as libc::c_int as libc::c_long) {
                    (Some(
                        ((*(*(*pyCtxt).f).ob_type).tp_dealloc)
                            .expect("non-null function pointer"),
                    ))
                        .expect("non-null function pointer")((*pyCtxt).f);
                }
            }
            if !((*pyCtxt).arg).is_null() {
                let ref mut fresh150 = (*(*pyCtxt).arg).ob_refcnt;
                *fresh150 -= 1;
                if !(*fresh150 != 0 as libc::c_int as libc::c_long) {
                    (Some(
                        ((*(*(*pyCtxt).arg).ob_type).tp_dealloc)
                            .expect("non-null function pointer"),
                    ))
                        .expect("non-null function pointer")((*pyCtxt).arg);
                }
            }
            xmlFree.expect("non-null function pointer")(pyCtxt as *mut libc::c_void);
        }
    }
    xmlFreeTextReader(reader);
    let ref mut fresh151 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
    *fresh151 += 1;
    return &mut _Py_NoneStruct;
}
unsafe extern "C" fn libxml_xmlXPathFuncCallback(
    mut ctxt: xmlXPathParserContextPtr,
    mut nargs: libc::c_int,
) {
    let mut list: *mut PyObject = 0 as *mut PyObject;
    let mut cur: *mut PyObject = 0 as *mut PyObject;
    let mut result: *mut PyObject = 0 as *mut PyObject;
    let mut obj: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut rctxt: xmlXPathContextPtr = 0 as *mut xmlXPathContext;
    let mut current_function: *mut PyObject = 0 as *mut PyObject;
    let mut name: *const xmlChar = 0 as *const xmlChar;
    let mut ns_uri: *const xmlChar = 0 as *const xmlChar;
    let mut i: libc::c_int = 0;
    if ctxt.is_null() {
        return;
    }
    rctxt = (*ctxt).context;
    if rctxt.is_null() {
        return;
    }
    name = (*rctxt).function;
    ns_uri = (*rctxt).functionURI;
    i = 0 as libc::c_int;
    while i < libxml_xpathCallbacksNb {
        if xmlStrEqual(
            name,
            (*(*libxml_xpathCallbacks).as_mut_ptr().offset(i as isize)).name,
        ) != 0
            && xmlStrEqual(
                ns_uri,
                (*(*libxml_xpathCallbacks).as_mut_ptr().offset(i as isize)).ns_uri,
            ) != 0
        {
            current_function = (*(*libxml_xpathCallbacks)
                .as_mut_ptr()
                .offset(i as isize))
                .function;
        }
        i += 1;
    }
    if current_function.is_null() {
        printf(
            b"libxml_xmlXPathFuncCallback: internal error %s not found !\n\0"
                as *const u8 as *const libc::c_char,
            name,
        );
        return;
    }
    list = PyTuple_New((nargs + 1 as libc::c_int) as Py_ssize_t);
    PyTuple_SetItem(
        list,
        0 as libc::c_int as Py_ssize_t,
        libxml_xmlXPathParserContextPtrWrap(ctxt),
    );
    i = nargs - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        obj = valuePop(ctxt);
        cur = libxml_xmlXPathObjectPtrWrap(obj);
        PyTuple_SetItem(list, (i + 1 as libc::c_int) as Py_ssize_t, cur);
        i -= 1;
    }
    result = PyEval_CallObjectWithKeywords(
        current_function,
        list,
        0 as *mut libc::c_void as *mut PyObject,
    );
    let ref mut fresh152 = (*list).ob_refcnt;
    *fresh152 -= 1;
    if !(*fresh152 != 0 as libc::c_int as libc::c_long) {
        (Some(((*(*list).ob_type).tp_dealloc).expect("non-null function pointer")))
            .expect("non-null function pointer")(list);
    }
    obj = libxml_xmlXPathObjectPtrConvert(result);
    valuePush(ctxt, obj);
}
unsafe extern "C" fn libxml_xmlXPathFuncLookupFunc(
    mut ctxt: *mut libc::c_void,
    mut name: *const xmlChar,
    mut ns_uri: *const xmlChar,
) -> xmlXPathFunction {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < libxml_xpathCallbacksNb {
        if ctxt
            == (*(*libxml_xpathCallbacks).as_mut_ptr().offset(i as isize)).ctx
                as *mut libc::c_void
            && xmlStrEqual(
                name,
                (*(*libxml_xpathCallbacks).as_mut_ptr().offset(i as isize)).name,
            ) != 0
            && xmlStrEqual(
                ns_uri,
                (*(*libxml_xpathCallbacks).as_mut_ptr().offset(i as isize)).ns_uri,
            ) != 0
        {
            return Some(
                libxml_xmlXPathFuncCallback
                    as unsafe extern "C" fn(xmlXPathParserContextPtr, libc::c_int) -> (),
            );
        }
        i += 1;
    }
    return None;
}
unsafe extern "C" fn libxml_xpathCallbacksInitialize() {
    let mut i: libc::c_int = 0;
    if libxml_xpathCallbacksInitialized != 0 as libc::c_int {
        return;
    }
    libxml_xpathCallbacks = xmlMalloc
        .expect(
            "non-null function pointer",
        )(
        (libxml_xpathCallbacksAllocd as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libxml_xpathCallback>() as libc::c_ulong),
    ) as *mut libxml_xpathCallbackArray;
    i = 0 as libc::c_int;
    while i < libxml_xpathCallbacksAllocd {
        let ref mut fresh153 = (*(*libxml_xpathCallbacks)
            .as_mut_ptr()
            .offset(i as isize))
            .ctx;
        *fresh153 = 0 as xmlXPathContextPtr;
        let ref mut fresh154 = (*(*libxml_xpathCallbacks)
            .as_mut_ptr()
            .offset(i as isize))
            .name;
        *fresh154 = 0 as *mut xmlChar;
        let ref mut fresh155 = (*(*libxml_xpathCallbacks)
            .as_mut_ptr()
            .offset(i as isize))
            .ns_uri;
        *fresh155 = 0 as *mut xmlChar;
        let ref mut fresh156 = (*(*libxml_xpathCallbacks)
            .as_mut_ptr()
            .offset(i as isize))
            .function;
        *fresh156 = 0 as *mut PyObject;
        i += 1;
    }
    libxml_xpathCallbacksInitialized = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlRegisterXPathFunction(
    mut self_0: *mut PyObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut current_block: u64;
    let mut py_retval: *mut PyObject = 0 as *mut PyObject;
    let mut c_retval: libc::c_int = 0 as libc::c_int;
    let mut name: *mut xmlChar = 0 as *mut xmlChar;
    let mut ns_uri: *mut xmlChar = 0 as *mut xmlChar;
    let mut ctx: xmlXPathContextPtr = 0 as *mut xmlXPathContext;
    let mut pyobj_ctx: *mut PyObject = 0 as *mut PyObject;
    let mut pyobj_f: *mut PyObject = 0 as *mut PyObject;
    let mut i: libc::c_int = 0;
    if _PyArg_ParseTuple_SizeT(
        args,
        b"OszO:registerXPathFunction\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        &mut pyobj_ctx as *mut *mut PyObject,
        &mut name as *mut *mut xmlChar,
        &mut ns_uri as *mut *mut xmlChar,
        &mut pyobj_f as *mut *mut PyObject,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    ctx = if pyobj_ctx == &mut _Py_NoneStruct as *mut PyObject {
        0 as xmlXPathContextPtr
    } else {
        (*(pyobj_ctx as *mut PyxmlXPathContext_Object)).obj
    };
    if libxml_xpathCallbacksInitialized == 0 as libc::c_int {
        libxml_xpathCallbacksInitialize();
    }
    xmlXPathRegisterFuncLookup(
        ctx,
        Some(
            libxml_xmlXPathFuncLookupFunc
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const xmlChar,
                    *const xmlChar,
                ) -> xmlXPathFunction,
        ),
        ctx as *mut libc::c_void,
    );
    if pyobj_ctx.is_null() || name.is_null() || pyobj_f.is_null() {
        py_retval = libxml_intWrap(-(1 as libc::c_int));
        return py_retval;
    }
    i = 0 as libc::c_int;
    loop {
        if !(i < libxml_xpathCallbacksNb) {
            current_block = 18377268871191777778;
            break;
        }
        if ctx == (*(*libxml_xpathCallbacks).as_mut_ptr().offset(i as isize)).ctx
            && xmlStrEqual(
                name,
                (*(*libxml_xpathCallbacks).as_mut_ptr().offset(i as isize)).name,
            ) != 0
            && xmlStrEqual(
                ns_uri,
                (*(*libxml_xpathCallbacks).as_mut_ptr().offset(i as isize)).ns_uri,
            ) != 0
        {
            if !pyobj_f.is_null() {
                let ref mut fresh157 = (*pyobj_f).ob_refcnt;
                *fresh157 += 1;
            }
            if !((*(*libxml_xpathCallbacks).as_mut_ptr().offset(i as isize)).function)
                .is_null()
            {
                let ref mut fresh158 = (*(*(*libxml_xpathCallbacks)
                    .as_mut_ptr()
                    .offset(i as isize))
                    .function)
                    .ob_refcnt;
                *fresh158 -= 1;
                if !(*fresh158 != 0 as libc::c_int as libc::c_long) {
                    (Some(
                        ((*(*(*(*libxml_xpathCallbacks).as_mut_ptr().offset(i as isize))
                            .function)
                            .ob_type)
                            .tp_dealloc)
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        (*(*libxml_xpathCallbacks).as_mut_ptr().offset(i as isize))
                            .function,
                    );
                }
            }
            let ref mut fresh159 = (*(*libxml_xpathCallbacks)
                .as_mut_ptr()
                .offset(i as isize))
                .function;
            *fresh159 = pyobj_f;
            c_retval = 1 as libc::c_int;
            current_block = 4356395540054036081;
            break;
        } else {
            i += 1;
        }
    }
    match current_block {
        18377268871191777778 => {
            if libxml_xpathCallbacksNb >= libxml_xpathCallbacksAllocd {
                libxml_xpathCallbacksAllocd += 10 as libc::c_int;
                libxml_xpathCallbacks = xmlRealloc
                    .expect(
                        "non-null function pointer",
                    )(
                    libxml_xpathCallbacks as *mut libc::c_void,
                    (libxml_xpathCallbacksAllocd as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libxml_xpathCallback>()
                                as libc::c_ulong,
                        ),
                ) as *mut libxml_xpathCallbackArray;
            }
            let fresh160 = libxml_xpathCallbacksNb;
            libxml_xpathCallbacksNb = libxml_xpathCallbacksNb + 1;
            i = fresh160;
            if !pyobj_f.is_null() {
                let ref mut fresh161 = (*pyobj_f).ob_refcnt;
                *fresh161 += 1;
            }
            let ref mut fresh162 = (*(*libxml_xpathCallbacks)
                .as_mut_ptr()
                .offset(i as isize))
                .ctx;
            *fresh162 = ctx;
            let ref mut fresh163 = (*(*libxml_xpathCallbacks)
                .as_mut_ptr()
                .offset(i as isize))
                .name;
            *fresh163 = xmlStrdup(name);
            let ref mut fresh164 = (*(*libxml_xpathCallbacks)
                .as_mut_ptr()
                .offset(i as isize))
                .ns_uri;
            *fresh164 = xmlStrdup(ns_uri);
            let ref mut fresh165 = (*(*libxml_xpathCallbacks)
                .as_mut_ptr()
                .offset(i as isize))
                .function;
            *fresh165 = pyobj_f;
            c_retval = 1 as libc::c_int;
        }
        _ => {}
    }
    py_retval = libxml_intWrap(c_retval);
    return py_retval;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlXPathRegisterVariable(
    mut self_0: *mut PyObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut py_retval: *mut PyObject = 0 as *mut PyObject;
    let mut c_retval: libc::c_int = 0 as libc::c_int;
    let mut name: *mut xmlChar = 0 as *mut xmlChar;
    let mut ns_uri: *mut xmlChar = 0 as *mut xmlChar;
    let mut ctx: xmlXPathContextPtr = 0 as *mut xmlXPathContext;
    let mut val: xmlXPathObjectPtr = 0 as *mut xmlXPathObject;
    let mut pyobj_ctx: *mut PyObject = 0 as *mut PyObject;
    let mut pyobj_value: *mut PyObject = 0 as *mut PyObject;
    if _PyArg_ParseTuple_SizeT(
        args,
        b"OszO:xpathRegisterVariable\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        &mut pyobj_ctx as *mut *mut PyObject,
        &mut name as *mut *mut xmlChar,
        &mut ns_uri as *mut *mut xmlChar,
        &mut pyobj_value as *mut *mut PyObject,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    ctx = if pyobj_ctx == &mut _Py_NoneStruct as *mut PyObject {
        0 as xmlXPathContextPtr
    } else {
        (*(pyobj_ctx as *mut PyxmlXPathContext_Object)).obj
    };
    val = libxml_xmlXPathObjectPtrConvert(pyobj_value);
    c_retval = xmlXPathRegisterVariableNS(ctx, name, ns_uri, val);
    py_retval = libxml_intWrap(c_retval);
    return py_retval;
}
unsafe extern "C" fn libxml_name(
    mut self_0: *mut PyObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut resultobj: *mut PyObject = 0 as *mut PyObject;
    let mut obj: *mut PyObject = 0 as *mut PyObject;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut res: *const xmlChar = 0 as *const xmlChar;
    if _PyArg_ParseTuple_SizeT(
        args,
        b"O:name\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut obj as *mut *mut PyObject,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    cur = if obj == &mut _Py_NoneStruct as *mut PyObject {
        0 as xmlNodePtr
    } else {
        (*(obj as *mut PyxmlNode_Object)).obj
    };
    match (*cur).type_0 as libc::c_uint {
        9 | 13 => {
            let mut doc: xmlDocPtr = cur as xmlDocPtr;
            res = (*doc).URL;
        }
        2 => {
            let mut attr: xmlAttrPtr = cur as xmlAttrPtr;
            res = (*attr).name;
        }
        18 => {
            let mut ns: xmlNsPtr = cur as xmlNsPtr;
            res = (*ns).prefix;
        }
        _ => {
            res = (*cur).name;
        }
    }
    resultobj = libxml_constxmlCharPtrWrap(res);
    return resultobj;
}
unsafe extern "C" fn libxml_doc(
    mut self_0: *mut PyObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut resultobj: *mut PyObject = 0 as *mut PyObject;
    let mut obj: *mut PyObject = 0 as *mut PyObject;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut res: xmlDocPtr = 0 as *mut xmlDoc;
    if _PyArg_ParseTuple_SizeT(
        args,
        b"O:doc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut obj as *mut *mut PyObject,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    cur = if obj == &mut _Py_NoneStruct as *mut PyObject {
        0 as xmlNodePtr
    } else {
        (*(obj as *mut PyxmlNode_Object)).obj
    };
    match (*cur).type_0 as libc::c_uint {
        9 | 13 => {
            res = 0 as xmlDocPtr;
        }
        2 => {
            let mut attr: xmlAttrPtr = cur as xmlAttrPtr;
            res = (*attr).doc;
        }
        18 => {
            res = 0 as xmlDocPtr;
        }
        _ => {
            res = (*cur).doc;
        }
    }
    resultobj = libxml_xmlDocPtrWrap(res);
    return resultobj;
}
unsafe extern "C" fn libxml_properties(
    mut self_0: *mut PyObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut resultobj: *mut PyObject = 0 as *mut PyObject;
    let mut obj: *mut PyObject = 0 as *mut PyObject;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut res: xmlAttrPtr = 0 as *mut xmlAttr;
    if _PyArg_ParseTuple_SizeT(
        args,
        b"O:properties\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut obj as *mut *mut PyObject,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    cur = if obj == &mut _Py_NoneStruct as *mut PyObject {
        0 as xmlNodePtr
    } else {
        (*(obj as *mut PyxmlNode_Object)).obj
    };
    if !cur.is_null()
        && (*cur).type_0 as libc::c_uint
            == XML_ELEMENT_NODE as libc::c_int as libc::c_uint
    {
        res = (*cur).properties;
    } else {
        res = 0 as xmlAttrPtr;
    }
    resultobj = libxml_xmlAttrPtrWrap(res);
    return resultobj;
}
unsafe extern "C" fn libxml_next(
    mut self_0: *mut PyObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut resultobj: *mut PyObject = 0 as *mut PyObject;
    let mut obj: *mut PyObject = 0 as *mut PyObject;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut res: xmlNodePtr = 0 as *mut xmlNode;
    if _PyArg_ParseTuple_SizeT(
        args,
        b"O:next\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut obj as *mut *mut PyObject,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    cur = if obj == &mut _Py_NoneStruct as *mut PyObject {
        0 as xmlNodePtr
    } else {
        (*(obj as *mut PyxmlNode_Object)).obj
    };
    match (*cur).type_0 as libc::c_uint {
        9 | 13 => {
            res = 0 as xmlNodePtr;
        }
        2 => {
            let mut attr: xmlAttrPtr = cur as xmlAttrPtr;
            res = (*attr).next as xmlNodePtr;
        }
        18 => {
            let mut ns: xmlNsPtr = cur as xmlNsPtr;
            res = (*ns).next as xmlNodePtr;
        }
        _ => {
            res = (*cur).next;
        }
    }
    resultobj = libxml_xmlNodePtrWrap(res);
    return resultobj;
}
unsafe extern "C" fn libxml_prev(
    mut self_0: *mut PyObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut resultobj: *mut PyObject = 0 as *mut PyObject;
    let mut obj: *mut PyObject = 0 as *mut PyObject;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut res: xmlNodePtr = 0 as *mut xmlNode;
    if _PyArg_ParseTuple_SizeT(
        args,
        b"O:prev\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut obj as *mut *mut PyObject,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    cur = if obj == &mut _Py_NoneStruct as *mut PyObject {
        0 as xmlNodePtr
    } else {
        (*(obj as *mut PyxmlNode_Object)).obj
    };
    match (*cur).type_0 as libc::c_uint {
        9 | 13 => {
            res = 0 as xmlNodePtr;
        }
        2 => {
            let mut attr: xmlAttrPtr = cur as xmlAttrPtr;
            res = (*attr).prev as xmlNodePtr;
        }
        18 => {
            res = 0 as xmlNodePtr;
        }
        _ => {
            res = (*cur).prev;
        }
    }
    resultobj = libxml_xmlNodePtrWrap(res);
    return resultobj;
}
unsafe extern "C" fn libxml_children(
    mut self_0: *mut PyObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut resultobj: *mut PyObject = 0 as *mut PyObject;
    let mut obj: *mut PyObject = 0 as *mut PyObject;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut res: xmlNodePtr = 0 as *mut xmlNode;
    if _PyArg_ParseTuple_SizeT(
        args,
        b"O:children\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut obj as *mut *mut PyObject,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    cur = if obj == &mut _Py_NoneStruct as *mut PyObject {
        0 as xmlNodePtr
    } else {
        (*(obj as *mut PyxmlNode_Object)).obj
    };
    match (*cur).type_0 as libc::c_uint {
        1 | 5 | 6 | 7 | 8 | 9 | 13 | 14 => {
            res = (*cur).children;
        }
        2 => {
            let mut attr: xmlAttrPtr = cur as xmlAttrPtr;
            res = (*attr).children;
        }
        _ => {
            res = 0 as xmlNodePtr;
        }
    }
    resultobj = libxml_xmlNodePtrWrap(res);
    return resultobj;
}
unsafe extern "C" fn libxml_last(
    mut self_0: *mut PyObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut resultobj: *mut PyObject = 0 as *mut PyObject;
    let mut obj: *mut PyObject = 0 as *mut PyObject;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut res: xmlNodePtr = 0 as *mut xmlNode;
    if _PyArg_ParseTuple_SizeT(
        args,
        b"O:last\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut obj as *mut *mut PyObject,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    cur = if obj == &mut _Py_NoneStruct as *mut PyObject {
        0 as xmlNodePtr
    } else {
        (*(obj as *mut PyxmlNode_Object)).obj
    };
    match (*cur).type_0 as libc::c_uint {
        1 | 5 | 6 | 7 | 8 | 9 | 13 | 14 => {
            res = (*cur).last;
        }
        2 => {
            let mut attr: xmlAttrPtr = cur as xmlAttrPtr;
            res = (*attr).last;
        }
        _ => {
            res = 0 as xmlNodePtr;
        }
    }
    resultobj = libxml_xmlNodePtrWrap(res);
    return resultobj;
}
unsafe extern "C" fn libxml_parent(
    mut self_0: *mut PyObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut resultobj: *mut PyObject = 0 as *mut PyObject;
    let mut obj: *mut PyObject = 0 as *mut PyObject;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut res: xmlNodePtr = 0 as *mut xmlNode;
    if _PyArg_ParseTuple_SizeT(
        args,
        b"O:parent\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut obj as *mut *mut PyObject,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    cur = if obj == &mut _Py_NoneStruct as *mut PyObject {
        0 as xmlNodePtr
    } else {
        (*(obj as *mut PyxmlNode_Object)).obj
    };
    match (*cur).type_0 as libc::c_uint {
        9 | 13 => {
            res = 0 as xmlNodePtr;
        }
        2 => {
            let mut attr: xmlAttrPtr = cur as xmlAttrPtr;
            res = (*attr).parent;
        }
        17 | 18 | 19 | 20 => {
            res = 0 as xmlNodePtr;
        }
        _ => {
            res = (*cur).parent;
        }
    }
    resultobj = libxml_xmlNodePtrWrap(res);
    return resultobj;
}
unsafe extern "C" fn libxml_type(
    mut self_0: *mut PyObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut resultobj: *mut PyObject = 0 as *mut PyObject;
    let mut obj: *mut PyObject = 0 as *mut PyObject;
    let mut cur: xmlNodePtr = 0 as *mut xmlNode;
    let mut res: *const xmlChar = 0 as *const xmlChar;
    if _PyArg_ParseTuple_SizeT(
        args,
        b"O:last\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut obj as *mut *mut PyObject,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    cur = if obj == &mut _Py_NoneStruct as *mut PyObject {
        0 as xmlNodePtr
    } else {
        (*(obj as *mut PyxmlNode_Object)).obj
    };
    if cur.is_null() {
        let ref mut fresh166 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
        *fresh166 += 1;
        return &mut _Py_NoneStruct;
    }
    match (*cur).type_0 as libc::c_uint {
        1 => {
            res = b"element\0" as *const u8 as *const libc::c_char as *const xmlChar;
        }
        2 => {
            res = b"attribute\0" as *const u8 as *const libc::c_char as *const xmlChar;
        }
        3 => {
            res = b"text\0" as *const u8 as *const libc::c_char as *const xmlChar;
        }
        4 => {
            res = b"cdata\0" as *const u8 as *const libc::c_char as *const xmlChar;
        }
        5 => {
            res = b"entity_ref\0" as *const u8 as *const libc::c_char as *const xmlChar;
        }
        6 => {
            res = b"entity\0" as *const u8 as *const libc::c_char as *const xmlChar;
        }
        7 => {
            res = b"pi\0" as *const u8 as *const libc::c_char as *const xmlChar;
        }
        8 => {
            res = b"comment\0" as *const u8 as *const libc::c_char as *const xmlChar;
        }
        9 => {
            res = b"document_xml\0" as *const u8 as *const libc::c_char
                as *const xmlChar;
        }
        10 => {
            res = b"doctype\0" as *const u8 as *const libc::c_char as *const xmlChar;
        }
        11 => {
            res = b"fragment\0" as *const u8 as *const libc::c_char as *const xmlChar;
        }
        12 => {
            res = b"notation\0" as *const u8 as *const libc::c_char as *const xmlChar;
        }
        13 => {
            res = b"document_html\0" as *const u8 as *const libc::c_char
                as *const xmlChar;
        }
        14 => {
            res = b"dtd\0" as *const u8 as *const libc::c_char as *const xmlChar;
        }
        15 => {
            res = b"elem_decl\0" as *const u8 as *const libc::c_char as *const xmlChar;
        }
        16 => {
            res = b"attribute_decl\0" as *const u8 as *const libc::c_char
                as *const xmlChar;
        }
        17 => {
            res = b"entity_decl\0" as *const u8 as *const libc::c_char as *const xmlChar;
        }
        18 => {
            res = b"namespace\0" as *const u8 as *const libc::c_char as *const xmlChar;
        }
        19 => {
            res = b"xinclude_start\0" as *const u8 as *const libc::c_char
                as *const xmlChar;
        }
        20 => {
            res = b"xinclude_end\0" as *const u8 as *const libc::c_char
                as *const xmlChar;
        }
        _ => {}
    }
    resultobj = libxml_constxmlCharPtrWrap(res);
    return resultobj;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlNodeGetNsDefs(
    mut self_0: *mut PyObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut py_retval: *mut PyObject = 0 as *mut PyObject;
    let mut c_retval: xmlNsPtr = 0 as *mut xmlNs;
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    let mut pyobj_node: *mut PyObject = 0 as *mut PyObject;
    if _PyArg_ParseTuple_SizeT(
        args,
        b"O:xmlNodeGetNsDefs\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut pyobj_node as *mut *mut PyObject,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    node = if pyobj_node == &mut _Py_NoneStruct as *mut PyObject {
        0 as xmlNodePtr
    } else {
        (*(pyobj_node as *mut PyxmlNode_Object)).obj
    };
    if node.is_null()
        || (*node).type_0 as libc::c_uint
            != XML_ELEMENT_NODE as libc::c_int as libc::c_uint
    {
        let ref mut fresh167 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
        *fresh167 += 1;
        return &mut _Py_NoneStruct;
    }
    c_retval = (*node).nsDef;
    py_retval = libxml_xmlNsPtrWrap(c_retval);
    return py_retval;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlNodeRemoveNsDef(
    mut self_0: *mut PyObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut py_retval: *mut PyObject = 0 as *mut PyObject;
    let mut ns: xmlNsPtr = 0 as *mut xmlNs;
    let mut prev: xmlNsPtr = 0 as *mut xmlNs;
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    let mut pyobj_node: *mut PyObject = 0 as *mut PyObject;
    let mut href: *mut xmlChar = 0 as *mut xmlChar;
    let mut c_retval: xmlNsPtr = 0 as *mut xmlNs;
    if _PyArg_ParseTuple_SizeT(
        args,
        b"Oz:xmlNodeRemoveNsDef\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        &mut pyobj_node as *mut *mut PyObject,
        &mut href as *mut *mut xmlChar,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    node = if pyobj_node == &mut _Py_NoneStruct as *mut PyObject {
        0 as xmlNodePtr
    } else {
        (*(pyobj_node as *mut PyxmlNode_Object)).obj
    };
    ns = 0 as xmlNsPtr;
    if node.is_null()
        || (*node).type_0 as libc::c_uint
            != XML_ELEMENT_NODE as libc::c_int as libc::c_uint
    {
        let ref mut fresh168 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
        *fresh168 += 1;
        return &mut _Py_NoneStruct;
    }
    if href.is_null() {
        ns = (*node).nsDef;
        let ref mut fresh169 = (*node).nsDef;
        *fresh169 = 0 as *mut xmlNs;
        c_retval = 0 as xmlNsPtr;
    } else {
        prev = 0 as xmlNsPtr;
        ns = (*node).nsDef;
        while !ns.is_null() {
            if xmlStrEqual((*ns).href, href) != 0 {
                if !prev.is_null() {
                    let ref mut fresh170 = (*prev).next;
                    *fresh170 = (*ns).next;
                } else {
                    let ref mut fresh171 = (*node).nsDef;
                    *fresh171 = (*ns).next;
                }
                let ref mut fresh172 = (*ns).next;
                *fresh172 = 0 as *mut _xmlNs;
                c_retval = 0 as xmlNsPtr;
                break;
            } else {
                prev = ns;
                ns = (*ns).next;
            }
        }
    }
    c_retval = ns;
    py_retval = libxml_xmlNsPtrWrap(c_retval);
    return py_retval;
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlNodeGetNs(
    mut self_0: *mut PyObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut py_retval: *mut PyObject = 0 as *mut PyObject;
    let mut c_retval: xmlNsPtr = 0 as *mut xmlNs;
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    let mut pyobj_node: *mut PyObject = 0 as *mut PyObject;
    if _PyArg_ParseTuple_SizeT(
        args,
        b"O:xmlNodeGetNs\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut pyobj_node as *mut *mut PyObject,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    node = if pyobj_node == &mut _Py_NoneStruct as *mut PyObject {
        0 as xmlNodePtr
    } else {
        (*(pyobj_node as *mut PyxmlNode_Object)).obj
    };
    if node.is_null()
        || (*node).type_0 as libc::c_uint
            != XML_ELEMENT_NODE as libc::c_int as libc::c_uint
            && (*node).type_0 as libc::c_uint
                != XML_ATTRIBUTE_NODE as libc::c_int as libc::c_uint
    {
        let ref mut fresh173 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
        *fresh173 += 1;
        return &mut _Py_NoneStruct;
    }
    c_retval = (*node).ns;
    py_retval = libxml_xmlNsPtrWrap(c_retval);
    return py_retval;
}
unsafe extern "C" fn libxml_serializeNode(
    mut self_0: *mut PyObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut py_retval: *mut PyObject = 0 as *mut PyObject;
    let mut c_retval: *mut xmlChar = 0 as *mut xmlChar;
    let mut pyobj_node: *mut PyObject = 0 as *mut PyObject;
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    let mut doc: xmlDocPtr = 0 as *mut xmlDoc;
    let mut encoding: *const libc::c_char = 0 as *const libc::c_char;
    let mut format: libc::c_int = 0;
    let mut ctxt: xmlSaveCtxtPtr = 0 as *mut xmlSaveCtxt;
    let mut buf: xmlBufferPtr = 0 as *mut xmlBuffer;
    let mut options: libc::c_int = 0 as libc::c_int;
    if _PyArg_ParseTuple_SizeT(
        args,
        b"Ozi:serializeNode\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut pyobj_node as *mut *mut PyObject,
        &mut encoding as *mut *const libc::c_char,
        &mut format as *mut libc::c_int,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    node = if pyobj_node == &mut _Py_NoneStruct as *mut PyObject {
        0 as xmlNodePtr
    } else {
        (*(pyobj_node as *mut PyxmlNode_Object)).obj
    };
    if node.is_null() {
        let ref mut fresh174 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
        *fresh174 += 1;
        return &mut _Py_NoneStruct;
    }
    if (*node).type_0 as libc::c_uint == XML_DOCUMENT_NODE as libc::c_int as libc::c_uint
    {
        doc = node as xmlDocPtr;
        node = 0 as xmlNodePtr;
    } else if (*node).type_0 as libc::c_uint
            == XML_HTML_DOCUMENT_NODE as libc::c_int as libc::c_uint
        {
        doc = node as xmlDocPtr;
        node = 0 as xmlNodePtr;
    } else {
        if (*node).type_0 as libc::c_uint
            == XML_NAMESPACE_DECL as libc::c_int as libc::c_uint
        {
            doc = 0 as xmlDocPtr;
        } else {
            doc = (*node).doc;
        }
        if !(doc.is_null()
            || (*doc).type_0 as libc::c_uint
                == XML_DOCUMENT_NODE as libc::c_int as libc::c_uint)
        {
            if (*doc).type_0 as libc::c_uint
                == XML_HTML_DOCUMENT_NODE as libc::c_int as libc::c_uint
            {} else {
                let ref mut fresh175 = (*(&mut _Py_NoneStruct as *mut PyObject))
                    .ob_refcnt;
                *fresh175 += 1;
                return &mut _Py_NoneStruct;
            }
        }
    }
    buf = xmlBufferCreate();
    if buf.is_null() {
        let ref mut fresh176 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
        *fresh176 += 1;
        return &mut _Py_NoneStruct;
    }
    if format != 0 {
        options |= XML_SAVE_FORMAT as libc::c_int;
    }
    ctxt = xmlSaveToBuffer(buf, encoding, options);
    if ctxt.is_null() {
        xmlBufferFree(buf);
        let ref mut fresh177 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
        *fresh177 += 1;
        return &mut _Py_NoneStruct;
    }
    if node.is_null() {
        xmlSaveDoc(ctxt, doc);
    } else {
        xmlSaveTree(ctxt, node);
    }
    xmlSaveClose(ctxt);
    c_retval = (*buf).content;
    let ref mut fresh178 = (*buf).content;
    *fresh178 = 0 as *mut xmlChar;
    xmlBufferFree(buf);
    py_retval = libxml_charPtrWrap(c_retval as *mut libc::c_char);
    return py_retval;
}
unsafe extern "C" fn libxml_saveNodeTo(
    mut self_0: *mut PyObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut py_file: *mut PyObject = 0 as *mut PyObject;
    let mut output: *mut FILE = 0 as *mut FILE;
    let mut pyobj_node: *mut PyObject = 0 as *mut PyObject;
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    let mut doc: xmlDocPtr = 0 as *mut xmlDoc;
    let mut encoding: *const libc::c_char = 0 as *const libc::c_char;
    let mut format: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut buf: xmlOutputBufferPtr = 0 as *mut xmlOutputBuffer;
    let mut handler: xmlCharEncodingHandlerPtr = 0 as xmlCharEncodingHandlerPtr;
    if _PyArg_ParseTuple_SizeT(
        args,
        b"OOzi:serializeNode\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut pyobj_node as *mut *mut PyObject,
        &mut py_file as *mut *mut PyObject,
        &mut encoding as *mut *const libc::c_char,
        &mut format as *mut libc::c_int,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    node = if pyobj_node == &mut _Py_NoneStruct as *mut PyObject {
        0 as xmlNodePtr
    } else {
        (*(pyobj_node as *mut PyxmlNode_Object)).obj
    };
    if node.is_null() {
        return PyLong_FromLong(-(1 as libc::c_int) as libc::c_long);
    }
    output = if py_file == &mut _Py_NoneStruct as *mut PyObject {
        0 as *mut FILE
    } else if (*py_file).ob_type == &mut PyFile_Type as *mut PyTypeObject
            || PyType_IsSubtype((*py_file).ob_type, &mut PyFile_Type) != 0
        {
        PyFile_AsFile(py_file)
    } else {
        stdout
    };
    if output.is_null() {
        return PyLong_FromLong(-(1 as libc::c_int) as libc::c_long);
    }
    if (*node).type_0 as libc::c_uint == XML_DOCUMENT_NODE as libc::c_int as libc::c_uint
    {
        doc = node as xmlDocPtr;
    } else if (*node).type_0 as libc::c_uint
            == XML_HTML_DOCUMENT_NODE as libc::c_int as libc::c_uint
        {
        doc = node as xmlDocPtr;
    } else {
        doc = (*node).doc;
    }
    if (*doc).type_0 as libc::c_uint
        == XML_HTML_DOCUMENT_NODE as libc::c_int as libc::c_uint
    {
        if encoding.is_null() {
            encoding = htmlGetMetaEncoding(doc) as *const libc::c_char;
        }
    }
    if !encoding.is_null() {
        handler = xmlFindCharEncodingHandler(encoding);
        if handler.is_null() {
            return PyLong_FromLong(-(1 as libc::c_int) as libc::c_long);
        }
    }
    if (*doc).type_0 as libc::c_uint
        == XML_HTML_DOCUMENT_NODE as libc::c_int as libc::c_uint
    {
        if handler.is_null() {
            handler = xmlFindCharEncodingHandler(
                b"HTML\0" as *const u8 as *const libc::c_char,
            );
        }
        if handler.is_null() {
            handler = xmlFindCharEncodingHandler(
                b"ascii\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    buf = xmlOutputBufferCreateFile(output, handler);
    if (*node).type_0 as libc::c_uint == XML_DOCUMENT_NODE as libc::c_int as libc::c_uint
    {
        len = xmlSaveFormatFileTo(buf, doc, encoding, format);
    } else if (*node).type_0 as libc::c_uint
            == XML_HTML_DOCUMENT_NODE as libc::c_int as libc::c_uint
        {
        htmlDocContentDumpFormatOutput(buf, doc, encoding, format);
        len = xmlOutputBufferClose(buf);
    } else if (*doc).type_0 as libc::c_uint
            == XML_HTML_DOCUMENT_NODE as libc::c_int as libc::c_uint
        {
        htmlNodeDumpFormatOutput(buf, doc, node, encoding, format);
        len = xmlOutputBufferClose(buf);
    } else {
        xmlNodeDumpOutput(buf, doc, node, 0 as libc::c_int, format, encoding);
        len = xmlOutputBufferClose(buf);
    }
    return PyLong_FromLong(len as libc::c_long);
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlNewNode(
    mut self_0: *mut PyObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut py_retval: *mut PyObject = 0 as *mut PyObject;
    let mut name: *mut xmlChar = 0 as *mut xmlChar;
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    if _PyArg_ParseTuple_SizeT(
        args,
        b"s:xmlNewNode\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut name as *mut *mut xmlChar,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    node = xmlNewNode(0 as xmlNsPtr, name);
    if node.is_null() {
        let ref mut fresh179 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
        *fresh179 += 1;
        return &mut _Py_NoneStruct;
    }
    py_retval = libxml_xmlNodePtrWrap(node);
    return py_retval;
}
unsafe extern "C" fn libxml_addLocalCatalog(
    mut self_0: *mut PyObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut URL: *mut xmlChar = 0 as *mut xmlChar;
    let mut ctxt: xmlParserCtxtPtr = 0 as *mut xmlParserCtxt;
    let mut pyobj_ctxt: *mut PyObject = 0 as *mut PyObject;
    if _PyArg_ParseTuple_SizeT(
        args,
        b"Os:addLocalCatalog\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut pyobj_ctxt as *mut *mut PyObject,
        &mut URL as *mut *mut xmlChar,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    ctxt = if pyobj_ctxt == &mut _Py_NoneStruct as *mut PyObject {
        0 as xmlParserCtxtPtr
    } else {
        (*(pyobj_ctxt as *mut PyparserCtxt_Object)).obj
    };
    if !URL.is_null() {
        let ref mut fresh180 = (*ctxt).catalogs;
        *fresh180 = xmlCatalogAddLocal((*ctxt).catalogs, URL);
    }
    let ref mut fresh181 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
    *fresh181 += 1;
    return &mut _Py_NoneStruct;
}
unsafe extern "C" fn libxml_xmlRelaxNGValidityGenericErrorFuncHandler(
    mut ctx: *mut libc::c_void,
    mut str: *mut libc::c_char,
) {
    let mut list: *mut PyObject = 0 as *mut PyObject;
    let mut result: *mut PyObject = 0 as *mut PyObject;
    let mut pyCtxt: xmlRelaxNGValidCtxtPyCtxtPtr = 0 as *mut xmlRelaxNGValidCtxtPyCtxt;
    pyCtxt = ctx as xmlRelaxNGValidCtxtPyCtxtPtr;
    list = PyTuple_New(2 as libc::c_int as Py_ssize_t);
    PyTuple_SetItem(list, 0 as libc::c_int as Py_ssize_t, libxml_charPtrWrap(str));
    PyTuple_SetItem(list, 1 as libc::c_int as Py_ssize_t, (*pyCtxt).arg);
    if !((*pyCtxt).arg).is_null() {
        let ref mut fresh182 = (*(*pyCtxt).arg).ob_refcnt;
        *fresh182 += 1;
    }
    result = PyEval_CallObjectWithKeywords(
        (*pyCtxt).error,
        list,
        0 as *mut libc::c_void as *mut PyObject,
    );
    if result.is_null() {
        PyErr_Print();
    }
    if !list.is_null() {
        let ref mut fresh183 = (*list).ob_refcnt;
        *fresh183 -= 1;
        if !(*fresh183 != 0 as libc::c_int as libc::c_long) {
            (Some(((*(*list).ob_type).tp_dealloc).expect("non-null function pointer")))
                .expect("non-null function pointer")(list);
        }
    }
    if !result.is_null() {
        let ref mut fresh184 = (*result).ob_refcnt;
        *fresh184 -= 1;
        if !(*fresh184 != 0 as libc::c_int as libc::c_long) {
            (Some(((*(*result).ob_type).tp_dealloc).expect("non-null function pointer")))
                .expect("non-null function pointer")(result);
        }
    }
}
unsafe extern "C" fn libxml_xmlRelaxNGValidityGenericWarningFuncHandler(
    mut ctx: *mut libc::c_void,
    mut str: *mut libc::c_char,
) {
    let mut list: *mut PyObject = 0 as *mut PyObject;
    let mut result: *mut PyObject = 0 as *mut PyObject;
    let mut pyCtxt: xmlRelaxNGValidCtxtPyCtxtPtr = 0 as *mut xmlRelaxNGValidCtxtPyCtxt;
    pyCtxt = ctx as xmlRelaxNGValidCtxtPyCtxtPtr;
    list = PyTuple_New(2 as libc::c_int as Py_ssize_t);
    PyTuple_SetItem(list, 0 as libc::c_int as Py_ssize_t, libxml_charPtrWrap(str));
    PyTuple_SetItem(list, 1 as libc::c_int as Py_ssize_t, (*pyCtxt).arg);
    if !((*pyCtxt).arg).is_null() {
        let ref mut fresh185 = (*(*pyCtxt).arg).ob_refcnt;
        *fresh185 += 1;
    }
    result = PyEval_CallObjectWithKeywords(
        (*pyCtxt).warn,
        list,
        0 as *mut libc::c_void as *mut PyObject,
    );
    if result.is_null() {
        PyErr_Print();
    }
    if !list.is_null() {
        let ref mut fresh186 = (*list).ob_refcnt;
        *fresh186 -= 1;
        if !(*fresh186 != 0 as libc::c_int as libc::c_long) {
            (Some(((*(*list).ob_type).tp_dealloc).expect("non-null function pointer")))
                .expect("non-null function pointer")(list);
        }
    }
    if !result.is_null() {
        let ref mut fresh187 = (*result).ob_refcnt;
        *fresh187 -= 1;
        if !(*fresh187 != 0 as libc::c_int as libc::c_long) {
            (Some(((*(*result).ob_type).tp_dealloc).expect("non-null function pointer")))
                .expect("non-null function pointer")(result);
        }
    }
}
unsafe extern "C" fn libxml_xmlRelaxNGValidityErrorFunc(
    mut ctx: *mut libc::c_void,
    mut msg: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    libxml_xmlRelaxNGValidityGenericErrorFuncHandler(
        ctx,
        libxml_buildMessage(msg, ap.as_va_list()),
    );
}
unsafe extern "C" fn libxml_xmlRelaxNGValidityWarningFunc(
    mut ctx: *mut libc::c_void,
    mut msg: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    libxml_xmlRelaxNGValidityGenericWarningFuncHandler(
        ctx,
        libxml_buildMessage(msg, ap.as_va_list()),
    );
}
unsafe extern "C" fn libxml_xmlRelaxNGSetValidErrors(
    mut self_0: *mut PyObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut py_retval: *mut PyObject = 0 as *mut PyObject;
    let mut pyobj_error: *mut PyObject = 0 as *mut PyObject;
    let mut pyobj_warn: *mut PyObject = 0 as *mut PyObject;
    let mut pyobj_ctx: *mut PyObject = 0 as *mut PyObject;
    let mut pyobj_arg: *mut PyObject = &mut _Py_NoneStruct;
    let mut ctxt: xmlRelaxNGValidCtxtPtr = 0 as *mut xmlRelaxNGValidCtxt;
    let mut pyCtxt: xmlRelaxNGValidCtxtPyCtxtPtr = 0 as *mut xmlRelaxNGValidCtxtPyCtxt;
    if _PyArg_ParseTuple_SizeT(
        args,
        b"OOO|O:xmlRelaxNGSetValidErrors\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        &mut pyobj_ctx as *mut *mut PyObject,
        &mut pyobj_error as *mut *mut PyObject,
        &mut pyobj_warn as *mut *mut PyObject,
        &mut pyobj_arg as *mut *mut PyObject,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    ctxt = if pyobj_ctx == &mut _Py_NoneStruct as *mut PyObject {
        0 as xmlRelaxNGValidCtxtPtr
    } else {
        (*(pyobj_ctx as *mut PyrelaxNgValidCtxt_Object)).obj
    };
    if xmlRelaxNGGetValidErrors(
        ctxt,
        0 as *mut xmlRelaxNGValidityErrorFunc,
        0 as *mut xmlRelaxNGValidityWarningFunc,
        &mut pyCtxt as *mut xmlRelaxNGValidCtxtPyCtxtPtr as *mut *mut libc::c_void,
    ) == -(1 as libc::c_int)
    {
        py_retval = libxml_intWrap(-(1 as libc::c_int));
        return py_retval;
    }
    if pyCtxt.is_null() {
        pyCtxt = xmlMalloc
            .expect(
                "non-null function pointer",
            )(::std::mem::size_of::<xmlRelaxNGValidCtxtPyCtxt>() as libc::c_ulong)
            as xmlRelaxNGValidCtxtPyCtxtPtr;
        if pyCtxt.is_null() {
            py_retval = libxml_intWrap(-(1 as libc::c_int));
            return py_retval;
        }
        memset(
            pyCtxt as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<xmlRelaxNGValidCtxtPyCtxt>() as libc::c_ulong,
        );
    }
    if !((*pyCtxt).error).is_null() {
        let ref mut fresh188 = (*(*pyCtxt).error).ob_refcnt;
        *fresh188 -= 1;
        if !(*fresh188 != 0 as libc::c_int as libc::c_long) {
            (Some(
                ((*(*(*pyCtxt).error).ob_type).tp_dealloc)
                    .expect("non-null function pointer"),
            ))
                .expect("non-null function pointer")((*pyCtxt).error);
        }
    }
    if !pyobj_error.is_null() {
        let ref mut fresh189 = (*pyobj_error).ob_refcnt;
        *fresh189 += 1;
    }
    let ref mut fresh190 = (*pyCtxt).error;
    *fresh190 = pyobj_error;
    if !((*pyCtxt).warn).is_null() {
        let ref mut fresh191 = (*(*pyCtxt).warn).ob_refcnt;
        *fresh191 -= 1;
        if !(*fresh191 != 0 as libc::c_int as libc::c_long) {
            (Some(
                ((*(*(*pyCtxt).warn).ob_type).tp_dealloc)
                    .expect("non-null function pointer"),
            ))
                .expect("non-null function pointer")((*pyCtxt).warn);
        }
    }
    if !pyobj_warn.is_null() {
        let ref mut fresh192 = (*pyobj_warn).ob_refcnt;
        *fresh192 += 1;
    }
    let ref mut fresh193 = (*pyCtxt).warn;
    *fresh193 = pyobj_warn;
    if !((*pyCtxt).arg).is_null() {
        let ref mut fresh194 = (*(*pyCtxt).arg).ob_refcnt;
        *fresh194 -= 1;
        if !(*fresh194 != 0 as libc::c_int as libc::c_long) {
            (Some(
                ((*(*(*pyCtxt).arg).ob_type).tp_dealloc)
                    .expect("non-null function pointer"),
            ))
                .expect("non-null function pointer")((*pyCtxt).arg);
        }
    }
    if !pyobj_arg.is_null() {
        let ref mut fresh195 = (*pyobj_arg).ob_refcnt;
        *fresh195 += 1;
    }
    let ref mut fresh196 = (*pyCtxt).arg;
    *fresh196 = pyobj_arg;
    xmlRelaxNGSetValidErrors(
        ctxt,
        Some(
            libxml_xmlRelaxNGValidityErrorFunc
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                    ...
                ) -> (),
        ),
        Some(
            libxml_xmlRelaxNGValidityWarningFunc
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                    ...
                ) -> (),
        ),
        pyCtxt as *mut libc::c_void,
    );
    py_retval = libxml_intWrap(1 as libc::c_int);
    return py_retval;
}
unsafe extern "C" fn libxml_xmlRelaxNGFreeValidCtxt(
    mut self_0: *mut PyObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut ctxt: xmlRelaxNGValidCtxtPtr = 0 as *mut xmlRelaxNGValidCtxt;
    let mut pyCtxt: xmlRelaxNGValidCtxtPyCtxtPtr = 0 as *mut xmlRelaxNGValidCtxtPyCtxt;
    let mut pyobj_ctxt: *mut PyObject = 0 as *mut PyObject;
    if _PyArg_ParseTuple_SizeT(
        args,
        b"O:xmlRelaxNGFreeValidCtxt\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        &mut pyobj_ctxt as *mut *mut PyObject,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    ctxt = if pyobj_ctxt == &mut _Py_NoneStruct as *mut PyObject {
        0 as xmlRelaxNGValidCtxtPtr
    } else {
        (*(pyobj_ctxt as *mut PyrelaxNgValidCtxt_Object)).obj
    };
    if xmlRelaxNGGetValidErrors(
        ctxt,
        0 as *mut xmlRelaxNGValidityErrorFunc,
        0 as *mut xmlRelaxNGValidityWarningFunc,
        &mut pyCtxt as *mut xmlRelaxNGValidCtxtPyCtxtPtr as *mut *mut libc::c_void,
    ) == 0 as libc::c_int
    {
        if !pyCtxt.is_null() {
            if !((*pyCtxt).error).is_null() {
                let ref mut fresh197 = (*(*pyCtxt).error).ob_refcnt;
                *fresh197 -= 1;
                if !(*fresh197 != 0 as libc::c_int as libc::c_long) {
                    (Some(
                        ((*(*(*pyCtxt).error).ob_type).tp_dealloc)
                            .expect("non-null function pointer"),
                    ))
                        .expect("non-null function pointer")((*pyCtxt).error);
                }
            }
            if !((*pyCtxt).warn).is_null() {
                let ref mut fresh198 = (*(*pyCtxt).warn).ob_refcnt;
                *fresh198 -= 1;
                if !(*fresh198 != 0 as libc::c_int as libc::c_long) {
                    (Some(
                        ((*(*(*pyCtxt).warn).ob_type).tp_dealloc)
                            .expect("non-null function pointer"),
                    ))
                        .expect("non-null function pointer")((*pyCtxt).warn);
                }
            }
            if !((*pyCtxt).arg).is_null() {
                let ref mut fresh199 = (*(*pyCtxt).arg).ob_refcnt;
                *fresh199 -= 1;
                if !(*fresh199 != 0 as libc::c_int as libc::c_long) {
                    (Some(
                        ((*(*(*pyCtxt).arg).ob_type).tp_dealloc)
                            .expect("non-null function pointer"),
                    ))
                        .expect("non-null function pointer")((*pyCtxt).arg);
                }
            }
            xmlFree.expect("non-null function pointer")(pyCtxt as *mut libc::c_void);
        }
    }
    xmlRelaxNGFreeValidCtxt(ctxt);
    let ref mut fresh200 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
    *fresh200 += 1;
    return &mut _Py_NoneStruct;
}
unsafe extern "C" fn libxml_xmlSchemaValidityGenericErrorFuncHandler(
    mut ctx: *mut libc::c_void,
    mut str: *mut libc::c_char,
) {
    let mut list: *mut PyObject = 0 as *mut PyObject;
    let mut result: *mut PyObject = 0 as *mut PyObject;
    let mut pyCtxt: xmlSchemaValidCtxtPyCtxtPtr = 0 as *mut xmlSchemaValidCtxtPyCtxt;
    pyCtxt = ctx as xmlSchemaValidCtxtPyCtxtPtr;
    list = PyTuple_New(2 as libc::c_int as Py_ssize_t);
    PyTuple_SetItem(list, 0 as libc::c_int as Py_ssize_t, libxml_charPtrWrap(str));
    PyTuple_SetItem(list, 1 as libc::c_int as Py_ssize_t, (*pyCtxt).arg);
    if !((*pyCtxt).arg).is_null() {
        let ref mut fresh201 = (*(*pyCtxt).arg).ob_refcnt;
        *fresh201 += 1;
    }
    result = PyEval_CallObjectWithKeywords(
        (*pyCtxt).error,
        list,
        0 as *mut libc::c_void as *mut PyObject,
    );
    if result.is_null() {
        PyErr_Print();
    }
    if !list.is_null() {
        let ref mut fresh202 = (*list).ob_refcnt;
        *fresh202 -= 1;
        if !(*fresh202 != 0 as libc::c_int as libc::c_long) {
            (Some(((*(*list).ob_type).tp_dealloc).expect("non-null function pointer")))
                .expect("non-null function pointer")(list);
        }
    }
    if !result.is_null() {
        let ref mut fresh203 = (*result).ob_refcnt;
        *fresh203 -= 1;
        if !(*fresh203 != 0 as libc::c_int as libc::c_long) {
            (Some(((*(*result).ob_type).tp_dealloc).expect("non-null function pointer")))
                .expect("non-null function pointer")(result);
        }
    }
}
unsafe extern "C" fn libxml_xmlSchemaValidityGenericWarningFuncHandler(
    mut ctx: *mut libc::c_void,
    mut str: *mut libc::c_char,
) {
    let mut list: *mut PyObject = 0 as *mut PyObject;
    let mut result: *mut PyObject = 0 as *mut PyObject;
    let mut pyCtxt: xmlSchemaValidCtxtPyCtxtPtr = 0 as *mut xmlSchemaValidCtxtPyCtxt;
    pyCtxt = ctx as xmlSchemaValidCtxtPyCtxtPtr;
    list = PyTuple_New(2 as libc::c_int as Py_ssize_t);
    PyTuple_SetItem(list, 0 as libc::c_int as Py_ssize_t, libxml_charPtrWrap(str));
    PyTuple_SetItem(list, 1 as libc::c_int as Py_ssize_t, (*pyCtxt).arg);
    if !((*pyCtxt).arg).is_null() {
        let ref mut fresh204 = (*(*pyCtxt).arg).ob_refcnt;
        *fresh204 += 1;
    }
    result = PyEval_CallObjectWithKeywords(
        (*pyCtxt).warn,
        list,
        0 as *mut libc::c_void as *mut PyObject,
    );
    if result.is_null() {
        PyErr_Print();
    }
    if !list.is_null() {
        let ref mut fresh205 = (*list).ob_refcnt;
        *fresh205 -= 1;
        if !(*fresh205 != 0 as libc::c_int as libc::c_long) {
            (Some(((*(*list).ob_type).tp_dealloc).expect("non-null function pointer")))
                .expect("non-null function pointer")(list);
        }
    }
    if !result.is_null() {
        let ref mut fresh206 = (*result).ob_refcnt;
        *fresh206 -= 1;
        if !(*fresh206 != 0 as libc::c_int as libc::c_long) {
            (Some(((*(*result).ob_type).tp_dealloc).expect("non-null function pointer")))
                .expect("non-null function pointer")(result);
        }
    }
}
unsafe extern "C" fn libxml_xmlSchemaValidityErrorFunc(
    mut ctx: *mut libc::c_void,
    mut msg: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    libxml_xmlSchemaValidityGenericErrorFuncHandler(
        ctx,
        libxml_buildMessage(msg, ap.as_va_list()),
    );
}
unsafe extern "C" fn libxml_xmlSchemaValidityWarningFunc(
    mut ctx: *mut libc::c_void,
    mut msg: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    libxml_xmlSchemaValidityGenericWarningFuncHandler(
        ctx,
        libxml_buildMessage(msg, ap.as_va_list()),
    );
}
#[no_mangle]
pub unsafe extern "C" fn libxml_xmlSchemaSetValidErrors(
    mut self_0: *mut PyObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut py_retval: *mut PyObject = 0 as *mut PyObject;
    let mut pyobj_error: *mut PyObject = 0 as *mut PyObject;
    let mut pyobj_warn: *mut PyObject = 0 as *mut PyObject;
    let mut pyobj_ctx: *mut PyObject = 0 as *mut PyObject;
    let mut pyobj_arg: *mut PyObject = &mut _Py_NoneStruct;
    let mut ctxt: xmlSchemaValidCtxtPtr = 0 as *mut xmlSchemaValidCtxt;
    let mut pyCtxt: xmlSchemaValidCtxtPyCtxtPtr = 0 as *mut xmlSchemaValidCtxtPyCtxt;
    if _PyArg_ParseTuple_SizeT(
        args,
        b"OOO|O:xmlSchemaSetValidErrors\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        &mut pyobj_ctx as *mut *mut PyObject,
        &mut pyobj_error as *mut *mut PyObject,
        &mut pyobj_warn as *mut *mut PyObject,
        &mut pyobj_arg as *mut *mut PyObject,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    ctxt = if pyobj_ctx == &mut _Py_NoneStruct as *mut PyObject {
        0 as xmlSchemaValidCtxtPtr
    } else {
        (*(pyobj_ctx as *mut PySchemaValidCtxt_Object)).obj
    };
    if xmlSchemaGetValidErrors(
        ctxt,
        0 as *mut xmlSchemaValidityErrorFunc,
        0 as *mut xmlSchemaValidityWarningFunc,
        &mut pyCtxt as *mut xmlSchemaValidCtxtPyCtxtPtr as *mut *mut libc::c_void,
    ) == -(1 as libc::c_int)
    {
        py_retval = libxml_intWrap(-(1 as libc::c_int));
        return py_retval;
    }
    if pyCtxt.is_null() {
        pyCtxt = xmlMalloc
            .expect(
                "non-null function pointer",
            )(::std::mem::size_of::<xmlSchemaValidCtxtPyCtxt>() as libc::c_ulong)
            as xmlSchemaValidCtxtPyCtxtPtr;
        if pyCtxt.is_null() {
            py_retval = libxml_intWrap(-(1 as libc::c_int));
            return py_retval;
        }
        memset(
            pyCtxt as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<xmlSchemaValidCtxtPyCtxt>() as libc::c_ulong,
        );
    }
    if !((*pyCtxt).error).is_null() {
        let ref mut fresh207 = (*(*pyCtxt).error).ob_refcnt;
        *fresh207 -= 1;
        if !(*fresh207 != 0 as libc::c_int as libc::c_long) {
            (Some(
                ((*(*(*pyCtxt).error).ob_type).tp_dealloc)
                    .expect("non-null function pointer"),
            ))
                .expect("non-null function pointer")((*pyCtxt).error);
        }
    }
    if !pyobj_error.is_null() {
        let ref mut fresh208 = (*pyobj_error).ob_refcnt;
        *fresh208 += 1;
    }
    let ref mut fresh209 = (*pyCtxt).error;
    *fresh209 = pyobj_error;
    if !((*pyCtxt).warn).is_null() {
        let ref mut fresh210 = (*(*pyCtxt).warn).ob_refcnt;
        *fresh210 -= 1;
        if !(*fresh210 != 0 as libc::c_int as libc::c_long) {
            (Some(
                ((*(*(*pyCtxt).warn).ob_type).tp_dealloc)
                    .expect("non-null function pointer"),
            ))
                .expect("non-null function pointer")((*pyCtxt).warn);
        }
    }
    if !pyobj_warn.is_null() {
        let ref mut fresh211 = (*pyobj_warn).ob_refcnt;
        *fresh211 += 1;
    }
    let ref mut fresh212 = (*pyCtxt).warn;
    *fresh212 = pyobj_warn;
    if !((*pyCtxt).arg).is_null() {
        let ref mut fresh213 = (*(*pyCtxt).arg).ob_refcnt;
        *fresh213 -= 1;
        if !(*fresh213 != 0 as libc::c_int as libc::c_long) {
            (Some(
                ((*(*(*pyCtxt).arg).ob_type).tp_dealloc)
                    .expect("non-null function pointer"),
            ))
                .expect("non-null function pointer")((*pyCtxt).arg);
        }
    }
    if !pyobj_arg.is_null() {
        let ref mut fresh214 = (*pyobj_arg).ob_refcnt;
        *fresh214 += 1;
    }
    let ref mut fresh215 = (*pyCtxt).arg;
    *fresh215 = pyobj_arg;
    xmlSchemaSetValidErrors(
        ctxt,
        Some(
            libxml_xmlSchemaValidityErrorFunc
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                    ...
                ) -> (),
        ),
        Some(
            libxml_xmlSchemaValidityWarningFunc
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                    ...
                ) -> (),
        ),
        pyCtxt as *mut libc::c_void,
    );
    py_retval = libxml_intWrap(1 as libc::c_int);
    return py_retval;
}
unsafe extern "C" fn libxml_xmlSchemaFreeValidCtxt(
    mut self_0: *mut PyObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut ctxt: xmlSchemaValidCtxtPtr = 0 as *mut xmlSchemaValidCtxt;
    let mut pyCtxt: xmlSchemaValidCtxtPyCtxtPtr = 0 as *mut xmlSchemaValidCtxtPyCtxt;
    let mut pyobj_ctxt: *mut PyObject = 0 as *mut PyObject;
    if _PyArg_ParseTuple_SizeT(
        args,
        b"O:xmlSchemaFreeValidCtxt\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        &mut pyobj_ctxt as *mut *mut PyObject,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    ctxt = if pyobj_ctxt == &mut _Py_NoneStruct as *mut PyObject {
        0 as xmlSchemaValidCtxtPtr
    } else {
        (*(pyobj_ctxt as *mut PySchemaValidCtxt_Object)).obj
    };
    if xmlSchemaGetValidErrors(
        ctxt,
        0 as *mut xmlSchemaValidityErrorFunc,
        0 as *mut xmlSchemaValidityWarningFunc,
        &mut pyCtxt as *mut xmlSchemaValidCtxtPyCtxtPtr as *mut *mut libc::c_void,
    ) == 0 as libc::c_int
    {
        if !pyCtxt.is_null() {
            if !((*pyCtxt).error).is_null() {
                let ref mut fresh216 = (*(*pyCtxt).error).ob_refcnt;
                *fresh216 -= 1;
                if !(*fresh216 != 0 as libc::c_int as libc::c_long) {
                    (Some(
                        ((*(*(*pyCtxt).error).ob_type).tp_dealloc)
                            .expect("non-null function pointer"),
                    ))
                        .expect("non-null function pointer")((*pyCtxt).error);
                }
            }
            if !((*pyCtxt).warn).is_null() {
                let ref mut fresh217 = (*(*pyCtxt).warn).ob_refcnt;
                *fresh217 -= 1;
                if !(*fresh217 != 0 as libc::c_int as libc::c_long) {
                    (Some(
                        ((*(*(*pyCtxt).warn).ob_type).tp_dealloc)
                            .expect("non-null function pointer"),
                    ))
                        .expect("non-null function pointer")((*pyCtxt).warn);
                }
            }
            if !((*pyCtxt).arg).is_null() {
                let ref mut fresh218 = (*(*pyCtxt).arg).ob_refcnt;
                *fresh218 -= 1;
                if !(*fresh218 != 0 as libc::c_int as libc::c_long) {
                    (Some(
                        ((*(*(*pyCtxt).arg).ob_type).tp_dealloc)
                            .expect("non-null function pointer"),
                    ))
                        .expect("non-null function pointer")((*pyCtxt).arg);
                }
            }
            xmlFree.expect("non-null function pointer")(pyCtxt as *mut libc::c_void);
        }
    }
    xmlSchemaFreeValidCtxt(ctxt);
    let ref mut fresh219 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
    *fresh219 += 1;
    return &mut _Py_NoneStruct;
}
unsafe extern "C" fn PyxmlNodeSet_Convert(
    mut py_nodeset: *mut PyObject,
    mut result: *mut xmlNodeSetPtr,
) -> libc::c_int {
    let mut nodeSet: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    let mut is_tuple: libc::c_int = 0 as libc::c_int;
    if (*(*py_nodeset).ob_type).tp_flags & (1 as libc::c_long) << 26 as libc::c_int
        != 0 as libc::c_int as libc::c_long
    {
        is_tuple = 1 as libc::c_int;
    } else if (*(*py_nodeset).ob_type).tp_flags
            & (1 as libc::c_long) << 25 as libc::c_int
            != 0 as libc::c_int as libc::c_long
        {
        is_tuple = 0 as libc::c_int;
    } else if py_nodeset == &mut _Py_NoneStruct as *mut PyObject {
        *result = 0 as xmlNodeSetPtr;
        return 0 as libc::c_int;
    } else {
        PyErr_SetString(
            PyExc_TypeError,
            b"must be a tuple or list of nodes.\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    nodeSet = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::std::mem::size_of::<xmlNodeSet>() as libc::c_ulong) as xmlNodeSetPtr;
    if nodeSet.is_null() {
        PyErr_SetString(PyExc_MemoryError, b"\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    (*nodeSet).nodeNr = 0 as libc::c_int;
    (*nodeSet)
        .nodeMax = (if is_tuple != 0 {
        (*(py_nodeset as *mut PyVarObject)).ob_size
    } else {
        (*(py_nodeset as *mut PyVarObject)).ob_size
    }) as libc::c_int;
    let ref mut fresh220 = (*nodeSet).nodeTab;
    *fresh220 = xmlMalloc
        .expect(
            "non-null function pointer",
        )(
        ((*nodeSet).nodeMax as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<xmlNodePtr>() as libc::c_ulong),
    ) as *mut xmlNodePtr;
    if ((*nodeSet).nodeTab).is_null() {
        xmlFree.expect("non-null function pointer")(nodeSet as *mut libc::c_void);
        PyErr_SetString(PyExc_MemoryError, b"\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    memset(
        (*nodeSet).nodeTab as *mut libc::c_void,
        0 as libc::c_int,
        ((*nodeSet).nodeMax as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<xmlNodePtr>() as libc::c_ulong),
    );
    let mut idx: libc::c_int = 0;
    idx = 0 as libc::c_int;
    while idx < (*nodeSet).nodeMax {
        let mut pynode: xmlNodePtr = if (if is_tuple != 0 {
            *((*(py_nodeset as *mut PyTupleObject)).ob_item)
                .as_mut_ptr()
                .offset(idx as isize)
        } else {
            *((*(py_nodeset as *mut PyListObject)).ob_item).offset(idx as isize)
        }) == &mut _Py_NoneStruct as *mut PyObject
        {
            0 as xmlNodePtr
        } else {
            (*((if is_tuple != 0 {
                *((*(py_nodeset as *mut PyTupleObject)).ob_item)
                    .as_mut_ptr()
                    .offset(idx as isize)
            } else {
                *((*(py_nodeset as *mut PyListObject)).ob_item).offset(idx as isize)
            }) as *mut PyxmlNode_Object))
                .obj
        };
        if !pynode.is_null() {
            let ref mut fresh221 = (*nodeSet).nodeNr;
            let fresh222 = *fresh221;
            *fresh221 = *fresh221 + 1;
            let ref mut fresh223 = *((*nodeSet).nodeTab).offset(fresh222 as isize);
            *fresh223 = pynode;
        }
        idx += 1;
    }
    *result = nodeSet;
    return 0 as libc::c_int;
}
unsafe extern "C" fn PystringSet_Convert(
    mut py_strings: *mut PyObject,
    mut result: *mut *mut *mut xmlChar,
) -> libc::c_int {
    let mut strings: *mut *mut xmlChar = 0 as *mut *mut xmlChar;
    let mut is_tuple: libc::c_int = 0 as libc::c_int;
    let mut count: libc::c_int = 0;
    let mut init_index: libc::c_int = 0 as libc::c_int;
    if (*(*py_strings).ob_type).tp_flags & (1 as libc::c_long) << 26 as libc::c_int
        != 0 as libc::c_int as libc::c_long
    {
        is_tuple = 1 as libc::c_int;
    } else if (*(*py_strings).ob_type).tp_flags
            & (1 as libc::c_long) << 25 as libc::c_int
            != 0 as libc::c_int as libc::c_long
        {
        is_tuple = 0 as libc::c_int;
    } else if py_strings == &mut _Py_NoneStruct as *mut PyObject {
        *result = 0 as *mut *mut xmlChar;
        return 0 as libc::c_int;
    } else {
        PyErr_SetString(
            PyExc_TypeError,
            b"must be a tuple or list of strings.\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    count = (if is_tuple != 0 {
        (*(py_strings as *mut PyVarObject)).ob_size
    } else {
        (*(py_strings as *mut PyVarObject)).ob_size
    }) as libc::c_int;
    strings = xmlMalloc
        .expect(
            "non-null function pointer",
        )(
        (::std::mem::size_of::<*mut xmlChar>() as libc::c_ulong)
            .wrapping_mul(count as libc::c_ulong),
    ) as *mut *mut xmlChar;
    if strings.is_null() {
        PyErr_SetString(PyExc_MemoryError, b"\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    memset(
        strings as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<*mut xmlChar>() as libc::c_ulong)
            .wrapping_mul(count as libc::c_ulong),
    );
    let mut idx: libc::c_int = 0;
    idx = 0 as libc::c_int;
    while idx < count {
        let mut s: *mut libc::c_char = PyString_AsString(
            if is_tuple != 0 {
                *((*(py_strings as *mut PyTupleObject)).ob_item)
                    .as_mut_ptr()
                    .offset(idx as isize)
            } else {
                *((*(py_strings as *mut PyListObject)).ob_item).offset(idx as isize)
            },
        );
        if !s.is_null() {
            let fresh224 = init_index;
            init_index = init_index + 1;
            let ref mut fresh225 = *strings.offset(fresh224 as isize);
            *fresh225 = s as *mut xmlChar;
        } else {
            xmlFree.expect("non-null function pointer")(strings as *mut libc::c_void);
            PyErr_SetString(
                PyExc_TypeError,
                b"must be a tuple or list of strings.\0" as *const u8
                    as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        idx += 1;
    }
    *result = strings;
    return 0 as libc::c_int;
}
unsafe extern "C" fn libxml_C14NDocDumpMemory(
    mut self_0: *mut PyObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut py_retval: *mut PyObject = 0 as *mut PyObject;
    let mut pyobj_doc: *mut PyObject = 0 as *mut PyObject;
    let mut pyobj_nodes: *mut PyObject = 0 as *mut PyObject;
    let mut exclusive: libc::c_int = 0;
    let mut pyobj_prefixes: *mut PyObject = 0 as *mut PyObject;
    let mut with_comments: libc::c_int = 0;
    let mut doc: xmlDocPtr = 0 as *mut xmlDoc;
    let mut nodes: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    let mut prefixes: *mut *mut xmlChar = 0 as *mut *mut xmlChar;
    let mut doc_txt: *mut xmlChar = 0 as *mut xmlChar;
    let mut result: libc::c_int = 0;
    if _PyArg_ParseTuple_SizeT(
        args,
        b"OOiOi:C14NDocDumpMemory\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        &mut pyobj_doc as *mut *mut PyObject,
        &mut pyobj_nodes as *mut *mut PyObject,
        &mut exclusive as *mut libc::c_int,
        &mut pyobj_prefixes as *mut *mut PyObject,
        &mut with_comments as *mut libc::c_int,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    doc = (if pyobj_doc == &mut _Py_NoneStruct as *mut PyObject {
        0 as xmlNodePtr
    } else {
        (*(pyobj_doc as *mut PyxmlNode_Object)).obj
    }) as xmlDocPtr;
    if doc.is_null() {
        PyErr_SetString(
            PyExc_TypeError,
            b"bad document.\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut PyObject;
    }
    result = PyxmlNodeSet_Convert(pyobj_nodes, &mut nodes);
    if result < 0 as libc::c_int {
        return 0 as *mut PyObject;
    }
    if exclusive != 0 {
        result = PystringSet_Convert(pyobj_prefixes, &mut prefixes);
        if result < 0 as libc::c_int {
            if !nodes.is_null() {
                xmlFree
                    .expect(
                        "non-null function pointer",
                    )((*nodes).nodeTab as *mut libc::c_void);
                xmlFree.expect("non-null function pointer")(nodes as *mut libc::c_void);
            }
            return 0 as *mut PyObject;
        }
    }
    result = xmlC14NDocDumpMemory(
        doc,
        nodes,
        exclusive,
        prefixes,
        with_comments,
        &mut doc_txt,
    );
    if !nodes.is_null() {
        xmlFree
            .expect("non-null function pointer")((*nodes).nodeTab as *mut libc::c_void);
        xmlFree.expect("non-null function pointer")(nodes as *mut libc::c_void);
    }
    if !prefixes.is_null() {
        let mut idx: *mut *mut xmlChar = prefixes;
        while !(*idx).is_null() {
            let fresh226 = idx;
            idx = idx.offset(1);
            xmlFree.expect("non-null function pointer")(*fresh226 as *mut libc::c_void);
        }
        xmlFree.expect("non-null function pointer")(prefixes as *mut libc::c_void);
    }
    if result < 0 as libc::c_int {
        PyErr_SetString(
            PyExc_Exception,
            b"libxml2 xmlC14NDocDumpMemory failure.\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as *mut PyObject;
    } else {
        py_retval = PyString_FromStringAndSize(
            doc_txt as *const libc::c_char,
            result as Py_ssize_t,
        );
        xmlFree.expect("non-null function pointer")(doc_txt as *mut libc::c_void);
        return py_retval;
    };
}
unsafe extern "C" fn libxml_C14NDocSaveTo(
    mut self_0: *mut PyObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut pyobj_doc: *mut PyObject = 0 as *mut PyObject;
    let mut py_file: *mut PyObject = 0 as *mut PyObject;
    let mut pyobj_nodes: *mut PyObject = 0 as *mut PyObject;
    let mut exclusive: libc::c_int = 0;
    let mut pyobj_prefixes: *mut PyObject = 0 as *mut PyObject;
    let mut with_comments: libc::c_int = 0;
    let mut doc: xmlDocPtr = 0 as *mut xmlDoc;
    let mut nodes: xmlNodeSetPtr = 0 as *mut xmlNodeSet;
    let mut prefixes: *mut *mut xmlChar = 0 as *mut *mut xmlChar;
    let mut output: *mut FILE = 0 as *mut FILE;
    let mut buf: xmlOutputBufferPtr = 0 as *mut xmlOutputBuffer;
    let mut result: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    if _PyArg_ParseTuple_SizeT(
        args,
        b"OOiOiO:C14NDocSaveTo\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        &mut pyobj_doc as *mut *mut PyObject,
        &mut pyobj_nodes as *mut *mut PyObject,
        &mut exclusive as *mut libc::c_int,
        &mut pyobj_prefixes as *mut *mut PyObject,
        &mut with_comments as *mut libc::c_int,
        &mut py_file as *mut *mut PyObject,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    doc = (if pyobj_doc == &mut _Py_NoneStruct as *mut PyObject {
        0 as xmlNodePtr
    } else {
        (*(pyobj_doc as *mut PyxmlNode_Object)).obj
    }) as xmlDocPtr;
    if doc.is_null() {
        PyErr_SetString(
            PyExc_TypeError,
            b"bad document.\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut PyObject;
    }
    output = if py_file == &mut _Py_NoneStruct as *mut PyObject {
        0 as *mut FILE
    } else if (*py_file).ob_type == &mut PyFile_Type as *mut PyTypeObject
            || PyType_IsSubtype((*py_file).ob_type, &mut PyFile_Type) != 0
        {
        PyFile_AsFile(py_file)
    } else {
        stdout
    };
    if output.is_null() {
        PyErr_SetString(
            PyExc_TypeError,
            b"bad file.\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut PyObject;
    }
    buf = xmlOutputBufferCreateFile(output, 0 as xmlCharEncodingHandlerPtr);
    result = PyxmlNodeSet_Convert(pyobj_nodes, &mut nodes);
    if result < 0 as libc::c_int {
        xmlOutputBufferClose(buf);
        return 0 as *mut PyObject;
    }
    if exclusive != 0 {
        result = PystringSet_Convert(pyobj_prefixes, &mut prefixes);
        if result < 0 as libc::c_int {
            if !nodes.is_null() {
                xmlFree
                    .expect(
                        "non-null function pointer",
                    )((*nodes).nodeTab as *mut libc::c_void);
                xmlFree.expect("non-null function pointer")(nodes as *mut libc::c_void);
            }
            xmlOutputBufferClose(buf);
            return 0 as *mut PyObject;
        }
    }
    result = xmlC14NDocSaveTo(doc, nodes, exclusive, prefixes, with_comments, buf);
    if !nodes.is_null() {
        xmlFree
            .expect("non-null function pointer")((*nodes).nodeTab as *mut libc::c_void);
        xmlFree.expect("non-null function pointer")(nodes as *mut libc::c_void);
    }
    if !prefixes.is_null() {
        let mut idx: *mut *mut xmlChar = prefixes;
        while !(*idx).is_null() {
            let fresh227 = idx;
            idx = idx.offset(1);
            xmlFree.expect("non-null function pointer")(*fresh227 as *mut libc::c_void);
        }
        xmlFree.expect("non-null function pointer")(prefixes as *mut libc::c_void);
    }
    len = xmlOutputBufferClose(buf);
    if result < 0 as libc::c_int {
        PyErr_SetString(
            PyExc_Exception,
            b"libxml2 xmlC14NDocSaveTo failure.\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut PyObject;
    } else {
        return PyLong_FromLong(len as libc::c_long)
    };
}
unsafe extern "C" fn libxml_getObjDesc(
    mut self_0: *mut PyObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut obj: *mut PyObject = 0 as *mut PyObject;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    if _PyArg_ParseTuple_SizeT(
        args,
        b"O:getObjDesc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut obj as *mut *mut PyObject,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    str = PyCapsule_GetPointer(obj, PyCapsule_GetName(obj)) as *mut libc::c_char;
    return _Py_BuildValue_SizeT(
        b"s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        str,
    );
}
unsafe extern "C" fn libxml_compareNodesEqual(
    mut self_0: *mut PyObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut py_node1: *mut PyObject = 0 as *mut PyObject;
    let mut py_node2: *mut PyObject = 0 as *mut PyObject;
    let mut node1: xmlNodePtr = 0 as *mut xmlNode;
    let mut node2: xmlNodePtr = 0 as *mut xmlNode;
    if _PyArg_ParseTuple_SizeT(
        args,
        b"OO:compareNodesEqual\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        &mut py_node1 as *mut *mut PyObject,
        &mut py_node2 as *mut *mut PyObject,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    node1 = if py_node1 == &mut _Py_NoneStruct as *mut PyObject {
        0 as xmlNodePtr
    } else {
        (*(py_node1 as *mut PyxmlNode_Object)).obj
    };
    node2 = if py_node2 == &mut _Py_NoneStruct as *mut PyObject {
        0 as xmlNodePtr
    } else {
        (*(py_node2 as *mut PyxmlNode_Object)).obj
    };
    if node1 == node2 {
        return _Py_BuildValue_SizeT(
            b"i\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            1 as libc::c_int,
        )
    } else {
        return _Py_BuildValue_SizeT(
            b"i\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as libc::c_int,
        )
    };
}
unsafe extern "C" fn libxml_nodeHash(
    mut self_0: *mut PyObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut py_node1: *mut PyObject = 0 as *mut PyObject;
    let mut node1: xmlNodePtr = 0 as *mut xmlNode;
    if _PyArg_ParseTuple_SizeT(
        args,
        b"O:nodeHash\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut py_node1 as *mut *mut PyObject,
    ) == 0
    {
        return 0 as *mut PyObject;
    }
    node1 = if py_node1 == &mut _Py_NoneStruct as *mut PyObject {
        0 as xmlNodePtr
    } else {
        (*(py_node1 as *mut PyxmlNode_Object)).obj
    };
    return PyLong_FromVoidPtr(node1 as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn libxml_deprecationWarning(
    mut func: *const libc::c_char,
) -> libc::c_int {
    return PyErr_WarnEx(
        PyExc_PendingDeprecationWarning,
        func,
        1 as libc::c_int as Py_ssize_t,
    );
}
static mut libxmlMethods: [PyMethodDef; 926] = unsafe {
    [
        {
            let mut init = PyMethodDef {
                ml_name: b"htmlAutoCloseTag\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_htmlAutoCloseTag
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"htmlCreateFileParserCtxt\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_htmlCreateFileParserCtxt
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"htmlCreateMemoryParserCtxt\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_htmlCreateMemoryParserCtxt
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"htmlCreatePushParser\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_htmlCreatePushParser
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"htmlCtxtReadDoc\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_htmlCtxtReadDoc
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"htmlCtxtReadFd\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_htmlCtxtReadFd
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"htmlCtxtReadFile\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_htmlCtxtReadFile
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"htmlCtxtReadMemory\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_htmlCtxtReadMemory
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"htmlCtxtReset\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_htmlCtxtReset
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"htmlCtxtUseOptions\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_htmlCtxtUseOptions
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"htmlDefaultSAXHandlerInit\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_htmlDefaultSAXHandlerInit
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"htmlDocContentDumpFormatOutput\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_htmlDocContentDumpFormatOutput
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"htmlDocContentDumpOutput\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_htmlDocContentDumpOutput
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"htmlDocDump\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_htmlDocDump
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"htmlFreeParserCtxt\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_htmlFreeParserCtxt
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"htmlGetMetaEncoding\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_htmlGetMetaEncoding
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"htmlHandleOmittedElem\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_htmlHandleOmittedElem
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"htmlInitAutoClose\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_htmlInitAutoClose
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"htmlIsAutoClosed\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_htmlIsAutoClosed
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"htmlIsBooleanAttr\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_htmlIsBooleanAttr
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"htmlIsScriptAttribute\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_htmlIsScriptAttribute
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"htmlNewDoc\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_htmlNewDoc
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"htmlNewDocNoDtD\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_htmlNewDocNoDtD
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"htmlNewParserCtxt\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_htmlNewParserCtxt
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"htmlNodeDumpFile\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_htmlNodeDumpFile
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"htmlNodeDumpFileFormat\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_htmlNodeDumpFileFormat
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"htmlNodeDumpFormatOutput\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_htmlNodeDumpFormatOutput
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"htmlNodeDumpOutput\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_htmlNodeDumpOutput
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"htmlParseCharRef\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_htmlParseCharRef
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"htmlParseChunk\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_htmlParseChunk
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"htmlParseDoc\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_htmlParseDoc
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"htmlParseDocument\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_htmlParseDocument
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"htmlParseElement\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_htmlParseElement
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"htmlParseFile\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_htmlParseFile
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"htmlReadDoc\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_htmlReadDoc
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"htmlReadFd\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_htmlReadFd
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"htmlReadFile\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_htmlReadFile
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"htmlReadMemory\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_htmlReadMemory
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"htmlSAXParseFile\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_htmlSAXParseFile
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"htmlSaveFile\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_htmlSaveFile
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"htmlSaveFileEnc\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_htmlSaveFileEnc
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"htmlSaveFileFormat\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_htmlSaveFileFormat
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"htmlSetMetaEncoding\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_htmlSetMetaEncoding
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"namePop\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_namePop
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"namePush\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_namePush
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"nodePop\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_nodePop
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"nodePush\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_nodePush
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"valuePop\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_valuePop
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlACatalogAdd\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlACatalogAdd
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlACatalogDump\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlACatalogDump
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlACatalogRemove\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlACatalogRemove
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlACatalogResolve\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlACatalogResolve
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlACatalogResolvePublic\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlACatalogResolvePublic
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlACatalogResolveSystem\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlACatalogResolveSystem
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlACatalogResolveURI\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlACatalogResolveURI
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlAddChild\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlAddChild
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlAddChildList\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlAddChildList
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlAddDocEntity\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlAddDocEntity
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlAddDtdEntity\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlAddDtdEntity
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlAddEncodingAlias\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlAddEncodingAlias
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlAddNextSibling\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlAddNextSibling
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlAddPrevSibling\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlAddPrevSibling
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlAddSibling\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlAddSibling
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlBoolToText\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlBoolToText
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlBuildQName\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlBuildQName
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlBuildRelativeURI\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlBuildRelativeURI
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlBuildURI\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlBuildURI
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlByteConsumed\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlByteConsumed
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlCanonicPath\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlCanonicPath
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlCatalogAdd\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlCatalogAdd
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlCatalogCleanup\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlCatalogCleanup
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlCatalogConvert\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlCatalogConvert
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlCatalogDump\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlCatalogDump
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlCatalogGetPublic\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlCatalogGetPublic
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlCatalogGetSystem\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlCatalogGetSystem
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlCatalogIsEmpty\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlCatalogIsEmpty
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlCatalogRemove\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlCatalogRemove
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlCatalogResolve\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlCatalogResolve
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlCatalogResolvePublic\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlCatalogResolvePublic
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlCatalogResolveSystem\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlCatalogResolveSystem
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlCatalogResolveURI\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlCatalogResolveURI
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlCatalogSetDebug\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlCatalogSetDebug
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlCharStrdup\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlCharStrdup
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlCharStrndup\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlCharStrndup
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlCheckFilename\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlCheckFilename
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlCheckLanguageID\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlCheckLanguageID
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlCheckUTF8\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlCheckUTF8
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlCheckVersion\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlCheckVersion
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlCleanupCharEncodingHandlers\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlCleanupCharEncodingHandlers
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlCleanupEncodingAliases\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlCleanupEncodingAliases
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlCleanupGlobals\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlCleanupGlobals
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlCleanupInputCallbacks\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlCleanupInputCallbacks
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlCleanupOutputCallbacks\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlCleanupOutputCallbacks
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlCleanupPredefinedEntities\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlCleanupPredefinedEntities
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlClearParserCtxt\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlClearParserCtxt
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlConvertSGMLCatalog\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlConvertSGMLCatalog
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlCopyChar\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlCopyChar
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlCopyCharMultiByte\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlCopyCharMultiByte
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlCopyDoc\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlCopyDoc
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlCopyDtd\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlCopyDtd
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlCopyError\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlCopyError
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlCopyNamespace\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlCopyNamespace
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlCopyNamespaceList\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlCopyNamespaceList
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlCopyNode\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlCopyNode
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlCopyNodeList\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlCopyNodeList
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlCopyProp\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlCopyProp
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlCopyPropList\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlCopyPropList
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlCreateDocParserCtxt\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlCreateDocParserCtxt
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlCreateEntityParserCtxt\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlCreateEntityParserCtxt
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlCreateFileParserCtxt\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlCreateFileParserCtxt
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlCreateInputBuffer\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlCreateInputBuffer
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlCreateIntSubset\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlCreateIntSubset
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlCreateMemoryParserCtxt\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlCreateMemoryParserCtxt
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlCreateOutputBuffer\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlCreateOutputBuffer
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlCreatePushParser\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlCreatePushParser
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlCreateURI\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlCreateURI
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlCreateURLParserCtxt\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlCreateURLParserCtxt
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlCtxtReadDoc\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlCtxtReadDoc
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlCtxtReadFd\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlCtxtReadFd
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlCtxtReadFile\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlCtxtReadFile
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlCtxtReadMemory\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlCtxtReadMemory
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlCtxtReset\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlCtxtReset
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlCtxtResetPush\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlCtxtResetPush
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlCtxtUseOptions\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlCtxtUseOptions
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlDebugCheckDocument\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlDebugCheckDocument
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlDebugDumpAttr\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlDebugDumpAttr
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlDebugDumpAttrList\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlDebugDumpAttrList
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlDebugDumpDTD\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlDebugDumpDTD
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlDebugDumpDocument\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlDebugDumpDocument
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlDebugDumpDocumentHead\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlDebugDumpDocumentHead
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlDebugDumpEntities\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlDebugDumpEntities
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlDebugDumpNode\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlDebugDumpNode
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlDebugDumpNodeList\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlDebugDumpNodeList
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlDebugDumpOneNode\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlDebugDumpOneNode
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlDebugDumpString\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlDebugDumpString
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlDebugMemory\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlDebugMemory
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlDecodeEntities\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlDecodeEntities
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlDefaultSAXHandlerInit\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlDefaultSAXHandlerInit
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlDelEncodingAlias\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlDelEncodingAlias
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlDictCleanup\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlDictCleanup
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlDocCopyNode\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlDocCopyNode
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlDocCopyNodeList\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlDocCopyNodeList
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlDocDump\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlDocDump
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlDocFormatDump\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlDocFormatDump
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlDocGetRootElement\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlDocGetRootElement
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlDocSetRootElement\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlDocSetRootElement
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlDumpMemory\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlDumpMemory
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlElemDump\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlElemDump
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlEncodeEntities\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlEncodeEntities
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlEncodeEntitiesReentrant\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlEncodeEntitiesReentrant
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlEncodeSpecialChars\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlEncodeSpecialChars
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlErrorGetCode\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlErrorGetCode
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlErrorGetDomain\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlErrorGetDomain
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlErrorGetFile\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlErrorGetFile
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlErrorGetLevel\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlErrorGetLevel
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlErrorGetLine\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlErrorGetLine
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlErrorGetMessage\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlErrorGetMessage
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlFileMatch\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlFileMatch
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlFirstElementChild\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlFirstElementChild
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlFreeCatalog\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlFreeCatalog
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlFreeDoc\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlFreeDoc
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlFreeDtd\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlFreeDtd
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlFreeNode\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlFreeNode
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlFreeNodeList\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlFreeNodeList
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlFreeNs\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlFreeNs
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlFreeNsList\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlFreeNsList
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlFreeParserInputBuffer\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlFreeParserInputBuffer
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlFreeProp\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlFreeProp
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlFreePropList\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlFreePropList
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlFreeURI\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlFreeURI
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlGetCompressMode\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlGetCompressMode
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlGetDocCompressMode\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlGetDocCompressMode
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlGetDocEntity\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlGetDocEntity
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlGetDtdAttrDesc\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlGetDtdAttrDesc
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlGetDtdElementDesc\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlGetDtdElementDesc
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlGetDtdEntity\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlGetDtdEntity
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlGetDtdQAttrDesc\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlGetDtdQAttrDesc
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlGetDtdQElementDesc\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlGetDtdQElementDesc
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlGetEncodingAlias\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlGetEncodingAlias
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlGetID\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlGetID
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlGetIntSubset\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlGetIntSubset
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlGetLastChild\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlGetLastChild
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlGetLastError\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlGetLastError
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlGetLineNo\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlGetLineNo
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlGetNoNsProp\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlGetNoNsProp
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlGetNodePath\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlGetNodePath
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlGetNsProp\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlGetNsProp
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlGetParameterEntity\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlGetParameterEntity
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlGetPredefinedEntity\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlGetPredefinedEntity
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlGetProp\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlGetProp
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlHandleEntity\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlHandleEntity
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlHasNsProp\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlHasNsProp
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlHasProp\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlHasProp
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlIOFTPMatch\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlIOFTPMatch
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlIOHTTPMatch\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlIOHTTPMatch
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlInitCharEncodingHandlers\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlInitCharEncodingHandlers
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlInitGlobals\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlInitGlobals
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlInitParser\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlInitParser
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlInitParserCtxt\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlInitParserCtxt
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlInitializeCatalog\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlInitializeCatalog
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlInitializeDict\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlInitializeDict
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlInitializePredefinedEntities\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlInitializePredefinedEntities
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlIsBaseChar\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlIsBaseChar
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlIsBlank\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlIsBlank
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlIsBlankNode\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlIsBlankNode
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlIsChar\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlIsChar
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlIsCombining\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlIsCombining
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlIsDigit\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlIsDigit
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlIsExtender\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlIsExtender
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlIsID\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlIsID
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlIsIdeographic\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlIsIdeographic
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlIsLetter\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlIsLetter
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlIsMixedElement\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlIsMixedElement
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlIsPubidChar\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlIsPubidChar
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlIsRef\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlIsRef
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlIsXHTML\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlIsXHTML
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlKeepBlanksDefault\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlKeepBlanksDefault
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlLastElementChild\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlLastElementChild
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlLineNumbersDefault\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlLineNumbersDefault
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlLoadACatalog\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlLoadACatalog
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlLoadCatalog\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlLoadCatalog
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlLoadCatalogs\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlLoadCatalogs
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlLoadSGMLSuperCatalog\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlLoadSGMLSuperCatalog
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlLsCountNode\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlLsCountNode
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlLsOneNode\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlLsOneNode
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlMemoryUsed\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlMemoryUsed
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNamespaceParseNCName\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNamespaceParseNCName
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNamespaceParseNSDef\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNamespaceParseNSDef
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNanoFTPCleanup\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNanoFTPCleanup
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNanoFTPInit\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNanoFTPInit
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNanoFTPProxy\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNanoFTPProxy
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNanoFTPScanProxy\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNanoFTPScanProxy
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNanoHTTPCleanup\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNanoHTTPCleanup
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNanoHTTPInit\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNanoHTTPInit
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNanoHTTPScanProxy\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNanoHTTPScanProxy
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNewCDataBlock\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNewCDataBlock
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNewCatalog\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNewCatalog
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNewCharRef\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNewCharRef
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNewChild\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNewChild
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNewComment\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNewComment
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNewDoc\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNewDoc
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNewDocComment\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNewDocComment
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNewDocFragment\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNewDocFragment
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNewDocNode\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNewDocNode
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNewDocNodeEatName\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNewDocNodeEatName
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNewDocPI\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNewDocPI
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNewDocProp\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNewDocProp
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNewDocRawNode\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNewDocRawNode
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNewDocText\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNewDocText
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNewDocTextLen\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNewDocTextLen
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNewDtd\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNewDtd
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNewEntity\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNewEntity
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNewGlobalNs\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNewGlobalNs
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNewNode\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNewNode
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNewNodeEatName\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNewNodeEatName
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNewNs\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNewNs
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNewNsProp\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNewNsProp
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNewNsPropEatName\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNewNsPropEatName
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNewPI\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNewPI
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNewParserCtxt\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNewParserCtxt
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNewProp\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNewProp
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNewReference\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNewReference
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNewText\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNewText
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNewTextChild\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNewTextChild
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNewTextLen\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNewTextLen
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNewTextReader\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNewTextReader
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNewTextReaderFilename\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNewTextReaderFilename
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNewValidCtxt\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNewValidCtxt
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNextChar\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNextChar
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNextElementSibling\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNextElementSibling
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNodeAddContent\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNodeAddContent
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNodeAddContentLen\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNodeAddContentLen
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNodeDumpOutput\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNodeDumpOutput
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNodeGetBase\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNodeGetBase
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNodeGetContent\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNodeGetContent
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNodeGetLang\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNodeGetLang
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNodeGetNs\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNodeGetNs
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNodeGetNsDefs\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNodeGetNsDefs
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNodeGetSpacePreserve\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNodeGetSpacePreserve
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNodeIsText\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNodeIsText
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNodeListGetRawString\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNodeListGetRawString
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNodeListGetString\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNodeListGetString
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNodeSetBase\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNodeSetBase
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNodeSetContent\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNodeSetContent
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNodeSetContentLen\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNodeSetContentLen
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNodeSetLang\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNodeSetLang
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNodeSetName\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNodeSetName
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNodeSetSpacePreserve\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNodeSetSpacePreserve
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNormalizeURIPath\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNormalizeURIPath
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNormalizeWindowsPath\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNormalizeWindowsPath
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlOutputBufferGetContent\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlOutputBufferGetContent
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlOutputBufferWrite\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlOutputBufferWrite
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlOutputBufferWriteString\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlOutputBufferWriteString
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParseAttValue\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParseAttValue
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParseAttributeListDecl\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParseAttributeListDecl
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParseCDSect\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParseCDSect
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParseCatalogFile\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParseCatalogFile
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParseCharData\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParseCharData
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParseCharRef\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParseCharRef
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParseChunk\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParseChunk
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParseComment\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParseComment
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParseContent\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParseContent
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParseDTD\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParseDTD
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParseDoc\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParseDoc
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParseDocTypeDecl\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParseDocTypeDecl
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParseDocument\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParseDocument
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParseElement\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParseElement
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParseElementDecl\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParseElementDecl
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParseEncName\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParseEncName
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParseEncodingDecl\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParseEncodingDecl
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParseEndTag\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParseEndTag
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParseEntity\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParseEntity
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParseEntityDecl\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParseEntityDecl
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParseEntityRef\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParseEntityRef
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParseExtParsedEnt\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParseExtParsedEnt
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParseExternalSubset\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParseExternalSubset
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParseFile\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParseFile
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParseMarkupDecl\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParseMarkupDecl
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParseMemory\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParseMemory
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParseMisc\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParseMisc
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParseName\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParseName
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParseNamespace\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParseNamespace
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParseNmtoken\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParseNmtoken
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParseNotationDecl\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParseNotationDecl
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParsePEReference\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParsePEReference
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParsePI\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParsePI
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParsePITarget\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParsePITarget
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParsePubidLiteral\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParsePubidLiteral
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParseQuotedString\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParseQuotedString
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParseReference\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParseReference
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParseSDDecl\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParseSDDecl
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParseStartTag\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParseStartTag
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParseSystemLiteral\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParseSystemLiteral
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParseTextDecl\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParseTextDecl
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParseURI\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParseURI
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParseURIRaw\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParseURIRaw
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParseURIReference\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParseURIReference
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParseVersionInfo\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParseVersionInfo
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParseVersionNum\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParseVersionNum
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParseXMLDecl\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParseXMLDecl
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParserGetDirectory\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParserGetDirectory
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParserGetDoc\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParserGetDoc
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParserGetIsValid\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParserGetIsValid
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParserGetWellFormed\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParserGetWellFormed
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParserHandlePEReference\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParserHandlePEReference
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParserHandleReference\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParserHandleReference
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParserInputBufferGrow\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParserInputBufferGrow
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParserInputBufferPush\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParserInputBufferPush
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParserInputBufferRead\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParserInputBufferRead
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParserSetLineNumbers\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParserSetLineNumbers
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParserSetLoadSubset\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParserSetLoadSubset
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParserSetPedantic\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParserSetPedantic
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParserSetReplaceEntities\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParserSetReplaceEntities
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParserSetValidate\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParserSetValidate
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlPathToURI\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlPathToURI
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlPedanticParserDefault\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlPedanticParserDefault
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlPopInput\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlPopInput
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlPopOutputCallbacks\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlPopOutputCallbacks
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlPreviousElementSibling\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlPreviousElementSibling
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlPrintURI\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlPrintURI
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlPythonCleanupParser\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlPythonCleanupParser
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlReadDoc\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlReadDoc
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlReadFd\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlReadFd
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlReadFile\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlReadFile
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlReadMemory\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlReadMemory
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlReaderForDoc\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlReaderForDoc
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlReaderForFd\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlReaderForFd
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlReaderForFile\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlReaderForFile
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlReaderForMemory\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlReaderForMemory
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlReaderNewDoc\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlReaderNewDoc
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlReaderNewFd\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlReaderNewFd
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlReaderNewFile\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlReaderNewFile
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlReaderNewMemory\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlReaderNewMemory
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlReaderNewWalker\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlReaderNewWalker
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlReaderWalker\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlReaderWalker
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlReconciliateNs\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlReconciliateNs
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlRecoverDoc\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlRecoverDoc
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlRecoverFile\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlRecoverFile
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlRecoverMemory\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlRecoverMemory
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlRegFreeRegexp\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlRegFreeRegexp
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlRegexpCompile\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlRegexpCompile
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlRegexpExec\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlRegexpExec
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlRegexpIsDeterminist\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlRegexpIsDeterminist
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlRegexpPrint\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlRegexpPrint
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlRegisterDefaultInputCallbacks\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlRegisterDefaultInputCallbacks
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlRegisterDefaultOutputCallbacks\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlRegisterDefaultOutputCallbacks
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlRegisterHTTPPostCallbacks\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlRegisterHTTPPostCallbacks
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlRegisterXPathFunction\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlRegisterXPathFunction
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlRelaxNGCleanupTypes\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlRelaxNGCleanupTypes
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlRelaxNGDump\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlRelaxNGDump
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlRelaxNGDumpTree\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlRelaxNGDumpTree
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlRelaxNGFree\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlRelaxNGFree
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlRelaxNGFreeParserCtxt\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlRelaxNGFreeParserCtxt
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlRelaxNGInitTypes\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlRelaxNGInitTypes
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlRelaxNGNewDocParserCtxt\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlRelaxNGNewDocParserCtxt
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlRelaxNGNewMemParserCtxt\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlRelaxNGNewMemParserCtxt
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlRelaxNGNewParserCtxt\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlRelaxNGNewParserCtxt
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlRelaxNGNewValidCtxt\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlRelaxNGNewValidCtxt
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlRelaxNGParse\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlRelaxNGParse
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlRelaxNGValidateDoc\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlRelaxNGValidateDoc
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlRelaxNGValidateFullElement\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlRelaxNGValidateFullElement
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlRelaxNGValidatePopElement\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlRelaxNGValidatePopElement
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlRelaxNGValidatePushCData\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlRelaxNGValidatePushCData
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlRelaxNGValidatePushElement\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlRelaxNGValidatePushElement
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlRelaxParserSetFlag\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlRelaxParserSetFlag
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlRemoveID\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlRemoveID
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlRemoveProp\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlRemoveProp
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlRemoveRef\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlRemoveRef
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlReplaceNode\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlReplaceNode
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlResetError\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlResetError
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlResetLastError\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlResetLastError
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlSAXDefaultVersion\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlSAXDefaultVersion
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlSAXParseFile\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlSAXParseFile
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlSaveFile\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlSaveFile
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlSaveFileEnc\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlSaveFileEnc
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlSaveFormatFile\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlSaveFormatFile
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlSaveFormatFileEnc\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlSaveFormatFileEnc
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlSaveUri\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlSaveUri
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlScanName\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlScanName
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlSchemaCleanupTypes\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlSchemaCleanupTypes
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlSchemaCollapseString\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlSchemaCollapseString
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlSchemaDump\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlSchemaDump
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlSchemaFree\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlSchemaFree
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlSchemaFreeParserCtxt\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlSchemaFreeParserCtxt
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlSchemaInitTypes\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlSchemaInitTypes
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlSchemaIsValid\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlSchemaIsValid
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlSchemaNewDocParserCtxt\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlSchemaNewDocParserCtxt
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlSchemaNewMemParserCtxt\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlSchemaNewMemParserCtxt
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlSchemaNewParserCtxt\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlSchemaNewParserCtxt
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlSchemaNewValidCtxt\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlSchemaNewValidCtxt
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlSchemaParse\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlSchemaParse
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlSchemaSetValidOptions\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlSchemaSetValidOptions
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlSchemaValidCtxtGetOptions\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlSchemaValidCtxtGetOptions
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlSchemaValidCtxtGetParserCtxt\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlSchemaValidCtxtGetParserCtxt
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlSchemaValidateDoc\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlSchemaValidateDoc
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlSchemaValidateFile\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlSchemaValidateFile
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlSchemaValidateOneElement\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlSchemaValidateOneElement
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlSchemaValidateSetFilename\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlSchemaValidateSetFilename
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlSchemaWhiteSpaceReplace\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlSchemaWhiteSpaceReplace
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlSearchNs\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlSearchNs
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlSearchNsByHref\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlSearchNsByHref
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlSetCompressMode\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlSetCompressMode
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlSetDocCompressMode\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlSetDocCompressMode
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlSetEntityLoader\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlSetEntityLoader
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlSetListDoc\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlSetListDoc
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlSetNs\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlSetNs
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlSetNsProp\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlSetNsProp
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlSetProp\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlSetProp
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlSetTreeDoc\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlSetTreeDoc
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlSetupParserForBuffer\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlSetupParserForBuffer
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlShellPrintNode\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlShellPrintNode
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlShellPrintXPathError\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlShellPrintXPathError
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlSkipBlankChars\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlSkipBlankChars
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlStopParser\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlStopParser
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlStrEqual\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlStrEqual
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlStrQEqual\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlStrQEqual
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlStrcasecmp\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlStrcasecmp
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlStrcasestr\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlStrcasestr
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlStrcat\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlStrcat
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlStrchr\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlStrchr
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlStrcmp\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlStrcmp
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlStrdup\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlStrdup
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlStringDecodeEntities\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlStringDecodeEntities
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlStringGetNodeList\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlStringGetNodeList
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlStringLenDecodeEntities\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlStringLenDecodeEntities
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlStringLenGetNodeList\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlStringLenGetNodeList
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlStrlen\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlStrlen
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlStrncasecmp\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlStrncasecmp
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlStrncat\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlStrncat
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlStrncatNew\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlStrncatNew
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlStrncmp\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlStrncmp
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlStrndup\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlStrndup
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlStrstr\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlStrstr
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlStrsub\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlStrsub
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlSubstituteEntitiesDefault\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlSubstituteEntitiesDefault
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextConcat\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextConcat
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextMerge\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextMerge
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderAttributeCount\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderAttributeCount
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderByteConsumed\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderByteConsumed
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderClose\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderClose
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderConstBaseUri\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderConstBaseUri
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderConstEncoding\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderConstEncoding
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderConstLocalName\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderConstLocalName
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderConstName\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderConstName
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderConstNamespaceUri\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderConstNamespaceUri
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderConstPrefix\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderConstPrefix
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderConstString\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderConstString
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderConstValue\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderConstValue
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderConstXmlLang\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderConstXmlLang
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderConstXmlVersion\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderConstXmlVersion
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderCurrentDoc\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderCurrentDoc
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderCurrentNode\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderCurrentNode
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderDepth\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderDepth
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderExpand\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderExpand
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderGetAttribute\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderGetAttribute
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderGetAttributeNo\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderGetAttributeNo
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderGetAttributeNs\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderGetAttributeNs
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderGetParserColumnNumber\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderGetParserColumnNumber
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderGetParserLineNumber\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderGetParserLineNumber
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderGetParserProp\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderGetParserProp
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderGetRemainder\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderGetRemainder
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderHasAttributes\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderHasAttributes
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderHasValue\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderHasValue
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderIsDefault\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderIsDefault
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderIsEmptyElement\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderIsEmptyElement
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderIsNamespaceDecl\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderIsNamespaceDecl
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderIsValid\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderIsValid
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderLocatorBaseURI\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderLocatorBaseURI
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderLocatorLineNumber\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderLocatorLineNumber
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderLookupNamespace\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderLookupNamespace
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderMoveToAttribute\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderMoveToAttribute
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderMoveToAttributeNo\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderMoveToAttributeNo
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderMoveToAttributeNs\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderMoveToAttributeNs
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderMoveToElement\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderMoveToElement
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderMoveToFirstAttribute\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderMoveToFirstAttribute
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderMoveToNextAttribute\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderMoveToNextAttribute
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderNext\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderNext
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderNextSibling\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderNextSibling
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderNodeType\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderNodeType
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderNormalization\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderNormalization
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderPreserve\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderPreserve
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderQuoteChar\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderQuoteChar
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderRead\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderRead
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderReadAttributeValue\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderReadAttributeValue
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderReadInnerXml\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderReadInnerXml
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderReadOuterXml\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderReadOuterXml
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderReadState\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderReadState
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderReadString\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderReadString
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderRelaxNGSetSchema\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderRelaxNGSetSchema
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderRelaxNGValidate\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderRelaxNGValidate
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderRelaxNGValidateCtxt\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderRelaxNGValidateCtxt
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderSchemaValidate\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderSchemaValidate
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderSchemaValidateCtxt\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderSchemaValidateCtxt
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderSetParserProp\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderSetParserProp
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderSetSchema\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderSetSchema
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderSetup\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderSetup
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderStandalone\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderStandalone
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlThrDefDefaultBufferSize\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlThrDefDefaultBufferSize
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlThrDefDoValidityCheckingDefaultValue\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlThrDefDoValidityCheckingDefaultValue
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlThrDefGetWarningsDefaultValue\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlThrDefGetWarningsDefaultValue
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlThrDefIndentTreeOutput\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlThrDefIndentTreeOutput
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlThrDefKeepBlanksDefaultValue\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlThrDefKeepBlanksDefaultValue
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlThrDefLineNumbersDefaultValue\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlThrDefLineNumbersDefaultValue
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlThrDefLoadExtDtdDefaultValue\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlThrDefLoadExtDtdDefaultValue
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlThrDefParserDebugEntities\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlThrDefParserDebugEntities
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlThrDefPedanticParserDefaultValue\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlThrDefPedanticParserDefaultValue
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlThrDefSaveNoEmptyTags\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlThrDefSaveNoEmptyTags
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlThrDefSubstituteEntitiesDefaultValue\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlThrDefSubstituteEntitiesDefaultValue
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlThrDefTreeIndentString\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlThrDefTreeIndentString
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsAegeanNumbers\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsAegeanNumbers
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsAlphabeticPresentationForms\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsAlphabeticPresentationForms
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsArabic\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsArabic
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsArabicPresentationFormsA\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsArabicPresentationFormsA
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsArabicPresentationFormsB\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsArabicPresentationFormsB
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsArmenian\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsArmenian
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsArrows\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsArrows
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsBasicLatin\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsBasicLatin
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsBengali\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsBengali
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsBlock\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsBlock
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsBlockElements\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsBlockElements
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsBopomofo\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsBopomofo
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsBopomofoExtended\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsBopomofoExtended
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsBoxDrawing\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsBoxDrawing
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsBraillePatterns\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsBraillePatterns
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsBuhid\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsBuhid
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsByzantineMusicalSymbols\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsByzantineMusicalSymbols
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsCJKCompatibility\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsCJKCompatibility
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsCJKCompatibilityForms\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsCJKCompatibilityForms
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsCJKCompatibilityIdeographs\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsCJKCompatibilityIdeographs
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsCJKCompatibilityIdeographsSupplement\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsCJKCompatibilityIdeographsSupplement
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsCJKRadicalsSupplement\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsCJKRadicalsSupplement
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsCJKSymbolsandPunctuation\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsCJKSymbolsandPunctuation
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsCJKUnifiedIdeographs\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsCJKUnifiedIdeographs
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsCJKUnifiedIdeographsExtensionA\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsCJKUnifiedIdeographsExtensionA
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsCJKUnifiedIdeographsExtensionB\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsCJKUnifiedIdeographsExtensionB
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsCat\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsCat
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsCatC\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsCatC
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsCatCc\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsCatCc
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsCatCf\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsCatCf
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsCatCo\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsCatCo
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsCatCs\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsCatCs
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsCatL\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsCatL
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsCatLl\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsCatLl
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsCatLm\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsCatLm
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsCatLo\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsCatLo
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsCatLt\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsCatLt
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsCatLu\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsCatLu
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsCatM\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsCatM
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsCatMc\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsCatMc
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsCatMe\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsCatMe
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsCatMn\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsCatMn
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsCatN\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsCatN
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsCatNd\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsCatNd
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsCatNl\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsCatNl
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsCatNo\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsCatNo
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsCatP\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsCatP
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsCatPc\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsCatPc
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsCatPd\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsCatPd
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsCatPe\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsCatPe
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsCatPf\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsCatPf
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsCatPi\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsCatPi
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsCatPo\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsCatPo
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsCatPs\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsCatPs
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsCatS\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsCatS
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsCatSc\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsCatSc
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsCatSk\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsCatSk
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsCatSm\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsCatSm
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsCatSo\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsCatSo
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsCatZ\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsCatZ
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsCatZl\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsCatZl
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsCatZp\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsCatZp
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsCatZs\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsCatZs
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsCherokee\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsCherokee
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsCombiningDiacriticalMarks\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsCombiningDiacriticalMarks
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsCombiningDiacriticalMarksforSymbols\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsCombiningDiacriticalMarksforSymbols
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsCombiningHalfMarks\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsCombiningHalfMarks
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsCombiningMarksforSymbols\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsCombiningMarksforSymbols
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsControlPictures\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsControlPictures
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsCurrencySymbols\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsCurrencySymbols
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsCypriotSyllabary\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsCypriotSyllabary
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsCyrillic\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsCyrillic
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsCyrillicSupplement\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsCyrillicSupplement
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsDeseret\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsDeseret
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsDevanagari\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsDevanagari
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsDingbats\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsDingbats
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsEnclosedAlphanumerics\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsEnclosedAlphanumerics
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsEnclosedCJKLettersandMonths\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsEnclosedCJKLettersandMonths
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsEthiopic\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsEthiopic
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsGeneralPunctuation\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsGeneralPunctuation
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsGeometricShapes\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsGeometricShapes
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsGeorgian\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsGeorgian
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsGothic\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsGothic
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsGreek\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsGreek
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsGreekExtended\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsGreekExtended
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsGreekandCoptic\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsGreekandCoptic
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsGujarati\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsGujarati
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsGurmukhi\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsGurmukhi
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsHalfwidthandFullwidthForms\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsHalfwidthandFullwidthForms
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsHangulCompatibilityJamo\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsHangulCompatibilityJamo
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsHangulJamo\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsHangulJamo
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsHangulSyllables\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsHangulSyllables
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsHanunoo\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsHanunoo
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsHebrew\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsHebrew
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsHighPrivateUseSurrogates\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsHighPrivateUseSurrogates
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsHighSurrogates\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsHighSurrogates
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsHiragana\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsHiragana
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsIPAExtensions\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsIPAExtensions
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsIdeographicDescriptionCharacters\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsIdeographicDescriptionCharacters
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsKanbun\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsKanbun
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsKangxiRadicals\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsKangxiRadicals
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsKannada\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsKannada
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsKatakana\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsKatakana
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsKatakanaPhoneticExtensions\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsKatakanaPhoneticExtensions
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsKhmer\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsKhmer
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsKhmerSymbols\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsKhmerSymbols
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsLao\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsLao
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsLatin1Supplement\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsLatin1Supplement
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsLatinExtendedA\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsLatinExtendedA
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsLatinExtendedAdditional\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsLatinExtendedAdditional
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsLatinExtendedB\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsLatinExtendedB
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsLetterlikeSymbols\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsLetterlikeSymbols
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsLimbu\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsLimbu
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsLinearBIdeograms\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsLinearBIdeograms
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsLinearBSyllabary\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsLinearBSyllabary
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsLowSurrogates\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsLowSurrogates
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsMalayalam\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsMalayalam
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsMathematicalAlphanumericSymbols\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsMathematicalAlphanumericSymbols
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsMathematicalOperators\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsMathematicalOperators
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsMiscellaneousMathematicalSymbolsA\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsMiscellaneousMathematicalSymbolsA
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsMiscellaneousMathematicalSymbolsB\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsMiscellaneousMathematicalSymbolsB
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsMiscellaneousSymbols\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsMiscellaneousSymbols
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsMiscellaneousSymbolsandArrows\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsMiscellaneousSymbolsandArrows
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsMiscellaneousTechnical\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsMiscellaneousTechnical
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsMongolian\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsMongolian
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsMusicalSymbols\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsMusicalSymbols
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsMyanmar\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsMyanmar
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsNumberForms\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsNumberForms
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsOgham\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsOgham
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsOldItalic\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsOldItalic
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsOpticalCharacterRecognition\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsOpticalCharacterRecognition
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsOriya\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsOriya
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsOsmanya\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsOsmanya
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsPhoneticExtensions\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsPhoneticExtensions
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsPrivateUse\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsPrivateUse
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsPrivateUseArea\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsPrivateUseArea
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsRunic\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsRunic
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsShavian\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsShavian
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsSinhala\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsSinhala
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsSmallFormVariants\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsSmallFormVariants
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsSpacingModifierLetters\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsSpacingModifierLetters
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsSpecials\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsSpecials
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsSuperscriptsandSubscripts\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsSuperscriptsandSubscripts
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsSupplementalArrowsA\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsSupplementalArrowsA
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsSupplementalArrowsB\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsSupplementalArrowsB
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsSupplementalMathematicalOperators\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsSupplementalMathematicalOperators
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsSupplementaryPrivateUseAreaA\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsSupplementaryPrivateUseAreaA
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsSupplementaryPrivateUseAreaB\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsSupplementaryPrivateUseAreaB
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsSyriac\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsSyriac
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsTagalog\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsTagalog
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsTagbanwa\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsTagbanwa
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsTags\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsTags
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsTaiLe\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsTaiLe
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsTaiXuanJingSymbols\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsTaiXuanJingSymbols
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsTamil\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsTamil
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsTelugu\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsTelugu
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsThaana\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsThaana
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsThai\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsThai
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsTibetan\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsTibetan
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsUgaritic\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsUgaritic
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsUnifiedCanadianAboriginalSyllabics\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsUnifiedCanadianAboriginalSyllabics
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsVariationSelectors\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsVariationSelectors
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsVariationSelectorsSupplement\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsVariationSelectorsSupplement
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsYiRadicals\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsYiRadicals
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsYiSyllables\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsYiSyllables
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUCSIsYijingHexagramSymbols\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUCSIsYijingHexagramSymbols
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlURIEscape\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlURIEscape
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlURIEscapeStr\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlURIEscapeStr
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlURIGetAuthority\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlURIGetAuthority
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlURIGetFragment\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlURIGetFragment
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlURIGetOpaque\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlURIGetOpaque
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlURIGetPath\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlURIGetPath
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlURIGetPort\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlURIGetPort
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlURIGetQuery\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlURIGetQuery
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlURIGetQueryRaw\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlURIGetQueryRaw
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlURIGetScheme\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlURIGetScheme
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlURIGetServer\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlURIGetServer
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlURIGetUser\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlURIGetUser
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlURISetAuthority\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlURISetAuthority
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlURISetFragment\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlURISetFragment
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlURISetOpaque\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlURISetOpaque
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlURISetPath\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlURISetPath
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlURISetPort\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlURISetPort
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlURISetQuery\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlURISetQuery
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlURISetQueryRaw\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlURISetQueryRaw
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlURISetScheme\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlURISetScheme
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlURISetServer\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlURISetServer
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlURISetUser\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlURISetUser
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlURIUnescapeString\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlURIUnescapeString
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUTF8Charcmp\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUTF8Charcmp
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUTF8Size\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUTF8Size
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUTF8Strlen\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUTF8Strlen
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUTF8Strloc\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUTF8Strloc
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUTF8Strndup\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUTF8Strndup
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUTF8Strpos\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUTF8Strpos
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUTF8Strsize\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUTF8Strsize
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUTF8Strsub\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUTF8Strsub
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUnlinkNode\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUnlinkNode
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUnsetNsProp\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUnsetNsProp
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUnsetProp\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUnsetProp
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlValidCtxtNormalizeAttributeValue\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlValidCtxtNormalizeAttributeValue
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlValidNormalizeAttributeValue\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlValidNormalizeAttributeValue
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlValidateDocument\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlValidateDocument
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlValidateDocumentFinal\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlValidateDocumentFinal
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlValidateDtd\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlValidateDtd
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlValidateDtdFinal\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlValidateDtdFinal
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlValidateElement\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlValidateElement
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlValidateNCName\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlValidateNCName
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlValidateNMToken\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlValidateNMToken
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlValidateName\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlValidateName
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlValidateNameValue\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlValidateNameValue
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlValidateNamesValue\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlValidateNamesValue
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlValidateNmtokenValue\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlValidateNmtokenValue
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlValidateNmtokensValue\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlValidateNmtokensValue
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlValidateNotationUse\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlValidateNotationUse
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlValidateOneAttribute\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlValidateOneAttribute
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlValidateOneElement\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlValidateOneElement
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlValidateOneNamespace\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlValidateOneNamespace
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlValidatePopElement\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlValidatePopElement
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlValidatePushCData\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlValidatePushCData
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlValidatePushElement\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlValidatePushElement
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlValidateQName\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlValidateQName
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlValidateRoot\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlValidateRoot
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXIncludeProcess\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXIncludeProcess
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXIncludeProcessFlags\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXIncludeProcessFlags
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXIncludeProcessTree\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXIncludeProcessTree
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXIncludeProcessTreeFlags\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXIncludeProcessTreeFlags
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathAddValues\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathAddValues
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathBooleanFunction\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathBooleanFunction
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathCastBooleanToNumber\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathCastBooleanToNumber
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathCastBooleanToString\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathCastBooleanToString
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathCastNodeToNumber\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathCastNodeToNumber
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathCastNodeToString\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathCastNodeToString
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathCastNumberToBoolean\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathCastNumberToBoolean
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathCastNumberToString\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathCastNumberToString
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathCastStringToBoolean\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathCastStringToBoolean
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathCastStringToNumber\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathCastStringToNumber
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathCeilingFunction\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathCeilingFunction
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathCmpNodes\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathCmpNodes
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathCompareValues\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathCompareValues
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathConcatFunction\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathConcatFunction
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathContainsFunction\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathContainsFunction
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathContextSetCache\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathContextSetCache
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathCountFunction\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathCountFunction
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathDivValues\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathDivValues
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathEqualValues\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathEqualValues
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathErr\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathErr
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathEval\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathEval
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathEvalExpr\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathEvalExpr
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathEvalExpression\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathEvalExpression
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathFalseFunction\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathFalseFunction
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathFloorFunction\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathFloorFunction
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathFreeContext\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathFreeContext
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathFreeParserContext\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathFreeParserContext
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathGetContextDoc\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathGetContextDoc
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathGetContextNode\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathGetContextNode
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathGetContextPosition\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathGetContextPosition
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathGetContextSize\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathGetContextSize
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathGetFunction\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathGetFunction
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathGetFunctionURI\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathGetFunctionURI
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathIdFunction\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathIdFunction
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathInit\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathInit
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathIsInf\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathIsInf
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathIsNaN\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathIsNaN
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathIsNodeType\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathIsNodeType
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathLangFunction\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathLangFunction
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathLastFunction\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathLastFunction
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathLocalNameFunction\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathLocalNameFunction
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathModValues\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathModValues
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathMultValues\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathMultValues
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathNamespaceURIFunction\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathNamespaceURIFunction
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathNewBoolean\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathNewBoolean
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathNewCString\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathNewCString
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathNewContext\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathNewContext
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathNewFloat\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathNewFloat
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathNewNodeSet\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathNewNodeSet
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathNewParserContext\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathNewParserContext
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathNewString\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathNewString
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathNewValueTree\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathNewValueTree
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathNextAncestor\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathNextAncestor
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathNextAncestorOrSelf\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathNextAncestorOrSelf
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathNextAttribute\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathNextAttribute
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathNextChild\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathNextChild
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathNextDescendant\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathNextDescendant
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathNextDescendantOrSelf\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathNextDescendantOrSelf
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathNextFollowing\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathNextFollowing
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathNextFollowingSibling\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathNextFollowingSibling
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathNextNamespace\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathNextNamespace
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathNextParent\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathNextParent
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathNextPreceding\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathNextPreceding
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathNextPrecedingSibling\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathNextPrecedingSibling
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathNextSelf\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathNextSelf
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathNodeEval\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathNodeEval
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathNodeSetFreeNs\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathNodeSetFreeNs
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathNormalizeFunction\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathNormalizeFunction
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathNotEqualValues\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathNotEqualValues
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathNotFunction\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathNotFunction
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathNsLookup\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathNsLookup
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathNumberFunction\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathNumberFunction
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathOrderDocElems\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathOrderDocElems
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathParseNCName\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathParseNCName
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathParseName\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathParseName
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathParserGetContext\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathParserGetContext
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathPopBoolean\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathPopBoolean
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathPopNumber\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathPopNumber
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathPopString\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathPopString
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathPositionFunction\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathPositionFunction
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathRegisterAllFunctions\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathRegisterAllFunctions
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathRegisterNs\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathRegisterNs
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathRegisterVariable\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathRegisterVariable
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathRegisteredFuncsCleanup\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathRegisteredFuncsCleanup
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathRegisteredNsCleanup\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathRegisteredNsCleanup
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathRegisteredVariablesCleanup\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathRegisteredVariablesCleanup
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathRoot\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathRoot
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathRoundFunction\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathRoundFunction
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathSetContextDoc\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathSetContextDoc
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathSetContextNode\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathSetContextNode
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathStartsWithFunction\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathStartsWithFunction
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathStringEvalNumber\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathStringEvalNumber
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathStringFunction\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathStringFunction
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathStringLengthFunction\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathStringLengthFunction
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathSubValues\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathSubValues
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathSubstringAfterFunction\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathSubstringAfterFunction
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathSubstringBeforeFunction\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathSubstringBeforeFunction
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathSubstringFunction\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathSubstringFunction
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathSumFunction\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathSumFunction
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathTranslateFunction\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathTranslateFunction
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathTrueFunction\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathTrueFunction
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathValueFlipSign\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathValueFlipSign
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathVariableLookup\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathVariableLookup
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPathVariableLookupNS\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPathVariableLookupNS
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPatherror\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPatherror
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPtrEval\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPtrEval
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlXPtrNewContext\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlXPtrNewContext
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"name\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_name
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"children\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_children
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"properties\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_properties
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"last\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_last
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"prev\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_prev
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"next\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_next
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"parent\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_parent
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"type\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_type
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"doc\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_doc
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNewNode\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNewNode
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlNodeRemoveNsDef\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlNodeRemoveNsDef
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlSetValidErrors\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlSetValidErrors
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlFreeValidCtxt\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlFreeValidCtxt
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"serializeNode\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_serializeNode
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"saveNodeTo\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_saveNodeTo
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"outputBufferCreate\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlCreateOutputBuffer
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"outputBufferGetPythonFile\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_outputBufferGetPythonFile
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlOutputBufferClose\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlOutputBufferClose
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlOutputBufferFlush\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlOutputBufferFlush
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlSaveFileTo\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlSaveFileTo
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlSaveFormatFileTo\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlSaveFormatFileTo
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"inputBufferCreate\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlCreateInputBuffer
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"setEntityLoader\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlSetEntityLoader
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlRegisterErrorHandler\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlRegisterErrorHandler
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParserCtxtSetErrorHandler\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParserCtxtSetErrorHandler
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlParserCtxtGetErrorHandler\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlParserCtxtGetErrorHandler
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlFreeParserCtxt\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlFreeParserCtxt
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderSetErrorHandler\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderSetErrorHandler
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlTextReaderGetErrorHandler\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlTextReaderGetErrorHandler
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlFreeTextReader\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlFreeTextReader
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"addLocalCatalog\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_addLocalCatalog
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlRelaxNGSetValidErrors\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlRelaxNGSetValidErrors
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlRelaxNGFreeValidCtxt\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlRelaxNGFreeValidCtxt
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlSchemaSetValidErrors\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlSchemaSetValidErrors
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlSchemaFreeValidCtxt\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlSchemaFreeValidCtxt
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlC14NDocDumpMemory\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_C14NDocDumpMemory
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlC14NDocSaveTo\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_C14NDocSaveTo
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"getObjDesc\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_getObjDesc
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"compareNodesEqual\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_compareNodesEqual
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"nodeHash\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ml_meth: Some(
                    libxml_nodeHash
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlRegisterInputCallback\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlRegisterInputCallback
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"xmlUnregisterInputCallback\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ml_meth: Some(
                    libxml_xmlUnregisterInputCallback
                        as unsafe extern "C" fn(
                            *mut PyObject,
                            *mut PyObject,
                        ) -> *mut PyObject,
                ),
                ml_flags: 0x1 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: 0 as *const libc::c_char,
                ml_meth: None,
                ml_flags: 0 as libc::c_int,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
    ]
};
#[no_mangle]
pub unsafe extern "C" fn initlibxml2mod() {
    let mut module: *mut PyObject = 0 as *mut PyObject;
    module = Py_InitModule4_64(
        b"libxml2mod\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        libxmlMethods.as_mut_ptr(),
        0 as *mut libc::c_void as *mut libc::c_char,
        0 as *mut libc::c_void as *mut PyObject,
        1013 as libc::c_int,
    );
    if module.is_null() {
        return;
    }
    xmlInitParser();
    libxml_xmlErrorInitialize();
}
