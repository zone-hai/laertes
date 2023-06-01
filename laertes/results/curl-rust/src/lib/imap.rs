use ::libc;
use ::c2rust_bitfields;
extern "C" {
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    fn sscanf(_: * const i8, _: * const i8, _: ...) -> i32;
    
    
    
    fn memmove(
        _: * mut core::ffi::c_void,
        _: * const core::ffi::c_void,
        _: u64,
    ) -> * mut core::ffi::c_void;
    fn memcmp(
        _: * const core::ffi::c_void,
        _: * const core::ffi::c_void,
        _: u64,
    ) -> i32;
    fn strcpy(_: * mut i8, _: * const i8) -> * mut i8;
    fn strlen(_: * const i8) -> u64;
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
}
pub use crate::src::lib::connect::Curl_conncontrol;
pub use crate::src::lib::curl_ctype::Curl_isdigit;
pub use crate::src::lib::curl_sasl::Curl_sasl_can_authenticate;
pub use crate::src::lib::curl_sasl::Curl_sasl_cleanup;
pub use crate::src::lib::curl_sasl::Curl_sasl_continue;
pub use crate::src::lib::curl_sasl::Curl_sasl_decode_mech;
pub use crate::src::lib::curl_sasl::Curl_sasl_init;
pub use crate::src::lib::curl_sasl::Curl_sasl_parse_url_auth_option;
pub use crate::src::lib::curl_sasl::Curl_sasl_start;
pub use crate::src::lib::dynbuf::Curl_dyn_addf;
pub use crate::src::lib::dynbuf::Curl_dyn_free;
pub use crate::src::lib::dynbuf::Curl_dyn_init;
pub use crate::src::lib::dynbuf::Curl_dyn_ptr;
pub use crate::src::lib::dynbuf::Curl_dyn_reset;
pub use crate::src::lib::escape::Curl_urldecode;
pub use crate::src::lib::mime::Curl_mime_add_header;
pub use crate::src::lib::mime::Curl_mime_prepare_headers;
pub use crate::src::lib::mime::Curl_mime_read;
pub use crate::src::lib::mime::Curl_mime_rewind;
pub use crate::src::lib::mime::Curl_mime_size;
pub use crate::src::lib::mime::curl_mime_headers;
pub use crate::src::lib::mprintf::curl_msnprintf;
pub use crate::src::lib::pingpong::Curl_pp_disconnect;
pub use crate::src::lib::pingpong::Curl_pp_flushsend;
pub use crate::src::lib::pingpong::Curl_pp_getsock;
pub use crate::src::lib::pingpong::Curl_pp_init;
pub use crate::src::lib::pingpong::Curl_pp_moredata;
pub use crate::src::lib::pingpong::Curl_pp_readresp;
pub use crate::src::lib::pingpong::Curl_pp_sendf;
pub use crate::src::lib::pingpong::Curl_pp_setup;
pub use crate::src::lib::pingpong::Curl_pp_statemach;
pub use crate::src::lib::pingpong::Curl_pp_vsendf;
pub use crate::src::lib::progress::Curl_pgrsSetDownloadCounter;
pub use crate::src::lib::progress::Curl_pgrsSetDownloadSize;
pub use crate::src::lib::progress::Curl_pgrsSetUploadCounter;
pub use crate::src::lib::progress::Curl_pgrsSetUploadSize;
pub use crate::src::lib::sendf::Curl_client_write;
pub use crate::src::lib::sendf::Curl_failf;
pub use crate::src::lib::sendf::Curl_infof;
pub use crate::src::lib::strcase::Curl_strcasecompare;
pub use crate::src::lib::strcase::Curl_strncasecompare;
pub use crate::src::lib::strtoofft::curlx_strtoofft;
pub use crate::src::lib::transfer::Curl_checkheaders;
pub use crate::src::lib::transfer::Curl_setup_transfer;
pub use crate::src::lib::urlapi::curl_url_get;
pub use crate::src::lib::vtls::vtls::Curl_ssl_connect_nonblocking;
pub use crate::src::lib::warnless::curlx_sltosi;
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
pub use crate::src::lib::easy::Curl_cmalloc;
pub use crate::src::lib::easy::Curl_cstrdup;
pub use crate::src::lib::escape::Gsasl;
pub use crate::src::lib::mqtt::_IO_codecvt;
pub use crate::src::lib::speedcheck::nghttp2_session;
pub use crate::src::lib::urlapi::psl_ctx_st;
pub use crate::src::src::tool_cb_rea::_IO_wide_data;
pub use crate::src::src::tool_msgs::_IO_marker;
pub use crate::src::lib::openldap::ldapconninfo;
pub use crate::src::lib::openldap::ldapreqinfo;
pub use crate::src::lib::smb::smb_request;
pub use crate::src::lib::telnet::TELNET;
pub use crate::src::lib::tftp::tftp_state_data;
pub use crate::src::lib::urlapi::Curl_URL;
pub use crate::src::lib::vtls::openssl::ssl_backend_data;
pub type __builtin_va_list = [crate::src::lib::dict::__va_list_tag; 1];
// #[derive(Copy, Clone)]

pub type __va_list_tag = crate::src::lib::dict::__va_list_tag;
pub type __uint8_t = u8;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __pid_t = i32;
pub type __time_t = i64;
pub type __ssize_t = i64;
pub type __socklen_t = u32;
pub type pid_t = i32;
pub type ssize_t = i64;
pub type time_t = i64;
pub type size_t = u64;
pub type int32_t = i32;
pub type socklen_t = u32;
pub type sa_family_t = u16;
// #[derive(Copy, Clone)]

pub type sockaddr = crate::src::lib::http2::sockaddr;
pub type curl_socklen_t = u32;
pub type curl_off_t = i64;
pub type va_list = [crate::src::lib::dict::__va_list_tag; 1];
// #[derive(Copy, Clone)]

pub type _IO_FILE = crate::src::lib::http2::_IO_FILE;
pub type _IO_lock_t = ();
pub type FILE = crate::src::lib::http2::_IO_FILE;
// #[derive(Copy, Clone)]

pub type Curl_easy = crate::src::lib::http2::Curl_easy;
// #[derive(Copy, Clone)]

pub type curl_tlssessioninfo = crate::src::lib::http2::curl_tlssessioninfo;
pub type curl_sslbackend = u32;
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
pub type bit = u32;
pub type CURLproxycode = u32;
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
pub type wildcard_dtor<'a1> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,) -> ()>;
// #[derive(Copy, Clone)]

pub type Curl_llist = crate::src::lib::http2::Curl_llist;
pub type Curl_llist_dtor<'a1, 'a2> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,_: Option<&'a2 mut core::ffi::c_void>,) -> ()>;
// #[derive(Copy, Clone)]

pub type Curl_llist_element = crate::src::lib::http2::Curl_llist_element;
pub type wildcard_states = u32;
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
pub type trailers_state = u32;
pub const TRAILERS_DONE: trailers_state = 3;
pub const TRAILERS_SENDING: trailers_state = 2;
pub const TRAILERS_INITIALIZED: trailers_state = 1;
pub const TRAILERS_NONE: trailers_state = 0;
// #[derive(Copy, Clone)]

pub type dynbuf = crate::src::lib::http2::dynbuf;
pub type Curl_HttpReq = u32;
pub const HTTPREQ_HEAD: Curl_HttpReq = 5;
pub const HTTPREQ_PUT: Curl_HttpReq = 4;
pub const HTTPREQ_POST_MIME: Curl_HttpReq = 3;
pub const HTTPREQ_POST_FORM: Curl_HttpReq = 2;
pub const HTTPREQ_POST: Curl_HttpReq = 1;
pub const HTTPREQ_GET: Curl_HttpReq = 0;
// #[derive(Copy, Clone)]

pub type urlpieces = crate::src::lib::http2::urlpieces;
pub type CURLU = crate::src::lib::urlapi::Curl_URL;
pub type curl_read_callback<'a1, 'a2> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut i8>,_: u64,_: u64,_: Option<&'a2 mut core::ffi::c_void>,) -> u64>;
// #[derive(Copy, Clone)]

pub type time_node = crate::src::lib::http2::time_node;
pub type expire_id = u32;
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
pub type Curl_hash_dtor<'a1> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,) -> ()>;
pub type comp_function<'a1, 'a2> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,_: u64,_: Option<&'a2 mut core::ffi::c_void>,_: u64,) -> u64>;
pub type hash_function<'a1> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,_: u64,_: u64,) -> u64>;
// #[derive(Copy, Clone, BitfieldStruct)]

pub type Progress = crate::src::lib::http2::Progress;
pub type timediff_t = i64;
// #[derive(Copy, Clone)]

pub type CookieInfo = crate::src::lib::http2::CookieInfo;
// #[derive(Copy, Clone)]

pub type Cookie = crate::src::lib::http2::Cookie;
// #[derive(Copy, Clone, BitfieldStruct)]

pub type UserDefined = crate::src::lib::http2::UserDefined;
pub type curl_trailer_callback<'a1, 'a2, 'a3> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut Option<&'a2 mut crate::src::lib::http2::curl_slist>>,_: Option<&'a3 mut core::ffi::c_void>,) -> i32>;
pub type multidone_func<'a1> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::lib::http2::Curl_easy>,_: u32,) -> i32>;
pub type CURLcode = u32;
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
pub type curl_resolver_start_callback<'a1, 'a2, 'a3> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,_: Option<&'a2 mut core::ffi::c_void>,_: Option<&'a3 mut core::ffi::c_void>,) -> i32>;
// #[derive(Copy, Clone)]

pub type Curl_http2_dep = crate::src::lib::http2::Curl_http2_dep;
pub type curl_fnmatch_callback<'a1, 'a2, 'a3> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,_: Option<&'a2 i8>,_: Option<&'a3 i8>,) -> i32>;
pub type curl_chunk_end_callback<'a1> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,) -> i64>;
pub type curl_chunk_bgn_callback<'a1, 'a2> = Option<unsafe extern "C"  fn(_: Option<&'a1 core::ffi::c_void>,_: Option<&'a2 mut core::ffi::c_void>,_: i32,) -> i64>;
pub type Curl_RtspReq = u32;
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
pub type curl_usessl = u32;
pub const CURLUSESSL_LAST: curl_usessl = 4;
pub const CURLUSESSL_ALL: curl_usessl = 3;
pub const CURLUSESSL_CONTROL: curl_usessl = 2;
pub const CURLUSESSL_TRY: curl_usessl = 1;
pub const CURLUSESSL_NONE: curl_usessl = 0;
pub type CURL_NETRC_OPTION = u32;
pub const CURL_NETRC_LAST: CURL_NETRC_OPTION = 3;
pub const CURL_NETRC_REQUIRED: CURL_NETRC_OPTION = 2;
pub const CURL_NETRC_OPTIONAL: CURL_NETRC_OPTION = 1;
pub const CURL_NETRC_IGNORED: CURL_NETRC_OPTION = 0;
pub type curl_sshkeycallback<'a1, 'a2, 'a3, 'a4> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::lib::http2::Curl_easy>,_: Option<&'a2 crate::src::lib::http2::curl_khkey>,_: Option<&'a3 crate::src::lib::http2::curl_khkey>,_: u32,_: Option<&'a4 mut core::ffi::c_void>,) -> i32>;
pub type curl_khmatch = u32;
pub const CURLKHMATCH_LAST: curl_khmatch = 3;
pub const CURLKHMATCH_MISSING: curl_khmatch = 2;
pub const CURLKHMATCH_MISMATCH: curl_khmatch = 1;
pub const CURLKHMATCH_OK: curl_khmatch = 0;
// #[derive(Copy, Clone)]

pub type curl_khkey = crate::src::lib::http2::curl_khkey;
pub type curl_khtype = u32;
pub const CURLKHTYPE_ED25519: curl_khtype = 5;
pub const CURLKHTYPE_ECDSA: curl_khtype = 4;
pub const CURLKHTYPE_DSS: curl_khtype = 3;
pub const CURLKHTYPE_RSA: curl_khtype = 2;
pub const CURLKHTYPE_RSA1: curl_khtype = 1;
pub const CURLKHTYPE_UNKNOWN: curl_khtype = 0;
pub type CURL = crate::src::lib::http2::Curl_easy;
pub type curl_ftpccc = u32;
pub const CURLFTPSSL_CCC_LAST: curl_ftpccc = 3;
pub const CURLFTPSSL_CCC_ACTIVE: curl_ftpccc = 2;
pub const CURLFTPSSL_CCC_PASSIVE: curl_ftpccc = 1;
pub const CURLFTPSSL_CCC_NONE: curl_ftpccc = 0;
pub type curl_ftpauth = u32;
pub const CURLFTPAUTH_LAST: curl_ftpauth = 3;
pub const CURLFTPAUTH_TLS: curl_ftpauth = 2;
pub const CURLFTPAUTH_SSL: curl_ftpauth = 1;
pub const CURLFTPAUTH_DEFAULT: curl_ftpauth = 0;
pub type curl_ftpfile = u32;
pub const FTPFILE_SINGLECWD: curl_ftpfile = 3;
pub const FTPFILE_NOCWD: curl_ftpfile = 2;
pub const FTPFILE_MULTICWD: curl_ftpfile = 1;
// #[derive(Copy, Clone)]

pub type ssl_general_config = crate::src::lib::http2::ssl_general_config;
// #[derive(Copy, Clone, BitfieldStruct)]

