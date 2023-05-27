use ::libc;
use ::c2rust_bitfields;
extern "C" {
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    pub type ldap;
    
    
    
    
    
    pub type berelement;
    pub type sockbuf;
    pub type ldapmsg;
    
    
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strncmp(
        _: *const i8,
        _: *const i8,
        _: u64,
    ) -> i32;
    fn strlen(_: *const i8) -> u64;
    fn __errno_location() -> *mut i32;
    fn ber_free(ber: *mut BerElement, freebuf: i32);
    fn ldap_get_attribute_ber(
        ld: *mut LDAP,
        e: *mut LDAPMessage,
        ber: *mut BerElement,
        attr: *mut berval,
        vals: *mut *mut berval,
    ) -> i32;
    fn ldap_get_dn_ber(
        ld: *mut LDAP,
        e: *mut LDAPMessage,
        berout: *mut *mut BerElement,
        dn: *mut berval,
    ) -> i32;
    fn ldap_next_message(ld: *mut LDAP, msg: *mut LDAPMessage) -> *mut LDAPMessage;
    fn ldap_first_message(ld: *mut LDAP, chain: *mut LDAPMessage) -> *mut LDAPMessage;
    fn ldap_err2string(err: i32) -> *mut i8;
    fn ldap_parse_result(
        ld: *mut LDAP,
        res: *mut LDAPMessage,
        errcodep: *mut i32,
        matcheddnp: *mut *mut i8,
        diagmsgp: *mut *mut i8,
        referralsp: *mut *mut *mut i8,
        serverctrls: *mut *mut *mut LDAPControl,
        freeit: i32,
    ) -> i32;
    fn ldap_sasl_bind(
        ld: *mut LDAP,
        dn: *const i8,
        mechanism: *const i8,
        cred: *mut berval,
        serverctrls: *mut *mut LDAPControl,
        clientctrls: *mut *mut LDAPControl,
        msgidp: *mut i32,
    ) -> i32;
    fn ldap_free_urldesc(ludp: *mut LDAPURLDesc);
    fn ldap_url_parse(
        url: *const i8,
        ludpp: *mut *mut LDAPURLDesc,
    ) -> i32;
    fn ldap_memfree(p: *mut libc::c_void);
    fn ldap_unbind_ext(
        ld: *mut LDAP,
        serverctrls: *mut *mut LDAPControl,
        clientctrls: *mut *mut LDAPControl,
    ) -> i32;
    fn ldap_search_ext(
        ld: *mut LDAP,
        base: *const i8,
        scope: i32,
        filter: *const i8,
        attrs: *mut *mut i8,
        attrsonly: i32,
        serverctrls: *mut *mut LDAPControl,
        clientctrls: *mut *mut LDAPControl,
        timeout: *mut timeval,
        sizelimit: i32,
        msgidp: *mut i32,
    ) -> i32;
    fn ldap_msgfree(lm: *mut LDAPMessage) -> i32;
    fn ldap_msgtype(lm: *mut LDAPMessage) -> i32;
    fn ldap_result(
        ld: *mut LDAP,
        msgid: i32,
        all: i32,
        timeout: *mut timeval,
        result: *mut *mut LDAPMessage,
    ) -> i32;
    fn ber_sockbuf_add_io(
        sb: *mut Sockbuf,
        sbio: *mut Sockbuf_IO,
        layer: i32,
        arg: *mut libc::c_void,
    ) -> i32;
    fn ldap_abandon_ext(
        ld: *mut LDAP,
        msgid: i32,
        serverctrls: *mut *mut LDAPControl,
        clientctrls: *mut *mut LDAPControl,
    ) -> i32;
    fn ber_memfree(p: *mut libc::c_void);
    fn ldap_get_option(
        ld: *mut LDAP,
        option: i32,
        outvalue: *mut libc::c_void,
    ) -> i32;
    fn ldap_set_option(
        ld: *mut LDAP,
        option: i32,
        invalue: *const libc::c_void,
    ) -> i32;
    
    
    
    
    
    
    
    
    
    
    
    fn ldap_pvt_url_scheme2proto(_: *const i8) -> i32;
    fn ldap_init_fd(
        fd: ber_socket_t,
        proto: i32,
        url: *const i8,
        ld: *mut *mut LDAP,
    ) -> i32;
}
pub use crate::src::lib::base64::Curl_base64_encode;
pub use crate::src::lib::connect::Curl_conncontrol;
pub use crate::src::lib::curl_ctype::Curl_isprint;
pub use crate::src::lib::curl_ctype::Curl_isspace;
pub use crate::src::lib::mprintf::curl_msnprintf;
pub use crate::src::lib::sendf::Curl_client_write;
pub use crate::src::lib::sendf::Curl_failf;
pub use crate::src::lib::sendf::Curl_infof;
pub use crate::src::lib::transfer::Curl_setup_transfer;
pub use crate::src::lib::vtls::vtls::Curl_ssl_connect_nonblocking;
pub use crate::src::lib::vtls::vtls::Curl_ssl_data_pending;
pub use crate::src::lib::altsvc::altsvcinfo;
pub use crate::src::lib::asyn_thread::Curl_share;
pub use crate::src::lib::asyn_thread::thread_data;
pub use crate::src::lib::content_encoding::contenc_writer;
pub use crate::src::lib::easy::hsts;
pub use crate::src::lib::ftp::http_connect_state;
pub use crate::src::lib::http2::curl_pushheaders;
pub use crate::src::lib::conncache::Gsasl_session;
pub use crate::src::lib::easy::Curl_ccalloc;
pub use crate::src::lib::easy::Curl_cfree;
pub use crate::src::lib::escape::Gsasl;
pub use crate::src::lib::mqtt::_IO_codecvt;
pub use crate::src::lib::speedcheck::nghttp2_session;
pub use crate::src::lib::urlapi::psl_ctx_st;
pub use crate::src::src::tool_cb_rea::_IO_wide_data;
pub use crate::src::src::tool_msgs::_IO_marker;
pub use crate::src::lib::smb::smb_request;
pub use crate::src::lib::telnet::TELNET;
pub use crate::src::lib::tftp::tftp_state_data;
pub use crate::src::lib::urlapi::Curl_URL;
pub use crate::src::lib::vtls::openssl::ssl_backend_data;
pub type __uint8_t = crate::src::lib::http2::__uint8_t;
pub type __int32_t = crate::src::lib::http2::__int32_t;
pub type __uint32_t = crate::src::lib::http2::__uint32_t;
pub type __off_t = crate::src::lib::http2::__off_t;
pub type __off64_t = crate::src::lib::http2::__off64_t;
pub type __pid_t = crate::src::lib::http2::__pid_t;
pub type __time_t = crate::src::lib::http2::__time_t;
pub type __suseconds_t = i64;
pub type __ssize_t = crate::src::lib::http2::__ssize_t;
pub type __socklen_t = crate::src::lib::http2::__socklen_t;
pub type pid_t = crate::src::lib::http2::pid_t;
pub type ssize_t = crate::src::lib::http2::ssize_t;
pub type time_t = crate::src::lib::http2::time_t;
pub type size_t = crate::src::lib::http2::size_t;
pub type int32_t = crate::src::lib::http2::int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type socklen_t = crate::src::lib::http2::socklen_t;
pub type sa_family_t = crate::src::lib::http2::sa_family_t;
// #[derive(Copy, Clone)]

pub type sockaddr = crate::src::lib::http2::sockaddr;
pub type curl_socklen_t = crate::src::lib::http2::curl_socklen_t;
pub type curl_off_t = crate::src::lib::http2::curl_off_t;
// #[derive(Copy, Clone)]

pub type _IO_FILE = crate::src::lib::http2::_IO_FILE;
pub type _IO_lock_t = crate::src::lib::http2::_IO_lock_t;
pub type FILE = crate::src::lib::http2::FILE;
// #[derive(Copy, Clone)]

pub type Curl_easy = crate::src::lib::http2::Curl_easy;
// #[derive(Copy, Clone)]

pub type curl_tlssessioninfo = crate::src::lib::http2::curl_tlssessioninfo;
pub type curl_sslbackend = crate::src::lib::http2::curl_sslbackend;
pub const CURLSSLBACKEND_RUSTLS: curl_sslbackend = 14;
pub const CURLSSLBACKEND_BEARSSL: curl_sslbackend = 13;
pub const CURLSSLBACKEND_MESALINK: curl_sslbackend = 12;
pub const CURLSSLBACKEND_MBEDTLS: curl_sslbackend = 11;
pub const CURLSSLBACKEND_AXTLS: curl_sslbackend = 10;
pub const CURLSSLBACKEND_SECURETRANSPORT: curl_sslbackend = 9;
pub const CURLSSLBACKEND_SCHANNEL: curl_sslbackend = 8;
pub const CURLSSLBACKEND_WOLFSSL: curl_sslbackend = 7;
pub const CURLSSLBACKEND_POLARSSL: curl_sslbackend = 6;
pub const CURLSSLBACKEND_GSKIT: curl_sslbackend = 5;
pub const CURLSSLBACKEND_OBSOLETE4: curl_sslbackend = 4;
pub const CURLSSLBACKEND_NSS: curl_sslbackend = 3;
pub const CURLSSLBACKEND_GNUTLS: curl_sslbackend = 2;
pub const CURLSSLBACKEND_OPENSSL: curl_sslbackend = 1;
pub const CURLSSLBACKEND_NONE: curl_sslbackend = 0;
// #[derive(Copy, Clone, BitfieldStruct)]

pub type PureInfo = crate::src::lib::http2::PureInfo;
pub type bit = crate::src::lib::http2::bit;
pub type CURLproxycode = crate::src::lib::http2::CURLproxycode;
pub const CURLPX_LAST: CURLproxycode = 34;
pub const CURLPX_USER_REJECTED: CURLproxycode = 33;
pub const CURLPX_UNKNOWN_MODE: CURLproxycode = 32;
pub const CURLPX_UNKNOWN_FAIL: CURLproxycode = 31;
pub const CURLPX_SEND_REQUEST: CURLproxycode = 30;
pub const CURLPX_SEND_CONNECT: CURLproxycode = 29;
pub const CURLPX_SEND_AUTH: CURLproxycode = 28;
pub const CURLPX_RESOLVE_HOST: CURLproxycode = 27;
pub const CURLPX_REQUEST_FAILED: CURLproxycode = 26;
pub const CURLPX_REPLY_UNASSIGNED: CURLproxycode = 25;
pub const CURLPX_REPLY_TTL_EXPIRED: CURLproxycode = 24;
pub const CURLPX_REPLY_NOT_ALLOWED: CURLproxycode = 23;
pub const CURLPX_REPLY_NETWORK_UNREACHABLE: CURLproxycode = 22;
pub const CURLPX_REPLY_HOST_UNREACHABLE: CURLproxycode = 21;
pub const CURLPX_REPLY_GENERAL_SERVER_FAILURE: CURLproxycode = 20;
pub const CURLPX_REPLY_CONNECTION_REFUSED: CURLproxycode = 19;
pub const CURLPX_REPLY_COMMAND_NOT_SUPPORTED: CURLproxycode = 18;
pub const CURLPX_REPLY_ADDRESS_TYPE_NOT_SUPPORTED: CURLproxycode = 17;
pub const CURLPX_RECV_REQACK: CURLproxycode = 16;
pub const CURLPX_RECV_CONNECT: CURLproxycode = 15;
pub const CURLPX_RECV_AUTH: CURLproxycode = 14;
pub const CURLPX_RECV_ADDRESS: CURLproxycode = 13;
pub const CURLPX_NO_AUTH: CURLproxycode = 12;
pub const CURLPX_LONG_USER: CURLproxycode = 11;
pub const CURLPX_LONG_PASSWD: CURLproxycode = 10;
pub const CURLPX_LONG_HOSTNAME: CURLproxycode = 9;
pub const CURLPX_IDENTD_DIFFER: CURLproxycode = 8;
pub const CURLPX_IDENTD: CURLproxycode = 7;
pub const CURLPX_GSSAPI_PROTECTION: CURLproxycode = 6;
pub const CURLPX_GSSAPI_PERMSG: CURLproxycode = 5;
pub const CURLPX_GSSAPI: CURLproxycode = 4;
pub const CURLPX_CLOSED: CURLproxycode = 3;
pub const CURLPX_BAD_VERSION: CURLproxycode = 2;
pub const CURLPX_BAD_ADDRESS_TYPE: CURLproxycode = 1;
pub const CURLPX_OK: CURLproxycode = 0;
// #[derive(Copy, Clone)]

