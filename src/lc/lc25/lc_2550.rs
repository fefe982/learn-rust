// https://leetcode.com/problems/count-collisions-of-monkeys-on-a-polygon/
// 2550. Count Collisions of Monkeys on a Polygon
pub struct Solution;
impl Solution {
    pub fn monkey_move(n: i32) -> i32 {
        let mut p2 = 2;
        let mut p = 1;
        let mut n = n;
        const MOD: i64 = 1_000_000_007;
        while n > 0 {
            if n & 1 == 1 {
                p = (p * p2) % MOD;
            }
            p2 = (p2 * p2) % MOD;
            n >>= 1;
        }
        ((p - 2 + MOD) % MOD) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn monkey_move() {
        assert_eq!(Solution::monkey_move(3), 6);
        assert_eq!(Solution::monkey_move(4), 14);
    }
}
