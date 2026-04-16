// https://leetcode.com/problems/minimum-absolute-distance-between-mirror-pairs/
// 3761. Minimum Absolute Distance Between Mirror Pairs
pub struct Solution;
impl Solution {
    fn reverse_num(mut x: i32) -> i32 {
        let mut rev = 0;
        while x > 0 {
            rev = rev * 10 + x % 10;
            x /= 10;
        }
        rev
    }

    pub fn min_mirror_pair_distance(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let mut latest_idx: HashMap<i32, i32> = HashMap::new();
        let mut ans = i32::MAX;

        for (j, &x) in nums.iter().enumerate() {
            let j = j as i32;
            if let Some(&i) = latest_idx.get(&x) {
                ans = ans.min(j - i);
            }
            latest_idx.insert(Self::reverse_num(x), j);
        }

        if ans == i32::MAX {
            -1
        } else {
            ans
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_mirror_pair_distance() {
        assert_eq!(Solution::min_mirror_pair_distance(vec![12, 21, 45, 33, 54]), 1);
        assert_eq!(Solution::min_mirror_pair_distance(vec![120, 21]), 1);
        assert_eq!(Solution::min_mirror_pair_distance(vec![21, 120]), -1);
    }
}
