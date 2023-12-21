impl Solution {

    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();

        sol(1, n*2, 1, 0, "(".to_string(), &mut res);

        res
    }

}

pub fn sol(current: i32, size: i32, L: i32, R: i32, curr: String, vec: &mut Vec<String>) {

    if current == size {
        return vec.push(curr);
    }

    if current < size && L < size/2 {
        // let mut next_l = curr.clone();
        // next_l.push('(');
        let next_l = format!("{}{}", curr, '(');
        sol(current + 1, size, L + 1, R, next_l, vec);
    }
    if L > R {
        // let mut next_r = curr.clone();
        // next_r.push(')');
        let next_r = format!("{}{}", curr, ')');
        sol(current + 1, size, L, R + 1, next_r, vec);
    }
}