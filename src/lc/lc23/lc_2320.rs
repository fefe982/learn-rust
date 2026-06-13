// https://leetcode.com/problems/count-number-of-ways-to-place-houses/
// 2320. Count Number of Ways to Place Houses
pub struct Solution;
impl Solution {
    pub fn count_house_placements(n: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let mut p0 = 1;
        let mut p1 = 1;
        for _ in 1..n {
            let np0 = (p0 + p1) % MOD;
            let np1 = p0;
            p0 = np0;
            p1 = np1;
        }
        (((p0 + p1) * (p0 + p1)) % MOD) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_house_placements() {
        assert_eq!(Solution::count_house_placements(1), 4);
        assert_eq!(Solution::count_house_placements(2), 9);
    }
}
