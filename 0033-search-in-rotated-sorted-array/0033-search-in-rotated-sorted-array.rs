use std::cmp::Ordering::{Greater, Less, Equal};

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut l, mut r) = (0, nums.len() - 1);

        while l <= r {
            // Middle of remainder
            let m = l + (r - l) / 2;

            // Found element
            if nums[m] == target {
                return m as i32;
            }

            // Left side sorted
            if nums[l] <= nums[m] {
                if nums[l] <= target && target <= nums[m] {
                    // On left side
                    r = m - 1;
                } else {
                    // On right side
                    l = m + 1;
                }
            }
            // Right side sorted
            else {
                if nums[m] < target && target <= nums[r] {
                    // On right side
                    l = m + 1;
                } else {
                    // On left side
                    r = m - 1;
                }
            }
        }
        
        -1
    }
}