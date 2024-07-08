impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<(i32, i32)> = Vec::new();
        let mut res: Vec<i32> = vec![0; temperatures.len()];

        for (i, temp) in temperatures.iter().enumerate() {
            while !stack.is_empty() && *temp > stack.last().unwrap().0 {
                let (_, s_i) = stack.pop().unwrap();
                res[s_i as usize] = i as i32 - s_i;
            }
            stack.push((*temp, i as i32));
        }
        res
    }
}