#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));


#[cfg(test)]
mod tests{
    use super::*;
    use std::env;
    use std::fs::{File,remove_file};
    use std::os::raw::c_char;
    use std::ffi::CString;

    #[test]
    fn set_and_read_flags() {
        unsafe {
            let mut path = match env::current_dir(){
                Ok(p)  => p,
                Err(_) => panic!("Could not determine current dir"),
            };
            path.push("e2p-sys_testfile_Gahlu1ka");
            let path_cstr = match path.to_str() {
                Some(s) => match CString::new(s) {
                    Ok(cs) => cs,
                    Err(_) => panic!("Could not convert str to CStr"),
                },
                None    => panic!("Could not convert path to str"),
            };
            let path_ptr : *const c_char = path_cstr.as_ptr();
            let _f = match File::create(&path) {
                Ok(fh) => fh,
                Err(_) => panic!("Could not create file"),
            };

            fsetflags(path_ptr, EXT2_NOATIME_FL as u64);

            let mut readback : u64 = 0;
            let readback_ptr : *mut u64 = &mut readback;
            fgetflags(path_ptr, readback_ptr);

            drop(_f);
            let _ = remove_file(path);

            assert_eq!(readback, EXT2_NOATIME_FL as u64)
        }
    }
}
