use std::cmp::Ordering::{Greater, Less, Equal};

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut l, mut r) = (1, nums.len());
        let mut res = nums[0];

        while l <= r {
            let nums_l = nums[l - 1];
            if nums_l < nums[r - 1] {
                return res.min(nums_l);
            }

            let m = (l + r) / 2;
            let nums_m = nums[m - 1];
            res = res.min(nums_m);
            match nums_m.cmp(&nums_l) {
                Greater | Equal => l = m + 1,
                Less            => r = m - 1,
            }
        }
        
        res
    }
}