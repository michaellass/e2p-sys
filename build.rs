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

extern crate bindgen;
extern crate metadeps;

use regex::Regex;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::vec::Vec;

fn main() {
    metadeps::probe().unwrap();

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .allowlist_file(r"(.*/e2p\.h)|(.*/ext2.*\.h)")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .unwrap();

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .unwrap();

    // Determine a list of all public consts, so other crates know what they can expect
    let re = Regex::new(r"(?m)^|\s+pub const ([^: ]+)\s?:").unwrap();
    let mut constants = Vec::new();
    let binding_code = bindings.to_string();
    for cap in re.captures_iter(&binding_code) {
        if let Some(m) = cap.get(1) { constants.push(String::from(m.as_str())) }
    }

    let mut f = File::create(out_path.join("constants.rs")).unwrap();
    write!(
        &mut f,
        "pub const constants: [&str; {}] = {:?};",
        constants.len(),
        constants
    )
    .unwrap();
}
