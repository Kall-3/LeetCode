impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        
        nums.sort();

        let (mut L, mut R) = (0, 0);
        let mut res: Vec<_> = vec![];

        for (idx, &num) in nums.iter().enumerate().take(nums.len() - 2) {
            if idx > 0 && nums[idx] == nums[idx - 1] {
                continue;
            }

            L = idx + 1;
            R = nums.len() - 1;

            while L < R {
                let sum = num + nums[L] + nums[R];

                if sum < 0 {
                    L += 1;
                    continue;
                }
                if sum > 0 {
                    R -= 1;
                    continue;
                }

                res.push([num, nums[L], nums[R]].to_vec());

                let L_num = nums[L];
                let R_num = nums[R];

                while L < nums.len() && L_num == nums[L] {
                    L += 1;
                }
                while R > 0 && R_num == nums[R] {
                    R -= 1;
                }
            }
        }

        res
    }
}