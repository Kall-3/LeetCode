impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = Vec::new();

        for token in tokens.iter() {
            if is_digit(token) {
                stack.push(token.parse::<i32>().unwrap() );
            } else if token == "+" {
                let mut res = stack.pop().unwrap();

                res = stack.pop().unwrap() + res;

                stack.push(res);
            } else if token == "-" {
                let mut res = stack.pop().unwrap();

                res = stack.pop().unwrap() - res;

                stack.push(res);
            } else if token == "*" {
                let mut res = stack.pop().unwrap();

                res = stack.pop().unwrap() * res;

                stack.push(res);
            } else if token == "/" {
                let mut res = stack.pop().unwrap();

                res = stack.pop().unwrap() / res;

                stack.push(res);
            }
        }
        *stack.last().unwrap()
    }
}

fn is_digit(s: &str) -> bool {
    match s.parse::<i32>() {
        Ok(_)   => true,
        Err(_)  => false,
    }
}
