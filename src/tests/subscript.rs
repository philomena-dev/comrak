use super::*;

#[test]
fn subscript() {
    html_opts!(
        [extension.subscript],
        concat!("H~2~O\n"),
        concat!("<p>H<sub>2</sub>O</p>\n"),
    );
}

#[test]
fn subscript_alternate() {
    html_opts_i(
        concat!("H%2%O\n"),
        concat!("<p>H<sub>2</sub>O</p>\n"),
        false,
        |opts| opts.extension.alternate_subscript = true,
    );
}

#[test]
fn strikethrough_and_subscript() {
    html_opts!(
        [extension.subscript, extension.strikethrough],
        concat!("~~H~2~O~~\n"),
        concat!("<p><del>H<sub>2</sub>O</del></p>\n"),
    );
}

#[test]
fn no_strikethrough_when_only_subscript() {
    html_opts!(
        [extension.subscript],
        concat!("~~H~2~O~~\n"),
        concat!("<p>~~H<sub>2</sub>O~~</p>\n"),
    );
}
