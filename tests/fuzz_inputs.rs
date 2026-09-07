use cld2::{Format, Hints, Lang, detect_language_ext};

fn run(text: &str, hint: Option<&str>) {
    let hints = Hints {
        content_language: hint,
        ..Default::default()
    };
    let _ = detect_language_ext(text, Format::Text, &hints);
}

#[test]
fn empty_text_with_hint() {
    for _ in 0..32 {
        run("", Some("en"));
    }
}

#[test]
fn empty_text_no_hint() {
    for _ in 0..32 {
        run("", None);
    }
}

#[test]
fn one_byte_ascii() {
    for _ in 0..32 {
        run("a", Some("en"));
    }
}

#[test]
fn one_byte_no_hint() {
    for _ in 0..32 {
        run("a", None);
    }
}

#[test]
fn whitespace_only() {
    for _ in 0..32 {
        run("   \n\t ", Some("en"));
    }
}

#[test]
fn html_tags_in_text_mode() {
    for _ in 0..32 {
        run("<p>hello <b>world</b></p>", Some("en"));
    }
}

#[test]
fn invalid_utf8ish_high_bytes() {
    for _ in 0..32 {
        run("\u{FFFD}\u{FFFD}\u{FFFD}", Some("en"));
    }
}

#[test]
fn long_repeated_ascii() {
    let text = "the quick brown fox jumps over the lazy dog ".repeat(2000);
    for _ in 0..8 {
        run(&text, Some("en"));
    }
}

#[test]
fn cjk_text() {
    for _ in 0..32 {
        run("これは日本語のテストです。", Some("ja"));
    }
}

#[test]
fn rtl_text() {
    for _ in 0..32 {
        run("هذا اختبار باللغة العربية", Some("ar"));
    }
}

#[test]
fn mixed_scripts() {
    for _ in 0..32 {
        run("hello 你好 مرحبا привет", Some("en"));
    }
}

#[test]
fn unknown_hint_lang() {
    for _ in 0..32 {
        run("hello world", Some("xx"));
    }
}

#[test]
fn empty_hint_string() {
    for _ in 0..32 {
        run("hello world", Some(""));
    }
}

#[test]
fn language_hint_struct() {
    for _ in 0..32 {
        let hints = Hints {
            language: Some(Lang("en")),
            ..Default::default()
        };
        let _ = detect_language_ext("hello world", Format::Text, &hints);
    }
}

#[test]
fn tld_hint() {
    for _ in 0..32 {
        let hints = Hints {
            tld: Some("jp"),
            ..Default::default()
        };
        let _ = detect_language_ext("hello world", Format::Text, &hints);
    }
}

#[test]
fn all_hints_at_once() {
    for _ in 0..32 {
        let hints = Hints {
            content_language: Some("en"),
            tld: Some("us"),
            language: Some(Lang("en")),
            ..Default::default()
        };
        let _ = detect_language_ext("hello world", Format::Text, &hints);
    }
}
