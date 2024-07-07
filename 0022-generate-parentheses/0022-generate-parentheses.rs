impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut stack: Vec<String> = vec![];
        Self::rec("(".to_string(), &n, 1, 0, &mut stack);
        stack
    }

    fn rec(cur: String, size: &i32, L: i32, R: i32, res: &mut Vec<String>) {
        if L == *size && L == R {
            res.push(cur.clone());
            return
        }
    
        if L < *size {
            Self::rec(cur.clone() + "(", size, L + 1, R, res);
        }
    
        if L > R {
            Self::rec(cur + ")", size, L, R + 1, res);
        }
    }

}
