// https://leetcode.com/problems/maximum-balanced-shipments/
// 3638. Maximum Balanced Shipments
pub struct Solution;
impl Solution {
    pub fn max_balanced_shipments(weight: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut i = 0;
        while i + 1 < weight.len() {
            if weight[i] > weight[i + 1] {
                ans += 1;
                i += 2;
            } else {
                i += 1;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_balanced_shipments() {
        assert_eq!(Solution::max_balanced_shipments(vec![625, 467, 752]), 1);
        assert_eq!(Solution::max_balanced_shipments(vec![2, 5, 1, 4, 3]), 2);
        assert_eq!(Solution::max_balanced_shipments(vec![4, 4]), 0)
    }
}
