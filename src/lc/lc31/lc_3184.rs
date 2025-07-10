// https://leetcode.com/problems/count-pairs-that-form-a-complete-day-i/
// 3184. Count Pairs That Form a Complete Day I
pub struct Solution;
impl Solution {
    pub fn count_complete_day_pairs(hours: Vec<i32>) -> i32 {
        let mut m = vec![0; 24];
        for h in hours {
            m[(h % 24) as usize] += 1;
        }
        let mut ans = m[0] * (m[0] - 1) / 2 + m[12] * (m[12] - 1) / 2;
        for i in 1..12 {
            ans += m[i] * m[24 - i];
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_complete_day_pairs() {
        assert_eq!(Solution::count_complete_day_pairs(vec![12, 12, 30, 24, 24]), 2);
        assert_eq!(Solution::count_complete_day_pairs(vec![72, 48, 24, 3]), 3);
    }
}
