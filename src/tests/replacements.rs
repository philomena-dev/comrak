use super::*;

#[test]
fn image_mention() {
    html_opts_no_roundtrip(
        "hello world >>1234p >>1337",
        "<p>hello world <div id=\"1234\">p</div> &gt;&gt;1337</p>\n",
        |opts| {
            let mut replacements = HashMap::new();
            replacements.insert("1234p".to_string(), "<div id=\"1234\">p</div>".to_string());
            opts.extension.replacements = Some(replacements);
        },
    );
}

#[test]
fn image_mention_first_nonspace() {
    html_opts_no_roundtrip(">>1234p", "<p><div id=\"1234\">p</div></p>\n", |opts| {
        let mut replacements = HashMap::new();
        replacements.insert("1234p".to_string(), "<div id=\"1234\">p</div>".to_string());
        opts.extension.greentext = true;
        opts.extension.replacements = Some(replacements);
    });
}
