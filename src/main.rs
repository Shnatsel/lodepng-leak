extern crate lodepng;

use std::fs::File;
use std::io::prelude::*;

const ASAN_DEFAULT_OPTIONS: &'static [u8] = b"detect_odr_violation=1\0";

#[no_mangle]
pub extern "C" fn __asan_default_options() -> *const u8 {
    ASAN_DEFAULT_OPTIONS as *const [u8] as *const u8
}


fn main() -> Result<(), lodepng::Error>  {
	let mut file = File::open("in.png").unwrap();
	let mut data = Vec::new();
	file.read_to_end(&mut data);
    loop {
        let _result = lodepng::decode32(&data);
    }
}