// https://leetcode.com/problems/valid-perfect-square/
// 367. Valid Perfect Square
pub struct Solution;
impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        if num == 1 {
            return true;
        }
        if num == 46340 * 46340 {
            return true;
        }
        if num > 46340 * 46340 {
            return false;
        }
        let mut min = 1;
        let mut max = 46340.min(num);
        while min + 1 < max {
            let mid = (min + max) / 2;
            if mid * mid == num {
                return true;
            } else if mid * mid < num {
                min = mid;
            } else {
                max = mid;
            }
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_perfect_square() {
        assert_eq!(Solution::is_perfect_square(16), true);
        assert_eq!(Solution::is_perfect_square(14), false);
    }
}
