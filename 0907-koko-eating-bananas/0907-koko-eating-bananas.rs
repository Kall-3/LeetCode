impl Solution {
    pub fn min_eating_speed(mut piles: Vec<i32>, h: i32) -> i32 {
        let mut high = *piles.iter().max().unwrap();
        let mut low = 1;

        if piles.len() == h as usize {
            return high;
        }

        while low <= high {
            let k = (low + high) / 2;
            let mut time = 0;

            for pile in piles.iter() {
                time += (pile + k - 1) / k;

                if time > h {
                    break;
                }
            }

            if time <= h {
                high = k - 1;
            } else {
                low = k + 1;
            }
        }

        return low;
    }
}