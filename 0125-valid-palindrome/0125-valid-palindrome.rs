impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut s: Vec<char> = s.to_lowercase()
            .chars()
            .filter(|&a| a.is_alphanumeric())
            .collect();

        let (mut l, mut r) = (0, s.len());

        while l < r {
            if s[l] != s[r - 1] {
                return false;
            }
            l += 1;
            r -= 1;
        }

        true
    }
}