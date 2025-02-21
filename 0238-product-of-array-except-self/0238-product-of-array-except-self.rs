impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut sum = 1;
        let mut contains_zero_index = vec![];

        for (i, num) in nums.iter().enumerate() {
            if num != &0 {
                sum *= num;
            } else {
                contains_zero_index.push(i);
            }
        }

        if contains_zero_index.is_empty() {
            let mut ans = vec![sum; nums.len()];
            for (i, num) in nums.into_iter().enumerate() {
                ans[i] /= num;
            }
            return ans;
        } else if contains_zero_index.len() > 1 {
            return vec![0; nums.len()];
        } else {
            let mut ans = vec![0; nums.len()];
            ans[contains_zero_index[0]] = sum;
            return ans;
        }
    }
}
