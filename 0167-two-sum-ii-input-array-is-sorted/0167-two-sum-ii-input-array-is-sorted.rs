impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {

        let mut l = 0;
        let mut r = numbers.len() - 1;

        while l < r {
            let res = numbers[l] + numbers[r];

            if res < target {
                l += 1;
            } else if res > target {
                r -= 1;
            } else {
                return vec![l as i32 + 1, r as i32 + 1]
            }
        }

        return vec![0, 0]
    }
}