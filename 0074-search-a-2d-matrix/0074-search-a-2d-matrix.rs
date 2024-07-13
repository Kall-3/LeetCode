impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.is_empty() {
            return false;
        }

        let (mut L, mut R) = (0, matrix.len() as i32 - 1);

        while L < R {
            let M = (L + R) / 2;

            if matrix[M as usize][0] == target {
                return true;
            } else if matrix[M as usize][0] < target {
                L = M + 1;
            } else {
                R = M - 1;
            }
        }

        let mut M = (L + R) as i32 / 2;
        if matrix[M as usize][0] > target && M > 0 {
            M -= 1;
        }
        (L, R) = (0, matrix[0].len() as i32 - 1);

        while L <= R {
            let M2 = (L + R) / 2;

            if matrix[M as usize][M2 as usize] == target {
                return true;
            } else if matrix[M as usize][M2 as usize] < target {
                L = M2 + 1;
            } else {
                R = M2 - 1;
            }
        }

        false
    }
}