/*
MIT License

Copyright (c) 2019-2023 Michael Lass

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

#![warn(rust_2018_idioms)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

//! Low-level Rust bindings for libe2p from e2fsprogs
//!
//! The bindings need to be generated using bindgen while building this
//! crate because libe2p has no stable API and available file flags or
//! file system features differ between versions. This also means that
//! publicly availale documentation on this crate might not be accurate
//! on your system.
//!
//! For building this crate, you need to have e2fsprogs-dev (also called
//! e2fslibs-dev or libext2fs-dev) and libclang installed.

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
include!(concat!(env!("OUT_DIR"), "/constants.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::ffi::CString;
    use std::fs::{remove_file, File};
    use std::io::Read;
    use std::mem::transmute;
    use std::os::raw::c_char;

    #[test]
    fn set_and_read_flags() {
        let mut path = env::current_dir().unwrap();
        path.push("e2p-sys_testfile_Gahlu1ka");
        let path_cstr = CString::new(path.to_str().unwrap()).unwrap();
        let path_ptr: *const c_char = path_cstr.as_ptr();
        let _f = File::create(&path).unwrap();

        unsafe {
            fsetflags(path_ptr, EXT2_NOATIME_FL as u64);
        }

        let mut readback: u64 = 0;
        let readback_ptr: *mut u64 = &mut readback;

        unsafe {
            fgetflags(path_ptr, readback_ptr);
        }

        remove_file(path).unwrap();
        assert_eq!(readback, EXT2_NOATIME_FL as u64)
    }

    #[test]
    fn read_superblock() {
        let mut path = env::current_dir().unwrap();
        path.push("test_data");
        path.push("sb.raw");
        let mut buf = [0; 1024];
        let mut f = File::open(path).unwrap();
        f.read_exact(&mut buf).unwrap();

        let sb: ext2_super_block;
        unsafe {
            sb = transmute::<[u8; 1024], ext2_super_block>(buf);
        }

        // Randomly selected values from the superblock
        assert_eq!(sb.s_inodes_count, 256);
        assert_eq!(sb.s_blocks_per_group, 8192);
        assert_eq!(sb.s_magic, 0xEF53);
    }

    #[test]
    fn read_constant() {
        assert!(CONSTANTS.iter().any(|&s| s == "EXT2_IMMUTABLE_FL"));
        assert!(!CONSTANTS.iter().any(|&s| s == "NO_SUCH_CONST"));
        assert_eq!(EXT2_IMMUTABLE_FL, 0x10);
    }
}
