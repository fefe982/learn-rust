// https://leetcode.com/problems/sum-of-square-numbers/
// 633. Sum of Square Numbers
pub struct Solution;
impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        if c == 0 {
            return true;
        }
        let mut c = c as i64;
        while c % 4 == 0 {
            c /= 4;
        }
        if c % 4 == 3 {
            return false;
        }
        let mut l = 0;
        let mut r = (c as f64).sqrt() as i64;
        while l <= r {
            let sum = l * l + r * r;
            if sum == c {
                return true;
            }
            if sum < c {
                l += 1;
            } else {
                r -= 1;
            }
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_judge_square_sum() {
        assert_eq!(Solution::judge_square_sum(2147473645), true);
        assert_eq!(Solution::judge_square_sum(0), true);
        assert_eq!(Solution::judge_square_sum(5), true);
        assert_eq!(Solution::judge_square_sum(3), false);
    }
}