pub type curl_certinfo = crate::src::lib::http2::curl_certinfo;
// #[derive(Copy, Clone)]

pub type curl_slist = crate::src::lib::http2::curl_slist;
// #[derive(Copy, Clone)]

pub type WildcardData = crate::src::lib::http2::WildcardData;
pub type wildcard_dtor = crate::src::lib::http2::wildcard_dtor;
// #[derive(Copy, Clone)]

pub type Curl_llist = crate::src::lib::http2::Curl_llist;
pub type Curl_llist_dtor = crate::src::lib::http2::Curl_llist_dtor;
// #[derive(Copy, Clone)]

pub type Curl_llist_element = crate::src::lib::http2::Curl_llist_element;
pub type wildcard_states = crate::src::lib::http2::wildcard_states;
pub const CURLWC_DONE: wildcard_states = 7;
pub const CURLWC_ERROR: wildcard_states = 6;
pub const CURLWC_SKIP: wildcard_states = 5;
pub const CURLWC_CLEAN: wildcard_states = 4;
pub const CURLWC_DOWNLOADING: wildcard_states = 3;
pub const CURLWC_MATCHING: wildcard_states = 2;
pub const CURLWC_INIT: wildcard_states = 1;
pub const CURLWC_CLEAR: wildcard_states = 0;
// #[derive(Copy, Clone, BitfieldStruct)]

pub type UrlState = crate::src::lib::http2::UrlState;
// #[derive(Copy, Clone)]

pub type dynamically_allocated_data = crate::src::lib::http2::dynamically_allocated_data;
pub type trailers_state = crate::src::lib::http2::trailers_state;
pub const TRAILERS_DONE: trailers_state = 3;
pub const TRAILERS_SENDING: trailers_state = 2;
pub const TRAILERS_INITIALIZED: trailers_state = 1;
pub const TRAILERS_NONE: trailers_state = 0;
// #[derive(Copy, Clone)]

pub type dynbuf = crate::src::lib::http2::dynbuf;
pub type Curl_HttpReq = crate::src::lib::http2::Curl_HttpReq;
pub const HTTPREQ_HEAD: Curl_HttpReq = 5;
pub const HTTPREQ_PUT: Curl_HttpReq = 4;
pub const HTTPREQ_POST_MIME: Curl_HttpReq = 3;
pub const HTTPREQ_POST_FORM: Curl_HttpReq = 2;
pub const HTTPREQ_POST: Curl_HttpReq = 1;
pub const HTTPREQ_GET: Curl_HttpReq = 0;
// #[derive(Copy, Clone)]

pub type urlpieces = crate::src::lib::http2::urlpieces;
pub type CURLU = crate::src::lib::http2::CURLU;
pub type curl_read_callback = crate::src::lib::http2::curl_read_callback;
// #[derive(Copy, Clone)]

pub type time_node = crate::src::lib::http2::time_node;
pub type expire_id = crate::src::lib::http2::expire_id;
pub const EXPIRE_LAST: expire_id = 13;
pub const EXPIRE_QUIC: expire_id = 12;
pub const EXPIRE_TOOFAST: expire_id = 11;
pub const EXPIRE_TIMEOUT: expire_id = 10;
pub const EXPIRE_SPEEDCHECK: expire_id = 9;
pub const EXPIRE_RUN_NOW: expire_id = 8;
pub const EXPIRE_MULTI_PENDING: expire_id = 7;
pub const EXPIRE_HAPPY_EYEBALLS: expire_id = 6;
pub const EXPIRE_HAPPY_EYEBALLS_DNS: expire_id = 5;
pub const EXPIRE_DNS_PER_NAME2: expire_id = 4;
pub const EXPIRE_DNS_PER_NAME: expire_id = 3;
pub const EXPIRE_CONNECTTIMEOUT: expire_id = 2;
pub const EXPIRE_ASYNC_NAME: expire_id = 1;
pub const EXPIRE_100_TIMEOUT: expire_id = 0;
// #[derive(Copy, Clone)]

pub type curltime = crate::src::lib::http2::curltime;
// #[derive(Copy, Clone)]

pub type Curl_tree = crate::src::lib::http2::Curl_tree;
// #[derive(Copy, Clone, BitfieldStruct)]

pub type Curl_async = crate::src::lib::http2::Curl_async;
// #[derive(Copy, Clone)]

pub type Curl_dns_entry = crate::src::lib::http2::Curl_dns_entry;
// #[derive(Copy, Clone)]

pub type Curl_addrinfo = crate::src::lib::http2::Curl_addrinfo;
// #[derive(Copy, Clone, BitfieldStruct)]

pub type auth = crate::src::lib::http2::auth;
// #[derive(Copy, Clone, BitfieldStruct)]

pub type digestdata = crate::src::lib::http2::digestdata;
// #[derive(Copy, Clone)]

pub type tempbuf = crate::src::lib::http2::tempbuf;
// #[derive(Copy, Clone)]

pub type Curl_ssl_session = crate::src::lib::http2::Curl_ssl_session;
// #[derive(Copy, Clone, BitfieldStruct)]

pub type ssl_primary_config = crate::src::lib::http2::ssl_primary_config;
// #[derive(Copy, Clone)]

pub type curl_blob = crate::src::lib::http2::curl_blob;
// #[derive(Copy, Clone)]

pub type conncache = crate::src::lib::http2::conncache;
// #[derive(Copy, Clone)]

pub type Curl_hash = crate::src::lib::http2::Curl_hash;
pub type Curl_hash_dtor = crate::src::lib::http2::Curl_hash_dtor;
pub type comp_function = crate::src::lib::http2::comp_function;
pub type hash_function = crate::src::lib::http2::hash_function;
// #[derive(Copy, Clone, BitfieldStruct)]

pub type Progress = crate::src::lib::http2::Progress;
pub type timediff_t = crate::src::lib::http2::timediff_t;
// #[derive(Copy, Clone)]

pub type CookieInfo = crate::src::lib::http2::CookieInfo;
// #[derive(Copy, Clone)]

pub type Cookie = crate::src::lib::http2::Cookie;
// #[derive(Copy, Clone, BitfieldStruct)]

pub type UserDefined = crate::src::lib::http2::UserDefined;
pub type curl_trailer_callback = crate::src::lib::http2::curl_trailer_callback;
pub type multidone_func = crate::src::lib::http2::multidone_func;
pub type CURLcode = crate::src::lib::http2::CURLcode;
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
pub type curl_resolver_start_callback = crate::src::lib::http2::curl_resolver_start_callback;
// #[derive(Copy, Clone)]

pub type Curl_http2_dep = crate::src::lib::http2::Curl_http2_dep;
pub type curl_fnmatch_callback = crate::src::lib::http2::curl_fnmatch_callback;
pub type curl_chunk_end_callback = crate::src::lib::http2::curl_chunk_end_callback;
pub type curl_chunk_bgn_callback = crate::src::lib::http2::curl_chunk_bgn_callback;
pub type Curl_RtspReq = crate::src::lib::http2::Curl_RtspReq;
pub const RTSPREQ_LAST: Curl_RtspReq = 12;
pub const RTSPREQ_RECEIVE: Curl_RtspReq = 11;
pub const RTSPREQ_RECORD: Curl_RtspReq = 10;
pub const RTSPREQ_SET_PARAMETER: Curl_RtspReq = 9;
pub const RTSPREQ_GET_PARAMETER: Curl_RtspReq = 8;
pub const RTSPREQ_TEARDOWN: Curl_RtspReq = 7;
pub const RTSPREQ_PAUSE: Curl_RtspReq = 6;
pub const RTSPREQ_PLAY: Curl_RtspReq = 5;
pub const RTSPREQ_SETUP: Curl_RtspReq = 4;
pub const RTSPREQ_ANNOUNCE: Curl_RtspReq = 3;
pub const RTSPREQ_DESCRIBE: Curl_RtspReq = 2;
pub const RTSPREQ_OPTIONS: Curl_RtspReq = 1;
pub const RTSPREQ_NONE: Curl_RtspReq = 0;
pub type curl_usessl = crate::src::lib::http2::curl_usessl;
pub const CURLUSESSL_LAST: curl_usessl = 4;
pub const CURLUSESSL_ALL: curl_usessl = 3;
pub const CURLUSESSL_CONTROL: curl_usessl = 2;
pub const CURLUSESSL_TRY: curl_usessl = 1;
pub const CURLUSESSL_NONE: curl_usessl = 0;
pub type CURL_NETRC_OPTION = crate::src::lib::http2::CURL_NETRC_OPTION;
pub const CURL_NETRC_LAST: CURL_NETRC_OPTION = 3;
pub const CURL_NETRC_REQUIRED: CURL_NETRC_OPTION = 2;
pub const CURL_NETRC_OPTIONAL: CURL_NETRC_OPTION = 1;
pub const CURL_NETRC_IGNORED: CURL_NETRC_OPTION = 0;
pub type curl_sshkeycallback = crate::src::lib::http2::curl_sshkeycallback;
pub type curl_khmatch = crate::src::lib::http2::curl_khmatch;
pub const CURLKHMATCH_LAST: curl_khmatch = 3;
pub const CURLKHMATCH_MISSING: curl_khmatch = 2;
pub const CURLKHMATCH_MISMATCH: curl_khmatch = 1;
pub const CURLKHMATCH_OK: curl_khmatch = 0;
// #[derive(Copy, Clone)]

pub type curl_khkey = crate::src::lib::http2::curl_khkey;
pub type curl_khtype = crate::src::lib::http2::curl_khtype;
pub const CURLKHTYPE_ED25519: curl_khtype = 5;
pub const CURLKHTYPE_ECDSA: curl_khtype = 4;
pub const CURLKHTYPE_DSS: curl_khtype = 3;
pub const CURLKHTYPE_RSA: curl_khtype = 2;
pub const CURLKHTYPE_RSA1: curl_khtype = 1;
pub const CURLKHTYPE_UNKNOWN: curl_khtype = 0;
pub type CURL = crate::src::lib::http2::CURL;
pub type curl_ftpccc = crate::src::lib::http2::curl_ftpccc;
pub const CURLFTPSSL_CCC_LAST: curl_ftpccc = 3;
pub const CURLFTPSSL_CCC_ACTIVE: curl_ftpccc = 2;
pub const CURLFTPSSL_CCC_PASSIVE: curl_ftpccc = 1;
pub const CURLFTPSSL_CCC_NONE: curl_ftpccc = 0;
pub type curl_ftpauth = crate::src::lib::http2::curl_ftpauth;
pub const CURLFTPAUTH_LAST: curl_ftpauth = 3;
pub const CURLFTPAUTH_TLS: curl_ftpauth = 2;
pub const CURLFTPAUTH_SSL: curl_ftpauth = 1;
pub const CURLFTPAUTH_DEFAULT: curl_ftpauth = 0;
pub type curl_ftpfile = crate::src::lib::http2::curl_ftpfile;
pub const FTPFILE_SINGLECWD: curl_ftpfile = 3;
pub const FTPFILE_NOCWD: curl_ftpfile = 2;
pub const FTPFILE_MULTICWD: curl_ftpfile = 1;
// #[derive(Copy, Clone)]

