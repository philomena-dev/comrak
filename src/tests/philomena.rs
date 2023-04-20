use super::*;

#[test]
fn subscript() {
    html_opts!(
        [extension.philomena],
        concat!("e = mc%2%.\n"),
        concat!("<p>e = mc<sub>2</sub>.</p>\n"),
    );
}

#[test]
fn spoiler() {
    html_opts!(
        [extension.philomena],
        concat!("The ||dog dies at the end of Marley and Me||.\n"),
        concat!("<p>The <span class=\"spoiler\">dog dies at the end of Marley and Me</span>.</p>\n"),
    );
}

#[test]
fn underline() {
    html_opts!(
        [extension.philomena],
        concat!("__underlined__\n"),
        concat!("<p><ins>underlined</ins></p>\n"),
    );
}
