impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();

        let mut solutions: Vec<Vec<i32>> = Vec::new();

        let (mut left, mut right) = (0, nums.len() - 1);

        for (i, target) in nums.iter().take(nums.len() - 2).enumerate() {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            left = i + 1;
            right = nums.len() - 1;

            while left < right {
                let sum = target + nums[left] + nums[right];

                if sum < 0 {
                    left += 1;
                } else if sum > 0 {
                    right -= 1;
                } else {
                    solutions.push(vec![*target, nums[left], nums[right]]);
                    left += 1;
                    right -= 1;

                    while left < right && nums[left] == nums[left - 1] {
                        left += 1;
                    }
                    while left < right && nums[right] == nums[right + 1] {
                        right -= 1;
                    }
                }
            }
        }

        solutions
    }
}