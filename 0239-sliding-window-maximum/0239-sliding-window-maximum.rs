use std::collections::VecDeque;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        let mut queue: VecDeque<usize> = VecDeque::with_capacity(k as usize);

        let (mut l, mut r) = (0, 0);

        while r < nums.len() {

            while let Some(back) = queue.back() {
                if nums[r] > nums[*back] {
                    queue.pop_back();
                } else {
                    break;
                }
            }

            queue.push_back(r);

            if l > *queue.front().unwrap() {
                queue.pop_front();
            }

            if r + 1 >= k as usize {
                res.push(nums[*queue.front().unwrap()]);
                l += 1;
            }

            r += 1;
        } 

        res
    }
}
