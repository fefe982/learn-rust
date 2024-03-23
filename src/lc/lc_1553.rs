// https://leetcode.com/problems/minimum-number-of-days-to-eat-n-oranges/
// 1553. Minimum Number of Days to Eat N Oranges
pub struct Solution;
impl Solution {
    fn md(n: i32, cache: &mut std::collections::HashMap<i32, i32>) -> i32 {
        if n < 3 {
            return n;
        }
        if let Some(v) = cache.get(&n) {
            return *v;
        }
        let v = 1 + std::cmp::min(n % 2 + Solution::md(n / 2, cache), n % 3 + Solution::md(n / 3, cache));
        cache.insert(n, v);
        v
    }
    pub fn min_days(n: i32) -> i32 {
        Self::md(n, &mut std::collections::HashMap::new())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_days() {
        assert_eq!(Solution::min_days(10), 4);
        assert_eq!(Solution::min_days(6), 3);
    }
}
