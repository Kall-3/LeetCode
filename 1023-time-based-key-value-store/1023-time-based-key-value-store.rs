use std::collections::HashMap;

struct TimeMap {
    map: HashMap<String, Vec<(i32, String)>>,
}

impl TimeMap {

    fn new() -> Self {
        TimeMap {
            map: HashMap::new()
        }
    }
    
    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.map.entry(key).or_default().push((timestamp, value));
    }
    
    fn get(&self, key: String, timestamp: i32) -> String {
        if let Some(vec) = self.map.get(&key) {
            
            let (mut left, mut right) = (0, vec.len() as i32 - 1);
            let mut best = &String::new();

            if timestamp > vec[right as usize].0 {
                return vec[right as usize].1.clone();
            }

            while left <= right {
                let middle = (left + right) / 2;

                if vec[middle as usize].0 <= timestamp {
                    left = middle + 1;
                    best = &vec[middle as usize].1;
                } else {
                    right = middle - 1;
                }
            }
            return best.clone();
        }
        String::new()
    }
}