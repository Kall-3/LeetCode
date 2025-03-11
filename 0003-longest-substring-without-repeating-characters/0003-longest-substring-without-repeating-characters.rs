use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {

        let mut map: HashMap<char, usize> = HashMap::new();

        let mut left = 0;
        let mut max = 0;

        for (i, c) in s.chars().enumerate() {
            if let Some(j) = map.insert(c, i) {
                left = left.max(j + 1);
            }

            max = max.max(i + 1 - left);
        }

        max as i32
    }
}