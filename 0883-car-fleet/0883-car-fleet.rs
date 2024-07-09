impl Solution {
    pub fn car_fleet(target: i32, mut position: Vec<i32>, mut speed: Vec<i32>) -> i32 {
        let mut cars: Vec<(i32, i32)> = position
            .into_iter()
            .zip(speed.into_iter())
            .collect();

        cars.sort_unstable_by_key(|ps| ps.0);

        let mut fleets: i32 = 0;
        let mut curr_time: f32 = 0.0;

        for (pos, spd) in cars.into_iter().rev() {
            let time = (target - pos) as f32 / spd as f32;
        
            if time > curr_time {
                fleets += 1;
                curr_time = time;
            }
        }

        fleets
    }
}