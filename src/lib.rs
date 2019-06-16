#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));


#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::ffi::CString;
    use std::fs::{remove_file, File};
    use std::os::raw::c_char;

    #[test]
    fn set_and_read_flags() {
        unsafe {
            let mut path = env::current_dir().expect("Could not determine current dir");
            path.push("e2p-sys_testfile_Gahlu1ka");
            let path_cstr = CString::new(path.to_str().expect("Could not convert path to str"))
                .expect("Could not convert str to CStr");
            let path_ptr: *const c_char = path_cstr.as_ptr();
            let _f = File::create(&path).expect("Could not create file");

            fsetflags(path_ptr, EXT2_NOATIME_FL as u64);

            let mut readback: u64 = 0;
            let readback_ptr: *mut u64 = &mut readback;
            fgetflags(path_ptr, readback_ptr);

            drop(_f);
            let _ = remove_file(path);

            assert_eq!(readback, EXT2_NOATIME_FL as u64)
        }
    }
}
