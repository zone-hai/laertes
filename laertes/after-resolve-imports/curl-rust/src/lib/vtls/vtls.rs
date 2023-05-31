use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    fn fclose(__stream: *mut FILE) -> i32;
    fn fopen(_: *const i8, _: *const i8) -> *mut FILE;
    fn fread(
        _: *mut libc::c_void,
        _: u64,
        _: u64,
        _: *mut FILE,
    ) -> u64;
    fn fseek(
        __stream: *mut FILE,
        __off: i64,
        __whence: i32,
    ) -> i32;
    fn ftell(__stream: *mut FILE) -> i64;
    
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strncpy(
        _: *mut i8,
        _: *const i8,
        _: u64,
    ) -> *mut i8;
    
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: u64,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: i32,
        _: u64,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: u64,
    ) -> i32;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strncmp(
        _: *const i8,
        _: *const i8,
        _: u64,
    ) -> i32;
    fn strstr(_: *const i8, _: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
}
pub use crate::src::lib::base64::Curl_base64_decode;
pub use crate::src::lib::base64::Curl_base64_encode;
pub use crate::src::lib::getenv::curl_getenv;
pub use crate::src::lib::mprintf::curl_msnprintf;
pub use crate::src::lib::progress::Curl_pgrsTime;
pub use crate::src::lib::sendf::Curl_failf;
pub use crate::src::lib::sendf::Curl_infof;
pub use crate::src::lib::sendf::Curl_recv_plain;
pub use crate::src::lib::sendf::Curl_send_plain;
pub use crate::src::lib::share::Curl_share_lock;
pub use crate::src::lib::share::Curl_share_unlock;
pub use crate::src::lib::slist::Curl_slist_append_nodup;
pub use crate::src::lib::slist::curl_slist_free_all;
pub use crate::src::lib::strcase::Curl_safe_strcasecompare;
pub use crate::src::lib::strcase::Curl_strcasecompare;
pub use crate::src::lib::warnless::curlx_sotouz;
pub use crate::src::lib::altsvc::altsvcinfo;
pub use crate::src::lib::asyn_thread::thread_data;
pub use crate::src::lib::content_encoding::contenc_writer;
pub use crate::src::lib::easy::hsts;
pub use crate::src::lib::ftp::http_connect_state;
pub use crate::src::lib::http2::curl_pushheaders;
pub use crate::src::lib::openldap::ldapconninfo;
pub use crate::src::lib::openldap::ldapreqinfo;
pub use crate::src::lib::smb::smb_request;
pub use crate::src::lib::telnet::TELNET;
pub use crate::src::lib::tftp::tftp_state_data;
pub use crate::src::lib::urlapi::Curl_URL;
pub use crate::src::lib::vtls::openssl::ssl_backend_data;
pub use crate::src::lib::altsvc::Gsasl;
pub use crate::src::lib::easy::Curl_ccalloc;
pub use crate::src::lib::easy::Curl_cfree;
pub use crate::src::lib::easy::Curl_cmalloc;
pub use crate::src::lib::easy::Curl_cstrdup;
pub use crate::src::lib::hostip6::psl_ctx_st;
pub use crate::src::lib::imap::_IO_marker;
pub use crate::src::lib::speedcheck::_IO_codecvt;
pub use crate::src::lib::urlapi::Gsasl_session;
pub use crate::src::lib::version::nghttp2_session;
pub use crate::src::lib::vtls::openssl::Curl_ssl_openssl;
pub type __uint8_t = crate::src::lib::altsvc::__uint8_t;
pub type __int32_t = crate::src::lib::altsvc::__int32_t;
pub type __uint32_t = crate::src::lib::altsvc::__uint32_t;
pub type __off_t = crate::src::lib::altsvc::__off_t;
pub type __off64_t = crate::src::lib::altsvc::__off64_t;
pub type __pid_t = crate::src::lib::altsvc::__pid_t;
pub type __time_t = crate::src::lib::altsvc::__time_t;
pub type __ssize_t = crate::src::lib::altsvc::__ssize_t;
pub type __socklen_t = crate::src::lib::altsvc::__socklen_t;
pub type pid_t = crate::src::lib::altsvc::pid_t;
pub type ssize_t = crate::src::lib::altsvc::ssize_t;
pub type time_t = crate::src::lib::altsvc::time_t;
pub type size_t = crate::src::lib::altsvc::size_t;
pub type int32_t = crate::src::lib::altsvc::int32_t;
pub type socklen_t = crate::src::lib::altsvc::socklen_t;
pub type sa_family_t = crate::src::lib::altsvc::sa_family_t;
// #[derive(Copy, Clone)]

pub type sockaddr = crate::src::lib::altsvc::sockaddr;
pub type curl_socklen_t = crate::src::lib::altsvc::curl_socklen_t;
pub type curl_off_t = crate::src::lib::altsvc::curl_off_t;
// #[derive(Copy, Clone)]

pub type _IO_FILE = crate::src::lib::altsvc::_IO_FILE;
pub type _IO_lock_t = crate::src::lib::altsvc::_IO_lock_t;
pub type FILE = crate::src::lib::altsvc::FILE;
// #[derive(Copy, Clone)]

pub type Curl_easy = crate::src::lib::altsvc::Curl_easy;
// #[derive(Copy, Clone)]

pub type curl_tlssessioninfo = crate::src::lib::altsvc::curl_tlssessioninfo;
pub type curl_sslbackend = crate::src::lib::altsvc::curl_sslbackend;
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

pub type PureInfo = crate::src::lib::altsvc::PureInfo;
pub type bit = crate::src::lib::altsvc::bit;
pub type CURLproxycode = crate::src::lib::altsvc::CURLproxycode;
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

pub type curl_certinfo = crate::src::lib::altsvc::curl_certinfo;
// #[derive(Copy, Clone)]

pub type curl_slist = crate::src::lib::altsvc::curl_slist;
// #[derive(Copy, Clone)]

pub type WildcardData = crate::src::lib::altsvc::WildcardData;
pub type wildcard_dtor = crate::src::lib::altsvc::wildcard_dtor;
// #[derive(Copy, Clone)]

pub type Curl_llist = crate::src::lib::altsvc::Curl_llist;
pub type Curl_llist_dtor = crate::src::lib::altsvc::Curl_llist_dtor;
// #[derive(Copy, Clone)]

pub type Curl_llist_element = crate::src::lib::altsvc::Curl_llist_element;
pub type wildcard_states = crate::src::lib::altsvc::wildcard_states;
pub const CURLWC_DONE: wildcard_states = 7;
pub const CURLWC_ERROR: wildcard_states = 6;
pub const CURLWC_SKIP: wildcard_states = 5;
pub const CURLWC_CLEAN: wildcard_states = 4;
pub const CURLWC_DOWNLOADING: wildcard_states = 3;
pub const CURLWC_MATCHING: wildcard_states = 2;
pub const CURLWC_INIT: wildcard_states = 1;
pub const CURLWC_CLEAR: wildcard_states = 0;
// #[derive(Copy, Clone, BitfieldStruct)]

pub type UrlState = crate::src::lib::altsvc::UrlState;
// #[derive(Copy, Clone)]

pub type dynamically_allocated_data = crate::src::lib::altsvc::dynamically_allocated_data;
pub type trailers_state = crate::src::lib::altsvc::trailers_state;
pub const TRAILERS_DONE: trailers_state = 3;
pub const TRAILERS_SENDING: trailers_state = 2;
pub const TRAILERS_INITIALIZED: trailers_state = 1;
pub const TRAILERS_NONE: trailers_state = 0;
// #[derive(Copy, Clone)]

pub type dynbuf = crate::src::lib::altsvc::dynbuf;
pub type Curl_HttpReq = crate::src::lib::altsvc::Curl_HttpReq;
pub const HTTPREQ_HEAD: Curl_HttpReq = 5;
pub const HTTPREQ_PUT: Curl_HttpReq = 4;
pub const HTTPREQ_POST_MIME: Curl_HttpReq = 3;
pub const HTTPREQ_POST_FORM: Curl_HttpReq = 2;
pub const HTTPREQ_POST: Curl_HttpReq = 1;
pub const HTTPREQ_GET: Curl_HttpReq = 0;
// #[derive(Copy, Clone)]

pub type urlpieces = crate::src::lib::altsvc::urlpieces;
pub type CURLU = crate::src::lib::altsvc::CURLU;
pub type curl_read_callback = crate::src::lib::altsvc::curl_read_callback;
// #[derive(Copy, Clone)]

pub type time_node = crate::src::lib::altsvc::time_node;
pub type expire_id = crate::src::lib::altsvc::expire_id;
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

pub type curltime = crate::src::lib::altsvc::curltime;
// #[derive(Copy, Clone)]

pub type Curl_tree = crate::src::lib::altsvc::Curl_tree;
// #[derive(Copy, Clone, BitfieldStruct)]

pub type Curl_async = crate::src::lib::altsvc::Curl_async;
// #[derive(Copy, Clone)]

pub type Curl_dns_entry = crate::src::lib::altsvc::Curl_dns_entry;
// #[derive(Copy, Clone)]

pub type Curl_addrinfo = crate::src::lib::altsvc::Curl_addrinfo;
// #[derive(Copy, Clone, BitfieldStruct)]

pub type auth = crate::src::lib::altsvc::auth;
// #[derive(Copy, Clone, BitfieldStruct)]

pub type digestdata = crate::src::lib::altsvc::digestdata;
// #[derive(Copy, Clone)]

pub type tempbuf = crate::src::lib::altsvc::tempbuf;
// #[derive(Copy, Clone)]

pub type Curl_ssl_session = crate::src::lib::altsvc::Curl_ssl_session;
// #[derive(Copy, Clone, BitfieldStruct)]

pub type ssl_primary_config = crate::src::lib::altsvc::ssl_primary_config;
// #[derive(Copy, Clone)]

pub type curl_blob = crate::src::lib::altsvc::curl_blob;
// #[derive(Copy, Clone)]

pub type conncache = crate::src::lib::altsvc::conncache;
// #[derive(Copy, Clone)]

pub type Curl_hash = crate::src::lib::altsvc::Curl_hash;
pub type Curl_hash_dtor = crate::src::lib::altsvc::Curl_hash_dtor;
pub type comp_function = crate::src::lib::altsvc::comp_function;
pub type hash_function = crate::src::lib::altsvc::hash_function;
// #[derive(Copy, Clone, BitfieldStruct)]

pub type Progress = crate::src::lib::altsvc::Progress;
pub type timediff_t = crate::src::lib::altsvc::timediff_t;
// #[derive(Copy, Clone)]

pub type CookieInfo = crate::src::lib::altsvc::CookieInfo;
// #[derive(Copy, Clone)]

pub type Cookie = crate::src::lib::altsvc::Cookie;
// #[derive(Copy, Clone, BitfieldStruct)]

pub type UserDefined = crate::src::lib::altsvc::UserDefined;
pub type curl_trailer_callback = crate::src::lib::altsvc::curl_trailer_callback;
pub type multidone_func = crate::src::lib::altsvc::multidone_func;
pub type CURLcode = crate::src::lib::altsvc::CURLcode;
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
pub type curl_resolver_start_callback = crate::src::lib::altsvc::curl_resolver_start_callback;
// #[derive(Copy, Clone)]

pub type Curl_http2_dep = crate::src::lib::altsvc::Curl_http2_dep;
pub type curl_fnmatch_callback = crate::src::lib::altsvc::curl_fnmatch_callback;
pub type curl_chunk_end_callback = crate::src::lib::altsvc::curl_chunk_end_callback;
pub type curl_chunk_bgn_callback = crate::src::lib::altsvc::curl_chunk_bgn_callback;
pub type Curl_RtspReq = crate::src::lib::altsvc::Curl_RtspReq;
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
pub type curl_usessl = crate::src::lib::altsvc::curl_usessl;
pub const CURLUSESSL_LAST: curl_usessl = 4;
pub const CURLUSESSL_ALL: curl_usessl = 3;
pub const CURLUSESSL_CONTROL: curl_usessl = 2;
pub const CURLUSESSL_TRY: curl_usessl = 1;
pub const CURLUSESSL_NONE: curl_usessl = 0;
pub type CURL_NETRC_OPTION = crate::src::lib::altsvc::CURL_NETRC_OPTION;
pub const CURL_NETRC_LAST: CURL_NETRC_OPTION = 3;
pub const CURL_NETRC_REQUIRED: CURL_NETRC_OPTION = 2;
pub const CURL_NETRC_OPTIONAL: CURL_NETRC_OPTION = 1;
pub const CURL_NETRC_IGNORED: CURL_NETRC_OPTION = 0;
pub type curl_sshkeycallback = crate::src::lib::altsvc::curl_sshkeycallback;
pub type curl_khmatch = crate::src::lib::altsvc::curl_khmatch;
pub const CURLKHMATCH_LAST: curl_khmatch = 3;
pub const CURLKHMATCH_MISSING: curl_khmatch = 2;
pub const CURLKHMATCH_MISMATCH: curl_khmatch = 1;
pub const CURLKHMATCH_OK: curl_khmatch = 0;
// #[derive(Copy, Clone)]

