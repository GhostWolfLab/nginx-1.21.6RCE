#![no_std]

extern crate libc;

use libc::c_void;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
extern fn recv(fd: i32, buf: *mut c_void, len: usize, _flags: i32) -> isize {
    // Perform read
    let ret: isize = unsafe {
        libc::read(fd, buf, len)
    };

    if ret > 0 {
        let buf = unsafe {
            core::slice::from_raw_parts(buf as *const u8, len)
        };

        const EXPLOIT: &[u8] = b"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
        if buf.windows(EXPLOIT.len()).position(|x| x == EXPLOIT).is_some() {
            let args = [
                "sudo\0".as_ptr() as *const i8,
                "/usr/bin/ffplay\0".as_ptr() as *const i8,
                "/home/pleb/badapple.mp4\0".as_ptr() as *const i8,
                core::ptr::null(),
            ];
            let envp = [
                "DISPLAY=:0\0".as_ptr() as *const i8,
                core::ptr::null(),
            ];
            unsafe {
                libc::execve(
                    "/usr/bin/sudo\0".as_ptr() as *const i8,
                    args.as_ptr(),
                    envp.as_ptr());
            }
        }
    }

    ret
}

