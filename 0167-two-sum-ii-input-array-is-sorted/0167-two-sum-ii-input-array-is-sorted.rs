impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left: usize = 0;
        let mut right: usize = numbers.len() - 1;

        while left < right {
            if numbers[left] + numbers[right] == target {
                return vec![(left + 1).try_into().unwrap(), (right + 1).try_into().unwrap()];
            } else if numbers[left] + numbers[right] > target {
                right -= 1;
            } else {
                left += 1;
            }
        }
        vec![]
    }
}