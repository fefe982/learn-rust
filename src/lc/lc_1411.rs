// https://leetcode.com/problems/number-of-ways-to-paint-n-3-grid/description/
// 1411. Number of Ways to Paint N × 3 Grid
pub struct Solution;
impl Solution {
    pub fn num_of_ways(n: i32) -> i32 {
        let mut w1 = 6;
        let mut w2 = 6;
        let m: i64 = 1_0000_0000_7;
        for _ in 1..n {
            let nw1 = (w1 + w2) * 2 % m;
            let nw2 = (w1 * 2 + w2 * 3) % m;
            w1 = nw1;
            w2 = nw2;
        }
        ((w1 + w2) % m) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_num_of_ways() {
        assert_eq!(Solution::num_of_ways(1), 12);
        assert_eq!(Solution::num_of_ways(5000), 30228214);
    }
}
