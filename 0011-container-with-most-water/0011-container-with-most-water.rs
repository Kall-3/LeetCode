impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut L, mut R) = (0, height.len() - 1);
        let mut max = 0;

        while L < R {
            max = max.max(height[L].min(height[R]) * (R - L) as i32);

            if height[L] > height[R] {
                R -= 1;
            } else {
                L += 1;
            }
        }

        max
    }
}