use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use lol_alloc::{FreeListAllocator, LockedAllocator};

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOCATOR: LockedAllocator<FreeListAllocator> =
    LockedAllocator::new(FreeListAllocator::new());

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Config {
    #[tsify(optional)]
    pub do_not_minify_doctype: Option<bool>,
    #[tsify(optional)]
    pub ensure_spec_compliant_unquoted_attribute_values: Option<bool>,
    #[tsify(optional)]
    pub keep_closing_tags: Option<bool>,
    #[tsify(optional)]
    pub keep_html_and_head_opening_tags: Option<bool>,
    #[tsify(optional)]
    pub keep_spaces_between_attributes: Option<bool>,
    #[tsify(optional)]
    pub keep_comments: Option<bool>,
    #[tsify(optional)]
    pub minify_css: Option<bool>,
    #[tsify(optional)]
    pub minify_css_level_1: Option<bool>,
    #[tsify(optional)]
    pub minify_css_level_2: Option<bool>,
    #[tsify(optional)]
    pub minify_css_level_3: Option<bool>,
    #[tsify(optional)]
    pub minify_js: Option<bool>,
    #[tsify(optional)]
    pub remove_bangs: Option<bool>,
    #[tsify(optional)]
    pub remove_processing_instructions: Option<bool>,
}

#[wasm_bindgen]
pub fn minify(code: &[u8], config: Config) -> Vec<u8> {
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
