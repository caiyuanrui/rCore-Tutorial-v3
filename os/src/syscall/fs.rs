const FD_STDOUT: usize = 1;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            let slice = unsafe { core::slice::from_raw_parts(buf, len) };
            let s = core::str::from_utf8(slice).unwrap();
            println!("{}", s);
            len as isize
        }
        _ => {
            unimplemented!("Unsupported fd in sys_write!");
        }
    }
}
