use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use lol_alloc::{FreeListAllocator, LockedAllocator};

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOCATOR: LockedAllocator<FreeListAllocator> =
    LockedAllocator::new(FreeListAllocator::new());

#[wasm_bindgen]
pub struct Config {
    pub do_not_minify_doctype: Option<bool>,
    pub ensure_spec_compliant_unquoted_attribute_values: Option<bool>,
    pub keep_closing_tags: Option<bool>,
    pub keep_html_and_head_opening_tags: Option<bool>,
    pub keep_spaces_between_attributes: Option<bool>,
    pub keep_comments: Option<bool>,
    pub minify_css: Option<bool>,
    pub minify_css_level_1: Option<bool>,
    pub minify_css_level_2: Option<bool>,
    pub minify_css_level_3: Option<bool>,
    pub minify_js: Option<bool>,
    pub remove_bangs: Option<bool>,
    pub remove_processing_instructions: Option<bool>,
}

#[wasm_bindgen]
pub fn minify(code: &[u8], config: &Config) -> Vec<u8> {
    let cfg = minify_html::Cfg {
        do_not_minify_doctype: config.do_not_minify_doctype.unwrap_or(false),
        ensure_spec_compliant_unquoted_attribute_values: config
            .ensure_spec_compliant_unquoted_attribute_values
            .unwrap_or(false),
        keep_closing_tags: config.keep_closing_tags.unwrap_or(false),
        keep_html_and_head_opening_tags: config.keep_html_and_head_opening_tags.unwrap_or(false),
        keep_spaces_between_attributes: config.keep_spaces_between_attributes.unwrap_or(false),
        keep_comments: config.keep_comments.unwrap_or(false),
        minify_css: config.minify_css.unwrap_or(false),
        minify_css_level_1: config.minify_css_level_1.unwrap_or(false),
        minify_css_level_2: config.minify_css_level_2.unwrap_or(false),
        minify_css_level_3: config.minify_css_level_3.unwrap_or(false),
        minify_js: config.minify_js.unwrap_or(false),
        remove_bangs: config.remove_bangs.unwrap_or(false),
        remove_processing_instructions: config.remove_processing_instructions.unwrap_or(false),
    };
    minify_html::minify(code, &cfg)
}
