impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let (mut L, mut R) = (1, height.len().max(2) - 2);
        let (mut L_max, mut R_max) = (height[0], height[height.len() - 1]);
        let mut water = 0;

        while L <= R {
            if L_max <= R_max {
                water += (L_max.min(R_max) - height[L]).max(0);
                L_max = L_max.max(height[L]);
                L += 1;
            } else {
                water += (L_max.min(R_max) - height[R]).max(0);
                R_max = R_max.max(height[R]);
                R -= 1;
            }
        }

        water
    }
}