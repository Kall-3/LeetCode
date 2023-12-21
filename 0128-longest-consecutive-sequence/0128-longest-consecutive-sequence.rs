impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }

        let mut vec = nums.clone();
        vec.sort();

        let mut max = 1;
        let mut count = 1;

        for i in 1..vec.len() {
            if vec[i] == vec[i - 1] + 1 {
                count += 1;
            } else if vec[i] == vec[i - 1] {
                continue;
            } else {
                if count > max {
                    max = count;
                }
                count = 1;
            }
        }
        if count > max {
            max = count;
        }
        return max;
    }
}