pub type ssl_general_config = crate::src::lib::http2::ssl_general_config;
// #[derive(Copy, Clone, BitfieldStruct)]

pub type ssl_config_data = crate::src::lib::http2::ssl_config_data;
pub type CURL_TLSAUTH = crate::src::lib::http2::CURL_TLSAUTH;
pub const CURL_TLSAUTH_LAST: CURL_TLSAUTH = 2;
pub const CURL_TLSAUTH_SRP: CURL_TLSAUTH = 1;
pub const CURL_TLSAUTH_NONE: CURL_TLSAUTH = 0;
pub type curl_ssl_ctx_callback = crate::src::lib::http2::curl_ssl_ctx_callback;
pub type curl_proxytype = crate::src::lib::http2::curl_proxytype;
pub const CURLPROXY_SOCKS5_HOSTNAME: curl_proxytype = 7;
pub const CURLPROXY_SOCKS4A: curl_proxytype = 6;
pub const CURLPROXY_SOCKS5: curl_proxytype = 5;
pub const CURLPROXY_SOCKS4: curl_proxytype = 4;
pub const CURLPROXY_HTTPS: curl_proxytype = 2;
pub const CURLPROXY_HTTP_1_0: curl_proxytype = 1;
pub const CURLPROXY_HTTP: curl_proxytype = 0;
pub type curl_TimeCond = crate::src::lib::http2::curl_TimeCond;
pub const CURL_TIMECOND_LAST: curl_TimeCond = 4;
pub const CURL_TIMECOND_LASTMOD: curl_TimeCond = 3;
pub const CURL_TIMECOND_IFUNMODSINCE: curl_TimeCond = 2;
pub const CURL_TIMECOND_IFMODSINCE: curl_TimeCond = 1;
pub const CURL_TIMECOND_NONE: curl_TimeCond = 0;
// #[derive(Copy, Clone)]

pub type curl_mimepart = crate::src::lib::http2::curl_mimepart;
// #[derive(Copy, Clone)]

pub type mime_encoder_state = crate::src::lib::http2::mime_encoder_state;
// #[derive(Copy, Clone)]

pub type mime_encoder = crate::src::lib::http2::mime_encoder;
// #[derive(Copy, Clone)]

pub type mime_state = crate::src::lib::http2::mime_state;
pub type mimestate = crate::src::lib::http2::mimestate;
pub const MIMESTATE_LAST: mimestate = 9;
pub const MIMESTATE_END: mimestate = 8;
pub const MIMESTATE_CONTENT: mimestate = 7;
pub const MIMESTATE_BOUNDARY2: mimestate = 6;
pub const MIMESTATE_BOUNDARY1: mimestate = 5;
pub const MIMESTATE_BODY: mimestate = 4;
pub const MIMESTATE_EOH: mimestate = 3;
pub const MIMESTATE_USERHEADERS: mimestate = 2;
pub const MIMESTATE_CURLHEADERS: mimestate = 1;
pub const MIMESTATE_BEGIN: mimestate = 0;
pub type curl_free_callback = crate::src::lib::http2::curl_free_callback;
pub type curl_seek_callback = crate::src::lib::http2::curl_seek_callback;
pub type mimekind = crate::src::lib::http2::mimekind;
pub const MIMEKIND_LAST: mimekind = 5;
pub const MIMEKIND_MULTIPART: mimekind = 4;
pub const MIMEKIND_CALLBACK: mimekind = 3;
pub const MIMEKIND_FILE: mimekind = 2;
pub const MIMEKIND_DATA: mimekind = 1;
pub const MIMEKIND_NONE: mimekind = 0;
// #[derive(Copy, Clone)]

pub type curl_mime = crate::src::lib::http2::curl_mime;
// #[derive(Copy, Clone)]

pub type curl_httppost = crate::src::lib::http2::curl_httppost;
pub type curl_hstswrite_callback = crate::src::lib::http2::curl_hstswrite_callback;
// #[derive(Copy, Clone)]

pub type curl_index = crate::src::lib::http2::curl_index;
// #[derive(Copy, Clone, BitfieldStruct)]

pub type curl_hstsentry = crate::src::lib::http2::curl_hstsentry;
pub type CURLSTScode = crate::src::lib::http2::CURLSTScode;
pub const CURLSTS_FAIL: CURLSTScode = 2;
pub const CURLSTS_DONE: CURLSTScode = 1;
pub const CURLSTS_OK: CURLSTScode = 0;
pub type curl_hstsread_callback = crate::src::lib::http2::curl_hstsread_callback;
pub type curl_conv_callback = crate::src::lib::http2::curl_conv_callback;
pub type curl_closesocket_callback = crate::src::lib::http2::curl_closesocket_callback;
pub type curl_socket_t = crate::src::lib::http2::curl_socket_t;
pub type curl_opensocket_callback = crate::src::lib::http2::curl_opensocket_callback;
// #[derive(Copy, Clone)]

pub type curl_sockaddr = crate::src::lib::http2::curl_sockaddr;
pub type curlsocktype = crate::src::lib::http2::curlsocktype;
pub const CURLSOCKTYPE_LAST: curlsocktype = 2;
pub const CURLSOCKTYPE_ACCEPT: curlsocktype = 1;
pub const CURLSOCKTYPE_IPCXN: curlsocktype = 0;
pub type curl_sockopt_callback = crate::src::lib::http2::curl_sockopt_callback;
pub type curl_ioctl_callback = crate::src::lib::http2::curl_ioctl_callback;
pub type curlioerr = crate::src::lib::http2::curlioerr;
pub const CURLIOE_LAST: curlioerr = 3;
pub const CURLIOE_FAILRESTART: curlioerr = 2;
pub const CURLIOE_UNKNOWNCMD: curlioerr = 1;
pub const CURLIOE_OK: curlioerr = 0;
pub type curl_debug_callback = crate::src::lib::http2::curl_debug_callback;
pub type curl_infotype = crate::src::lib::http2::curl_infotype;
pub const CURLINFO_END: curl_infotype = 7;
pub const CURLINFO_SSL_DATA_OUT: curl_infotype = 6;
pub const CURLINFO_SSL_DATA_IN: curl_infotype = 5;
pub const CURLINFO_DATA_OUT: curl_infotype = 4;
pub const CURLINFO_DATA_IN: curl_infotype = 3;
pub const CURLINFO_HEADER_OUT: curl_infotype = 2;
pub const CURLINFO_HEADER_IN: curl_infotype = 1;
pub const CURLINFO_TEXT: curl_infotype = 0;
pub type curl_xferinfo_callback = crate::src::lib::http2::curl_xferinfo_callback;
pub type curl_progress_callback = crate::src::lib::http2::curl_progress_callback;
pub type curl_write_callback = crate::src::lib::http2::curl_write_callback;
// #[derive(Copy, Clone, BitfieldStruct)]

pub type SingleRequest = crate::src::lib::http2::SingleRequest;
// #[derive(Copy, Clone)]

pub type dohdata = crate::src::lib::http2::dohdata;
// #[derive(Copy, Clone)]

pub type dnsprobe = crate::src::lib::http2::dnsprobe;
// #[derive(Copy, Clone)]

pub type C2RustUnnamed = crate::src::lib::http2::C2RustUnnamed;
// #[derive(Copy, Clone)]

pub type SSHPROTO = crate::src::lib::http2::SSHPROTO;
// #[derive(Copy, Clone)]

pub type SMTP = crate::src::lib::http2::SMTP;
pub type curl_pp_transfer = crate::src::lib::http2::curl_pp_transfer;
pub const PPTRANSFER_NONE: curl_pp_transfer = 2;
pub const PPTRANSFER_INFO: curl_pp_transfer = 1;
pub const PPTRANSFER_BODY: curl_pp_transfer = 0;
// #[derive(Copy, Clone)]

pub type RTSP = crate::src::lib::http2::RTSP;
// #[derive(Copy, Clone)]

pub type HTTP = crate::src::lib::http2::HTTP;
pub type uint8_t = crate::src::lib::http2::uint8_t;
pub type uint32_t = crate::src::lib::http2::uint32_t;
pub type C2RustUnnamed_0 = u32;
pub const HTTPSEND_BODY: C2RustUnnamed_0 = 2;
pub const HTTPSEND_REQUEST: C2RustUnnamed_0 = 1;
pub const HTTPSEND_NADA: C2RustUnnamed_0 = 0;
// #[derive(Copy, Clone)]

pub type back = crate::src::lib::http2::back;
// #[derive(Copy, Clone)]

pub type POP3 = crate::src::lib::http2::POP3;
// #[derive(Copy, Clone)]

pub type MQTT = crate::src::lib::http2::MQTT;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ldapreqinfo {
    pub msgid: i32,
    pub nument: i32,
}
// #[derive(Copy, Clone)]

pub type IMAP = crate::src::lib::http2::IMAP;
// #[derive(Copy, Clone)]

pub type FTP = crate::src::lib::http2::FTP;
// #[derive(Copy, Clone)]

pub type FILEPROTO = crate::src::lib::http2::FILEPROTO;
pub type upgrade101 = crate::src::lib::http2::upgrade101;
pub const UPGR101_WORKING: upgrade101 = 3;
pub const UPGR101_RECEIVED: upgrade101 = 2;
pub const UPGR101_REQUESTED: upgrade101 = 1;
pub const UPGR101_INIT: upgrade101 = 0;
pub type expect100 = crate::src::lib::http2::expect100;
pub const EXP100_FAILED: expect100 = 3;
pub const EXP100_SENDING_REQUEST: expect100 = 2;
pub const EXP100_AWAITING_CONTINUE: expect100 = 1;
pub const EXP100_SEND_DATA: expect100 = 0;
pub type C2RustUnnamed_1 = u32;
pub const HEADER_ALLBAD: C2RustUnnamed_1 = 2;
pub const HEADER_PARTHEADER: C2RustUnnamed_1 = 1;
pub const HEADER_NORMAL: C2RustUnnamed_1 = 0;
// #[derive(Copy, Clone)]

pub type PslCache = crate::src::lib::http2::PslCache;
pub type psl_ctx_t = crate::src::lib::http2::psl_ctx_t;
// #[derive(Copy, Clone)]

pub type Curl_multi = crate::src::lib::http2::Curl_multi;
pub type curl_multi_timer_callback = crate::src::lib::http2::curl_multi_timer_callback;
pub type CURLM = crate::src::lib::http2::CURLM;
pub type curl_push_callback = crate::src::lib::http2::curl_push_callback;
pub type curl_socket_callback = crate::src::lib::http2::curl_socket_callback;
// #[derive(Copy, Clone)]

pub type Names = crate::src::lib::http2::Names;
pub type C2RustUnnamed_2 = u32;
pub const HCACHE_SHARED: C2RustUnnamed_2 = 2;
pub const HCACHE_MULTI: C2RustUnnamed_2 = 1;
pub const HCACHE_NONE: C2RustUnnamed_2 = 0;
// #[derive(Copy, Clone)]

pub type Curl_message = crate::src::lib::http2::Curl_message;
// #[derive(Copy, Clone)]

pub type CURLMsg = crate::src::lib::http2::CURLMsg;
// #[derive(Copy, Clone)]

