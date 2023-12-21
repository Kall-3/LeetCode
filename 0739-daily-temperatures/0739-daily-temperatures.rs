impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {

        let mut ans: Vec<i32> = vec![0; temperatures.len() as usize];
        let mut stack: Vec<(usize, i32)> = Vec::new();

        for (idx, &temp) in temperatures.iter().enumerate() {

            while stack.last().unwrap_or(&(0, 200)).1 < temp {
                let (i,_) = stack.pop().unwrap();
                ans[i] = (idx - i) as i32;
            }

            stack.push((idx, temp));
        }

        ans
    }
}