impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, nums.len() as i32 - 1);

        while left < right {
            let mid = (left + right) / 2;

            if nums[right as usize] < nums[mid as usize] {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        nums[left as usize]
    }
}