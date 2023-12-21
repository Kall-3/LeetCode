impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut prefix: Vec<_> = nums.iter().scan(1, |state, x| {
            let val = *state;
            *state *= x;
            Some(val)
        }).collect();
        
        nums.iter()
            .enumerate()
            .rev()
            .scan(1, |state, (i, x)| {
                let val = *state;
                *state *= x;
                Some((i, val))
        }).for_each(|(i, postfix)| {
            prefix[i] *= postfix;
        });
        
        prefix
    }
}