use ::libc;
extern "C" {
    fn curl_strequal(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
    fn curl_version_info(_: CURLversion) -> *mut curl_version_info_data;
}
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
pub type CURLversion = libc::c_uint;
pub const CURLVERSION_LAST: CURLversion = 10;
pub const CURLVERSION_TENTH: CURLversion = 9;
pub const CURLVERSION_NINTH: CURLversion = 8;
pub const CURLVERSION_EIGHTH: CURLversion = 7;
pub const CURLVERSION_SEVENTH: CURLversion = 6;
pub const CURLVERSION_SIXTH: CURLversion = 5;
pub const CURLVERSION_FIFTH: CURLversion = 4;
pub const CURLVERSION_FOURTH: CURLversion = 3;
pub const CURLVERSION_THIRD: CURLversion = 2;
pub const CURLVERSION_SECOND: CURLversion = 1;
pub const CURLVERSION_FIRST: CURLversion = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct curl_version_info_data {
    pub age: CURLversion,
    pub version: *const libc::c_char,
    pub version_num: libc::c_uint,
    pub host: *const libc::c_char,
    pub features: libc::c_int,
    pub ssl_version: *const libc::c_char,
    pub ssl_version_num: libc::c_long,
    pub libz_version: *const libc::c_char,
    pub protocols: *const *const libc::c_char,
    pub ares: *const libc::c_char,
    pub ares_num: libc::c_int,
    pub libidn: *const libc::c_char,
    pub iconv_ver_num: libc::c_int,
    pub libssh_version: *const libc::c_char,
    pub brotli_ver_num: libc::c_uint,
    pub brotli_version: *const libc::c_char,
    pub nghttp2_ver_num: libc::c_uint,
    pub nghttp2_version: *const libc::c_char,
    pub quic_version: *const libc::c_char,
    pub cainfo: *const libc::c_char,
    pub capath: *const libc::c_char,
    pub zstd_ver_num: libc::c_uint,
    pub zstd_version: *const libc::c_char,
    pub hyper_version: *const libc::c_char,
    pub gsasl_version: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct proto_name_pattern {
    pub proto_name: *const libc::c_char,
    pub proto_pattern: libc::c_long,
}
#[no_mangle]
pub static mut curlinfo: *mut curl_version_info_data = 0 as *const curl_version_info_data
    as *mut curl_version_info_data;
#[no_mangle]
pub static mut built_in_protos: libc::c_long = 0 as libc::c_int as libc::c_long;
#[no_mangle]
pub unsafe extern "C" fn get_libcurl_info() -> CURLcode {
    static mut possibly_built_in: [proto_name_pattern; 27] = [
        {
            let mut init = proto_name_pattern {
                proto_name: b"dict\0" as *const u8 as *const libc::c_char,
                proto_pattern: ((1 as libc::c_int) << 9 as libc::c_int) as libc::c_long,
            };
            init
        },
        {
            let mut init = proto_name_pattern {
                proto_name: b"file\0" as *const u8 as *const libc::c_char,
                proto_pattern: ((1 as libc::c_int) << 10 as libc::c_int) as libc::c_long,
            };
            init
        },
        {
            let mut init = proto_name_pattern {
                proto_name: b"ftp\0" as *const u8 as *const libc::c_char,
                proto_pattern: ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_long,
            };
            init
        },
        {
            let mut init = proto_name_pattern {
                proto_name: b"ftps\0" as *const u8 as *const libc::c_char,
                proto_pattern: ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_long,
            };
            init
        },
        {
            let mut init = proto_name_pattern {
                proto_name: b"gopher\0" as *const u8 as *const libc::c_char,
                proto_pattern: ((1 as libc::c_int) << 25 as libc::c_int) as libc::c_long,
            };
            init
        },
        {
            let mut init = proto_name_pattern {
                proto_name: b"gophers\0" as *const u8 as *const libc::c_char,
                proto_pattern: ((1 as libc::c_int) << 29 as libc::c_int) as libc::c_long,
            };
            init
        },
        {
            let mut init = proto_name_pattern {
                proto_name: b"http\0" as *const u8 as *const libc::c_char,
                proto_pattern: ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_long,
            };
            init
        },
        {
            let mut init = proto_name_pattern {
                proto_name: b"https\0" as *const u8 as *const libc::c_char,
                proto_pattern: ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_long,
            };
            init
        },
        {
            let mut init = proto_name_pattern {
                proto_name: b"imap\0" as *const u8 as *const libc::c_char,
                proto_pattern: ((1 as libc::c_int) << 12 as libc::c_int) as libc::c_long,
            };
            init
        },
        {
            let mut init = proto_name_pattern {
                proto_name: b"imaps\0" as *const u8 as *const libc::c_char,
                proto_pattern: ((1 as libc::c_int) << 13 as libc::c_int) as libc::c_long,
            };
            init
        },
        {
            let mut init = proto_name_pattern {
                proto_name: b"ldap\0" as *const u8 as *const libc::c_char,
                proto_pattern: ((1 as libc::c_int) << 7 as libc::c_int) as libc::c_long,
            };
            init
        },
        {
            let mut init = proto_name_pattern {
                proto_name: b"ldaps\0" as *const u8 as *const libc::c_char,
                proto_pattern: ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_long,
            };
            init
        },
        {
            let mut init = proto_name_pattern {
                proto_name: b"mqtt\0" as *const u8 as *const libc::c_char,
                proto_pattern: ((1 as libc::c_int) << 28 as libc::c_int) as libc::c_long,
            };
            init
        },
        {
            let mut init = proto_name_pattern {
                proto_name: b"pop3\0" as *const u8 as *const libc::c_char,
                proto_pattern: ((1 as libc::c_int) << 14 as libc::c_int) as libc::c_long,
            };
            init
        },
        {
            let mut init = proto_name_pattern {
                proto_name: b"pop3s\0" as *const u8 as *const libc::c_char,
                proto_pattern: ((1 as libc::c_int) << 15 as libc::c_int) as libc::c_long,
            };
            init
        },
        {
            let mut init = proto_name_pattern {
                proto_name: b"rtmp\0" as *const u8 as *const libc::c_char,
                proto_pattern: ((1 as libc::c_int) << 19 as libc::c_int) as libc::c_long,
            };
            init
        },
        {
            let mut init = proto_name_pattern {
                proto_name: b"rtmps\0" as *const u8 as *const libc::c_char,
                proto_pattern: ((1 as libc::c_int) << 23 as libc::c_int) as libc::c_long,
            };
            init
        },
        {
            let mut init = proto_name_pattern {
                proto_name: b"rtsp\0" as *const u8 as *const libc::c_char,
                proto_pattern: ((1 as libc::c_int) << 18 as libc::c_int) as libc::c_long,
            };
            init
        },
        {
            let mut init = proto_name_pattern {
                proto_name: b"scp\0" as *const u8 as *const libc::c_char,
                proto_pattern: ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_long,
            };
            init
        },
        {
            let mut init = proto_name_pattern {
                proto_name: b"sftp\0" as *const u8 as *const libc::c_char,
                proto_pattern: ((1 as libc::c_int) << 5 as libc::c_int) as libc::c_long,
            };
            init
        },
        {
            let mut init = proto_name_pattern {
                proto_name: b"smb\0" as *const u8 as *const libc::c_char,
                proto_pattern: ((1 as libc::c_int) << 26 as libc::c_int) as libc::c_long,
            };
            init
        },
        {
            let mut init = proto_name_pattern {
                proto_name: b"smbs\0" as *const u8 as *const libc::c_char,
                proto_pattern: ((1 as libc::c_int) << 27 as libc::c_int) as libc::c_long,
            };
            init
        },
        {
            let mut init = proto_name_pattern {
                proto_name: b"smtp\0" as *const u8 as *const libc::c_char,
                proto_pattern: ((1 as libc::c_int) << 16 as libc::c_int) as libc::c_long,
            };
            init
        },
        {
            let mut init = proto_name_pattern {
                proto_name: b"smtps\0" as *const u8 as *const libc::c_char,
                proto_pattern: ((1 as libc::c_int) << 17 as libc::c_int) as libc::c_long,
            };
            init
        },
        {
            let mut init = proto_name_pattern {
                proto_name: b"telnet\0" as *const u8 as *const libc::c_char,
                proto_pattern: ((1 as libc::c_int) << 6 as libc::c_int) as libc::c_long,
            };
            init
        },
        {
            let mut init = proto_name_pattern {
                proto_name: b"tftp\0" as *const u8 as *const libc::c_char,
                proto_pattern: ((1 as libc::c_int) << 11 as libc::c_int) as libc::c_long,
            };
            init
        },
        {
            let mut init = proto_name_pattern {
                proto_name: 0 as *const libc::c_char,
                proto_pattern: 0 as libc::c_int as libc::c_long,
            };
            init
        },
    ];
    let mut proto: *const *const libc::c_char = 0 as *const *const libc::c_char;
    curlinfo = curl_version_info(CURLVERSION_TENTH);
    if curlinfo.is_null() {
        return CURLE_FAILED_INIT;
    }
    built_in_protos = 0 as libc::c_int as libc::c_long;
    if !((*curlinfo).protocols).is_null() {
        proto = (*curlinfo).protocols;
        while !(*proto).is_null() {
            let mut p: *const proto_name_pattern = 0 as *const proto_name_pattern;
            p = possibly_built_in.as_ptr();
            while !((*p).proto_name).is_null() {
                if curl_strequal(*proto, (*p).proto_name) != 0 {
                    built_in_protos |= (*p).proto_pattern;
                    break;
                } else {
                    p = p.offset(1);
                }
            }
            proto = proto.offset(1);
        }
    }
    return CURLE_OK;
}