pub type C2RustUnnamed_3 = crate::src::lib::http2::C2RustUnnamed_3;
pub type CURLMSG = crate::src::lib::http2::CURLMSG;
pub const CURLMSG_LAST: CURLMSG = 2;
pub const CURLMSG_DONE: CURLMSG = 1;
pub const CURLMSG_NONE: CURLMSG = 0;
pub type CURLMstate = crate::src::lib::http2::CURLMstate;
pub const MSTATE_LAST: CURLMstate = 17;
pub const MSTATE_MSGSENT: CURLMstate = 16;
pub const MSTATE_COMPLETED: CURLMstate = 15;
pub const MSTATE_DONE: CURLMstate = 14;
pub const MSTATE_RATELIMITING: CURLMstate = 13;
pub const MSTATE_PERFORMING: CURLMstate = 12;
pub const MSTATE_DID: CURLMstate = 11;
pub const MSTATE_DOING_MORE: CURLMstate = 10;
pub const MSTATE_DOING: CURLMstate = 9;
pub const MSTATE_DO: CURLMstate = 8;
pub const MSTATE_PROTOCONNECTING: CURLMstate = 7;
pub const MSTATE_PROTOCONNECT: CURLMstate = 6;
pub const MSTATE_TUNNELING: CURLMstate = 5;
pub const MSTATE_CONNECTING: CURLMstate = 4;
pub const MSTATE_RESOLVING: CURLMstate = 3;
pub const MSTATE_CONNECT: CURLMstate = 2;
pub const MSTATE_PENDING: CURLMstate = 1;
pub const MSTATE_INIT: CURLMstate = 0;
// #[derive(Copy, Clone)]

pub type connectdata = crate::src::lib::http2::connectdata;
// #[derive(Copy, Clone)]

pub type connectbundle = crate::src::lib::http2::connectbundle;
// #[derive(Copy, Clone)]

pub type C2RustUnnamed_4 = crate::src::lib::http2::C2RustUnnamed_4;
// #[derive(Copy, Clone)]

pub type mqtt_conn = crate::src::lib::http2::mqtt_conn;
pub type mqttstate = crate::src::lib::http2::mqttstate;
pub const MQTT_NOSTATE: mqttstate = 7;
pub const MQTT_PUB_REMAIN: mqttstate = 6;
pub const MQTT_PUBWAIT: mqttstate = 5;
pub const MQTT_SUBACK_COMING: mqttstate = 4;
pub const MQTT_SUBACK: mqttstate = 3;
pub const MQTT_CONNACK: mqttstate = 2;
pub const MQTT_REMAINING_LENGTH: mqttstate = 1;
pub const MQTT_FIRST: mqttstate = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ldapconninfo {
    pub ld: *mut LDAP,
    pub recv: Option::<Curl_recv>,
    pub send: Option::<Curl_send>,
    pub proto: i32,
    pub msgid: i32,
    pub ssldone: bool,
    pub sslinst: bool,
    pub didbind: bool,
}
pub type Curl_send = crate::src::lib::http2::Curl_send;
pub type Curl_recv = crate::src::lib::http2::Curl_recv;
pub type LDAP = ldap;
// #[derive(Copy, Clone)]

pub type smb_conn = crate::src::lib::http2::smb_conn;
pub type smb_conn_state = crate::src::lib::http2::smb_conn_state;
pub const SMB_CONNECTED: smb_conn_state = 4;
pub const SMB_SETUP: smb_conn_state = 3;
pub const SMB_NEGOTIATE: smb_conn_state = 2;
pub const SMB_CONNECTING: smb_conn_state = 1;
pub const SMB_NOT_CONNECTED: smb_conn_state = 0;
// #[derive(Copy, Clone)]

pub type rtsp_conn = crate::src::lib::http2::rtsp_conn;
// #[derive(Copy, Clone)]

pub type smtp_conn = crate::src::lib::http2::smtp_conn;
// #[derive(Copy, Clone)]

pub type SASL = crate::src::lib::http2::SASL;
pub type saslstate = crate::src::lib::http2::saslstate;
pub const SASL_FINAL: saslstate = 17;
pub const SASL_CANCEL: saslstate = 16;
pub const SASL_GSASL: saslstate = 15;
pub const SASL_OAUTH2_RESP: saslstate = 14;
pub const SASL_OAUTH2: saslstate = 13;
pub const SASL_GSSAPI_NO_DATA: saslstate = 12;
pub const SASL_GSSAPI_TOKEN: saslstate = 11;
pub const SASL_GSSAPI: saslstate = 10;
pub const SASL_NTLM_TYPE2MSG: saslstate = 9;
pub const SASL_NTLM: saslstate = 8;
pub const SASL_DIGESTMD5_RESP: saslstate = 7;
pub const SASL_DIGESTMD5: saslstate = 6;
pub const SASL_CRAMMD5: saslstate = 5;
pub const SASL_EXTERNAL: saslstate = 4;
pub const SASL_LOGIN_PASSWD: saslstate = 3;
pub const SASL_LOGIN: saslstate = 2;
pub const SASL_PLAIN: saslstate = 1;
pub const SASL_STOP: saslstate = 0;
// #[derive(Copy, Clone)]

pub type SASLproto = crate::src::lib::http2::SASLproto;
pub type smtpstate = crate::src::lib::http2::smtpstate;
pub const SMTP_LAST: smtpstate = 13;
pub const SMTP_QUIT: smtpstate = 12;
pub const SMTP_POSTDATA: smtpstate = 11;
pub const SMTP_DATA: smtpstate = 10;
pub const SMTP_RCPT: smtpstate = 9;
pub const SMTP_MAIL: smtpstate = 8;
pub const SMTP_COMMAND: smtpstate = 7;
pub const SMTP_AUTH: smtpstate = 6;
pub const SMTP_UPGRADETLS: smtpstate = 5;
pub const SMTP_STARTTLS: smtpstate = 4;
pub const SMTP_HELO: smtpstate = 3;
pub const SMTP_EHLO: smtpstate = 2;
pub const SMTP_SERVERGREET: smtpstate = 1;
pub const SMTP_STOP: smtpstate = 0;
// #[derive(Copy, Clone)]

pub type pingpong = crate::src::lib::http2::pingpong;
// #[derive(Copy, Clone)]

pub type pop3_conn = crate::src::lib::http2::pop3_conn;
pub type pop3state = crate::src::lib::http2::pop3state;
pub const POP3_LAST: pop3state = 11;
pub const POP3_QUIT: pop3state = 10;
pub const POP3_COMMAND: pop3state = 9;
pub const POP3_PASS: pop3state = 8;
pub const POP3_USER: pop3state = 7;
pub const POP3_APOP: pop3state = 6;
pub const POP3_AUTH: pop3state = 5;
pub const POP3_UPGRADETLS: pop3state = 4;
pub const POP3_STARTTLS: pop3state = 3;
pub const POP3_CAPA: pop3state = 2;
pub const POP3_SERVERGREET: pop3state = 1;
pub const POP3_STOP: pop3state = 0;
// #[derive(Copy, Clone)]

pub type imap_conn = crate::src::lib::http2::imap_conn;
pub type imapstate = crate::src::lib::http2::imapstate;
pub const IMAP_LAST: imapstate = 15;
pub const IMAP_LOGOUT: imapstate = 14;
pub const IMAP_SEARCH: imapstate = 13;
pub const IMAP_APPEND_FINAL: imapstate = 12;
pub const IMAP_APPEND: imapstate = 11;
pub const IMAP_FETCH_FINAL: imapstate = 10;
pub const IMAP_FETCH: imapstate = 9;
pub const IMAP_SELECT: imapstate = 8;
pub const IMAP_LIST: imapstate = 7;
pub const IMAP_LOGIN: imapstate = 6;
pub const IMAP_AUTHENTICATE: imapstate = 5;
pub const IMAP_UPGRADETLS: imapstate = 4;
pub const IMAP_STARTTLS: imapstate = 3;
pub const IMAP_CAPABILITY: imapstate = 2;
pub const IMAP_SERVERGREET: imapstate = 1;
pub const IMAP_STOP: imapstate = 0;
// #[derive(Copy, Clone)]

pub type ssh_conn = crate::src::lib::http2::ssh_conn;
pub type sshstate = crate::src::lib::http2::sshstate;
pub const SSH_LAST: sshstate = 60;
pub const SSH_QUIT: sshstate = 59;
pub const SSH_SESSION_FREE: sshstate = 58;
pub const SSH_SESSION_DISCONNECT: sshstate = 57;
pub const SSH_SCP_CHANNEL_FREE: sshstate = 56;
pub const SSH_SCP_WAIT_CLOSE: sshstate = 55;
pub const SSH_SCP_WAIT_EOF: sshstate = 54;
pub const SSH_SCP_SEND_EOF: sshstate = 53;
pub const SSH_SCP_DONE: sshstate = 52;
pub const SSH_SCP_DOWNLOAD: sshstate = 51;
pub const SSH_SCP_DOWNLOAD_INIT: sshstate = 50;
pub const SSH_SCP_UPLOAD_INIT: sshstate = 49;
pub const SSH_SCP_TRANS_INIT: sshstate = 48;
pub const SSH_SFTP_SHUTDOWN: sshstate = 47;
pub const SSH_SFTP_CLOSE: sshstate = 46;
pub const SSH_SFTP_DOWNLOAD_STAT: sshstate = 45;
pub const SSH_SFTP_DOWNLOAD_INIT: sshstate = 44;
pub const SSH_SFTP_READDIR_DONE: sshstate = 43;
pub const SSH_SFTP_READDIR_BOTTOM: sshstate = 42;
pub const SSH_SFTP_READDIR_LINK: sshstate = 41;
pub const SSH_SFTP_READDIR: sshstate = 40;
pub const SSH_SFTP_READDIR_INIT: sshstate = 39;
pub const SSH_SFTP_CREATE_DIRS_MKDIR: sshstate = 38;
pub const SSH_SFTP_CREATE_DIRS: sshstate = 37;
pub const SSH_SFTP_CREATE_DIRS_INIT: sshstate = 36;
pub const SSH_SFTP_UPLOAD_INIT: sshstate = 35;
pub const SSH_SFTP_TRANS_INIT: sshstate = 34;
pub const SSH_SFTP_FILETIME: sshstate = 33;
pub const SSH_SFTP_GETINFO: sshstate = 32;
pub const SSH_SFTP_QUOTE_STATVFS: sshstate = 31;
pub const SSH_SFTP_QUOTE_UNLINK: sshstate = 30;
pub const SSH_SFTP_QUOTE_RMDIR: sshstate = 29;
pub const SSH_SFTP_QUOTE_RENAME: sshstate = 28;
pub const SSH_SFTP_QUOTE_MKDIR: sshstate = 27;
pub const SSH_SFTP_QUOTE_SYMLINK: sshstate = 26;
pub const SSH_SFTP_QUOTE_SETSTAT: sshstate = 25;
pub const SSH_SFTP_QUOTE_STAT: sshstate = 24;
pub const SSH_SFTP_NEXT_QUOTE: sshstate = 23;
pub const SSH_SFTP_QUOTE: sshstate = 22;
pub const SSH_SFTP_POSTQUOTE_INIT: sshstate = 21;
pub const SSH_SFTP_QUOTE_INIT: sshstate = 20;
pub const SSH_SFTP_REALPATH: sshstate = 19;
pub const SSH_SFTP_INIT: sshstate = 18;
pub const SSH_AUTH_DONE: sshstate = 17;
pub const SSH_AUTH_GSSAPI: sshstate = 16;
pub const SSH_AUTH_KEY: sshstate = 15;
pub const SSH_AUTH_KEY_INIT: sshstate = 14;
pub const SSH_AUTH_HOST: sshstate = 13;
pub const SSH_AUTH_HOST_INIT: sshstate = 12;
pub const SSH_AUTH_AGENT: sshstate = 11;
pub const SSH_AUTH_AGENT_LIST: sshstate = 10;
pub const SSH_AUTH_AGENT_INIT: sshstate = 9;
pub const SSH_AUTH_PASS: sshstate = 8;
pub const SSH_AUTH_PASS_INIT: sshstate = 7;
pub const SSH_AUTH_PKEY: sshstate = 6;
pub const SSH_AUTH_PKEY_INIT: sshstate = 5;
pub const SSH_AUTHLIST: sshstate = 4;
pub const SSH_HOSTKEY: sshstate = 3;
pub const SSH_S_STARTUP: sshstate = 2;
pub const SSH_INIT: sshstate = 1;
pub const SSH_STOP: sshstate = 0;
pub const SSH_NO_STATE: sshstate = -1;
// #[derive(Copy, Clone)]

