use crate::tee_client_api::{TEEC_Context, TEEC_Session};
use libc::*;

#[repr(C)]
pub struct TEEC_Context_imp {
    fd: *mut c_void,
}

#[repr(C)]
pub struct TEEC_Session_imp {
    ctx: *mut TEEC_Context,
    session_id: u32,
}

#[repr(C)]
pub struct TEEC_SharedMemory_imp {
    id: c_int,
    alloced_size: size_t,
    shadow_buffer: *mut c_void,
    registered_fd: c_int,
    priv_: *mut c_void,
}

#[repr(C)]
pub struct TEEC_Operation_imp {
    session: *mut TEEC_Session,
}
