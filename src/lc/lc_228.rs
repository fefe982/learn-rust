// https://leetcode.com/problems/summary-ranges/
// 228. Summary Ranges
pub struct Solution;
impl Solution {
    fn gen_str(s: i32, e: i32) -> String {
        if s == e {
            s.to_string()
        } else {
            format!("{}->{}", s, e)
        }
    }
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        if nums.len() == 0 {
            return Vec::new();
        }
        let mut res = Vec::new();
        let mut s = nums[0];
        let mut e = nums[0];
        for i in 1..nums.len() {
            if nums[i] != e + 1 {
                res.push(Self::gen_str(s, e));
                s = nums[i];
            }
            e = nums[i];
        }
        res.push(Self::gen_str(s, e));
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn summary_ranges() {
        assert_eq!(
            Solution::summary_ranges(vec![0, 1, 2, 4, 5, 7]),
            vec_str!["0->2", "4->5", "7"]
        );
        assert_eq!(
            Solution::summary_ranges(vec![0, 2, 3, 4, 6, 8, 9]),
            vec_str!["0", "2->4", "6", "8->9"]
        );
    }
}
