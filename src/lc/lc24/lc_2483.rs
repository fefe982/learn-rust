// https://leetcode.com/problems/minimum-penalty-for-a-shop/
// 2483. Minimum Penalty for a Shop
pub struct Solution;
impl Solution {
    pub fn best_closing_time(customers: String) -> i32 {
        let cs = customers.as_bytes();
        let mut min = cs.len();
        let mut penalty = cs.len();
        let mut idx = 0;
        for (&c, i) in cs.iter().zip(1..) {
            if c == b'Y' {
                penalty -= 1;
            } else {
                min -= 1;
            }
            if penalty < min {
                idx = i;
                min = penalty;
            }
        }
        idx
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn best_closing_time() {
        assert_eq!(Solution::best_closing_time("YYNY".to_string()), 2);
        assert_eq!(Solution::best_closing_time("NNNN".to_string()), 0);
        assert_eq!(Solution::best_closing_time("YYYY".to_string()), 4);
    }
}