pub type curl_khkey = crate::src::lib::altsvc::curl_khkey;
pub type curl_khtype = crate::src::lib::altsvc::curl_khtype;
pub const CURLKHTYPE_ED25519: curl_khtype = 5;
pub const CURLKHTYPE_ECDSA: curl_khtype = 4;
pub const CURLKHTYPE_DSS: curl_khtype = 3;
pub const CURLKHTYPE_RSA: curl_khtype = 2;
pub const CURLKHTYPE_RSA1: curl_khtype = 1;
pub const CURLKHTYPE_UNKNOWN: curl_khtype = 0;
pub type CURL = crate::src::lib::altsvc::CURL;
pub type curl_ftpccc = crate::src::lib::altsvc::curl_ftpccc;
pub const CURLFTPSSL_CCC_LAST: curl_ftpccc = 3;
pub const CURLFTPSSL_CCC_ACTIVE: curl_ftpccc = 2;
pub const CURLFTPSSL_CCC_PASSIVE: curl_ftpccc = 1;
pub const CURLFTPSSL_CCC_NONE: curl_ftpccc = 0;
pub type curl_ftpauth = crate::src::lib::altsvc::curl_ftpauth;
pub const CURLFTPAUTH_LAST: curl_ftpauth = 3;
pub const CURLFTPAUTH_TLS: curl_ftpauth = 2;
pub const CURLFTPAUTH_SSL: curl_ftpauth = 1;
pub const CURLFTPAUTH_DEFAULT: curl_ftpauth = 0;
pub type curl_ftpfile = crate::src::lib::altsvc::curl_ftpfile;
pub const FTPFILE_SINGLECWD: curl_ftpfile = 3;
pub const FTPFILE_NOCWD: curl_ftpfile = 2;
pub const FTPFILE_MULTICWD: curl_ftpfile = 1;
// #[derive(Copy, Clone)]

pub type ssl_general_config = crate::src::lib::altsvc::ssl_general_config;
// #[derive(Copy, Clone, BitfieldStruct)]

pub type ssl_config_data = crate::src::lib::altsvc::ssl_config_data;
pub type CURL_TLSAUTH = crate::src::lib::altsvc::CURL_TLSAUTH;
pub const CURL_TLSAUTH_LAST: CURL_TLSAUTH = 2;
pub const CURL_TLSAUTH_SRP: CURL_TLSAUTH = 1;
pub const CURL_TLSAUTH_NONE: CURL_TLSAUTH = 0;
pub type curl_ssl_ctx_callback = crate::src::lib::altsvc::curl_ssl_ctx_callback;
pub type curl_proxytype = crate::src::lib::altsvc::curl_proxytype;
pub const CURLPROXY_SOCKS5_HOSTNAME: curl_proxytype = 7;
pub const CURLPROXY_SOCKS4A: curl_proxytype = 6;
pub const CURLPROXY_SOCKS5: curl_proxytype = 5;
pub const CURLPROXY_SOCKS4: curl_proxytype = 4;
pub const CURLPROXY_HTTPS: curl_proxytype = 2;
pub const CURLPROXY_HTTP_1_0: curl_proxytype = 1;
pub const CURLPROXY_HTTP: curl_proxytype = 0;
pub type curl_TimeCond = crate::src::lib::altsvc::curl_TimeCond;
pub const CURL_TIMECOND_LAST: curl_TimeCond = 4;
pub const CURL_TIMECOND_LASTMOD: curl_TimeCond = 3;
pub const CURL_TIMECOND_IFUNMODSINCE: curl_TimeCond = 2;
pub const CURL_TIMECOND_IFMODSINCE: curl_TimeCond = 1;
pub const CURL_TIMECOND_NONE: curl_TimeCond = 0;
// #[derive(Copy, Clone)]

pub type curl_mimepart = crate::src::lib::altsvc::curl_mimepart;
// #[derive(Copy, Clone)]

pub type mime_encoder_state = crate::src::lib::altsvc::mime_encoder_state;
// #[derive(Copy, Clone)]

pub type mime_encoder = crate::src::lib::altsvc::mime_encoder;
// #[derive(Copy, Clone)]

pub type mime_state = crate::src::lib::altsvc::mime_state;
pub type mimestate = crate::src::lib::altsvc::mimestate;
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
pub type curl_free_callback = crate::src::lib::altsvc::curl_free_callback;
pub type curl_seek_callback = crate::src::lib::altsvc::curl_seek_callback;
pub type mimekind = crate::src::lib::altsvc::mimekind;
pub const MIMEKIND_LAST: mimekind = 5;
pub const MIMEKIND_MULTIPART: mimekind = 4;
pub const MIMEKIND_CALLBACK: mimekind = 3;
pub const MIMEKIND_FILE: mimekind = 2;
pub const MIMEKIND_DATA: mimekind = 1;
pub const MIMEKIND_NONE: mimekind = 0;
// #[derive(Copy, Clone)]

pub type curl_mime = crate::src::lib::altsvc::curl_mime;
// #[derive(Copy, Clone)]

pub type curl_httppost = crate::src::lib::altsvc::curl_httppost;
pub type curl_hstswrite_callback = crate::src::lib::altsvc::curl_hstswrite_callback;
// #[derive(Copy, Clone)]

pub type curl_index = crate::src::lib::altsvc::curl_index;
// #[derive(Copy, Clone, BitfieldStruct)]

pub type curl_hstsentry = crate::src::lib::altsvc::curl_hstsentry;
pub type CURLSTScode = crate::src::lib::altsvc::CURLSTScode;
pub const CURLSTS_FAIL: CURLSTScode = 2;
pub const CURLSTS_DONE: CURLSTScode = 1;
pub const CURLSTS_OK: CURLSTScode = 0;
pub type curl_hstsread_callback = crate::src::lib::altsvc::curl_hstsread_callback;
pub type curl_conv_callback = crate::src::lib::altsvc::curl_conv_callback;
pub type curl_closesocket_callback = crate::src::lib::altsvc::curl_closesocket_callback;
pub type curl_socket_t = crate::src::lib::altsvc::curl_socket_t;
pub type curl_opensocket_callback = crate::src::lib::altsvc::curl_opensocket_callback;
// #[derive(Copy, Clone)]

pub type curl_sockaddr = crate::src::lib::altsvc::curl_sockaddr;
pub type curlsocktype = crate::src::lib::altsvc::curlsocktype;
pub const CURLSOCKTYPE_LAST: curlsocktype = 2;
pub const CURLSOCKTYPE_ACCEPT: curlsocktype = 1;
pub const CURLSOCKTYPE_IPCXN: curlsocktype = 0;
pub type curl_sockopt_callback = crate::src::lib::altsvc::curl_sockopt_callback;
pub type curl_ioctl_callback = crate::src::lib::altsvc::curl_ioctl_callback;
pub type curlioerr = crate::src::lib::altsvc::curlioerr;
pub const CURLIOE_LAST: curlioerr = 3;
pub const CURLIOE_FAILRESTART: curlioerr = 2;
pub const CURLIOE_UNKNOWNCMD: curlioerr = 1;
pub const CURLIOE_OK: curlioerr = 0;
pub type curl_debug_callback = crate::src::lib::altsvc::curl_debug_callback;
pub type curl_infotype = crate::src::lib::altsvc::curl_infotype;
pub const CURLINFO_END: curl_infotype = 7;
pub const CURLINFO_SSL_DATA_OUT: curl_infotype = 6;
pub const CURLINFO_SSL_DATA_IN: curl_infotype = 5;
pub const CURLINFO_DATA_OUT: curl_infotype = 4;
pub const CURLINFO_DATA_IN: curl_infotype = 3;
pub const CURLINFO_HEADER_OUT: curl_infotype = 2;
pub const CURLINFO_HEADER_IN: curl_infotype = 1;
pub const CURLINFO_TEXT: curl_infotype = 0;
pub type curl_xferinfo_callback = crate::src::lib::altsvc::curl_xferinfo_callback;
pub type curl_progress_callback = crate::src::lib::altsvc::curl_progress_callback;
pub type curl_write_callback = crate::src::lib::altsvc::curl_write_callback;
// #[derive(Copy, Clone, BitfieldStruct)]

pub type SingleRequest = crate::src::lib::altsvc::SingleRequest;
// #[derive(Copy, Clone)]

pub type dohdata = crate::src::lib::altsvc::dohdata;
// #[derive(Copy, Clone)]

pub type dnsprobe = crate::src::lib::altsvc::dnsprobe;
// #[derive(Copy, Clone)]

pub type C2RustUnnamed = crate::src::lib::altsvc::C2RustUnnamed;
// #[derive(Copy, Clone)]

pub type SSHPROTO = crate::src::lib::altsvc::SSHPROTO;
// #[derive(Copy, Clone)]

pub type SMTP = crate::src::lib::altsvc::SMTP;
pub type curl_pp_transfer = crate::src::lib::altsvc::curl_pp_transfer;
pub const PPTRANSFER_NONE: curl_pp_transfer = 2;
pub const PPTRANSFER_INFO: curl_pp_transfer = 1;
pub const PPTRANSFER_BODY: curl_pp_transfer = 0;
// #[derive(Copy, Clone)]

pub type RTSP = crate::src::lib::altsvc::RTSP;
// #[derive(Copy, Clone)]

pub type HTTP = crate::src::lib::altsvc::HTTP;
pub type uint8_t = crate::src::lib::altsvc::uint8_t;
pub type uint32_t = crate::src::lib::altsvc::uint32_t;
pub type C2RustUnnamed_0 = u32;
pub const HTTPSEND_BODY: C2RustUnnamed_0 = 2;
pub const HTTPSEND_REQUEST: C2RustUnnamed_0 = 1;
pub const HTTPSEND_NADA: C2RustUnnamed_0 = 0;
// #[derive(Copy, Clone)]

pub type back = crate::src::lib::altsvc::back;
// #[derive(Copy, Clone)]

pub type POP3 = crate::src::lib::altsvc::POP3;
// #[derive(Copy, Clone)]

pub type MQTT = crate::src::lib::altsvc::MQTT;
// #[derive(Copy, Clone)]

pub type IMAP = crate::src::lib::altsvc::IMAP;
// #[derive(Copy, Clone)]

pub type FTP = crate::src::lib::altsvc::FTP;
// #[derive(Copy, Clone)]

pub type FILEPROTO = crate::src::lib::altsvc::FILEPROTO;
pub type upgrade101 = crate::src::lib::altsvc::upgrade101;
pub const UPGR101_WORKING: upgrade101 = 3;
pub const UPGR101_RECEIVED: upgrade101 = 2;
pub const UPGR101_REQUESTED: upgrade101 = 1;
pub const UPGR101_INIT: upgrade101 = 0;
pub type expect100 = crate::src::lib::altsvc::expect100;
pub const EXP100_FAILED: expect100 = 3;
pub const EXP100_SENDING_REQUEST: expect100 = 2;
pub const EXP100_AWAITING_CONTINUE: expect100 = 1;
pub const EXP100_SEND_DATA: expect100 = 0;
pub type C2RustUnnamed_1 = u32;
pub const HEADER_ALLBAD: C2RustUnnamed_1 = 2;
pub const HEADER_PARTHEADER: C2RustUnnamed_1 = 1;
pub const HEADER_NORMAL: C2RustUnnamed_1 = 0;
// #[derive(Copy, Clone)]

pub type PslCache = crate::src::lib::altsvc::PslCache;
pub type psl_ctx_t = crate::src::lib::altsvc::psl_ctx_t;
// #[derive(Copy, Clone)]

pub type Curl_share = crate::src::lib::asyn_thread::Curl_share;
pub type curl_unlock_function = crate::src::lib::asyn_thread::curl_unlock_function;
pub type curl_lock_data = crate::src::lib::asyn_thread::curl_lock_data;
pub const CURL_LOCK_DATA_LAST: curl_lock_data = 7;
pub const CURL_LOCK_DATA_PSL: curl_lock_data = 6;
pub const CURL_LOCK_DATA_CONNECT: curl_lock_data = 5;
pub const CURL_LOCK_DATA_SSL_SESSION: curl_lock_data = 4;
pub const CURL_LOCK_DATA_DNS: curl_lock_data = 3;
pub const CURL_LOCK_DATA_COOKIE: curl_lock_data = 2;
pub const CURL_LOCK_DATA_SHARE: curl_lock_data = 1;
pub const CURL_LOCK_DATA_NONE: curl_lock_data = 0;
pub type curl_lock_function = crate::src::lib::asyn_thread::curl_lock_function;
pub type curl_lock_access = crate::src::lib::asyn_thread::curl_lock_access;
pub const CURL_LOCK_ACCESS_LAST: curl_lock_access = 3;
pub const CURL_LOCK_ACCESS_SINGLE: curl_lock_access = 2;
pub const CURL_LOCK_ACCESS_SHARED: curl_lock_access = 1;
pub const CURL_LOCK_ACCESS_NONE: curl_lock_access = 0;
// #[derive(Copy, Clone)]

