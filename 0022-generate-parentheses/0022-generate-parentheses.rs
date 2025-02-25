impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut ans: Vec<String> = Vec::new();

        Solution::rec("".to_string(), n, 0, 0, &mut ans);

        ans
    }

    fn rec(curr: String, n: i32, l: i32, r: i32, ans: &mut Vec<String>) {
        if l + r >= n * 2 {
            ans.push(curr);
            return;
        }

        if l > r {
            Solution::rec(curr.clone() + ")", n, l, r + 1, ans);
        }
        if l < n {
            Solution::rec(curr.clone() + "(", n, l + 1, r, ans);
        }
    }
}