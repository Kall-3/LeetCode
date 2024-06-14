impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut fst = 0;
        let mut snd = nums.len() - 1;

        let mut sorted = nums.clone();
        sorted.sort();

        while fst < snd {
            let sum = sorted[fst] + sorted[snd];
            if sum < target {
                fst += 1;
            } else if sum > target{
                snd -= 1;
            } else {
                break;
            }
        }

        let fst = nums.iter().position(|&x| x == sorted[fst]).unwrap() as i32;
        let snd = nums.iter().rposition(|&x| x == sorted[snd]).unwrap() as i32;
        return vec![fst, snd];

    }
}