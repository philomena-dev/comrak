use super::*;

#[test]
fn image_mention() {
    html_opts_i(
        "hello world >>1234p >>1337",
        "<p>hello world <div id=\"1234\">p</div> &gt;&gt;1337</p>\n",
        true,
        |opts| {
            let mut replacements = HashMap::new();
            replacements.insert("1234p".to_string(), "<div id=\"1234\">p</div>".to_string());
            opts.extension.replacements = Some(replacements);
        },
    );
}

#[test]
fn image_mention_id_termination() {
    html_opts_i(
        ">>123t4",
        "<p><div id=\"123\">t</div>4</p>\n",
        true,
        |opts| {
            let mut replacements = HashMap::new();
            replacements.insert("123t".to_string(), "<div id=\"123\">t</div>".to_string());
            opts.extension.greentext = true;
            opts.extension.replacements = Some(replacements);
        },
    );
}

#[test]
fn image_mention_id_only() {
    html_opts_i(
        ">>1234 5678",
        "<p><div id=\"1234\"></div> 5678</p>\n",
        true,
        |opts| {
            let mut replacements = HashMap::new();
            replacements.insert("1234".to_string(), "<div id=\"1234\"></div>".to_string());
            opts.extension.greentext = true;
            opts.extension.replacements = Some(replacements);
        },
    );
}

#[test]
fn image_mention_fail_blockquote() {
    html_opts_i("> ", "<blockquote>\n</blockquote>\n", true, |opts| {
        opts.extension.greentext = true;
        opts.extension.replacements = Some(HashMap::new())
    });
}

#[test]
fn image_mention_fail_empty() {
    html_opts_i(">>", "<p>&gt;&gt;</p>\n", true, |opts| {
        opts.extension.greentext = true;
        opts.extension.replacements = Some(HashMap::new())
    });
}

#[test]
fn image_mention_fail_improper_character() {
    html_opts_i(">>#", "<p>&gt;&gt;#</p>\n", true, |opts| {
        opts.extension.greentext = true;
        opts.extension.replacements = Some(HashMap::new())
    });
}

#[test]
fn image_mention_first_nonspace() {
    html_opts_i(
        ">>1234p",
        "<p><div id=\"1234\">p</div></p>\n",
        true,
        |opts| {
            let mut replacements = HashMap::new();
            replacements.insert("1234p".to_string(), "<div id=\"1234\">p</div>".to_string());
            opts.extension.greentext = true;
            opts.extension.replacements = Some(replacements);
        },
    );
}
