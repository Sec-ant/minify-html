use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use lol_alloc::{FreeListAllocator, LockedAllocator};

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOCATOR: LockedAllocator<FreeListAllocator> =
    LockedAllocator::new(FreeListAllocator::new());

#[wasm_bindgen]
pub struct Config {
    pub do_not_minify_doctype: bool,
    pub ensure_spec_compliant_unquoted_attribute_values: bool,
    pub keep_closing_tags: bool,
    pub keep_html_and_head_opening_tags: bool,
    pub keep_spaces_between_attributes: bool,
    pub keep_comments: bool,
    pub minify_css: bool,
    pub minify_css_level_1: bool,
    pub minify_css_level_2: bool,
    pub minify_css_level_3: bool,
    pub minify_js: bool,
    pub remove_bangs: bool,
    pub remove_processing_instructions: bool,
}

#[wasm_bindgen]
pub fn minify(code: &[u8], config: &Config) -> Vec<u8> {
    let cfg = minify_html::Cfg {
        do_not_minify_doctype: config.do_not_minify_doctype,
        ensure_spec_compliant_unquoted_attribute_values: config
            .ensure_spec_compliant_unquoted_attribute_values,
        keep_closing_tags: config.keep_closing_tags,
        keep_html_and_head_opening_tags: config.keep_html_and_head_opening_tags,
        keep_spaces_between_attributes: config.keep_spaces_between_attributes,
        keep_comments: config.keep_comments,
        minify_css: config.minify_css,
        minify_css_level_1: config.minify_css_level_1,
        minify_css_level_2: config.minify_css_level_2,
        minify_css_level_3: config.minify_css_level_3,
        minify_js: config.minify_js,
        remove_bangs: config.remove_bangs,
        remove_processing_instructions: config.remove_processing_instructions,
    };
    minify_html::minify(code, &cfg)
}
