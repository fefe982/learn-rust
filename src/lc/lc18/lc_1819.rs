// https://leetcode.com/problems/number-of-different-subsequences-gcds/
// 1819. Number of Different Subsequences GCDs
pub struct Solution;
impl Solution {
    fn gcd(mut a: i32, mut b: i32) -> i32 {
        loop {
            if b == 0 {
                return a;
            }
            a %= b;
            if a == 0 {
                return b;
            }
            b %= a;
        }
    }
    pub fn count_different_subsequence_gc_ds(nums: Vec<i32>) -> i32 {
        let mut cnt = vec![false; 200001];
        let mut max = 0;
        for n in nums {
            max = max.max(n);
            cnt[n as usize] = true;
        }
        for i in (1..=max / 3).rev() {
            if cnt[i as usize] {
                continue;
            }
            let mut g = 0;
            for j in 2..=max / i {
                let n = i * j;
                if cnt[n as usize] {
                    g = if g == 0 { n } else { Self::gcd(n, g) };
                }
                if g == i {
                    cnt[i as usize] = true;
                    break;
                }
            }
        }
        let mut total = 0;
        for i in 1..max + 1 {
            if cnt[i as usize] {
                total += 1;
            }
        }
        total
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_different_subsequence_gc_ds() {
        assert_eq!(Solution::count_different_subsequence_gc_ds(vec![6, 10, 3]), 5);
        assert_eq!(Solution::count_different_subsequence_gc_ds(vec![5, 15, 40, 5, 6]), 7);
    }
}
