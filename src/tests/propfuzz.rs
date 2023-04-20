use crate::*;

#[cfg(not(target_arch = "wasm32"))]
use propfuzz::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
#[propfuzz]
fn propfuzz_doesnt_crash(md: String) {
    let options = Options {
        extension: ExtensionOptions {
            strikethrough: true,
            tagfilter: true,
            table: true,
            autolink: true,
            tasklist: true,
            superscript: true,
            philomena: true,
            philomena_replacements: None,
            header_ids: Some("user-content-".to_string()),
            footnotes: true,
            description_lists: true,
            multiline_block_quotes: true,
            math_dollars: true,
            math_code: true,
            front_matter_delimiter: None,
            #[cfg(feature = "shortcodes")]
            shortcodes: true,
        },
        parse: ParseOptions {
            smart: true,
            default_info_string: Some("Rust".to_string()),
            relaxed_tasklist_matching: true,
            relaxed_autolinks: true,
        },
        render: RenderOptions {
            hardbreaks: true,
            github_pre_lang: true,
            full_info_string: true,
            width: 80,
            unsafe_: true,
            escape: false,
            list_style: ListStyleType::Dash,
            sourcepos: true,
            escaped_char_spans: true,
        },
    };

    parse_document(&Arena::new(), &md, &options);
}
