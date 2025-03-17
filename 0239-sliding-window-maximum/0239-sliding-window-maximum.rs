use std::collections::VecDeque;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;

        let mut stack: VecDeque<(usize, i32)> = VecDeque::new();
        let mut res: Vec<i32> = Vec::new();

        for i in 0..nums.len() {
            while !stack.is_empty() && nums[i] > stack.back().unwrap().1 {
                stack.pop_back();
            }
            stack.push_back((i, nums[i]));

            if stack.front().unwrap().0 + k < i + 1 {
                stack.pop_front();
            }

            if i + 2 > k {
                res.push(stack.front().unwrap().1);
            }
        }

        res
    }
}