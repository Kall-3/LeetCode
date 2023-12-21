use std::cmp::min;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut L, mut R) = (0, height.len() - 1);
        let mut max = 0;

        while L < R {
            let area = min(height[L], height[R]) * (R - L) as i32;
            if area > max {
                max = area;
            }

            if height[L] > height[R] {
                R -= 1;
            } else {
                L += 1;
            }
        }

        max
    }
}