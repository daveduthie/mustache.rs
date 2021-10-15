use wasm_bindgen::prelude::*;

pub fn hello(name: &str) -> String {
    return "Hello".to_owned() + name
}
