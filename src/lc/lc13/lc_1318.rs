// https://leetcode.com/problems/minimum-flips-to-make-a-or-b-equal-to-c/
// 1318. Minimum Flips to Make a OR b Equal to c
pub struct Solution;
impl Solution {
    pub fn min_flips(a: i32, b: i32, c: i32) -> i32 {
        let mut bit = 1;
        let or = a | b;
        let mut cnt = 0;
        for _ in 0..31 {
            if or & bit != c & bit {
                if c & bit != 0 {
                    cnt += 1;
                } else {
                    if a & bit != 0 {
                        cnt += 1;
                    }
                    if b & bit != 0 {
                        cnt += 1;
                    }
                }
            }
            bit <<= 1;
        }
        cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_flips() {
        assert_eq!(Solution::min_flips(8, 3, 5), 3);
        assert_eq!(Solution::min_flips(2, 6, 5), 3);
        assert_eq!(Solution::min_flips(4, 2, 7), 1);
        assert_eq!(Solution::min_flips(1, 2, 3), 0);
    }
}