pub type Curl_multi = crate::src::lib::altsvc::Curl_multi;
pub type curl_multi_timer_callback = crate::src::lib::altsvc::curl_multi_timer_callback;
pub type CURLM = crate::src::lib::altsvc::CURLM;
pub type curl_push_callback = crate::src::lib::altsvc::curl_push_callback;
pub type curl_socket_callback = crate::src::lib::altsvc::curl_socket_callback;
// #[derive(Copy, Clone)]

pub type Names = crate::src::lib::altsvc::Names;
pub type C2RustUnnamed_2 = u32;
pub const HCACHE_SHARED: C2RustUnnamed_2 = 2;
pub const HCACHE_MULTI: C2RustUnnamed_2 = 1;
pub const HCACHE_NONE: C2RustUnnamed_2 = 0;
// #[derive(Copy, Clone)]

pub type Curl_message = crate::src::lib::altsvc::Curl_message;
// #[derive(Copy, Clone)]

pub type CURLMsg = crate::src::lib::altsvc::CURLMsg;
// #[derive(Copy, Clone)]

pub type C2RustUnnamed_3 = crate::src::lib::altsvc::C2RustUnnamed_3;
pub type CURLMSG = crate::src::lib::altsvc::CURLMSG;
pub const CURLMSG_LAST: CURLMSG = 2;
pub const CURLMSG_DONE: CURLMSG = 1;
pub const CURLMSG_NONE: CURLMSG = 0;
pub type CURLMstate = crate::src::lib::altsvc::CURLMstate;
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

pub type connectdata = crate::src::lib::altsvc::connectdata;
// #[derive(Copy, Clone)]

pub type connectbundle = crate::src::lib::altsvc::connectbundle;
// #[derive(Copy, Clone)]

pub type C2RustUnnamed_4 = crate::src::lib::altsvc::C2RustUnnamed_4;
// #[derive(Copy, Clone)]

pub type mqtt_conn = crate::src::lib::altsvc::mqtt_conn;
pub type mqttstate = crate::src::lib::altsvc::mqttstate;
pub const MQTT_NOSTATE: mqttstate = 7;
pub const MQTT_PUB_REMAIN: mqttstate = 6;
pub const MQTT_PUBWAIT: mqttstate = 5;
pub const MQTT_SUBACK_COMING: mqttstate = 4;
pub const MQTT_SUBACK: mqttstate = 3;
pub const MQTT_CONNACK: mqttstate = 2;
pub const MQTT_REMAINING_LENGTH: mqttstate = 1;
pub const MQTT_FIRST: mqttstate = 0;
// #[derive(Copy, Clone)]

pub type smb_conn = crate::src::lib::altsvc::smb_conn;
pub type smb_conn_state = crate::src::lib::altsvc::smb_conn_state;
pub const SMB_CONNECTED: smb_conn_state = 4;
pub const SMB_SETUP: smb_conn_state = 3;
pub const SMB_NEGOTIATE: smb_conn_state = 2;
pub const SMB_CONNECTING: smb_conn_state = 1;
pub const SMB_NOT_CONNECTED: smb_conn_state = 0;
// #[derive(Copy, Clone)]

pub type rtsp_conn = crate::src::lib::altsvc::rtsp_conn;
// #[derive(Copy, Clone)]

pub type smtp_conn = crate::src::lib::altsvc::smtp_conn;
// #[derive(Copy, Clone)]

pub type SASL = crate::src::lib::altsvc::SASL;
pub type saslstate = crate::src::lib::altsvc::saslstate;
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

pub type SASLproto = crate::src::lib::altsvc::SASLproto;
pub type smtpstate = crate::src::lib::altsvc::smtpstate;
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

pub type pingpong = crate::src::lib::altsvc::pingpong;
// #[derive(Copy, Clone)]

pub type pop3_conn = crate::src::lib::altsvc::pop3_conn;
pub type pop3state = crate::src::lib::altsvc::pop3state;
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

pub type imap_conn = crate::src::lib::altsvc::imap_conn;
pub type imapstate = crate::src::lib::altsvc::imapstate;
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

pub type ssh_conn = crate::src::lib::altsvc::ssh_conn;
pub type sshstate = crate::src::lib::altsvc::sshstate;
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

pub type http_conn = crate::src::lib::altsvc::http_conn;
// #[derive(Copy, Clone)]

pub type nghttp2_settings_entry = crate::src::lib::altsvc::nghttp2_settings_entry;
// #[derive(Copy, Clone)]

pub type h2settings = crate::src::lib::altsvc::h2settings;
pub type Curl_recv = crate::src::lib::altsvc::Curl_recv;
pub type Curl_send = crate::src::lib::altsvc::Curl_send;
// #[derive(Copy, Clone)]

pub type ftp_conn = crate::src::lib::altsvc::ftp_conn;
pub type ftpstate = crate::src::lib::altsvc::ftpstate;
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

pub type ntlmdata = crate::src::lib::altsvc::ntlmdata;
pub type curlntlm = crate::src::lib::altsvc::curlntlm;
pub const NTLMSTATE_LAST: curlntlm = 4;
pub const NTLMSTATE_TYPE3: curlntlm = 3;
pub const NTLMSTATE_TYPE2: curlntlm = 2;
pub const NTLMSTATE_TYPE1: curlntlm = 1;
pub const NTLMSTATE_NONE: curlntlm = 0;
// #[derive(Copy, Clone)]

pub type gsasldata = crate::src::lib::altsvc::gsasldata;
// #[derive(Copy, Clone)]

pub type Curl_handler = crate::src::lib::altsvc::Curl_handler;
// #[derive(Copy, Clone, BitfieldStruct)]

pub type ConnectBits = crate::src::lib::altsvc::ConnectBits;
// #[derive(Copy, Clone, BitfieldStruct)]

pub type ssl_connect_data = crate::src::lib::altsvc::ssl_connect_data;
pub type ssl_connect_state = crate::src::lib::altsvc::ssl_connect_state;
pub const ssl_connect_done: ssl_connect_state = 5;
pub const ssl_connect_3: ssl_connect_state = 4;
pub const ssl_connect_2_writing: ssl_connect_state = 3;
pub const ssl_connect_2_reading: ssl_connect_state = 2;
pub const ssl_connect_2: ssl_connect_state = 1;
pub const ssl_connect_1: ssl_connect_state = 0;
pub type ssl_connection_state = crate::src::lib::altsvc::ssl_connection_state;
pub const ssl_connection_complete: ssl_connection_state = 2;
pub const ssl_connection_negotiating: ssl_connection_state = 1;
pub const ssl_connection_none: ssl_connection_state = 0;
// #[derive(Copy, Clone)]

pub type proxy_info = crate::src::lib::altsvc::proxy_info;
// #[derive(Copy, Clone)]

pub type hostname = crate::src::lib::altsvc::hostname;
pub type C2RustUnnamed_5 = u32;
pub const TRNSPRT_QUIC: C2RustUnnamed_5 = 5;
pub const TRNSPRT_UDP: C2RustUnnamed_5 = 4;
pub const TRNSPRT_TCP: C2RustUnnamed_5 = 3;
// #[derive(Copy, Clone)]

pub type Curl_chunker = crate::src::lib::altsvc::Curl_chunker;
pub type ChunkyState = crate::src::lib::altsvc::ChunkyState;
pub const CHUNK_TRAILER_POSTCR: ChunkyState = 7;
pub const CHUNK_TRAILER_CR: ChunkyState = 6;
pub const CHUNK_TRAILER: ChunkyState = 5;
pub const CHUNK_STOP: ChunkyState = 4;
pub const CHUNK_POSTLF: ChunkyState = 3;
pub const CHUNK_DATA: ChunkyState = 2;
pub const CHUNK_LF: ChunkyState = 1;
pub const CHUNK_HEX: ChunkyState = 0;
// #[derive(Copy, Clone)]

pub type connstate = crate::src::lib::altsvc::connstate;
pub type connect_t = crate::src::lib::altsvc::connect_t;
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
pub type curl_malloc_callback = crate::src::lib::altsvc::curl_malloc_callback;
pub type curl_strdup_callback = crate::src::lib::altsvc::curl_strdup_callback;
pub type curl_calloc_callback = crate::src::lib::altsvc::curl_calloc_callback;
pub type C2RustUnnamed_6 = u32;
pub const CURL_SSLVERSION_LAST: C2RustUnnamed_6 = 8;
pub const CURL_SSLVERSION_TLSv1_3: C2RustUnnamed_6 = 7;
pub const CURL_SSLVERSION_TLSv1_2: C2RustUnnamed_6 = 6;
pub const CURL_SSLVERSION_TLSv1_1: C2RustUnnamed_6 = 5;
pub const CURL_SSLVERSION_TLSv1_0: C2RustUnnamed_6 = 4;
pub const CURL_SSLVERSION_SSLv3: C2RustUnnamed_6 = 3;
pub const CURL_SSLVERSION_SSLv2: C2RustUnnamed_6 = 2;
pub const CURL_SSLVERSION_TLSv1: C2RustUnnamed_6 = 1;
pub const CURL_SSLVERSION_DEFAULT: C2RustUnnamed_6 = 0;
pub type C2RustUnnamed_7 = u32;
pub const CURL_SSLVERSION_MAX_LAST: C2RustUnnamed_7 = 524288;
pub const CURL_SSLVERSION_MAX_TLSv1_3: C2RustUnnamed_7 = 458752;
pub const CURL_SSLVERSION_MAX_TLSv1_2: C2RustUnnamed_7 = 393216;
pub const CURL_SSLVERSION_MAX_TLSv1_1: C2RustUnnamed_7 = 327680;
pub const CURL_SSLVERSION_MAX_TLSv1_0: C2RustUnnamed_7 = 262144;
pub const CURL_SSLVERSION_MAX_DEFAULT: C2RustUnnamed_7 = 65536;
pub const CURL_SSLVERSION_MAX_NONE: C2RustUnnamed_7 = 0;
// #[derive(Copy, Clone)]

pub type curl_ssl_backend = crate::src::lib::getinfo::curl_ssl_backend;
pub type CURLsslset = u32;
pub const CURLSSLSET_NO_BACKENDS: CURLsslset = 3;
pub const CURLSSLSET_TOO_LATE: CURLsslset = 2;
pub const CURLSSLSET_UNKNOWN_BACKEND: CURLsslset = 1;
pub const CURLSSLSET_OK: CURLsslset = 0;
// #[derive(Copy, Clone)]

