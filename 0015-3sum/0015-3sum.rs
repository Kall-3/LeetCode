impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();

        let (mut R, mut L) = (0, 0);
        let mut res: Vec<Vec<i32>> = Vec::new();

        for (i, &num) in nums.iter().enumerate().take(nums.len() - 2) {
            if i > 0 && num == nums[i - 1] {
                continue;
            }

            L = i + 1;
            R = nums.len() - 1;

            while L < R {
                let sum = nums[L] + nums[R] + num;
                if sum < 0 {
                    L += 1;
                    continue;
                } 
                if sum > 0 {
                    R -= 1;
                    continue;
                }

                res.push(vec![nums[i], nums[L], nums[R]]);

                let L_val = nums[L];
                let R_val = nums[R];

                while L < R && nums[L] == L_val {
                    L += 1;
                }
                while R > L && nums[R] == R_val {
                    R -= 1;
                }
            }
        }
        res
    }
}