// https://leetcode.com/problems/find-the-least-frequent-digit/
// 3663. Find The Least Frequent Digit
pub struct Solution;
impl Solution {
    pub fn get_least_frequent_digit(n: i32) -> i32 {
        let mut cnt = [0; 10];
        let mut n = n;
        while n > 0 {
            cnt[(n % 10) as usize] += 1;
            n /= 10;
        }
        let mut ans = 0;
        let mut min = i32::MAX;
        for i in 0..10 {
            if cnt[i] > 0 && cnt[i] < min {
                min = cnt[i];
                ans = i;
            }
        }
        ans as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_the_least_frequent_digit() {
        assert_eq!(Solution::get_least_frequent_digit(10), 0);
        assert_eq!(Solution::get_least_frequent_digit(1553322), 1);
        assert_eq!(Solution::get_least_frequent_digit(723344511), 2);
    }
}
