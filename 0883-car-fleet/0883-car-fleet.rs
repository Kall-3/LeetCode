impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut cars: Vec<(i32, i32)> = position.into_iter().zip(speed.into_iter()).collect();

        cars.sort_by(|a, b| a.0.cmp(&b.0));

        let mut fleets = 1;

        while let Some((p1, s1)) = cars.pop() {
            while let Some((p2, s2)) = cars.last() {
                let t1 = (target - p1) as f32 / s1.clone() as f32;
                let t2 = (target - p2) as f32 / s2.clone() as f32;

                if t2 > t1 {
                    fleets += 1;
                    break;
                } else {
                    cars.pop();
                }
            }
        }

        fleets
    }
}