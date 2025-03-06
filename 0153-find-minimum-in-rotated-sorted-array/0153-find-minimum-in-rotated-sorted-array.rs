impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, nums.len() - 1);

        let mut min = i32::MAX;

        while left <= right {
            let middle = (right + left) / 2;

            if nums[left] <= nums[middle] {
                if nums[middle] <= nums[right] {
                    // now totally sorted
                    return min.min(nums[left])
                }
                // sorted left -> middle
                min = min.min(nums[right]);
                left = middle + 1;
            } else {
                // sorted middle -> right
                min = min.min(nums[middle]);
                right = middle - 1;
            }
        }

        min
    }
}