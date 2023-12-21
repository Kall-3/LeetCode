impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut res = 0;
        let (mut L, mut R) = (0, height.len() - 1);
        let (mut L_max, mut R_max) = (height[L], height[R]);

        while L < R {
            if L_max <= R_max {
                let water = L_max.min(R_max) - height[L];

                if water > 0 {
                    res += water;
                }
                L_max = L_max.max(height[L]);
                L += 1;
            } else {
                R -= 1;
                let water = L_max.min(R_max) - height[R];

                if water > 0 {
                    res += water;
                }
                R_max = R_max.max(height[R]);
            }
        }
        
        res
    }
}