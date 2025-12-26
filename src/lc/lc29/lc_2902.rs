// https://leetcode.com/problems/count-of-sub-multisets-with-bounded-sum/
// 2902. Count of Sub-multisets with Bounded Sum
pub struct Solution;
const MOD: i64 = 1_000_000_007;
impl Solution {
    fn count(m: &Vec<(i32, i32)>, i: usize, l: i32, r: i32, cache: &mut Vec<Vec<i64>>) -> i64 {
        if i == m.len() {
            if l == 0 {
                return 1;
            } else {
                return 0;
            }
        }
        if cache[i][r as usize] != -1 {
            return cache[i][r as usize];
        }
        let mut ret = 0;
        let (n, c) = m[i];
        if n == 0 {
            ret += (Self::count(m, i + 1, l, r, cache) * (c + 1) as i64) % MOD;
        } else {
            for j in 0..=c.min(r / n) {
                ret = (ret + Self::count(m, i + 1, (l - j * n).max(0), r - j * n, cache)) % MOD;
            }
        }
        cache[i][r as usize] = ret;
        ret
    }
    pub fn count_sub_multisets(nums: Vec<i32>, l: i32, r: i32) -> i32 {
        let mut m = std::collections::BTreeMap::<i32, i32>::new();
        for n in nums {
            *m.entry(n).or_default() += 1;
        }
        let m = m.into_iter().collect::<Vec<_>>();
        let mut cache = vec![vec![-1; r as usize + 1]; m.len()];
        Self::count(&m, 0, l, r, &mut cache) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_sub_multisets() {
        assert_eq!(Solution::count_sub_multisets(vec![0, 0, 1, 2, 3], 2, 3), 9);
        assert_eq!(Solution::count_sub_multisets(vec![1, 2, 2, 3], 6, 6), 1);
        assert_eq!(Solution::count_sub_multisets(vec![2, 1, 4, 2, 7], 1, 5), 7);
        assert_eq!(Solution::count_sub_multisets(vec![1, 2, 1, 3, 5, 2], 3, 5), 9);
    }
}
