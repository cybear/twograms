pub fn parse_file(s: &str) -> Vec<&str> {
    s.split(|c: char| c.is_ascii_punctuation())
        .map(str::trim)
        .filter(|sentence| !sentence.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    static TESTDATA: &str = "I am a fish. No, wait, I am a plant.";

    #[test]
    fn test_parse_file() {
        let sentences = parse_file(TESTDATA);
        assert_eq!(sentences[0], "I am a fish");
        assert_eq!(sentences[1], "No");
        assert_eq!(sentences[2], "wait");
        assert_eq!(sentences[3], "I am a plant");
    }
}
