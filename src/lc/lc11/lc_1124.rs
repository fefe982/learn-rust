// https://leetcode.com/problems/longest-well-performing-interval/
// 1124. Longest Well-Performing Interval
pub struct Solution;
impl Solution {
    pub fn longest_wpi(hours: Vec<i32>) -> i32 {
        let mut v = vec![];
        let mut c = 0;
        let mut res = 0;
        for i in 0..hours.len() {
            if hours[i] > 8 {
                c -= 1;
            } else {
                c += 1;
            }
            if c < 0 {
                res = i as i32 + 1;
            } else if (c as usize) < v.len() {
                res = res.max(i as i32 - v[c as usize]);
            } else if c as usize > v.len() {
                v.push(i as i32);
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn longest_wpi() {
        assert_eq!(Solution::longest_wpi(vec![6, 6, 9]), 1);
        assert_eq!(Solution::longest_wpi(vec![9, 9, 6, 0, 6, 6, 9]), 3);
        assert_eq!(Solution::longest_wpi(vec![6, 6, 6]), 0);
    }
}
