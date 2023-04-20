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
        "<blockquote>\n<p>1</p>\n<blockquote>\n<p>2</p>\n</blockquote>\n<p>1</p>\n</blockquote>\n"
    );
}

#[test]
fn unnest_quotes_on_line_end_commonmark() {
    html(
        "> 1\n> > 2\n> \n> 1",
        "<blockquote>\n<p>1</p>\n<blockquote>\n<p>2</p>\n</blockquote>\n<p>1</p>\n</blockquote>\n"
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
fn image_mention() {
    html_opts_no_roundtrip(
        "hello world >>1234p >>1337",
        "<div class=\"paragraph\">hello world <div id=\"1234\">p</div> &gt;&gt;1337</div>\n",
        |mut opts| {
            let mut replacements = HashMap::new();
            replacements.insert("1234p".to_string(), "<div id=\"1234\">p</div>".to_string());

            opts.extension.philomena = true;
            opts.extension.philomena_replacements = Some(replacements);
        }
    );
}
