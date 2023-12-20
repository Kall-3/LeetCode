impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let word: Vec<_> = s.chars()
                            .filter_map(|c| c.is_ascii_alphanumeric().then(|| c.to_lowercase().to_string() ))
                            .collect();

        word.iter().eq(word.iter().rev())
    }
}