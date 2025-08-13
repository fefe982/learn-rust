// https://leetcode.com/problems/broken-calculator/
// 991. Broken Calculator
pub struct Solution;
impl Solution {
    pub fn broken_calc(start_value: i32, target: i32) -> i32 {
        let mut cnt = 0;
        let mut target = target;
        while target > start_value {
            if target & 1 == 0 {
                target >>= 1;
            } else {
                target += 1;
            }
            cnt += 1;
        }
        cnt + start_value - target
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn broken_calc() {
        assert_eq!(Solution::broken_calc(2, 3), 2);
        assert_eq!(Solution::broken_calc(5, 8), 2);
        assert_eq!(Solution::broken_calc(3, 10), 3);
    }
}
