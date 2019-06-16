/*
MIT License

Copyright (c) 2019 Michael Lass

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

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
