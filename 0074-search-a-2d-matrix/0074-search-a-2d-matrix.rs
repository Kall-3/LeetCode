impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let rows = matrix.len() as i32;
        let cols = matrix[0].len() as i32;

        let (mut L, mut R) = (0, rows * cols - 1);

        while L <= R {
            let M = (L + R) / 2;
            let M_val = matrix[(M / cols) as usize][(M % cols) as usize];
            
            if M_val == target {
                return true;
            } else if M_val < target {
                L = M + 1;
            } else {
                R = M - 1;
            }
        }

        false
    }
}