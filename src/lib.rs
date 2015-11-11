#![feature(plugin_registrar, rustc_private, slice_patterns)]

extern crate syntax;
extern crate rustc;

use std::rc::Rc;
use rustc::plugin::Registry;
use syntax::ast::{LitByteStr, TokenTree};
use syntax::codemap::Span;
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
use syntax::ext::build::AstBuilder;
use syntax::parse::token;

fn decode_hex_str(hex: &str) -> Result<Vec<u8>, String> {
    let mut vec = Vec::with_capacity(hex.len() / 2);
    let mut filtered = hex.chars().filter(|&c| !c.is_whitespace());
    // eat hex characters two at a time
    while let Some(first) = filtered.next() {
        let hi = try!(decode_hex_char(first));
        if let Some(second) = filtered.next() {
            let lo = try!(decode_hex_char(second));
            // finally, combine them
            vec.push((hi << 4) | lo)
        } else {
            return Err("odd number of hex characters".into());
        }
    }
    Ok(vec)
}

fn decode_hex_char(c: char) -> Result<u8, String> {
    if c.is_digit(16) {
        Ok(c.to_digit(16).unwrap() as u8)
    } else {
        Err(format!("non-hexadecimal non-whitespace character {:?}", c))
    }
}

fn expand_hex(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) -> Box<MacResult + 'static> {
    match args {
        [TokenTree::Token(tok_span, token::Literal(token::Str_(name), _))] => {
            match decode_hex_str(&*name.as_str()) {
                Ok(bytes) => {
                    // success!
                    let byte_str = LitByteStr(Rc::new(bytes));
                    return MacEager::expr(cx.expr_lit(sp, byte_str));
                }
                Err(reason) => cx.span_err(tok_span, &reason),
            }
        }
        [ref tt] => cx.span_err(tt.get_span(), "expected string literal"),
        [_, ref tt, ..] => cx.span_err(tt.get_span(), "too many arguments"),
        _ => cx.span_err(sp, "expected string literal"),
    }
    // error path
    DummyResult::any(sp)
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("hex", expand_hex);
}
