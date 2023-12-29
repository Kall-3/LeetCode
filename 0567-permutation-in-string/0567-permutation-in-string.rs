use std::collections::HashMap;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s1.len() > s2.len() { return false }

        let mut mapping: [i16; 26] = [0; 26];
        
        s1.as_bytes().iter().for_each(|&x| mapping[(x - b'a') as usize] -= 1);
        s2[0..s1.len()].as_bytes().iter().for_each(|&x| mapping[(x - b'a') as usize] += 1);

        if mapping.iter().all(|&x| x == 0) { return true }

        let s2 = s2.as_bytes();

        for r in s1.len()..s2.len() {
            mapping[(s2[r] -b'a') as usize] += 1;
            mapping[(s2[r - s1.len()] -b'a') as usize] -= 1;

            if mapping.iter().all(|&x| x == 0) { return true }
        }

        false
    }
}