pub type ssl_config_data = crate::src::lib::http2::ssl_config_data;
pub type CURL_TLSAUTH = u32;
pub const CURL_TLSAUTH_LAST: CURL_TLSAUTH = 2;
pub const CURL_TLSAUTH_SRP: CURL_TLSAUTH = 1;
pub const CURL_TLSAUTH_NONE: CURL_TLSAUTH = 0;
pub type curl_ssl_ctx_callback<'a1, 'a2, 'a3> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::lib::http2::Curl_easy>,_: Option<&'a2 mut core::ffi::c_void>,_: Option<&'a3 mut core::ffi::c_void>,) -> u32>;
pub type curl_proxytype = u32;
pub const CURLPROXY_SOCKS5_HOSTNAME: curl_proxytype = 7;
pub const CURLPROXY_SOCKS4A: curl_proxytype = 6;
pub const CURLPROXY_SOCKS5: curl_proxytype = 5;
pub const CURLPROXY_SOCKS4: curl_proxytype = 4;
pub const CURLPROXY_HTTPS: curl_proxytype = 2;
pub const CURLPROXY_HTTP_1_0: curl_proxytype = 1;
pub const CURLPROXY_HTTP: curl_proxytype = 0;
pub type curl_TimeCond = u32;
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
pub type mimestate = u32;
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
pub type curl_free_callback<'a1> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,) -> ()>;
pub type curl_seek_callback<'a1> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,_: i64,_: i32,) -> i32>;
pub type mimekind = u32;
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
pub type curl_hstswrite_callback<'a1, 'a2, 'a3, 'a4> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::lib::http2::Curl_easy>,_: Option<&'a2 mut crate::src::lib::http2::curl_hstsentry>,_: Option<&'a3 mut crate::src::lib::http2::curl_index>,_: Option<&'a4 mut core::ffi::c_void>,) -> u32>;
// #[derive(Copy, Clone)]

pub type curl_index = crate::src::lib::http2::curl_index;
// #[derive(Copy, Clone, BitfieldStruct)]

pub type curl_hstsentry = crate::src::lib::http2::curl_hstsentry;
pub type CURLSTScode = u32;
pub const CURLSTS_FAIL: CURLSTScode = 2;
pub const CURLSTS_DONE: CURLSTScode = 1;
pub const CURLSTS_OK: CURLSTScode = 0;
pub type curl_hstsread_callback<'a1, 'a2, 'a3> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::lib::http2::Curl_easy>,_: Option<&'a2 mut crate::src::lib::http2::curl_hstsentry>,_: Option<&'a3 mut core::ffi::c_void>,) -> u32>;
pub type curl_conv_callback<'a1> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut i8>,_: u64,) -> u32>;
pub type curl_closesocket_callback<'a1> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,_: i32,) -> i32>;
pub type curl_socket_t = i32;
pub type curl_opensocket_callback<'a1, 'a2> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,_: u32,_: Option<&'a2 mut crate::src::lib::http2::curl_sockaddr>,) -> i32>;
// #[derive(Copy, Clone)]

pub type curl_sockaddr = crate::src::lib::http2::curl_sockaddr;
pub type curlsocktype = u32;
pub const CURLSOCKTYPE_LAST: curlsocktype = 2;
pub const CURLSOCKTYPE_ACCEPT: curlsocktype = 1;
pub const CURLSOCKTYPE_IPCXN: curlsocktype = 0;
pub type curl_sockopt_callback<'a1> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,_: i32,_: u32,) -> i32>;
pub type curl_ioctl_callback<'a1, 'a2> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::lib::http2::Curl_easy>,_: i32,_: Option<&'a2 mut core::ffi::c_void>,) -> u32>;
pub type curlioerr = u32;
pub const CURLIOE_LAST: curlioerr = 3;
pub const CURLIOE_FAILRESTART: curlioerr = 2;
pub const CURLIOE_UNKNOWNCMD: curlioerr = 1;
pub const CURLIOE_OK: curlioerr = 0;
pub type curl_debug_callback<'a1, 'a2, 'a3> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::lib::http2::Curl_easy>,_: u32,_: Option<&'a2 mut i8>,_: u64,_: Option<&'a3 mut core::ffi::c_void>,) -> i32>;
pub type curl_infotype = u32;
pub const CURLINFO_END: curl_infotype = 7;
pub const CURLINFO_SSL_DATA_OUT: curl_infotype = 6;
pub const CURLINFO_SSL_DATA_IN: curl_infotype = 5;
pub const CURLINFO_DATA_OUT: curl_infotype = 4;
pub const CURLINFO_DATA_IN: curl_infotype = 3;
pub const CURLINFO_HEADER_OUT: curl_infotype = 2;
pub const CURLINFO_HEADER_IN: curl_infotype = 1;
pub const CURLINFO_TEXT: curl_infotype = 0;
pub type curl_xferinfo_callback<'a1> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,_: i64,_: i64,_: i64,_: i64,) -> i32>;
pub type curl_progress_callback<'a1> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,_: f64,_: f64,_: f64,_: f64,) -> i32>;
pub type curl_write_callback<'a1, 'a2> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut i8>,_: u64,_: u64,_: Option<&'a2 mut core::ffi::c_void>,) -> u64>;
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
pub type curl_pp_transfer = u32;
pub const PPTRANSFER_NONE: curl_pp_transfer = 2;
pub const PPTRANSFER_INFO: curl_pp_transfer = 1;
pub const PPTRANSFER_BODY: curl_pp_transfer = 0;
// #[derive(Copy, Clone)]

pub type RTSP = crate::src::lib::http2::RTSP;
// #[derive(Copy, Clone)]

pub type HTTP = crate::src::lib::http2::HTTP;
pub type uint8_t = u8;
pub type uint32_t = u32;
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
// #[derive(Copy, Clone)]

pub type IMAP = crate::src::lib::http2::IMAP;
// #[derive(Copy, Clone)]

pub type FTP = crate::src::lib::http2::FTP;
// #[derive(Copy, Clone)]

pub type FILEPROTO = crate::src::lib::http2::FILEPROTO;
pub type upgrade101 = u32;
pub const UPGR101_WORKING: upgrade101 = 3;
pub const UPGR101_RECEIVED: upgrade101 = 2;
pub const UPGR101_REQUESTED: upgrade101 = 1;
pub const UPGR101_INIT: upgrade101 = 0;
pub type expect100 = u32;
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
pub type psl_ctx_t = crate::src::lib::urlapi::psl_ctx_st;
// #[derive(Copy, Clone)]

pub type Curl_multi = crate::src::lib::http2::Curl_multi;
pub type curl_multi_timer_callback<'a1, 'a2> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::lib::http2::Curl_multi>,_: i64,_: Option<&'a2 mut core::ffi::c_void>,) -> i32>;
pub type CURLM = crate::src::lib::http2::Curl_multi;
pub type curl_push_callback<'a1, 'a2, 'a3, 'a4> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::lib::http2::Curl_easy>,_: Option<&'a2 mut crate::src::lib::http2::Curl_easy>,_: u64,_: Option<&'a3 mut crate::src::lib::http2::curl_pushheaders>,_: Option<&'a4 mut core::ffi::c_void>,) -> i32>;
pub type curl_socket_callback<'a1, 'a2, 'a3> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::lib::http2::Curl_easy>,_: i32,_: i32,_: Option<&'a2 mut core::ffi::c_void>,_: Option<&'a3 mut core::ffi::c_void>,) -> i32>;
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
pub type CURLMSG = u32;
pub const CURLMSG_LAST: CURLMSG = 2;
pub const CURLMSG_DONE: CURLMSG = 1;
pub const CURLMSG_NONE: CURLMSG = 0;
pub type CURLMstate = u32;
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
pub type mqttstate = u32;
pub const MQTT_NOSTATE: mqttstate = 7;
pub const MQTT_PUB_REMAIN: mqttstate = 6;
pub const MQTT_PUBWAIT: mqttstate = 5;
pub const MQTT_SUBACK_COMING: mqttstate = 4;
pub const MQTT_SUBACK: mqttstate = 3;
pub const MQTT_CONNACK: mqttstate = 2;
pub const MQTT_REMAINING_LENGTH: mqttstate = 1;
pub const MQTT_FIRST: mqttstate = 0;
// #[derive(Copy, Clone)]

pub type smb_conn = crate::src::lib::http2::smb_conn;
pub type smb_conn_state = u32;
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
pub type saslstate = u32;
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
pub type smtpstate = u32;
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
pub type pop3state = u32;
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
pub type imapstate = u32;
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
pub type sshstate = i32;
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
pub type Curl_recv<'a1, 'a2, 'a3> = unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::lib::http2::Curl_easy>,_: i32,_: Option<&'a2 mut i8>,_: u64,_: Option<&'a3 mut u32>,) -> i64;
pub type Curl_send<'a1, 'a2, 'a3> = unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::lib::http2::Curl_easy>,_: i32,_: Option<&'a2 core::ffi::c_void>,_: u64,_: Option<&'a3 mut u32>,) -> i64;
// #[derive(Copy, Clone)]

