// https://leetcode.com/problems/number-of-ways-to-rearrange-sticks-with-k-sticks-visible/
// 1866. Number of Ways to Rearrange Sticks With K Sticks Visible
pub struct Solution;
const MOD: i64 = 1_0000_0000_7;
impl Solution {
    pub fn num_ways(n: usize, k: usize, mem: &mut Vec<Vec<i64>>) -> i64 {
        if n == k {
            return 1;
        }
        if n < k || k == 0 {
            return 0;
        }
        if mem[n][k] != 0 {
            return mem[n][k];
        }
        let ans = (Self::num_ways(n - 1, k - 1, mem) + (n - 1) as i64 * Self::num_ways(n - 1, k, mem) % MOD) % MOD;
        mem[n][k] = ans;
        ans
    }
    pub fn rearrange_sticks(n: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let mut v = vec![vec![0; k + 1]; n + 1];
        Self::num_ways(n, k, &mut v) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rearrange_sticks() {
        assert_eq!(Solution::rearrange_sticks(3, 2), 3);
        assert_eq!(Solution::rearrange_sticks(5, 5), 1);
        assert_eq!(Solution::rearrange_sticks(20, 11), 647427950);
    }
}
