use std::cmp::Ordering::{Equal, Less, Greater};

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let max_pile = *piles.iter().max().unwrap() as usize;
        let (mut l, mut r) = (1, max_pile);
        let mut res = 0;

        while l <= r {
            let k = (l + r) / 2;
            let time: usize = piles.iter()
                         .map(|&nbr_bananas| ((nbr_bananas - 1) as usize / k) + 1)
                         .sum();

            match time.cmp(&(h as usize)) {
                Less | Equal    => {
                    r = k - 1;
                    res = k;
                },
                Greater         => l = k + 1,
            }
        }

        res as i32
    }
}
