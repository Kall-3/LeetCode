use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut map: HashMap<char, usize> = HashMap::new();
        let mut res = 0;

        let mut left = 0;

        for (i, c) in s.chars().enumerate() {
            if map.contains_key(&c) {
                left = left.max(map.insert(c, i).unwrap() + 1);
            } else {
                map.insert(c, i);
            }
            res = res.max(i - left + 1);
        }

        res as i32
    }
}