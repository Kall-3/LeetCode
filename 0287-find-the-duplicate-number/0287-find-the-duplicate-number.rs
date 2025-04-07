use std::collections::HashSet;

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut slow = 0;
        let mut fast = 0;
        loop {
            slow = nums[slow] as usize;
            fast = nums[nums[fast] as usize] as usize;

            if slow == fast {
                break;
            }
        }

        fast = 0;
        while fast != slow {
            slow = nums[slow] as usize;
            fast = nums[fast] as usize;
        }

        slow as i32
    }
}