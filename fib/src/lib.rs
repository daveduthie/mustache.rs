extern crate serde;
extern crate serde_json;

// Very important to use `transparent` to prevent ABI issues
#[repr(transparent)]
pub struct JsInteropString(*mut String);

impl JsInteropString {
    // Unsafe because we create a string and say it's full of valid
    // UTF-8 data, but it isn't!
    unsafe fn with_capacity(cap: usize) -> Self {
        let mut d = Vec::with_capacity(cap);
        d.set_len(cap);
        let s = Box::new(String::from_utf8_unchecked(d));
        JsInteropString(Box::into_raw(s))
    }

    pub unsafe fn as_string(&self) -> &String {
        &*self.0
    }

    unsafe fn as_mut_string(&mut self) -> &mut String {
        &mut *self.0
    }

    unsafe fn into_boxed_string(self) -> Box<String> {
        Box::from_raw(self.0)
    }

    unsafe fn as_mut_ptr(&mut self) -> *mut u8 {
        self.as_mut_string().as_mut_vec().as_mut_ptr()
    }
}

#[no_mangle]
pub unsafe extern "C" fn stringPrepare(cap: usize) -> JsInteropString {
    JsInteropString::with_capacity(cap)
}

#[no_mangle]
pub unsafe extern "C" fn stringData(mut s: JsInteropString) -> *mut u8 {
    s.as_mut_ptr()
}

#[no_mangle]
pub unsafe extern "C" fn stringLen(s: JsInteropString) -> usize {
    s.as_string().len()
}

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct DifferenceArgs {
    pub a: i32,
    pub b: i32,
}

#[derive(Serialize)]
pub struct DifferenceResult {
    pub the_answer: i32,
}

#[no_mangle]
pub unsafe extern "C" fn difference(s: JsInteropString) -> JsInteropString {
    let mut ret = 0;
    if let Ok(args) = serde_json::from_str(&s.as_string()[..]) {
        let n = real_code::compute(args);
        ret = n;
    }

    let result = DifferenceResult { the_answer: ret};
    let result_str: String = serde_json::to_string(&result).unwrap();
    
    JsInteropString(Box::into_raw(Box::new(result_str)))
   
}

mod real_code {
    use crate::DifferenceArgs;

    pub fn compute(args: DifferenceArgs) -> i32 {
        args.a - args.b
    }
}
