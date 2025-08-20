// https://leetcode.com/problems/minimum-cost-to-cut-a-stick/
// 1547. Minimum Cost to Cut a Stick
pub struct Solution;
impl Solution {
    fn cost(
        low: i32,
        high: i32,
        cuts: &[i32],
        cache: &mut std::collections::HashMap<(i32, i32), i32>,
    ) -> i32 {
        if cuts.len() == 1 {
            return high - low;
        }
        if let Some(&c) = cache.get(&(low, high)) {
            return c;
        }
        let l = cuts.len();
        let mut m = std::cmp::min(
            Self::cost(cuts[0], high, &cuts[1..], cache),
            Self::cost(low, cuts[l - 1], &cuts[0..l - 1], cache),
        );
        for i in 1..l - 1 {
            m = std::cmp::min(
                m,
                Self::cost(low, cuts[i], &cuts[0..i], cache)
                    + Self::cost(cuts[i], high, &cuts[i + 1..], cache),
            );
        }
        m += high - low;
        cache.insert((low, high), m);
        m
    }
    pub fn min_cost(n: i32, mut cuts: Vec<i32>) -> i32 {
        cuts.sort_unstable();
        Self::cost(0, n, &cuts[..], &mut std::collections::HashMap::new())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_cost() {
        assert_eq!(Solution::min_cost(7, vec![1, 3, 4, 5]), 16);
        assert_eq!(Solution::min_cost(9, vec![5, 6, 1, 4, 2]), 22);
    }
}
