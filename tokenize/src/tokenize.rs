use regex::Regex;
pub fn words(s: String) -> String {
    let s1 = Regex::new(r#""|—|\]|\[|\.|\!|\,|\:|\;|\)|\(|\&|\#|\?|»|«|\/"#).unwrap().replace_all(&s, "");
    let s2 = Regex::new(r#"\t\r\n\f\v"#).unwrap().replace_all(&s1, "");
    let s3 = s2.to_lowercase();
    return s3;
}