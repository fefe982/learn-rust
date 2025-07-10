// https://leetcode.com/problems/find-number-of-ways-to-reach-the-k-th-stair/
// 3154. Find Number of Ways to Reach the K-th Stair
pub struct Solution;
impl Solution {
    pub fn ways_to_reach_stair(k: i32) -> i32 {
        let mut pow = 1;
        let mut ans = 0;
        let mut n = 1;
        let comb = |a: i32, b: i32| -> i32 {
            let a = a.min(b - a) as i64;
            let b = b as i64;
            let mut ans = 1;
            for i in 0..a {
                ans = ans * (b - i) / (i + 1);
            }
            ans as i32
        };
        while pow - n <= k {
            if pow - n <= k && k <= pow {
                ans += comb(pow - k, n);
            }
            pow *= 2;
            n += 1;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ways_to_reach_stair() {
        assert_eq!(Solution::ways_to_reach_stair(0), 2);
        assert_eq!(Solution::ways_to_reach_stair(1), 4);
    }
}
