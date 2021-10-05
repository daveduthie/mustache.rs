extern crate nom;
extern crate serde_json;
extern crate string_builder;
extern crate web_sys;
#[cfg(console_error_panic_hook)]
extern crate console_error_panic_hook;

mod parser;
mod tokens;
mod utils;

use serde_json::Value;
use string_builder::Builder;

use parser::tokenize;
use tokens::Tokens;
use wasm_bindgen::prelude::*;

trait ILookup {
    fn lookup(&self, path: &[String]) -> &Self;
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
            val => serde_json::ser::to_string(val).unwrap(),
        }
    }
}

#[cfg(console_error_panic_hook)]
#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct Mustache {
    tokens: Tokens,
}

#[wasm_bindgen]
impl Mustache {
    pub fn new(template: &str) -> Self {
        let (_, tokens) = tokenize(template).unwrap(); // todo how to convert to a std result type?
        Mustache { tokens }
    }

    pub fn render(&self, ctx: &JsValue) -> String {
        let ctx: serde_json::Value = ctx.into_serde().unwrap_or(serde_json::Value::Null);
        let mut result = Builder::default();
        utils::log!("ctx: {:?}", ctx);
        for token in &self.tokens {
            match token {
                tokens::MustacheToken::Text(text) => result.append(text.clone()),
                tokens::MustacheToken::Lookup(idents) => {
                    result.append(ctx.lookup(idents).to_mustache_str())
                }
            }
        }

        result.string().unwrap_or(String::from(""))
    }
}
