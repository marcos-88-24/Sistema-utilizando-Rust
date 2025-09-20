use regex::Regex;

pub fn tokenizar(txt: &str) -> Vec<String> {
    let re = Regex::new(r"[A-Za-z0-9À-ÿ]+").unwrap();
    re.find_iter(txt)
        .map(|m| m.as_str().to_lowercase())
        .collect()
}
