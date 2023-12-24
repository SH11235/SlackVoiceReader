use regex::Regex;

pub fn replace_url_with_text(input: &str) -> String {
    let url_regex = Regex::new(
        r"http[s]?://(?:[a-zA-Z]|[0-9]|[$-_@.&+]|[!*\\(\\),]|(?:%[0-9a-fA-F][0-9a-fA-F]))+",
    )
    .unwrap();
    let result = url_regex.replace_all(input, "URL");
    result.into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_url_with_text() {
        let input = "Check out this link: https://example.com";
        let expected = "Check out this link: URL";
        let result = replace_url_with_text(input);
        assert_eq!(result, expected);

        let input_no_space = "https://example.com is a URL";
        let expected_no_space = "URL is a URL";
        let result_no_space = replace_url_with_text(input_no_space);
        assert_eq!(result_no_space, expected_no_space);

        let input_newline = "URLs:\nhttps://example.com\nhttps://example.org";
        let expected_newline = "URLs:\nURL\nURL";
        let result_newline = replace_url_with_text(input_newline);
        assert_eq!(result_newline, expected_newline);

        let input_japanese = "これはテストです\nhttps://www.google.co.jp\nこれもテストです";
        let expected_japanese = "これはテストです\nURL\nこれもテストです";
        let result_japanese = replace_url_with_text(input_japanese);
        assert_eq!(result_japanese, expected_japanese);
    }
}
