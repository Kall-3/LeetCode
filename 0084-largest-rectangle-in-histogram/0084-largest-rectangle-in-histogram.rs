impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut stack: Vec<(i32, i32)> = Vec::new();
        let mut max = 0;

        for (idx, hgt) in heights.iter().enumerate() {
            let (mut old_idx, mut old_hgt) = (idx as i32, 0);

            // New height limits area
            while stack.len() > 0 && &stack[stack.len() - 1].1 > hgt {
                (old_idx, old_hgt) = stack.pop().unwrap();
                let area = old_hgt * (idx as i32 - old_idx);

                if area > max {
                    max = area;
                }
            }
            
            // Add new element to stack, and extend it back as many times as we poped elements i.e. before limited
            stack.push((old_idx, *hgt));
        }

        // Calculate area of remaining elements on stack, that extend all way back
        for (idx, hgt) in stack.iter() {
            let area = hgt * (heights.len() as i32 - idx);
            if area > max {
                max = area;
            }
        }

        // Return max
        max
    }
}