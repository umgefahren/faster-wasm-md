mod utils;

use std::io::Cursor;
use pulldown_cmark::html::write_html;
use pulldown_cmark::Parser;
use wasm_bindgen::prelude::*;
use web_sys::Element;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, faster-wasm-md!");
}


#[derive(Clone, Copy)]
#[wasm_bindgen]
pub struct Options {
    tables_enabled: bool,
    footnotes_enabled: bool,
    strikethrough_enabled: bool,
    tasklist_enabled: bool,
    smart_punctuation_enabled: bool,
    heading_attributes_enabled: bool
}

#[wasm_bindgen]
impl Options {
    pub fn empty() -> Self {
        Self {
            tables_enabled: false,
            footnotes_enabled: false,
            strikethrough_enabled: false,
            tasklist_enabled: false,
            smart_punctuation_enabled: false,
            heading_attributes_enabled: false
        }
    }

    pub fn all() -> Self {
        Self {
            tables_enabled: true,
            footnotes_enabled: true,
            strikethrough_enabled: true,
            tasklist_enabled: true,
            smart_punctuation_enabled: true,
            heading_attributes_enabled: true
        }
    }

    pub fn set_tables(&mut self, val: bool) {
        self.tables_enabled = val;
    }

    pub fn set_footnotes(&mut self, val: bool) {
        self.footnotes_enabled = val;
    }

    pub fn set_strikethrough(&mut self, val: bool) {
        self.strikethrough_enabled = val
    }

    pub fn set_tasklist(&mut self, val: bool) {
        self.tasklist_enabled = val;
    }

    pub fn set_smart_punctuation(&mut self, val: bool) {
        self.smart_punctuation_enabled = val;
    }

    pub fn set_heading_attributes(&mut self, val: bool) {
        self.heading_attributes_enabled = val;
    }

    pub fn get_tables(&self) -> bool {
        self.tables_enabled
    }

    pub fn get_footnotes(&self) -> bool {
        self.footnotes_enabled
    }

    pub fn get_strikethrough(&self) -> bool {
        self.strikethrough_enabled
    }

    pub fn get_tasklist(&self) -> bool {
        self.tasklist_enabled
    }

    pub fn get_smart_punctuation(&self) -> bool {
        self.smart_punctuation_enabled
    }

    pub fn get_heading_attributes(&self) -> bool {
        self.heading_attributes_enabled
    }
}

impl From<Options> for pulldown_cmark::Options {
    fn from(o: Options) -> Self {
        let mut ret = pulldown_cmark::Options::empty();
        if o.get_tables() {
            ret.insert(pulldown_cmark::Options::ENABLE_TABLES);
        }
        if o.get_footnotes() {
            ret.insert(pulldown_cmark::Options::ENABLE_FOOTNOTES);
        }
        if o.get_strikethrough() {
            ret.insert(pulldown_cmark::Options::ENABLE_STRIKETHROUGH);
        }
        if o.get_tasklist() {
            ret.insert(pulldown_cmark::Options::ENABLE_TASKLISTS);
        }
        if o.get_smart_punctuation() {
            ret.insert(pulldown_cmark::Options::ENABLE_SMART_PUNCTUATION);
        }
        if o.get_heading_attributes() {
            ret.insert(pulldown_cmark::Options::ENABLE_HEADING_ATTRIBUTES);
        }
        ret
    }
}

#[wasm_bindgen]
pub struct Converter {
    options: pulldown_cmark::Options,
    md: String,
}

#[wasm_bindgen]
impl Converter {
    pub fn new(ops: Options, md: String) -> Self {
        Self {
            options: pulldown_cmark::Options::from(ops),
            md
        }
    }

    pub fn out(&self) -> String {
        let parser = Parser::new_ext(self.md.as_str(), self.options);
        let mut bytes = Vec::new();
        write_html(Cursor::new(&mut bytes), parser).unwrap();
        String::from_utf8_lossy(&bytes).to_string()
    }
}


#[wasm_bindgen]
pub fn attach_md_to_element(ops: Options, md: String, el: &Element) {
    let converter = Converter::new(ops, md);
    let out = converter.out();
    el.set_inner_html(&out);
}