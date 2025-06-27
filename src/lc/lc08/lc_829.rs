// https://leetcode.com/problems/consecutive-numbers-sum/
// 829. Consecutive Numbers Sum
pub struct Solution;

impl Solution {
    pub fn consecutive_numbers_sum(n: i32) -> i32 {
        let mut last = 2;
        let mut cnt_last = 0;
        let mut cnt_all = 1;
        let mut m = n;
        while m > 1 {
            while m % last == 0 {
                m /= last;
                cnt_last += 1;
            }
            if last != 2 {
                cnt_all *= cnt_last + 1;
            }
            cnt_last = 0;
            while last * last < m && m % last != 0 {
                last += 1;
            }
            if m % last != 0 {
                last = m;
            }
        }
        cnt_all
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn consecutive_numbers_sum() {
        assert_eq!(Solution::consecutive_numbers_sum(5), 2);
        assert_eq!(Solution::consecutive_numbers_sum(9), 3);
        assert_eq!(Solution::consecutive_numbers_sum(15), 4);
    }
}
