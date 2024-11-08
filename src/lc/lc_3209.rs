// https://leetcode.com/problems/number-of-subarrays-with-and-value-of-k/
// 3209. Number of Subarrays With AND Value of K
pub struct Solution;
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let mut cnt = vec![vec![]; 30];
        let max = (1 << 30) - 1;
        for (i, n) in nums.iter().enumerate() {
            let mut n = n ^ max;
            while n > 0 {
                let bit = n.trailing_zeros() as usize;
                cnt[bit].push(i);
                n -= 1 << bit;
            }
        }
        let mut next0 = vec![0; 30];
        let mut ans = 0;
        let len = nums.len();
        'nextn: for (i, n) in nums.into_iter().enumerate() {
            if n & k != k {
                continue;
            }
            let mut beg = i;
            let mut end = len;
            let mut n = n;
            while n > 0 {
                let bit = n.trailing_zeros() as usize;
                let bitmask = 1 << bit;
                while next0[bit] < cnt[bit].len() && cnt[bit][next0[bit]] < i {
                    next0[bit] += 1;
                }
                if bitmask & k > 0 {
                    if next0[bit] < cnt[bit].len() {
                        end = end.min(cnt[bit][next0[bit]]);
                    }
                } else {
                    if next0[bit] >= cnt[bit].len() {
                        continue 'nextn;
                    }
                    beg = beg.max(cnt[bit][next0[bit]]);
                }
                n -= bitmask;
            }
            if end > beg {
                ans += (end - beg) as i64;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_subarrays() {
        assert_eq!(Solution::count_subarrays(vec![1, 1, 1], 1), 6);
        assert_eq!(Solution::count_subarrays(vec![1, 1, 2], 1), 3);
        assert_eq!(Solution::count_subarrays(vec![1, 2, 3], 2), 2);
    }
}
