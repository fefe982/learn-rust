// https://leetcode.com/problems/count-special-triplets/
// 3583. Count Special Triplets
pub struct Solution;
impl Solution {
    pub fn special_triplets(nums: Vec<i32>) -> i32 {
        let mut c1 = vec![0; 50001];
        let mut c2 = vec![0; 50001];
        let mut ans = 0;
        const MOD: i32 = 1_000_000_007;
        for n in nums {
            if n % 2 == 0 {
                ans = (ans + c2[n as usize / 2]) % MOD;
            }
            if n <= 50000 {
                c2[n as usize] = (c2[n as usize] + c1[n as usize]) % MOD;
            }
            if n % 2 == 0 {
                c1[n as usize / 2] += 1;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn special_triplets() {
        assert_eq!(Solution::special_triplets(vec![6, 3, 6]), 1);
        assert_eq!(Solution::special_triplets(vec![0, 1, 0, 0]), 1);
        assert_eq!(Solution::special_triplets(vec![8, 4, 2, 8, 4]), 2);
    }
}
