impl Solution {
    pub fn largest_rectangle_area(mut heights: Vec<i32>) -> i32 {
        heights.push(0);
        heights.insert(0, 0);

        let mut stack: Vec<usize> = Vec::new();
        stack.push(0);

        let mut max_area = 0;

        for i in 1..heights.len() {
            while heights[*stack.last().unwrap()] > heights[i] {
                let height = heights[stack.pop().unwrap()];
                let width = i - stack.last().unwrap() - 1;

                max_area = max_area.max(height * width as i32);
            }

            stack.push(i);
        }

        max_area
    }
}