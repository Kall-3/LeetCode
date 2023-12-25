use std::cmp::Ordering::{Greater, Less, Equal};

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut l, mut r) = (1, nums.len());
        let mut res = nums[0];

        while l <= r {
            if nums[l - 1] < nums[r - 1] {
                return res.min(nums[l - 1]);
            }

            let m = (l + r) / 2;
            res = res.min(nums[m - 1]);
            match nums[m - 1].cmp(&nums[l - 1]) {
                Greater | Equal => l = m + 1,
                Less            => r = m - 1,
            }
        }
        
        res
    }
}