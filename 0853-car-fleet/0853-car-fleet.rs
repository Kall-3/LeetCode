impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        
        let mut stack: Vec<f32> = Vec::new();

        let mut cars: Vec<(i32, i32)> = position.into_iter().zip(speed).collect();
        cars.sort();

        for (pos, speed) in cars.into_iter().rev() {
            let arrival = (target - pos) as f32 / speed as f32;
            stack.push(arrival);

            let len = stack.len();
            if len > 1 && stack[len - 2] >= arrival { // intersection
                stack.pop();
            }
        }

        stack.len() as i32
    }
}