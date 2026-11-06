use super::*;

#[test]
fn greentext_preserved() {
    html_opts!(
        [extension.greentext, render.hardbreaks],
        ">implying\n>>implying",
        "<p>&gt;implying<br />\n&gt;&gt;implying</p>\n"
    );
}

#[test]
fn greentext_preserved_no_guillemets() {
    html_opts!(
        [extension.greentext, render.hardbreaks, parse.smart],
        " >implying\n >>implying << >>not implying",
        "<p>&gt;implying<br />\n&gt;&gt;implying &lt;&lt; &gt;&gt;not implying</p>\n"
    );
}

#[test]
fn empty_line() {
    html_opts!([extension.greentext], ">", "<p>&gt;</p>\n");
}

#[test]
fn separate_quotes_on_line_end() {
    html_opts!(
        [extension.greentext],
        "> 1\n>\n> 2",
        "<blockquote>\n<p>1</p>\n<p>2</p>\n</blockquote>\n"
    );
}

#[test]
fn unnest_quotes_on_line_end() {
    html_opts!(
        [extension.greentext],
        "> 1\n> > 2\n> 1",
        "<blockquote>\n<p>1</p>\n<blockquote>\n<p>2</p>\n</blockquote>\n<p>1</p>\n</blockquote>\n"
    );
}

#[test]
fn unnest_quotes_on_line_end_commonmark() {
    html_opts!(
        [extension.greentext],
        "> 1\n> > 2\n> \n> 1",
        "<blockquote>\n<p>1</p>\n<blockquote>\n<p>2</p>\n</blockquote>\n<p>1</p>\n</blockquote>\n"
    );
}

#[test]
fn greentext_disabled_blockquote() {
    assert_ast_match!(
        [],
        "> First line.\n> The second sentence in the first line.\n>\n> Second line.",
        (document (1:1-4:14) [
            (block_quote (1:1-4:14) [
                (paragraph (1:3-2:40) [
                    (text (1:3-1:13) "First line.")
                    (softbreak (1:14-1:14))
                    (text (2:3-2:40) "The second sentence in the first line.")
                ])
                (paragraph (4:3-4:14) [
                    (text (4:3-4:14) "Second line.")
                ])
            ])
        ])
    );
}

#[test]
fn greentext_enabled_blockquote() {
    assert_ast_match!(
        [extension.greentext],
        "> First line.\n> The second sentence in the first line.\n>\n> Second line.",
        (document (1:1-4:14) [
            (block_quote (1:1-4:14) [
                (paragraph (1:3-2:40) [
                    (text (1:3-1:13) "First line.")
                    (softbreak (1:14-1:14))
                    (text (2:3-2:40) "The second sentence in the first line.")
                ])
                (paragraph (4:3-4:14) [
                    (text (4:3-4:14) "Second line.")
                ])
            ])
        ])
    );
}

#[test]
fn greentext_disabled_blockquote_alerts() {
    html_opts!(
        [extension.alerts],
        "> [!Note]\n> This is the first line.\n>\n> Second line.",
        concat!(
            "<div class=\"markdown-alert markdown-alert-note\">\n",
            "<p class=\"markdown-alert-title\">Note</p>\n",
            "<p>This is the first line.</p>\n",
            "<p>Second line.</p>\n",
            "</div>\n",
        ),
    );
}

#[test]
fn greentext_enabled_blockquote_alerts() {
    html_opts!(
        [extension.greentext, extension.alerts],
        "> [!Note]\n> This is the first line.\n>\n> Second line.",
        concat!(
            "<div class=\"markdown-alert markdown-alert-note\">\n",
            "<p class=\"markdown-alert-title\">Note</p>\n",
            "<p>This is the first line.</p>\n",
            "<p>Second line.</p>\n",
            "</div>\n",
        ),
    );
}

#[test]
fn greentext_and_embed_after_quote_closes_the_quote() {
    html_opts_i(
        "> quote\n>>1p\nreply",
        "<blockquote>\n<p>quote</p>\n</blockquote>\n<p><div id=\"1\">p</div><br />\nreply</p>\n",
        false,
        |opts| {
            let mut replacements = HashMap::new();
            replacements.insert("1p".to_string(), "<div id=\"1\">p</div>".to_string());

            opts.extension.greentext = true;
            opts.extension.replacements = Some(replacements);
            opts.render.hardbreaks = true;
        },
    );

    html_opts!(
        [extension.greentext],
        "> quote\n>implying",
        "<blockquote>\n<p>quote</p>\n</blockquote>\n<p>&gt;implying</p>\n",
    );

    html_opts!(
        [extension.greentext],
        "> a\n>> b",
        "<blockquote>\n<p>a</p>\n</blockquote>\n<p>&gt;&gt; b</p>\n",
    );

    html_opts!(
        [extension.greentext, render.hardbreaks],
        "> quote\n>\treply",
        "<blockquote>\n<p>quote<br />\nreply</p>\n</blockquote>\n",
    );
}
