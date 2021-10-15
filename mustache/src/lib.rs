#[cfg(console_error_panic_hook)]
extern crate console_error_panic_hook;
extern crate lazy_static;
extern crate nom;
extern crate serde_json;
extern crate string_builder;
extern crate web_sys;

use std::{collections::HashSet, sync::Mutex};

use lazy_static::lazy_static;
use serde_json::{json, Value};
use string_builder::Builder;
use wasm_bindgen::prelude::*;

use parser::tokenize;
use tokens::{MustacheToken, Tokens};

mod parser;
mod tokens;
mod utils;

trait ILookup {
    fn lookup(&self, path: &[String]) -> &serde_json::Value;
    fn to_mustache_str(&self) -> String;
}

impl ILookup for serde_json::Value {
    fn lookup(&self, path: &[String]) -> &Self {
        let mut ctx = self;
        for name in path {
            match ctx {
                Value::Object(o) => match o.get(name) {
                    Some(val) => ctx = val,
                    None => return &serde_json::Value::Null,
                },
                Value::Array(_) => todo!(),
                scalar => return scalar,
            }
        }

        ctx
    }

    fn to_mustache_str(&self) -> String {
        match self {
            Value::Null => String::default(),
            Value::Bool(b) => b.to_string(),
            Value::Number(n) => n.to_string(),
            Value::String(s) => s.clone(),
            val => serde_json::ser::to_string(val).expect("failed to write json value"),
        }
    }
}

type Context = serde_json::Value;

// TODO: replace with unsafe code: js is single threaded, so we don't need the mutex
lazy_static! {
    static ref CONTEXT: Mutex<Context> = Mutex::new(json!(null));
}

#[wasm_bindgen]
pub fn set_context(val: &JsValue) {
    let parsed_val: serde_json::Value = val.into_serde().unwrap_or(json!(null));
    let ctx: &mut Context = &mut CONTEXT.lock().unwrap();
    *ctx = parsed_val;
}

#[wasm_bindgen]
pub struct Mustache {
    tokens: Tokens,
}

#[wasm_bindgen]
impl Mustache {
    pub fn new(template: &str) -> Self {
        // todo how to convert to a std result type?
        let (_, tokens) = tokenize(template).unwrap();
        Mustache { tokens }
    }

    pub fn deps(&self, prefix: &JsValue) -> Vec<JsValue> {
        let parsed_prefix: Vec<String> = JsValue::into_serde(prefix).expect("wot?");
        let mut deps_set = HashSet::new();
        let prefix_len = parsed_prefix.len();
        for tok in &self.tokens {
            if let MustacheToken::Lookup(idents) = tok {
                if parsed_prefix == &idents[..prefix_len] {
                    if let Some(ident) = idents.get(prefix_len) {
                        deps_set.insert(ident);
                    }
                }
            }
        }
        deps_set.iter().map(|s| JsValue::from(*s)).collect()
    }

    pub fn render(&self) -> String {
        let mut result = Builder::default();
        let context = CONTEXT.lock().unwrap();

        for token in &self.tokens {
            match token {
                tokens::MustacheToken::Text(text) => result.append(text.clone()),
                tokens::MustacheToken::Lookup(idents) => {
                    result.append(context.lookup(idents).to_mustache_str())
                }
            }
        }

        result.string().unwrap_or(String::from(""))
    }
}

#[cfg(console_error_panic_hook)]
#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}
