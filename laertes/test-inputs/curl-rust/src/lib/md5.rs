use ::libc;
extern "C" {
    fn curlx_uztoui(uznum: size_t) -> libc::c_uint;
    fn MD5_Final(md: *mut libc::c_uchar, c: *mut MD5_CTX) -> libc::c_int;
    fn MD5_Init(c: *mut MD5_CTX) -> libc::c_int;
    fn MD5_Update(
        c: *mut MD5_CTX,
        data: *const libc::c_void,
        len: size_t,
    ) -> libc::c_int;
    static mut Curl_cmalloc: curl_malloc_callback;
    static mut Curl_cfree: curl_free_callback;
}
pub type size_t = libc::c_ulong;
pub type curl_malloc_callback = Option::<
    unsafe extern "C" fn(size_t) -> *mut libc::c_void,
>;
pub type curl_free_callback = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type CURLcode = libc::c_uint;
pub const CURL_LAST: CURLcode = 99;
pub const CURLE_SSL_CLIENTCERT: CURLcode = 98;
pub const CURLE_PROXY: CURLcode = 97;
pub const CURLE_QUIC_CONNECT_ERROR: CURLcode = 96;
pub const CURLE_HTTP3: CURLcode = 95;
pub const CURLE_AUTH_ERROR: CURLcode = 94;
pub const CURLE_RECURSIVE_API_CALL: CURLcode = 93;
pub const CURLE_HTTP2_STREAM: CURLcode = 92;
pub const CURLE_SSL_INVALIDCERTSTATUS: CURLcode = 91;
pub const CURLE_SSL_PINNEDPUBKEYNOTMATCH: CURLcode = 90;
pub const CURLE_NO_CONNECTION_AVAILABLE: CURLcode = 89;
pub const CURLE_CHUNK_FAILED: CURLcode = 88;
pub const CURLE_FTP_BAD_FILE_LIST: CURLcode = 87;
pub const CURLE_RTSP_SESSION_ERROR: CURLcode = 86;
pub const CURLE_RTSP_CSEQ_ERROR: CURLcode = 85;
pub const CURLE_FTP_PRET_FAILED: CURLcode = 84;
pub const CURLE_SSL_ISSUER_ERROR: CURLcode = 83;
pub const CURLE_SSL_CRL_BADFILE: CURLcode = 82;
pub const CURLE_AGAIN: CURLcode = 81;
pub const CURLE_SSL_SHUTDOWN_FAILED: CURLcode = 80;
pub const CURLE_SSH: CURLcode = 79;
pub const CURLE_REMOTE_FILE_NOT_FOUND: CURLcode = 78;
pub const CURLE_SSL_CACERT_BADFILE: CURLcode = 77;
pub const CURLE_CONV_REQD: CURLcode = 76;
pub const CURLE_CONV_FAILED: CURLcode = 75;
pub const CURLE_TFTP_NOSUCHUSER: CURLcode = 74;
pub const CURLE_REMOTE_FILE_EXISTS: CURLcode = 73;
pub const CURLE_TFTP_UNKNOWNID: CURLcode = 72;
pub const CURLE_TFTP_ILLEGAL: CURLcode = 71;
pub const CURLE_REMOTE_DISK_FULL: CURLcode = 70;
pub const CURLE_TFTP_PERM: CURLcode = 69;
pub const CURLE_TFTP_NOTFOUND: CURLcode = 68;
pub const CURLE_LOGIN_DENIED: CURLcode = 67;
pub const CURLE_SSL_ENGINE_INITFAILED: CURLcode = 66;
pub const CURLE_SEND_FAIL_REWIND: CURLcode = 65;
pub const CURLE_USE_SSL_FAILED: CURLcode = 64;
pub const CURLE_FILESIZE_EXCEEDED: CURLcode = 63;
pub const CURLE_LDAP_INVALID_URL: CURLcode = 62;
pub const CURLE_BAD_CONTENT_ENCODING: CURLcode = 61;
pub const CURLE_PEER_FAILED_VERIFICATION: CURLcode = 60;
pub const CURLE_SSL_CIPHER: CURLcode = 59;
pub const CURLE_SSL_CERTPROBLEM: CURLcode = 58;
pub const CURLE_OBSOLETE57: CURLcode = 57;
pub const CURLE_RECV_ERROR: CURLcode = 56;
pub const CURLE_SEND_ERROR: CURLcode = 55;
pub const CURLE_SSL_ENGINE_SETFAILED: CURLcode = 54;
pub const CURLE_SSL_ENGINE_NOTFOUND: CURLcode = 53;
pub const CURLE_GOT_NOTHING: CURLcode = 52;
pub const CURLE_OBSOLETE51: CURLcode = 51;
pub const CURLE_OBSOLETE50: CURLcode = 50;
pub const CURLE_SETOPT_OPTION_SYNTAX: CURLcode = 49;
pub const CURLE_UNKNOWN_OPTION: CURLcode = 48;
pub const CURLE_TOO_MANY_REDIRECTS: CURLcode = 47;
pub const CURLE_OBSOLETE46: CURLcode = 46;
pub const CURLE_INTERFACE_FAILED: CURLcode = 45;
pub const CURLE_OBSOLETE44: CURLcode = 44;
pub const CURLE_BAD_FUNCTION_ARGUMENT: CURLcode = 43;
pub const CURLE_ABORTED_BY_CALLBACK: CURLcode = 42;
pub const CURLE_FUNCTION_NOT_FOUND: CURLcode = 41;
pub const CURLE_OBSOLETE40: CURLcode = 40;
pub const CURLE_LDAP_SEARCH_FAILED: CURLcode = 39;
pub const CURLE_LDAP_CANNOT_BIND: CURLcode = 38;
pub const CURLE_FILE_COULDNT_READ_FILE: CURLcode = 37;
pub const CURLE_BAD_DOWNLOAD_RESUME: CURLcode = 36;
pub const CURLE_SSL_CONNECT_ERROR: CURLcode = 35;
pub const CURLE_HTTP_POST_ERROR: CURLcode = 34;
pub const CURLE_RANGE_ERROR: CURLcode = 33;
pub const CURLE_OBSOLETE32: CURLcode = 32;
pub const CURLE_FTP_COULDNT_USE_REST: CURLcode = 31;
pub const CURLE_FTP_PORT_FAILED: CURLcode = 30;
pub const CURLE_OBSOLETE29: CURLcode = 29;
pub const CURLE_OPERATION_TIMEDOUT: CURLcode = 28;
pub const CURLE_OUT_OF_MEMORY: CURLcode = 27;
pub const CURLE_READ_ERROR: CURLcode = 26;
pub const CURLE_UPLOAD_FAILED: CURLcode = 25;
pub const CURLE_OBSOLETE24: CURLcode = 24;
pub const CURLE_WRITE_ERROR: CURLcode = 23;
pub const CURLE_HTTP_RETURNED_ERROR: CURLcode = 22;
pub const CURLE_QUOTE_ERROR: CURLcode = 21;
pub const CURLE_OBSOLETE20: CURLcode = 20;
pub const CURLE_FTP_COULDNT_RETR_FILE: CURLcode = 19;
pub const CURLE_PARTIAL_FILE: CURLcode = 18;
pub const CURLE_FTP_COULDNT_SET_TYPE: CURLcode = 17;
pub const CURLE_HTTP2: CURLcode = 16;
pub const CURLE_FTP_CANT_GET_HOST: CURLcode = 15;
pub const CURLE_FTP_WEIRD_227_FORMAT: CURLcode = 14;
pub const CURLE_FTP_WEIRD_PASV_REPLY: CURLcode = 13;
pub const CURLE_FTP_ACCEPT_TIMEOUT: CURLcode = 12;
pub const CURLE_FTP_WEIRD_PASS_REPLY: CURLcode = 11;
pub const CURLE_FTP_ACCEPT_FAILED: CURLcode = 10;
pub const CURLE_REMOTE_ACCESS_DENIED: CURLcode = 9;
pub const CURLE_WEIRD_SERVER_REPLY: CURLcode = 8;
pub const CURLE_COULDNT_CONNECT: CURLcode = 7;
pub const CURLE_COULDNT_RESOLVE_HOST: CURLcode = 6;
pub const CURLE_COULDNT_RESOLVE_PROXY: CURLcode = 5;
pub const CURLE_NOT_BUILT_IN: CURLcode = 4;
pub const CURLE_URL_MALFORMAT: CURLcode = 3;
pub const CURLE_FAILED_INIT: CURLcode = 2;
pub const CURLE_UNSUPPORTED_PROTOCOL: CURLcode = 1;
pub const CURLE_OK: CURLcode = 0;
pub type HMAC_hinit_func = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type HMAC_hupdate_func = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const libc::c_uchar, libc::c_uint) -> (),
>;
pub type HMAC_hfinal_func = Option::<
    unsafe extern "C" fn(*mut libc::c_uchar, *mut libc::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HMAC_params {
    pub hmac_hinit: HMAC_hinit_func,
    pub hmac_hupdate: HMAC_hupdate_func,
    pub hmac_hfinal: HMAC_hfinal_func,
    pub hmac_ctxtsize: libc::c_uint,
    pub hmac_maxkeylen: libc::c_uint,
    pub hmac_resultlen: libc::c_uint,
}
pub type Curl_MD5_init_func = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type Curl_MD5_update_func = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const libc::c_uchar, libc::c_uint) -> (),
>;
pub type Curl_MD5_final_func = Option::<
    unsafe extern "C" fn(*mut libc::c_uchar, *mut libc::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MD5_params {
    pub md5_init_func: Curl_MD5_init_func,
    pub md5_update_func: Curl_MD5_update_func,
    pub md5_final_func: Curl_MD5_final_func,
    pub md5_ctxtsize: libc::c_uint,
    pub md5_resultlen: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MD5_context {
    pub md5_hash: *const MD5_params,
    pub md5_hashctx: *mut libc::c_void,
}
pub type MD5_CTX = MD5state_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MD5state_st {
    pub A: libc::c_uint,
    pub B: libc::c_uint,
    pub C: libc::c_uint,
    pub D: libc::c_uint,
    pub Nl: libc::c_uint,
    pub Nh: libc::c_uint,
    pub data: [libc::c_uint; 16],
    pub num: libc::c_uint,
}
#[no_mangle]
pub static mut Curl_HMAC_MD5: [HMAC_params; 1] = unsafe {
    [
        {
            let mut init = HMAC_params {
                hmac_hinit: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> ()>,
                    HMAC_hinit_func,
                >(
                    ::std::mem::transmute::<
                        Option::<unsafe extern "C" fn(*mut MD5_CTX) -> libc::c_int>,
                        Option::<unsafe extern "C" fn() -> ()>,
                    >(
                        Some(
                            MD5_Init as unsafe extern "C" fn(*mut MD5_CTX) -> libc::c_int,
                        ),
                    ),
                ),
                hmac_hupdate: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> ()>,
                    HMAC_hupdate_func,
                >(
                    ::std::mem::transmute::<
                        Option::<
                            unsafe extern "C" fn(
                                *mut MD5_CTX,
                                *const libc::c_void,
                                size_t,
                            ) -> libc::c_int,
                        >,
                        Option::<unsafe extern "C" fn() -> ()>,
                    >(
                        Some(
                            MD5_Update
                                as unsafe extern "C" fn(
                                    *mut MD5_CTX,
                                    *const libc::c_void,
                                    size_t,
                                ) -> libc::c_int,
                        ),
                    ),
                ),
                hmac_hfinal: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> ()>,
                    HMAC_hfinal_func,
                >(
                    ::std::mem::transmute::<
                        Option::<
                            unsafe extern "C" fn(
                                *mut libc::c_uchar,
                                *mut MD5_CTX,
                            ) -> libc::c_int,
                        >,
                        Option::<unsafe extern "C" fn() -> ()>,
                    >(
                        Some(
                            MD5_Final
                                as unsafe extern "C" fn(
                                    *mut libc::c_uchar,
                                    *mut MD5_CTX,
                                ) -> libc::c_int,
                        ),
                    ),
                ),
                hmac_ctxtsize: ::std::mem::size_of::<MD5_CTX>() as libc::c_ulong
                    as libc::c_uint,
                hmac_maxkeylen: 64 as libc::c_int as libc::c_uint,
                hmac_resultlen: 16 as libc::c_int as libc::c_uint,
            };
            init
        },
    ]
};
#[no_mangle]
pub static mut Curl_DIGEST_MD5: [MD5_params; 1] = unsafe {
    [
        {
            let mut init = MD5_params {
                md5_init_func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> ()>,
                    Curl_MD5_init_func,
                >(
                    ::std::mem::transmute::<
                        Option::<unsafe extern "C" fn(*mut MD5_CTX) -> libc::c_int>,
                        Option::<unsafe extern "C" fn() -> ()>,
                    >(
                        Some(
                            MD5_Init as unsafe extern "C" fn(*mut MD5_CTX) -> libc::c_int,
                        ),
                    ),
                ),
                md5_update_func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> ()>,
                    Curl_MD5_update_func,
                >(
                    ::std::mem::transmute::<
                        Option::<
                            unsafe extern "C" fn(
                                *mut MD5_CTX,
                                *const libc::c_void,
                                size_t,
                            ) -> libc::c_int,
                        >,
                        Option::<unsafe extern "C" fn() -> ()>,
                    >(
                        Some(
                            MD5_Update
                                as unsafe extern "C" fn(
                                    *mut MD5_CTX,
                                    *const libc::c_void,
                                    size_t,
                                ) -> libc::c_int,
                        ),
                    ),
                ),
                md5_final_func: ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> ()>,
                    Curl_MD5_final_func,
                >(
                    ::std::mem::transmute::<
                        Option::<
                            unsafe extern "C" fn(
                                *mut libc::c_uchar,
                                *mut MD5_CTX,
                            ) -> libc::c_int,
                        >,
                        Option::<unsafe extern "C" fn() -> ()>,
                    >(
                        Some(
                            MD5_Final
                                as unsafe extern "C" fn(
                                    *mut libc::c_uchar,
                                    *mut MD5_CTX,
                                ) -> libc::c_int,
                        ),
                    ),
                ),
                md5_ctxtsize: ::std::mem::size_of::<MD5_CTX>() as libc::c_ulong
                    as libc::c_uint,
                md5_resultlen: 16 as libc::c_int as libc::c_uint,
            };
            init
        },
    ]
};
#[no_mangle]
pub unsafe extern "C" fn Curl_md5it(
    mut outbuffer: *mut libc::c_uchar,
    mut input: *const libc::c_uchar,
    len: size_t,
) {
    let mut ctx: MD5_CTX = MD5_CTX {
        A: 0,
        B: 0,
        C: 0,
        D: 0,
        Nl: 0,
        Nh: 0,
        data: [0; 16],
        num: 0,
    };
    MD5_Init(&mut ctx);
    MD5_Update(&mut ctx, input as *const libc::c_void, curlx_uztoui(len) as size_t);
    MD5_Final(outbuffer, &mut ctx);
}
#[no_mangle]
pub unsafe extern "C" fn Curl_MD5_init(
    mut md5params: *const MD5_params,
) -> *mut MD5_context {
    let mut ctxt: *mut MD5_context = 0 as *mut MD5_context;
    ctxt = Curl_cmalloc
        .expect(
            "non-null function pointer",
        )(::std::mem::size_of::<MD5_context>() as libc::c_ulong) as *mut MD5_context;
    if ctxt.is_null() {
        return ctxt;
    }
    let ref mut fresh0 = (*ctxt).md5_hashctx;
    *fresh0 = Curl_cmalloc
        .expect("non-null function pointer")((*md5params).md5_ctxtsize as size_t);
    if ((*ctxt).md5_hashctx).is_null() {
        Curl_cfree.expect("non-null function pointer")(ctxt as *mut libc::c_void);
        return 0 as *mut MD5_context;
    }
    let ref mut fresh1 = (*ctxt).md5_hash;
    *fresh1 = md5params;
    (Some(((*md5params).md5_init_func).expect("non-null function pointer")))
        .expect("non-null function pointer")((*ctxt).md5_hashctx);
    return ctxt;
}
#[no_mangle]
pub unsafe extern "C" fn Curl_MD5_update(
    mut context: *mut MD5_context,
    mut data: *const libc::c_uchar,
    mut len: libc::c_uint,
) -> CURLcode {
    (Some(((*(*context).md5_hash).md5_update_func).expect("non-null function pointer")))
        .expect("non-null function pointer")((*context).md5_hashctx, data, len);
    return CURLE_OK;
}
#[no_mangle]
pub unsafe extern "C" fn Curl_MD5_final(
    mut context: *mut MD5_context,
    mut result: *mut libc::c_uchar,
) -> CURLcode {
    (Some(((*(*context).md5_hash).md5_final_func).expect("non-null function pointer")))
        .expect("non-null function pointer")(result, (*context).md5_hashctx);
    Curl_cfree.expect("non-null function pointer")((*context).md5_hashctx);
    Curl_cfree.expect("non-null function pointer")(context as *mut libc::c_void);
    return CURLE_OK;
}
