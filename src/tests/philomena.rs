use super::*;

#[test]
fn subscript() {
    html_opts!(
        [extension.philomena],
        concat!("e = mc%2%.\n"),
        concat!("<div class=\"paragraph\">e = mc<sub>2</sub>.</div>\n"),
    );
}

#[test]
fn spoiler() {
    html_opts!(
        [extension.philomena],
        concat!("The ||dog dies at the end of Marley and Me||.\n"),
        concat!("<div class=\"paragraph\">The <span class=\"spoiler\">dog dies at the end of Marley and Me</span>.</div>\n"),
    );
}

#[test]
fn spoiler_in_table() {
    html_opts!(
        [extension.table, extension.philomena],
        concat!("Text | Result\n--- | ---\n`||some clever text||` | ||some clever text||\n"),
        concat!(
            "<table>\n",
            "<thead>\n",
            "<tr>\n",
            "<th>Text</th>\n",
            "<th>Result</th>\n",
            "</tr>\n",
            "</thead>\n",
            "<tbody>\n",
            "<tr>\n",
            "<td><code>||some clever text||</code></td>\n",
            "<td><span class=\"spoiler\">some clever text</span></td>\n",
            "</tr>\n",
            "</tbody>\n",
            "</table>\n"
        ),
    );
}

#[test]
fn spoiler_regressions() {
    html_opts!(
        [extension.philomena],
        concat!("|should not be spoiler|\n||should be spoiler||\n|||should be spoiler surrounded by pipes|||"),
        concat!(
            "<div class=\"paragraph\">|should not be spoiler|\n",
            "<span class=\"spoiler\">should be spoiler</span>\n",
            "|<span class=\"spoiler\">should be spoiler surrounded by pipes</span>|</div>\n"
        ),
    );
}

#[test]
fn mismatched_spoilers() {
    html_opts!(
        [extension.philomena],
        concat!("|||this is a spoiler with pipe in front||\n||this is not a spoiler|\n||this is a spoiler with pipe after|||"),
        concat!(
            "<div class=\"paragraph\">|<span class=\"spoiler\">this is a spoiler with pipe in front</span>\n",
            "||this is not a spoiler|\n",
            "<span class=\"spoiler\">this is a spoiler with pipe after</span>|</div>\n"
        ),
    );
}

#[test]
fn underline() {
    html_opts!(
        [extension.philomena],
        concat!("__underlined__\n"),
        concat!("<div class=\"paragraph\"><ins>underlined</ins></div>\n"),
    );
}

#[test]
fn no_setext_headings_in_philomena() {
    html_opts!(
        [extension.philomena],
        concat!("text text\n---"),
        concat!("<div class=\"paragraph\">text text</div>\n<hr />\n"),
    );
}

#[test]
fn greentext_preserved() {
    html_opts!(
        [render.hardbreaks],
        ">implying\n>>implying",
        "<p>&gt;implying<br />\n&gt;&gt;implying</p>\n"
    );
}

#[test]
fn separate_quotes_on_line_end() {
    html(
        "> 1\n>\n> 2",
        "<blockquote>\n<p>1</p>\n</blockquote>\n<p>&gt;</p>\n<blockquote>\n<p>2</p>\n</blockquote>\n"
    );
}

#[test]
fn unnest_quotes_on_line_end() {
    html(
        "> 1\n> > 2\n> 1",
        "<blockquote>\n<p>1</p>\n<blockquote>\n<p>2</p>\n</blockquote>\n<p>1</p>\n</blockquote>\n",
    );
}

#[test]
fn unnest_quotes_on_line_end_commonmark() {
    html(
        "> 1\n> > 2\n> \n> 1",
        "<blockquote>\n<p>1</p>\n<blockquote>\n<p>2</p>\n</blockquote>\n<p>1</p>\n</blockquote>\n",
    );
}

#[test]
fn philomena_images() {
    html_opts!(
        [extension.philomena],
        concat!("![full](http://i.imgur.com/QqK1vq7.png)"),
        concat!("<div class=\"paragraph\"><span class=\"imgspoiler\"><img src=\"http://i.imgur.com/QqK1vq7.png\" alt=\"full\" /></span></div>\n"),
    );
}

#[test]
fn no_empty_link() {
    html(
        "[](https://example.com/evil.domain.for.seo.spam)",
        "<p>[](https://example.com/evil.domain.for.seo.spam)</p>\n",
    );

    html(
        "[    ](https://example.com/evil.domain.for.seo.spam)",
        "<p>[    ](https://example.com/evil.domain.for.seo.spam)</p>\n",
    );
}

#[test]
fn empty_image_allowed() {
    html(
        "![   ](https://example.com/evil.domain.for.seo.spam)",
        "<p><img src=\"https://example.com/evil.domain.for.seo.spam\" alt=\"   \" /></p>\n",
    );
}

#[test]
fn image_inside_link_allowed() {
    html(
        "[![](https://example.com/image.png)](https://example.com/)",
        "<p><a href=\"https://example.com/\"><img src=\"https://example.com/image.png\" alt=\"\" /></a></p>\n",
    );
}

#[test]
fn image_mention() {
    html_opts_no_roundtrip(
        "hello world >>1234p >>1337",
        "<div class=\"paragraph\">hello world <div id=\"1234\">p</div> &gt;&gt;1337</div>\n",
        |opts| {
            let mut replacements = HashMap::new();
            replacements.insert("1234p".to_string(), "<div id=\"1234\">p</div>".to_string());

            opts.extension.philomena = true;
            opts.extension.philomena_replacements = Some(replacements);
        },
    );
}

#[test]
fn auto_relative_links() {
    html_opts_no_roundtrip(
        "[some link text](https://example.com/some/path)",
        "<div class=\"paragraph\"><a href=\"/some/path\">some link text</a></div>\n",
        |opts| {
            opts.extension.autolink = true;
            opts.extension.philomena = true;
            opts.extension.philomena_domains = Some(vec![String::from("example.com")]);
        },
    );

    html_opts_no_roundtrip(
        "https://example.com/some/path",
        "<div class=\"paragraph\"><a href=\"/some/path\">https://example.com/some/path</a></div>\n",
        |opts| {
            opts.extension.autolink = true;
            opts.extension.philomena = true;
            opts.extension.philomena_domains = Some(vec![String::from("example.com")]);
        },
    );

    html_opts_no_roundtrip(
        "[some link text](https://example.com/some/path?parameter=aaaaaa&other_parameter=bbbbbb#id12345)",
        "<div class=\"paragraph\"><a href=\"/some/path?parameter=aaaaaa&amp;other_parameter=bbbbbb#id12345\">some link text</a></div>\n",
        |opts| {
            opts.extension.autolink = true;
            opts.extension.philomena = true;
            opts.extension.philomena_domains = Some(vec![String::from("example.com")]);
        },
    );

    html_opts_no_roundtrip(
        "https://example.com/some/path?parameter=aaaaaa&other_parameter=bbbbbb#id12345",
        "<div class=\"paragraph\"><a href=\"/some/path?parameter=aaaaaa&amp;other_parameter=bbbbbb#id12345\">https://example.com/some/path?parameter=aaaaaa&amp;other_parameter=bbbbbb#id12345</a></div>\n",
        |opts| {
            opts.extension.autolink = true;
            opts.extension.philomena = true;
            opts.extension.philomena_domains = Some(vec![String::from("example.com")]);
        },
    );
}