pub type ftp_conn = crate::src::lib::http2::ftp_conn;
pub type ftpstate = u32;
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
pub type curlntlm = u32;
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
pub type ssl_connect_state = u32;
pub const ssl_connect_done: ssl_connect_state = 5;
pub const ssl_connect_3: ssl_connect_state = 4;
pub const ssl_connect_2_writing: ssl_connect_state = 3;
pub const ssl_connect_2_reading: ssl_connect_state = 2;
pub const ssl_connect_2: ssl_connect_state = 1;
pub const ssl_connect_1: ssl_connect_state = 0;
pub type ssl_connection_state = u32;
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
pub type ChunkyState = u32;
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
pub type connect_t = u32;
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
pub type curl_malloc_callback<'a1> = Option<unsafe extern "C"  fn(_: u64,) -> Option<&'a1 mut core::ffi::c_void>>;
pub type curl_strdup_callback<'a1, 'a2> = Option<unsafe extern "C"  fn(_: Option<&'a1 i8>,) -> Option<&'a2 mut i8>>;
pub type curl_calloc_callback<'a1> = Option<unsafe extern "C"  fn(_: u64,_: u64,) -> Option<&'a1 mut core::ffi::c_void>>;
pub type CURLUcode = u32;
pub const CURLUE_NO_FRAGMENT: CURLUcode = 17;
pub const CURLUE_NO_QUERY: CURLUcode = 16;
pub const CURLUE_NO_PORT: CURLUcode = 15;
pub const CURLUE_NO_HOST: CURLUcode = 14;
pub const CURLUE_NO_OPTIONS: CURLUcode = 13;
pub const CURLUE_NO_PASSWORD: CURLUcode = 12;
pub const CURLUE_NO_USER: CURLUcode = 11;
pub const CURLUE_NO_SCHEME: CURLUcode = 10;
pub const CURLUE_UNKNOWN_PART: CURLUcode = 9;
pub const CURLUE_USER_NOT_ALLOWED: CURLUcode = 8;
pub const CURLUE_OUT_OF_MEMORY: CURLUcode = 7;
pub const CURLUE_URLDECODE: CURLUcode = 6;
pub const CURLUE_UNSUPPORTED_SCHEME: CURLUcode = 5;
pub const CURLUE_BAD_PORT_NUMBER: CURLUcode = 4;
pub const CURLUE_MALFORMED_INPUT: CURLUcode = 3;
pub const CURLUE_BAD_PARTPOINTER: CURLUcode = 2;
pub const CURLUE_BAD_HANDLE: CURLUcode = 1;
pub const CURLUE_OK: CURLUcode = 0;
pub type CURLUPart = u32;
pub const CURLUPART_ZONEID: CURLUPart = 10;
pub const CURLUPART_FRAGMENT: CURLUPart = 9;
pub const CURLUPART_QUERY: CURLUPart = 8;
pub const CURLUPART_PATH: CURLUPart = 7;
pub const CURLUPART_PORT: CURLUPart = 6;
pub const CURLUPART_HOST: CURLUPart = 5;
pub const CURLUPART_OPTIONS: CURLUPart = 4;
pub const CURLUPART_PASSWORD: CURLUPart = 3;
pub const CURLUPART_USER: CURLUPart = 2;
pub const CURLUPART_SCHEME: CURLUPart = 1;
pub const CURLUPART_URL: CURLUPart = 0;
pub type mimestrategy = u32;
pub const MIMESTRATEGY_LAST: mimestrategy = 2;
pub const MIMESTRATEGY_FORM: mimestrategy = 1;
pub const MIMESTRATEGY_MAIL: mimestrategy = 0;
pub type saslprogress = u32;
pub const SASL_DONE: saslprogress = 2;
pub const SASL_INPROGRESS: saslprogress = 1;
pub const SASL_IDLE: saslprogress = 0;
pub type CURLofft = u32;
pub const CURL_OFFT_INVAL: CURLofft = 2;
pub const CURL_OFFT_FLOW: CURLofft = 1;
pub const CURL_OFFT_OK: CURLofft = 0;
pub type urlreject = u32;
pub const REJECT_ZERO: urlreject = 4;
pub const REJECT_CTRL: urlreject = 3;
pub const REJECT_NADA: urlreject = 2;
pub const STRING_CUSTOMREQUEST: dupstring = 6;
pub type dupstring = u32;
pub const STRING_LAST: dupstring = 80;
pub const STRING_AWS_SIGV4: dupstring = 79;
pub const STRING_COPYPOSTFIELDS: dupstring = 78;
pub const STRING_LASTZEROTERMINATED: dupstring = 77;
pub const STRING_SSL_EC_CURVES: dupstring = 76;
pub const STRING_DNS_LOCAL_IP6: dupstring = 75;
pub const STRING_DNS_LOCAL_IP4: dupstring = 74;
pub const STRING_DNS_INTERFACE: dupstring = 73;
pub const STRING_DNS_SERVERS: dupstring = 72;
pub const STRING_SASL_AUTHZID: dupstring = 71;
pub const STRING_HSTS: dupstring = 70;
pub const STRING_ALTSVC: dupstring = 69;
pub const STRING_DOH: dupstring = 68;
pub const STRING_TARGET: dupstring = 67;
pub const STRING_UNIX_SOCKET_PATH: dupstring = 66;
pub const STRING_BEARER: dupstring = 65;
pub const STRING_TLSAUTH_PASSWORD_PROXY: dupstring = 64;
pub const STRING_TLSAUTH_PASSWORD: dupstring = 63;
pub const STRING_TLSAUTH_USERNAME_PROXY: dupstring = 62;
pub const STRING_TLSAUTH_USERNAME: dupstring = 61;
pub const STRING_MAIL_AUTH: dupstring = 60;
pub const STRING_MAIL_FROM: dupstring = 59;
pub const STRING_SERVICE_NAME: dupstring = 58;
pub const STRING_PROXY_SERVICE_NAME: dupstring = 57;
pub const STRING_SSH_KNOWNHOSTS: dupstring = 56;
pub const STRING_SSH_HOST_PUBLIC_KEY_MD5: dupstring = 55;
pub const STRING_SSH_PUBLIC_KEY: dupstring = 54;
pub const STRING_SSH_PRIVATE_KEY: dupstring = 53;
pub const STRING_RTSP_TRANSPORT: dupstring = 52;
pub const STRING_RTSP_STREAM_URI: dupstring = 51;
pub const STRING_RTSP_SESSION_ID: dupstring = 50;
pub const STRING_NOPROXY: dupstring = 49;
pub const STRING_PROXYPASSWORD: dupstring = 48;
pub const STRING_PROXYUSERNAME: dupstring = 47;
pub const STRING_OPTIONS: dupstring = 46;
pub const STRING_PASSWORD: dupstring = 45;
pub const STRING_USERNAME: dupstring = 44;
pub const STRING_SSL_ENGINE: dupstring = 43;
pub const STRING_SSL_ISSUERCERT_PROXY: dupstring = 42;
pub const STRING_SSL_ISSUERCERT: dupstring = 41;
pub const STRING_SSL_CRLFILE_PROXY: dupstring = 40;
pub const STRING_SSL_CRLFILE: dupstring = 39;
pub const STRING_USERAGENT: dupstring = 38;
pub const STRING_SSL_RANDOM_FILE: dupstring = 37;
pub const STRING_SSL_EGDSOCKET: dupstring = 36;
pub const STRING_SSL_CIPHER13_LIST_PROXY: dupstring = 35;
pub const STRING_SSL_CIPHER13_LIST: dupstring = 34;
pub const STRING_SSL_CIPHER_LIST_PROXY: dupstring = 33;
pub const STRING_SSL_CIPHER_LIST: dupstring = 32;
pub const STRING_SSL_PINNEDPUBLICKEY_PROXY: dupstring = 31;
pub const STRING_SSL_PINNEDPUBLICKEY: dupstring = 30;
pub const STRING_SSL_CAFILE_PROXY: dupstring = 29;
pub const STRING_SSL_CAFILE: dupstring = 28;
pub const STRING_SSL_CAPATH_PROXY: dupstring = 27;
pub const STRING_SSL_CAPATH: dupstring = 26;
pub const STRING_SET_URL: dupstring = 25;
pub const STRING_SET_REFERER: dupstring = 24;
pub const STRING_SET_RANGE: dupstring = 23;
pub const STRING_PRE_PROXY: dupstring = 22;
pub const STRING_PROXY: dupstring = 21;
pub const STRING_NETRC_FILE: dupstring = 20;
pub const STRING_KRB_LEVEL: dupstring = 19;
pub const STRING_KEY_TYPE_PROXY: dupstring = 18;
pub const STRING_KEY_TYPE: dupstring = 17;
pub const STRING_KEY_PASSWD_PROXY: dupstring = 16;
pub const STRING_KEY_PASSWD: dupstring = 15;
pub const STRING_KEY_PROXY: dupstring = 14;
pub const STRING_KEY: dupstring = 13;
pub const STRING_FTPPORT: dupstring = 12;
pub const STRING_FTP_ALTERNATIVE_TO_USER: dupstring = 11;
pub const STRING_FTP_ACCOUNT: dupstring = 10;
pub const STRING_ENCODING: dupstring = 9;
pub const STRING_DEVICE: dupstring = 8;
pub const STRING_DEFAULT_PROTOCOL: dupstring = 7;
pub const STRING_COOKIEJAR: dupstring = 5;
pub const STRING_COOKIE: dupstring = 4;
pub const STRING_CERT_TYPE_PROXY: dupstring = 3;
pub const STRING_CERT_TYPE: dupstring = 2;
pub const STRING_CERT_PROXY: dupstring = 1;
pub const STRING_CERT: dupstring = 0;
#[no_mangle]
pub static mut Curl_handler_imap: crate::src::lib::http2::Curl_handler = unsafe {
    {
        let mut init = Curl_handler {
            scheme: b"IMAP\0" as *const u8 as *const i8,
            setup_connection: Some(
                imap_setup_connection,
            ),
            do_it: Some(
                imap_do,
            ),
            done: Some(
                imap_done,
            ),
            do_more: None,
            connect_it: Some(
                imap_connect,
            ),
            connecting: Some(
                imap_multi_statemach,
            ),
            doing: Some(
                imap_doing,
            ),
            proto_getsock: Some(
                imap_getsock,
            ),
            doing_getsock: Some(
                imap_getsock,
            ),
            domore_getsock: None,
            perform_getsock: None,
            disconnect: Some(
                imap_disconnect,
            ),
            readwrite: None,
            connection_check: None,
            attach: None,
            defport: 143 as i32,
            protocol: ((1 as i32) << 12 as i32) as u32,
            family: ((1 as i32) << 12 as i32) as u32,
            flags: ((1 as i32) << 2 as i32
                | (1 as i32) << 10 as i32) as u32,
        };
        init
    }
};
#[no_mangle]
pub static mut Curl_handler_imaps: crate::src::lib::http2::Curl_handler = unsafe {
    {
        let mut init = Curl_handler {
            scheme: b"IMAPS\0" as *const u8 as *const i8,
            setup_connection: Some(
                imap_setup_connection,
            ),
            do_it: Some(
                imap_do,
            ),
            done: Some(
                imap_done,
            ),
            do_more: None,
            connect_it: Some(
                imap_connect,
            ),
            connecting: Some(
                imap_multi_statemach,
            ),
            doing: Some(
                imap_doing,
            ),
            proto_getsock: Some(
                imap_getsock,
            ),
            doing_getsock: Some(
                imap_getsock,
            ),
            domore_getsock: None,
            perform_getsock: None,
            disconnect: Some(
                imap_disconnect,
            ),
            readwrite: None,
            connection_check: None,
            attach: None,
            defport: 993 as i32,
            protocol: ((1 as i32) << 13 as i32) as u32,
            family: ((1 as i32) << 12 as i32) as u32,
            flags: ((1 as i32) << 2 as i32
                | (1 as i32) << 0 as i32
                | (1 as i32) << 10 as i32) as u32,
        };
        init
    }
};
static mut saslimap: crate::src::lib::http2::SASLproto = unsafe {
    {
        let mut init = SASLproto {
            service: b"imap\0" as *const u8 as *const i8,
            contcode: '+' as i32,
            finalcode: 1 as i32,
            maxirlen: 0 as i32 as size_t,
            sendauth: Some(
                imap_perform_authenticate,
            ),
            sendcont: Some(
                imap_continue_authenticate,
            ),
            getmessage: Some(
                imap_get_message,
            ),
        };
        init
    }
};
unsafe extern "C" fn imap_to_imaps(mut conn: * mut crate::src::lib::http2::connectdata) {
    let mut fresh0 = &mut ((*conn).handler);
    *fresh0 = &Curl_handler_imaps;
    let mut fresh1 = &mut ((*conn).bits);
    (*fresh1).set_tls_upgraded(1 as i32 as bit);
}
unsafe extern "C" fn imap_matchresp(
    mut line: * const i8,
    mut len: u64,
    mut cmd: * const i8,
) -> bool {
    let mut end: * const i8 = line.offset(len as isize);
    let mut cmd_len: u64 = strlen(cmd);
    line = line.offset(2 as i32 as isize);
    if line < end && Curl_isdigit(*line as u8 as i32) != 0 {
        loop {
            line = line.offset(1);
            if !(line < end && Curl_isdigit(*line as u8 as i32) != 0)
            {
                break;
            }
        }
        if line == end || *line as i32 != ' ' as i32 {
            return 0 as i32 != 0;
        }
        line = line.offset(1);
    }
    if line.offset(cmd_len as isize) <= end
        && Curl_strncasecompare(line, cmd, cmd_len) != 0
        && (*line.offset(cmd_len as isize) as i32 == ' ' as i32
            || line.offset(cmd_len as isize).offset(2 as i32 as isize) == end)
    {
        return 1 as i32 != 0;
    }
    return 0 as i32 != 0;
}
unsafe extern "C" fn imap_endofresp(
    mut data: * mut crate::src::lib::http2::Curl_easy,
    mut conn: * mut crate::src::lib::http2::connectdata,
    mut line: * mut i8,
    mut len: u64,
    mut resp: * mut i32,
) -> bool {
    let mut imap: * mut crate::src::lib::http2::IMAP = (*data).req.p.imap;
    let mut imapc: Option<&'_ mut crate::src::lib::http2::imap_conn> = Some(&mut (*conn).proto.imapc);
    let mut id: * const i8 = ((*(borrow_mut(&mut imapc)).unwrap()).resptag).as_mut_ptr();
    let mut id_len: u64 = strlen(id);
    if len >= id_len.wrapping_add(1 as i32 as u64)
        && memcmp(id as *const libc::c_void, line as *const libc::c_void, id_len) == 0
        && *line.offset(id_len as isize) as i32 == ' ' as i32
    {
        line = line
            .offset(id_len.wrapping_add(1 as i32 as u64) as isize);
        len = (len as u64)
            .wrapping_sub(id_len.wrapping_add(1 as i32 as u64))
            as size_t as size_t;
        if len >= 2 as i32 as u64
            && memcmp(
                line as *const libc::c_void,
                b"OK\0" as *const u8 as *const i8 as *const libc::c_void,
                2 as i32 as u64,
            ) == 0
        {
            *resp = 1 as i32;
        } else if len >= 7 as i32 as u64
                && memcmp(
                    line as *const libc::c_void,
                    b"PREAUTH\0" as *const u8 as *const i8
                        as *const libc::c_void,
                    7 as i32 as u64,
                ) == 0
            {
            *resp = 3 as i32;
        } else {
            *resp = 2 as i32;
        }
        return 1 as i32 != 0;
    }
    if len >= 2 as i32 as u64
        && memcmp(
            b"* \0" as *const u8 as *const i8 as *const libc::c_void,
            line as *const libc::c_void,
            2 as i32 as u64,
        ) == 0
    {
        match (*(borrow(& imapc)).unwrap()).state as u32 {
            2 => {
                if !imap_matchresp(
                    line,
                    len,
                    b"CAPABILITY\0" as *const u8 as *const i8,
                ) {
                    return 0 as i32 != 0;
                }
            }
            7 => {
                if ((*imap).custom).is_null()
                    && !imap_matchresp(
                        line,
                        len,
                        b"LIST\0" as *const u8 as *const i8,
                    )
                    || !((*imap).custom).is_null()
                        && !imap_matchresp(line, len, (*imap).custom)
                        && (Curl_strcasecompare(
                            (*imap).custom,
                            b"STORE\0" as *const u8 as *const i8,
                        ) == 0
                            || !imap_matchresp(
                                line,
                                len,
                                b"FETCH\0" as *const u8 as *const i8,
                            ))
                        && Curl_strcasecompare(
                            (*imap).custom,
                            b"SELECT\0" as *const u8 as *const i8,
                        ) == 0
                        && Curl_strcasecompare(
                            (*imap).custom,
                            b"EXAMINE\0" as *const u8 as *const i8,
                        ) == 0
                        && Curl_strcasecompare(
                            (*imap).custom,
                            b"SEARCH\0" as *const u8 as *const i8,
                        ) == 0
                        && Curl_strcasecompare(
                            (*imap).custom,
                            b"EXPUNGE\0" as *const u8 as *const i8,
                        ) == 0
                        && Curl_strcasecompare(
                            (*imap).custom,
                            b"LSUB\0" as *const u8 as *const i8,
                        ) == 0
                        && Curl_strcasecompare(
                            (*imap).custom,
                            b"UID\0" as *const u8 as *const i8,
                        ) == 0
                        && Curl_strcasecompare(
                            (*imap).custom,
                            b"NOOP\0" as *const u8 as *const i8,
                        ) == 0
                {
                    return 0 as i32 != 0;
                }
            }
            8 => {}
            9 => {
                if !imap_matchresp(
                    line,
                    len,
                    b"FETCH\0" as *const u8 as *const i8,
                ) {
                    return 0 as i32 != 0;
                }
            }
            13 => {
                if !imap_matchresp(
                    line,
                    len,
                    b"SEARCH\0" as *const u8 as *const i8,
                ) {
                    return 0 as i32 != 0;
                }
            }
            _ => return 0 as i32 != 0,
        }
        *resp = '*' as i32;
        return 1 as i32 != 0;
    }
    if !imap.is_null() && ((*imap).custom).is_null()
        && (len == 3 as i32 as u64
            && *line.offset(0 as i32 as isize) as i32 == '+' as i32
            || len >= 2 as i32 as u64
                && memcmp(
                    b"+ \0" as *const u8 as *const i8 as *const libc::c_void,
                    line as *const libc::c_void,
                    2 as i32 as u64,
                ) == 0)
    {
        match (*(borrow(& imapc)).unwrap()).state as u32 {
            5 | 11 => {
                *resp = '+' as i32;
            }
            _ => {
                Curl_failf(
                    data,
                    b"Unexpected continuation response\0" as *const u8
                        as *const i8,
                );
                *resp = -(1 as i32);
            }
        }
        return 1 as i32 != 0;
    }
    return 0 as i32 != 0;
}
unsafe extern "C" fn imap_get_message(
    mut buffer: * mut i8,
    mut outptr: * mut * mut i8,
) {
    let mut len: u64 = strlen(buffer);
    let mut message: * mut i8 = 0 as *mut i8;
    if len > 2 as i32 as u64 {
        len = (len as u64).wrapping_sub(2 as i32 as u64)
            as size_t as size_t;
        message = buffer.offset(2 as i32 as isize);
        while *message as i32 == ' ' as i32
            || *message as i32 == '\t' as i32
        {
            message = message.offset(1);
            len = len.wrapping_sub(1);
        }
        loop {
            let mut fresh2 = len;
            len = len.wrapping_sub(1);
            if !(fresh2 != 0) {
                break;
            }
            if *message.offset(len as isize) as i32 != '\r' as i32
                && *message.offset(len as isize) as i32 != '\n' as i32
                && *message.offset(len as isize) as i32 != ' ' as i32
                && *message.offset(len as isize) as i32 != '\t' as i32
            {
                break;
            }
        }
        len = len.wrapping_add(1);
        if len != 0 {
            *message.offset(len as isize) = '\u{0}' as i32 as i8;
        }
    } else {
        message = &mut *buffer.offset(len as isize) as *mut i8;
    }
    *outptr = message;
}
unsafe extern "C" fn state(mut data: * mut crate::src::lib::http2::Curl_easy, mut newstate: u32) {
    let mut imapc: Option<&'_ mut crate::src::lib::http2::imap_conn> = Some(&mut (*(*data).conn).proto.imapc);
    (*(borrow_mut(&mut imapc)).unwrap()).state = newstate;
}
unsafe extern "C" fn imap_perform_capability(
    mut data: * mut crate::src::lib::http2::Curl_easy,
    mut conn: * mut crate::src::lib::http2::connectdata,
) -> u32 {
    let mut result: u32 = CURLE_OK;
    let mut imapc: Option<&'_ mut crate::src::lib::http2::imap_conn> = Some(&mut (*conn).proto.imapc);
    (*(borrow_mut(&mut imapc)).unwrap()).sasl.authmechs = 0 as i32 as u16;
    (*(borrow_mut(&mut imapc)).unwrap()).sasl.authused = 0 as i32 as u16;
    (*(borrow_mut(&mut imapc)).unwrap()).tls_supported = 0 as i32 != 0;
    result = imap_sendf(data, conn, b"CAPABILITY\0" as *const u8 as *const i8);
    if result as u64 == 0 {
        state(data, IMAP_CAPABILITY);
    }
    return result;
}
unsafe extern "C" fn imap_perform_starttls(
    mut data: * mut crate::src::lib::http2::Curl_easy,
    mut conn: * mut crate::src::lib::http2::connectdata,
) -> u32 {
    let mut result: u32 = imap_sendf(
        data,
        conn,
        b"STARTTLS\0" as *const u8 as *const i8,
    );
    if result as u64 == 0 {
        state(data, IMAP_STARTTLS);
    }
    return result;
}
unsafe extern "C" fn imap_perform_upgrade_tls(
    mut data: * mut crate::src::lib::http2::Curl_easy,
    mut conn: * mut crate::src::lib::http2::connectdata,
) -> u32 {
    let mut imapc: Option<&'_ mut crate::src::lib::http2::imap_conn> = Some(&mut (*conn).proto.imapc);
    let mut result: u32 = Curl_ssl_connect_nonblocking(
        data,
        conn,
        0 as i32 != 0,
        0 as i32,
        &mut (*(borrow_mut(&mut imapc)).unwrap()).ssldone,
    );
    if result as u64 == 0 {
        if (*(borrow(& imapc)).unwrap()).state as u32
            != IMAP_UPGRADETLS as i32 as u32
        {
            state(data, IMAP_UPGRADETLS);
        }
        if (*(borrow(& imapc)).unwrap()).ssldone {
            imap_to_imaps(conn);
            result = imap_perform_capability(data, conn);
        }
    }
    return result;
}
unsafe extern "C" fn imap_perform_login(
    mut data: * mut crate::src::lib::http2::Curl_easy,
    mut conn: * mut crate::src::lib::http2::connectdata,
) -> u32 {
    let mut result: u32 = CURLE_OK;
    let mut user: * mut i8 = 0 as *mut i8;
    let mut passwd: * mut i8 = 0 as *mut i8;
    if ((*conn).bits).user_passwd() == 0 {
        state(data, IMAP_STOP);
        return result;
    }
    user = imap_atom((*conn).user, 0 as i32 != 0);
    passwd = imap_atom((*conn).passwd, 0 as i32 != 0);
    result = imap_sendf(
        data,
        conn,
        b"LOGIN %s %s\0" as *const u8 as *const i8,
        if !user.is_null() {
            user as *const i8
        } else {
            b"\0" as *const u8 as *const i8
        },
        if !passwd.is_null() {
            passwd as *const i8
        } else {
            b"\0" as *const u8 as *const i8
        },
    );
    Curl_cfree.expect("non-null function pointer")(user as *mut libc::c_void);
    Curl_cfree.expect("non-null function pointer")(passwd as *mut libc::c_void);
    if result as u64 == 0 {
        state(data, IMAP_LOGIN);
    }
    return result;
}
unsafe extern "C" fn imap_perform_authenticate(
    mut data: * mut crate::src::lib::http2::Curl_easy,
    mut conn: * mut crate::src::lib::http2::connectdata,
    mut mech: * const i8,
    mut initresp: * const i8,
) -> u32 {
    let mut result: u32 = CURLE_OK;
    if !initresp.is_null() {
        result = imap_sendf(
            data,
            conn,
            b"AUTHENTICATE %s %s\0" as *const u8 as *const i8,
            mech,
            initresp,
        );
    } else {
        result = imap_sendf(
            data,
            conn,
            b"AUTHENTICATE %s\0" as *const u8 as *const i8,
            mech,
        );
    }
    return result;
}
unsafe extern "C" fn imap_continue_authenticate(
    mut data: * mut crate::src::lib::http2::Curl_easy,
    mut conn: * mut crate::src::lib::http2::connectdata,
    mut resp: * const i8,
) -> u32 {
    let mut imapc: Option<&'_ mut crate::src::lib::http2::imap_conn> = Some(&mut (*conn).proto.imapc);
    return Curl_pp_sendf(
        data,
        Some(&mut (*(borrow_mut(&mut imapc)).unwrap()).pp),
        b"%s\0" as *const u8 as *const i8,
        resp,
    );
}
unsafe extern "C" fn imap_perform_authentication(
    mut data: * mut crate::src::lib::http2::Curl_easy,
    mut conn: * mut crate::src::lib::http2::connectdata,
) -> u32 {
    let mut result: u32 = CURLE_OK;
    let mut imapc: * mut crate::src::lib::http2::imap_conn = &mut (*conn).proto.imapc;
    let mut progress: u32 = SASL_IDLE;
    if (*imapc).preauth as i32 != 0
        || !Curl_sasl_can_authenticate(Some(&mut (*imapc).sasl), conn)
    {
        state(data, IMAP_STOP);
        return result;
    }
    result = Curl_sasl_start(
        &mut (*imapc).sasl,
        data,
        conn,
        (*imapc).ir_supported,
        Some(&mut progress),
    );
    if result as u64 == 0 {
        if progress as u32 == SASL_INPROGRESS as i32 as u32 {
            state(data, IMAP_AUTHENTICATE);
        } else if !(*imapc).login_disabled
                && (*imapc).preftype
                    & ((1 as i32) << 0 as i32) as u32 != 0
            {
            result = imap_perform_login(data, conn);
        } else {
            Curl_infof(
                data,
                b"No known authentication mechanisms supported!\0" as *const u8
                    as *const i8,
            );
            result = CURLE_LOGIN_DENIED;
        }
    }
    return result;
}
unsafe extern "C" fn imap_perform_list(mut data: * mut crate::src::lib::http2::Curl_easy) -> u32 {
    let mut result: u32 = CURLE_OK;
    let mut conn: * mut crate::src::lib::http2::connectdata = (*data).conn;
    let mut imap: * mut crate::src::lib::http2::IMAP = (*data).req.p.imap;
    if !((*imap).custom).is_null() {
        result = imap_sendf(
            data,
            conn,
            b"%s%s\0" as *const u8 as *const i8,
            (*imap).custom,
            if !((*imap).custom_params).is_null() {
                (*imap).custom_params as *const i8
            } else {
                b"\0" as *const u8 as *const i8
            },
        );
    } else {
        let mut mailbox: * mut i8 = if !((*imap).mailbox).is_null() {
            imap_atom((*imap).mailbox, 1 as i32 != 0)
        } else {
            Curl_cstrdup
                .expect(
                    "non-null function pointer",
                )(b"\0" as *const u8 as *const i8)
        };
        if mailbox.is_null() {
            return CURLE_OUT_OF_MEMORY;
        }
        result = imap_sendf(
            data,
            conn,
            b"LIST \"%s\" *\0" as *const u8 as *const i8,
            mailbox,
        );
        Curl_cfree.expect("non-null function pointer")(mailbox as *mut libc::c_void);
    }
    if result as u64 == 0 {
        state(data, IMAP_LIST);
    }
    return result;
}
unsafe extern "C" fn imap_perform_select(mut data: * mut crate::src::lib::http2::Curl_easy) -> u32 {
    let mut result: u32 = CURLE_OK;
    let mut conn: * mut crate::src::lib::http2::connectdata = (*data).conn;
    let mut imap: * mut crate::src::lib::http2::IMAP = (*data).req.p.imap;
    let mut imapc: Option<&'_ mut crate::src::lib::http2::imap_conn> = Some(&mut (*conn).proto.imapc);
    let mut mailbox: * mut i8 = 0 as *mut i8;
    Curl_cfree
        .expect("non-null function pointer")((*(borrow_mut(&mut imapc)).unwrap()).mailbox as *mut libc::c_void);
    let mut fresh3 = &mut ((*(borrow_mut(&mut imapc)).unwrap()).mailbox);
    *fresh3 = 0 as *mut i8;
    Curl_cfree
        .expect(
            "non-null function pointer",
        )((*(borrow_mut(&mut imapc)).unwrap()).mailbox_uidvalidity as *mut libc::c_void);
    let mut fresh4 = &mut ((*(borrow_mut(&mut imapc)).unwrap()).mailbox_uidvalidity);
    *fresh4 = 0 as *mut i8;
    if ((*imap).mailbox).is_null() {
        Curl_failf(
            data,
            b"Cannot SELECT without a mailbox.\0" as *const u8 as *const i8,
        );
        return CURLE_URL_MALFORMAT;
    }
    mailbox = imap_atom((*imap).mailbox, 0 as i32 != 0);
    if mailbox.is_null() {
        return CURLE_OUT_OF_MEMORY;
    }
    result = imap_sendf(
        data,
        conn,
        b"SELECT %s\0" as *const u8 as *const i8,
        mailbox,
    );
    Curl_cfree.expect("non-null function pointer")(mailbox as *mut libc::c_void);
    if result as u64 == 0 {
        state(data, IMAP_SELECT);
    }
    return result;
}
unsafe extern "C" fn imap_perform_fetch(
    mut data: * mut crate::src::lib::http2::Curl_easy,
    mut conn: * mut crate::src::lib::http2::connectdata,
) -> u32 {
    let mut result: u32 = CURLE_OK;
    let mut imap: * mut crate::src::lib::http2::IMAP = (*data).req.p.imap;
    if !((*imap).uid).is_null() {
        if !((*imap).partial).is_null() {
            result = imap_sendf(
                data,
                conn,
                b"UID FETCH %s BODY[%s]<%s>\0" as *const u8 as *const i8,
                (*imap).uid,
                if !((*imap).section).is_null() {
                    (*imap).section as *const i8
                } else {
                    b"\0" as *const u8 as *const i8
                },
                (*imap).partial,
            );
        } else {
            result = imap_sendf(
                data,
                conn,
                b"UID FETCH %s BODY[%s]\0" as *const u8 as *const i8,
                (*imap).uid,
                if !((*imap).section).is_null() {
                    (*imap).section as *const i8
                } else {
                    b"\0" as *const u8 as *const i8
                },
            );
        }
    } else if !((*imap).mindex).is_null() {
        if !((*imap).partial).is_null() {
            result = imap_sendf(
                data,
                conn,
                b"FETCH %s BODY[%s]<%s>\0" as *const u8 as *const i8,
                (*imap).mindex,
                if !((*imap).section).is_null() {
                    (*imap).section as *const i8
                } else {
                    b"\0" as *const u8 as *const i8
                },
                (*imap).partial,
            );
        } else {
            result = imap_sendf(
                data,
                conn,
                b"FETCH %s BODY[%s]\0" as *const u8 as *const i8,
                (*imap).mindex,
                if !((*imap).section).is_null() {
                    (*imap).section as *const i8
                } else {
                    b"\0" as *const u8 as *const i8
                },
            );
        }
    } else {
        Curl_failf(
            data,
            b"Cannot FETCH without a UID.\0" as *const u8 as *const i8,
        );
        return CURLE_URL_MALFORMAT;
    }
    if result as u64 == 0 {
        state(data, IMAP_FETCH);
    }
    return result;
}
unsafe extern "C" fn imap_perform_append(mut data: * mut crate::src::lib::http2::Curl_easy) -> u32 {
    let mut result: u32 = CURLE_OK;
    let mut conn: * mut crate::src::lib::http2::connectdata = (*data).conn;
    let mut imap: * mut crate::src::lib::http2::IMAP = (*data).req.p.imap;
    let mut mailbox: * mut i8 = 0 as *mut i8;
    if ((*imap).mailbox).is_null() {
        Curl_failf(
            data,
            b"Cannot APPEND without a mailbox.\0" as *const u8 as *const i8,
        );
        return CURLE_URL_MALFORMAT;
    }
    if (*data).set.mimepost.kind as u32
        != MIMEKIND_NONE as i32 as u32
    {
        (*data).set.mimepost.flags
            &= !((1 as i32) << 1 as i32) as u32;
        curl_mime_headers(
            &mut (*data).set.mimepost,
            (*data).set.headers,
            0 as i32,
        );
        result = Curl_mime_prepare_headers(
            &mut (*data).set.mimepost,
            0 as *const i8,
            0 as *const i8,
            MIMESTRATEGY_MAIL,
        );
        if result as u64 == 0 {
            if (Curl_checkheaders(
                data,
                b"Mime-Version\0" as *const u8 as *const i8,
            ))
                .is_null()
            {
                result = Curl_mime_add_header(
                    Some(&mut (*data).set.mimepost.curlheaders),
                    b"Mime-Version: 1.0\0" as *const u8 as *const i8,
                );
            }
        }
        if result as u64 == 0 {
            result = Curl_mime_rewind(&mut (*data).set.mimepost);
        }
        if result as u64 != 0 {
            return result;
        }
        (*data).state.infilesize = Curl_mime_size(&mut (*data).set.mimepost);
        let mut fresh5 = &mut ((*data).state.fread_func);
        *fresh5 = core::intrinsics::transmute::<Option<unsafe extern "C"  fn(_: * mut i8,_: u64,_: u64,_: * mut core::ffi::c_void,) -> u64>, Option<unsafe extern "C"  fn(_: * mut i8,_: u64,_: u64,_: * mut core::ffi::c_void,) -> u64>>(
            Some(
                Curl_mime_read,
            ),
        );
        let mut fresh6 = &mut ((*data).state.in_0);
        *fresh6 = &mut (*data).set.mimepost as *mut curl_mimepart as *mut libc::c_void;
    }
    if (*data).state.infilesize < 0 as i32 as i64 {
        Curl_failf(
            data,
            b"Cannot APPEND with unknown input file size\0" as *const u8
                as *const i8,
        );
        return CURLE_UPLOAD_FAILED;
    }
    mailbox = imap_atom((*imap).mailbox, 0 as i32 != 0);
    if mailbox.is_null() {
        return CURLE_OUT_OF_MEMORY;
    }
    result = imap_sendf(
        data,
        conn,
        b"APPEND %s (\\Seen) {%ld}\0" as *const u8 as *const i8,
        mailbox,
        (*data).state.infilesize,
    );
    Curl_cfree.expect("non-null function pointer")(mailbox as *mut libc::c_void);
    if result as u64 == 0 {
        state(data, IMAP_APPEND);
    }
    return result;
}
unsafe extern "C" fn imap_perform_search(
    mut data: * mut crate::src::lib::http2::Curl_easy,
    mut conn: * mut crate::src::lib::http2::connectdata,
) -> u32 {
    let mut result: u32 = CURLE_OK;
    let mut imap: * mut crate::src::lib::http2::IMAP = (*data).req.p.imap;
    if ((*imap).query).is_null() {
        Curl_failf(
            data,
            b"Cannot SEARCH without a query string.\0" as *const u8
                as *const i8,
        );
        return CURLE_URL_MALFORMAT;
    }
    result = imap_sendf(
        data,
        conn,
        b"SEARCH %s\0" as *const u8 as *const i8,
        (*imap).query,
    );
    if result as u64 == 0 {
        state(data, IMAP_SEARCH);
    }
    return result;
}
unsafe extern "C" fn imap_perform_logout(
    mut data: * mut crate::src::lib::http2::Curl_easy,
    mut conn: * mut crate::src::lib::http2::connectdata,
) -> u32 {
    let mut result: u32 = imap_sendf(
        data,
        conn,
        b"LOGOUT\0" as *const u8 as *const i8,
    );
    if result as u64 == 0 {
        state(data, IMAP_LOGOUT);
    }
    return result;
}
unsafe extern "C" fn imap_state_servergreet_resp(
    mut data: * mut crate::src::lib::http2::Curl_easy,
    mut imapcode: i32,
    mut instate: u32,
) -> u32 {
    let mut conn: * mut crate::src::lib::http2::connectdata = (*data).conn;
    if imapcode == 3 as i32 {
        let mut imapc: Option<&'_ mut crate::src::lib::http2::imap_conn> = Some(&mut (*conn).proto.imapc);
        (*(borrow_mut(&mut imapc)).unwrap()).preauth = 1 as i32 != 0;
        Curl_infof(
            data,
            b"PREAUTH connection, already authenticated!\0" as *const u8
                as *const i8,
        );
    } else if imapcode != 1 as i32 {
        Curl_failf(
            data,
            b"Got unexpected imap-server response\0" as *const u8 as *const i8,
        );
        return CURLE_WEIRD_SERVER_REPLY;
    }
    return imap_perform_capability(data, conn);
}
unsafe extern "C" fn imap_state_capability_resp(
    mut data: * mut crate::src::lib::http2::Curl_easy,
    mut imapcode: i32,
    mut instate: u32,
) -> u32 {
    let mut result: u32 = CURLE_OK;
    let mut conn: * mut crate::src::lib::http2::connectdata = (*data).conn;
    let mut imapc: Option<&'_ mut crate::src::lib::http2::imap_conn> = Some(&mut (*conn).proto.imapc);
    let mut line: * const i8 = (*data).state.buffer;
    if imapcode == '*' as i32 {
        line = line.offset(2 as i32 as isize);
        loop {
            let mut wordlen: u64 = 0;
            while *line as i32 != 0
                && (*line as i32 == ' ' as i32
                    || *line as i32 == '\t' as i32
                    || *line as i32 == '\r' as i32
                    || *line as i32 == '\n' as i32)
            {
                line = line.offset(1);
            }
            if *line == 0 {
                break;
            }
            wordlen = 0 as i32 as size_t;
            while *line.offset(wordlen as isize) as i32 != 0
                && *line.offset(wordlen as isize) as i32 != ' ' as i32
                && *line.offset(wordlen as isize) as i32 != '\t' as i32
                && *line.offset(wordlen as isize) as i32 != '\r' as i32
                && *line.offset(wordlen as isize) as i32 != '\n' as i32
            {
                wordlen = wordlen.wrapping_add(1);
            }
            if wordlen == 8 as i32 as u64
                && memcmp(
                    line as *const libc::c_void,
                    b"STARTTLS\0" as *const u8 as *const i8
                        as *const libc::c_void,
                    8 as i32 as u64,
                ) == 0
            {
                (*(borrow_mut(&mut imapc)).unwrap()).tls_supported = 1 as i32 != 0;
            } else if wordlen == 13 as i32 as u64
                    && memcmp(
                        line as *const libc::c_void,
                        b"LOGINDISABLED\0" as *const u8 as *const i8
                            as *const libc::c_void,
                        13 as i32 as u64,
                    ) == 0
                {
                (*(borrow_mut(&mut imapc)).unwrap()).login_disabled = 1 as i32 != 0;
            } else if wordlen == 7 as i32 as u64
                    && memcmp(
                        line as *const libc::c_void,
                        b"SASL-IR\0" as *const u8 as *const i8
                            as *const libc::c_void,
                        7 as i32 as u64,
                    ) == 0
                {
                (*(borrow_mut(&mut imapc)).unwrap()).ir_supported = 1 as i32 != 0;
            } else if wordlen > 5 as i32 as u64
                    && memcmp(
                        line as *const libc::c_void,
                        b"AUTH=\0" as *const u8 as *const i8
                            as *const libc::c_void,
                        5 as i32 as u64,
                    ) == 0
                {
                let mut llen: u64 = 0;
                let mut mechbit: u16 = 0;
                line = line.offset(5 as i32 as isize);
                wordlen = (wordlen as u64)
                    .wrapping_sub(5 as i32 as u64) as size_t as size_t;
                mechbit = Curl_sasl_decode_mech(line, wordlen, Some(&mut llen));
                if mechbit as i32 != 0 && llen == wordlen {
                    let mut fresh7 = &mut ((*(borrow_mut(&mut imapc)).unwrap()).sasl.authmechs);
                    *fresh7 = (*fresh7 as i32 | mechbit as i32)
                        as u16;
                }
            }
            line = line.offset(wordlen as isize);
        }
    } else if (*data).set.use_ssl as u32 != 0
            && ((*conn).ssl[0 as i32 as usize]).use_0() == 0
        {
        if imapcode == 1 as i32 && (*(borrow(& imapc)).unwrap()).tls_supported as i32 != 0
            && !(*(borrow(& imapc)).unwrap()).preauth
        {
            result = imap_perform_starttls(data, conn);
        } else if (*data).set.use_ssl as u32
                <= CURLUSESSL_TRY as i32 as u32
            {
            result = imap_perform_authentication(data, conn);
        } else {
            Curl_failf(
                data,
                b"STARTTLS not available.\0" as *const u8 as *const i8,
            );
            result = CURLE_USE_SSL_FAILED;
        }
    } else {
        result = imap_perform_authentication(data, conn);
    }
    return result;
}
unsafe extern "C" fn imap_state_starttls_resp(
    mut data: * mut crate::src::lib::http2::Curl_easy,
    mut imapcode: i32,
    mut instate: u32,
) -> u32 {
    let mut result: u32 = CURLE_OK;
    let mut conn: * mut crate::src::lib::http2::connectdata = (*data).conn;
    if (*(*data).conn).proto.imapc.pp.cache_size != 0 {
        return CURLE_WEIRD_SERVER_REPLY;
    }
    if imapcode != 1 as i32 {
        if (*data).set.use_ssl as u32
            != CURLUSESSL_TRY as i32 as u32
        {
            Curl_failf(data, b"STARTTLS denied\0" as *const u8 as *const i8);
            result = CURLE_USE_SSL_FAILED;
        } else {
            result = imap_perform_authentication(data, conn);
        }
    } else {
        result = imap_perform_upgrade_tls(data, conn);
    }
    return result;
}
unsafe extern "C" fn imap_state_auth_resp(
    mut data: * mut crate::src::lib::http2::Curl_easy,
    mut conn: * mut crate::src::lib::http2::connectdata,
    mut imapcode: i32,
    mut instate: u32,
) -> u32 {
    let mut result: u32 = CURLE_OK;
    let mut imapc: Option<&'_ mut crate::src::lib::http2::imap_conn> = Some(&mut (*conn).proto.imapc);
    let mut progress: u32 = SASL_IDLE;
    result = Curl_sasl_continue(&mut (*(borrow_mut(&mut imapc)).unwrap()).sasl, data, conn, imapcode, Some(&mut progress));
    if result as u64 == 0 {
        match progress as u32 {
            2 => {
                state(data, IMAP_STOP);
            }
            0 => {
                if !(*(borrow(& imapc)).unwrap()).login_disabled
                    && (*(borrow(& imapc)).unwrap()).preftype
                        & ((1 as i32) << 0 as i32) as u32 != 0
                {
                    result = imap_perform_login(data, conn);
                } else {
                    Curl_failf(
                        data,
                        b"Authentication cancelled\0" as *const u8 as *const i8,
                    );
                    result = CURLE_LOGIN_DENIED;
                }
            }
            _ => {}
        }
    }
    return result;
}
unsafe extern "C" fn imap_state_login_resp(
    mut data: * mut crate::src::lib::http2::Curl_easy,
    mut imapcode: i32,
    mut instate: u32,
) -> u32 {
    let mut result: u32 = CURLE_OK;
    if imapcode != 1 as i32 {
        Curl_failf(
            data,
            b"Access denied. %c\0" as *const u8 as *const i8,
            imapcode,
        );
        result = CURLE_LOGIN_DENIED;
    } else {
        state(data, IMAP_STOP);
    }
    return result;
}
unsafe extern "C" fn imap_state_listsearch_resp(
    mut data: * mut crate::src::lib::http2::Curl_easy,
    mut imapcode: i32,
    mut instate: u32,
) -> u32 {
    let mut result: u32 = CURLE_OK;
    let mut line: * mut i8 = (*data).state.buffer;
    let mut len: u64 = strlen(line);
    if imapcode == '*' as i32 {
        *line.offset(len as isize) = '\n' as i32 as i8;
        result = Curl_client_write(
            data,
            (1 as i32) << 0 as i32,
            line,
            len.wrapping_add(1 as i32 as u64),
        );
        *line.offset(len as isize) = '\u{0}' as i32 as i8;
    } else if imapcode != 1 as i32 {
        result = CURLE_QUOTE_ERROR;
    } else {
        state(data, IMAP_STOP);
    }
    return result;
}
unsafe extern "C" fn imap_state_select_resp(
    mut data: * mut crate::src::lib::http2::Curl_easy,
    mut imapcode: i32,
    mut instate: u32,
) -> u32 {
    let mut result: u32 = CURLE_OK;
    let mut conn: * mut crate::src::lib::http2::connectdata = (*data).conn;
    let mut imap: * mut crate::src::lib::http2::IMAP = (*data).req.p.imap;
    let mut imapc: Option<&'_ mut crate::src::lib::http2::imap_conn> = Some(&mut (*conn).proto.imapc);
    let mut line: * const i8 = (*data).state.buffer;
    if imapcode == '*' as i32 {
        let mut tmp: [i8; 20] = [0; 20];
        if sscanf(
            line.offset(2 as i32 as isize),
            b"OK [UIDVALIDITY %19[0123456789]]\0" as *const u8 as *const i8,
            tmp.as_mut_ptr(),
        ) == 1 as i32
        {
            Curl_cfree
                .expect(
                    "non-null function pointer",
                )((*(borrow_mut(&mut imapc)).unwrap()).mailbox_uidvalidity as *mut libc::c_void);
            let mut fresh8 = &mut ((*(borrow_mut(&mut imapc)).unwrap()).mailbox_uidvalidity);
            *fresh8 = 0 as *mut i8;
            let mut fresh9 = &mut ((*(borrow_mut(&mut imapc)).unwrap()).mailbox_uidvalidity);
            *fresh9 = Curl_cstrdup.expect("non-null function pointer")(tmp.as_mut_ptr());
        }
    } else if imapcode == 1 as i32 {
        if !((*imap).uidvalidity).is_null() && !((*(borrow(& imapc)).unwrap()).mailbox_uidvalidity).is_null()
            && Curl_strcasecompare((*imap).uidvalidity, (*(borrow(& imapc)).unwrap()).mailbox_uidvalidity)
                == 0
        {
            Curl_failf(
                data,
                b"Mailbox UIDVALIDITY has changed\0" as *const u8 as *const i8,
            );
            result = CURLE_REMOTE_FILE_NOT_FOUND;
        } else {
            let mut fresh10 = &mut ((*(borrow_mut(&mut imapc)).unwrap()).mailbox);
            *fresh10 = Curl_cstrdup.expect("non-null function pointer")((*imap).mailbox);
            if !((*imap).custom).is_null() {
                result = imap_perform_list(data);
            } else if !((*imap).query).is_null() {
                result = imap_perform_search(data, conn);
            } else {
                result = imap_perform_fetch(data, conn);
            }
        }
    } else {
        Curl_failf(data, b"Select failed\0" as *const u8 as *const i8);
        result = CURLE_LOGIN_DENIED;
    }
    return result;
}
unsafe extern "C" fn imap_state_fetch_resp(
    mut data: * mut crate::src::lib::http2::Curl_easy,
    mut conn: * mut crate::src::lib::http2::connectdata,
    mut imapcode: i32,
    mut instate: u32,
) -> u32 {
    let mut result: u32 = CURLE_OK;
    let mut imapc: Option<&'_ mut crate::src::lib::http2::imap_conn> = Some(&mut (*conn).proto.imapc);
    let mut pp: Option<&'_ mut crate::src::lib::http2::pingpong> = Some(&mut (*(borrow_mut(&mut imapc)).unwrap()).pp);
    let mut ptr: * const i8 = (*data).state.buffer;
    let mut parsed: bool = 0 as i32 != 0;
    let mut size: i64 = 0 as i32 as curl_off_t;
    if imapcode != '*' as i32 {
        Curl_pgrsSetDownloadSize(data, -(1 as i32) as curl_off_t);
        state(data, IMAP_STOP);
        return CURLE_REMOTE_FILE_NOT_FOUND;
    }
    while *ptr as i32 != 0 && *ptr as i32 != '{' as i32 {
        ptr = ptr.offset(1);
    }
    if *ptr as i32 == '{' as i32 {
        let mut endptr: * mut i8 = 0 as *mut i8;
        if curlx_strtoofft(
            ptr.offset(1 as i32 as isize),
            Some(&mut endptr),
            10 as i32,
            Some(&mut size),
        ) as u64 == 0
        {
            if endptr.offset_from(ptr) as i64 > 1 as i32 as i64
                && *endptr.offset(0 as i32 as isize) as i32 == '}' as i32
                && *endptr.offset(1 as i32 as isize) as i32
                    == '\r' as i32
                && *endptr.offset(2 as i32 as isize) as i32
                    == '\u{0}' as i32
            {
                parsed = 1 as i32 != 0;
            }
        }
    }
    if parsed {
        Curl_infof(
            data,
            b"Found %ld bytes to download\0" as *const u8 as *const i8,
            size,
        );
        Curl_pgrsSetDownloadSize(data, size);
        if !((*(borrow(& pp)).unwrap()).cache).is_null() {
            let mut chunk: u64 = (*(borrow_mut(&mut pp)).unwrap()).cache_size;
            if chunk > size as size_t {
                chunk = size as size_t;
            }
            if chunk == 0 {
                state(data, IMAP_STOP);
                return CURLE_OK;
            }
            result = Curl_client_write(
                data,
                (1 as i32) << 0 as i32,
                (*(borrow_mut(&mut pp)).unwrap()).cache,
                chunk,
            );
            if result as u64 != 0 {
                return result;
            }
            let mut fresh11 = &mut ((*data).req.bytecount);
            *fresh11 = (*fresh11 as u64).wrapping_add(chunk) as curl_off_t
                as curl_off_t;
            Curl_infof(
                data,
                b"Written %zu bytes, %lu bytes are left for transfer\0" as *const u8
                    as *const i8,
                chunk,
                (size as u64).wrapping_sub(chunk),
            );
            if (*(borrow(& pp)).unwrap()).cache_size > chunk {
                memmove(
                    (*(borrow_mut(&mut pp)).unwrap()).cache as *mut libc::c_void,
                    ((*(borrow(& pp)).unwrap()).cache).offset(chunk as isize) as *const libc::c_void,
                    ((*(borrow(& pp)).unwrap()).cache_size).wrapping_sub(chunk),
                );
                let mut fresh12 = &mut ((*(borrow_mut(&mut pp)).unwrap()).cache_size);
                *fresh12 = (*fresh12 as u64).wrapping_sub(chunk) as size_t
                    as size_t;
            } else {
                Curl_cfree
                    .expect(
                        "non-null function pointer",
                    )((*(borrow_mut(&mut pp)).unwrap()).cache as *mut libc::c_void);
                let mut fresh13 = &mut ((*(borrow_mut(&mut pp)).unwrap()).cache);
                *fresh13 = 0 as *mut i8;
                (*(borrow_mut(&mut pp)).unwrap()).cache_size = 0 as i32 as size_t;
            }
        }
        if (*data).req.bytecount == size {
            Curl_setup_transfer(
                data,
                -(1 as i32),
                -(1 as i32) as curl_off_t,
                0 as i32 != 0,
                -(1 as i32),
            );
        } else {
            (*data).req.maxdownload = size;
            (*(*data).conn).cselect_bits = 0x1 as i32;
            Curl_setup_transfer(
                data,
                0 as i32,
                size,
                0 as i32 != 0,
                -(1 as i32),
            );
        }
    } else {
        Curl_failf(
            data,
            b"Failed to parse FETCH response.\0" as *const u8 as *const i8,
        );
        result = CURLE_WEIRD_SERVER_REPLY;
    }
    state(data, IMAP_STOP);
    return result;
}
unsafe extern "C" fn imap_state_fetch_final_resp(
    mut data: * mut crate::src::lib::http2::Curl_easy,
    mut imapcode: i32,
    mut instate: u32,
) -> u32 {
    let mut result: u32 = CURLE_OK;
    if imapcode != 1 as i32 {
        result = CURLE_WEIRD_SERVER_REPLY;
    } else {
        state(data, IMAP_STOP);
    }
    return result;
}
unsafe extern "C" fn imap_state_append_resp(
    mut data: * mut crate::src::lib::http2::Curl_easy,
    mut imapcode: i32,
    mut instate: u32,
) -> u32 {
    let mut result: u32 = CURLE_OK;
    if imapcode != '+' as i32 {
        result = CURLE_UPLOAD_FAILED;
    } else {
        Curl_pgrsSetUploadSize(data, (*data).state.infilesize);
        Curl_setup_transfer(
            data,
            -(1 as i32),
            -(1 as i32) as curl_off_t,
            0 as i32 != 0,
            0 as i32,
        );
        state(data, IMAP_STOP);
    }
    return result;
}
unsafe extern "C" fn imap_state_append_final_resp(
    mut data: * mut crate::src::lib::http2::Curl_easy,
    mut imapcode: i32,
    mut instate: u32,
) -> u32 {
    let mut result: u32 = CURLE_OK;
    if imapcode != 1 as i32 {
        result = CURLE_UPLOAD_FAILED;
    } else {
        state(data, IMAP_STOP);
    }
    return result;
}
unsafe extern "C" fn imap_statemachine(
    mut data: * mut crate::src::lib::http2::Curl_easy,
    mut conn: * mut crate::src::lib::http2::connectdata,
) -> u32 {
    let mut result: u32 = CURLE_OK;
    let mut sock: i32 = (*conn).sock[0 as i32 as usize];
    let mut imapcode: i32 = 0;
    let mut imapc: * mut crate::src::lib::http2::imap_conn = &mut (*conn).proto.imapc;
    let mut pp: * mut crate::src::lib::http2::pingpong = &mut (*imapc).pp;
    let mut nread: u64 = 0 as i32 as size_t;
    if (*imapc).state as u32 == IMAP_UPGRADETLS as i32 as u32 {
        return imap_perform_upgrade_tls(data, conn);
    }
    if (*pp).sendleft != 0 {
        return Curl_pp_flushsend(data, pp);
    }
    loop {
        result = Curl_pp_readresp(data, sock, pp, &mut imapcode, Some(&mut nread));
        if result as u64 != 0 {
            return result;
        }
        if imapcode == -(1 as i32) {
            return CURLE_WEIRD_SERVER_REPLY;
        }
        if imapcode == 0 {
            break;
        }
        match (*imapc).state as u32 {
            1 => {
                result = imap_state_servergreet_resp(data, imapcode, (*imapc).state);
            }
            2 => {
                result = imap_state_capability_resp(data, imapcode, (*imapc).state);
            }
            3 => {
                result = imap_state_starttls_resp(data, imapcode, (*imapc).state);
            }
            5 => {
                result = imap_state_auth_resp(data, conn, imapcode, (*imapc).state);
            }
            6 => {
                result = imap_state_login_resp(data, imapcode, (*imapc).state);
            }
            7 | 13 => {
                result = imap_state_listsearch_resp(data, imapcode, (*imapc).state);
            }
            8 => {
                result = imap_state_select_resp(data, imapcode, (*imapc).state);
            }
            9 => {
                result = imap_state_fetch_resp(data, conn, imapcode, (*imapc).state);
            }
            10 => {
                result = imap_state_fetch_final_resp(data, imapcode, (*imapc).state);
            }
            11 => {
                result = imap_state_append_resp(data, imapcode, (*imapc).state);
            }
            12 => {
                result = imap_state_append_final_resp(data, imapcode, (*imapc).state);
            }
            14 | _ => {
                state(data, IMAP_STOP);
            }
        }
        if !(result as u64 == 0
            && (*imapc).state as u32 != IMAP_STOP as i32 as u32
            && Curl_pp_moredata(pp) as i32 != 0)
        {
            break;
        }
    }
    return result;
}
unsafe extern "C" fn imap_multi_statemach(
    mut data: * mut crate::src::lib::http2::Curl_easy,
    mut done: * mut bool,
) -> u32 {
    let mut result: u32 = CURLE_OK;
    let mut conn: * mut crate::src::lib::http2::connectdata = (*data).conn;
    let mut imapc: Option<&'_ mut crate::src::lib::http2::imap_conn> = Some(&mut (*conn).proto.imapc);
    if (*(*conn).handler).flags
        & ((1 as i32) << 0 as i32) as u32 != 0
        && !(*(borrow(& imapc)).unwrap()).ssldone
    {
        result = Curl_ssl_connect_nonblocking(
            data,
            conn,
            0 as i32 != 0,
            0 as i32,
            &mut (*(borrow_mut(&mut imapc)).unwrap()).ssldone,
        );
        if result as u32 != 0 || !(*(borrow(& imapc)).unwrap()).ssldone {
            return result;
        }
    }
    result = Curl_pp_statemach(
        data,
        &mut (*(borrow_mut(&mut imapc)).unwrap()).pp,
        0 as i32 != 0,
        0 as i32 != 0,
    );
    *done = if (*(borrow(& imapc)).unwrap()).state as u32 == IMAP_STOP as i32 as u32
    {
        1 as i32
    } else {
        0 as i32
    } != 0;
    return result;
}
unsafe extern "C" fn imap_block_statemach(
    mut data: * mut crate::src::lib::http2::Curl_easy,
    mut conn: * mut crate::src::lib::http2::connectdata,
    mut disconnecting: bool,
) -> u32 {
    let mut result: u32 = CURLE_OK;
    let mut imapc: Option<&'_ mut crate::src::lib::http2::imap_conn> = Some(&mut (*conn).proto.imapc);
    while (*(borrow(& imapc)).unwrap()).state as u32 != IMAP_STOP as i32 as u32
        && result as u64 == 0
    {
        result = Curl_pp_statemach(
            data,
            &mut (*(borrow_mut(&mut imapc)).unwrap()).pp,
            1 as i32 != 0,
            disconnecting,
        );
    }
    return result;
}
unsafe extern "C" fn imap_init(mut data: * mut crate::src::lib::http2::Curl_easy) -> u32 {
    let mut result: u32 = CURLE_OK;
    let mut imap: * mut crate::src::lib::http2::IMAP = (0 as * mut crate::src::lib::http2::IMAP);
    let mut fresh14 = &mut ((*data).req.p.imap);
    *fresh14 = Curl_ccalloc
        .expect(
            "non-null function pointer",
        )(::std::mem::size_of::<IMAP>() as u64, 1 as i32 as size_t)
        as *mut IMAP;
    imap = *fresh14;
    if imap.is_null() {
        result = CURLE_OUT_OF_MEMORY;
    }
    return result;
}
unsafe extern "C" fn imap_getsock(
    mut data: * mut crate::src::lib::http2::Curl_easy,
    mut conn: * mut crate::src::lib::http2::connectdata,
    mut socks: * mut i32,
) -> i32 {
    return Curl_pp_getsock(data, Some(&mut (*conn).proto.imapc.pp), socks);
}
unsafe extern "C" fn imap_connect(
    mut data: * mut crate::src::lib::http2::Curl_easy,
    mut done: * mut bool,
) -> u32 {
    let mut result: u32 = CURLE_OK;
    let mut conn: * mut crate::src::lib::http2::connectdata = (*data).conn;
    let mut imapc: * mut crate::src::lib::http2::imap_conn = &mut (*conn).proto.imapc;
    let mut pp: Option<&'_ mut crate::src::lib::http2::pingpong> = Some(&mut (*imapc).pp);
    *done = 0 as i32 != 0;
    Curl_conncontrol(conn, 0 as i32);
    (*(borrow_mut(&mut pp)).unwrap()).response_time = (120 as i32 * 1000 as i32) as timediff_t;
    let mut fresh15 = &mut ((*(borrow_mut(&mut pp)).unwrap()).statemachine);
    *fresh15 = Some(
        imap_statemachine,
    );
    let mut fresh16 = &mut ((*(borrow_mut(&mut pp)).unwrap()).endofresp);
    *fresh16 = Some(
        imap_endofresp,
    );
    (*imapc).preftype = !(0 as u32);
    Curl_sasl_init(Some(&mut (*imapc).sasl), &saslimap);
    Curl_dyn_init(
        &mut (*imapc).dyn_0,
        (64 as i32 * 1024 as i32) as size_t,
    );
    Curl_pp_setup(borrow_mut(&mut pp));
    Curl_pp_init(data, borrow_mut(&mut pp));
    result = imap_parse_url_options(conn);
    if result as u64 != 0 {
        return result;
    }
    state(data, IMAP_SERVERGREET);
    strcpy(((*imapc).resptag).as_mut_ptr(), b"*\0" as *const u8 as *const i8);
    result = imap_multi_statemach(data, done);
    return result;
}
unsafe extern "C" fn imap_done(
    mut data: * mut crate::src::lib::http2::Curl_easy,
    mut status: u32,
    mut premature: bool,
) -> u32 {
    let mut result: u32 = CURLE_OK;
    let mut conn: * mut crate::src::lib::http2::connectdata = (*data).conn;
    let mut imap: * mut crate::src::lib::http2::IMAP = (*data).req.p.imap;
    if imap.is_null() {
        return CURLE_OK;
    }
    if status as u64 != 0 {
        Curl_conncontrol(conn, 1 as i32);
        result = status;
    } else if ((*data).set).connect_only() == 0 && ((*imap).custom).is_null()
            && (!((*imap).uid).is_null() || !((*imap).mindex).is_null()
                || ((*data).set).upload() as i32 != 0
                || (*data).set.mimepost.kind as u32
                    != MIMEKIND_NONE as i32 as u32)
        {
        if ((*data).set).upload() == 0
            && (*data).set.mimepost.kind as u32
                == MIMEKIND_NONE as i32 as u32
        {
            state(data, IMAP_FETCH_FINAL);
        } else {
            result = Curl_pp_sendf(
                data,
                Some(&mut (*conn).proto.imapc.pp),
                b"%s\0" as *const u8 as *const i8,
                b"\0" as *const u8 as *const i8,
            );
            if result as u64 == 0 {
                state(data, IMAP_APPEND_FINAL);
            }
        }
        if result as u64 == 0 {
            result = imap_block_statemach(data, conn, 0 as i32 != 0);
        }
    }
    Curl_cfree.expect("non-null function pointer")((*imap).mailbox as *mut libc::c_void);
    let mut fresh17 = &mut ((*imap).mailbox);
    *fresh17 = 0 as *mut i8;
    Curl_cfree
        .expect("non-null function pointer")((*imap).uidvalidity as *mut libc::c_void);
    let mut fresh18 = &mut ((*imap).uidvalidity);
    *fresh18 = 0 as *mut i8;
    Curl_cfree.expect("non-null function pointer")((*imap).uid as *mut libc::c_void);
    let mut fresh19 = &mut ((*imap).uid);
    *fresh19 = 0 as *mut i8;
    Curl_cfree.expect("non-null function pointer")((*imap).mindex as *mut libc::c_void);
    let mut fresh20 = &mut ((*imap).mindex);
    *fresh20 = 0 as *mut i8;
    Curl_cfree.expect("non-null function pointer")((*imap).section as *mut libc::c_void);
    let mut fresh21 = &mut ((*imap).section);
    *fresh21 = 0 as *mut i8;
    Curl_cfree.expect("non-null function pointer")((*imap).partial as *mut libc::c_void);
    let mut fresh22 = &mut ((*imap).partial);
    *fresh22 = 0 as *mut i8;
    Curl_cfree.expect("non-null function pointer")((*imap).query as *mut libc::c_void);
    let mut fresh23 = &mut ((*imap).query);
    *fresh23 = 0 as *mut i8;
    Curl_cfree.expect("non-null function pointer")((*imap).custom as *mut libc::c_void);
    let mut fresh24 = &mut ((*imap).custom);
    *fresh24 = 0 as *mut i8;
    Curl_cfree
        .expect("non-null function pointer")((*imap).custom_params as *mut libc::c_void);
    let mut fresh25 = &mut ((*imap).custom_params);
    *fresh25 = 0 as *mut i8;
    (*imap).transfer = PPTRANSFER_BODY;
    return result;
}
unsafe extern "C" fn imap_perform<'a1>(
    mut data: * mut crate::src::lib::http2::Curl_easy,
    mut connected: Option<&'a1 mut bool>,
    mut dophase_done: * mut bool,
) -> u32 {
    let mut result: u32 = CURLE_OK;
    let mut conn: * mut crate::src::lib::http2::connectdata = (*data).conn;
    let mut imap: * mut crate::src::lib::http2::IMAP = (*data).req.p.imap;
    let mut imapc: Option<&'_ mut crate::src::lib::http2::imap_conn> = Some(&mut (*conn).proto.imapc);
    let mut selected: bool = 0 as i32 != 0;
    if ((*data).set).opt_no_body() != 0 {
        (*imap).transfer = PPTRANSFER_INFO;
    }
    *dophase_done = 0 as i32 != 0;
    if !((*imap).mailbox).is_null() && !((*(borrow(& imapc)).unwrap()).mailbox).is_null()
        && Curl_strcasecompare((*imap).mailbox, (*(borrow(& imapc)).unwrap()).mailbox) != 0
        && (((*imap).uidvalidity).is_null() || ((*(borrow(& imapc)).unwrap()).mailbox_uidvalidity).is_null()
            || Curl_strcasecompare((*imap).uidvalidity, (*(borrow(& imapc)).unwrap()).mailbox_uidvalidity)
                != 0)
    {
        selected = 1 as i32 != 0;
    }
    if ((*data).set).upload() as i32 != 0
        || (*data).set.mimepost.kind as u32
            != MIMEKIND_NONE as i32 as u32
    {
        result = imap_perform_append(data);
    } else if !((*imap).custom).is_null()
            && (selected as i32 != 0 || ((*imap).mailbox).is_null())
        {
        result = imap_perform_list(data);
    } else if ((*imap).custom).is_null() && selected as i32 != 0
            && (!((*imap).uid).is_null() || !((*imap).mindex).is_null())
        {
        result = imap_perform_fetch(data, conn);
    } else if ((*imap).custom).is_null() && selected as i32 != 0
            && !((*imap).query).is_null()
        {
        result = imap_perform_search(data, conn);
    } else if !((*imap).mailbox).is_null() && !selected
            && (!((*imap).custom).is_null() || !((*imap).uid).is_null()
                || !((*imap).mindex).is_null() || !((*imap).query).is_null())
        {
        result = imap_perform_select(data);
    } else {
        result = imap_perform_list(data);
    }
    if result as u64 != 0 {
        return result;
    }
    result = imap_multi_statemach(data, dophase_done);
    *(borrow_mut(&mut connected)).unwrap() = (*conn).bits.tcpconnect[0 as i32 as usize];
    *dophase_done;
    return result;
}
unsafe extern "C" fn imap_do(mut data: * mut crate::src::lib::http2::Curl_easy, mut done: * mut bool) -> u32 {
    let mut result: u32 = CURLE_OK;
    *done = 0 as i32 != 0;
    result = imap_parse_url_path(data);
    if result as u64 != 0 {
        return result;
    }
    result = imap_parse_custom_request(data);
    if result as u64 != 0 {
        return result;
    }
    result = imap_regular_transfer(data, done);
    return result;
}
unsafe extern "C" fn imap_disconnect(
    mut data: * mut crate::src::lib::http2::Curl_easy,
    mut conn: * mut crate::src::lib::http2::connectdata,
    mut dead_connection: bool,
) -> u32 {
    let mut imapc: Option<&'_ mut crate::src::lib::http2::imap_conn> = Some(&mut (*conn).proto.imapc);
    if !dead_connection && ((*conn).bits).protoconnstart() as i32 != 0 {
        if imap_perform_logout(data, conn) as u64 == 0 {
            imap_block_statemach(data, conn, 1 as i32 != 0);
        }
    }
    Curl_pp_disconnect(Some(&mut (*(borrow_mut(&mut imapc)).unwrap()).pp));
    Curl_dyn_free(&mut (*(borrow_mut(&mut imapc)).unwrap()).dyn_0);
    Curl_sasl_cleanup(conn, (*(borrow_mut(&mut imapc)).unwrap()).sasl.authused as u32);
    Curl_cfree
        .expect("non-null function pointer")((*(borrow_mut(&mut imapc)).unwrap()).mailbox as *mut libc::c_void);
    let mut fresh26 = &mut ((*(borrow_mut(&mut imapc)).unwrap()).mailbox);
    *fresh26 = 0 as *mut i8;
    Curl_cfree
        .expect(
            "non-null function pointer",
        )((*(borrow_mut(&mut imapc)).unwrap()).mailbox_uidvalidity as *mut libc::c_void);
    let mut fresh27 = &mut ((*(borrow_mut(&mut imapc)).unwrap()).mailbox_uidvalidity);
    *fresh27 = 0 as *mut i8;
    return CURLE_OK;
}
unsafe extern "C" fn imap_dophase_done(
    mut data: * mut crate::src::lib::http2::Curl_easy,
    mut connected: bool,
) -> u32 {
    let mut imap: * mut crate::src::lib::http2::IMAP = (*data).req.p.imap;
    if (*imap).transfer as u32 != PPTRANSFER_BODY as i32 as u32
    {
        Curl_setup_transfer(
            data,
            -(1 as i32),
            -(1 as i32) as curl_off_t,
            0 as i32 != 0,
            -(1 as i32),
        );
    }
    return CURLE_OK;
}
unsafe extern "C" fn imap_doing(
    mut data: * mut crate::src::lib::http2::Curl_easy,
    mut dophase_done: * mut bool,
) -> u32 {
    let mut result: u32 = imap_multi_statemach(data, dophase_done);
    if !(result as u64 != 0) {
        if *dophase_done {
            result = imap_dophase_done(data, 0 as i32 != 0);
        }
    }
    return result;
}
unsafe extern "C" fn imap_regular_transfer(
    mut data: * mut crate::src::lib::http2::Curl_easy,
    mut dophase_done: * mut bool,
) -> u32 {
    let mut result: u32 = CURLE_OK;
    let mut connected: bool = 0 as i32 != 0;
    (*data).req.size = -(1 as i32) as curl_off_t;
    Curl_pgrsSetUploadCounter(data, 0 as i32 as curl_off_t);
    Curl_pgrsSetDownloadCounter(data, 0 as i32 as curl_off_t);
    Curl_pgrsSetUploadSize(data, -(1 as i32) as curl_off_t);
    Curl_pgrsSetDownloadSize(data, -(1 as i32) as curl_off_t);
    result = imap_perform(data, Some(&mut connected), dophase_done);
    if result as u64 == 0 && *dophase_done as i32 != 0 {
        result = imap_dophase_done(data, connected);
    }
    return result;
}
unsafe extern "C" fn imap_setup_connection(
    mut data: * mut crate::src::lib::http2::Curl_easy,
    mut conn: * mut crate::src::lib::http2::connectdata,
) -> u32 {
    let mut result: u32 = imap_init(data);
    if result as u64 != 0 {
        return result;
    }
    let mut fresh28 = &mut ((*conn).bits);
    (*fresh28).set_tls_upgraded(0 as i32 as bit);
    return CURLE_OK;
}
unsafe extern "C" fn imap_sendf(
    mut data: * mut crate::src::lib::http2::Curl_easy,
    mut conn: * mut crate::src::lib::http2::connectdata,
    mut fmt: * const i8,
    mut args: ...
) -> u32 {
    let mut result: u32 = CURLE_OK;
    let mut imapc: * mut crate::src::lib::http2::imap_conn = &mut (*conn).proto.imapc;
    let mut fresh29 = &mut ((*imapc).cmdid);
    *fresh29 = (*fresh29).wrapping_add(1);
    curl_msnprintf(
        ((*imapc).resptag).as_mut_ptr(),
        ::std::mem::size_of::<[i8; 5]>() as u64,
        b"%c%03d\0" as *const u8 as *const i8,
        'A' as i32
            + curlx_sltosi((*conn).connection_id % 26 as i32 as i64),
        (*fresh29).wrapping_rem(1000 as i32 as u32),
    );
    Curl_dyn_reset(Some(&mut (*imapc).dyn_0));
    result = Curl_dyn_addf(
        &mut (*imapc).dyn_0 as *mut dynbuf,
        b"%s %s\0" as *const u8 as *const i8,
        ((*imapc).resptag).as_mut_ptr(),
        fmt,
    );
    if result as u64 == 0 {
        let mut ap: core::ffi::VaListImpl;
        ap = args.clone();
        result = Curl_pp_vsendf(
            data,
            Some(&mut (*imapc).pp),
            Curl_dyn_ptr(&mut (*imapc).dyn_0),
            ap.as_va_list(),
        );
    }
    return result;
}
unsafe extern "C" fn imap_atom(
    mut str: * const i8,
    mut escape_only: bool,
) -> * mut i8 {
    let atom_specials: [i8; 8] = *core::intrinsics::transmute::<&'_ [u8; 8], &'_ [i8; 8]>(b"(){ %*]\0");
    let mut p1: * const i8 = 0 as *const i8;
    let mut p2: * mut i8 = 0 as *mut i8;
    let mut backsp_count: u64 = 0 as i32 as size_t;
    let mut quote_count: u64 = 0 as i32 as size_t;
    let mut others_exists: bool = 0 as i32 != 0;
    let mut newlen: u64 = 0 as i32 as size_t;
    let mut newstr: * mut i8 = 0 as *mut i8;
    if str.is_null() {
        return 0 as *mut i8;
    }
    p1 = str;
    while *p1 != 0 {
        if *p1 as i32 == '\\' as i32 {
            backsp_count = backsp_count.wrapping_add(1);
        } else if *p1 as i32 == '"' as i32 {
            quote_count = quote_count.wrapping_add(1);
        } else if !escape_only {
            let mut p3: * const i8 = atom_specials.as_ptr();
            while *p3 as i32 != 0 && !others_exists {
                if *p1 as i32 == *p3 as i32 {
                    others_exists = 1 as i32 != 0;
                }
                p3 = p3.offset(1);
            }
        }
        p1 = p1.offset(1);
    }
    if backsp_count == 0 && quote_count == 0 && !others_exists {
        return Curl_cstrdup.expect("non-null function pointer")(str);
    }
    newlen = (strlen(str))
        .wrapping_add(backsp_count)
        .wrapping_add(quote_count)
        .wrapping_add(
            (if escape_only as i32 != 0 {
                0 as i32
            } else {
                2 as i32
            }) as u64,
        );
    newstr = Curl_cmalloc
        .expect(
            "non-null function pointer",
        )(
        newlen
            .wrapping_add(1 as i32 as u64)
            .wrapping_mul(::std::mem::size_of::<i8>() as u64),
    ) as *mut i8;
    if newstr.is_null() {
        return 0 as *mut i8;
    }
    p2 = newstr;
    if !escape_only {
        *newstr.offset(0 as i32 as isize) = '"' as i32 as i8;
        *newstr
            .offset(
                newlen.wrapping_sub(1 as i32 as u64) as isize,
            ) = '"' as i32 as i8;
        p2 = p2.offset(1);
    }
    p1 = str;
    while *p1 != 0 {
        if *p1 as i32 == '\\' as i32 || *p1 as i32 == '"' as i32 {
            *p2 = '\\' as i32 as i8;
            p2 = p2.offset(1);
        }
        *p2 = *p1;
        p1 = p1.offset(1);
        p2 = p2.offset(1);
    }
    *newstr.offset(newlen as isize) = '\u{0}' as i32 as i8;
    return newstr;
}
 extern "C" fn imap_is_bchar(mut ch: i8) -> bool {
    match ch as i32 {
        47 => {}
        61 => {}
        126 => {}
        44 => {}
        58 | 64 | 38 | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 65 | 66 | 67
        | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80 | 81 | 82 | 83
        | 84 | 85 | 86 | 87 | 88 | 89 | 90 | 97 | 98 | 99 | 100 | 101 | 102 | 103 | 104
        | 105 | 106 | 107 | 108 | 109 | 110 | 111 | 112 | 113 | 114 | 115 | 116 | 117
        | 118 | 119 | 120 | 121 | 122 | 45 | 46 | 95 | 33 | 36 | 39 | 40 | 41 | 42 | 43
        | 37 => {}
        _ => return 0 as i32 != 0,
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn imap_parse_url_options(mut conn: * mut crate::src::lib::http2::connectdata) -> u32 {
    let mut result: u32 = CURLE_OK;
    let mut imapc: Option<&'_ mut crate::src::lib::http2::imap_conn> = Some(&mut (*conn).proto.imapc);
    let mut ptr: * const i8 = (*conn).options;
    (*(borrow_mut(&mut imapc)).unwrap()).sasl.resetprefs = 1 as i32 != 0;
    while result as u64 == 0 && !ptr.is_null() && *ptr as i32 != 0 {
        let mut key: * const i8 = ptr;
        let mut value: * const i8 = 0 as *const i8;
        while *ptr as i32 != 0 && *ptr as i32 != '=' as i32 {
            ptr = ptr.offset(1);
        }
        value = ptr.offset(1 as i32 as isize);
        while *ptr as i32 != 0 && *ptr as i32 != ';' as i32 {
            ptr = ptr.offset(1);
        }
        if Curl_strncasecompare(
            key,
            b"AUTH=\0" as *const u8 as *const i8,
            5 as i32 as size_t,
        ) != 0
        {
            result = Curl_sasl_parse_url_auth_option(
                Some(&mut (*(borrow_mut(&mut imapc)).unwrap()).sasl),
                value,
                ptr.offset_from(value) as i64 as size_t,
            );
        } else {
            result = CURLE_URL_MALFORMAT;
        }
        if *ptr as i32 == ';' as i32 {
            ptr = ptr.offset(1);
        }
    }
    match (*(borrow(& imapc)).unwrap()).sasl.prefmech as i32 {
        0 => {
            (*(borrow_mut(&mut imapc)).unwrap()).preftype = 0 as i32 as u32;
        }
        65503 => {
            (*(borrow_mut(&mut imapc)).unwrap()).preftype = !(0 as u32);
        }
        _ => {
            (*(borrow_mut(&mut imapc)).unwrap()).preftype = ((1 as i32) << 1 as i32) as u32;
        }
    }
    return result;
}
unsafe extern "C" fn imap_parse_url_path(mut data: * mut crate::src::lib::http2::Curl_easy) -> u32 {
    let mut result: u32 = CURLE_OK;
    let mut imap: * mut crate::src::lib::http2::IMAP = (*data).req.p.imap;
    let mut begin: * const i8 = &mut *((*data).state.up.path)
        .offset(1 as i32 as isize) as *mut i8;
    let mut ptr: * const i8 = begin;
    while imap_is_bchar(*ptr) {
        ptr = ptr.offset(1);
    }
    if ptr != begin {
        let mut end: * const i8 = ptr;
        if end > begin
            && *end.offset(-(1 as i32) as isize) as i32 == '/' as i32
        {
            end = end.offset(-1);
        }
        result = Curl_urldecode(
            data,
            begin,
            end.offset_from(begin) as i64 as size_t,
            Some(&mut (*imap).mailbox),
            Option::<&'_ mut u64>::None,
            REJECT_CTRL,
        );
        if result as u64 != 0 {
            return result;
        }
    } else {
        let mut fresh30 = &mut ((*imap).mailbox);
        *fresh30 = 0 as *mut i8;
    }
    while *ptr as i32 == ';' as i32 {
        let mut name: * mut i8 = 0 as *mut i8;
        let mut value: * mut i8 = 0 as *mut i8;
        let mut valuelen: u64 = 0;
        ptr = ptr.offset(1);
        begin = ptr;
        while *ptr as i32 != 0 && *ptr as i32 != '=' as i32 {
            ptr = ptr.offset(1);
        }
        if *ptr == 0 {
            return CURLE_URL_MALFORMAT;
        }
        result = Curl_urldecode(
            data,
            begin,
            ptr.offset_from(begin) as i64 as size_t,
            Some(&mut name),
            Option::<&'_ mut u64>::None,
            REJECT_CTRL,
        );
        if result as u64 != 0 {
            return result;
        }
        ptr = ptr.offset(1);
        begin = ptr;
        while imap_is_bchar(*ptr) {
            ptr = ptr.offset(1);
        }
        result = Curl_urldecode(
            data,
            begin,
            ptr.offset_from(begin) as i64 as size_t,
            Some(&mut value),
            Some(&mut valuelen),
            REJECT_CTRL,
        );
        if result as u64 != 0 {
            Curl_cfree.expect("non-null function pointer")(name as *mut libc::c_void);
            return result;
        }
        if Curl_strcasecompare(
            name,
            b"UIDVALIDITY\0" as *const u8 as *const i8,
        ) != 0 && ((*imap).uidvalidity).is_null()
        {
            if valuelen > 0 as i32 as u64
                && *value
                    .offset(
                        valuelen.wrapping_sub(1 as i32 as u64) as isize,
                    ) as i32 == '/' as i32
            {
                *value
                    .offset(
                        valuelen.wrapping_sub(1 as i32 as u64) as isize,
                    ) = '\u{0}' as i32 as i8;
            }
            let mut fresh31 = &mut ((*imap).uidvalidity);
            *fresh31 = value;
            value = 0 as *mut i8;
        } else if Curl_strcasecompare(name, b"UID\0" as *const u8 as *const i8)
                != 0 && ((*imap).uid).is_null()
            {
            if valuelen > 0 as i32 as u64
                && *value
                    .offset(
                        valuelen.wrapping_sub(1 as i32 as u64) as isize,
                    ) as i32 == '/' as i32
            {
                *value
                    .offset(
                        valuelen.wrapping_sub(1 as i32 as u64) as isize,
                    ) = '\u{0}' as i32 as i8;
            }
            let mut fresh32 = &mut ((*imap).uid);
            *fresh32 = value;
            value = 0 as *mut i8;
        } else if Curl_strcasecompare(
                name,
                b"MAILINDEX\0" as *const u8 as *const i8,
            ) != 0 && ((*imap).mindex).is_null()
            {
            if valuelen > 0 as i32 as u64
                && *value
                    .offset(
                        valuelen.wrapping_sub(1 as i32 as u64) as isize,
                    ) as i32 == '/' as i32
            {
                *value
                    .offset(
                        valuelen.wrapping_sub(1 as i32 as u64) as isize,
                    ) = '\u{0}' as i32 as i8;
            }
            let mut fresh33 = &mut ((*imap).mindex);
            *fresh33 = value;
            value = 0 as *mut i8;
        } else if Curl_strcasecompare(
                name,
                b"SECTION\0" as *const u8 as *const i8,
            ) != 0 && ((*imap).section).is_null()
            {
            if valuelen > 0 as i32 as u64
                && *value
                    .offset(
                        valuelen.wrapping_sub(1 as i32 as u64) as isize,
                    ) as i32 == '/' as i32
            {
                *value
                    .offset(
                        valuelen.wrapping_sub(1 as i32 as u64) as isize,
                    ) = '\u{0}' as i32 as i8;
            }
            let mut fresh34 = &mut ((*imap).section);
            *fresh34 = value;
            value = 0 as *mut i8;
        } else if Curl_strcasecompare(
                name,
                b"PARTIAL\0" as *const u8 as *const i8,
            ) != 0 && ((*imap).partial).is_null()
            {
            if valuelen > 0 as i32 as u64
                && *value
                    .offset(
                        valuelen.wrapping_sub(1 as i32 as u64) as isize,
                    ) as i32 == '/' as i32
            {
                *value
                    .offset(
                        valuelen.wrapping_sub(1 as i32 as u64) as isize,
                    ) = '\u{0}' as i32 as i8;
            }
            let mut fresh35 = &mut ((*imap).partial);
            *fresh35 = value;
            value = 0 as *mut i8;
        } else {
            Curl_cfree.expect("non-null function pointer")(name as *mut libc::c_void);
            Curl_cfree.expect("non-null function pointer")(value as *mut libc::c_void);
            return CURLE_URL_MALFORMAT;
        }
        Curl_cfree.expect("non-null function pointer")(name as *mut libc::c_void);
        Curl_cfree.expect("non-null function pointer")(value as *mut libc::c_void);
    }
    if !((*imap).mailbox).is_null() && ((*imap).uid).is_null()
        && ((*imap).mindex).is_null()
    {
        curl_url_get(
            (*data).state.uh,
            CURLUPART_QUERY,
            Some(&mut (*imap).query),
            ((1 as i32) << 6 as i32) as u32,
        );
    }
    if *ptr != 0 {
        return CURLE_URL_MALFORMAT;
    }
    return CURLE_OK;
}
unsafe extern "C" fn imap_parse_custom_request(mut data: * mut crate::src::lib::http2::Curl_easy) -> u32 {
    let mut result: u32 = CURLE_OK;
    let mut imap: * mut crate::src::lib::http2::IMAP = (*data).req.p.imap;
    let mut custom: * const i8 = (*data)
        .set
        .str_0[STRING_CUSTOMREQUEST as i32 as usize];
    if !custom.is_null() {
        result = Curl_urldecode(
            data,
            custom,
            0 as i32 as size_t,
            Some(&mut (*imap).custom),
            Option::<&'_ mut u64>::None,
            REJECT_CTRL,
        );
        if result as u64 == 0 {
            let mut params: * const i8 = (*imap).custom;
            while *params as i32 != 0 && *params as i32 != ' ' as i32 {
                params = params.offset(1);
            }
            if *params != 0 {
                let mut fresh36 = &mut ((*imap).custom_params);
                *fresh36 = Curl_cstrdup.expect("non-null function pointer")(params);
                *((*imap).custom)
                    .offset(
                        params.offset_from((*imap).custom) as i64 as isize,
                    ) = '\u{0}' as i32 as i8;
                if ((*imap).custom_params).is_null() {
                    result = CURLE_OUT_OF_MEMORY;
                }
            }
        }
    }
    return result;
}
use crate::laertes_rt::*;
