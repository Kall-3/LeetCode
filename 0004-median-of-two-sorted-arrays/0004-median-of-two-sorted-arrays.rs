impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        // If one list empty, return middle of other
        if nums1.is_empty() {
            let middle = nums2.len() / 2;
            if nums2.len() % 2 == 1 {
                return nums2[middle] as f64;
            } else {
                return (nums2[middle] + nums2[middle - 1]) as f64 / 2.0;
            }
        }
        if nums2.is_empty() {
            let middle = nums1.len() / 2;
            if nums1.len() % 2 == 1 {
                return nums1[middle] as f64;
            } else {
                return (nums1[middle] + nums1[middle - 1]) as f64 / 2.0;
            }
        }

        let (mut fst, mut snd) = (0, 0);
        let total = nums1.len() + nums2.len() - 2;
        let middle = total / 2 + 1;
        let mut res = vec![];

        // Create new sorted list up to centre
        while fst + snd <= middle {
            if fst < nums1.len() && (snd < nums2.len() && nums1[fst] <= nums2[snd]) || snd == nums2.len() {
                res.push(nums1[fst]);
                fst += 1;
            } else if snd < nums2.len() {
                res.push(nums2[snd]);
                snd += 1;
            }
        }
        
        // Return median of centre element/-s
        if total % 2 == 1 {
            return res[res.len() - 1] as f64;
        }
        return (res[res.len() - 1] + res[res.len() - 2]) as f64 / 2.0;
    }
}