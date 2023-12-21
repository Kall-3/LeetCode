impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();

        for c in s.chars() {
            if "({[".contains(c) {
                stack.push(c);
            } else {
                match stack.last() {
                    Some(last) if (c == ')' && last == &'(') || (c == '}' && last == &'{') || (c == ']' && last == &'[') => { stack.pop(); },
                    None => return false,
                    _ => stack.push(c),
                };
            }
        }

        stack.is_empty()
    }
}