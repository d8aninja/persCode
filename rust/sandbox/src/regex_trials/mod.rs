use regex::Regex;

pub fn regex_em() {
    let re = Regex::new(r"[a-z]+(?:([0-9]+)|([A-Z]+))").unwrap();
    let caps = re.captures("abc123").unwrap();

    let text1 = caps.get(1).map_or("", |m| m.as_str());
    let text2 = caps.get(2).map_or("", |m| m.as_str());

    assert_eq!(text1, "123");
    assert_eq!(text2, "");
}