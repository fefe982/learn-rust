// https://leetcode.com/problems/number-of-times-binary-string-is-prefix-aligned/
// 1375. Number of Times Binary String Is Prefix-Aligned
pub struct Solution;
impl Solution {
    pub fn num_times_all_blue(flips: Vec<i32>) -> i32 {
        let mut m = 0;
        let mut c = 0;
        for (&f, idx) in flips.iter().zip(1..) {
            m = std::cmp::max(m, f);
            if m == idx {
                c += 1;
            }
        }
        c
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn num_times_all_blue() {
        assert_eq!(Solution::num_times_all_blue(vec![3, 2, 4, 1, 5]), 2);
        assert_eq!(Solution::num_times_all_blue(vec![4, 1, 2, 3]), 1);
    }
}
