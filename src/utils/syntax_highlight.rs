use anyhow::{Error, Ok};
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

pub fn syntax_highlight(lang: Option<&str>, code: &str) -> Result<String, Error> {
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let language = match lang {
        None | Some("") => "rs",
        Some(l) => l,
    };

    let syntax = ps.find_syntax_by_extension(language);

    return match syntax {
        None => anyhow::bail!("Please add the language to your code snippets"),
        Some(s) => {
            Ok(highlighted_html_for_string(code, &ps, s, &ts.themes["base16-ocean.dark"]).unwrap())
        }
    };
}

#[test]
fn creates_syntax() {
    assert!(syntax_highlight(Some("js"), "console.log(5)")
        .unwrap()
        .contains("<span style=\"color:#c0c5ce;\">console.</span>"));
}