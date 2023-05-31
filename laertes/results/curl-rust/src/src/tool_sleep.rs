use ::libc;
extern "C" {
    fn poll(__fds: * mut crate::src::lib::multi::pollfd, __nfds: u64, __timeout: i32) -> i32;
}
pub type nfds_t = u64;
// #[derive(Copy, Clone)]

pub type pollfd = crate::src::lib::multi::pollfd;
#[no_mangle]
pub unsafe extern "C" fn tool_go_sleep(mut ms: i64) {
    poll(0 as *mut pollfd, 0 as i32 as nfds_t, ms as i32);
}
use crate::laertes_rt::*;
