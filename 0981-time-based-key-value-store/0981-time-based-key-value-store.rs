use std::collections::HashMap;

struct TimeMap {
    map: HashMap<String, Vec<(String, i32)>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {

    fn new() -> Self {
        TimeMap {
            map: HashMap::new(),
        }
    }
    
    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.map.entry(key).or_default().push((value, timestamp));
    }
    
    fn get(&self, key: String, timestamp: i32) -> String {
        let mut best = "";

        if let Some(values) = self.map.get(&key) {
            if values[0].1 > timestamp {
                return best.to_string();
            }

            let (mut l, mut r) = (0, values.len() - 1);
            while l <= r {
                let m = l + (r - l) / 2;

                if values[m].1 == timestamp {
                    return values[m].0.to_string();
                }

                if values[m].1 > timestamp {
                    r = m - 1;
                } else {
                    best = &values[m].0;
                    l = m + 1;
                }
            }
        }
        best.to_string()
    }
}

/**
 * Your TimeMap object will be instantiated and called as such:
 * let obj = TimeMap::new();
 * obj.set(key, value, timestamp);
 * let ret_2: String = obj.get(key, timestamp);
 */