// https://leetcode.com/problems/maximize-happiness-of-selected-children/
// 3075. Maximize Happiness of Selected Children
pub struct Solution;
impl Solution {
    pub fn maximum_happiness_sum(happiness: Vec<i32>, k: i32) -> i64 {
        let mut happiness = happiness;
        happiness.sort_unstable_by(|a, b| b.cmp(a));
        let mut ans = 0;
        let k = k as usize;
        for (i, h) in happiness.into_iter().take(k).enumerate() {
            let hadd = h as i64 - i as i64;
            if hadd < 0 {
                break;
            }
            ans += hadd;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_maximum_happiness_sum() {
        assert_eq!(Solution::maximum_happiness_sum(vec![1, 2, 3], 2), 4);
        assert_eq!(Solution::maximum_happiness_sum(vec![1, 1, 1, 1], 2), 1);
        assert_eq!(Solution::maximum_happiness_sum(vec![2, 3, 4, 5], 1), 5);
    }
}
