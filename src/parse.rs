pub fn parse_file<'a>(s: &'a str) -> Vec<&'a str> {
    s.split(|c: char| c.is_ascii_punctuation())
        .map(|sentence| sentence.trim())
        .filter(|sentence| sentence.len() > 0)
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
