impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        //let mut k_min = (piles.iter().sum::<i32>() / h).max(1);
        let mut k_min = 1;
        let mut k_max = *piles.iter().max().unwrap();

        while k_min < k_max {
            let k_mid = (k_min + k_max) / 2;

            if !Self::can_finish(&piles, h, k_mid) {
                k_min = k_mid + 1;
            } else {
                k_max = k_mid;
            }
        }

        k_min
    }

    fn can_finish(piles: &Vec<i32>, mut h: i32, k_mid: i32) -> bool {
        for &p in piles {
            h -= ((p - 1) / k_mid) + 1;
        }
        h >= 0
    }
}