pub type http_conn = crate::src::lib::http2::http_conn;
// #[derive(Copy, Clone)]

pub type nghttp2_settings_entry = crate::src::lib::http2::nghttp2_settings_entry;
// #[derive(Copy, Clone)]

pub type h2settings = crate::src::lib::http2::h2settings;
// #[derive(Copy, Clone)]

pub type ftp_conn = crate::src::lib::http2::ftp_conn;
pub type ftpstate = crate::src::lib::http2::ftpstate;
pub const FTP_LAST: ftpstate = 35;
pub const FTP_QUIT: ftpstate = 34;
pub const FTP_STOR: ftpstate = 33;
pub const FTP_RETR: ftpstate = 32;
pub const FTP_LIST: ftpstate = 31;
pub const FTP_PASV: ftpstate = 30;
pub const FTP_PRET: ftpstate = 29;
pub const FTP_PORT: ftpstate = 28;
pub const FTP_RETR_REST: ftpstate = 27;
pub const FTP_REST: ftpstate = 26;
pub const FTP_STOR_SIZE: ftpstate = 25;
pub const FTP_RETR_SIZE: ftpstate = 24;
pub const FTP_SIZE: ftpstate = 23;
pub const FTP_STOR_TYPE: ftpstate = 22;
pub const FTP_RETR_TYPE: ftpstate = 21;
pub const FTP_LIST_TYPE: ftpstate = 20;
pub const FTP_TYPE: ftpstate = 19;
pub const FTP_MDTM: ftpstate = 18;
pub const FTP_MKD: ftpstate = 17;
pub const FTP_CWD: ftpstate = 16;
pub const FTP_POSTQUOTE: ftpstate = 15;
pub const FTP_STOR_PREQUOTE: ftpstate = 14;
pub const FTP_RETR_PREQUOTE: ftpstate = 13;
pub const FTP_QUOTE: ftpstate = 12;
pub const FTP_NAMEFMT: ftpstate = 11;
pub const FTP_SYST: ftpstate = 10;
pub const FTP_PWD: ftpstate = 9;
pub const FTP_CCC: ftpstate = 8;
pub const FTP_PROT: ftpstate = 7;
pub const FTP_PBSZ: ftpstate = 6;
pub const FTP_ACCT: ftpstate = 5;
pub const FTP_PASS: ftpstate = 4;
pub const FTP_USER: ftpstate = 3;
pub const FTP_AUTH: ftpstate = 2;
pub const FTP_WAIT220: ftpstate = 1;
pub const FTP_STOP: ftpstate = 0;
// #[derive(Copy, Clone)]

pub type ntlmdata = crate::src::lib::http2::ntlmdata;
pub type curlntlm = crate::src::lib::http2::curlntlm;
pub const NTLMSTATE_LAST: curlntlm = 4;
pub const NTLMSTATE_TYPE3: curlntlm = 3;
pub const NTLMSTATE_TYPE2: curlntlm = 2;
pub const NTLMSTATE_TYPE1: curlntlm = 1;
pub const NTLMSTATE_NONE: curlntlm = 0;
// #[derive(Copy, Clone)]

pub type gsasldata = crate::src::lib::http2::gsasldata;
// #[derive(Copy, Clone)]

pub type Curl_handler = crate::src::lib::http2::Curl_handler;
// #[derive(Copy, Clone, BitfieldStruct)]

pub type ConnectBits = crate::src::lib::http2::ConnectBits;
// #[derive(Copy, Clone, BitfieldStruct)]

pub type ssl_connect_data = crate::src::lib::http2::ssl_connect_data;
pub type ssl_connect_state = crate::src::lib::http2::ssl_connect_state;
pub const ssl_connect_done: ssl_connect_state = 5;
pub const ssl_connect_3: ssl_connect_state = 4;
pub const ssl_connect_2_writing: ssl_connect_state = 3;
pub const ssl_connect_2_reading: ssl_connect_state = 2;
pub const ssl_connect_2: ssl_connect_state = 1;
pub const ssl_connect_1: ssl_connect_state = 0;
pub type ssl_connection_state = crate::src::lib::http2::ssl_connection_state;
pub const ssl_connection_complete: ssl_connection_state = 2;
pub const ssl_connection_negotiating: ssl_connection_state = 1;
pub const ssl_connection_none: ssl_connection_state = 0;
// #[derive(Copy, Clone)]

pub type proxy_info = crate::src::lib::http2::proxy_info;
// #[derive(Copy, Clone)]

pub type hostname = crate::src::lib::http2::hostname;
pub type C2RustUnnamed_5 = u32;
pub const TRNSPRT_QUIC: C2RustUnnamed_5 = 5;
pub const TRNSPRT_UDP: C2RustUnnamed_5 = 4;
pub const TRNSPRT_TCP: C2RustUnnamed_5 = 3;
// #[derive(Copy, Clone)]

pub type Curl_chunker = crate::src::lib::http2::Curl_chunker;
pub type ChunkyState = crate::src::lib::http2::ChunkyState;
pub const CHUNK_TRAILER_POSTCR: ChunkyState = 7;
pub const CHUNK_TRAILER_CR: ChunkyState = 6;
pub const CHUNK_TRAILER: ChunkyState = 5;
pub const CHUNK_STOP: ChunkyState = 4;
pub const CHUNK_POSTLF: ChunkyState = 3;
pub const CHUNK_DATA: ChunkyState = 2;
pub const CHUNK_LF: ChunkyState = 1;
pub const CHUNK_HEX: ChunkyState = 0;
// #[derive(Copy, Clone)]

