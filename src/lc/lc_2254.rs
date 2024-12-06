// https://leetcode.com/problems/maximum-number-of-integers-to-choose-from-a-range-i/
// 2254. Maximum Number of Integers to Choose From a Range I
pub struct Solution;
impl Solution {
    pub fn max_count(banned: Vec<i32>, n: i32, max_sum: i32) -> i32 {
        let mut sum = 0;
        let mut cnt = 0;
        let mut banned = banned;
        banned.sort();
        let mut ib = 0;
        for i in 1..=n {
            while ib < banned.len() && banned[ib] < i {
                ib += 1;
            }
            if ib < banned.len() && banned[ib] == i {
                continue;
            }
            sum += i;
            if sum > max_sum {
                break;
            }
            cnt += 1;
        }
        cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_count() {
        assert_eq!(Solution::max_count(vec![1, 6, 5], 5, 6), 2);
        assert_eq!(Solution::max_count(vec![1, 2, 3, 4, 5, 6, 7], 8, 1), 0);
        assert_eq!(Solution::max_count(vec![11], 7, 50), 7);
    }
}
