use std::collections::HashMap;

impl Solution {
    pub fn next_greater_element(mut nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut stack: Vec<i32> = Vec::new();

        for num in nums2 {
            while !stack.is_empty() && *stack.last().unwrap() < num {
                map.insert(stack.pop().unwrap(), num);
            }
            stack.push(num);
        }
        for i in 0..nums1.len() {
            nums1[i] = *map.get(&nums1[i]).unwrap_or(&-1);
        }
        nums1
    }
}