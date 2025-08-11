// https://leetcode.cn/problems/semi-ordered-permutation/
// 2717. Semi-Ordered Permutation
pub struct Solution;
impl Solution {
    pub fn semi_ordered_permutation(nums: Vec<i32>) -> i32 {
        let mut o1 = -1;
        let mut on = -1;
        let l = nums.len() as i32;
        for (n, i) in nums.into_iter().zip(0..) {
            if n == 1 {
                o1 = i;
            } else if n == l {
                on = i;
            }
            if o1 >= 0 && on >= 0 {
                break;
            }
        }
        o1 + l - on - if on > o1 { 1 } else { 2 }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_semi_ordered_permutation() {
        assert_eq!(Solution::semi_ordered_permutation(vec![2, 1, 4, 3]), 2);
        assert_eq!(Solution::semi_ordered_permutation(vec![2, 4, 1, 3]), 3);
        assert_eq!(Solution::semi_ordered_permutation(vec![1, 3, 4, 2, 5]), 0);
    }
}
