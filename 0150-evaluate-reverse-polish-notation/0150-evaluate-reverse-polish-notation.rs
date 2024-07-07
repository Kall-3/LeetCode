impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = vec![];

        for token in tokens {
            match token.as_str() {
                "+" => {
                    let val1 = stack.pop().unwrap();
                    let val2 = stack.pop().unwrap();
                    stack.push(val1 + val2);
                }
                "-" => {
                    let val1 = stack.pop().unwrap();
                    let val2 = stack.pop().unwrap();
                    stack.push(val2 - val1);
                }
                "*" => {
                    let val1 = stack.pop().unwrap();
                    let val2 = stack.pop().unwrap();
                    stack.push(val1 * val2);
                }
                "/" => {
                    let val1 = stack.pop().unwrap();
                    let val2 = stack.pop().unwrap();
                    stack.push(val2 / val1);
                }
                _ => stack.push(token.parse::<i32>().unwrap()),
            }
        }
        
        stack.pop().unwrap()
    }
}