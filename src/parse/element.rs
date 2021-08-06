use std::collections::HashMap;

use crate::ast::NodeData;
use crate::Cfg;
use crate::gen::codepoints::{ATTR_QUOTE, DOUBLE_QUOTE, NOT_UNQUOTED_ATTR_VAL_CHAR, SINGLE_QUOTE, TAG_NAME_CHAR, WHITESPACE, WHITESPACE_OR_SLASH};
use crate::parse::Code;
use crate::parse::content::{parse_content, ParsedContent};
use crate::parse::script::parse_script_content;
use crate::parse::style::parse_style_content;
use crate::parse::textarea::parse_textarea_content;
use crate::spec::tag::ns::Namespace;
use crate::spec::tag::void::VOID_TAGS;

fn parse_tag_name(code: &mut Code) -> Vec<u8> {
    debug_assert!(code.str().starts_with(b"<"));
    code.shift(1);
    code.shift_if_next(b'/');
    let mut name = code.copy_and_shift_while_in_lookup(TAG_NAME_CHAR);
    name.make_ascii_lowercase();
    name
}

pub fn peek_tag_name(code: &mut Code) -> Vec<u8> {
    let cp = code.take_checkpoint();
    let name = parse_tag_name(code);
    code.restore_checkpoint(cp);
    name
}

pub struct ParsedTag {
    attributes: HashMap<Vec<u8>, Vec<u8>>,
    name: Vec<u8>,
    self_closing: bool,
}

// While not valid, attributes in closing tags still need to be parsed (and then discarded) as attributes e.g. `</div x=">">`, which is why this function is used for both opening and closing tags.
// TODO Use generics to create version that doesn't create a HashMap.
pub fn parse_tag(code: &mut Code) -> ParsedTag {
    let mut elem_name = parse_tag_name(code);
    let mut attributes = HashMap::<Vec<u8>, Vec<u8>>::new();
    let mut self_closing = false;
    loop {
        // At the beginning of this loop, the last parsed unit was either the tag name or an attribute (including its value, if it had one).
        let last = code.shift_while_in_lookup(WHITESPACE_OR_SLASH);
        if code.at_end() || code.shift_if_next(b'>') {
            self_closing = last.filter(|&c| c == b'/').is_some();
            // End of tag.
            break;
        };
        let mut attr_name = code.copy_and_shift_while_not_in_lookup(WHITESPACE_OR_SLASH);
        attr_name.make_ascii_lowercase();
        // See comment for WHITESPACE_OR_SLASH in codepoints.ts for details of complex attr parsing.
        code.shift_while_in_lookup(WHITESPACE);
        let has_value = code.shift_if_next(b'=');
        code.shift_while_in_lookup(WHITESPACE);
        let attr_value = if !has_value {
            Vec::new()
        } else {
            let attr_delim = code.shift_if_next_in_lookup(ATTR_QUOTE);
            // It seems that for unquoted attribute values, if it's the last value in a tag and is immediately followed by `>`, any trailing `/` is NOT interpreted as a self-closing indicator and is always included as part of the value, even for SVG self-closable elements.
            let attr_delim_pred = match attr_delim {
                Some(b'"') => DOUBLE_QUOTE,
                Some(b'\'') => SINGLE_QUOTE,
                None => NOT_UNQUOTED_ATTR_VAL_CHAR,
                _ => unreachable!(),
            };
            let attr_value = code.copy_and_shift_while_not_in_lookup(attr_delim_pred);
            if let Some(c) = attr_delim {
                // It might not be next if EOF (i.e. attribute value not closed).
                code.shift_if_next(c);
            };
            attr_value
        };
        attributes.insert(attr_name, attr_value);
    };
    ParsedTag {
        attributes,
        name: elem_name,
        self_closing,
    }
}

// `<` or `</` must be next. If `</` is next, tag is reinterpreted as opening tag (i.e. `/` is ignored).
// `parent` should be an empty slice if it doesn't exist.
pub fn parse_element(cfg: &Cfg, code: &mut Code, ns: Namespace, parent: &[u8]) -> NodeData {
    let ParsedTag {
        name: elem_name,
        attributes,
        self_closing,
    } = parse_tag(code);

    // See spec for more details.
    if self_closing && ns != Namespace::Html || VOID_TAGS.contains(elem_name.as_slice()) {
        return NodeData::Element {
            attributes,
            children: Vec::new(),
            closing_tag_omitted: true,
            name: elem_name,
        };
    };

    let child_ns = if elem_name == b"svg" {
        Namespace::Svg
    } else {
        ns
    };

    let ParsedContent {
        mut closing_tag_omitted,
        children,
    } = match elem_name.as_slice() {
        b"script" => parse_script_content(cfg, code),
        b"style" => parse_style_content(cfg, code),
        b"textarea" => parse_textarea_content(cfg, code),
        _ => parse_content(cfg, code, child_ns, parent, &elem_name)
    };

    if !closing_tag_omitted {
        let closing_tag = parse_tag(code);
        debug_assert_eq!(closing_tag.name, elem_name);
    };

    NodeData::Element {
        attributes,
        children,
        closing_tag_omitted,
        name: elem_name,
    }
}
