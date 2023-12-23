impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {

        let (mut b, mut t) = (1, matrix.len());
        let (mut l, mut r) = (1, matrix[0].len());
        let mut m = 0;

        while b <= t {
            m = (b + t) / 2;

            if matrix[m - 1][0] <= target {
                // potential here
                if m < matrix.len() && matrix[m][0] > target {
                    // row above larger, so number is on this row
                    break;
                }
                b = m + 1;
            } else if matrix[m - 1][0] > target {
                // potential below
                t = m - 1;
            }
        }

        let row = &matrix[m - 1];
        while l <= r {
            m = (l + r) / 2;

            if row[m - 1] < target {
                l = m + 1;
            } else if row[m - 1] > target {
                r = m - 1;
            } else if row[m - 1] == target {
                return true;
            }
        }

        false
    }
}