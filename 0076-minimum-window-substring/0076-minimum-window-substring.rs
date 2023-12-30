use std::collections::HashMap;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if s.len() < t.len() { return "".to_string() }

        let s: Vec<char> = s.chars().collect();

        let mut tCount: HashMap<char, i32> = HashMap::with_capacity(52);
        let mut window: HashMap<char, i32> = HashMap::with_capacity(52);

        // Reference to needed chars in tCount
        t.chars().for_each(|x| *tCount.entry(x).or_default() += 1);

        let mut best_len = usize::MAX;
        let mut best = (-1 as i32, -1 as i32);
        let mut l = 0;

        let (mut have, need) = (0, tCount.len());
    
        for r in 0..s.len() {

            let char = s[r];

            // Add new element on the right
            *window.entry(char).or_default() += 1;
            have += (window.get(&char) == tCount.get(&char)) as usize;
            
           // If mapping contains all t chars, move left pointer until we dont and save best result
           while have == need {

                // Save if best
                if (r - l + 1) < best_len {
                    best_len = (r - l + 1);
                    best = (l as i32, r as i32);
                }

                // Remove elements
                *window.get_mut(&s[l]).unwrap() -= 1;

                // Check if condition is still met
                if window.get(&s[l]) < tCount.get(&s[l]) {
                    have -= 1;
                }

                // Move pointer
                l += 1;
            }
        }


        if best.0 > -1 && best.1 > -1 {
            return s[best.0 as usize..=best.1 as usize].iter().collect::<String>()
        }
        String::new()
    }
}