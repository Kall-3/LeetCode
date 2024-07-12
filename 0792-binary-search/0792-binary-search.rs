impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return -1;
        }

        let (mut L, mut R) = (0, nums.len() as i32 - 1);

        while L <= R {
            let M = (L + R) / 2;

            if nums[M as usize] == target {
                return M as i32;
            } else if nums[M as usize] < target {
                L = M + 1; 
            } else {
                R = M - 1;
            }
        }
        return -1;
    }
}