pub type Curl_ssl = crate::src::lib::getinfo::Curl_ssl;
pub type CURLINFO = crate::src::lib::easy::CURLINFO;
pub const CURLINFO_LASTONE: CURLINFO = 60;
pub const CURLINFO_REFERER: CURLINFO = 1048636;
pub const CURLINFO_PROXY_ERROR: CURLINFO = 2097211;
pub const CURLINFO_EFFECTIVE_METHOD: CURLINFO = 1048634;
pub const CURLINFO_RETRY_AFTER: CURLINFO = 6291513;
pub const CURLINFO_APPCONNECT_TIME_T: CURLINFO = 6291512;
pub const CURLINFO_REDIRECT_TIME_T: CURLINFO = 6291511;
pub const CURLINFO_STARTTRANSFER_TIME_T: CURLINFO = 6291510;
pub const CURLINFO_PRETRANSFER_TIME_T: CURLINFO = 6291509;
pub const CURLINFO_CONNECT_TIME_T: CURLINFO = 6291508;
pub const CURLINFO_NAMELOOKUP_TIME_T: CURLINFO = 6291507;
pub const CURLINFO_TOTAL_TIME_T: CURLINFO = 6291506;
pub const CURLINFO_SCHEME: CURLINFO = 1048625;
pub const CURLINFO_PROTOCOL: CURLINFO = 2097200;
pub const CURLINFO_PROXY_SSL_VERIFYRESULT: CURLINFO = 2097199;
pub const CURLINFO_HTTP_VERSION: CURLINFO = 2097198;
pub const CURLINFO_TLS_SSL_PTR: CURLINFO = 4194349;
pub const CURLINFO_ACTIVESOCKET: CURLINFO = 5242924;
pub const CURLINFO_TLS_SESSION: CURLINFO = 4194347;
pub const CURLINFO_LOCAL_PORT: CURLINFO = 2097194;
pub const CURLINFO_LOCAL_IP: CURLINFO = 1048617;
pub const CURLINFO_PRIMARY_PORT: CURLINFO = 2097192;
pub const CURLINFO_RTSP_CSEQ_RECV: CURLINFO = 2097191;
pub const CURLINFO_RTSP_SERVER_CSEQ: CURLINFO = 2097190;
pub const CURLINFO_RTSP_CLIENT_CSEQ: CURLINFO = 2097189;
pub const CURLINFO_RTSP_SESSION_ID: CURLINFO = 1048612;
pub const CURLINFO_CONDITION_UNMET: CURLINFO = 2097187;
pub const CURLINFO_CERTINFO: CURLINFO = 4194338;
pub const CURLINFO_APPCONNECT_TIME: CURLINFO = 3145761;
pub const CURLINFO_PRIMARY_IP: CURLINFO = 1048608;
pub const CURLINFO_REDIRECT_URL: CURLINFO = 1048607;
pub const CURLINFO_FTP_ENTRY_PATH: CURLINFO = 1048606;
pub const CURLINFO_LASTSOCKET: CURLINFO = 2097181;
pub const CURLINFO_COOKIELIST: CURLINFO = 4194332;
pub const CURLINFO_SSL_ENGINES: CURLINFO = 4194331;
pub const CURLINFO_NUM_CONNECTS: CURLINFO = 2097178;
pub const CURLINFO_OS_ERRNO: CURLINFO = 2097177;
pub const CURLINFO_PROXYAUTH_AVAIL: CURLINFO = 2097176;
pub const CURLINFO_HTTPAUTH_AVAIL: CURLINFO = 2097175;
pub const CURLINFO_HTTP_CONNECTCODE: CURLINFO = 2097174;
pub const CURLINFO_PRIVATE: CURLINFO = 1048597;
pub const CURLINFO_REDIRECT_COUNT: CURLINFO = 2097172;
pub const CURLINFO_REDIRECT_TIME: CURLINFO = 3145747;
pub const CURLINFO_CONTENT_TYPE: CURLINFO = 1048594;
pub const CURLINFO_STARTTRANSFER_TIME: CURLINFO = 3145745;
pub const CURLINFO_CONTENT_LENGTH_UPLOAD_T: CURLINFO = 6291472;
pub const CURLINFO_CONTENT_LENGTH_UPLOAD: CURLINFO = 3145744;
pub const CURLINFO_CONTENT_LENGTH_DOWNLOAD_T: CURLINFO = 6291471;
pub const CURLINFO_CONTENT_LENGTH_DOWNLOAD: CURLINFO = 3145743;
pub const CURLINFO_FILETIME_T: CURLINFO = 6291470;
pub const CURLINFO_FILETIME: CURLINFO = 2097166;
pub const CURLINFO_SSL_VERIFYRESULT: CURLINFO = 2097165;
pub const CURLINFO_REQUEST_SIZE: CURLINFO = 2097164;
pub const CURLINFO_HEADER_SIZE: CURLINFO = 2097163;
pub const CURLINFO_SPEED_UPLOAD_T: CURLINFO = 6291466;
pub const CURLINFO_SPEED_UPLOAD: CURLINFO = 3145738;
pub const CURLINFO_SPEED_DOWNLOAD_T: CURLINFO = 6291465;
pub const CURLINFO_SPEED_DOWNLOAD: CURLINFO = 3145737;
pub const CURLINFO_SIZE_DOWNLOAD_T: CURLINFO = 6291464;
pub const CURLINFO_SIZE_DOWNLOAD: CURLINFO = 3145736;
pub const CURLINFO_SIZE_UPLOAD_T: CURLINFO = 6291463;
pub const CURLINFO_SIZE_UPLOAD: CURLINFO = 3145735;
pub const CURLINFO_PRETRANSFER_TIME: CURLINFO = 3145734;
pub const CURLINFO_CONNECT_TIME: CURLINFO = 3145733;
pub const CURLINFO_NAMELOOKUP_TIME: CURLINFO = 3145732;
pub const CURLINFO_TOTAL_TIME: CURLINFO = 3145731;
pub const CURLINFO_RESPONSE_CODE: CURLINFO = 2097154;
pub const CURLINFO_EFFECTIVE_URL: CURLINFO = 1048577;
pub const CURLINFO_NONE: CURLINFO = 0;
pub type CURLSHcode = crate::src::lib::conncache::CURLSHcode;
pub const CURLSHE_LAST: CURLSHcode = 6;
pub const CURLSHE_NOT_BUILT_IN: CURLSHcode = 5;
pub const CURLSHE_NOMEM: CURLSHcode = 4;
pub const CURLSHE_INVALID: CURLSHcode = 3;
pub const CURLSHE_IN_USE: CURLSHcode = 2;
pub const CURLSHE_BAD_OPTION: CURLSHcode = 1;
pub const CURLSHE_OK: CURLSHcode = 0;
pub type timerid = crate::src::lib::connect::timerid;
pub const TIMER_LAST: timerid = 11;
pub const TIMER_REDIRECT: timerid = 10;
pub const TIMER_STARTACCEPT: timerid = 9;
pub const TIMER_POSTRANSFER: timerid = 8;
pub const TIMER_STARTTRANSFER: timerid = 7;
pub const TIMER_PRETRANSFER: timerid = 6;
pub const TIMER_APPCONNECT: timerid = 5;
pub const TIMER_CONNECT: timerid = 4;
pub const TIMER_NAMELOOKUP: timerid = 3;
pub const TIMER_STARTSINGLE: timerid = 2;
pub const TIMER_STARTOP: timerid = 1;
pub const TIMER_NONE: timerid = 0;
unsafe extern "C" fn blobdup(
    mut dest: *mut *mut curl_blob,
    mut src: *mut curl_blob,
) -> CURLcode {
    if !src.is_null() {
        let mut d: *mut curl_blob = 0 as *mut curl_blob;
        d = Curl_cmalloc
            .expect(
                "non-null function pointer",
            )(
            (::std::mem::size_of::<curl_blob>() as u64)
                .wrapping_add((*src).len),
        ) as *mut curl_blob;
        if d.is_null() {
            return CURLE_OUT_OF_MEMORY;
        }
        (*d).len = (*src).len;
        (*d).flags = 1 as i32 as u32;
        let fresh0 = &mut ((*d).data);
        *fresh0 = (d as *mut i8)
            .offset(::std::mem::size_of::<curl_blob>() as u64 as isize)
            as *mut libc::c_void;
        memcpy((*d).data, (*src).data, (*src).len);
        *dest = d;
    }
    return CURLE_OK;
}
unsafe extern "C" fn blobcmp(
    mut first: *mut curl_blob,
    mut second: *mut curl_blob,
) -> bool {
    if first.is_null() && second.is_null() {
        return 1 as i32 != 0;
    }
    if first.is_null() || second.is_null() {
        return 0 as i32 != 0;
    }
    if (*first).len != (*second).len {
        return 0 as i32 != 0;
    }
    return memcmp((*first).data, (*second).data, (*first).len) == 0;
}
unsafe extern "C" fn safecmp(
    mut a: *mut i8,
    mut b: *mut i8,
) -> bool {
    if !a.is_null() && !b.is_null() {
        return strcmp(a, b) == 0
    } else {
        if a.is_null() && b.is_null() {
            return 1 as i32 != 0;
        }
    }
    return 0 as i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn Curl_ssl_config_matches(
    mut data: *mut ssl_primary_config,
    mut needle: *mut ssl_primary_config,
) -> bool {
    if (*data).version == (*needle).version
        && (*data).version_max == (*needle).version_max
        && (*data).verifypeer() as i32 == (*needle).verifypeer() as i32
        && (*data).verifyhost() as i32 == (*needle).verifyhost() as i32
        && (*data).verifystatus() as i32
            == (*needle).verifystatus() as i32
        && blobcmp((*data).cert_blob, (*needle).cert_blob) as i32 != 0
        && blobcmp((*data).ca_info_blob, (*needle).ca_info_blob) as i32 != 0
        && blobcmp((*data).issuercert_blob, (*needle).issuercert_blob) as i32
            != 0 && safecmp((*data).CApath, (*needle).CApath) as i32 != 0
        && safecmp((*data).CAfile, (*needle).CAfile) as i32 != 0
        && safecmp((*data).issuercert, (*needle).issuercert) as i32 != 0
        && safecmp((*data).clientcert, (*needle).clientcert) as i32 != 0
        && safecmp((*data).random_file, (*needle).random_file) as i32 != 0
        && safecmp((*data).egdsocket, (*needle).egdsocket) as i32 != 0
        && Curl_safe_strcasecompare((*data).cipher_list, (*needle).cipher_list) != 0
        && Curl_safe_strcasecompare((*data).cipher_list13, (*needle).cipher_list13) != 0
        && Curl_safe_strcasecompare((*data).curves, (*needle).curves) != 0
        && Curl_safe_strcasecompare((*data).pinned_key, (*needle).pinned_key) != 0
    {
        return 1 as i32 != 0;
    }
    return 0 as i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn Curl_clone_primary_ssl_config(
    mut source: *mut ssl_primary_config,
    mut dest: *mut ssl_primary_config,
) -> bool {
    (*dest).version = (*source).version;
    (*dest).version_max = (*source).version_max;
    (*dest).set_verifypeer((*source).verifypeer());
    (*dest).set_verifyhost((*source).verifyhost());
    (*dest).set_verifystatus((*source).verifystatus());
    (*dest).set_sessionid((*source).sessionid());
    if blobdup(&mut (*dest).cert_blob, (*source).cert_blob) as u64 != 0 {
        return 0 as i32 != 0;
    }
    if blobdup(&mut (*dest).ca_info_blob, (*source).ca_info_blob) as u64 != 0 {
        return 0 as i32 != 0;
    }
    if blobdup(&mut (*dest).issuercert_blob, (*source).issuercert_blob) as u64 != 0 {
        return 0 as i32 != 0;
    }
    if !((*source).CApath).is_null() {
        let fresh1 = &mut ((*dest).CApath);
        *fresh1 = Curl_cstrdup.expect("non-null function pointer")((*source).CApath);
        if ((*dest).CApath).is_null() {
            return 0 as i32 != 0;
        }
    } else {
        let fresh2 = &mut ((*dest).CApath);
        *fresh2 = 0 as *mut i8;
    }
    if !((*source).CAfile).is_null() {
        let fresh3 = &mut ((*dest).CAfile);
        *fresh3 = Curl_cstrdup.expect("non-null function pointer")((*source).CAfile);
        if ((*dest).CAfile).is_null() {
            return 0 as i32 != 0;
        }
    } else {
        let fresh4 = &mut ((*dest).CAfile);
        *fresh4 = 0 as *mut i8;
    }
    if !((*source).issuercert).is_null() {
        let fresh5 = &mut ((*dest).issuercert);
        *fresh5 = Curl_cstrdup.expect("non-null function pointer")((*source).issuercert);
        if ((*dest).issuercert).is_null() {
            return 0 as i32 != 0;
        }
    } else {
        let fresh6 = &mut ((*dest).issuercert);
        *fresh6 = 0 as *mut i8;
    }
    if !((*source).clientcert).is_null() {
        let fresh7 = &mut ((*dest).clientcert);
        *fresh7 = Curl_cstrdup.expect("non-null function pointer")((*source).clientcert);
        if ((*dest).clientcert).is_null() {
            return 0 as i32 != 0;
        }
    } else {
        let fresh8 = &mut ((*dest).clientcert);
        *fresh8 = 0 as *mut i8;
    }
    if !((*source).random_file).is_null() {
        let fresh9 = &mut ((*dest).random_file);
        *fresh9 = Curl_cstrdup
            .expect("non-null function pointer")((*source).random_file);
        if ((*dest).random_file).is_null() {
            return 0 as i32 != 0;
        }
    } else {
        let fresh10 = &mut ((*dest).random_file);
        *fresh10 = 0 as *mut i8;
    }
    if !((*source).egdsocket).is_null() {
        let fresh11 = &mut ((*dest).egdsocket);
        *fresh11 = Curl_cstrdup.expect("non-null function pointer")((*source).egdsocket);
        if ((*dest).egdsocket).is_null() {
            return 0 as i32 != 0;
        }
    } else {
        let fresh12 = &mut ((*dest).egdsocket);
        *fresh12 = 0 as *mut i8;
    }
    if !((*source).cipher_list).is_null() {
        let fresh13 = &mut ((*dest).cipher_list);
        *fresh13 = Curl_cstrdup
            .expect("non-null function pointer")((*source).cipher_list);
        if ((*dest).cipher_list).is_null() {
            return 0 as i32 != 0;
        }
    } else {
        let fresh14 = &mut ((*dest).cipher_list);
        *fresh14 = 0 as *mut i8;
    }
    if !((*source).cipher_list13).is_null() {
        let fresh15 = &mut ((*dest).cipher_list13);
        *fresh15 = Curl_cstrdup
            .expect("non-null function pointer")((*source).cipher_list13);
        if ((*dest).cipher_list13).is_null() {
            return 0 as i32 != 0;
        }
    } else {
        let fresh16 = &mut ((*dest).cipher_list13);
        *fresh16 = 0 as *mut i8;
    }
    if !((*source).pinned_key).is_null() {
        let fresh17 = &mut ((*dest).pinned_key);
        *fresh17 = Curl_cstrdup
            .expect("non-null function pointer")((*source).pinned_key);
        if ((*dest).pinned_key).is_null() {
            return 0 as i32 != 0;
        }
    } else {
        let fresh18 = &mut ((*dest).pinned_key);
        *fresh18 = 0 as *mut i8;
    }
    if !((*source).curves).is_null() {
        let fresh19 = &mut ((*dest).curves);
        *fresh19 = Curl_cstrdup.expect("non-null function pointer")((*source).curves);
        if ((*dest).curves).is_null() {
            return 0 as i32 != 0;
        }
    } else {
        let fresh20 = &mut ((*dest).curves);
        *fresh20 = 0 as *mut i8;
    }
    return 1 as i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn Curl_free_primary_ssl_config(
    mut sslc: *mut ssl_primary_config,
) {
    Curl_cfree.expect("non-null function pointer")((*sslc).CApath as *mut libc::c_void);
    let fresh21 = &mut ((*sslc).CApath);
    *fresh21 = 0 as *mut i8;
    Curl_cfree.expect("non-null function pointer")((*sslc).CAfile as *mut libc::c_void);
    let fresh22 = &mut ((*sslc).CAfile);
    *fresh22 = 0 as *mut i8;
    Curl_cfree
        .expect("non-null function pointer")((*sslc).issuercert as *mut libc::c_void);
    let fresh23 = &mut ((*sslc).issuercert);
    *fresh23 = 0 as *mut i8;
    Curl_cfree
        .expect("non-null function pointer")((*sslc).clientcert as *mut libc::c_void);
    let fresh24 = &mut ((*sslc).clientcert);
    *fresh24 = 0 as *mut i8;
    Curl_cfree
        .expect("non-null function pointer")((*sslc).random_file as *mut libc::c_void);
    let fresh25 = &mut ((*sslc).random_file);
    *fresh25 = 0 as *mut i8;
    Curl_cfree
        .expect("non-null function pointer")((*sslc).egdsocket as *mut libc::c_void);
    let fresh26 = &mut ((*sslc).egdsocket);
    *fresh26 = 0 as *mut i8;
    Curl_cfree
        .expect("non-null function pointer")((*sslc).cipher_list as *mut libc::c_void);
    let fresh27 = &mut ((*sslc).cipher_list);
    *fresh27 = 0 as *mut i8;
    Curl_cfree
        .expect("non-null function pointer")((*sslc).cipher_list13 as *mut libc::c_void);
    let fresh28 = &mut ((*sslc).cipher_list13);
    *fresh28 = 0 as *mut i8;
    Curl_cfree
        .expect("non-null function pointer")((*sslc).pinned_key as *mut libc::c_void);
    let fresh29 = &mut ((*sslc).pinned_key);
    *fresh29 = 0 as *mut i8;
    Curl_cfree
        .expect("non-null function pointer")((*sslc).cert_blob as *mut libc::c_void);
    let fresh30 = &mut ((*sslc).cert_blob);
    *fresh30 = 0 as *mut curl_blob;
    Curl_cfree
        .expect("non-null function pointer")((*sslc).ca_info_blob as *mut libc::c_void);
    let fresh31 = &mut ((*sslc).ca_info_blob);
    *fresh31 = 0 as *mut curl_blob;
    Curl_cfree
        .expect(
            "non-null function pointer",
        )((*sslc).issuercert_blob as *mut libc::c_void);
    let fresh32 = &mut ((*sslc).issuercert_blob);
    *fresh32 = 0 as *mut curl_blob;
    Curl_cfree.expect("non-null function pointer")((*sslc).curves as *mut libc::c_void);
    let fresh33 = &mut ((*sslc).curves);
    *fresh33 = 0 as *mut i8;
}
#[no_mangle]
pub unsafe extern "C" fn Curl_ssl_backend() -> i32 {
    multissl_setup(0 as *const Curl_ssl);
    return (*Curl_ssl).info.id as i32;
}
static mut init_ssl: bool = 0 as i32 != 0;
#[no_mangle]
pub unsafe extern "C" fn Curl_ssl_init() -> i32 {
    if init_ssl {
        return 1 as i32;
    }
    init_ssl = 1 as i32 != 0;
    return ((*Curl_ssl).init).expect("non-null function pointer")();
}
#[no_mangle]
pub unsafe extern "C" fn Curl_ssl_cleanup() {
    if init_ssl {
        ((*Curl_ssl).cleanup).expect("non-null function pointer")();
        init_ssl = 0 as i32 != 0;
    }
}
unsafe extern "C" fn ssl_prefs_check(mut data: *mut Curl_easy) -> bool {
    let sslver: i64 = (*data).set.ssl.primary.version;
    if sslver < 0 as i32 as i64
        || sslver >= CURL_SSLVERSION_LAST as i32 as i64
    {
        Curl_failf(
            data,
            b"Unrecognized parameter value passed via CURLOPT_SSLVERSION\0" as *const u8
                as *const i8,
        );
        return 0 as i32 != 0;
    }
    match (*data).set.ssl.primary.version_max {
        0 | 65536 => {}
        _ => {
            if ((*data).set.ssl.primary.version_max >> 16 as i32) < sslver {
                Curl_failf(
                    data,
                    b"CURL_SSLVERSION_MAX incompatible with CURL_SSLVERSION\0"
                        as *const u8 as *const i8,
                );
                return 0 as i32 != 0;
            }
        }
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn ssl_connect_init_proxy(
    mut conn: *mut connectdata,
    mut sockindex: i32,
) -> CURLcode {
    if ssl_connection_complete as i32 as u32
        == (*conn).ssl[sockindex as usize].state as u32
        && ((*conn).proxy_ssl[sockindex as usize]).use_0() == 0
    {
        let mut pbdata: *mut ssl_backend_data = 0 as *mut ssl_backend_data;
        if (*Curl_ssl).supports
            & ((1 as i32) << 4 as i32) as u32 == 0
        {
            return CURLE_NOT_BUILT_IN;
        }
        pbdata = (*conn).proxy_ssl[sockindex as usize].backend;
        (*conn).proxy_ssl[sockindex as usize] = (*conn).ssl[sockindex as usize];
        memset(
            &mut *((*conn).ssl).as_mut_ptr().offset(sockindex as isize)
                as *mut ssl_connect_data as *mut libc::c_void,
            0 as i32,
            ::std::mem::size_of::<ssl_connect_data>() as u64,
        );
        memset(
            pbdata as *mut libc::c_void,
            0 as i32,
            (*Curl_ssl).sizeof_ssl_backend_data,
        );
        let fresh34 = &mut ((*conn).ssl[sockindex as usize].backend);
        *fresh34 = pbdata;
    }
    return CURLE_OK;
}
#[no_mangle]
pub unsafe extern "C" fn Curl_ssl_connect(
    mut data: *mut Curl_easy,
    mut conn: *mut connectdata,
    mut sockindex: i32,
) -> CURLcode {
    let mut result: CURLcode = CURLE_OK;
    if (*conn).bits.proxy_ssl_connected[sockindex as usize] {
        result = ssl_connect_init_proxy(conn, sockindex);
        if result as u64 != 0 {
            return result;
        }
    }
    if !ssl_prefs_check(data) {
        return CURLE_SSL_CONNECT_ERROR;
    }
    let fresh35 = &mut ((*conn).ssl[sockindex as usize]);
    (*fresh35).set_use_0(1 as i32 as bit);
    (*conn).ssl[sockindex as usize].state = ssl_connection_negotiating;
    result = ((*Curl_ssl).connect_blocking)
        .expect("non-null function pointer")(data, conn, sockindex);
    if result as u64 == 0 {
        Curl_pgrsTime(data, TIMER_APPCONNECT);
    } else {
        let fresh36 = &mut ((*conn).ssl[sockindex as usize]);
        (*fresh36).set_use_0(0 as i32 as bit);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn Curl_ssl_connect_nonblocking(
    mut data: *mut Curl_easy,
    mut conn: *mut connectdata,
    mut isproxy: bool,
    mut sockindex: i32,
    mut done: *mut bool,
) -> CURLcode {
    let mut result: CURLcode = CURLE_OK;
    if (*conn).bits.proxy_ssl_connected[sockindex as usize] {
        result = ssl_connect_init_proxy(conn, sockindex);
        if result as u64 != 0 {
            return result;
        }
    }
    if !ssl_prefs_check(data) {
        return CURLE_SSL_CONNECT_ERROR;
    }
    let fresh37 = &mut ((*conn).ssl[sockindex as usize]);
    (*fresh37).set_use_0(1 as i32 as bit);
    result = ((*Curl_ssl).connect_nonblocking)
        .expect("non-null function pointer")(data, conn, sockindex, done);
    if result as u64 != 0 {
        let fresh38 = &mut ((*conn).ssl[sockindex as usize]);
        (*fresh38).set_use_0(0 as i32 as bit);
    } else if *done as i32 != 0 && !isproxy {
        Curl_pgrsTime(data, TIMER_APPCONNECT);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn Curl_ssl_sessionid_lock(mut data: *mut Curl_easy) {
    if !((*data).share).is_null()
        && (*(*data).share).specifier
            & ((1 as i32) << CURL_LOCK_DATA_SSL_SESSION as i32)
                as u32 != 0
    {
        Curl_share_lock(data, CURL_LOCK_DATA_SSL_SESSION, CURL_LOCK_ACCESS_SINGLE);
    }
}
#[no_mangle]
pub unsafe extern "C" fn Curl_ssl_sessionid_unlock(mut data: *mut Curl_easy) {
    if !((*data).share).is_null()
        && (*(*data).share).specifier
            & ((1 as i32) << CURL_LOCK_DATA_SSL_SESSION as i32)
                as u32 != 0
    {
        Curl_share_unlock(data, CURL_LOCK_DATA_SSL_SESSION);
    }
}
#[no_mangle]
pub unsafe extern "C" fn Curl_ssl_getsessionid(
    mut data: *mut Curl_easy,
    mut conn: *mut connectdata,
    isProxy: bool,
    mut ssl_sessionid: *mut *mut libc::c_void,
    mut idsize: *mut size_t,
    mut sockindex: i32,
) -> bool {
    let mut check: *mut Curl_ssl_session = 0 as *mut Curl_ssl_session;
    let mut i: size_t = 0;
    let mut general_age: *mut i64 = 0 as *mut i64;
    let mut no_match: bool = 1 as i32 != 0;
    let ssl_config: *mut ssl_primary_config = if isProxy as i32 != 0 {
        &mut (*conn).proxy_ssl_config
    } else {
        &mut (*conn).ssl_config
    };
    let name: *const i8 = if isProxy as i32 != 0 {
        (*conn).http_proxy.host.name
    } else {
        (*conn).host.name
    };
    let mut port: i32 = if isProxy as i32 != 0 {
        (*conn).port
    } else {
        (*conn).remote_port
    };
    *ssl_sessionid = 0 as *mut libc::c_void;
    if (if CURLPROXY_HTTPS as i32 as u32
        == (*conn).http_proxy.proxytype as u32
        && ssl_connection_complete as i32 as u32
            != (*conn)
                .proxy_ssl[(if (*conn).sock[1 as i32 as usize]
                    == -(1 as i32)
                {
                    0 as i32
                } else {
                    1 as i32
                }) as usize]
                .state as u32
    {
        ((*data).set.proxy_ssl.primary).sessionid() as i32
    } else {
        ((*data).set.ssl.primary).sessionid() as i32
    }) == 0 || ((*data).state.session).is_null()
    {
        return 1 as i32 != 0;
    }
    if !((*data).share).is_null()
        && (*(*data).share).specifier
            & ((1 as i32) << CURL_LOCK_DATA_SSL_SESSION as i32)
                as u32 != 0
    {
        general_age = &mut (*(*data).share).sessionage;
    } else {
        general_age = &mut (*data).state.sessionage;
    }
    i = 0 as i32 as size_t;
    while i < (*data).set.general_ssl.max_ssl_sessions {
        check = &mut *((*data).state.session).offset(i as isize)
            as *mut Curl_ssl_session;
        if !((*check).sessionid).is_null() {
            if Curl_strcasecompare(name, (*check).name) != 0
                && (((*conn).bits).conn_to_host() == 0
                    && ((*check).conn_to_host).is_null()
                    || ((*conn).bits).conn_to_host() as i32 != 0
                        && !((*check).conn_to_host).is_null()
                        && Curl_strcasecompare(
                            (*conn).conn_to_host.name,
                            (*check).conn_to_host,
                        ) != 0)
                && (((*conn).bits).conn_to_port() == 0
                    && (*check).conn_to_port == -(1 as i32)
                    || ((*conn).bits).conn_to_port() as i32 != 0
                        && (*check).conn_to_port != -(1 as i32)
                        && (*conn).conn_to_port == (*check).conn_to_port)
                && port == (*check).remote_port
                && Curl_strcasecompare((*(*conn).handler).scheme, (*check).scheme) != 0
                && Curl_ssl_config_matches(ssl_config, &mut (*check).ssl_config)
                    as i32 != 0
            {
                *general_age += 1;
                (*check).age = *general_age;
                *ssl_sessionid = (*check).sessionid;
                if !idsize.is_null() {
                    *idsize = (*check).idsize;
                }
                no_match = 0 as i32 != 0;
                break;
            }
        }
        i = i.wrapping_add(1);
    }
    return no_match;
}
#[no_mangle]
pub unsafe extern "C" fn Curl_ssl_kill_session(mut session: *mut Curl_ssl_session) {
    if !((*session).sessionid).is_null() {
        ((*Curl_ssl).session_free)
            .expect("non-null function pointer")((*session).sessionid);
        let fresh39 = &mut ((*session).sessionid);
        *fresh39 = 0 as *mut libc::c_void;
        (*session).age = 0 as i32 as i64;
        Curl_free_primary_ssl_config(&mut (*session).ssl_config);
        Curl_cfree
            .expect("non-null function pointer")((*session).name as *mut libc::c_void);
        let fresh40 = &mut ((*session).name);
        *fresh40 = 0 as *mut i8;
        Curl_cfree
            .expect(
                "non-null function pointer",
            )((*session).conn_to_host as *mut libc::c_void);
        let fresh41 = &mut ((*session).conn_to_host);
        *fresh41 = 0 as *mut i8;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Curl_ssl_delsessionid(
    mut data: *mut Curl_easy,
    mut ssl_sessionid: *mut libc::c_void,
) {
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < (*data).set.general_ssl.max_ssl_sessions {
        let mut check: *mut Curl_ssl_session = &mut *((*data).state.session)
            .offset(i as isize) as *mut Curl_ssl_session;
        if (*check).sessionid == ssl_sessionid {
            Curl_ssl_kill_session(check);
            break;
        } else {
            i = i.wrapping_add(1);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn Curl_ssl_addsessionid(
    mut data: *mut Curl_easy,
    mut conn: *mut connectdata,
    isProxy: bool,
    mut ssl_sessionid: *mut libc::c_void,
    mut idsize: size_t,
    mut sockindex: i32,
) -> CURLcode {
    let mut i: size_t = 0;
    let mut store: *mut Curl_ssl_session = 0 as *mut Curl_ssl_session;
    let mut oldest_age: i64 = 0;
    let mut clone_host: *mut i8 = 0 as *mut i8;
    let mut clone_conn_to_host: *mut i8 = 0 as *mut i8;
    let mut conn_to_port: i32 = 0;
    let mut general_age: *mut i64 = 0 as *mut i64;
    let ssl_config: *mut ssl_primary_config = if isProxy as i32 != 0 {
        &mut (*conn).proxy_ssl_config
    } else {
        &mut (*conn).ssl_config
    };
    let mut hostname: *const i8 = if isProxy as i32 != 0 {
        (*conn).http_proxy.host.name
    } else {
        (*conn).host.name
    };
    if ((*data).state.session).is_null() {
        return CURLE_OK;
    }
    store = &mut *((*data).state.session).offset(0 as i32 as isize)
        as *mut Curl_ssl_session;
    oldest_age = (*((*data).state.session).offset(0 as i32 as isize)).age;
    clone_host = Curl_cstrdup.expect("non-null function pointer")(hostname);
    if clone_host.is_null() {
        return CURLE_OUT_OF_MEMORY;
    }
    if ((*conn).bits).conn_to_host() != 0 {
        clone_conn_to_host = Curl_cstrdup
            .expect("non-null function pointer")((*conn).conn_to_host.name);
        if clone_conn_to_host.is_null() {
            Curl_cfree
                .expect("non-null function pointer")(clone_host as *mut libc::c_void);
            return CURLE_OUT_OF_MEMORY;
        }
    } else {
        clone_conn_to_host = 0 as *mut i8;
    }
    if ((*conn).bits).conn_to_port() != 0 {
        conn_to_port = (*conn).conn_to_port;
    } else {
        conn_to_port = -(1 as i32);
    }
    if !((*data).share).is_null()
        && (*(*data).share).specifier
            & ((1 as i32) << CURL_LOCK_DATA_SSL_SESSION as i32)
                as u32 != 0
    {
        general_age = &mut (*(*data).share).sessionage;
    } else {
        general_age = &mut (*data).state.sessionage;
    }
    i = 1 as i32 as size_t;
    while i < (*data).set.general_ssl.max_ssl_sessions
        && !((*((*data).state.session).offset(i as isize)).sessionid).is_null()
    {
        if (*((*data).state.session).offset(i as isize)).age < oldest_age {
            oldest_age = (*((*data).state.session).offset(i as isize)).age;
            store = &mut *((*data).state.session).offset(i as isize)
                as *mut Curl_ssl_session;
        }
        i = i.wrapping_add(1);
    }
    if i == (*data).set.general_ssl.max_ssl_sessions {
        Curl_ssl_kill_session(store);
    } else {
        store = &mut *((*data).state.session).offset(i as isize)
            as *mut Curl_ssl_session;
    }
    let fresh42 = &mut ((*store).sessionid);
    *fresh42 = ssl_sessionid;
    (*store).idsize = idsize;
    (*store).age = *general_age;
    Curl_cfree.expect("non-null function pointer")((*store).name as *mut libc::c_void);
    Curl_cfree
        .expect("non-null function pointer")((*store).conn_to_host as *mut libc::c_void);
    let fresh43 = &mut ((*store).name);
    *fresh43 = clone_host;
    let fresh44 = &mut ((*store).conn_to_host);
    *fresh44 = clone_conn_to_host;
    (*store).conn_to_port = conn_to_port;
    (*store)
        .remote_port = if isProxy as i32 != 0 {
        (*conn).port
    } else {
        (*conn).remote_port
    };
    let fresh45 = &mut ((*store).scheme);
    *fresh45 = (*(*conn).handler).scheme;
    if !Curl_clone_primary_ssl_config(ssl_config, &mut (*store).ssl_config) {
        Curl_free_primary_ssl_config(&mut (*store).ssl_config);
        let fresh46 = &mut ((*store).sessionid);
        *fresh46 = 0 as *mut libc::c_void;
        Curl_cfree.expect("non-null function pointer")(clone_host as *mut libc::c_void);
        Curl_cfree
            .expect(
                "non-null function pointer",
            )(clone_conn_to_host as *mut libc::c_void);
        return CURLE_OUT_OF_MEMORY;
    }
    return CURLE_OK;
}
#[no_mangle]
pub unsafe extern "C" fn Curl_ssl_associate_conn(
    mut data: *mut Curl_easy,
    mut conn: *mut connectdata,
) {
    if ((*Curl_ssl).associate_connection).is_some() {
        ((*Curl_ssl).associate_connection)
            .expect("non-null function pointer")(data, conn, 0 as i32);
        if (*conn).sock[1 as i32 as usize] != 0
            && ((*conn).bits).sock_accepted() as i32 != 0
        {
            ((*Curl_ssl).associate_connection)
                .expect("non-null function pointer")(data, conn, 1 as i32);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn Curl_ssl_detach_conn(
    mut data: *mut Curl_easy,
    mut conn: *mut connectdata,
) {
    if ((*Curl_ssl).disassociate_connection).is_some() {
        ((*Curl_ssl).disassociate_connection)
            .expect("non-null function pointer")(data, 0 as i32);
        if (*conn).sock[1 as i32 as usize] != 0
            && ((*conn).bits).sock_accepted() as i32 != 0
        {
            ((*Curl_ssl).disassociate_connection)
                .expect("non-null function pointer")(data, 1 as i32);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn Curl_ssl_close_all(mut data: *mut Curl_easy) {
    if !((*data).state.session).is_null()
        && !(!((*data).share).is_null()
            && (*(*data).share).specifier
                & ((1 as i32) << CURL_LOCK_DATA_SSL_SESSION as i32)
                    as u32 != 0)
    {
        let mut i: size_t = 0;
        i = 0 as i32 as size_t;
        while i < (*data).set.general_ssl.max_ssl_sessions {
            Curl_ssl_kill_session(&mut *((*data).state.session).offset(i as isize));
            i = i.wrapping_add(1);
        }
        Curl_cfree
            .expect(
                "non-null function pointer",
            )((*data).state.session as *mut libc::c_void);
        let fresh47 = &mut ((*data).state.session);
        *fresh47 = 0 as *mut Curl_ssl_session;
    }
    ((*Curl_ssl).close_all).expect("non-null function pointer")(data);
}
#[no_mangle]
pub unsafe extern "C" fn Curl_ssl_getsock(
    mut conn: *mut connectdata,
    mut socks: *mut curl_socket_t,
) -> i32 {
    let mut connssl: *mut ssl_connect_data = &mut *((*conn).ssl)
        .as_mut_ptr()
        .offset(0 as i32 as isize) as *mut ssl_connect_data;
    if (*connssl).connecting_state as u32
        == ssl_connect_2_writing as i32 as u32
    {
        *socks
            .offset(0 as i32 as isize) = (*conn).sock[0 as i32 as usize];
        return (1 as i32) << 16 as i32 + 0 as i32;
    }
    if (*connssl).connecting_state as u32
        == ssl_connect_2_reading as i32 as u32
    {
        *socks
            .offset(0 as i32 as isize) = (*conn).sock[0 as i32 as usize];
        return (1 as i32) << 0 as i32;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn Curl_ssl_close(
    mut data: *mut Curl_easy,
    mut conn: *mut connectdata,
    mut sockindex: i32,
) {
    ((*Curl_ssl).close_one).expect("non-null function pointer")(data, conn, sockindex);
    (*conn).ssl[sockindex as usize].state = ssl_connection_none;
}
#[no_mangle]
pub unsafe extern "C" fn Curl_ssl_shutdown(
    mut data: *mut Curl_easy,
    mut conn: *mut connectdata,
    mut sockindex: i32,
) -> CURLcode {
    if ((*Curl_ssl).shut_down).expect("non-null function pointer")(data, conn, sockindex)
        != 0
    {
        return CURLE_SSL_SHUTDOWN_FAILED;
    }
    let fresh48 = &mut ((*conn).ssl[sockindex as usize]);
    (*fresh48).set_use_0(0 as i32 as bit);
    (*conn).ssl[sockindex as usize].state = ssl_connection_none;
    let fresh49 = &mut ((*conn).recv[sockindex as usize]);
    *fresh49 = Some(
        Curl_recv_plain
            as unsafe extern "C" fn(
                *mut Curl_easy,
                i32,
                *mut i8,
                size_t,
                *mut CURLcode,
            ) -> ssize_t,
    );
    let fresh50 = &mut ((*conn).send[sockindex as usize]);
    *fresh50 = Some(
        Curl_send_plain
            as unsafe extern "C" fn(
                *mut Curl_easy,
                i32,
                *const libc::c_void,
                size_t,
                *mut CURLcode,
            ) -> ssize_t,
    );
    return CURLE_OK;
}
#[no_mangle]
pub unsafe extern "C" fn Curl_ssl_set_engine(
    mut data: *mut Curl_easy,
    mut engine: *const i8,
) -> CURLcode {
    return ((*Curl_ssl).set_engine).expect("non-null function pointer")(data, engine);
}
#[no_mangle]
pub unsafe extern "C" fn Curl_ssl_set_engine_default(
    mut data: *mut Curl_easy,
) -> CURLcode {
    return ((*Curl_ssl).set_engine_default).expect("non-null function pointer")(data);
}
#[no_mangle]
pub unsafe extern "C" fn Curl_ssl_engines_list(
    mut data: *mut Curl_easy,
) -> *mut curl_slist {
    return ((*Curl_ssl).engines_list).expect("non-null function pointer")(data);
}
#[no_mangle]
pub unsafe extern "C" fn Curl_ssl_initsessions(
    mut data: *mut Curl_easy,
    mut amount: size_t,
) -> CURLcode {
    let mut session: *mut Curl_ssl_session = 0 as *mut Curl_ssl_session;
    if !((*data).state.session).is_null() {
        return CURLE_OK;
    }
    session = Curl_ccalloc
        .expect(
            "non-null function pointer",
        )(amount, ::std::mem::size_of::<Curl_ssl_session>() as u64)
        as *mut Curl_ssl_session;
    if session.is_null() {
        return CURLE_OUT_OF_MEMORY;
    }
    (*data).set.general_ssl.max_ssl_sessions = amount;
    let fresh51 = &mut ((*data).state.session);
    *fresh51 = session;
    (*data).state.sessionage = 1 as i32 as i64;
    return CURLE_OK;
}
#[no_mangle]
pub unsafe extern "C" fn Curl_ssl_version(
    mut buffer: *mut i8,
    mut size: size_t,
) {
    ((*Curl_ssl).version).expect("non-null function pointer")(buffer, size);
}
#[no_mangle]
pub unsafe extern "C" fn Curl_ssl_check_cxn(mut conn: *mut connectdata) -> i32 {
    return ((*Curl_ssl).check_cxn).expect("non-null function pointer")(conn);
}
#[no_mangle]
pub unsafe extern "C" fn Curl_ssl_data_pending(
    mut conn: *const connectdata,
    mut connindex: i32,
) -> bool {
    return ((*Curl_ssl).data_pending)
        .expect("non-null function pointer")(conn, connindex);
}
#[no_mangle]
pub unsafe extern "C" fn Curl_ssl_free_certinfo(mut data: *mut Curl_easy) {
    let mut ci: *mut curl_certinfo = &mut (*data).info.certs;
    if (*ci).num_of_certs != 0 {
        let mut i: i32 = 0;
        i = 0 as i32;
        while i < (*ci).num_of_certs {
            curl_slist_free_all(*((*ci).certinfo).offset(i as isize));
            let fresh52 = &mut (*((*ci).certinfo).offset(i as isize));
            *fresh52 = 0 as *mut curl_slist;
            i += 1;
        }
        Curl_cfree
            .expect("non-null function pointer")((*ci).certinfo as *mut libc::c_void);
        let fresh53 = &mut ((*ci).certinfo);
        *fresh53 = 0 as *mut *mut curl_slist;
        (*ci).num_of_certs = 0 as i32;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Curl_ssl_init_certinfo(
    mut data: *mut Curl_easy,
    mut num: i32,
) -> CURLcode {
    let mut ci: *mut curl_certinfo = &mut (*data).info.certs;
    let mut table: *mut *mut curl_slist = 0 as *mut *mut curl_slist;
    Curl_ssl_free_certinfo(data);
    table = Curl_ccalloc
        .expect(
            "non-null function pointer",
        )(num as size_t, ::std::mem::size_of::<*mut curl_slist>() as u64)
        as *mut *mut curl_slist;
    if table.is_null() {
        return CURLE_OUT_OF_MEMORY;
    }
    (*ci).num_of_certs = num;
    let fresh54 = &mut ((*ci).certinfo);
    *fresh54 = table;
    return CURLE_OK;
}
#[no_mangle]
pub unsafe extern "C" fn Curl_ssl_push_certinfo_len(
    mut data: *mut Curl_easy,
    mut certnum: i32,
    mut label: *const i8,
    mut value: *const i8,
    mut valuelen: size_t,
) -> CURLcode {
    let mut ci: *mut curl_certinfo = &mut (*data).info.certs;
    let mut output: *mut i8 = 0 as *mut i8;
    let mut nl: *mut curl_slist = 0 as *mut curl_slist;
    let mut result: CURLcode = CURLE_OK;
    let mut labellen: size_t = strlen(label);
    let mut outlen: size_t = labellen
        .wrapping_add(1 as i32 as u64)
        .wrapping_add(valuelen)
        .wrapping_add(1 as i32 as u64);
    output = Curl_cmalloc.expect("non-null function pointer")(outlen)
        as *mut i8;
    if output.is_null() {
        return CURLE_OUT_OF_MEMORY;
    }
    curl_msnprintf(output, outlen, b"%s:\0" as *const u8 as *const i8, label);
    memcpy(
        &mut *output
            .offset(labellen.wrapping_add(1 as i32 as u64) as isize)
            as *mut i8 as *mut libc::c_void,
        value as *const libc::c_void,
        valuelen,
    );
    *output
        .offset(
            labellen
                .wrapping_add(1 as i32 as u64)
                .wrapping_add(valuelen) as isize,
        ) = 0 as i32 as i8;
    nl = Curl_slist_append_nodup(*((*ci).certinfo).offset(certnum as isize), output);
    if nl.is_null() {
        Curl_cfree.expect("non-null function pointer")(output as *mut libc::c_void);
        curl_slist_free_all(*((*ci).certinfo).offset(certnum as isize));
        result = CURLE_OUT_OF_MEMORY;
    }
    let fresh55 = &mut (*((*ci).certinfo).offset(certnum as isize));
    *fresh55 = nl;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn Curl_ssl_push_certinfo(
    mut data: *mut Curl_easy,
    mut certnum: i32,
    mut label: *const i8,
    mut value: *const i8,
) -> CURLcode {
    let mut valuelen: size_t = strlen(value);
    return Curl_ssl_push_certinfo_len(data, certnum, label, value, valuelen);
}
#[no_mangle]
pub unsafe extern "C" fn Curl_ssl_random(
    mut data: *mut Curl_easy,
    mut entropy: *mut u8,
    mut length: size_t,
) -> CURLcode {
    return ((*Curl_ssl).random)
        .expect("non-null function pointer")(data, entropy, length);
}
unsafe extern "C" fn pubkey_pem_to_der(
    mut pem: *const i8,
    mut der: *mut *mut u8,
    mut der_len: *mut size_t,
) -> CURLcode {
    let mut stripped_pem: *mut i8 = 0 as *mut i8;
    let mut begin_pos: *mut i8 = 0 as *mut i8;
    let mut end_pos: *mut i8 = 0 as *mut i8;
    let mut pem_count: size_t = 0;
    let mut stripped_pem_count: size_t = 0 as i32 as size_t;
    let mut pem_len: size_t = 0;
    let mut result: CURLcode = CURLE_OK;
    if pem.is_null() {
        return CURLE_BAD_CONTENT_ENCODING;
    }
    begin_pos = strstr(
        pem,
        b"-----BEGIN PUBLIC KEY-----\0" as *const u8 as *const i8,
    );
    if begin_pos.is_null() {
        return CURLE_BAD_CONTENT_ENCODING;
    }
    pem_count = begin_pos.offset_from(pem) as i64 as size_t;
    if 0 as i32 as u64 != pem_count
        && '\n' as i32
            != *pem
                .offset(
                    pem_count.wrapping_sub(1 as i32 as u64) as isize,
                ) as i32
    {
        return CURLE_BAD_CONTENT_ENCODING;
    }
    pem_count = (pem_count as u64)
        .wrapping_add(26 as i32 as u64) as size_t as size_t;
    end_pos = strstr(
        pem.offset(pem_count as isize),
        b"\n-----END PUBLIC KEY-----\0" as *const u8 as *const i8,
    );
    if end_pos.is_null() {
        return CURLE_BAD_CONTENT_ENCODING;
    }
    pem_len = end_pos.offset_from(pem) as i64 as size_t;
    stripped_pem = Curl_cmalloc
        .expect(
            "non-null function pointer",
        )(
        pem_len.wrapping_sub(pem_count).wrapping_add(1 as i32 as u64),
    ) as *mut i8;
    if stripped_pem.is_null() {
        return CURLE_OUT_OF_MEMORY;
    }
    while pem_count < pem_len {
        if '\n' as i32 != *pem.offset(pem_count as isize) as i32
            && '\r' as i32 != *pem.offset(pem_count as isize) as i32
        {
            let fresh56 = stripped_pem_count;
            stripped_pem_count = stripped_pem_count.wrapping_add(1);
            *stripped_pem.offset(fresh56 as isize) = *pem.offset(pem_count as isize);
        }
        pem_count = pem_count.wrapping_add(1);
    }
    *stripped_pem.offset(stripped_pem_count as isize) = '\u{0}' as i32 as i8;
    result = Curl_base64_decode(stripped_pem, der, der_len);
    Curl_cfree.expect("non-null function pointer")(stripped_pem as *mut libc::c_void);
    stripped_pem = 0 as *mut i8;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn Curl_pin_peer_pubkey(
    mut data: *mut Curl_easy,
    mut pinnedpubkey: *const i8,
    mut pubkey: *const u8,
    mut pubkeylen: size_t,
) -> CURLcode {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut buf: *mut u8 = 0 as *mut u8;
    let mut pem_ptr: *mut u8 = 0 as *mut u8;
    let mut result: CURLcode = CURLE_SSL_PINNEDPUBKEYNOTMATCH;
    if pinnedpubkey.is_null() {
        return CURLE_OK;
    }
    if pubkey.is_null() || pubkeylen == 0 {
        return result;
    }
    if strncmp(
        pinnedpubkey,
        b"sha256//\0" as *const u8 as *const i8,
        8 as i32 as u64,
    ) == 0 as i32
    {
        let mut encode: CURLcode = CURLE_OK;
        let mut encodedlen: size_t = 0;
        let mut pinkeylen: size_t = 0;
        let mut encoded: *mut i8 = 0 as *mut i8;
        let mut pinkeycopy: *mut i8 = 0 as *mut i8;
        let mut begin_pos: *mut i8 = 0 as *mut i8;
        let mut end_pos: *mut i8 = 0 as *mut i8;
        let mut sha256sumdigest: *mut u8 = 0 as *mut u8;
        if ((*Curl_ssl).sha256sum).is_none() {
            return result;
        }
        sha256sumdigest = Curl_cmalloc
            .expect("non-null function pointer")(32 as i32 as size_t)
            as *mut u8;
        if sha256sumdigest.is_null() {
            return CURLE_OUT_OF_MEMORY;
        }
        encode = ((*Curl_ssl).sha256sum)
            .expect(
                "non-null function pointer",
            )(pubkey, pubkeylen, sha256sumdigest, 32 as i32 as size_t);
        if encode as u32 != CURLE_OK as i32 as u32 {
            return encode;
        }
        encode = Curl_base64_encode(
            data,
            sha256sumdigest as *mut i8,
            32 as i32 as size_t,
            &mut encoded,
            &mut encodedlen,
        );
        Curl_cfree
            .expect("non-null function pointer")(sha256sumdigest as *mut libc::c_void);
        sha256sumdigest = 0 as *mut u8;
        if encode as u64 != 0 {
            return encode;
        }
        Curl_infof(
            data,
            b" public key hash: sha256//%s\0" as *const u8 as *const i8,
            encoded,
        );
        pinkeylen = (strlen(pinnedpubkey))
            .wrapping_add(1 as i32 as u64);
        pinkeycopy = Curl_cmalloc.expect("non-null function pointer")(pinkeylen)
            as *mut i8;
        if pinkeycopy.is_null() {
            Curl_cfree.expect("non-null function pointer")(encoded as *mut libc::c_void);
            encoded = 0 as *mut i8;
            return CURLE_OUT_OF_MEMORY;
        }
        memcpy(
            pinkeycopy as *mut libc::c_void,
            pinnedpubkey as *const libc::c_void,
            pinkeylen,
        );
        begin_pos = pinkeycopy;
        loop {
            end_pos = strstr(
                begin_pos,
                b";sha256//\0" as *const u8 as *const i8,
            );
            if !end_pos.is_null() {
                *end_pos
                    .offset(0 as i32 as isize) = '\u{0}' as i32 as i8;
            }
            if encodedlen == strlen(begin_pos.offset(8 as i32 as isize))
                && memcmp(
                    encoded as *const libc::c_void,
                    begin_pos.offset(8 as i32 as isize) as *const libc::c_void,
                    encodedlen,
                ) == 0
            {
                result = CURLE_OK;
                break;
            } else {
                if !end_pos.is_null() {
                    *end_pos
                        .offset(0 as i32 as isize) = ';' as i32 as i8;
                    begin_pos = strstr(
                        end_pos,
                        b"sha256//\0" as *const u8 as *const i8,
                    );
                }
                if !(!end_pos.is_null() && !begin_pos.is_null()) {
                    break;
                }
            }
        }
        Curl_cfree.expect("non-null function pointer")(encoded as *mut libc::c_void);
        encoded = 0 as *mut i8;
        Curl_cfree.expect("non-null function pointer")(pinkeycopy as *mut libc::c_void);
        pinkeycopy = 0 as *mut i8;
        return result;
    }
    fp = fopen(pinnedpubkey, b"rb\0" as *const u8 as *const i8);
    if fp.is_null() {
        return result;
    }
    let mut filesize: i64 = 0;
    let mut size: size_t = 0;
    let mut pem_len: size_t = 0;
    let mut pem_read: CURLcode = CURLE_OK;
    if !(fseek(fp, 0 as i32 as i64, 2 as i32) != 0) {
        filesize = ftell(fp);
        if !(fseek(fp, 0 as i32 as i64, 0 as i32) != 0) {
            if !(filesize < 0 as i32 as i64
                || filesize > 1048576 as i32 as i64)
            {
                size = curlx_sotouz(filesize);
                if !(pubkeylen > size) {
                    buf = Curl_cmalloc
                        .expect(
                            "non-null function pointer",
                        )(size.wrapping_add(1 as i32 as u64))
                        as *mut u8;
                    if !buf.is_null() {
                        if !(fread(
                            buf as *mut libc::c_void,
                            size,
                            1 as i32 as u64,
                            fp,
                        ) as i32 != 1 as i32)
                        {
                            if pubkeylen == size {
                                if memcmp(
                                    pubkey as *const libc::c_void,
                                    buf as *const libc::c_void,
                                    pubkeylen,
                                ) == 0
                                {
                                    result = CURLE_OK;
                                }
                            } else {
                                *buf
                                    .offset(size as isize) = '\u{0}' as i32 as u8;
                                pem_read = pubkey_pem_to_der(
                                    buf as *const i8,
                                    &mut pem_ptr,
                                    &mut pem_len,
                                );
                                if !(pem_read as u64 != 0) {
                                    if pubkeylen == pem_len
                                        && memcmp(
                                            pubkey as *const libc::c_void,
                                            pem_ptr as *const libc::c_void,
                                            pubkeylen,
                                        ) == 0
                                    {
                                        result = CURLE_OK;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Curl_cfree.expect("non-null function pointer")(buf as *mut libc::c_void);
    buf = 0 as *mut u8;
    Curl_cfree.expect("non-null function pointer")(pem_ptr as *mut libc::c_void);
    pem_ptr = 0 as *mut u8;
    fclose(fp);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn Curl_ssl_cert_status_request() -> bool {
    return ((*Curl_ssl).cert_status_request).expect("non-null function pointer")();
}
#[no_mangle]
pub unsafe extern "C" fn Curl_ssl_false_start() -> bool {
    return ((*Curl_ssl).false_start).expect("non-null function pointer")();
}
#[no_mangle]
pub unsafe extern "C" fn Curl_ssl_tls13_ciphersuites() -> bool {
    return (*Curl_ssl).supports
        & ((1 as i32) << 5 as i32) as u32 != 0;
}
#[no_mangle]
pub extern "C" fn Curl_none_init() -> i32 {
    return 1 as i32;
}
#[no_mangle]
pub extern "C" fn Curl_none_cleanup() {}
#[no_mangle]
pub extern "C" fn Curl_none_shutdown(
    mut data: *mut Curl_easy,
    mut conn: *mut connectdata,
    mut sockindex: i32,
) -> i32 {
    return 0 as i32;
}
#[no_mangle]
pub extern "C" fn Curl_none_check_cxn(mut conn: *mut connectdata) -> i32 {
    return -(1 as i32);
}
#[no_mangle]
pub extern "C" fn Curl_none_random(
    mut data: *mut Curl_easy,
    mut entropy: *mut u8,
    mut length: size_t,
) -> CURLcode {
    return CURLE_NOT_BUILT_IN;
}
#[no_mangle]
pub extern "C" fn Curl_none_close_all(mut data: *mut Curl_easy) {}
#[no_mangle]
pub extern "C" fn Curl_none_session_free(mut ptr: *mut libc::c_void) {}
#[no_mangle]
pub extern "C" fn Curl_none_data_pending(
    mut conn: *const connectdata,
    mut connindex: i32,
) -> bool {
    return 0 as i32 != 0;
}
#[no_mangle]
pub extern "C" fn Curl_none_cert_status_request() -> bool {
    return 0 as i32 != 0;
}
#[no_mangle]
pub extern "C" fn Curl_none_set_engine(
    mut data: *mut Curl_easy,
    mut engine: *const i8,
) -> CURLcode {
    return CURLE_NOT_BUILT_IN;
}
#[no_mangle]
pub extern "C" fn Curl_none_set_engine_default(
    mut data: *mut Curl_easy,
) -> CURLcode {
    return CURLE_NOT_BUILT_IN;
}
#[no_mangle]
pub extern "C" fn Curl_none_engines_list(
    mut data: *mut Curl_easy,
) -> *mut curl_slist {
    return 0 as *mut libc::c_void as *mut curl_slist;
}
#[no_mangle]
pub extern "C" fn Curl_none_false_start() -> bool {
    return 0 as i32 != 0;
}
unsafe extern "C" fn multissl_init() -> i32 {
    if multissl_setup(0 as *const Curl_ssl) != 0 {
        return 1 as i32;
    }
    return ((*Curl_ssl).init).expect("non-null function pointer")();
}
unsafe extern "C" fn multissl_connect(
    mut data: *mut Curl_easy,
    mut conn: *mut connectdata,
    mut sockindex: i32,
) -> CURLcode {
    if multissl_setup(0 as *const Curl_ssl) != 0 {
        return CURLE_FAILED_INIT;
    }
    return ((*Curl_ssl).connect_blocking)
        .expect("non-null function pointer")(data, conn, sockindex);
}
unsafe extern "C" fn multissl_connect_nonblocking(
    mut data: *mut Curl_easy,
    mut conn: *mut connectdata,
    mut sockindex: i32,
    mut done: *mut bool,
) -> CURLcode {
    if multissl_setup(0 as *const Curl_ssl) != 0 {
        return CURLE_FAILED_INIT;
    }
    return ((*Curl_ssl).connect_nonblocking)
        .expect("non-null function pointer")(data, conn, sockindex, done);
}
unsafe extern "C" fn multissl_getsock(
    mut conn: *mut connectdata,
    mut socks: *mut curl_socket_t,
) -> i32 {
    if multissl_setup(0 as *const Curl_ssl) != 0 {
        return 0 as i32;
    }
    return ((*Curl_ssl).getsock).expect("non-null function pointer")(conn, socks);
}
unsafe extern "C" fn multissl_get_internals(
    mut connssl: *mut ssl_connect_data,
    mut info: CURLINFO,
) -> *mut libc::c_void {
    if multissl_setup(0 as *const Curl_ssl) != 0 {
        return 0 as *mut libc::c_void;
    }
    return ((*Curl_ssl).get_internals)
        .expect("non-null function pointer")(connssl, info);
}
unsafe extern "C" fn multissl_close(
    mut data: *mut Curl_easy,
    mut conn: *mut connectdata,
    mut sockindex: i32,
) {
    if multissl_setup(0 as *const Curl_ssl) != 0 {
        return;
    }
    ((*Curl_ssl).close_one).expect("non-null function pointer")(data, conn, sockindex);
}
static mut Curl_ssl_multi: Curl_ssl = unsafe {
    {
        let mut init = Curl_ssl {
            info: {
                let mut init = curl_ssl_backend {
                    id: CURLSSLBACKEND_NONE,
                    name: b"multi\0" as *const u8 as *const i8,
                };
                init
            },
            supports: 0 as i32 as u32,
            sizeof_ssl_backend_data: -(1 as i32) as size_t,
            init: Some(multissl_init as unsafe extern "C" fn() -> i32),
            cleanup: Some(Curl_none_cleanup as unsafe extern "C" fn() -> ()),
            version: Some(
                multissl_version
                    as unsafe extern "C" fn(*mut i8, size_t) -> size_t,
            ),
            check_cxn: Some(
                Curl_none_check_cxn
                    as unsafe extern "C" fn(*mut connectdata) -> i32,
            ),
            shut_down: Some(
                Curl_none_shutdown
                    as unsafe extern "C" fn(
                        *mut Curl_easy,
                        *mut connectdata,
                        i32,
                    ) -> i32,
            ),
            data_pending: Some(
                Curl_none_data_pending
                    as unsafe extern "C" fn(*const connectdata, i32) -> bool,
            ),
            random: Some(
                Curl_none_random
                    as unsafe extern "C" fn(
                        *mut Curl_easy,
                        *mut u8,
                        size_t,
                    ) -> CURLcode,
            ),
            cert_status_request: Some(
                Curl_none_cert_status_request as unsafe extern "C" fn() -> bool,
            ),
            connect_blocking: Some(
                multissl_connect
                    as unsafe extern "C" fn(
                        *mut Curl_easy,
                        *mut connectdata,
                        i32,
                    ) -> CURLcode,
            ),
            connect_nonblocking: Some(
                multissl_connect_nonblocking
                    as unsafe extern "C" fn(
                        *mut Curl_easy,
                        *mut connectdata,
                        i32,
                        *mut bool,
                    ) -> CURLcode,
            ),
            getsock: Some(
                multissl_getsock
                    as unsafe extern "C" fn(
                        *mut connectdata,
                        *mut curl_socket_t,
                    ) -> i32,
            ),
            get_internals: Some(
                multissl_get_internals
                    as unsafe extern "C" fn(
                        *mut ssl_connect_data,
                        CURLINFO,
                    ) -> *mut libc::c_void,
            ),
            close_one: Some(
                multissl_close
                    as unsafe extern "C" fn(
                        *mut Curl_easy,
                        *mut connectdata,
                        i32,
                    ) -> (),
            ),
            close_all: Some(
                Curl_none_close_all as unsafe extern "C" fn(*mut Curl_easy) -> (),
            ),
            session_free: Some(
                Curl_none_session_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
            ),
            set_engine: Some(
                Curl_none_set_engine
                    as unsafe extern "C" fn(
                        *mut Curl_easy,
                        *const i8,
                    ) -> CURLcode,
            ),
            set_engine_default: Some(
                Curl_none_set_engine_default
                    as unsafe extern "C" fn(*mut Curl_easy) -> CURLcode,
            ),
            engines_list: Some(
                Curl_none_engines_list
                    as unsafe extern "C" fn(*mut Curl_easy) -> *mut curl_slist,
            ),
            false_start: Some(Curl_none_false_start as unsafe extern "C" fn() -> bool),
            sha256sum: None,
            associate_connection: None,
            disassociate_connection: None,
        };
        init
    }
};
#[no_mangle]
pub static mut Curl_ssl: *const Curl_ssl = unsafe {
    &Curl_ssl_openssl as *const Curl_ssl
};
static mut available_backends: [*const Curl_ssl; 2] = unsafe {
    [&Curl_ssl_openssl as *const Curl_ssl, 0 as *const Curl_ssl]
};
unsafe extern "C" fn multissl_version(
    mut buffer: *mut i8,
    mut size: size_t,
) -> size_t {
    static mut selected: *const Curl_ssl = 0 as *const Curl_ssl;
    static mut backends: [i8; 200] = [0; 200];
    static mut backends_len: size_t = 0;
    let mut current: *const Curl_ssl = 0 as *const Curl_ssl;
    current = if Curl_ssl == &Curl_ssl_multi as *const Curl_ssl {
        available_backends[0 as i32 as usize]
    } else {
        Curl_ssl
    };
    if current != selected {
        let mut p: *mut i8 = backends.as_mut_ptr();
        let mut end: *mut i8 = backends
            .as_mut_ptr()
            .offset(
                ::std::mem::size_of::<[i8; 200]>() as u64 as isize,
            );
        let mut i: i32 = 0;
        selected = current;
        backends[0 as i32 as usize] = '\u{0}' as i32 as i8;
        i = 0 as i32;
        while !(available_backends[i as usize]).is_null() {
            let mut vb: [i8; 200] = [0; 200];
            let mut paren: bool = selected != available_backends[i as usize];
            if ((*available_backends[i as usize]).version)
                .expect(
                    "non-null function pointer",
                )(
                vb.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 200]>() as u64,
            ) != 0
            {
                p = p
                    .offset(
                        curl_msnprintf(
                            p,
                            end.offset_from(p) as i64 as size_t,
                            b"%s%s%s%s\0" as *const u8 as *const i8,
                            if p != backends.as_mut_ptr() {
                                b" \0" as *const u8 as *const i8
                            } else {
                                b"\0" as *const u8 as *const i8
                            },
                            if paren as i32 != 0 {
                                b"(\0" as *const u8 as *const i8
                            } else {
                                b"\0" as *const u8 as *const i8
                            },
                            vb.as_mut_ptr(),
                            if paren as i32 != 0 {
                                b")\0" as *const u8 as *const i8
                            } else {
                                b"\0" as *const u8 as *const i8
                            },
                        ) as isize,
                    );
            }
            i += 1;
        }
        backends_len = p.offset_from(backends.as_mut_ptr()) as i64 as size_t;
    }
    if size == 0 {
        return 0 as i32 as size_t;
    }
    if size <= backends_len {
        strncpy(
            buffer,
            backends.as_mut_ptr(),
            size.wrapping_sub(1 as i32 as u64),
        );
        *buffer
            .offset(
                size.wrapping_sub(1 as i32 as u64) as isize,
            ) = '\u{0}' as i32 as i8;
        return size.wrapping_sub(1 as i32 as u64);
    }
    strcpy(buffer, backends.as_mut_ptr());
    return backends_len;
}
unsafe extern "C" fn multissl_setup(mut backend: *const Curl_ssl) -> i32 {
    let mut env: *const i8 = 0 as *const i8;
    let mut env_tmp: *mut i8 = 0 as *mut i8;
    if Curl_ssl != &Curl_ssl_multi as *const Curl_ssl {
        return 1 as i32;
    }
    if !backend.is_null() {
        Curl_ssl = backend;
        return 0 as i32;
    }
    if (available_backends[0 as i32 as usize]).is_null() {
        return 1 as i32;
    }
    env_tmp = curl_getenv(b"CURL_SSL_BACKEND\0" as *const u8 as *const i8);
    env = env_tmp;
    if !env.is_null() {
        let mut i: i32 = 0;
        i = 0 as i32;
        while !(available_backends[i as usize]).is_null() {
            if Curl_strcasecompare(env, (*available_backends[i as usize]).info.name) != 0
            {
                Curl_ssl = available_backends[i as usize];
                Curl_cfree
                    .expect("non-null function pointer")(env_tmp as *mut libc::c_void);
                return 0 as i32;
            }
            i += 1;
        }
    }
    Curl_ssl = available_backends[0 as i32 as usize];
    Curl_cfree.expect("non-null function pointer")(env_tmp as *mut libc::c_void);
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn curl_global_sslset(
    mut id: curl_sslbackend,
    mut name: *const i8,
    mut avail: *mut *mut *const curl_ssl_backend,
) -> CURLsslset {
    let mut i: i32 = 0;
    if !avail.is_null() {
        *avail = &mut available_backends as *mut [*const Curl_ssl; 2]
            as *mut *const curl_ssl_backend;
    }
    if Curl_ssl != &Curl_ssl_multi as *const Curl_ssl {
        return (if id as u32 == (*Curl_ssl).info.id as u32
            || !name.is_null() && Curl_strcasecompare(name, (*Curl_ssl).info.name) != 0
        {
            CURLSSLSET_OK as i32
        } else {
            CURLSSLSET_UNKNOWN_BACKEND as i32
        }) as CURLsslset;
    }
    i = 0 as i32;
    while !(available_backends[i as usize]).is_null() {
        if (*available_backends[i as usize]).info.id as u32
            == id as u32
            || !name.is_null()
                && Curl_strcasecompare((*available_backends[i as usize]).info.name, name)
                    != 0
        {
            multissl_setup(available_backends[i as usize]);
            return CURLSSLSET_OK;
        }
        i += 1;
    }
    return CURLSSLSET_UNKNOWN_BACKEND;
}
