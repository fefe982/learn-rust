// https://leetcode.com/problems/removing-minimum-number-of-magic-beans/
// 2171. Removing Minimum Number of Magic Beans
pub struct Solution;
impl Solution {
    pub fn minimum_removal(beans: Vec<i32>) -> i64 {
        let mut beans = beans;
        beans.sort_unstable();
        let n = beans.len();
        let mut sum = 0i64;
        for i in 0..n {
            sum += beans[i] as i64;
        }
        let mut ans = sum - beans[0] as i64 * n as i64;
        if ans == 0 {
            return ans;
        }
        let mut last_cnt = ans;
        let mut last = 0;
        for i in 1..n {
            if beans[i] == beans[last] {
                continue;
            }
            last_cnt += beans[last] as i64 * (i - last) as i64;
            last_cnt -= (beans[i] - beans[last]) as i64 * (n - i) as i64;
            ans = ans.min(last_cnt);
            last = i;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_minimum_removal() {
        assert_eq!(Solution::minimum_removal(vec![4, 1, 6, 5]), 4);
        assert_eq!(Solution::minimum_removal(vec![2, 10, 3, 2]), 7);
    }
}
