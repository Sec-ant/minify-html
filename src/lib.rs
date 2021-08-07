pub use crate::cfg::Cfg;
use crate::minify::content::minify_content;
use crate::parse::content::parse_content;
use crate::parse::Code;
use crate::spec::tag::ns::Namespace;
use crate::spec::tag::EMPTY_SLICE;

mod ast;
mod cfg;
mod gen;
mod minify;
mod parse;
mod pattern;
mod spec;
#[cfg(test)]
mod tests;
mod whitespace;

/// Copies a slice into a new Vec and minifies it, returning the Vec.
/// The resulting Vec will only contain minified code.
///
/// # Arguments
///
/// * `code` - A slice of bytes representing the source code to minify.
/// * `cfg` - Configuration object to adjust minification approach.
///
/// # Examples
///
/// ```
/// use minify_html::{Cfg, minify};
///
/// let mut code: &[u8] = b"<p>  Hello, world!  </p>";
/// let mut cfg = Cfg::new();
/// cfg.keep_comments = true;
/// let minified = minify(&code, &cfg);
/// assert_eq!(minified, b"<p>Hello, world!".to_vec());
/// ```
pub fn minify(src: &[u8], cfg: &Cfg) -> Vec<u8> {
    let mut code = Code::new(src);
    let parsed = parse_content(&mut code, Namespace::Html, EMPTY_SLICE, EMPTY_SLICE);
    let mut out = Vec::with_capacity(src.len());
    minify_content(cfg, &mut out, false, EMPTY_SLICE, parsed.children);
    out
}
