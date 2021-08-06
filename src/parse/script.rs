use aho_corasick::AhoCorasick;
use aho_corasick::AhoCorasickBuilder;
use lazy_static::lazy_static;

use crate::ast::NodeData;
use crate::Cfg;
use crate::parse::Code;
use crate::parse::content::ParsedContent;

lazy_static! {
    static ref END: AhoCorasick = AhoCorasickBuilder::new()
        .ascii_case_insensitive(true)
        .build(&["</script"]);
}

pub fn parse_script_content(cfg: &Cfg, code: &mut Code) -> ParsedContent {
    let (len, closing_tag_omitted) = match END.find(code.str()) {
        Some(m) => (m.start(), false),
        None => (code.rem(), true),
    };
    ParsedContent {
        closing_tag_omitted,
        children: vec![NodeData::ScriptOrStyleContent { code: code.copy_and_shift(len) }],
    }
}
