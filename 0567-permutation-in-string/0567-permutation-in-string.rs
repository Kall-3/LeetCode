use std::collections::HashMap;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let mut target = vec![0; 26];
        let len = s1.len();

        for c in s1.chars() {
            target[c as usize - 'a' as usize] -= 1;
        }
        for c in s2.chars().take(len) {
            target[c as usize - 'a' as usize] += 1;
        }
        if target.iter().all(|&c| c == 0)  {
            return true;
        }

        let chars: Vec<char> = s2.chars().collect();

        for (i, c) in s2.chars().enumerate().skip(len) {
            target[c as usize - 'a' as usize] += 1;
            target[chars[i - len] as usize - 'a' as usize] -= 1;

            if target.iter().all(|&c| c == 0) {
                return true;
            }
        }

        false
    }
}