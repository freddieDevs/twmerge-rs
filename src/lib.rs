#![allow(unused)]
mod validators;
mod types;
mod default_config;
mod from_theme;

#[cfg(test)]
mod tests {
    use validators::is_length;

    use super::*;
    fn test_is_length() {
        assert_eq!(is_length("1"), true);
        assert_eq!(is_length("1023713"), true);
        assert_eq!(is_length("1.5"), true);
        assert_eq!(is_length("1145.67597"), true);
        assert_eq!(is_length("px"), true);
        assert_eq!(is_length("full"), true);
        assert_eq!(is_length("screen"), true);
        assert_eq!(is_length("3/4"), true);
        assert_eq!(is_length("25/66"), true);

        assert_eq!(is_length("[6.9%]"), false);
        assert_eq!(is_length("[486px]"), false);
        assert_eq!(is_length("[45.5rem]"), false);
        assert_eq!(is_length("[57vw]"), false);
        assert_eq!(is_length("[75vh]"), false);
        assert_eq!(is_length("[length:var(--arbitrary)]"), false);
        assert_eq!(is_length("5t7"), false);
        assert_eq!(is_length("[1]"), false);
        assert_eq!(is_length("[56px]"), false);
        assert_eq!(is_length("65px]"), false);
        assert_eq!(is_length("one"), false);
    }
}