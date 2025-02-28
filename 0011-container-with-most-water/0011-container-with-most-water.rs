impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, height.len() - 1);

        let mut max_water = 0;

        while left < right {
            let water = (right - left) as i32 * height[left].min(height[right]);

            max_water = max_water.max(water);

            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }

        max_water
    }
}