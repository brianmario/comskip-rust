#[no_mangle]
pub static mut xPos: libc::c_int = 0;
#[no_mangle]
pub static mut yPos: libc::c_int = 0;
#[no_mangle]
pub static mut lMouseDown: libc::c_int = 0;
#[no_mangle]
pub static mut key: libc::c_int = 0;
#[no_mangle]
pub static mut osname: [libc::c_char; 1024] = [0; 1024];
