// https://leetcode.com/problems/prime-arrangements/
// 1175. Prime Arrangements
pub struct Solution;
impl Solution {
    pub fn num_prime_arrangements(n: i32) -> i32 {
        let prime = [
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
        ];
        let pc = prime.partition_point(|x| *x <= n) as i32;
        let npc = n - pc;
        let mod_num = 1_000_000_007;
        let mut res = 1i64;
        let min = pc.min(npc);
        let max = pc.max(npc);
        for i in 1..=min as i64 {
            res = (res * i * i) % mod_num;
        }
        for i in (min as i64 + 1)..=max as i64 {
            res = (res * i) % mod_num;
        }
        res as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn num_prime_arrangements() {
        assert_eq!(Solution::num_prime_arrangements(5), 12);
        assert_eq!(Solution::num_prime_arrangements(100), 682289015);
    }
}
