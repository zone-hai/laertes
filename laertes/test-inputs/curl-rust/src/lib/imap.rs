use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type Curl_URL;
    pub type thread_data;
    pub type altsvcinfo;
    pub type hsts;
    pub type TELNET;
    pub type smb_request;
    pub type ldapreqinfo;
    pub type contenc_writer;
    pub type psl_ctx_st;
    pub type Curl_share;
    pub type curl_pushheaders;
    pub type http_connect_state;
    pub type ldapconninfo;
    pub type tftp_state_data;
    pub type nghttp2_session;
    pub type Gsasl_session;
    pub type Gsasl;
    pub type ssl_backend_data;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn Curl_isdigit(c: libc::c_int) -> libc::c_int;
    fn curl_mime_headers(
        part: *mut curl_mimepart,
        headers: *mut curl_slist,
        take_ownership: libc::c_int,
    ) -> CURLcode;
    fn curl_url_get(
        handle: *mut CURLU,
        what: CURLUPart,
        part: *mut *mut libc::c_char,
        flags: libc::c_uint,
    ) -> CURLUcode;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn Curl_dyn_init(s: *mut dynbuf, toobig: size_t);
    fn Curl_dyn_free(s: *mut dynbuf);
    fn Curl_dyn_addf(s: *mut dynbuf, fmt: *const libc::c_char, _: ...) -> CURLcode;
    fn Curl_dyn_reset(s: *mut dynbuf);
    fn Curl_dyn_ptr(s: *const dynbuf) -> *mut libc::c_char;
    fn Curl_mime_add_header(
        slp: *mut *mut curl_slist,
        fmt: *const libc::c_char,
        _: ...
    ) -> CURLcode;
    fn Curl_mime_prepare_headers(
        part: *mut curl_mimepart,
        contenttype: *const libc::c_char,
        disposition: *const libc::c_char,
        strategy: mimestrategy,
    ) -> CURLcode;
    fn Curl_mime_size(part: *mut curl_mimepart) -> curl_off_t;
    fn Curl_mime_read(
        buffer: *mut libc::c_char,
        size: size_t,
        nitems: size_t,
        instream: *mut libc::c_void,
    ) -> size_t;
    fn Curl_mime_rewind(part: *mut curl_mimepart) -> CURLcode;
    fn Curl_pp_statemach(
        data: *mut Curl_easy,
        pp: *mut pingpong,
        block: bool,
        disconnecting: bool,
    ) -> CURLcode;
    fn Curl_pp_init(data: *mut Curl_easy, pp: *mut pingpong);
    fn Curl_pp_setup(pp: *mut pingpong);
    fn Curl_pp_sendf(
        data: *mut Curl_easy,
        pp: *mut pingpong,
        fmt: *const libc::c_char,
        _: ...
    ) -> CURLcode;
    fn Curl_pp_vsendf(
        data: *mut Curl_easy,
        pp: *mut pingpong,
        fmt: *const libc::c_char,
        args: ::std::ffi::VaList,
    ) -> CURLcode;
    fn Curl_pp_readresp(
        data: *mut Curl_easy,
        sockfd: curl_socket_t,
        pp: *mut pingpong,
        code: *mut libc::c_int,
        size: *mut size_t,
    ) -> CURLcode;
    fn Curl_pp_flushsend(data: *mut Curl_easy, pp: *mut pingpong) -> CURLcode;
    fn Curl_pp_disconnect(pp: *mut pingpong) -> CURLcode;
    fn Curl_pp_getsock(
        data: *mut Curl_easy,
        pp: *mut pingpong,
        socks: *mut curl_socket_t,
    ) -> libc::c_int;
    fn Curl_pp_moredata(pp: *mut pingpong) -> bool;
    fn Curl_sasl_cleanup(conn: *mut connectdata, authused: libc::c_uint);
    fn Curl_sasl_decode_mech(
        ptr: *const libc::c_char,
        maxlen: size_t,
        len: *mut size_t,
    ) -> libc::c_ushort;
    fn Curl_sasl_parse_url_auth_option(
        sasl: *mut SASL,
        value: *const libc::c_char,
        len: size_t,
    ) -> CURLcode;
    fn Curl_sasl_init(sasl: *mut SASL, params: *const SASLproto);
    fn Curl_sasl_can_authenticate(sasl: *mut SASL, conn: *mut connectdata) -> bool;
    fn Curl_sasl_start(
        sasl: *mut SASL,
        data: *mut Curl_easy,
        conn: *mut connectdata,
        force_ir: bool,
        progress: *mut saslprogress,
    ) -> CURLcode;
    fn Curl_sasl_continue(
        sasl: *mut SASL,
        data: *mut Curl_easy,
        conn: *mut connectdata,
        code: libc::c_int,
        progress: *mut saslprogress,
    ) -> CURLcode;
    fn Curl_infof(_: *mut Curl_easy, fmt: *const libc::c_char, _: ...);
    fn Curl_failf(_: *mut Curl_easy, fmt: *const libc::c_char, _: ...);
    fn Curl_client_write(
        data: *mut Curl_easy,
        type_0: libc::c_int,
        ptr: *mut libc::c_char,
        len: size_t,
    ) -> CURLcode;
    fn Curl_pgrsSetDownloadSize(data: *mut Curl_easy, size: curl_off_t);
    fn Curl_pgrsSetUploadSize(data: *mut Curl_easy, size: curl_off_t);
    fn Curl_pgrsSetDownloadCounter(data: *mut Curl_easy, size: curl_off_t);
    fn Curl_pgrsSetUploadCounter(data: *mut Curl_easy, size: curl_off_t);
    fn Curl_checkheaders(
        data: *const Curl_easy,
        thisheader: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn Curl_setup_transfer(
        data: *mut Curl_easy,
        sockindex: libc::c_int,
        size: curl_off_t,
        getheader: bool,
        writesockindex: libc::c_int,
    );
    fn Curl_urldecode(
        data: *mut Curl_easy,
        string: *const libc::c_char,
        length: size_t,
        ostring: *mut *mut libc::c_char,
        olen: *mut size_t,
        ctrl: urlreject,
    ) -> CURLcode;
    fn curlx_strtoofft(
        str: *const libc::c_char,
        endp: *mut *mut libc::c_char,
        base: libc::c_int,
        num: *mut curl_off_t,
    ) -> CURLofft;
    fn Curl_strcasecompare(
        first: *const libc::c_char,
        second: *const libc::c_char,
    ) -> libc::c_int;
    fn Curl_strncasecompare(
        first: *const libc::c_char,
        second: *const libc::c_char,
        max: size_t,
    ) -> libc::c_int;
    fn Curl_ssl_connect_nonblocking(
        data: *mut Curl_easy,
        conn: *mut connectdata,
        isproxy: bool,
        sockindex: libc::c_int,
        done: *mut bool,
    ) -> CURLcode;
    fn Curl_conncontrol(conn: *mut connectdata, closeit: libc::c_int);
    fn curlx_sltosi(slnum: libc::c_long) -> libc::c_int;
    fn curl_msnprintf(
        buffer: *mut libc::c_char,
        maxlength: size_t,
        format: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    static mut Curl_cmalloc: curl_malloc_callback;
    static mut Curl_cfree: curl_free_callback;
    static mut Curl_cstrdup: curl_strdup_callback;
    static mut Curl_ccalloc: curl_calloc_callback;
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
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;
pub type int32_t = __int32_t;
pub type socklen_t = __socklen_t;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
pub type curl_socklen_t = socklen_t;
pub type curl_off_t = libc::c_long;
pub type va_list = __builtin_va_list;
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
pub struct Curl_easy {
    pub magic: libc::c_uint,
    pub next: *mut Curl_easy,
    pub prev: *mut Curl_easy,
    pub conn: *mut connectdata,
    pub connect_queue: Curl_llist_element,
    pub conn_queue: Curl_llist_element,
    pub mstate: CURLMstate,
    pub result: CURLcode,
    pub msg: Curl_message,
    pub sockets: [curl_socket_t; 5],
    pub actions: [libc::c_uchar; 5],
    pub numsocks: libc::c_int,
    pub dns: Names,
    pub multi: *mut Curl_multi,
    pub multi_easy: *mut Curl_multi,
    pub share: *mut Curl_share,
    pub psl: *mut PslCache,
    pub req: SingleRequest,
    pub set: UserDefined,
    pub cookies: *mut CookieInfo,
    pub hsts: *mut hsts,
    pub asi: *mut altsvcinfo,
    pub progress: Progress,
    pub state: UrlState,
    pub wildcard: WildcardData,
    pub info: PureInfo,
    pub tsi: curl_tlssessioninfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct curl_tlssessioninfo {
    pub backend: curl_sslbackend,
    pub internals: *mut libc::c_void,
}
pub type curl_sslbackend = libc::c_uint;
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct PureInfo {
    pub httpcode: libc::c_int,
    pub httpproxycode: libc::c_int,
    pub httpversion: libc::c_int,
    pub filetime: time_t,
    pub header_size: curl_off_t,
    pub request_size: curl_off_t,
    pub proxyauthavail: libc::c_ulong,
    pub httpauthavail: libc::c_ulong,
    pub numconnects: libc::c_long,
    pub contenttype: *mut libc::c_char,
    pub wouldredirect: *mut libc::c_char,
    pub retry_after: curl_off_t,
    pub conn_primary_ip: [libc::c_char; 46],
    pub conn_primary_port: libc::c_int,
    pub conn_local_ip: [libc::c_char; 46],
    pub conn_local_port: libc::c_int,
    pub conn_scheme: *const libc::c_char,
    pub conn_protocol: libc::c_uint,
    pub certs: curl_certinfo,
    pub pxcode: CURLproxycode,
    #[bitfield(name = "timecond", ty = "bit", bits = "0..=0")]
    pub timecond: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
pub type bit = libc::c_uint;
pub type CURLproxycode = libc::c_uint;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct curl_certinfo {
    pub num_of_certs: libc::c_int,
    pub certinfo: *mut *mut curl_slist,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct curl_slist {
    pub data: *mut libc::c_char,
    pub next: *mut curl_slist,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WildcardData {
    pub state: wildcard_states,
    pub path: *mut libc::c_char,
    pub pattern: *mut libc::c_char,
    pub filelist: Curl_llist,
    pub protdata: *mut libc::c_void,
    pub dtor: wildcard_dtor,
    pub customptr: *mut libc::c_void,
}
pub type wildcard_dtor = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Curl_llist {
    pub head: *mut Curl_llist_element,
    pub tail: *mut Curl_llist_element,
    pub dtor: Curl_llist_dtor,
    pub size: size_t,
}
pub type Curl_llist_dtor = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Curl_llist_element {
    pub ptr: *mut libc::c_void,
    pub prev: *mut Curl_llist_element,
    pub next: *mut Curl_llist_element,
}
pub type wildcard_states = libc::c_uint;
pub const CURLWC_DONE: wildcard_states = 7;
pub const CURLWC_ERROR: wildcard_states = 6;
pub const CURLWC_SKIP: wildcard_states = 5;
pub const CURLWC_CLEAN: wildcard_states = 4;
pub const CURLWC_DOWNLOADING: wildcard_states = 3;
pub const CURLWC_MATCHING: wildcard_states = 2;
pub const CURLWC_INIT: wildcard_states = 1;
pub const CURLWC_CLEAR: wildcard_states = 0;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct UrlState {
    pub conn_cache: *mut conncache,
    pub keeps_speed: curltime,
    pub lastconnect_id: libc::c_long,
    pub headerb: dynbuf,
    pub buffer: *mut libc::c_char,
    pub ulbuf: *mut libc::c_char,
    pub current_speed: curl_off_t,
    pub first_host: *mut libc::c_char,
    pub retrycount: libc::c_int,
    pub first_remote_port: libc::c_int,
    pub session: *mut Curl_ssl_session,
    pub sessionage: libc::c_long,
    pub tempwrite: [tempbuf; 3],
    pub tempcount: libc::c_uint,
    pub os_errno: libc::c_int,
    pub scratch: *mut libc::c_char,
    pub followlocation: libc::c_long,
    pub prev_signal: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    pub digest: digestdata,
    pub proxydigest: digestdata,
    pub authhost: auth,
    pub authproxy: auth,
    pub async_0: Curl_async,
    pub engine: *mut libc::c_void,
    pub expiretime: curltime,
    pub timenode: Curl_tree,
    pub timeoutlist: Curl_llist,
    pub expires: [time_node; 13],
    pub most_recent_ftp_entrypath: *mut libc::c_char,
    pub httpwant: libc::c_uchar,
    pub httpversion: libc::c_uchar,
    #[bitfield(name = "prev_block_had_trailing_cr", ty = "bit", bits = "0..=0")]
    pub prev_block_had_trailing_cr: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 5],
    pub crlf_conversions: curl_off_t,
    pub range: *mut libc::c_char,
    pub resume_from: curl_off_t,
    pub rtsp_next_client_CSeq: libc::c_long,
    pub rtsp_next_server_CSeq: libc::c_long,
    pub rtsp_CSeq_recv: libc::c_long,
    pub infilesize: curl_off_t,
    pub drain: size_t,
    pub fread_func: curl_read_callback,
    pub in_0: *mut libc::c_void,
    pub stream_depends_on: *mut Curl_easy,
    pub stream_weight: libc::c_int,
    pub uh: *mut CURLU,
    pub up: urlpieces,
    pub httpreq: Curl_HttpReq,
    pub url: *mut libc::c_char,
    pub referer: *mut libc::c_char,
    pub cookielist: *mut curl_slist,
    pub resolve: *mut curl_slist,
    pub trailers_bytes_sent: size_t,
    pub trailers_buf: dynbuf,
    pub trailers_state: trailers_state,
    pub aptr: dynamically_allocated_data,
    #[bitfield(name = "multi_owned_by_easy", ty = "bit", bits = "0..=0")]
    #[bitfield(name = "this_is_a_follow", ty = "bit", bits = "1..=1")]
    #[bitfield(name = "refused_stream", ty = "bit", bits = "2..=2")]
    #[bitfield(name = "errorbuf", ty = "bit", bits = "3..=3")]
    #[bitfield(name = "allow_port", ty = "bit", bits = "4..=4")]
    #[bitfield(name = "authproblem", ty = "bit", bits = "5..=5")]
    #[bitfield(name = "ftp_trying_alternative", ty = "bit", bits = "6..=6")]
    #[bitfield(name = "wildcardmatch", ty = "bit", bits = "7..=7")]
    #[bitfield(name = "expect100header", ty = "bit", bits = "8..=8")]
    #[bitfield(name = "disableexpect", ty = "bit", bits = "9..=9")]
    #[bitfield(name = "use_range", ty = "bit", bits = "10..=10")]
    #[bitfield(name = "rangestringalloc", ty = "bit", bits = "11..=11")]
    #[bitfield(name = "done", ty = "bit", bits = "12..=12")]
    #[bitfield(name = "stream_depends_e", ty = "bit", bits = "13..=13")]
    #[bitfield(name = "previouslypending", ty = "bit", bits = "14..=14")]
    #[bitfield(name = "cookie_engine", ty = "bit", bits = "15..=15")]
    #[bitfield(name = "prefer_ascii", ty = "bit", bits = "16..=16")]
    #[bitfield(name = "list_only", ty = "bit", bits = "17..=17")]
    #[bitfield(name = "url_alloc", ty = "bit", bits = "18..=18")]
    #[bitfield(name = "referer_alloc", ty = "bit", bits = "19..=19")]
    #[bitfield(name = "wildcard_resolve", ty = "bit", bits = "20..=20")]
    pub multi_owned_by_easy_this_is_a_follow_refused_stream_errorbuf_allow_port_authproblem_ftp_trying_alternative_wildcardmatch_expect100header_disableexpect_use_range_rangestringalloc_done_stream_depends_e_previouslypending_cookie_engine_prefer_ascii_list_only_url_alloc_referer_alloc_wildcard_resolve: [u8; 3],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dynamically_allocated_data {
    pub proxyuserpwd: *mut libc::c_char,
    pub uagent: *mut libc::c_char,
    pub accept_encoding: *mut libc::c_char,
    pub userpwd: *mut libc::c_char,
    pub rangeline: *mut libc::c_char,
    pub ref_0: *mut libc::c_char,
    pub host: *mut libc::c_char,
    pub cookiehost: *mut libc::c_char,
    pub rtsp_transport: *mut libc::c_char,
    pub te: *mut libc::c_char,
    pub user: *mut libc::c_char,
    pub passwd: *mut libc::c_char,
    pub proxyuser: *mut libc::c_char,
    pub proxypasswd: *mut libc::c_char,
}
pub type trailers_state = libc::c_uint;
pub const TRAILERS_DONE: trailers_state = 3;
pub const TRAILERS_SENDING: trailers_state = 2;
pub const TRAILERS_INITIALIZED: trailers_state = 1;
pub const TRAILERS_NONE: trailers_state = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dynbuf {
    pub bufr: *mut libc::c_char,
    pub leng: size_t,
    pub allc: size_t,
    pub toobig: size_t,
}
pub type Curl_HttpReq = libc::c_uint;
pub const HTTPREQ_HEAD: Curl_HttpReq = 5;
pub const HTTPREQ_PUT: Curl_HttpReq = 4;
pub const HTTPREQ_POST_MIME: Curl_HttpReq = 3;
pub const HTTPREQ_POST_FORM: Curl_HttpReq = 2;
pub const HTTPREQ_POST: Curl_HttpReq = 1;
pub const HTTPREQ_GET: Curl_HttpReq = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct urlpieces {
    pub scheme: *mut libc::c_char,
    pub hostname: *mut libc::c_char,
    pub port: *mut libc::c_char,
    pub user: *mut libc::c_char,
    pub password: *mut libc::c_char,
    pub options: *mut libc::c_char,
    pub path: *mut libc::c_char,
    pub query: *mut libc::c_char,
}
pub type CURLU = Curl_URL;
pub type curl_read_callback = Option::<
    unsafe extern "C" fn(*mut libc::c_char, size_t, size_t, *mut libc::c_void) -> size_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct time_node {
    pub list: Curl_llist_element,
    pub time: curltime,
    pub eid: expire_id,
}
pub type expire_id = libc::c_uint;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct curltime {
    pub tv_sec: time_t,
    pub tv_usec: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Curl_tree {
    pub smaller: *mut Curl_tree,
    pub larger: *mut Curl_tree,
    pub samen: *mut Curl_tree,
    pub samep: *mut Curl_tree,
    pub key: curltime,
    pub payload: *mut libc::c_void,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Curl_async {
    pub hostname: *mut libc::c_char,
    pub dns: *mut Curl_dns_entry,
    pub tdata: *mut thread_data,
    pub resolver: *mut libc::c_void,
    pub port: libc::c_int,
    pub status: libc::c_int,
    #[bitfield(name = "done", ty = "bit", bits = "0..=0")]
    pub done: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Curl_dns_entry {
    pub addr: *mut Curl_addrinfo,
    pub timestamp: time_t,
    pub inuse: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Curl_addrinfo {
    pub ai_flags: libc::c_int,
    pub ai_family: libc::c_int,
    pub ai_socktype: libc::c_int,
    pub ai_protocol: libc::c_int,
    pub ai_addrlen: curl_socklen_t,
    pub ai_canonname: *mut libc::c_char,
    pub ai_addr: *mut sockaddr,
    pub ai_next: *mut Curl_addrinfo,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct auth {
    pub want: libc::c_ulong,
    pub picked: libc::c_ulong,
    pub avail: libc::c_ulong,
    #[bitfield(name = "done", ty = "bit", bits = "0..=0")]
    #[bitfield(name = "multipass", ty = "bit", bits = "1..=1")]
    #[bitfield(name = "iestyle", ty = "bit", bits = "2..=2")]
    pub done_multipass_iestyle: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct digestdata {
    pub nonce: *mut libc::c_char,
    pub cnonce: *mut libc::c_char,
    pub realm: *mut libc::c_char,
    pub algo: libc::c_int,
    pub opaque: *mut libc::c_char,
    pub qop: *mut libc::c_char,
    pub algorithm: *mut libc::c_char,
    pub nc: libc::c_int,
    #[bitfield(name = "stale", ty = "bit", bits = "0..=0")]
    #[bitfield(name = "userhash", ty = "bit", bits = "1..=1")]
    pub stale_userhash: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tempbuf {
    pub b: dynbuf,
    pub type_0: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Curl_ssl_session {
    pub name: *mut libc::c_char,
    pub conn_to_host: *mut libc::c_char,
    pub scheme: *const libc::c_char,
    pub sessionid: *mut libc::c_void,
    pub idsize: size_t,
    pub age: libc::c_long,
    pub remote_port: libc::c_int,
    pub conn_to_port: libc::c_int,
    pub ssl_config: ssl_primary_config,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct ssl_primary_config {
    pub version: libc::c_long,
    pub version_max: libc::c_long,
    pub CApath: *mut libc::c_char,
    pub CAfile: *mut libc::c_char,
    pub issuercert: *mut libc::c_char,
    pub clientcert: *mut libc::c_char,
    pub random_file: *mut libc::c_char,
    pub egdsocket: *mut libc::c_char,
    pub cipher_list: *mut libc::c_char,
    pub cipher_list13: *mut libc::c_char,
    pub pinned_key: *mut libc::c_char,
    pub cert_blob: *mut curl_blob,
    pub ca_info_blob: *mut curl_blob,
    pub issuercert_blob: *mut curl_blob,
    pub curves: *mut libc::c_char,
    #[bitfield(name = "verifypeer", ty = "bit", bits = "0..=0")]
    #[bitfield(name = "verifyhost", ty = "bit", bits = "1..=1")]
    #[bitfield(name = "verifystatus", ty = "bit", bits = "2..=2")]
    #[bitfield(name = "sessionid", ty = "bit", bits = "3..=3")]
    pub verifypeer_verifyhost_verifystatus_sessionid: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct curl_blob {
    pub data: *mut libc::c_void,
    pub len: size_t,
    pub flags: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct conncache {
    pub hash: Curl_hash,
    pub num_conn: size_t,
    pub next_connection_id: libc::c_long,
    pub last_cleanup: curltime,
    pub closure_handle: *mut Curl_easy,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Curl_hash {
    pub table: *mut Curl_llist,
    pub hash_func: hash_function,
    pub comp_func: comp_function,
    pub dtor: Curl_hash_dtor,
    pub slots: libc::c_int,
    pub size: size_t,
}
pub type Curl_hash_dtor = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type comp_function = Option::<
    unsafe extern "C" fn(*mut libc::c_void, size_t, *mut libc::c_void, size_t) -> size_t,
>;
pub type hash_function = Option::<
    unsafe extern "C" fn(*mut libc::c_void, size_t, size_t) -> size_t,
>;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct Progress {
    pub lastshow: time_t,
    pub size_dl: curl_off_t,
    pub size_ul: curl_off_t,
    pub downloaded: curl_off_t,
    pub uploaded: curl_off_t,
    pub current_speed: curl_off_t,
    pub width: libc::c_int,
    pub flags: libc::c_int,
    pub timespent: timediff_t,
    pub dlspeed: curl_off_t,
    pub ulspeed: curl_off_t,
    pub t_nslookup: timediff_t,
    pub t_connect: timediff_t,
    pub t_appconnect: timediff_t,
    pub t_pretransfer: timediff_t,
    pub t_starttransfer: timediff_t,
    pub t_redirect: timediff_t,
    pub start: curltime,
    pub t_startsingle: curltime,
    pub t_startop: curltime,
    pub t_acceptdata: curltime,
    pub ul_limit_start: curltime,
    pub ul_limit_size: curl_off_t,
    pub dl_limit_start: curltime,
    pub dl_limit_size: curl_off_t,
    pub speeder: [curl_off_t; 6],
    pub speeder_time: [curltime; 6],
    pub speeder_c: libc::c_int,
    #[bitfield(name = "callback", ty = "bit", bits = "0..=0")]
    #[bitfield(name = "is_t_startransfer_set", ty = "bit", bits = "1..=1")]
    pub callback_is_t_startransfer_set: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
pub type timediff_t = curl_off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CookieInfo {
    pub cookies: [*mut Cookie; 256],
    pub filename: *mut libc::c_char,
    pub numcookies: libc::c_long,
    pub running: bool,
    pub newsession: bool,
    pub lastct: libc::c_int,
    pub next_expiration: curl_off_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Cookie {
    pub next: *mut Cookie,
    pub name: *mut libc::c_char,
    pub value: *mut libc::c_char,
    pub path: *mut libc::c_char,
    pub spath: *mut libc::c_char,
    pub domain: *mut libc::c_char,
    pub expires: curl_off_t,
    pub expirestr: *mut libc::c_char,
    pub version: *mut libc::c_char,
    pub maxage: *mut libc::c_char,
    pub tailmatch: bool,
    pub secure: bool,
    pub livecookie: bool,
    pub httponly: bool,
    pub creationtime: libc::c_int,
    pub prefix: libc::c_uchar,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct UserDefined {
    pub err: *mut FILE,
    pub debugdata: *mut libc::c_void,
    pub errorbuffer: *mut libc::c_char,
    pub proxyport: libc::c_long,
    pub out: *mut libc::c_void,
    pub in_set: *mut libc::c_void,
    pub writeheader: *mut libc::c_void,
    pub rtp_out: *mut libc::c_void,
    pub use_port: libc::c_long,
    pub httpauth: libc::c_ulong,
    pub proxyauth: libc::c_ulong,
    pub socks5auth: libc::c_ulong,
    pub maxredirs: libc::c_long,
    pub keep_post: libc::c_int,
    pub postfields: *mut libc::c_void,
    pub seek_func: curl_seek_callback,
    pub postfieldsize: curl_off_t,
    pub localport: libc::c_ushort,
    pub localportrange: libc::c_int,
    pub fwrite_func: curl_write_callback,
    pub fwrite_header: curl_write_callback,
    pub fwrite_rtp: curl_write_callback,
    pub fread_func_set: curl_read_callback,
    pub fprogress: curl_progress_callback,
    pub fxferinfo: curl_xferinfo_callback,
    pub fdebug: curl_debug_callback,
    pub ioctl_func: curl_ioctl_callback,
    pub fsockopt: curl_sockopt_callback,
    pub sockopt_client: *mut libc::c_void,
    pub fopensocket: curl_opensocket_callback,
    pub opensocket_client: *mut libc::c_void,
    pub fclosesocket: curl_closesocket_callback,
    pub closesocket_client: *mut libc::c_void,
    pub seek_client: *mut libc::c_void,
    pub convfromnetwork: curl_conv_callback,
    pub convtonetwork: curl_conv_callback,
    pub convfromutf8: curl_conv_callback,
    pub hsts_read: curl_hstsread_callback,
    pub hsts_read_userp: *mut libc::c_void,
    pub hsts_write: curl_hstswrite_callback,
    pub hsts_write_userp: *mut libc::c_void,
    pub progress_client: *mut libc::c_void,
    pub ioctl_client: *mut libc::c_void,
    pub timeout: libc::c_long,
    pub connecttimeout: libc::c_long,
    pub accepttimeout: libc::c_long,
    pub happy_eyeballs_timeout: libc::c_long,
    pub server_response_timeout: libc::c_long,
    pub maxage_conn: libc::c_long,
    pub tftp_blksize: libc::c_long,
    pub filesize: curl_off_t,
    pub low_speed_limit: libc::c_long,
    pub low_speed_time: libc::c_long,
    pub max_send_speed: curl_off_t,
    pub max_recv_speed: curl_off_t,
    pub set_resume_from: curl_off_t,
    pub headers: *mut curl_slist,
    pub proxyheaders: *mut curl_slist,
    pub httppost: *mut curl_httppost,
    pub mimepost: curl_mimepart,
    pub quote: *mut curl_slist,
    pub postquote: *mut curl_slist,
    pub prequote: *mut curl_slist,
    pub source_quote: *mut curl_slist,
    pub source_prequote: *mut curl_slist,
    pub source_postquote: *mut curl_slist,
    pub telnet_options: *mut curl_slist,
    pub resolve: *mut curl_slist,
    pub connect_to: *mut curl_slist,
    pub timecondition: curl_TimeCond,
    pub proxytype: curl_proxytype,
    pub timevalue: time_t,
    pub method: Curl_HttpReq,
    pub httpwant: libc::c_uchar,
    pub ssl: ssl_config_data,
    pub proxy_ssl: ssl_config_data,
    pub general_ssl: ssl_general_config,
    pub dns_cache_timeout: libc::c_long,
    pub buffer_size: libc::c_long,
    pub upload_buffer_size: libc::c_uint,
    pub private_data: *mut libc::c_void,
    pub http200aliases: *mut curl_slist,
    pub ipver: libc::c_uchar,
    pub max_filesize: curl_off_t,
    pub ftp_filemethod: curl_ftpfile,
    pub ftpsslauth: curl_ftpauth,
    pub ftp_ccc: curl_ftpccc,
    pub ftp_create_missing_dirs: libc::c_int,
    pub ssh_keyfunc: curl_sshkeycallback,
    pub ssh_keyfunc_userp: *mut libc::c_void,
    pub use_netrc: CURL_NETRC_OPTION,
    pub use_ssl: curl_usessl,
    pub new_file_perms: libc::c_long,
    pub new_directory_perms: libc::c_long,
    pub ssh_auth_types: libc::c_long,
    pub str_0: [*mut libc::c_char; 80],
    pub blobs: [*mut curl_blob; 8],
    pub scope_id: libc::c_uint,
    pub allowed_protocols: libc::c_long,
    pub redir_protocols: libc::c_long,
    pub mail_rcpt: *mut curl_slist,
    pub rtspreq: Curl_RtspReq,
    pub rtspversion: libc::c_long,
    pub chunk_bgn: curl_chunk_bgn_callback,
    pub chunk_end: curl_chunk_end_callback,
    pub fnmatch: curl_fnmatch_callback,
    pub fnmatch_data: *mut libc::c_void,
    pub gssapi_delegation: libc::c_long,
    pub tcp_keepidle: libc::c_long,
    pub tcp_keepintvl: libc::c_long,
    pub maxconnects: size_t,
    pub expect_100_timeout: libc::c_long,
    pub stream_depends_on: *mut Curl_easy,
    pub stream_weight: libc::c_int,
    pub stream_dependents: *mut Curl_http2_dep,
    pub resolver_start: curl_resolver_start_callback,
    pub resolver_start_client: *mut libc::c_void,
    pub upkeep_interval_ms: libc::c_long,
    pub fmultidone: multidone_func,
    pub dohfor: *mut Curl_easy,
    pub uh: *mut CURLU,
    pub trailer_data: *mut libc::c_void,
    pub trailer_callback: curl_trailer_callback,
    #[bitfield(name = "is_fread_set", ty = "bit", bits = "0..=0")]
    #[bitfield(name = "is_fwrite_set", ty = "bit", bits = "1..=1")]
    #[bitfield(name = "free_referer", ty = "bit", bits = "2..=2")]
    #[bitfield(name = "tftp_no_options", ty = "bit", bits = "3..=3")]
    #[bitfield(name = "sep_headers", ty = "bit", bits = "4..=4")]
    #[bitfield(name = "cookiesession", ty = "bit", bits = "5..=5")]
    #[bitfield(name = "crlf", ty = "bit", bits = "6..=6")]
    #[bitfield(name = "strip_path_slash", ty = "bit", bits = "7..=7")]
    #[bitfield(name = "ssh_compression", ty = "bit", bits = "8..=8")]
    #[bitfield(name = "get_filetime", ty = "bit", bits = "9..=9")]
    #[bitfield(name = "tunnel_thru_httpproxy", ty = "bit", bits = "10..=10")]
    #[bitfield(name = "prefer_ascii", ty = "bit", bits = "11..=11")]
    #[bitfield(name = "remote_append", ty = "bit", bits = "12..=12")]
    #[bitfield(name = "list_only", ty = "bit", bits = "13..=13")]
    #[bitfield(name = "ftp_use_port", ty = "bit", bits = "14..=14")]
    #[bitfield(name = "ftp_use_epsv", ty = "bit", bits = "15..=15")]
    #[bitfield(name = "ftp_use_eprt", ty = "bit", bits = "16..=16")]
    #[bitfield(name = "ftp_use_pret", ty = "bit", bits = "17..=17")]
    #[bitfield(name = "ftp_skip_ip", ty = "bit", bits = "18..=18")]
    #[bitfield(name = "hide_progress", ty = "bit", bits = "19..=19")]
    #[bitfield(name = "http_fail_on_error", ty = "bit", bits = "20..=20")]
    #[bitfield(name = "http_keep_sending_on_error", ty = "bit", bits = "21..=21")]
    #[bitfield(name = "http_follow_location", ty = "bit", bits = "22..=22")]
    #[bitfield(name = "http_transfer_encoding", ty = "bit", bits = "23..=23")]
    #[bitfield(name = "allow_auth_to_other_hosts", ty = "bit", bits = "24..=24")]
    #[bitfield(name = "include_header", ty = "bit", bits = "25..=25")]
    #[bitfield(name = "http_set_referer", ty = "bit", bits = "26..=26")]
    #[bitfield(name = "http_auto_referer", ty = "bit", bits = "27..=27")]
    #[bitfield(name = "opt_no_body", ty = "bit", bits = "28..=28")]
    #[bitfield(name = "upload", ty = "bit", bits = "29..=29")]
    #[bitfield(name = "verbose", ty = "bit", bits = "30..=30")]
    #[bitfield(name = "krb", ty = "bit", bits = "31..=31")]
    #[bitfield(name = "reuse_forbid", ty = "bit", bits = "32..=32")]
    #[bitfield(name = "reuse_fresh", ty = "bit", bits = "33..=33")]
    #[bitfield(name = "no_signal", ty = "bit", bits = "34..=34")]
    #[bitfield(name = "tcp_nodelay", ty = "bit", bits = "35..=35")]
    #[bitfield(name = "ignorecl", ty = "bit", bits = "36..=36")]
    #[bitfield(name = "connect_only", ty = "bit", bits = "37..=37")]
    #[bitfield(name = "http_te_skip", ty = "bit", bits = "38..=38")]
    #[bitfield(name = "http_ce_skip", ty = "bit", bits = "39..=39")]
    #[bitfield(name = "proxy_transfer_mode", ty = "bit", bits = "40..=40")]
    #[bitfield(name = "sasl_ir", ty = "bit", bits = "41..=41")]
    #[bitfield(name = "wildcard_enabled", ty = "bit", bits = "42..=42")]
    #[bitfield(name = "tcp_keepalive", ty = "bit", bits = "43..=43")]
    #[bitfield(name = "tcp_fastopen", ty = "bit", bits = "44..=44")]
    #[bitfield(name = "ssl_enable_npn", ty = "bit", bits = "45..=45")]
    #[bitfield(name = "ssl_enable_alpn", ty = "bit", bits = "46..=46")]
    #[bitfield(name = "path_as_is", ty = "bit", bits = "47..=47")]
    #[bitfield(name = "pipewait", ty = "bit", bits = "48..=48")]
    #[bitfield(name = "suppress_connect_headers", ty = "bit", bits = "49..=49")]
    #[bitfield(name = "dns_shuffle_addresses", ty = "bit", bits = "50..=50")]
    #[bitfield(name = "stream_depends_e", ty = "bit", bits = "51..=51")]
    #[bitfield(name = "haproxyprotocol", ty = "bit", bits = "52..=52")]
    #[bitfield(name = "abstract_unix_socket", ty = "bit", bits = "53..=53")]
    #[bitfield(name = "disallow_username_in_url", ty = "bit", bits = "54..=54")]
    #[bitfield(name = "doh", ty = "bit", bits = "55..=55")]
    #[bitfield(name = "doh_get", ty = "bit", bits = "56..=56")]
    #[bitfield(name = "doh_verifypeer", ty = "bit", bits = "57..=57")]
    #[bitfield(name = "doh_verifyhost", ty = "bit", bits = "58..=58")]
    #[bitfield(name = "doh_verifystatus", ty = "bit", bits = "59..=59")]
    #[bitfield(name = "http09_allowed", ty = "bit", bits = "60..=60")]
    #[bitfield(name = "mail_rcpt_allowfails", ty = "bit", bits = "61..=61")]
    pub is_fread_set_is_fwrite_set_free_referer_tftp_no_options_sep_headers_cookiesession_crlf_strip_path_slash_ssh_compression_get_filetime_tunnel_thru_httpproxy_prefer_ascii_remote_append_list_only_ftp_use_port_ftp_use_epsv_ftp_use_eprt_ftp_use_pret_ftp_skip_ip_hide_progress_http_fail_on_error_http_keep_sending_on_error_http_follow_location_http_transfer_encoding_allow_auth_to_other_hosts_include_header_http_set_referer_http_auto_referer_opt_no_body_upload_verbose_krb_reuse_forbid_reuse_fresh_no_signal_tcp_nodelay_ignorecl_connect_only_http_te_skip_http_ce_skip_proxy_transfer_mode_sasl_ir_wildcard_enabled_tcp_keepalive_tcp_fastopen_ssl_enable_npn_ssl_enable_alpn_path_as_is_pipewait_suppress_connect_headers_dns_shuffle_addresses_stream_depends_e_haproxyprotocol_abstract_unix_socket_disallow_username_in_url_doh_doh_get_doh_verifypeer_doh_verifyhost_doh_verifystatus_http09_allowed_mail_rcpt_allowfails: [u8; 8],
}
pub type curl_trailer_callback = Option::<
    unsafe extern "C" fn(*mut *mut curl_slist, *mut libc::c_void) -> libc::c_int,
>;
pub type multidone_func = Option::<
    unsafe extern "C" fn(*mut Curl_easy, CURLcode) -> libc::c_int,
>;
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
pub type curl_resolver_start_callback = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *mut libc::c_void,
        *mut libc::c_void,
    ) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Curl_http2_dep {
    pub next: *mut Curl_http2_dep,
    pub data: *mut Curl_easy,
}
pub type curl_fnmatch_callback = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        *const libc::c_char,
        *const libc::c_char,
    ) -> libc::c_int,
>;
pub type curl_chunk_end_callback = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> libc::c_long,
>;
pub type curl_chunk_bgn_callback = Option::<
    unsafe extern "C" fn(
        *const libc::c_void,
        *mut libc::c_void,
        libc::c_int,
    ) -> libc::c_long,
>;
pub type Curl_RtspReq = libc::c_uint;
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
pub type curl_usessl = libc::c_uint;
pub const CURLUSESSL_LAST: curl_usessl = 4;
pub const CURLUSESSL_ALL: curl_usessl = 3;
pub const CURLUSESSL_CONTROL: curl_usessl = 2;
pub const CURLUSESSL_TRY: curl_usessl = 1;
pub const CURLUSESSL_NONE: curl_usessl = 0;
pub type CURL_NETRC_OPTION = libc::c_uint;
pub const CURL_NETRC_LAST: CURL_NETRC_OPTION = 3;
pub const CURL_NETRC_REQUIRED: CURL_NETRC_OPTION = 2;
pub const CURL_NETRC_OPTIONAL: CURL_NETRC_OPTION = 1;
pub const CURL_NETRC_IGNORED: CURL_NETRC_OPTION = 0;
pub type curl_sshkeycallback = Option::<
    unsafe extern "C" fn(
        *mut CURL,
        *const curl_khkey,
        *const curl_khkey,
        curl_khmatch,
        *mut libc::c_void,
    ) -> libc::c_int,
>;
pub type curl_khmatch = libc::c_uint;
pub const CURLKHMATCH_LAST: curl_khmatch = 3;
pub const CURLKHMATCH_MISSING: curl_khmatch = 2;
pub const CURLKHMATCH_MISMATCH: curl_khmatch = 1;
pub const CURLKHMATCH_OK: curl_khmatch = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct curl_khkey {
    pub key: *const libc::c_char,
    pub len: size_t,
    pub keytype: curl_khtype,
}
pub type curl_khtype = libc::c_uint;
pub const CURLKHTYPE_ED25519: curl_khtype = 5;
pub const CURLKHTYPE_ECDSA: curl_khtype = 4;
pub const CURLKHTYPE_DSS: curl_khtype = 3;
pub const CURLKHTYPE_RSA: curl_khtype = 2;
pub const CURLKHTYPE_RSA1: curl_khtype = 1;
pub const CURLKHTYPE_UNKNOWN: curl_khtype = 0;
pub type CURL = Curl_easy;
pub type curl_ftpccc = libc::c_uint;
pub const CURLFTPSSL_CCC_LAST: curl_ftpccc = 3;
pub const CURLFTPSSL_CCC_ACTIVE: curl_ftpccc = 2;
pub const CURLFTPSSL_CCC_PASSIVE: curl_ftpccc = 1;
pub const CURLFTPSSL_CCC_NONE: curl_ftpccc = 0;
pub type curl_ftpauth = libc::c_uint;
pub const CURLFTPAUTH_LAST: curl_ftpauth = 3;
pub const CURLFTPAUTH_TLS: curl_ftpauth = 2;
pub const CURLFTPAUTH_SSL: curl_ftpauth = 1;
pub const CURLFTPAUTH_DEFAULT: curl_ftpauth = 0;
pub type curl_ftpfile = libc::c_uint;
pub const FTPFILE_SINGLECWD: curl_ftpfile = 3;
pub const FTPFILE_NOCWD: curl_ftpfile = 2;
pub const FTPFILE_MULTICWD: curl_ftpfile = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ssl_general_config {
    pub max_ssl_sessions: size_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct ssl_config_data {
    pub primary: ssl_primary_config,
    pub certverifyresult: libc::c_long,
    pub CRLfile: *mut libc::c_char,
    pub fsslctx: curl_ssl_ctx_callback,
    pub fsslctxp: *mut libc::c_void,
    pub cert_type: *mut libc::c_char,
    pub key: *mut libc::c_char,
    pub key_blob: *mut curl_blob,
    pub key_type: *mut libc::c_char,
    pub key_passwd: *mut libc::c_char,
    pub username: *mut libc::c_char,
    pub password: *mut libc::c_char,
    pub authtype: CURL_TLSAUTH,
    #[bitfield(name = "certinfo", ty = "bit", bits = "0..=0")]
    #[bitfield(name = "falsestart", ty = "bit", bits = "1..=1")]
    #[bitfield(name = "enable_beast", ty = "bit", bits = "2..=2")]
    #[bitfield(name = "no_revoke", ty = "bit", bits = "3..=3")]
    #[bitfield(name = "no_partialchain", ty = "bit", bits = "4..=4")]
    #[bitfield(name = "revoke_best_effort", ty = "bit", bits = "5..=5")]
    #[bitfield(name = "native_ca_store", ty = "bit", bits = "6..=6")]
    #[bitfield(name = "auto_client_cert", ty = "bit", bits = "7..=7")]
    pub certinfo_falsestart_enable_beast_no_revoke_no_partialchain_revoke_best_effort_native_ca_store_auto_client_cert: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
pub type CURL_TLSAUTH = libc::c_uint;
pub const CURL_TLSAUTH_LAST: CURL_TLSAUTH = 2;
pub const CURL_TLSAUTH_SRP: CURL_TLSAUTH = 1;
pub const CURL_TLSAUTH_NONE: CURL_TLSAUTH = 0;
pub type curl_ssl_ctx_callback = Option::<
    unsafe extern "C" fn(*mut CURL, *mut libc::c_void, *mut libc::c_void) -> CURLcode,
>;
pub type curl_proxytype = libc::c_uint;
pub const CURLPROXY_SOCKS5_HOSTNAME: curl_proxytype = 7;
pub const CURLPROXY_SOCKS4A: curl_proxytype = 6;
pub const CURLPROXY_SOCKS5: curl_proxytype = 5;
pub const CURLPROXY_SOCKS4: curl_proxytype = 4;
pub const CURLPROXY_HTTPS: curl_proxytype = 2;
pub const CURLPROXY_HTTP_1_0: curl_proxytype = 1;
pub const CURLPROXY_HTTP: curl_proxytype = 0;
pub type curl_TimeCond = libc::c_uint;
pub const CURL_TIMECOND_LAST: curl_TimeCond = 4;
pub const CURL_TIMECOND_LASTMOD: curl_TimeCond = 3;
pub const CURL_TIMECOND_IFUNMODSINCE: curl_TimeCond = 2;
pub const CURL_TIMECOND_IFMODSINCE: curl_TimeCond = 1;
pub const CURL_TIMECOND_NONE: curl_TimeCond = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct curl_mimepart {
    pub easy: *mut Curl_easy,
    pub parent: *mut curl_mime,
    pub nextpart: *mut curl_mimepart,
    pub kind: mimekind,
    pub flags: libc::c_uint,
    pub data: *mut libc::c_char,
    pub readfunc: curl_read_callback,
    pub seekfunc: curl_seek_callback,
    pub freefunc: curl_free_callback,
    pub arg: *mut libc::c_void,
    pub fp: *mut FILE,
    pub curlheaders: *mut curl_slist,
    pub userheaders: *mut curl_slist,
    pub mimetype: *mut libc::c_char,
    pub filename: *mut libc::c_char,
    pub name: *mut libc::c_char,
    pub datasize: curl_off_t,
    pub state: mime_state,
    pub encoder: *const mime_encoder,
    pub encstate: mime_encoder_state,
    pub lastreadstatus: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mime_encoder_state {
    pub pos: size_t,
    pub bufbeg: size_t,
    pub bufend: size_t,
    pub buf: [libc::c_char; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mime_encoder {
    pub name: *const libc::c_char,
    pub encodefunc: Option::<
        unsafe extern "C" fn(
            *mut libc::c_char,
            size_t,
            bool,
            *mut curl_mimepart,
        ) -> size_t,
    >,
    pub sizefunc: Option::<unsafe extern "C" fn(*mut curl_mimepart) -> curl_off_t>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mime_state {
    pub state: mimestate,
    pub ptr: *mut libc::c_void,
    pub offset: curl_off_t,
}
pub type mimestate = libc::c_uint;
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
pub type curl_free_callback = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type curl_seek_callback = Option::<
    unsafe extern "C" fn(*mut libc::c_void, curl_off_t, libc::c_int) -> libc::c_int,
>;
pub type mimekind = libc::c_uint;
pub const MIMEKIND_LAST: mimekind = 5;
pub const MIMEKIND_MULTIPART: mimekind = 4;
pub const MIMEKIND_CALLBACK: mimekind = 3;
pub const MIMEKIND_FILE: mimekind = 2;
pub const MIMEKIND_DATA: mimekind = 1;
pub const MIMEKIND_NONE: mimekind = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct curl_mime {
    pub easy: *mut Curl_easy,
    pub parent: *mut curl_mimepart,
    pub firstpart: *mut curl_mimepart,
    pub lastpart: *mut curl_mimepart,
    pub boundary: [libc::c_char; 41],
    pub state: mime_state,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct curl_httppost {
    pub next: *mut curl_httppost,
    pub name: *mut libc::c_char,
    pub namelength: libc::c_long,
    pub contents: *mut libc::c_char,
    pub contentslength: libc::c_long,
    pub buffer: *mut libc::c_char,
    pub bufferlength: libc::c_long,
    pub contenttype: *mut libc::c_char,
    pub contentheader: *mut curl_slist,
    pub more: *mut curl_httppost,
    pub flags: libc::c_long,
    pub showfilename: *mut libc::c_char,
    pub userp: *mut libc::c_void,
    pub contentlen: curl_off_t,
}
pub type curl_hstswrite_callback = Option::<
    unsafe extern "C" fn(
        *mut CURL,
        *mut curl_hstsentry,
        *mut curl_index,
        *mut libc::c_void,
    ) -> CURLSTScode,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct curl_index {
    pub index: size_t,
    pub total: size_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct curl_hstsentry {
    pub name: *mut libc::c_char,
    pub namelen: size_t,
    #[bitfield(name = "includeSubDomains", ty = "libc::c_uint", bits = "0..=0")]
    pub includeSubDomains: [u8; 1],
    pub expire: [libc::c_char; 18],
}
pub type CURLSTScode = libc::c_uint;
pub const CURLSTS_FAIL: CURLSTScode = 2;
pub const CURLSTS_DONE: CURLSTScode = 1;
pub const CURLSTS_OK: CURLSTScode = 0;
pub type curl_hstsread_callback = Option::<
    unsafe extern "C" fn(
        *mut CURL,
        *mut curl_hstsentry,
        *mut libc::c_void,
    ) -> CURLSTScode,
>;
pub type curl_conv_callback = Option::<
    unsafe extern "C" fn(*mut libc::c_char, size_t) -> CURLcode,
>;
pub type curl_closesocket_callback = Option::<
    unsafe extern "C" fn(*mut libc::c_void, curl_socket_t) -> libc::c_int,
>;
pub type curl_socket_t = libc::c_int;
pub type curl_opensocket_callback = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        curlsocktype,
        *mut curl_sockaddr,
    ) -> curl_socket_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct curl_sockaddr {
    pub family: libc::c_int,
    pub socktype: libc::c_int,
    pub protocol: libc::c_int,
    pub addrlen: libc::c_uint,
    pub addr: sockaddr,
}
pub type curlsocktype = libc::c_uint;
pub const CURLSOCKTYPE_LAST: curlsocktype = 2;
pub const CURLSOCKTYPE_ACCEPT: curlsocktype = 1;
pub const CURLSOCKTYPE_IPCXN: curlsocktype = 0;
pub type curl_sockopt_callback = Option::<
    unsafe extern "C" fn(*mut libc::c_void, curl_socket_t, curlsocktype) -> libc::c_int,
>;
pub type curl_ioctl_callback = Option::<
    unsafe extern "C" fn(*mut CURL, libc::c_int, *mut libc::c_void) -> curlioerr,
>;
pub type curlioerr = libc::c_uint;
pub const CURLIOE_LAST: curlioerr = 3;
pub const CURLIOE_FAILRESTART: curlioerr = 2;
pub const CURLIOE_UNKNOWNCMD: curlioerr = 1;
pub const CURLIOE_OK: curlioerr = 0;
pub type curl_debug_callback = Option::<
    unsafe extern "C" fn(
        *mut CURL,
        curl_infotype,
        *mut libc::c_char,
        size_t,
        *mut libc::c_void,
    ) -> libc::c_int,
>;
pub type curl_infotype = libc::c_uint;
pub const CURLINFO_END: curl_infotype = 7;
pub const CURLINFO_SSL_DATA_OUT: curl_infotype = 6;
pub const CURLINFO_SSL_DATA_IN: curl_infotype = 5;
pub const CURLINFO_DATA_OUT: curl_infotype = 4;
pub const CURLINFO_DATA_IN: curl_infotype = 3;
pub const CURLINFO_HEADER_OUT: curl_infotype = 2;
pub const CURLINFO_HEADER_IN: curl_infotype = 1;
pub const CURLINFO_TEXT: curl_infotype = 0;
pub type curl_xferinfo_callback = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        curl_off_t,
        curl_off_t,
        curl_off_t,
        curl_off_t,
    ) -> libc::c_int,
>;
pub type curl_progress_callback = Option::<
    unsafe extern "C" fn(
        *mut libc::c_void,
        libc::c_double,
        libc::c_double,
        libc::c_double,
        libc::c_double,
    ) -> libc::c_int,
>;
pub type curl_write_callback = Option::<
    unsafe extern "C" fn(*mut libc::c_char, size_t, size_t, *mut libc::c_void) -> size_t,
>;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct SingleRequest {
    pub size: curl_off_t,
    pub maxdownload: curl_off_t,
    pub bytecount: curl_off_t,
    pub writebytecount: curl_off_t,
    pub headerbytecount: curl_off_t,
    pub deductheadercount: curl_off_t,
    pub pendingheader: curl_off_t,
    pub start: curltime,
    pub now: curltime,
    pub badheader: C2RustUnnamed_1,
    pub headerline: libc::c_int,
    pub str_0: *mut libc::c_char,
    pub offset: curl_off_t,
    pub httpcode: libc::c_int,
    pub keepon: libc::c_int,
    pub start100: curltime,
    pub exp100: expect100,
    pub upgr101: upgrade101,
    pub writer_stack: *mut contenc_writer,
    pub timeofdoc: time_t,
    pub bodywrites: libc::c_long,
    pub location: *mut libc::c_char,
    pub newurl: *mut libc::c_char,
    pub upload_present: ssize_t,
    pub upload_fromhere: *mut libc::c_char,
    pub p: C2RustUnnamed,
    pub doh: *mut dohdata,
    #[bitfield(name = "header", ty = "bit", bits = "0..=0")]
    #[bitfield(name = "content_range", ty = "bit", bits = "1..=1")]
    #[bitfield(name = "upload_done", ty = "bit", bits = "2..=2")]
    #[bitfield(name = "ignorebody", ty = "bit", bits = "3..=3")]
    #[bitfield(name = "http_bodyless", ty = "bit", bits = "4..=4")]
    #[bitfield(name = "chunk", ty = "bit", bits = "5..=5")]
    #[bitfield(name = "ignore_cl", ty = "bit", bits = "6..=6")]
    #[bitfield(name = "upload_chunky", ty = "bit", bits = "7..=7")]
    #[bitfield(name = "getheader", ty = "bit", bits = "8..=8")]
    #[bitfield(name = "forbidchunk", ty = "bit", bits = "9..=9")]
    pub header_content_range_upload_done_ignorebody_http_bodyless_chunk_ignore_cl_upload_chunky_getheader_forbidchunk: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dohdata {
    pub headers: *mut curl_slist,
    pub probe: [dnsprobe; 2],
    pub pending: libc::c_uint,
    pub port: libc::c_int,
    pub host: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dnsprobe {
    pub easy: *mut CURL,
    pub dnstype: libc::c_int,
    pub dohbuffer: [libc::c_uchar; 512],
    pub dohlen: size_t,
    pub serverdoh: dynbuf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub file: *mut FILEPROTO,
    pub ftp: *mut FTP,
    pub http: *mut HTTP,
    pub imap: *mut IMAP,
    pub ldap: *mut ldapreqinfo,
    pub mqtt: *mut MQTT,
    pub pop3: *mut POP3,
    pub rtsp: *mut RTSP,
    pub smb: *mut smb_request,
    pub smtp: *mut SMTP,
    pub ssh: *mut SSHPROTO,
    pub telnet: *mut TELNET,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SSHPROTO {
    pub path: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SMTP {
    pub transfer: curl_pp_transfer,
    pub custom: *mut libc::c_char,
    pub rcpt: *mut curl_slist,
    pub rcpt_had_ok: bool,
    pub trailing_crlf: bool,
    pub rcpt_last_error: libc::c_int,
    pub eob: size_t,
}
pub type curl_pp_transfer = libc::c_uint;
pub const PPTRANSFER_NONE: curl_pp_transfer = 2;
pub const PPTRANSFER_INFO: curl_pp_transfer = 1;
pub const PPTRANSFER_BODY: curl_pp_transfer = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RTSP {
    pub http_wrapper: HTTP,
    pub CSeq_sent: libc::c_long,
    pub CSeq_recv: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HTTP {
    pub sendit: *mut curl_mimepart,
    pub postsize: curl_off_t,
    pub postdata: *const libc::c_char,
    pub p_pragma: *const libc::c_char,
    pub form: curl_mimepart,
    pub backup: back,
    pub sending: C2RustUnnamed_0,
    pub send_buffer: dynbuf,
    pub stream_id: int32_t,
    pub bodystarted: bool,
    pub header_recvbuf: dynbuf,
    pub nread_header_recvbuf: size_t,
    pub trailer_recvbuf: dynbuf,
    pub status_code: libc::c_int,
    pub pausedata: *const uint8_t,
    pub pauselen: size_t,
    pub close_handled: bool,
    pub push_headers: *mut *mut libc::c_char,
    pub push_headers_used: size_t,
    pub push_headers_alloc: size_t,
    pub error: uint32_t,
    pub closed: bool,
    pub mem: *mut libc::c_char,
    pub len: size_t,
    pub memlen: size_t,
    pub upload_mem: *const uint8_t,
    pub upload_len: size_t,
    pub upload_left: curl_off_t,
}
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const HTTPSEND_BODY: C2RustUnnamed_0 = 2;
pub const HTTPSEND_REQUEST: C2RustUnnamed_0 = 1;
pub const HTTPSEND_NADA: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct back {
    pub fread_func: curl_read_callback,
    pub fread_in: *mut libc::c_void,
    pub postdata: *const libc::c_char,
    pub postsize: curl_off_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct POP3 {
    pub transfer: curl_pp_transfer,
    pub id: *mut libc::c_char,
    pub custom: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MQTT {
    pub sendleftovers: *mut libc::c_char,
    pub nsend: size_t,
    pub npacket: size_t,
    pub firstbyte: libc::c_uchar,
    pub remaining_length: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IMAP {
    pub transfer: curl_pp_transfer,
    pub mailbox: *mut libc::c_char,
    pub uidvalidity: *mut libc::c_char,
    pub uid: *mut libc::c_char,
    pub mindex: *mut libc::c_char,
    pub section: *mut libc::c_char,
    pub partial: *mut libc::c_char,
    pub query: *mut libc::c_char,
    pub custom: *mut libc::c_char,
    pub custom_params: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FTP {
    pub path: *mut libc::c_char,
    pub pathalloc: *mut libc::c_char,
    pub transfer: curl_pp_transfer,
    pub downloadsize: curl_off_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FILEPROTO {
    pub path: *mut libc::c_char,
    pub freepath: *mut libc::c_char,
    pub fd: libc::c_int,
}
pub type upgrade101 = libc::c_uint;
pub const UPGR101_WORKING: upgrade101 = 3;
pub const UPGR101_RECEIVED: upgrade101 = 2;
pub const UPGR101_REQUESTED: upgrade101 = 1;
pub const UPGR101_INIT: upgrade101 = 0;
pub type expect100 = libc::c_uint;
pub const EXP100_FAILED: expect100 = 3;
pub const EXP100_SENDING_REQUEST: expect100 = 2;
pub const EXP100_AWAITING_CONTINUE: expect100 = 1;
pub const EXP100_SEND_DATA: expect100 = 0;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const HEADER_ALLBAD: C2RustUnnamed_1 = 2;
pub const HEADER_PARTHEADER: C2RustUnnamed_1 = 1;
pub const HEADER_NORMAL: C2RustUnnamed_1 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PslCache {
    pub psl: *const psl_ctx_t,
    pub expires: time_t,
    pub dynamic: bool,
}
pub type psl_ctx_t = psl_ctx_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Curl_multi {
    pub magic: libc::c_uint,
    pub easyp: *mut Curl_easy,
    pub easylp: *mut Curl_easy,
    pub num_easy: libc::c_int,
    pub num_alive: libc::c_int,
    pub msglist: Curl_llist,
    pub pending: Curl_llist,
    pub socket_cb: curl_socket_callback,
    pub socket_userp: *mut libc::c_void,
    pub push_cb: curl_push_callback,
    pub push_userp: *mut libc::c_void,
    pub hostcache: Curl_hash,
    pub psl: PslCache,
    pub timetree: *mut Curl_tree,
    pub sockhash: Curl_hash,
    pub conn_cache: conncache,
    pub maxconnects: libc::c_long,
    pub max_host_connections: libc::c_long,
    pub max_total_connections: libc::c_long,
    pub timer_cb: curl_multi_timer_callback,
    pub timer_userp: *mut libc::c_void,
    pub timer_lastcall: curltime,
    pub max_concurrent_streams: libc::c_uint,
    pub wakeup_pair: [curl_socket_t; 2],
    pub multiplexing: bool,
    pub recheckstate: bool,
    pub in_callback: bool,
    pub ipv6_works: bool,
    pub ssl_seeded: bool,
}
pub type curl_multi_timer_callback = Option::<
    unsafe extern "C" fn(*mut CURLM, libc::c_long, *mut libc::c_void) -> libc::c_int,
>;
pub type CURLM = Curl_multi;
pub type curl_push_callback = Option::<
    unsafe extern "C" fn(
        *mut CURL,
        *mut CURL,
        size_t,
        *mut curl_pushheaders,
        *mut libc::c_void,
    ) -> libc::c_int,
>;
pub type curl_socket_callback = Option::<
    unsafe extern "C" fn(
        *mut CURL,
        curl_socket_t,
        libc::c_int,
        *mut libc::c_void,
        *mut libc::c_void,
    ) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Names {
    pub hostcache: *mut Curl_hash,
    pub hostcachetype: C2RustUnnamed_2,
}
pub type C2RustUnnamed_2 = libc::c_uint;
pub const HCACHE_SHARED: C2RustUnnamed_2 = 2;
pub const HCACHE_MULTI: C2RustUnnamed_2 = 1;
pub const HCACHE_NONE: C2RustUnnamed_2 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Curl_message {
    pub list: Curl_llist_element,
    pub extmsg: CURLMsg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CURLMsg {
    pub msg: CURLMSG,
    pub easy_handle: *mut CURL,
    pub data: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub whatever: *mut libc::c_void,
    pub result: CURLcode,
}
pub type CURLMSG = libc::c_uint;
pub const CURLMSG_LAST: CURLMSG = 2;
pub const CURLMSG_DONE: CURLMSG = 1;
pub const CURLMSG_NONE: CURLMSG = 0;
pub type CURLMstate = libc::c_uint;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct connectdata {
    pub cnnct: connstate,
    pub bundle_node: Curl_llist_element,
    pub chunk: Curl_chunker,
    pub fclosesocket: curl_closesocket_callback,
    pub closesocket_client: *mut libc::c_void,
    pub connection_id: libc::c_long,
    pub dns_entry: *mut Curl_dns_entry,
    pub ip_addr: *mut Curl_addrinfo,
    pub tempaddr: [*mut Curl_addrinfo; 2],
    pub scope_id: libc::c_uint,
    pub transport: C2RustUnnamed_5,
    pub host: hostname,
    pub hostname_resolve: *mut libc::c_char,
    pub secondaryhostname: *mut libc::c_char,
    pub conn_to_host: hostname,
    pub socks_proxy: proxy_info,
    pub http_proxy: proxy_info,
    pub port: libc::c_int,
    pub remote_port: libc::c_int,
    pub conn_to_port: libc::c_int,
    pub secondary_port: libc::c_ushort,
    pub primary_ip: [libc::c_char; 46],
    pub ip_version: libc::c_uchar,
    pub user: *mut libc::c_char,
    pub passwd: *mut libc::c_char,
    pub options: *mut libc::c_char,
    pub sasl_authzid: *mut libc::c_char,
    pub httpversion: libc::c_uchar,
    pub now: curltime,
    pub created: curltime,
    pub lastused: curltime,
    pub sock: [curl_socket_t; 2],
    pub tempsock: [curl_socket_t; 2],
    pub tempfamily: [libc::c_int; 2],
    pub recv: [Option::<Curl_recv>; 2],
    pub send: [Option::<Curl_send>; 2],
    pub ssl: [ssl_connect_data; 2],
    pub proxy_ssl: [ssl_connect_data; 2],
    pub ssl_extra: *mut libc::c_void,
    pub ssl_config: ssl_primary_config,
    pub proxy_ssl_config: ssl_primary_config,
    pub bits: ConnectBits,
    pub num_addr: libc::c_int,
    pub connecttime: curltime,
    pub timeoutms_per_addr: [timediff_t; 2],
    pub handler: *const Curl_handler,
    pub given: *const Curl_handler,
    pub keepalive: curltime,
    pub sockfd: curl_socket_t,
    pub writesockfd: curl_socket_t,
    pub easyq: Curl_llist,
    pub seek_func: curl_seek_callback,
    pub seek_client: *mut libc::c_void,
    pub gsasl: gsasldata,
    pub http_ntlm_state: curlntlm,
    pub proxy_ntlm_state: curlntlm,
    pub ntlm: ntlmdata,
    pub proxyntlm: ntlmdata,
    pub trailer: dynbuf,
    pub proto: C2RustUnnamed_4,
    pub connect_state: *mut http_connect_state,
    pub bundle: *mut connectbundle,
    pub unix_domain_socket: *mut libc::c_char,
    pub localdev: *mut libc::c_char,
    pub localportrange: libc::c_int,
    pub cselect_bits: libc::c_int,
    pub waitfor: libc::c_int,
    pub negnpn: libc::c_int,
    pub localport: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct connectbundle {
    pub multiuse: libc::c_int,
    pub num_connections: size_t,
    pub conn_list: Curl_llist,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub ftpc: ftp_conn,
    pub httpc: http_conn,
    pub sshc: ssh_conn,
    pub tftpc: *mut tftp_state_data,
    pub imapc: imap_conn,
    pub pop3c: pop3_conn,
    pub smtpc: smtp_conn,
    pub rtspc: rtsp_conn,
    pub smbc: smb_conn,
    pub rtmp: *mut libc::c_void,
    pub ldapc: *mut ldapconninfo,
    pub mqtt: mqtt_conn,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mqtt_conn {
    pub state: mqttstate,
    pub nextstate: mqttstate,
    pub packetid: libc::c_uint,
}
pub type mqttstate = libc::c_uint;
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
pub struct smb_conn {
    pub state: smb_conn_state,
    pub user: *mut libc::c_char,
    pub domain: *mut libc::c_char,
    pub share: *mut libc::c_char,
    pub challenge: [libc::c_uchar; 8],
    pub session_key: libc::c_uint,
    pub uid: libc::c_ushort,
    pub recv_buf: *mut libc::c_char,
    pub upload_size: size_t,
    pub send_size: size_t,
    pub sent: size_t,
    pub got: size_t,
}
pub type smb_conn_state = libc::c_uint;
pub const SMB_CONNECTED: smb_conn_state = 4;
pub const SMB_SETUP: smb_conn_state = 3;
pub const SMB_NEGOTIATE: smb_conn_state = 2;
pub const SMB_CONNECTING: smb_conn_state = 1;
pub const SMB_NOT_CONNECTED: smb_conn_state = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rtsp_conn {
    pub rtp_buf: *mut libc::c_char,
    pub rtp_bufsize: ssize_t,
    pub rtp_channel: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct smtp_conn {
    pub pp: pingpong,
    pub state: smtpstate,
    pub ssldone: bool,
    pub domain: *mut libc::c_char,
    pub sasl: SASL,
    pub tls_supported: bool,
    pub size_supported: bool,
    pub utf8_supported: bool,
    pub auth_supported: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SASL {
    pub params: *const SASLproto,
    pub state: saslstate,
    pub authmechs: libc::c_ushort,
    pub prefmech: libc::c_ushort,
    pub authused: libc::c_ushort,
    pub resetprefs: bool,
    pub mutual_auth: bool,
    pub force_ir: bool,
}
pub type saslstate = libc::c_uint;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SASLproto {
    pub service: *const libc::c_char,
    pub contcode: libc::c_int,
    pub finalcode: libc::c_int,
    pub maxirlen: size_t,
    pub sendauth: Option::<
        unsafe extern "C" fn(
            *mut Curl_easy,
            *mut connectdata,
            *const libc::c_char,
            *const libc::c_char,
        ) -> CURLcode,
    >,
    pub sendcont: Option::<
        unsafe extern "C" fn(
            *mut Curl_easy,
            *mut connectdata,
            *const libc::c_char,
        ) -> CURLcode,
    >,
    pub getmessage: Option::<
        unsafe extern "C" fn(*mut libc::c_char, *mut *mut libc::c_char) -> (),
    >,
}
pub type smtpstate = libc::c_uint;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pingpong {
    pub cache: *mut libc::c_char,
    pub cache_size: size_t,
    pub nread_resp: size_t,
    pub linestart_resp: *mut libc::c_char,
    pub pending_resp: bool,
    pub sendthis: *mut libc::c_char,
    pub sendleft: size_t,
    pub sendsize: size_t,
    pub response: curltime,
    pub response_time: timediff_t,
    pub sendbuf: dynbuf,
    pub statemachine: Option::<
        unsafe extern "C" fn(*mut Curl_easy, *mut connectdata) -> CURLcode,
    >,
    pub endofresp: Option::<
        unsafe extern "C" fn(
            *mut Curl_easy,
            *mut connectdata,
            *mut libc::c_char,
            size_t,
            *mut libc::c_int,
        ) -> bool,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pop3_conn {
    pub pp: pingpong,
    pub state: pop3state,
    pub ssldone: bool,
    pub tls_supported: bool,
    pub eob: size_t,
    pub strip: size_t,
    pub sasl: SASL,
    pub authtypes: libc::c_uint,
    pub preftype: libc::c_uint,
    pub apoptimestamp: *mut libc::c_char,
}
pub type pop3state = libc::c_uint;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct imap_conn {
    pub pp: pingpong,
    pub state: imapstate,
    pub ssldone: bool,
    pub preauth: bool,
    pub sasl: SASL,
    pub preftype: libc::c_uint,
    pub cmdid: libc::c_uint,
    pub resptag: [libc::c_char; 5],
    pub tls_supported: bool,
    pub login_disabled: bool,
    pub ir_supported: bool,
    pub mailbox: *mut libc::c_char,
    pub mailbox_uidvalidity: *mut libc::c_char,
    pub dyn_0: dynbuf,
}
pub type imapstate = libc::c_uint;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ssh_conn {
    pub authlist: *const libc::c_char,
    pub passphrase: *const libc::c_char,
    pub rsa_pub: *mut libc::c_char,
    pub rsa: *mut libc::c_char,
    pub authed: bool,
    pub acceptfail: bool,
    pub state: sshstate,
    pub nextstate: sshstate,
    pub actualcode: CURLcode,
    pub quote_item: *mut curl_slist,
    pub quote_path1: *mut libc::c_char,
    pub quote_path2: *mut libc::c_char,
    pub homedir: *mut libc::c_char,
    pub readdir_line: *mut libc::c_char,
    pub secondCreateDirs: libc::c_int,
    pub orig_waitfor: libc::c_int,
    pub slash_pos: *mut libc::c_char,
}
pub type sshstate = libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct http_conn {
    pub binsettings: [uint8_t; 80],
    pub binlen: size_t,
    pub trnsfr: *mut Curl_easy,
    pub h2: *mut nghttp2_session,
    pub send_underlying: Option::<Curl_send>,
    pub recv_underlying: Option::<Curl_recv>,
    pub inbuf: *mut libc::c_char,
    pub inbuflen: size_t,
    pub nread_inbuf: size_t,
    pub pause_stream_id: int32_t,
    pub drain_total: size_t,
    pub settings: h2settings,
    pub local_settings: [nghttp2_settings_entry; 3],
    pub local_settings_num: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nghttp2_settings_entry {
    pub settings_id: int32_t,
    pub value: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct h2settings {
    pub max_concurrent_streams: uint32_t,
    pub enable_push: bool,
}
pub type Curl_recv = unsafe extern "C" fn(
    *mut Curl_easy,
    libc::c_int,
    *mut libc::c_char,
    size_t,
    *mut CURLcode,
) -> ssize_t;
pub type Curl_send = unsafe extern "C" fn(
    *mut Curl_easy,
    libc::c_int,
    *const libc::c_void,
    size_t,
    *mut CURLcode,
) -> ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ftp_conn {
    pub pp: pingpong,
    pub entrypath: *mut libc::c_char,
    pub file: *mut libc::c_char,
    pub dirs: *mut *mut libc::c_char,
    pub dirdepth: libc::c_int,
    pub dont_check: bool,
    pub ctl_valid: bool,
    pub cwddone: bool,
    pub cwdcount: libc::c_int,
    pub cwdfail: bool,
    pub wait_data_conn: bool,
    pub newport: libc::c_ushort,
    pub newhost: *mut libc::c_char,
    pub prevpath: *mut libc::c_char,
    pub transfertype: libc::c_char,
    pub count1: libc::c_int,
    pub count2: libc::c_int,
    pub count3: libc::c_int,
    pub state: ftpstate,
    pub state_saved: ftpstate,
    pub retr_size_saved: curl_off_t,
    pub server_os: *mut libc::c_char,
    pub known_filesize: curl_off_t,
}
pub type ftpstate = libc::c_uint;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ntlmdata {
    pub flags: libc::c_uint,
    pub nonce: [libc::c_uchar; 8],
    pub target_info_len: libc::c_uint,
    pub target_info: *mut libc::c_void,
    pub ntlm_auth_hlpr_socket: curl_socket_t,
    pub ntlm_auth_hlpr_pid: pid_t,
    pub challenge: *mut libc::c_char,
    pub response: *mut libc::c_char,
}
pub type curlntlm = libc::c_uint;
pub const NTLMSTATE_LAST: curlntlm = 4;
pub const NTLMSTATE_TYPE3: curlntlm = 3;
pub const NTLMSTATE_TYPE2: curlntlm = 2;
pub const NTLMSTATE_TYPE1: curlntlm = 1;
pub const NTLMSTATE_NONE: curlntlm = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsasldata {
    pub ctx: *mut Gsasl,
    pub client: *mut Gsasl_session,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Curl_handler {
    pub scheme: *const libc::c_char,
    pub setup_connection: Option::<
        unsafe extern "C" fn(*mut Curl_easy, *mut connectdata) -> CURLcode,
    >,
    pub do_it: Option::<unsafe extern "C" fn(*mut Curl_easy, *mut bool) -> CURLcode>,
    pub done: Option::<unsafe extern "C" fn(*mut Curl_easy, CURLcode, bool) -> CURLcode>,
    pub do_more: Option::<
        unsafe extern "C" fn(*mut Curl_easy, *mut libc::c_int) -> CURLcode,
    >,
    pub connect_it: Option::<
        unsafe extern "C" fn(*mut Curl_easy, *mut bool) -> CURLcode,
    >,
    pub connecting: Option::<
        unsafe extern "C" fn(*mut Curl_easy, *mut bool) -> CURLcode,
    >,
    pub doing: Option::<unsafe extern "C" fn(*mut Curl_easy, *mut bool) -> CURLcode>,
    pub proto_getsock: Option::<
        unsafe extern "C" fn(
            *mut Curl_easy,
            *mut connectdata,
            *mut curl_socket_t,
        ) -> libc::c_int,
    >,
    pub doing_getsock: Option::<
        unsafe extern "C" fn(
            *mut Curl_easy,
            *mut connectdata,
            *mut curl_socket_t,
        ) -> libc::c_int,
    >,
    pub domore_getsock: Option::<
        unsafe extern "C" fn(
            *mut Curl_easy,
            *mut connectdata,
            *mut curl_socket_t,
        ) -> libc::c_int,
    >,
    pub perform_getsock: Option::<
        unsafe extern "C" fn(
            *mut Curl_easy,
            *mut connectdata,
            *mut curl_socket_t,
        ) -> libc::c_int,
    >,
    pub disconnect: Option::<
        unsafe extern "C" fn(*mut Curl_easy, *mut connectdata, bool) -> CURLcode,
    >,
    pub readwrite: Option::<
        unsafe extern "C" fn(
            *mut Curl_easy,
            *mut connectdata,
            *mut ssize_t,
            *mut bool,
        ) -> CURLcode,
    >,
    pub connection_check: Option::<
        unsafe extern "C" fn(
            *mut Curl_easy,
            *mut connectdata,
            libc::c_uint,
        ) -> libc::c_uint,
    >,
    pub attach: Option::<unsafe extern "C" fn(*mut Curl_easy, *mut connectdata) -> ()>,
    pub defport: libc::c_int,
    pub protocol: libc::c_uint,
    pub family: libc::c_uint,
    pub flags: libc::c_uint,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct ConnectBits {
    pub tcpconnect: [bool; 2],
    pub proxy_ssl_connected: [bool; 2],
    #[bitfield(name = "httpproxy", ty = "bit", bits = "0..=0")]
    #[bitfield(name = "socksproxy", ty = "bit", bits = "1..=1")]
    #[bitfield(name = "proxy_user_passwd", ty = "bit", bits = "2..=2")]
    #[bitfield(name = "tunnel_proxy", ty = "bit", bits = "3..=3")]
    #[bitfield(name = "proxy_connect_closed", ty = "bit", bits = "4..=4")]
    #[bitfield(name = "close", ty = "bit", bits = "5..=5")]
    #[bitfield(name = "reuse", ty = "bit", bits = "6..=6")]
    #[bitfield(name = "altused", ty = "bit", bits = "7..=7")]
    #[bitfield(name = "conn_to_host", ty = "bit", bits = "8..=8")]
    #[bitfield(name = "conn_to_port", ty = "bit", bits = "9..=9")]
    #[bitfield(name = "proxy", ty = "bit", bits = "10..=10")]
    #[bitfield(name = "user_passwd", ty = "bit", bits = "11..=11")]
    #[bitfield(name = "ipv6_ip", ty = "bit", bits = "12..=12")]
    #[bitfield(name = "ipv6", ty = "bit", bits = "13..=13")]
    #[bitfield(name = "do_more", ty = "bit", bits = "14..=14")]
    #[bitfield(name = "protoconnstart", ty = "bit", bits = "15..=15")]
    #[bitfield(name = "retry", ty = "bit", bits = "16..=16")]
    #[bitfield(name = "authneg", ty = "bit", bits = "17..=17")]
    #[bitfield(name = "rewindaftersend", ty = "bit", bits = "18..=18")]
    #[bitfield(name = "ftp_use_epsv", ty = "bit", bits = "19..=19")]
    #[bitfield(name = "ftp_use_eprt", ty = "bit", bits = "20..=20")]
    #[bitfield(name = "ftp_use_data_ssl", ty = "bit", bits = "21..=21")]
    #[bitfield(name = "ftp_use_control_ssl", ty = "bit", bits = "22..=22")]
    #[bitfield(name = "netrc", ty = "bit", bits = "23..=23")]
    #[bitfield(name = "bound", ty = "bit", bits = "24..=24")]
    #[bitfield(name = "multiplex", ty = "bit", bits = "25..=25")]
    #[bitfield(name = "tcp_fastopen", ty = "bit", bits = "26..=26")]
    #[bitfield(name = "tls_enable_npn", ty = "bit", bits = "27..=27")]
    #[bitfield(name = "tls_enable_alpn", ty = "bit", bits = "28..=28")]
    #[bitfield(name = "connect_only", ty = "bit", bits = "29..=29")]
    #[bitfield(name = "doh", ty = "bit", bits = "30..=30")]
    #[bitfield(name = "abstract_unix_socket", ty = "bit", bits = "31..=31")]
    #[bitfield(name = "tls_upgraded", ty = "bit", bits = "32..=32")]
    #[bitfield(name = "sock_accepted", ty = "bit", bits = "33..=33")]
    #[bitfield(name = "parallel_connect", ty = "bit", bits = "34..=34")]
    pub httpproxy_socksproxy_proxy_user_passwd_tunnel_proxy_proxy_connect_closed_close_reuse_altused_conn_to_host_conn_to_port_proxy_user_passwd_ipv6_ip_ipv6_do_more_protoconnstart_retry_authneg_rewindaftersend_ftp_use_epsv_ftp_use_eprt_ftp_use_data_ssl_ftp_use_control_ssl_netrc_bound_multiplex_tcp_fastopen_tls_enable_npn_tls_enable_alpn_connect_only_doh_abstract_unix_socket_tls_upgraded_sock_accepted_parallel_connect: [u8; 5],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct ssl_connect_data {
    pub state: ssl_connection_state,
    pub connecting_state: ssl_connect_state,
    pub backend: *mut ssl_backend_data,
    #[bitfield(name = "use_0", ty = "bit", bits = "0..=0")]
    pub use_0: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type ssl_connect_state = libc::c_uint;
pub const ssl_connect_done: ssl_connect_state = 5;
pub const ssl_connect_3: ssl_connect_state = 4;
pub const ssl_connect_2_writing: ssl_connect_state = 3;
pub const ssl_connect_2_reading: ssl_connect_state = 2;
pub const ssl_connect_2: ssl_connect_state = 1;
pub const ssl_connect_1: ssl_connect_state = 0;
pub type ssl_connection_state = libc::c_uint;
pub const ssl_connection_complete: ssl_connection_state = 2;
pub const ssl_connection_negotiating: ssl_connection_state = 1;
pub const ssl_connection_none: ssl_connection_state = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct proxy_info {
    pub host: hostname,
    pub port: libc::c_long,
    pub proxytype: curl_proxytype,
    pub user: *mut libc::c_char,
    pub passwd: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hostname {
    pub rawalloc: *mut libc::c_char,
    pub encalloc: *mut libc::c_char,
    pub name: *mut libc::c_char,
    pub dispname: *const libc::c_char,
}
pub type C2RustUnnamed_5 = libc::c_uint;
pub const TRNSPRT_QUIC: C2RustUnnamed_5 = 5;
pub const TRNSPRT_UDP: C2RustUnnamed_5 = 4;
pub const TRNSPRT_TCP: C2RustUnnamed_5 = 3;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Curl_chunker {
    pub datasize: curl_off_t,
    pub state: ChunkyState,
    pub hexindex: libc::c_uchar,
    pub hexbuffer: [libc::c_char; 17],
}
pub type ChunkyState = libc::c_uint;
pub const CHUNK_TRAILER_POSTCR: ChunkyState = 7;
pub const CHUNK_TRAILER_CR: ChunkyState = 6;
pub const CHUNK_TRAILER: ChunkyState = 5;
pub const CHUNK_STOP: ChunkyState = 4;
pub const CHUNK_POSTLF: ChunkyState = 3;
pub const CHUNK_DATA: ChunkyState = 2;
pub const CHUNK_LF: ChunkyState = 1;
pub const CHUNK_HEX: ChunkyState = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct connstate {
    pub state: connect_t,
    pub outstanding: ssize_t,
    pub outp: *mut libc::c_uchar,
}
pub type connect_t = libc::c_uint;
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
pub type curl_malloc_callback = Option::<
    unsafe extern "C" fn(size_t) -> *mut libc::c_void,
>;
pub type curl_strdup_callback = Option::<
    unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_char,
>;
pub type curl_calloc_callback = Option::<
    unsafe extern "C" fn(size_t, size_t) -> *mut libc::c_void,
>;
pub type CURLUcode = libc::c_uint;
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
pub type CURLUPart = libc::c_uint;
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
pub type mimestrategy = libc::c_uint;
pub const MIMESTRATEGY_LAST: mimestrategy = 2;
pub const MIMESTRATEGY_FORM: mimestrategy = 1;
pub const MIMESTRATEGY_MAIL: mimestrategy = 0;
pub type saslprogress = libc::c_uint;
pub const SASL_DONE: saslprogress = 2;
pub const SASL_INPROGRESS: saslprogress = 1;
pub const SASL_IDLE: saslprogress = 0;
pub type CURLofft = libc::c_uint;
pub const CURL_OFFT_INVAL: CURLofft = 2;
pub const CURL_OFFT_FLOW: CURLofft = 1;
pub const CURL_OFFT_OK: CURLofft = 0;
pub type urlreject = libc::c_uint;
pub const REJECT_ZERO: urlreject = 4;
pub const REJECT_CTRL: urlreject = 3;
pub const REJECT_NADA: urlreject = 2;
pub const STRING_CUSTOMREQUEST: dupstring = 6;
pub type dupstring = libc::c_uint;
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
pub static mut Curl_handler_imap: Curl_handler = unsafe {
    {
        let mut init = Curl_handler {
            scheme: b"IMAP\0" as *const u8 as *const libc::c_char,
            setup_connection: Some(
                imap_setup_connection
                    as unsafe extern "C" fn(*mut Curl_easy, *mut connectdata) -> CURLcode,
            ),
            do_it: Some(
                imap_do as unsafe extern "C" fn(*mut Curl_easy, *mut bool) -> CURLcode,
            ),
            done: Some(
                imap_done
                    as unsafe extern "C" fn(*mut Curl_easy, CURLcode, bool) -> CURLcode,
            ),
            do_more: None,
            connect_it: Some(
                imap_connect
                    as unsafe extern "C" fn(*mut Curl_easy, *mut bool) -> CURLcode,
            ),
            connecting: Some(
                imap_multi_statemach
                    as unsafe extern "C" fn(*mut Curl_easy, *mut bool) -> CURLcode,
            ),
            doing: Some(
                imap_doing as unsafe extern "C" fn(*mut Curl_easy, *mut bool) -> CURLcode,
            ),
            proto_getsock: Some(
                imap_getsock
                    as unsafe extern "C" fn(
                        *mut Curl_easy,
                        *mut connectdata,
                        *mut curl_socket_t,
                    ) -> libc::c_int,
            ),
            doing_getsock: Some(
                imap_getsock
                    as unsafe extern "C" fn(
                        *mut Curl_easy,
                        *mut connectdata,
                        *mut curl_socket_t,
                    ) -> libc::c_int,
            ),
            domore_getsock: None,
            perform_getsock: None,
            disconnect: Some(
                imap_disconnect
                    as unsafe extern "C" fn(
                        *mut Curl_easy,
                        *mut connectdata,
                        bool,
                    ) -> CURLcode,
            ),
            readwrite: None,
            connection_check: None,
            attach: None,
            defport: 143 as libc::c_int,
            protocol: ((1 as libc::c_int) << 12 as libc::c_int) as libc::c_uint,
            family: ((1 as libc::c_int) << 12 as libc::c_int) as libc::c_uint,
            flags: ((1 as libc::c_int) << 2 as libc::c_int
                | (1 as libc::c_int) << 10 as libc::c_int) as libc::c_uint,
        };
        init
    }
};
#[no_mangle]
pub static mut Curl_handler_imaps: Curl_handler = unsafe {
    {
        let mut init = Curl_handler {
            scheme: b"IMAPS\0" as *const u8 as *const libc::c_char,
            setup_connection: Some(
                imap_setup_connection
                    as unsafe extern "C" fn(*mut Curl_easy, *mut connectdata) -> CURLcode,
            ),
            do_it: Some(
                imap_do as unsafe extern "C" fn(*mut Curl_easy, *mut bool) -> CURLcode,
            ),
            done: Some(
                imap_done
                    as unsafe extern "C" fn(*mut Curl_easy, CURLcode, bool) -> CURLcode,
            ),
            do_more: None,
            connect_it: Some(
                imap_connect
                    as unsafe extern "C" fn(*mut Curl_easy, *mut bool) -> CURLcode,
            ),
            connecting: Some(
                imap_multi_statemach
                    as unsafe extern "C" fn(*mut Curl_easy, *mut bool) -> CURLcode,
            ),
            doing: Some(
                imap_doing as unsafe extern "C" fn(*mut Curl_easy, *mut bool) -> CURLcode,
            ),
            proto_getsock: Some(
                imap_getsock
                    as unsafe extern "C" fn(
                        *mut Curl_easy,
                        *mut connectdata,
                        *mut curl_socket_t,
                    ) -> libc::c_int,
            ),
            doing_getsock: Some(
                imap_getsock
                    as unsafe extern "C" fn(
                        *mut Curl_easy,
                        *mut connectdata,
                        *mut curl_socket_t,
                    ) -> libc::c_int,
            ),
            domore_getsock: None,
            perform_getsock: None,
            disconnect: Some(
                imap_disconnect
                    as unsafe extern "C" fn(
                        *mut Curl_easy,
                        *mut connectdata,
                        bool,
                    ) -> CURLcode,
            ),
            readwrite: None,
            connection_check: None,
            attach: None,
            defport: 993 as libc::c_int,
            protocol: ((1 as libc::c_int) << 13 as libc::c_int) as libc::c_uint,
            family: ((1 as libc::c_int) << 12 as libc::c_int) as libc::c_uint,
            flags: ((1 as libc::c_int) << 2 as libc::c_int
                | (1 as libc::c_int) << 0 as libc::c_int
                | (1 as libc::c_int) << 10 as libc::c_int) as libc::c_uint,
        };
        init
    }
};
static mut saslimap: SASLproto = unsafe {
    {
        let mut init = SASLproto {
            service: b"imap\0" as *const u8 as *const libc::c_char,
            contcode: '+' as i32,
            finalcode: 1 as libc::c_int,
            maxirlen: 0 as libc::c_int as size_t,
            sendauth: Some(
                imap_perform_authenticate
                    as unsafe extern "C" fn(
                        *mut Curl_easy,
                        *mut connectdata,
                        *const libc::c_char,
                        *const libc::c_char,
                    ) -> CURLcode,
            ),
            sendcont: Some(
                imap_continue_authenticate
                    as unsafe extern "C" fn(
                        *mut Curl_easy,
                        *mut connectdata,
                        *const libc::c_char,
                    ) -> CURLcode,
            ),
            getmessage: Some(
                imap_get_message
                    as unsafe extern "C" fn(
                        *mut libc::c_char,
                        *mut *mut libc::c_char,
                    ) -> (),
            ),
        };
        init
    }
};
unsafe extern "C" fn imap_to_imaps(mut conn: *mut connectdata) {
    let ref mut fresh0 = (*conn).handler;
    *fresh0 = &Curl_handler_imaps;
    let ref mut fresh1 = (*conn).bits;
    (*fresh1).set_tls_upgraded(1 as libc::c_int as bit);
}
unsafe extern "C" fn imap_matchresp(
    mut line: *const libc::c_char,
    mut len: size_t,
    mut cmd: *const libc::c_char,
) -> bool {
    let mut end: *const libc::c_char = line.offset(len as isize);
    let mut cmd_len: size_t = strlen(cmd);
    line = line.offset(2 as libc::c_int as isize);
    if line < end && Curl_isdigit(*line as libc::c_uchar as libc::c_int) != 0 {
        loop {
            line = line.offset(1);
            if !(line < end && Curl_isdigit(*line as libc::c_uchar as libc::c_int) != 0)
            {
                break;
            }
        }
        if line == end || *line as libc::c_int != ' ' as i32 {
            return 0 as libc::c_int != 0;
        }
        line = line.offset(1);
    }
    if line.offset(cmd_len as isize) <= end
        && Curl_strncasecompare(line, cmd, cmd_len) != 0
        && (*line.offset(cmd_len as isize) as libc::c_int == ' ' as i32
            || line.offset(cmd_len as isize).offset(2 as libc::c_int as isize) == end)
    {
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn imap_endofresp(
    mut data: *mut Curl_easy,
    mut conn: *mut connectdata,
    mut line: *mut libc::c_char,
    mut len: size_t,
    mut resp: *mut libc::c_int,
) -> bool {
    let mut imap: *mut IMAP = (*data).req.p.imap;
    let mut imapc: *mut imap_conn = &mut (*conn).proto.imapc;
    let mut id: *const libc::c_char = ((*imapc).resptag).as_mut_ptr();
    let mut id_len: size_t = strlen(id);
    if len >= id_len.wrapping_add(1 as libc::c_int as libc::c_ulong)
        && memcmp(id as *const libc::c_void, line as *const libc::c_void, id_len) == 0
        && *line.offset(id_len as isize) as libc::c_int == ' ' as i32
    {
        line = line
            .offset(id_len.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
        len = (len as libc::c_ulong)
            .wrapping_sub(id_len.wrapping_add(1 as libc::c_int as libc::c_ulong))
            as size_t as size_t;
        if len >= 2 as libc::c_int as libc::c_ulong
            && memcmp(
                line as *const libc::c_void,
                b"OK\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                2 as libc::c_int as libc::c_ulong,
            ) == 0
        {
            *resp = 1 as libc::c_int;
        } else if len >= 7 as libc::c_int as libc::c_ulong
                && memcmp(
                    line as *const libc::c_void,
                    b"PREAUTH\0" as *const u8 as *const libc::c_char
                        as *const libc::c_void,
                    7 as libc::c_int as libc::c_ulong,
                ) == 0
            {
            *resp = 3 as libc::c_int;
        } else {
            *resp = 2 as libc::c_int;
        }
        return 1 as libc::c_int != 0;
    }
    if len >= 2 as libc::c_int as libc::c_ulong
        && memcmp(
            b"* \0" as *const u8 as *const libc::c_char as *const libc::c_void,
            line as *const libc::c_void,
            2 as libc::c_int as libc::c_ulong,
        ) == 0
    {
        match (*imapc).state as libc::c_uint {
            2 => {
                if !imap_matchresp(
                    line,
                    len,
                    b"CAPABILITY\0" as *const u8 as *const libc::c_char,
                ) {
                    return 0 as libc::c_int != 0;
                }
            }
            7 => {
                if ((*imap).custom).is_null()
                    && !imap_matchresp(
                        line,
                        len,
                        b"LIST\0" as *const u8 as *const libc::c_char,
                    )
                    || !((*imap).custom).is_null()
                        && !imap_matchresp(line, len, (*imap).custom)
                        && (Curl_strcasecompare(
                            (*imap).custom,
                            b"STORE\0" as *const u8 as *const libc::c_char,
                        ) == 0
                            || !imap_matchresp(
                                line,
                                len,
                                b"FETCH\0" as *const u8 as *const libc::c_char,
                            ))
                        && Curl_strcasecompare(
                            (*imap).custom,
                            b"SELECT\0" as *const u8 as *const libc::c_char,
                        ) == 0
                        && Curl_strcasecompare(
                            (*imap).custom,
                            b"EXAMINE\0" as *const u8 as *const libc::c_char,
                        ) == 0
                        && Curl_strcasecompare(
                            (*imap).custom,
                            b"SEARCH\0" as *const u8 as *const libc::c_char,
                        ) == 0
                        && Curl_strcasecompare(
                            (*imap).custom,
                            b"EXPUNGE\0" as *const u8 as *const libc::c_char,
                        ) == 0
                        && Curl_strcasecompare(
                            (*imap).custom,
                            b"LSUB\0" as *const u8 as *const libc::c_char,
                        ) == 0
                        && Curl_strcasecompare(
                            (*imap).custom,
                            b"UID\0" as *const u8 as *const libc::c_char,
                        ) == 0
                        && Curl_strcasecompare(
                            (*imap).custom,
                            b"NOOP\0" as *const u8 as *const libc::c_char,
                        ) == 0
                {
                    return 0 as libc::c_int != 0;
                }
            }
            8 => {}
            9 => {
                if !imap_matchresp(
                    line,
                    len,
                    b"FETCH\0" as *const u8 as *const libc::c_char,
                ) {
                    return 0 as libc::c_int != 0;
                }
            }
            13 => {
                if !imap_matchresp(
                    line,
                    len,
                    b"SEARCH\0" as *const u8 as *const libc::c_char,
                ) {
                    return 0 as libc::c_int != 0;
                }
            }
            _ => return 0 as libc::c_int != 0,
        }
        *resp = '*' as i32;
        return 1 as libc::c_int != 0;
    }
    if !imap.is_null() && ((*imap).custom).is_null()
        && (len == 3 as libc::c_int as libc::c_ulong
            && *line.offset(0 as libc::c_int as isize) as libc::c_int == '+' as i32
            || len >= 2 as libc::c_int as libc::c_ulong
                && memcmp(
                    b"+ \0" as *const u8 as *const libc::c_char as *const libc::c_void,
                    line as *const libc::c_void,
                    2 as libc::c_int as libc::c_ulong,
                ) == 0)
    {
        match (*imapc).state as libc::c_uint {
            5 | 11 => {
                *resp = '+' as i32;
            }
            _ => {
                Curl_failf(
                    data,
                    b"Unexpected continuation response\0" as *const u8
                        as *const libc::c_char,
                );
                *resp = -(1 as libc::c_int);
            }
        }
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn imap_get_message(
    mut buffer: *mut libc::c_char,
    mut outptr: *mut *mut libc::c_char,
) {
    let mut len: size_t = strlen(buffer);
    let mut message: *mut libc::c_char = 0 as *mut libc::c_char;
    if len > 2 as libc::c_int as libc::c_ulong {
        len = (len as libc::c_ulong).wrapping_sub(2 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
        message = buffer.offset(2 as libc::c_int as isize);
        while *message as libc::c_int == ' ' as i32
            || *message as libc::c_int == '\t' as i32
        {
            message = message.offset(1);
            len = len.wrapping_sub(1);
        }
        loop {
            let fresh2 = len;
            len = len.wrapping_sub(1);
            if !(fresh2 != 0) {
                break;
            }
            if *message.offset(len as isize) as libc::c_int != '\r' as i32
                && *message.offset(len as isize) as libc::c_int != '\n' as i32
                && *message.offset(len as isize) as libc::c_int != ' ' as i32
                && *message.offset(len as isize) as libc::c_int != '\t' as i32
            {
                break;
            }
        }
        len = len.wrapping_add(1);
        if len != 0 {
            *message.offset(len as isize) = '\u{0}' as i32 as libc::c_char;
        }
    } else {
        message = &mut *buffer.offset(len as isize) as *mut libc::c_char;
    }
    *outptr = message;
}
unsafe extern "C" fn state(mut data: *mut Curl_easy, mut newstate: imapstate) {
    let mut imapc: *mut imap_conn = &mut (*(*data).conn).proto.imapc;
    (*imapc).state = newstate;
}
unsafe extern "C" fn imap_perform_capability(
    mut data: *mut Curl_easy,
    mut conn: *mut connectdata,
) -> CURLcode {
    let mut result: CURLcode = CURLE_OK;
    let mut imapc: *mut imap_conn = &mut (*conn).proto.imapc;
    (*imapc).sasl.authmechs = 0 as libc::c_int as libc::c_ushort;
    (*imapc).sasl.authused = 0 as libc::c_int as libc::c_ushort;
    (*imapc).tls_supported = 0 as libc::c_int != 0;
    result = imap_sendf(data, conn, b"CAPABILITY\0" as *const u8 as *const libc::c_char);
    if result as u64 == 0 {
        state(data, IMAP_CAPABILITY);
    }
    return result;
}
unsafe extern "C" fn imap_perform_starttls(
    mut data: *mut Curl_easy,
    mut conn: *mut connectdata,
) -> CURLcode {
    let mut result: CURLcode = imap_sendf(
        data,
        conn,
        b"STARTTLS\0" as *const u8 as *const libc::c_char,
    );
    if result as u64 == 0 {
        state(data, IMAP_STARTTLS);
    }
    return result;
}
unsafe extern "C" fn imap_perform_upgrade_tls(
    mut data: *mut Curl_easy,
    mut conn: *mut connectdata,
) -> CURLcode {
    let mut imapc: *mut imap_conn = &mut (*conn).proto.imapc;
    let mut result: CURLcode = Curl_ssl_connect_nonblocking(
        data,
        conn,
        0 as libc::c_int != 0,
        0 as libc::c_int,
        &mut (*imapc).ssldone,
    );
    if result as u64 == 0 {
        if (*imapc).state as libc::c_uint
            != IMAP_UPGRADETLS as libc::c_int as libc::c_uint
        {
            state(data, IMAP_UPGRADETLS);
        }
        if (*imapc).ssldone {
            imap_to_imaps(conn);
            result = imap_perform_capability(data, conn);
        }
    }
    return result;
}
unsafe extern "C" fn imap_perform_login(
    mut data: *mut Curl_easy,
    mut conn: *mut connectdata,
) -> CURLcode {
    let mut result: CURLcode = CURLE_OK;
    let mut user: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut passwd: *mut libc::c_char = 0 as *mut libc::c_char;
    if ((*conn).bits).user_passwd() == 0 {
        state(data, IMAP_STOP);
        return result;
    }
    user = imap_atom((*conn).user, 0 as libc::c_int != 0);
    passwd = imap_atom((*conn).passwd, 0 as libc::c_int != 0);
    result = imap_sendf(
        data,
        conn,
        b"LOGIN %s %s\0" as *const u8 as *const libc::c_char,
        if !user.is_null() {
            user as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        if !passwd.is_null() {
            passwd as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
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
    mut data: *mut Curl_easy,
    mut conn: *mut connectdata,
    mut mech: *const libc::c_char,
    mut initresp: *const libc::c_char,
) -> CURLcode {
    let mut result: CURLcode = CURLE_OK;
    if !initresp.is_null() {
        result = imap_sendf(
            data,
            conn,
            b"AUTHENTICATE %s %s\0" as *const u8 as *const libc::c_char,
            mech,
            initresp,
        );
    } else {
        result = imap_sendf(
            data,
            conn,
            b"AUTHENTICATE %s\0" as *const u8 as *const libc::c_char,
            mech,
        );
    }
    return result;
}
unsafe extern "C" fn imap_continue_authenticate(
    mut data: *mut Curl_easy,
    mut conn: *mut connectdata,
    mut resp: *const libc::c_char,
) -> CURLcode {
    let mut imapc: *mut imap_conn = &mut (*conn).proto.imapc;
    return Curl_pp_sendf(
        data,
        &mut (*imapc).pp as *mut pingpong,
        b"%s\0" as *const u8 as *const libc::c_char,
        resp,
    );
}
unsafe extern "C" fn imap_perform_authentication(
    mut data: *mut Curl_easy,
    mut conn: *mut connectdata,
) -> CURLcode {
    let mut result: CURLcode = CURLE_OK;
    let mut imapc: *mut imap_conn = &mut (*conn).proto.imapc;
    let mut progress: saslprogress = SASL_IDLE;
    if (*imapc).preauth as libc::c_int != 0
        || !Curl_sasl_can_authenticate(&mut (*imapc).sasl, conn)
    {
        state(data, IMAP_STOP);
        return result;
    }
    result = Curl_sasl_start(
        &mut (*imapc).sasl,
        data,
        conn,
        (*imapc).ir_supported,
        &mut progress,
    );
    if result as u64 == 0 {
        if progress as libc::c_uint == SASL_INPROGRESS as libc::c_int as libc::c_uint {
            state(data, IMAP_AUTHENTICATE);
        } else if !(*imapc).login_disabled
                && (*imapc).preftype
                    & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint != 0
            {
            result = imap_perform_login(data, conn);
        } else {
            Curl_infof(
                data,
                b"No known authentication mechanisms supported!\0" as *const u8
                    as *const libc::c_char,
            );
            result = CURLE_LOGIN_DENIED;
        }
    }
    return result;
}
unsafe extern "C" fn imap_perform_list(mut data: *mut Curl_easy) -> CURLcode {
    let mut result: CURLcode = CURLE_OK;
    let mut conn: *mut connectdata = (*data).conn;
    let mut imap: *mut IMAP = (*data).req.p.imap;
    if !((*imap).custom).is_null() {
        result = imap_sendf(
            data,
            conn,
            b"%s%s\0" as *const u8 as *const libc::c_char,
            (*imap).custom,
            if !((*imap).custom_params).is_null() {
                (*imap).custom_params as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
        );
    } else {
        let mut mailbox: *mut libc::c_char = if !((*imap).mailbox).is_null() {
            imap_atom((*imap).mailbox, 1 as libc::c_int != 0)
        } else {
            Curl_cstrdup
                .expect(
                    "non-null function pointer",
                )(b"\0" as *const u8 as *const libc::c_char)
        };
        if mailbox.is_null() {
            return CURLE_OUT_OF_MEMORY;
        }
        result = imap_sendf(
            data,
            conn,
            b"LIST \"%s\" *\0" as *const u8 as *const libc::c_char,
            mailbox,
        );
        Curl_cfree.expect("non-null function pointer")(mailbox as *mut libc::c_void);
    }
    if result as u64 == 0 {
        state(data, IMAP_LIST);
    }
    return result;
}
unsafe extern "C" fn imap_perform_select(mut data: *mut Curl_easy) -> CURLcode {
    let mut result: CURLcode = CURLE_OK;
    let mut conn: *mut connectdata = (*data).conn;
    let mut imap: *mut IMAP = (*data).req.p.imap;
    let mut imapc: *mut imap_conn = &mut (*conn).proto.imapc;
    let mut mailbox: *mut libc::c_char = 0 as *mut libc::c_char;
    Curl_cfree
        .expect("non-null function pointer")((*imapc).mailbox as *mut libc::c_void);
    let ref mut fresh3 = (*imapc).mailbox;
    *fresh3 = 0 as *mut libc::c_char;
    Curl_cfree
        .expect(
            "non-null function pointer",
        )((*imapc).mailbox_uidvalidity as *mut libc::c_void);
    let ref mut fresh4 = (*imapc).mailbox_uidvalidity;
    *fresh4 = 0 as *mut libc::c_char;
    if ((*imap).mailbox).is_null() {
        Curl_failf(
            data,
            b"Cannot SELECT without a mailbox.\0" as *const u8 as *const libc::c_char,
        );
        return CURLE_URL_MALFORMAT;
    }
    mailbox = imap_atom((*imap).mailbox, 0 as libc::c_int != 0);
    if mailbox.is_null() {
        return CURLE_OUT_OF_MEMORY;
    }
    result = imap_sendf(
        data,
        conn,
        b"SELECT %s\0" as *const u8 as *const libc::c_char,
        mailbox,
    );
    Curl_cfree.expect("non-null function pointer")(mailbox as *mut libc::c_void);
    if result as u64 == 0 {
        state(data, IMAP_SELECT);
    }
    return result;
}
unsafe extern "C" fn imap_perform_fetch(
    mut data: *mut Curl_easy,
    mut conn: *mut connectdata,
) -> CURLcode {
    let mut result: CURLcode = CURLE_OK;
    let mut imap: *mut IMAP = (*data).req.p.imap;
    if !((*imap).uid).is_null() {
        if !((*imap).partial).is_null() {
            result = imap_sendf(
                data,
                conn,
                b"UID FETCH %s BODY[%s]<%s>\0" as *const u8 as *const libc::c_char,
                (*imap).uid,
                if !((*imap).section).is_null() {
                    (*imap).section as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
                (*imap).partial,
            );
        } else {
            result = imap_sendf(
                data,
                conn,
                b"UID FETCH %s BODY[%s]\0" as *const u8 as *const libc::c_char,
                (*imap).uid,
                if !((*imap).section).is_null() {
                    (*imap).section as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
            );
        }
    } else if !((*imap).mindex).is_null() {
        if !((*imap).partial).is_null() {
            result = imap_sendf(
                data,
                conn,
                b"FETCH %s BODY[%s]<%s>\0" as *const u8 as *const libc::c_char,
                (*imap).mindex,
                if !((*imap).section).is_null() {
                    (*imap).section as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
                (*imap).partial,
            );
        } else {
            result = imap_sendf(
                data,
                conn,
                b"FETCH %s BODY[%s]\0" as *const u8 as *const libc::c_char,
                (*imap).mindex,
                if !((*imap).section).is_null() {
                    (*imap).section as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
            );
        }
    } else {
        Curl_failf(
            data,
            b"Cannot FETCH without a UID.\0" as *const u8 as *const libc::c_char,
        );
        return CURLE_URL_MALFORMAT;
    }
    if result as u64 == 0 {
        state(data, IMAP_FETCH);
    }
    return result;
}
unsafe extern "C" fn imap_perform_append(mut data: *mut Curl_easy) -> CURLcode {
    let mut result: CURLcode = CURLE_OK;
    let mut conn: *mut connectdata = (*data).conn;
    let mut imap: *mut IMAP = (*data).req.p.imap;
    let mut mailbox: *mut libc::c_char = 0 as *mut libc::c_char;
    if ((*imap).mailbox).is_null() {
        Curl_failf(
            data,
            b"Cannot APPEND without a mailbox.\0" as *const u8 as *const libc::c_char,
        );
        return CURLE_URL_MALFORMAT;
    }
    if (*data).set.mimepost.kind as libc::c_uint
        != MIMEKIND_NONE as libc::c_int as libc::c_uint
    {
        (*data).set.mimepost.flags
            &= !((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint;
        curl_mime_headers(
            &mut (*data).set.mimepost,
            (*data).set.headers,
            0 as libc::c_int,
        );
        result = Curl_mime_prepare_headers(
            &mut (*data).set.mimepost,
            0 as *const libc::c_char,
            0 as *const libc::c_char,
            MIMESTRATEGY_MAIL,
        );
        if result as u64 == 0 {
            if (Curl_checkheaders(
                data,
                b"Mime-Version\0" as *const u8 as *const libc::c_char,
            ))
                .is_null()
            {
                result = Curl_mime_add_header(
                    &mut (*data).set.mimepost.curlheaders as *mut *mut curl_slist,
                    b"Mime-Version: 1.0\0" as *const u8 as *const libc::c_char,
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
        let ref mut fresh5 = (*data).state.fread_func;
        *fresh5 = ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut libc::c_char,
                    size_t,
                    size_t,
                    *mut libc::c_void,
                ) -> size_t,
            >,
            curl_read_callback,
        >(
            Some(
                Curl_mime_read
                    as unsafe extern "C" fn(
                        *mut libc::c_char,
                        size_t,
                        size_t,
                        *mut libc::c_void,
                    ) -> size_t,
            ),
        );
        let ref mut fresh6 = (*data).state.in_0;
        *fresh6 = &mut (*data).set.mimepost as *mut curl_mimepart as *mut libc::c_void;
    }
    if (*data).state.infilesize < 0 as libc::c_int as libc::c_long {
        Curl_failf(
            data,
            b"Cannot APPEND with unknown input file size\0" as *const u8
                as *const libc::c_char,
        );
        return CURLE_UPLOAD_FAILED;
    }
    mailbox = imap_atom((*imap).mailbox, 0 as libc::c_int != 0);
    if mailbox.is_null() {
        return CURLE_OUT_OF_MEMORY;
    }
    result = imap_sendf(
        data,
        conn,
        b"APPEND %s (\\Seen) {%ld}\0" as *const u8 as *const libc::c_char,
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
    mut data: *mut Curl_easy,
    mut conn: *mut connectdata,
) -> CURLcode {
    let mut result: CURLcode = CURLE_OK;
    let mut imap: *mut IMAP = (*data).req.p.imap;
    if ((*imap).query).is_null() {
        Curl_failf(
            data,
            b"Cannot SEARCH without a query string.\0" as *const u8
                as *const libc::c_char,
        );
        return CURLE_URL_MALFORMAT;
    }
    result = imap_sendf(
        data,
        conn,
        b"SEARCH %s\0" as *const u8 as *const libc::c_char,
        (*imap).query,
    );
    if result as u64 == 0 {
        state(data, IMAP_SEARCH);
    }
    return result;
}
unsafe extern "C" fn imap_perform_logout(
    mut data: *mut Curl_easy,
    mut conn: *mut connectdata,
) -> CURLcode {
    let mut result: CURLcode = imap_sendf(
        data,
        conn,
        b"LOGOUT\0" as *const u8 as *const libc::c_char,
    );
    if result as u64 == 0 {
        state(data, IMAP_LOGOUT);
    }
    return result;
}
unsafe extern "C" fn imap_state_servergreet_resp(
    mut data: *mut Curl_easy,
    mut imapcode: libc::c_int,
    mut instate: imapstate,
) -> CURLcode {
    let mut conn: *mut connectdata = (*data).conn;
    if imapcode == 3 as libc::c_int {
        let mut imapc: *mut imap_conn = &mut (*conn).proto.imapc;
        (*imapc).preauth = 1 as libc::c_int != 0;
        Curl_infof(
            data,
            b"PREAUTH connection, already authenticated!\0" as *const u8
                as *const libc::c_char,
        );
    } else if imapcode != 1 as libc::c_int {
        Curl_failf(
            data,
            b"Got unexpected imap-server response\0" as *const u8 as *const libc::c_char,
        );
        return CURLE_WEIRD_SERVER_REPLY;
    }
    return imap_perform_capability(data, conn);
}
unsafe extern "C" fn imap_state_capability_resp(
    mut data: *mut Curl_easy,
    mut imapcode: libc::c_int,
    mut instate: imapstate,
) -> CURLcode {
    let mut result: CURLcode = CURLE_OK;
    let mut conn: *mut connectdata = (*data).conn;
    let mut imapc: *mut imap_conn = &mut (*conn).proto.imapc;
    let mut line: *const libc::c_char = (*data).state.buffer;
    if imapcode == '*' as i32 {
        line = line.offset(2 as libc::c_int as isize);
        loop {
            let mut wordlen: size_t = 0;
            while *line as libc::c_int != 0
                && (*line as libc::c_int == ' ' as i32
                    || *line as libc::c_int == '\t' as i32
                    || *line as libc::c_int == '\r' as i32
                    || *line as libc::c_int == '\n' as i32)
            {
                line = line.offset(1);
            }
            if *line == 0 {
                break;
            }
            wordlen = 0 as libc::c_int as size_t;
            while *line.offset(wordlen as isize) as libc::c_int != 0
                && *line.offset(wordlen as isize) as libc::c_int != ' ' as i32
                && *line.offset(wordlen as isize) as libc::c_int != '\t' as i32
                && *line.offset(wordlen as isize) as libc::c_int != '\r' as i32
                && *line.offset(wordlen as isize) as libc::c_int != '\n' as i32
            {
                wordlen = wordlen.wrapping_add(1);
            }
            if wordlen == 8 as libc::c_int as libc::c_ulong
                && memcmp(
                    line as *const libc::c_void,
                    b"STARTTLS\0" as *const u8 as *const libc::c_char
                        as *const libc::c_void,
                    8 as libc::c_int as libc::c_ulong,
                ) == 0
            {
                (*imapc).tls_supported = 1 as libc::c_int != 0;
            } else if wordlen == 13 as libc::c_int as libc::c_ulong
                    && memcmp(
                        line as *const libc::c_void,
                        b"LOGINDISABLED\0" as *const u8 as *const libc::c_char
                            as *const libc::c_void,
                        13 as libc::c_int as libc::c_ulong,
                    ) == 0
                {
                (*imapc).login_disabled = 1 as libc::c_int != 0;
            } else if wordlen == 7 as libc::c_int as libc::c_ulong
                    && memcmp(
                        line as *const libc::c_void,
                        b"SASL-IR\0" as *const u8 as *const libc::c_char
                            as *const libc::c_void,
                        7 as libc::c_int as libc::c_ulong,
                    ) == 0
                {
                (*imapc).ir_supported = 1 as libc::c_int != 0;
            } else if wordlen > 5 as libc::c_int as libc::c_ulong
                    && memcmp(
                        line as *const libc::c_void,
                        b"AUTH=\0" as *const u8 as *const libc::c_char
                            as *const libc::c_void,
                        5 as libc::c_int as libc::c_ulong,
                    ) == 0
                {
                let mut llen: size_t = 0;
                let mut mechbit: libc::c_ushort = 0;
                line = line.offset(5 as libc::c_int as isize);
                wordlen = (wordlen as libc::c_ulong)
                    .wrapping_sub(5 as libc::c_int as libc::c_ulong) as size_t as size_t;
                mechbit = Curl_sasl_decode_mech(line, wordlen, &mut llen);
                if mechbit as libc::c_int != 0 && llen == wordlen {
                    let ref mut fresh7 = (*imapc).sasl.authmechs;
                    *fresh7 = (*fresh7 as libc::c_int | mechbit as libc::c_int)
                        as libc::c_ushort;
                }
            }
            line = line.offset(wordlen as isize);
        }
    } else if (*data).set.use_ssl as libc::c_uint != 0
            && ((*conn).ssl[0 as libc::c_int as usize]).use_0() == 0
        {
        if imapcode == 1 as libc::c_int && (*imapc).tls_supported as libc::c_int != 0
            && !(*imapc).preauth
        {
            result = imap_perform_starttls(data, conn);
        } else if (*data).set.use_ssl as libc::c_uint
                <= CURLUSESSL_TRY as libc::c_int as libc::c_uint
            {
            result = imap_perform_authentication(data, conn);
        } else {
            Curl_failf(
                data,
                b"STARTTLS not available.\0" as *const u8 as *const libc::c_char,
            );
            result = CURLE_USE_SSL_FAILED;
        }
    } else {
        result = imap_perform_authentication(data, conn);
    }
    return result;
}
unsafe extern "C" fn imap_state_starttls_resp(
    mut data: *mut Curl_easy,
    mut imapcode: libc::c_int,
    mut instate: imapstate,
) -> CURLcode {
    let mut result: CURLcode = CURLE_OK;
    let mut conn: *mut connectdata = (*data).conn;
    if (*(*data).conn).proto.imapc.pp.cache_size != 0 {
        return CURLE_WEIRD_SERVER_REPLY;
    }
    if imapcode != 1 as libc::c_int {
        if (*data).set.use_ssl as libc::c_uint
            != CURLUSESSL_TRY as libc::c_int as libc::c_uint
        {
            Curl_failf(data, b"STARTTLS denied\0" as *const u8 as *const libc::c_char);
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
    mut data: *mut Curl_easy,
    mut conn: *mut connectdata,
    mut imapcode: libc::c_int,
    mut instate: imapstate,
) -> CURLcode {
    let mut result: CURLcode = CURLE_OK;
    let mut imapc: *mut imap_conn = &mut (*conn).proto.imapc;
    let mut progress: saslprogress = SASL_IDLE;
    result = Curl_sasl_continue(&mut (*imapc).sasl, data, conn, imapcode, &mut progress);
    if result as u64 == 0 {
        match progress as libc::c_uint {
            2 => {
                state(data, IMAP_STOP);
            }
            0 => {
                if !(*imapc).login_disabled
                    && (*imapc).preftype
                        & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint != 0
                {
                    result = imap_perform_login(data, conn);
                } else {
                    Curl_failf(
                        data,
                        b"Authentication cancelled\0" as *const u8 as *const libc::c_char,
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
    mut data: *mut Curl_easy,
    mut imapcode: libc::c_int,
    mut instate: imapstate,
) -> CURLcode {
    let mut result: CURLcode = CURLE_OK;
    if imapcode != 1 as libc::c_int {
        Curl_failf(
            data,
            b"Access denied. %c\0" as *const u8 as *const libc::c_char,
            imapcode,
        );
        result = CURLE_LOGIN_DENIED;
    } else {
        state(data, IMAP_STOP);
    }
    return result;
}
unsafe extern "C" fn imap_state_listsearch_resp(
    mut data: *mut Curl_easy,
    mut imapcode: libc::c_int,
    mut instate: imapstate,
) -> CURLcode {
    let mut result: CURLcode = CURLE_OK;
    let mut line: *mut libc::c_char = (*data).state.buffer;
    let mut len: size_t = strlen(line);
    if imapcode == '*' as i32 {
        *line.offset(len as isize) = '\n' as i32 as libc::c_char;
        result = Curl_client_write(
            data,
            (1 as libc::c_int) << 0 as libc::c_int,
            line,
            len.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        *line.offset(len as isize) = '\u{0}' as i32 as libc::c_char;
    } else if imapcode != 1 as libc::c_int {
        result = CURLE_QUOTE_ERROR;
    } else {
        state(data, IMAP_STOP);
    }
    return result;
}
unsafe extern "C" fn imap_state_select_resp(
    mut data: *mut Curl_easy,
    mut imapcode: libc::c_int,
    mut instate: imapstate,
) -> CURLcode {
    let mut result: CURLcode = CURLE_OK;
    let mut conn: *mut connectdata = (*data).conn;
    let mut imap: *mut IMAP = (*data).req.p.imap;
    let mut imapc: *mut imap_conn = &mut (*conn).proto.imapc;
    let mut line: *const libc::c_char = (*data).state.buffer;
    if imapcode == '*' as i32 {
        let mut tmp: [libc::c_char; 20] = [0; 20];
        if sscanf(
            line.offset(2 as libc::c_int as isize),
            b"OK [UIDVALIDITY %19[0123456789]]\0" as *const u8 as *const libc::c_char,
            tmp.as_mut_ptr(),
        ) == 1 as libc::c_int
        {
            Curl_cfree
                .expect(
                    "non-null function pointer",
                )((*imapc).mailbox_uidvalidity as *mut libc::c_void);
            let ref mut fresh8 = (*imapc).mailbox_uidvalidity;
            *fresh8 = 0 as *mut libc::c_char;
            let ref mut fresh9 = (*imapc).mailbox_uidvalidity;
            *fresh9 = Curl_cstrdup.expect("non-null function pointer")(tmp.as_mut_ptr());
        }
    } else if imapcode == 1 as libc::c_int {
        if !((*imap).uidvalidity).is_null() && !((*imapc).mailbox_uidvalidity).is_null()
            && Curl_strcasecompare((*imap).uidvalidity, (*imapc).mailbox_uidvalidity)
                == 0
        {
            Curl_failf(
                data,
                b"Mailbox UIDVALIDITY has changed\0" as *const u8 as *const libc::c_char,
            );
            result = CURLE_REMOTE_FILE_NOT_FOUND;
        } else {
            let ref mut fresh10 = (*imapc).mailbox;
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
        Curl_failf(data, b"Select failed\0" as *const u8 as *const libc::c_char);
        result = CURLE_LOGIN_DENIED;
    }
    return result;
}
unsafe extern "C" fn imap_state_fetch_resp(
    mut data: *mut Curl_easy,
    mut conn: *mut connectdata,
    mut imapcode: libc::c_int,
    mut instate: imapstate,
) -> CURLcode {
    let mut result: CURLcode = CURLE_OK;
    let mut imapc: *mut imap_conn = &mut (*conn).proto.imapc;
    let mut pp: *mut pingpong = &mut (*imapc).pp;
    let mut ptr: *const libc::c_char = (*data).state.buffer;
    let mut parsed: bool = 0 as libc::c_int != 0;
    let mut size: curl_off_t = 0 as libc::c_int as curl_off_t;
    if imapcode != '*' as i32 {
        Curl_pgrsSetDownloadSize(data, -(1 as libc::c_int) as curl_off_t);
        state(data, IMAP_STOP);
        return CURLE_REMOTE_FILE_NOT_FOUND;
    }
    while *ptr as libc::c_int != 0 && *ptr as libc::c_int != '{' as i32 {
        ptr = ptr.offset(1);
    }
    if *ptr as libc::c_int == '{' as i32 {
        let mut endptr: *mut libc::c_char = 0 as *mut libc::c_char;
        if curlx_strtoofft(
            ptr.offset(1 as libc::c_int as isize),
            &mut endptr,
            10 as libc::c_int,
            &mut size,
        ) as u64 == 0
        {
            if endptr.offset_from(ptr) as libc::c_long > 1 as libc::c_int as libc::c_long
                && *endptr.offset(0 as libc::c_int as isize) as libc::c_int == '}' as i32
                && *endptr.offset(1 as libc::c_int as isize) as libc::c_int
                    == '\r' as i32
                && *endptr.offset(2 as libc::c_int as isize) as libc::c_int
                    == '\u{0}' as i32
            {
                parsed = 1 as libc::c_int != 0;
            }
        }
    }
    if parsed {
        Curl_infof(
            data,
            b"Found %ld bytes to download\0" as *const u8 as *const libc::c_char,
            size,
        );
        Curl_pgrsSetDownloadSize(data, size);
        if !((*pp).cache).is_null() {
            let mut chunk: size_t = (*pp).cache_size;
            if chunk > size as size_t {
                chunk = size as size_t;
            }
            if chunk == 0 {
                state(data, IMAP_STOP);
                return CURLE_OK;
            }
            result = Curl_client_write(
                data,
                (1 as libc::c_int) << 0 as libc::c_int,
                (*pp).cache,
                chunk,
            );
            if result as u64 != 0 {
                return result;
            }
            let ref mut fresh11 = (*data).req.bytecount;
            *fresh11 = (*fresh11 as libc::c_ulong).wrapping_add(chunk) as curl_off_t
                as curl_off_t;
            Curl_infof(
                data,
                b"Written %zu bytes, %lu bytes are left for transfer\0" as *const u8
                    as *const libc::c_char,
                chunk,
                (size as libc::c_ulong).wrapping_sub(chunk),
            );
            if (*pp).cache_size > chunk {
                memmove(
                    (*pp).cache as *mut libc::c_void,
                    ((*pp).cache).offset(chunk as isize) as *const libc::c_void,
                    ((*pp).cache_size).wrapping_sub(chunk),
                );
                let ref mut fresh12 = (*pp).cache_size;
                *fresh12 = (*fresh12 as libc::c_ulong).wrapping_sub(chunk) as size_t
                    as size_t;
            } else {
                Curl_cfree
                    .expect(
                        "non-null function pointer",
                    )((*pp).cache as *mut libc::c_void);
                let ref mut fresh13 = (*pp).cache;
                *fresh13 = 0 as *mut libc::c_char;
                (*pp).cache_size = 0 as libc::c_int as size_t;
            }
        }
        if (*data).req.bytecount == size {
            Curl_setup_transfer(
                data,
                -(1 as libc::c_int),
                -(1 as libc::c_int) as curl_off_t,
                0 as libc::c_int != 0,
                -(1 as libc::c_int),
            );
        } else {
            (*data).req.maxdownload = size;
            (*(*data).conn).cselect_bits = 0x1 as libc::c_int;
            Curl_setup_transfer(
                data,
                0 as libc::c_int,
                size,
                0 as libc::c_int != 0,
                -(1 as libc::c_int),
            );
        }
    } else {
        Curl_failf(
            data,
            b"Failed to parse FETCH response.\0" as *const u8 as *const libc::c_char,
        );
        result = CURLE_WEIRD_SERVER_REPLY;
    }
    state(data, IMAP_STOP);
    return result;
}
unsafe extern "C" fn imap_state_fetch_final_resp(
    mut data: *mut Curl_easy,
    mut imapcode: libc::c_int,
    mut instate: imapstate,
) -> CURLcode {
    let mut result: CURLcode = CURLE_OK;
    if imapcode != 1 as libc::c_int {
        result = CURLE_WEIRD_SERVER_REPLY;
    } else {
        state(data, IMAP_STOP);
    }
    return result;
}
unsafe extern "C" fn imap_state_append_resp(
    mut data: *mut Curl_easy,
    mut imapcode: libc::c_int,
    mut instate: imapstate,
) -> CURLcode {
    let mut result: CURLcode = CURLE_OK;
    if imapcode != '+' as i32 {
        result = CURLE_UPLOAD_FAILED;
    } else {
        Curl_pgrsSetUploadSize(data, (*data).state.infilesize);
        Curl_setup_transfer(
            data,
            -(1 as libc::c_int),
            -(1 as libc::c_int) as curl_off_t,
            0 as libc::c_int != 0,
            0 as libc::c_int,
        );
        state(data, IMAP_STOP);
    }
    return result;
}
unsafe extern "C" fn imap_state_append_final_resp(
    mut data: *mut Curl_easy,
    mut imapcode: libc::c_int,
    mut instate: imapstate,
) -> CURLcode {
    let mut result: CURLcode = CURLE_OK;
    if imapcode != 1 as libc::c_int {
        result = CURLE_UPLOAD_FAILED;
    } else {
        state(data, IMAP_STOP);
    }
    return result;
}
unsafe extern "C" fn imap_statemachine(
    mut data: *mut Curl_easy,
    mut conn: *mut connectdata,
) -> CURLcode {
    let mut result: CURLcode = CURLE_OK;
    let mut sock: curl_socket_t = (*conn).sock[0 as libc::c_int as usize];
    let mut imapcode: libc::c_int = 0;
    let mut imapc: *mut imap_conn = &mut (*conn).proto.imapc;
    let mut pp: *mut pingpong = &mut (*imapc).pp;
    let mut nread: size_t = 0 as libc::c_int as size_t;
    if (*imapc).state as libc::c_uint == IMAP_UPGRADETLS as libc::c_int as libc::c_uint {
        return imap_perform_upgrade_tls(data, conn);
    }
    if (*pp).sendleft != 0 {
        return Curl_pp_flushsend(data, pp);
    }
    loop {
        result = Curl_pp_readresp(data, sock, pp, &mut imapcode, &mut nread);
        if result as u64 != 0 {
            return result;
        }
        if imapcode == -(1 as libc::c_int) {
            return CURLE_WEIRD_SERVER_REPLY;
        }
        if imapcode == 0 {
            break;
        }
        match (*imapc).state as libc::c_uint {
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
            && (*imapc).state as libc::c_uint != IMAP_STOP as libc::c_int as libc::c_uint
            && Curl_pp_moredata(pp) as libc::c_int != 0)
        {
            break;
        }
    }
    return result;
}
unsafe extern "C" fn imap_multi_statemach(
    mut data: *mut Curl_easy,
    mut done: *mut bool,
) -> CURLcode {
    let mut result: CURLcode = CURLE_OK;
    let mut conn: *mut connectdata = (*data).conn;
    let mut imapc: *mut imap_conn = &mut (*conn).proto.imapc;
    if (*(*conn).handler).flags
        & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint != 0
        && !(*imapc).ssldone
    {
        result = Curl_ssl_connect_nonblocking(
            data,
            conn,
            0 as libc::c_int != 0,
            0 as libc::c_int,
            &mut (*imapc).ssldone,
        );
        if result as libc::c_uint != 0 || !(*imapc).ssldone {
            return result;
        }
    }
    result = Curl_pp_statemach(
        data,
        &mut (*imapc).pp,
        0 as libc::c_int != 0,
        0 as libc::c_int != 0,
    );
    *done = if (*imapc).state as libc::c_uint == IMAP_STOP as libc::c_int as libc::c_uint
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0;
    return result;
}
unsafe extern "C" fn imap_block_statemach(
    mut data: *mut Curl_easy,
    mut conn: *mut connectdata,
    mut disconnecting: bool,
) -> CURLcode {
    let mut result: CURLcode = CURLE_OK;
    let mut imapc: *mut imap_conn = &mut (*conn).proto.imapc;
    while (*imapc).state as libc::c_uint != IMAP_STOP as libc::c_int as libc::c_uint
        && result as u64 == 0
    {
        result = Curl_pp_statemach(
            data,
            &mut (*imapc).pp,
            1 as libc::c_int != 0,
            disconnecting,
        );
    }
    return result;
}
unsafe extern "C" fn imap_init(mut data: *mut Curl_easy) -> CURLcode {
    let mut result: CURLcode = CURLE_OK;
    let mut imap: *mut IMAP = 0 as *mut IMAP;
    let ref mut fresh14 = (*data).req.p.imap;
    *fresh14 = Curl_ccalloc
        .expect(
            "non-null function pointer",
        )(::std::mem::size_of::<IMAP>() as libc::c_ulong, 1 as libc::c_int as size_t)
        as *mut IMAP;
    imap = *fresh14;
    if imap.is_null() {
        result = CURLE_OUT_OF_MEMORY;
    }
    return result;
}
unsafe extern "C" fn imap_getsock(
    mut data: *mut Curl_easy,
    mut conn: *mut connectdata,
    mut socks: *mut curl_socket_t,
) -> libc::c_int {
    return Curl_pp_getsock(data, &mut (*conn).proto.imapc.pp, socks);
}
unsafe extern "C" fn imap_connect(
    mut data: *mut Curl_easy,
    mut done: *mut bool,
) -> CURLcode {
    let mut result: CURLcode = CURLE_OK;
    let mut conn: *mut connectdata = (*data).conn;
    let mut imapc: *mut imap_conn = &mut (*conn).proto.imapc;
    let mut pp: *mut pingpong = &mut (*imapc).pp;
    *done = 0 as libc::c_int != 0;
    Curl_conncontrol(conn, 0 as libc::c_int);
    (*pp).response_time = (120 as libc::c_int * 1000 as libc::c_int) as timediff_t;
    let ref mut fresh15 = (*pp).statemachine;
    *fresh15 = Some(
        imap_statemachine
            as unsafe extern "C" fn(*mut Curl_easy, *mut connectdata) -> CURLcode,
    );
    let ref mut fresh16 = (*pp).endofresp;
    *fresh16 = Some(
        imap_endofresp
            as unsafe extern "C" fn(
                *mut Curl_easy,
                *mut connectdata,
                *mut libc::c_char,
                size_t,
                *mut libc::c_int,
            ) -> bool,
    );
    (*imapc).preftype = !(0 as libc::c_uint);
    Curl_sasl_init(&mut (*imapc).sasl, &saslimap);
    Curl_dyn_init(
        &mut (*imapc).dyn_0,
        (64 as libc::c_int * 1024 as libc::c_int) as size_t,
    );
    Curl_pp_setup(pp);
    Curl_pp_init(data, pp);
    result = imap_parse_url_options(conn);
    if result as u64 != 0 {
        return result;
    }
    state(data, IMAP_SERVERGREET);
    strcpy(((*imapc).resptag).as_mut_ptr(), b"*\0" as *const u8 as *const libc::c_char);
    result = imap_multi_statemach(data, done);
    return result;
}
unsafe extern "C" fn imap_done(
    mut data: *mut Curl_easy,
    mut status: CURLcode,
    mut premature: bool,
) -> CURLcode {
    let mut result: CURLcode = CURLE_OK;
    let mut conn: *mut connectdata = (*data).conn;
    let mut imap: *mut IMAP = (*data).req.p.imap;
    if imap.is_null() {
        return CURLE_OK;
    }
    if status as u64 != 0 {
        Curl_conncontrol(conn, 1 as libc::c_int);
        result = status;
    } else if ((*data).set).connect_only() == 0 && ((*imap).custom).is_null()
            && (!((*imap).uid).is_null() || !((*imap).mindex).is_null()
                || ((*data).set).upload() as libc::c_int != 0
                || (*data).set.mimepost.kind as libc::c_uint
                    != MIMEKIND_NONE as libc::c_int as libc::c_uint)
        {
        if ((*data).set).upload() == 0
            && (*data).set.mimepost.kind as libc::c_uint
                == MIMEKIND_NONE as libc::c_int as libc::c_uint
        {
            state(data, IMAP_FETCH_FINAL);
        } else {
            result = Curl_pp_sendf(
                data,
                &mut (*conn).proto.imapc.pp as *mut pingpong,
                b"%s\0" as *const u8 as *const libc::c_char,
                b"\0" as *const u8 as *const libc::c_char,
            );
            if result as u64 == 0 {
                state(data, IMAP_APPEND_FINAL);
            }
        }
        if result as u64 == 0 {
            result = imap_block_statemach(data, conn, 0 as libc::c_int != 0);
        }
    }
    Curl_cfree.expect("non-null function pointer")((*imap).mailbox as *mut libc::c_void);
    let ref mut fresh17 = (*imap).mailbox;
    *fresh17 = 0 as *mut libc::c_char;
    Curl_cfree
        .expect("non-null function pointer")((*imap).uidvalidity as *mut libc::c_void);
    let ref mut fresh18 = (*imap).uidvalidity;
    *fresh18 = 0 as *mut libc::c_char;
    Curl_cfree.expect("non-null function pointer")((*imap).uid as *mut libc::c_void);
    let ref mut fresh19 = (*imap).uid;
    *fresh19 = 0 as *mut libc::c_char;
    Curl_cfree.expect("non-null function pointer")((*imap).mindex as *mut libc::c_void);
    let ref mut fresh20 = (*imap).mindex;
    *fresh20 = 0 as *mut libc::c_char;
    Curl_cfree.expect("non-null function pointer")((*imap).section as *mut libc::c_void);
    let ref mut fresh21 = (*imap).section;
    *fresh21 = 0 as *mut libc::c_char;
    Curl_cfree.expect("non-null function pointer")((*imap).partial as *mut libc::c_void);
    let ref mut fresh22 = (*imap).partial;
    *fresh22 = 0 as *mut libc::c_char;
    Curl_cfree.expect("non-null function pointer")((*imap).query as *mut libc::c_void);
    let ref mut fresh23 = (*imap).query;
    *fresh23 = 0 as *mut libc::c_char;
    Curl_cfree.expect("non-null function pointer")((*imap).custom as *mut libc::c_void);
    let ref mut fresh24 = (*imap).custom;
    *fresh24 = 0 as *mut libc::c_char;
    Curl_cfree
        .expect("non-null function pointer")((*imap).custom_params as *mut libc::c_void);
    let ref mut fresh25 = (*imap).custom_params;
    *fresh25 = 0 as *mut libc::c_char;
    (*imap).transfer = PPTRANSFER_BODY;
    return result;
}
unsafe extern "C" fn imap_perform(
    mut data: *mut Curl_easy,
    mut connected: *mut bool,
    mut dophase_done: *mut bool,
) -> CURLcode {
    let mut result: CURLcode = CURLE_OK;
    let mut conn: *mut connectdata = (*data).conn;
    let mut imap: *mut IMAP = (*data).req.p.imap;
    let mut imapc: *mut imap_conn = &mut (*conn).proto.imapc;
    let mut selected: bool = 0 as libc::c_int != 0;
    if ((*data).set).opt_no_body() != 0 {
        (*imap).transfer = PPTRANSFER_INFO;
    }
    *dophase_done = 0 as libc::c_int != 0;
    if !((*imap).mailbox).is_null() && !((*imapc).mailbox).is_null()
        && Curl_strcasecompare((*imap).mailbox, (*imapc).mailbox) != 0
        && (((*imap).uidvalidity).is_null() || ((*imapc).mailbox_uidvalidity).is_null()
            || Curl_strcasecompare((*imap).uidvalidity, (*imapc).mailbox_uidvalidity)
                != 0)
    {
        selected = 1 as libc::c_int != 0;
    }
    if ((*data).set).upload() as libc::c_int != 0
        || (*data).set.mimepost.kind as libc::c_uint
            != MIMEKIND_NONE as libc::c_int as libc::c_uint
    {
        result = imap_perform_append(data);
    } else if !((*imap).custom).is_null()
            && (selected as libc::c_int != 0 || ((*imap).mailbox).is_null())
        {
        result = imap_perform_list(data);
    } else if ((*imap).custom).is_null() && selected as libc::c_int != 0
            && (!((*imap).uid).is_null() || !((*imap).mindex).is_null())
        {
        result = imap_perform_fetch(data, conn);
    } else if ((*imap).custom).is_null() && selected as libc::c_int != 0
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
    *connected = (*conn).bits.tcpconnect[0 as libc::c_int as usize];
    *dophase_done;
    return result;
}
unsafe extern "C" fn imap_do(mut data: *mut Curl_easy, mut done: *mut bool) -> CURLcode {
    let mut result: CURLcode = CURLE_OK;
    *done = 0 as libc::c_int != 0;
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
    mut data: *mut Curl_easy,
    mut conn: *mut connectdata,
    mut dead_connection: bool,
) -> CURLcode {
    let mut imapc: *mut imap_conn = &mut (*conn).proto.imapc;
    if !dead_connection && ((*conn).bits).protoconnstart() as libc::c_int != 0 {
        if imap_perform_logout(data, conn) as u64 == 0 {
            imap_block_statemach(data, conn, 1 as libc::c_int != 0);
        }
    }
    Curl_pp_disconnect(&mut (*imapc).pp);
    Curl_dyn_free(&mut (*imapc).dyn_0);
    Curl_sasl_cleanup(conn, (*imapc).sasl.authused as libc::c_uint);
    Curl_cfree
        .expect("non-null function pointer")((*imapc).mailbox as *mut libc::c_void);
    let ref mut fresh26 = (*imapc).mailbox;
    *fresh26 = 0 as *mut libc::c_char;
    Curl_cfree
        .expect(
            "non-null function pointer",
        )((*imapc).mailbox_uidvalidity as *mut libc::c_void);
    let ref mut fresh27 = (*imapc).mailbox_uidvalidity;
    *fresh27 = 0 as *mut libc::c_char;
    return CURLE_OK;
}
unsafe extern "C" fn imap_dophase_done(
    mut data: *mut Curl_easy,
    mut connected: bool,
) -> CURLcode {
    let mut imap: *mut IMAP = (*data).req.p.imap;
    if (*imap).transfer as libc::c_uint != PPTRANSFER_BODY as libc::c_int as libc::c_uint
    {
        Curl_setup_transfer(
            data,
            -(1 as libc::c_int),
            -(1 as libc::c_int) as curl_off_t,
            0 as libc::c_int != 0,
            -(1 as libc::c_int),
        );
    }
    return CURLE_OK;
}
unsafe extern "C" fn imap_doing(
    mut data: *mut Curl_easy,
    mut dophase_done: *mut bool,
) -> CURLcode {
    let mut result: CURLcode = imap_multi_statemach(data, dophase_done);
    if !(result as u64 != 0) {
        if *dophase_done {
            result = imap_dophase_done(data, 0 as libc::c_int != 0);
        }
    }
    return result;
}
unsafe extern "C" fn imap_regular_transfer(
    mut data: *mut Curl_easy,
    mut dophase_done: *mut bool,
) -> CURLcode {
    let mut result: CURLcode = CURLE_OK;
    let mut connected: bool = 0 as libc::c_int != 0;
    (*data).req.size = -(1 as libc::c_int) as curl_off_t;
    Curl_pgrsSetUploadCounter(data, 0 as libc::c_int as curl_off_t);
    Curl_pgrsSetDownloadCounter(data, 0 as libc::c_int as curl_off_t);
    Curl_pgrsSetUploadSize(data, -(1 as libc::c_int) as curl_off_t);
    Curl_pgrsSetDownloadSize(data, -(1 as libc::c_int) as curl_off_t);
    result = imap_perform(data, &mut connected, dophase_done);
    if result as u64 == 0 && *dophase_done as libc::c_int != 0 {
        result = imap_dophase_done(data, connected);
    }
    return result;
}
unsafe extern "C" fn imap_setup_connection(
    mut data: *mut Curl_easy,
    mut conn: *mut connectdata,
) -> CURLcode {
    let mut result: CURLcode = imap_init(data);
    if result as u64 != 0 {
        return result;
    }
    let ref mut fresh28 = (*conn).bits;
    (*fresh28).set_tls_upgraded(0 as libc::c_int as bit);
    return CURLE_OK;
}
unsafe extern "C" fn imap_sendf(
    mut data: *mut Curl_easy,
    mut conn: *mut connectdata,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> CURLcode {
    let mut result: CURLcode = CURLE_OK;
    let mut imapc: *mut imap_conn = &mut (*conn).proto.imapc;
    let ref mut fresh29 = (*imapc).cmdid;
    *fresh29 = (*fresh29).wrapping_add(1);
    curl_msnprintf(
        ((*imapc).resptag).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong,
        b"%c%03d\0" as *const u8 as *const libc::c_char,
        'A' as i32
            + curlx_sltosi((*conn).connection_id % 26 as libc::c_int as libc::c_long),
        (*fresh29).wrapping_rem(1000 as libc::c_int as libc::c_uint),
    );
    Curl_dyn_reset(&mut (*imapc).dyn_0);
    result = Curl_dyn_addf(
        &mut (*imapc).dyn_0 as *mut dynbuf,
        b"%s %s\0" as *const u8 as *const libc::c_char,
        ((*imapc).resptag).as_mut_ptr(),
        fmt,
    );
    if result as u64 == 0 {
        let mut ap: ::std::ffi::VaListImpl;
        ap = args.clone();
        result = Curl_pp_vsendf(
            data,
            &mut (*imapc).pp,
            Curl_dyn_ptr(&mut (*imapc).dyn_0),
            ap.as_va_list(),
        );
    }
    return result;
}
unsafe extern "C" fn imap_atom(
    mut str: *const libc::c_char,
    mut escape_only: bool,
) -> *mut libc::c_char {
    let atom_specials: [libc::c_char; 8] = *::std::mem::transmute::<
        &[u8; 8],
        &[libc::c_char; 8],
    >(b"(){ %*]\0");
    let mut p1: *const libc::c_char = 0 as *const libc::c_char;
    let mut p2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut backsp_count: size_t = 0 as libc::c_int as size_t;
    let mut quote_count: size_t = 0 as libc::c_int as size_t;
    let mut others_exists: bool = 0 as libc::c_int != 0;
    let mut newlen: size_t = 0 as libc::c_int as size_t;
    let mut newstr: *mut libc::c_char = 0 as *mut libc::c_char;
    if str.is_null() {
        return 0 as *mut libc::c_char;
    }
    p1 = str;
    while *p1 != 0 {
        if *p1 as libc::c_int == '\\' as i32 {
            backsp_count = backsp_count.wrapping_add(1);
        } else if *p1 as libc::c_int == '"' as i32 {
            quote_count = quote_count.wrapping_add(1);
        } else if !escape_only {
            let mut p3: *const libc::c_char = atom_specials.as_ptr();
            while *p3 as libc::c_int != 0 && !others_exists {
                if *p1 as libc::c_int == *p3 as libc::c_int {
                    others_exists = 1 as libc::c_int != 0;
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
            (if escape_only as libc::c_int != 0 {
                0 as libc::c_int
            } else {
                2 as libc::c_int
            }) as libc::c_ulong,
        );
    newstr = Curl_cmalloc
        .expect(
            "non-null function pointer",
        )(
        newlen
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    if newstr.is_null() {
        return 0 as *mut libc::c_char;
    }
    p2 = newstr;
    if !escape_only {
        *newstr.offset(0 as libc::c_int as isize) = '"' as i32 as libc::c_char;
        *newstr
            .offset(
                newlen.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) = '"' as i32 as libc::c_char;
        p2 = p2.offset(1);
    }
    p1 = str;
    while *p1 != 0 {
        if *p1 as libc::c_int == '\\' as i32 || *p1 as libc::c_int == '"' as i32 {
            *p2 = '\\' as i32 as libc::c_char;
            p2 = p2.offset(1);
        }
        *p2 = *p1;
        p1 = p1.offset(1);
        p2 = p2.offset(1);
    }
    *newstr.offset(newlen as isize) = '\u{0}' as i32 as libc::c_char;
    return newstr;
}
unsafe extern "C" fn imap_is_bchar(mut ch: libc::c_char) -> bool {
    match ch as libc::c_int {
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
        _ => return 0 as libc::c_int != 0,
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn imap_parse_url_options(mut conn: *mut connectdata) -> CURLcode {
    let mut result: CURLcode = CURLE_OK;
    let mut imapc: *mut imap_conn = &mut (*conn).proto.imapc;
    let mut ptr: *const libc::c_char = (*conn).options;
    (*imapc).sasl.resetprefs = 1 as libc::c_int != 0;
    while result as u64 == 0 && !ptr.is_null() && *ptr as libc::c_int != 0 {
        let mut key: *const libc::c_char = ptr;
        let mut value: *const libc::c_char = 0 as *const libc::c_char;
        while *ptr as libc::c_int != 0 && *ptr as libc::c_int != '=' as i32 {
            ptr = ptr.offset(1);
        }
        value = ptr.offset(1 as libc::c_int as isize);
        while *ptr as libc::c_int != 0 && *ptr as libc::c_int != ';' as i32 {
            ptr = ptr.offset(1);
        }
        if Curl_strncasecompare(
            key,
            b"AUTH=\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as size_t,
        ) != 0
        {
            result = Curl_sasl_parse_url_auth_option(
                &mut (*imapc).sasl,
                value,
                ptr.offset_from(value) as libc::c_long as size_t,
            );
        } else {
            result = CURLE_URL_MALFORMAT;
        }
        if *ptr as libc::c_int == ';' as i32 {
            ptr = ptr.offset(1);
        }
    }
    match (*imapc).sasl.prefmech as libc::c_int {
        0 => {
            (*imapc).preftype = 0 as libc::c_int as libc::c_uint;
        }
        65503 => {
            (*imapc).preftype = !(0 as libc::c_uint);
        }
        _ => {
            (*imapc).preftype = ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint;
        }
    }
    return result;
}
unsafe extern "C" fn imap_parse_url_path(mut data: *mut Curl_easy) -> CURLcode {
    let mut result: CURLcode = CURLE_OK;
    let mut imap: *mut IMAP = (*data).req.p.imap;
    let mut begin: *const libc::c_char = &mut *((*data).state.up.path)
        .offset(1 as libc::c_int as isize) as *mut libc::c_char;
    let mut ptr: *const libc::c_char = begin;
    while imap_is_bchar(*ptr) {
        ptr = ptr.offset(1);
    }
    if ptr != begin {
        let mut end: *const libc::c_char = ptr;
        if end > begin
            && *end.offset(-(1 as libc::c_int) as isize) as libc::c_int == '/' as i32
        {
            end = end.offset(-1);
        }
        result = Curl_urldecode(
            data,
            begin,
            end.offset_from(begin) as libc::c_long as size_t,
            &mut (*imap).mailbox,
            0 as *mut size_t,
            REJECT_CTRL,
        );
        if result as u64 != 0 {
            return result;
        }
    } else {
        let ref mut fresh30 = (*imap).mailbox;
        *fresh30 = 0 as *mut libc::c_char;
    }
    while *ptr as libc::c_int == ';' as i32 {
        let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut valuelen: size_t = 0;
        ptr = ptr.offset(1);
        begin = ptr;
        while *ptr as libc::c_int != 0 && *ptr as libc::c_int != '=' as i32 {
            ptr = ptr.offset(1);
        }
        if *ptr == 0 {
            return CURLE_URL_MALFORMAT;
        }
        result = Curl_urldecode(
            data,
            begin,
            ptr.offset_from(begin) as libc::c_long as size_t,
            &mut name,
            0 as *mut size_t,
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
            ptr.offset_from(begin) as libc::c_long as size_t,
            &mut value,
            &mut valuelen,
            REJECT_CTRL,
        );
        if result as u64 != 0 {
            Curl_cfree.expect("non-null function pointer")(name as *mut libc::c_void);
            return result;
        }
        if Curl_strcasecompare(
            name,
            b"UIDVALIDITY\0" as *const u8 as *const libc::c_char,
        ) != 0 && ((*imap).uidvalidity).is_null()
        {
            if valuelen > 0 as libc::c_int as libc::c_ulong
                && *value
                    .offset(
                        valuelen.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    ) as libc::c_int == '/' as i32
            {
                *value
                    .offset(
                        valuelen.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    ) = '\u{0}' as i32 as libc::c_char;
            }
            let ref mut fresh31 = (*imap).uidvalidity;
            *fresh31 = value;
            value = 0 as *mut libc::c_char;
        } else if Curl_strcasecompare(name, b"UID\0" as *const u8 as *const libc::c_char)
                != 0 && ((*imap).uid).is_null()
            {
            if valuelen > 0 as libc::c_int as libc::c_ulong
                && *value
                    .offset(
                        valuelen.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    ) as libc::c_int == '/' as i32
            {
                *value
                    .offset(
                        valuelen.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    ) = '\u{0}' as i32 as libc::c_char;
            }
            let ref mut fresh32 = (*imap).uid;
            *fresh32 = value;
            value = 0 as *mut libc::c_char;
        } else if Curl_strcasecompare(
                name,
                b"MAILINDEX\0" as *const u8 as *const libc::c_char,
            ) != 0 && ((*imap).mindex).is_null()
            {
            if valuelen > 0 as libc::c_int as libc::c_ulong
                && *value
                    .offset(
                        valuelen.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    ) as libc::c_int == '/' as i32
            {
                *value
                    .offset(
                        valuelen.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    ) = '\u{0}' as i32 as libc::c_char;
            }
            let ref mut fresh33 = (*imap).mindex;
            *fresh33 = value;
            value = 0 as *mut libc::c_char;
        } else if Curl_strcasecompare(
                name,
                b"SECTION\0" as *const u8 as *const libc::c_char,
            ) != 0 && ((*imap).section).is_null()
            {
            if valuelen > 0 as libc::c_int as libc::c_ulong
                && *value
                    .offset(
                        valuelen.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    ) as libc::c_int == '/' as i32
            {
                *value
                    .offset(
                        valuelen.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    ) = '\u{0}' as i32 as libc::c_char;
            }
            let ref mut fresh34 = (*imap).section;
            *fresh34 = value;
            value = 0 as *mut libc::c_char;
        } else if Curl_strcasecompare(
                name,
                b"PARTIAL\0" as *const u8 as *const libc::c_char,
            ) != 0 && ((*imap).partial).is_null()
            {
            if valuelen > 0 as libc::c_int as libc::c_ulong
                && *value
                    .offset(
                        valuelen.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    ) as libc::c_int == '/' as i32
            {
                *value
                    .offset(
                        valuelen.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    ) = '\u{0}' as i32 as libc::c_char;
            }
            let ref mut fresh35 = (*imap).partial;
            *fresh35 = value;
            value = 0 as *mut libc::c_char;
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
            &mut (*imap).query,
            ((1 as libc::c_int) << 6 as libc::c_int) as libc::c_uint,
        );
    }
    if *ptr != 0 {
        return CURLE_URL_MALFORMAT;
    }
    return CURLE_OK;
}
unsafe extern "C" fn imap_parse_custom_request(mut data: *mut Curl_easy) -> CURLcode {
    let mut result: CURLcode = CURLE_OK;
    let mut imap: *mut IMAP = (*data).req.p.imap;
    let mut custom: *const libc::c_char = (*data)
        .set
        .str_0[STRING_CUSTOMREQUEST as libc::c_int as usize];
    if !custom.is_null() {
        result = Curl_urldecode(
            data,
            custom,
            0 as libc::c_int as size_t,
            &mut (*imap).custom,
            0 as *mut size_t,
            REJECT_CTRL,
        );
        if result as u64 == 0 {
            let mut params: *const libc::c_char = (*imap).custom;
            while *params as libc::c_int != 0 && *params as libc::c_int != ' ' as i32 {
                params = params.offset(1);
            }
            if *params != 0 {
                let ref mut fresh36 = (*imap).custom_params;
                *fresh36 = Curl_cstrdup.expect("non-null function pointer")(params);
                *((*imap).custom)
                    .offset(
                        params.offset_from((*imap).custom) as libc::c_long as isize,
                    ) = '\u{0}' as i32 as libc::c_char;
                if ((*imap).custom_params).is_null() {
                    result = CURLE_OUT_OF_MEMORY;
                }
            }
        }
    }
    return result;
}
