// https://leetcode.com/problems/apply-operations-to-make-two-strings-equal/
// 2896. Apply Operations to Make Two Strings Equal
pub struct Solution;
impl Solution {
    pub fn min_operations(s1: String, s2: String, x: i32) -> i32 {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let mut done = 0;
        let inf = i32::MAX / 2;
        let mut one = inf;
        let mut two = inf;
        let mut last = inf;
        for i in 0..s1.len() {
            if s1[i] == s2[i] {
                last += 1;
                two += 1;
            } else {
                let ndone = (one + x).min(last + 1);
                let none = done.min(two + 1);
                let ntwo = one;
                let nlast = done.min(two + x);
                done = ndone;
                one = none;
                two = ntwo;
                last = nlast;
            }
        }
        if done >= inf {
            -1
        } else {
            done
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_operations() {
        assert_eq!(
            Solution::min_operations("1100011000".to_string(), "0101001010".to_string(), 2),
            4
        );
        assert_eq!(
            Solution::min_operations("10110".to_string(), "00011".to_string(), 4),
            -1
        );
    }
}