pub type connstate = crate::src::lib::http2::connstate;
pub type connect_t = crate::src::lib::http2::connect_t;
pub const CONNECT_DONE: connect_t = 17;
pub const CONNECT_REQ_READ_MORE: connect_t = 16;
pub const CONNECT_REQ_READ: connect_t = 15;
pub const CONNECT_REQ_SENDING: connect_t = 14;
pub const CONNECT_REQ_SEND: connect_t = 13;
pub const CONNECT_RESOLVE_REMOTE: connect_t = 12;
pub const CONNECT_RESOLVED: connect_t = 11;
pub const CONNECT_RESOLVING: connect_t = 10;
pub const CONNECT_REQ_INIT: connect_t = 9;
pub const CONNECT_AUTH_READ: connect_t = 8;
pub const CONNECT_AUTH_SEND: connect_t = 7;
pub const CONNECT_AUTH_INIT: connect_t = 6;
pub const CONNECT_GSSAPI_INIT: connect_t = 5;
pub const CONNECT_SOCKS_READ: connect_t = 4;
pub const CONNECT_SOCKS_READ_INIT: connect_t = 3;
pub const CONNECT_SOCKS_SEND: connect_t = 2;
pub const CONNECT_SOCKS_INIT: connect_t = 1;
pub const CONNECT_INIT: connect_t = 0;
pub type curl_calloc_callback = crate::src::lib::http2::curl_calloc_callback;
pub type ber_tag_t = u64;
pub type ber_socket_t = i32;
pub type ber_len_t = u64;
pub type ber_slen_t = i64;
pub type BerElement = berelement;
pub type Sockbuf = sockbuf;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockbuf_io {
    pub sbi_setup: Option::<
        unsafe extern "C" fn(*mut Sockbuf_IO_Desc, *mut libc::c_void) -> i32,
    >,
    pub sbi_remove: Option::<unsafe extern "C" fn(*mut Sockbuf_IO_Desc) -> i32>,
    pub sbi_ctrl: Option::<
        unsafe extern "C" fn(
            *mut Sockbuf_IO_Desc,
            i32,
            *mut libc::c_void,
        ) -> i32,
    >,
    pub sbi_read: Option::<
        unsafe extern "C" fn(
            *mut Sockbuf_IO_Desc,
            *mut libc::c_void,
            ber_len_t,
        ) -> ber_slen_t,
    >,
    pub sbi_write: Option::<
        unsafe extern "C" fn(
            *mut Sockbuf_IO_Desc,
            *mut libc::c_void,
            ber_len_t,
        ) -> ber_slen_t,
    >,
    pub sbi_close: Option::<unsafe extern "C" fn(*mut Sockbuf_IO_Desc) -> i32>,
}
pub type Sockbuf_IO_Desc = sockbuf_io_desc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockbuf_io_desc {
    pub sbiod_level: i32,
    pub sbiod_sb: *mut Sockbuf,
    pub sbiod_io: *mut Sockbuf_IO,
    pub sbiod_pvt: *mut libc::c_void,
    pub sbiod_next: *mut sockbuf_io_desc,
}
pub type Sockbuf_IO = sockbuf_io;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct berval {
    pub bv_len: ber_len_t,
    pub bv_val: *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ldapcontrol {
    pub ldctl_oid: *mut i8,
    pub ldctl_value: berval,
    pub ldctl_iscritical: i8,
}
pub type LDAPControl = ldapcontrol;
pub type LDAPMessage = ldapmsg;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ldap_url_desc {
    pub lud_next: *mut ldap_url_desc,
    pub lud_scheme: *mut i8,
    pub lud_host: *mut i8,
    pub lud_port: i32,
    pub lud_dn: *mut i8,
    pub lud_attrs: *mut *mut i8,
    pub lud_scope: i32,
    pub lud_filter: *mut i8,
    pub lud_exts: *mut *mut i8,
    pub lud_crit_exts: i32,
}
pub type LDAPURLDesc = ldap_url_desc;
#[no_mangle]
pub static mut Curl_handler_ldap: Curl_handler = unsafe {
    {
        let mut init = Curl_handler {
            scheme: b"LDAP\0" as *const u8 as *const i8,
            setup_connection: Some(
                oldap_setup_connection
                    as unsafe extern "C" fn(*mut Curl_easy, *mut connectdata) -> CURLcode,
            ),
            do_it: Some(
                oldap_do as unsafe extern "C" fn(*mut Curl_easy, *mut bool) -> CURLcode,
            ),
            done: Some(
                oldap_done
                    as unsafe extern "C" fn(*mut Curl_easy, CURLcode, bool) -> CURLcode,
            ),
            do_more: None,
            connect_it: Some(
                oldap_connect
                    as unsafe extern "C" fn(*mut Curl_easy, *mut bool) -> CURLcode,
            ),
            connecting: Some(
                oldap_connecting
                    as unsafe extern "C" fn(*mut Curl_easy, *mut bool) -> CURLcode,
            ),
            doing: None,
            proto_getsock: None,
            doing_getsock: None,
            domore_getsock: None,
            perform_getsock: None,
            disconnect: Some(
                oldap_disconnect
                    as unsafe extern "C" fn(
                        *mut Curl_easy,
                        *mut connectdata,
                        bool,
                    ) -> CURLcode,
            ),
            readwrite: None,
            connection_check: None,
            attach: None,
            defport: 389 as i32,
            protocol: ((1 as i32) << 7 as i32) as u32,
            family: ((1 as i32) << 7 as i32) as u32,
            flags: 0 as i32 as u32,
        };
        init
    }
};
#[no_mangle]
pub static mut Curl_handler_ldaps: Curl_handler = unsafe {
    {
        let mut init = Curl_handler {
            scheme: b"LDAPS\0" as *const u8 as *const i8,
            setup_connection: Some(
                oldap_setup_connection
                    as unsafe extern "C" fn(*mut Curl_easy, *mut connectdata) -> CURLcode,
            ),
            do_it: Some(
                oldap_do as unsafe extern "C" fn(*mut Curl_easy, *mut bool) -> CURLcode,
            ),
            done: Some(
                oldap_done
                    as unsafe extern "C" fn(*mut Curl_easy, CURLcode, bool) -> CURLcode,
            ),
            do_more: None,
            connect_it: Some(
                oldap_connect
                    as unsafe extern "C" fn(*mut Curl_easy, *mut bool) -> CURLcode,
            ),
            connecting: Some(
                oldap_connecting
                    as unsafe extern "C" fn(*mut Curl_easy, *mut bool) -> CURLcode,
            ),
            doing: None,
            proto_getsock: None,
            doing_getsock: None,
            domore_getsock: None,
            perform_getsock: None,
            disconnect: Some(
                oldap_disconnect
                    as unsafe extern "C" fn(
                        *mut Curl_easy,
                        *mut connectdata,
                        bool,
                    ) -> CURLcode,
            ),
            readwrite: None,
            connection_check: None,
            attach: None,
            defport: 636 as i32,
            protocol: ((1 as i32) << 8 as i32) as u32,
            family: ((1 as i32) << 7 as i32) as u32,
            flags: ((1 as i32) << 0 as i32) as u32,
        };
        init
    }
};
static mut url_errs: [*const i8; 11] = [
    b"success\0" as *const u8 as *const i8,
    b"out of memory\0" as *const u8 as *const i8,
    b"bad parameter\0" as *const u8 as *const i8,
    b"unrecognized scheme\0" as *const u8 as *const i8,
    b"unbalanced delimiter\0" as *const u8 as *const i8,
    b"bad URL\0" as *const u8 as *const i8,
    b"bad host or port\0" as *const u8 as *const i8,
    b"bad or missing attributes\0" as *const u8 as *const i8,
    b"bad or missing scope\0" as *const u8 as *const i8,
    b"bad or missing filter\0" as *const u8 as *const i8,
    b"bad or missing extensions\0" as *const u8 as *const i8,
];
unsafe extern "C" fn oldap_setup_connection(
    mut data: *mut Curl_easy,
    mut conn: *mut connectdata,
) -> CURLcode {
    let mut li: *mut ldapconninfo = 0 as *mut ldapconninfo;
    let mut lud: *mut LDAPURLDesc = 0 as *mut LDAPURLDesc;
    let mut rc: i32 = 0;
    let mut proto: i32 = 0;
    let mut status: CURLcode = CURLE_OK;
    rc = ldap_url_parse((*data).state.url, &mut lud);
    if rc != 0 as i32 {
        let mut msg: *const i8 = b"url parsing problem\0" as *const u8
            as *const i8;
        status = CURLE_URL_MALFORMAT;
        if rc > 0 as i32 && rc <= 0xa as i32 {
            if rc == 0x1 as i32 {
                status = CURLE_OUT_OF_MEMORY;
            }
            msg = url_errs[rc as usize];
        }
        Curl_failf(data, b"LDAP local: %s\0" as *const u8 as *const i8, msg);
        return status;
    }
    proto = ldap_pvt_url_scheme2proto((*lud).lud_scheme);
    ldap_free_urldesc(lud);
    li = Curl_ccalloc
        .expect(
            "non-null function pointer",
        )(
        1 as i32 as size_t,
        ::std::mem::size_of::<ldapconninfo>() as u64,
    ) as *mut ldapconninfo;
    if li.is_null() {
        return CURLE_OUT_OF_MEMORY;
    }
    (*li).proto = proto;
    let ref mut fresh0 = (*conn).proto.ldapc;
    *fresh0 = li;
    Curl_conncontrol(conn, 0 as i32);
    return CURLE_OK;
}
unsafe extern "C" fn oldap_connect(
    mut data: *mut Curl_easy,
    mut done: *mut bool,
) -> CURLcode {
    let mut conn: *mut connectdata = (*data).conn;
    let mut li: *mut ldapconninfo = (*conn).proto.ldapc;
    let mut rc: i32 = 0;
    let mut proto: i32 = 3 as i32;
    let mut hosturl: [i8; 1024] = [0; 1024];
    let mut ptr: *mut i8 = 0 as *mut i8;
    strcpy(hosturl.as_mut_ptr(), b"ldap\0" as *const u8 as *const i8);
    ptr = hosturl.as_mut_ptr().offset(4 as i32 as isize);
    if (*(*conn).handler).flags
        & ((1 as i32) << 0 as i32) as u32 != 0
    {
        let fresh1 = ptr;
        ptr = ptr.offset(1);
        *fresh1 = 's' as i32 as i8;
    }
    curl_msnprintf(
        ptr,
        (::std::mem::size_of::<[i8; 1024]>() as u64)
            .wrapping_sub(
                ptr.offset_from(hosturl.as_mut_ptr()) as i64 as u64,
            ),
        b"://%s:%d\0" as *const u8 as *const i8,
        (*conn).host.name,
        (*conn).remote_port,
    );
    rc = ldap_init_fd(
        (*conn).sock[0 as i32 as usize],
        (*li).proto,
        hosturl.as_mut_ptr(),
        &mut (*li).ld,
    );
    if rc != 0 {
        Curl_failf(
            data,
            b"LDAP local: Cannot connect to %s, %s\0" as *const u8
                as *const i8,
            hosturl.as_mut_ptr(),
            ldap_err2string(rc),
        );
        return CURLE_COULDNT_CONNECT;
    }
    ldap_set_option(
        (*li).ld,
        0x11 as i32,
        &mut proto as *mut i32 as *const libc::c_void,
    );
    if (*(*conn).handler).flags
        & ((1 as i32) << 0 as i32) as u32 != 0
    {
        let mut result: CURLcode = CURLE_OK;
        result = Curl_ssl_connect_nonblocking(
            data,
            conn,
            0 as i32 != 0,
            0 as i32,
            &mut (*li).ssldone,
        );
        if result as u64 != 0 {
            return result;
        }
    }
    return CURLE_OK;
}
unsafe extern "C" fn oldap_connecting(
    mut data: *mut Curl_easy,
    mut done: *mut bool,
) -> CURLcode {
    let mut conn: *mut connectdata = (*data).conn;
    let mut li: *mut ldapconninfo = (*conn).proto.ldapc;
    let mut msg: *mut LDAPMessage = 0 as *mut LDAPMessage;
    let mut tv: timeval = {
        let mut init = timeval {
            tv_sec: 0 as i32 as __time_t,
            tv_usec: 1 as i32 as __suseconds_t,
        };
        init
    };
    let mut tvp: *mut timeval = 0 as *mut timeval;
    let mut rc: i32 = 0;
    let mut err: i32 = 0;
    let mut info: *mut i8 = 0 as *mut i8;
    if (*(*conn).handler).flags
        & ((1 as i32) << 0 as i32) as u32 != 0
    {
        if !(*li).ssldone {
            let mut result: CURLcode = Curl_ssl_connect_nonblocking(
                data,
                conn,
                0 as i32 != 0,
                0 as i32,
                &mut (*li).ssldone,
            );
            if result as u32 != 0 || !(*li).ssldone {
                return result;
            }
        }
        if !(*li).sslinst {
            let mut sb: *mut Sockbuf = 0 as *mut Sockbuf;
            ldap_get_option(
                (*li).ld,
                0x5008 as i32,
                &mut sb as *mut *mut Sockbuf as *mut libc::c_void,
            );
            ber_sockbuf_add_io(
                sb,
                &mut ldapsb_tls,
                20 as i32,
                data as *mut libc::c_void,
            );
            (*li).sslinst = 1 as i32 != 0;
            let ref mut fresh2 = (*li).recv;
            *fresh2 = (*conn).recv[0 as i32 as usize];
            let ref mut fresh3 = (*li).send;
            *fresh3 = (*conn).send[0 as i32 as usize];
        }
    }
    tvp = &mut tv;
    loop {
        if !(*li).didbind {
            let mut binddn: *mut i8 = 0 as *mut i8;
            let mut passwd: berval = berval {
                bv_len: 0,
                bv_val: 0 as *mut i8,
            };
            if ((*conn).bits).user_passwd() != 0 {
                binddn = (*conn).user;
                passwd.bv_val = (*conn).passwd;
                passwd.bv_len = strlen(passwd.bv_val);
            } else {
                binddn = 0 as *mut i8;
                passwd.bv_val = 0 as *mut i8;
                passwd.bv_len = 0 as i32 as ber_len_t;
            }
            rc = ldap_sasl_bind(
                (*li).ld,
                binddn,
                0 as *mut i8,
                &mut passwd,
                0 as *mut *mut LDAPControl,
                0 as *mut *mut LDAPControl,
                &mut (*li).msgid,
            );
            if rc != 0 {
                return CURLE_LDAP_CANNOT_BIND;
            }
            (*li).didbind = 1 as i32 != 0;
            if !tvp.is_null() {
                return CURLE_OK;
            }
        }
        rc = ldap_result((*li).ld, (*li).msgid, 0 as i32, tvp, &mut msg);
        if rc < 0 as i32 {
            Curl_failf(
                data,
                b"LDAP local: bind ldap_result %s\0" as *const u8 as *const i8,
                ldap_err2string(rc),
            );
            return CURLE_LDAP_CANNOT_BIND;
        }
        if rc == 0 as i32 {
            return CURLE_OK;
        }
        rc = ldap_parse_result(
            (*li).ld,
            msg,
            &mut err,
            0 as *mut *mut i8,
            &mut info,
            0 as *mut *mut *mut i8,
            0 as *mut *mut *mut LDAPControl,
            1 as i32,
        );
        if rc != 0 {
            Curl_failf(
                data,
                b"LDAP local: bind ldap_parse_result %s\0" as *const u8
                    as *const i8,
                ldap_err2string(rc),
            );
            return CURLE_LDAP_CANNOT_BIND;
        }
        if !(err == 0x2 as i32) {
            break;
        }
        let mut proto: i32 = 0;
        ldap_get_option(
            (*li).ld,
            0x11 as i32,
            &mut proto as *mut i32 as *mut libc::c_void,
        );
        if !(proto == 3 as i32) {
            break;
        }
        if !info.is_null() {
            ldap_memfree(info as *mut libc::c_void);
            info = 0 as *mut i8;
        }
        proto = 2 as i32;
        ldap_set_option(
            (*li).ld,
            0x11 as i32,
            &mut proto as *mut i32 as *const libc::c_void,
        );
        (*li).didbind = 0 as i32 != 0;
    }
    if err != 0 {
        Curl_failf(
            data,
            b"LDAP remote: bind failed %s %s\0" as *const u8 as *const i8,
            ldap_err2string(rc),
            if !info.is_null() {
                info as *const i8
            } else {
                b"\0" as *const u8 as *const i8
            },
        );
        if !info.is_null() {
            ldap_memfree(info as *mut libc::c_void);
        }
        return CURLE_LOGIN_DENIED;
    }
    if !info.is_null() {
        ldap_memfree(info as *mut libc::c_void);
    }
    let ref mut fresh4 = (*conn).recv[0 as i32 as usize];
    *fresh4 = Some(oldap_recv as Curl_recv);
    *done = 1 as i32 != 0;
    return CURLE_OK;
}
unsafe extern "C" fn oldap_disconnect(
    mut data: *mut Curl_easy,
    mut conn: *mut connectdata,
    mut dead_connection: bool,
) -> CURLcode {
    let mut li: *mut ldapconninfo = (*conn).proto.ldapc;
    if !li.is_null() {
        if !((*li).ld).is_null() {
            if ((*conn).ssl[0 as i32 as usize]).use_0() != 0 {
                let mut sb: *mut Sockbuf = 0 as *mut Sockbuf;
                ldap_get_option(
                    (*li).ld,
                    0x5008 as i32,
                    &mut sb as *mut *mut Sockbuf as *mut libc::c_void,
                );
                ber_sockbuf_add_io(
                    sb,
                    &mut ldapsb_tls,
                    20 as i32,
                    data as *mut libc::c_void,
                );
            }
            ldap_unbind_ext(
                (*li).ld,
                0 as *mut *mut LDAPControl,
                0 as *mut *mut LDAPControl,
            );
            let ref mut fresh5 = (*li).ld;
            *fresh5 = 0 as *mut LDAP;
        }
        let ref mut fresh6 = (*conn).proto.ldapc;
        *fresh6 = 0 as *mut ldapconninfo;
        Curl_cfree.expect("non-null function pointer")(li as *mut libc::c_void);
    }
    return CURLE_OK;
}
unsafe extern "C" fn oldap_do(
    mut data: *mut Curl_easy,
    mut done: *mut bool,
) -> CURLcode {
    let mut conn: *mut connectdata = (*data).conn;
    let mut li: *mut ldapconninfo = (*conn).proto.ldapc;
    let mut lr: *mut ldapreqinfo = 0 as *mut ldapreqinfo;
    let mut status: CURLcode = CURLE_OK;
    let mut rc: i32 = 0 as i32;
    let mut ludp: *mut LDAPURLDesc = 0 as *mut LDAPURLDesc;
    let mut msgid: i32 = 0;
    Curl_conncontrol(conn, 0 as i32);
    Curl_infof(
        data,
        b"LDAP local: %s\0" as *const u8 as *const i8,
        (*data).state.url,
    );
    rc = ldap_url_parse((*data).state.url, &mut ludp);
    if rc != 0 as i32 {
        let mut msg: *const i8 = b"url parsing problem\0" as *const u8
            as *const i8;
        status = CURLE_URL_MALFORMAT;
        if rc > 0 as i32 && rc <= 0xa as i32 {
            if rc == 0x1 as i32 {
                status = CURLE_OUT_OF_MEMORY;
            }
            msg = url_errs[rc as usize];
        }
        Curl_failf(data, b"LDAP local: %s\0" as *const u8 as *const i8, msg);
        return status;
    }
    rc = ldap_search_ext(
        (*li).ld,
        (*ludp).lud_dn,
        (*ludp).lud_scope,
        (*ludp).lud_filter,
        (*ludp).lud_attrs,
        0 as i32,
        0 as *mut *mut LDAPControl,
        0 as *mut *mut LDAPControl,
        0 as *mut timeval,
        0 as i32,
        &mut msgid,
    );
    ldap_free_urldesc(ludp);
    if rc != 0 as i32 {
        Curl_failf(
            data,
            b"LDAP local: ldap_search_ext %s\0" as *const u8 as *const i8,
            ldap_err2string(rc),
        );
        return CURLE_LDAP_SEARCH_FAILED;
    }
    lr = Curl_ccalloc
        .expect(
            "non-null function pointer",
        )(
        1 as i32 as size_t,
        ::std::mem::size_of::<ldapreqinfo>() as u64,
    ) as *mut ldapreqinfo;
    if lr.is_null() {
        return CURLE_OUT_OF_MEMORY;
    }
    (*lr).msgid = msgid;
    let ref mut fresh7 = (*data).req.p.ldap;
    *fresh7 = lr;
    Curl_setup_transfer(
        data,
        0 as i32,
        -(1 as i32) as curl_off_t,
        0 as i32 != 0,
        -(1 as i32),
    );
    *done = 1 as i32 != 0;
    return CURLE_OK;
}
unsafe extern "C" fn oldap_done(
    mut data: *mut Curl_easy,
    mut res: CURLcode,
    mut premature: bool,
) -> CURLcode {
    let mut conn: *mut connectdata = (*data).conn;
    let mut lr: *mut ldapreqinfo = (*data).req.p.ldap;
    if !lr.is_null() {
        if (*lr).msgid != 0 {
            let mut li: *mut ldapconninfo = (*conn).proto.ldapc;
            ldap_abandon_ext(
                (*li).ld,
                (*lr).msgid,
                0 as *mut *mut LDAPControl,
                0 as *mut *mut LDAPControl,
            );
            (*lr).msgid = 0 as i32;
        }
        let ref mut fresh8 = (*data).req.p.ldap;
        *fresh8 = 0 as *mut ldapreqinfo;
        Curl_cfree.expect("non-null function pointer")(lr as *mut libc::c_void);
    }
    return CURLE_OK;
}
unsafe extern "C" fn oldap_recv(
    mut data: *mut Curl_easy,
    mut sockindex: i32,
    mut buf: *mut i8,
    mut len: size_t,
    mut err: *mut CURLcode,
) -> ssize_t {
    let mut conn: *mut connectdata = (*data).conn;
    let mut li: *mut ldapconninfo = (*conn).proto.ldapc;
    let mut lr: *mut ldapreqinfo = (*data).req.p.ldap;
    let mut rc: i32 = 0;
    let mut ret: i32 = 0;
    let mut msg: *mut LDAPMessage = 0 as *mut LDAPMessage;
    let mut ent: *mut LDAPMessage = 0 as *mut LDAPMessage;
    let mut ber: *mut BerElement = 0 as *mut BerElement;
    let mut tv: timeval = {
        let mut init = timeval {
            tv_sec: 0 as i32 as __time_t,
            tv_usec: 1 as i32 as __suseconds_t,
        };
        init
    };
    rc = ldap_result((*li).ld, (*lr).msgid, 0x2 as i32, &mut tv, &mut msg);
    if rc < 0 as i32 {
        Curl_failf(
            data,
            b"LDAP local: search ldap_result %s\0" as *const u8 as *const i8,
            ldap_err2string(rc),
        );
        *err = CURLE_RECV_ERROR;
        return -(1 as i32) as ssize_t;
    }
    *err = CURLE_AGAIN;
    ret = -(1 as i32);
    if msg.is_null() {
        return ret as ssize_t;
    }
    ent = ldap_first_message((*li).ld, msg);
    while !ent.is_null() {
        let mut bv: berval = berval {
            bv_len: 0,
            bv_val: 0 as *mut i8,
        };
        let mut bvals: *mut berval = 0 as *mut berval;
        let mut binary: i32 = 0 as i32;
        let mut msgtype: i32 = 0;
        let mut writeerr: CURLcode = CURLE_OK;
        msgtype = ldap_msgtype(ent);
        if msgtype as u64 == 0x65 as u32 as ber_tag_t {
            let mut code: i32 = 0;
            let mut info: *mut i8 = 0 as *mut i8;
            rc = ldap_parse_result(
                (*li).ld,
                ent,
                &mut code,
                0 as *mut *mut i8,
                &mut info,
                0 as *mut *mut *mut i8,
                0 as *mut *mut *mut LDAPControl,
                0 as i32,
            );
            if rc != 0 {
                Curl_failf(
                    data,
                    b"LDAP local: search ldap_parse_result %s\0" as *const u8
                        as *const i8,
                    ldap_err2string(rc),
                );
                *err = CURLE_LDAP_SEARCH_FAILED;
            } else if code != 0 && code != 0x4 as i32 {
                Curl_failf(
                    data,
                    b"LDAP remote: search failed %s %s\0" as *const u8
                        as *const i8,
                    ldap_err2string(rc),
                    if !info.is_null() {
                        info as *const i8
                    } else {
                        b"\0" as *const u8 as *const i8
                    },
                );
                *err = CURLE_LDAP_SEARCH_FAILED;
            } else {
                if code == 0x4 as i32 {
                    Curl_infof(
                        data,
                        b"There are more than %d entries\0" as *const u8
                            as *const i8,
                        (*lr).nument,
                    );
                }
                (*data).req.size = (*data).req.bytecount;
                *err = CURLE_OK;
                ret = 0 as i32;
            }
            (*lr).msgid = 0 as i32;
            ldap_memfree(info as *mut libc::c_void);
            break;
        } else {
            if !(msgtype as u64 != 0x64 as u32 as ber_tag_t) {
                let ref mut fresh9 = (*lr).nument;
                *fresh9 += 1;
                rc = ldap_get_dn_ber((*li).ld, ent, &mut ber, &mut bv);
                if rc < 0 as i32 {
                    *err = CURLE_RECV_ERROR;
                    return -(1 as i32) as ssize_t;
                }
                writeerr = Curl_client_write(
                    data,
                    (1 as i32) << 0 as i32,
                    b"DN: \0" as *const u8 as *const i8 as *mut i8,
                    4 as i32 as size_t,
                );
                if writeerr as u64 != 0 {
                    *err = writeerr;
                    return -(1 as i32) as ssize_t;
                }
                writeerr = Curl_client_write(
                    data,
                    (1 as i32) << 0 as i32,
                    bv.bv_val,
                    bv.bv_len,
                );
                if writeerr as u64 != 0 {
                    *err = writeerr;
                    return -(1 as i32) as ssize_t;
                }
                writeerr = Curl_client_write(
                    data,
                    (1 as i32) << 0 as i32,
                    b"\n\0" as *const u8 as *const i8 as *mut i8,
                    1 as i32 as size_t,
                );
                if writeerr as u64 != 0 {
                    *err = writeerr;
                    return -(1 as i32) as ssize_t;
                }
                let ref mut fresh10 = (*data).req.bytecount;
                *fresh10 = (*fresh10 as u64)
                    .wrapping_add(
                        (bv.bv_len).wrapping_add(5 as i32 as u64),
                    ) as curl_off_t as curl_off_t;
                rc = ldap_get_attribute_ber((*li).ld, ent, ber, &mut bv, &mut bvals);
                while rc == 0 as i32 {
                    let mut i: i32 = 0;
                    if (bv.bv_val).is_null() {
                        break;
                    }
                    if bv.bv_len > 7 as i32 as u64
                        && strncmp(
                            (bv.bv_val)
                                .offset(bv.bv_len as isize)
                                .offset(-(7 as i32 as isize)),
                            b";binary\0" as *const u8 as *const i8,
                            7 as i32 as u64,
                        ) == 0
                    {
                        binary = 1 as i32;
                    } else {
                        binary = 0 as i32;
                    }
                    if bvals.is_null() {
                        writeerr = Curl_client_write(
                            data,
                            (1 as i32) << 0 as i32,
                            b"\t\0" as *const u8 as *const i8
                                as *mut i8,
                            1 as i32 as size_t,
                        );
                        if writeerr as u64 != 0 {
                            *err = writeerr;
                            return -(1 as i32) as ssize_t;
                        }
                        writeerr = Curl_client_write(
                            data,
                            (1 as i32) << 0 as i32,
                            bv.bv_val,
                            bv.bv_len,
                        );
                        if writeerr as u64 != 0 {
                            *err = writeerr;
                            return -(1 as i32) as ssize_t;
                        }
                        writeerr = Curl_client_write(
                            data,
                            (1 as i32) << 0 as i32,
                            b":\n\0" as *const u8 as *const i8
                                as *mut i8,
                            2 as i32 as size_t,
                        );
                        if writeerr as u64 != 0 {
                            *err = writeerr;
                            return -(1 as i32) as ssize_t;
                        }
                        let ref mut fresh11 = (*data).req.bytecount;
                        *fresh11 = (*fresh11 as u64)
                            .wrapping_add(
                                (bv.bv_len).wrapping_add(3 as i32 as u64),
                            ) as curl_off_t as curl_off_t;
                    } else {
                        i = 0 as i32;
                        while !((*bvals.offset(i as isize)).bv_val).is_null() {
                            let mut binval: i32 = 0 as i32;
                            writeerr = Curl_client_write(
                                data,
                                (1 as i32) << 0 as i32,
                                b"\t\0" as *const u8 as *const i8
                                    as *mut i8,
                                1 as i32 as size_t,
                            );
                            if writeerr as u64 != 0 {
                                *err = writeerr;
                                return -(1 as i32) as ssize_t;
                            }
                            writeerr = Curl_client_write(
                                data,
                                (1 as i32) << 0 as i32,
                                bv.bv_val,
                                bv.bv_len,
                            );
                            if writeerr as u64 != 0 {
                                *err = writeerr;
                                return -(1 as i32) as ssize_t;
                            }
                            writeerr = Curl_client_write(
                                data,
                                (1 as i32) << 0 as i32,
                                b":\0" as *const u8 as *const i8
                                    as *mut i8,
                                1 as i32 as size_t,
                            );
                            if writeerr as u64 != 0 {
                                *err = writeerr;
                                return -(1 as i32) as ssize_t;
                            }
                            let ref mut fresh12 = (*data).req.bytecount;
                            *fresh12 = (*fresh12 as u64)
                                .wrapping_add(
                                    (bv.bv_len).wrapping_add(2 as i32 as u64),
                                ) as curl_off_t as curl_off_t;
                            if binary == 0 {
                                if Curl_isspace(
                                    *((*bvals.offset(i as isize)).bv_val)
                                        .offset(0 as i32 as isize) as u8
                                        as i32,
                                ) != 0
                                    || Curl_isspace(
                                        *((*bvals.offset(i as isize)).bv_val)
                                            .offset(
                                                ((*bvals.offset(i as isize)).bv_len)
                                                    .wrapping_sub(1 as i32 as u64) as isize,
                                            ) as u8 as i32,
                                    ) != 0
                                {
                                    binval = 1 as i32;
                                } else {
                                    let mut j: u32 = 0;
                                    j = 0 as i32 as u32;
                                    while (j as u64)
                                        < (*bvals.offset(i as isize)).bv_len
                                    {
                                        if Curl_isprint(
                                            *((*bvals.offset(i as isize)).bv_val).offset(j as isize)
                                                as u8 as i32,
                                        ) == 0
                                        {
                                            binval = 1 as i32;
                                            break;
                                        } else {
                                            j = j.wrapping_add(1);
                                        }
                                    }
                                }
                            }
                            if binary != 0 || binval != 0 {
                                let mut val_b64: *mut i8 = 0 as *mut i8;
                                let mut val_b64_sz: size_t = 0 as i32 as size_t;
                                let mut error: CURLcode = Curl_base64_encode(
                                    data,
                                    (*bvals.offset(i as isize)).bv_val,
                                    (*bvals.offset(i as isize)).bv_len,
                                    &mut val_b64,
                                    &mut val_b64_sz,
                                );
                                if error as u64 != 0 {
                                    ber_memfree(bvals as *mut libc::c_void);
                                    ber_free(ber, 0 as i32);
                                    ldap_msgfree(msg);
                                    *err = error;
                                    return -(1 as i32) as ssize_t;
                                }
                                writeerr = Curl_client_write(
                                    data,
                                    (1 as i32) << 0 as i32,
                                    b": \0" as *const u8 as *const i8
                                        as *mut i8,
                                    2 as i32 as size_t,
                                );
                                if writeerr as u64 != 0 {
                                    *err = writeerr;
                                    return -(1 as i32) as ssize_t;
                                }
                                let ref mut fresh13 = (*data).req.bytecount;
                                *fresh13 += 2 as i32 as i64;
                                if val_b64_sz > 0 as i32 as u64 {
                                    writeerr = Curl_client_write(
                                        data,
                                        (1 as i32) << 0 as i32,
                                        val_b64,
                                        val_b64_sz,
                                    );
                                    if writeerr as u64 != 0 {
                                        *err = writeerr;
                                        return -(1 as i32) as ssize_t;
                                    }
                                    Curl_cfree
                                        .expect(
                                            "non-null function pointer",
                                        )(val_b64 as *mut libc::c_void);
                                    let ref mut fresh14 = (*data).req.bytecount;
                                    *fresh14 = (*fresh14 as u64)
                                        .wrapping_add(val_b64_sz) as curl_off_t as curl_off_t;
                                }
                            } else {
                                writeerr = Curl_client_write(
                                    data,
                                    (1 as i32) << 0 as i32,
                                    b" \0" as *const u8 as *const i8
                                        as *mut i8,
                                    1 as i32 as size_t,
                                );
                                if writeerr as u64 != 0 {
                                    *err = writeerr;
                                    return -(1 as i32) as ssize_t;
                                }
                                writeerr = Curl_client_write(
                                    data,
                                    (1 as i32) << 0 as i32,
                                    (*bvals.offset(i as isize)).bv_val,
                                    (*bvals.offset(i as isize)).bv_len,
                                );
                                if writeerr as u64 != 0 {
                                    *err = writeerr;
                                    return -(1 as i32) as ssize_t;
                                }
                                let ref mut fresh15 = (*data).req.bytecount;
                                *fresh15 = (*fresh15 as u64)
                                    .wrapping_add(
                                        ((*bvals.offset(i as isize)).bv_len)
                                            .wrapping_add(1 as i32 as u64),
                                    ) as curl_off_t as curl_off_t;
                            }
                            writeerr = Curl_client_write(
                                data,
                                (1 as i32) << 0 as i32,
                                b"\n\0" as *const u8 as *const i8
                                    as *mut i8,
                                1 as i32 as size_t,
                            );
                            if writeerr as u64 != 0 {
                                *err = writeerr;
                                return -(1 as i32) as ssize_t;
                            }
                            let ref mut fresh16 = (*data).req.bytecount;
                            *fresh16 += 1;
                            i += 1;
                        }
                        ber_memfree(bvals as *mut libc::c_void);
                        writeerr = Curl_client_write(
                            data,
                            (1 as i32) << 0 as i32,
                            b"\n\0" as *const u8 as *const i8
                                as *mut i8,
                            1 as i32 as size_t,
                        );
                        if writeerr as u64 != 0 {
                            *err = writeerr;
                            return -(1 as i32) as ssize_t;
                        }
                        let ref mut fresh17 = (*data).req.bytecount;
                        *fresh17 += 1;
                    }
                    rc = ldap_get_attribute_ber((*li).ld, ent, ber, &mut bv, &mut bvals);
                }
                writeerr = Curl_client_write(
                    data,
                    (1 as i32) << 0 as i32,
                    b"\n\0" as *const u8 as *const i8 as *mut i8,
                    1 as i32 as size_t,
                );
                if writeerr as u64 != 0 {
                    *err = writeerr;
                    return -(1 as i32) as ssize_t;
                }
                let ref mut fresh18 = (*data).req.bytecount;
                *fresh18 += 1;
                ber_free(ber, 0 as i32);
            }
            ent = ldap_next_message((*li).ld, ent);
        }
    }
    ldap_msgfree(msg);
    return ret as ssize_t;
}
unsafe extern "C" fn ldapsb_tls_setup(
    mut sbiod: *mut Sockbuf_IO_Desc,
    mut arg: *mut libc::c_void,
) -> i32 {
    let ref mut fresh19 = (*sbiod).sbiod_pvt;
    *fresh19 = arg;
    return 0 as i32;
}
unsafe extern "C" fn ldapsb_tls_remove(mut sbiod: *mut Sockbuf_IO_Desc) -> i32 {
    let ref mut fresh20 = (*sbiod).sbiod_pvt;
    *fresh20 = 0 as *mut libc::c_void;
    return 0 as i32;
}
 extern "C" fn ldapsb_tls_close(mut sbiod: *mut Sockbuf_IO_Desc) -> i32 {
    return 0 as i32;
}
unsafe extern "C" fn ldapsb_tls_ctrl(
    mut sbiod: *mut Sockbuf_IO_Desc,
    mut opt: i32,
    mut arg: *mut libc::c_void,
) -> i32 {
    if opt == 8 as i32 {
        let mut data: *mut Curl_easy = (*sbiod).sbiod_pvt as *mut Curl_easy;
        return Curl_ssl_data_pending((*data).conn, 0 as i32) as i32;
    }
    return 0 as i32;
}
unsafe extern "C" fn ldapsb_tls_read(
    mut sbiod: *mut Sockbuf_IO_Desc,
    mut buf: *mut libc::c_void,
    mut len: ber_len_t,
) -> ber_slen_t {
    let mut data: *mut Curl_easy = (*sbiod).sbiod_pvt as *mut Curl_easy;
    let mut ret: ber_slen_t = 0 as i32 as ber_slen_t;
    if !data.is_null() {
        let mut conn: *mut connectdata = (*data).conn;
        if !conn.is_null() {
            let mut li: *mut ldapconninfo = (*conn).proto.ldapc;
            let mut err: CURLcode = CURLE_RECV_ERROR;
            ret = ((*li).recv)
                .expect(
                    "non-null function pointer",
                )(data, 0 as i32, buf as *mut i8, len, &mut err);
            if ret < 0 as i32 as i64
                && err as u32 == CURLE_AGAIN as i32 as u32
            {
                *__errno_location() = 11 as i32;
            }
        }
    }
    return ret;
}
unsafe extern "C" fn ldapsb_tls_write(
    mut sbiod: *mut Sockbuf_IO_Desc,
    mut buf: *mut libc::c_void,
    mut len: ber_len_t,
) -> ber_slen_t {
    let mut data: *mut Curl_easy = (*sbiod).sbiod_pvt as *mut Curl_easy;
    let mut ret: ber_slen_t = 0 as i32 as ber_slen_t;
    if !data.is_null() {
        let mut conn: *mut connectdata = (*data).conn;
        if !conn.is_null() {
            let mut li: *mut ldapconninfo = (*conn).proto.ldapc;
            let mut err: CURLcode = CURLE_SEND_ERROR;
            ret = ((*li).send)
                .expect(
                    "non-null function pointer",
                )(data, 0 as i32, buf, len, &mut err);
            if ret < 0 as i32 as i64
                && err as u32 == CURLE_AGAIN as i32 as u32
            {
                *__errno_location() = 11 as i32;
            }
        }
    }
    return ret;
}
static mut ldapsb_tls: Sockbuf_IO = unsafe {
    {
        let mut init = sockbuf_io {
            sbi_setup: Some(
                ldapsb_tls_setup
                    as unsafe extern "C" fn(
                        *mut Sockbuf_IO_Desc,
                        *mut libc::c_void,
                    ) -> i32,
            ),
            sbi_remove: Some(
                ldapsb_tls_remove
                    as unsafe extern "C" fn(*mut Sockbuf_IO_Desc) -> i32,
            ),
            sbi_ctrl: Some(
                ldapsb_tls_ctrl
                    as unsafe extern "C" fn(
                        *mut Sockbuf_IO_Desc,
                        i32,
                        *mut libc::c_void,
                    ) -> i32,
            ),
            sbi_read: Some(
                ldapsb_tls_read
                    as unsafe extern "C" fn(
                        *mut Sockbuf_IO_Desc,
                        *mut libc::c_void,
                        ber_len_t,
                    ) -> ber_slen_t,
            ),
            sbi_write: Some(
                ldapsb_tls_write
                    as unsafe extern "C" fn(
                        *mut Sockbuf_IO_Desc,
                        *mut libc::c_void,
                        ber_len_t,
                    ) -> ber_slen_t,
            ),
            sbi_close: Some(
                ldapsb_tls_close
                    as unsafe extern "C" fn(*mut Sockbuf_IO_Desc) -> i32,
            ),
        };
        init
    }
};
