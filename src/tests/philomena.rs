use super::*;

#[test]
fn paragraph_divs() {
    html_opts!(
        [extension.philomena],
        "hello world",
        "<div class=\"paragraph\">hello world</div>\n"
    );
}

#[test]
fn imgspoiler_spans() {
    html_opts!(
        [extension.philomena],
        concat!("![full](http://example.com/example.png)"),
        concat!("<div class=\"paragraph\"><span class=\"imgspoiler\"><img src=\"http://example.com/example.png\" alt=\"full\" /></span></div>\n"),
    );
}
