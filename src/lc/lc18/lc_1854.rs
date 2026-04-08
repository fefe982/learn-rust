// https://leetcode.com/problems/maximum-population-year/
// 1854. Maximum Population Year
pub struct Solution;
impl Solution {
    pub fn maximum_population(logs: Vec<Vec<i32>>) -> i32 {
        let mut cnt = [0; 101];
        for log in logs {
            cnt[(log[0] - 1950) as usize] += 1;
            cnt[(log[1] - 1950) as usize] -= 1;
        }
        let mut max = cnt[0];
        let mut ans = 1950;
        for i in 1..cnt.len() {
            cnt[i] += cnt[i - 1];
            if cnt[i] > max {
                max = cnt[i];
                ans = (1950 + i) as i32;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_maximum_population() {
        assert_eq!(Solution::maximum_population(vec_vec![[1993, 1999], [2000, 2010]]), 1993);
        assert_eq!(
            Solution::maximum_population(vec_vec![[1950, 1961], [1960, 1971], [1970, 1981]]),
            1960
        );
    }
}
