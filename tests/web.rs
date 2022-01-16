//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use faster_wasm_md::{attach_md_to_element, Converter, Options};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

#[wasm_bindgen_test]
fn markdown_hash() {
    let md_hash = String::from("# Sample Markdown");
    let options = Options::all();
    let converter_hash = Converter::new(options, md_hash);
    let out_hash = converter_hash.out();
    let html_hash = "<h1>Sample Markdown</h1>\n";
    assert_eq!(out_hash, html_hash);
    let md_hash_hash = String::from("## Hash Hash");
    let converter_hash_hash = Converter::new(options, md_hash_hash);
    let out_hash_hash = converter_hash_hash.out();
    let html_hash_hash = "<h2>Hash Hash</h2>\n";
    assert_eq!(out_hash_hash, html_hash_hash);
    let md_hash_hash_hash = String::from("### Hash Hash Hash");
    let converter_hash_hash_hash = Converter::new(options, md_hash_hash_hash);
    let out_hash_hash_hash = converter_hash_hash_hash.out();
    let html_hash_hash_hash = "<h3>Hash Hash Hash</h3>\n";
    assert_eq!(out_hash_hash_hash, html_hash_hash_hash);
}

#[wasm_bindgen_test]
fn markdown_basic_formatting() {
    let md_bold = "**Bold**".to_string();
    let options = Options::all();
    let converter_bold = Converter::new(options, md_bold);
    let out_bold = converter_bold.out();
    let html_bold = "<p><strong>Bold</strong></p>\n";
    assert_eq!(out_bold, html_bold);
}

#[wasm_bindgen_test]
fn insert_markdown() {
    let options = Options::all();
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let val = document.create_element("div").unwrap();
    val.set_text_content(Some("Hello from Rust!"));
    let md = "# Hash".to_string();
    attach_md_to_element(options, md, &val);
    assert_eq!("<h1>Hash</h1>\n", val.inner_html());
}