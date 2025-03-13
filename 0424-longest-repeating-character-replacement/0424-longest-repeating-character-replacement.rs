use std::collections::HashMap;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {

        let s: Vec<char> = s.chars().into_iter().collect();

        let mut map = [0; 26];
        let mut left = 0;
        let mut max_count = 0;
        let mut max = 0;

        for (right, c) in s.iter().enumerate() {

            map[*c as usize - 'A' as usize] += 1;
            max_count = max_count.max(map[*c as usize - 'A' as usize]);

            while right + 1 - left - max_count > k as usize {
                map[s[left] as usize - 'A' as usize] -= 1;
                left += 1;
            }

            max = max.max(right + 1 - left);
        }

        max as i32
    }
}