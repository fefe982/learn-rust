// https://leetcode.com/problems/minimum-cost-to-merge-sorted-lists/
// 3801. Minimum Cost to Merge Sorted Lists
pub struct Solution;
impl Solution {
    pub fn min_merge_cost(lists: Vec<Vec<i32>>) -> i64 {
        let nlists = lists.len();
        let n = 1 << nlists;
        let mut len = vec![0; n];
        for mask in 1..n {
            let m = mask & (mask - 1);
            if m == 0 {
                len[mask] = lists[mask.trailing_zeros() as usize].len();
            } else {
                len[mask] = len[m] + len[mask ^ m];
            }
        }
        let mut all_list = lists.iter().flatten().cloned().collect::<Vec<_>>();
        all_list.sort_unstable();
        let mut j = 0;
        for i in 1..all_list.len() {
            if all_list[i] != all_list[j] {
                j += 1;
                all_list[j] = all_list[i];
            }
        }
        all_list.truncate(j + 1);
        let count_less_eq = |mut mask: usize, n: i32| -> i32 {
            let mut c = 0;
            while mask > 0 {
                let i = mask.trailing_zeros() as usize;
                c += lists[i].partition_point(|&x| x <= n) as i32;
                mask ^= 1 << i;
            }
            c
        };
        let mut median = vec![0; n];
        for mask in 1..n {
            let mid_count = (len[mask] + 1) / 2;
            let mut low = 0;
            let mut high = all_list.len();
            while low + 1 < high {
                let mid = (low + high) / 2;
                if count_less_eq(mask, all_list[mid - 1]) < mid_count as i32 {
                    low = mid;
                } else {
                    high = mid;
                }
            }
            median[mask] = all_list[low];
        }
        let mut dp = vec![i64::MAX; n];
        for mask in 1..n {
            let mut m = mask & (mask - 1);
            if m == 0 {
                dp[mask] = 0;
            } else {
                while m > mask ^ m {
                    dp[mask] = dp[mask].min(
                        dp[m]
                            + dp[mask ^ m]
                            + len[m] as i64
                            + len[mask ^ m] as i64
                            + (median[m] as i64 - median[mask ^ m] as i64).abs(),
                    );
                    m = mask & (m - 1);
                }
            }
        }
        dp[n - 1]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn min_merge_cost() {
        assert_eq!(Solution::min_merge_cost(vec_vec![[1, 3, 5], [2, 4], [6, 7, 8]]), 18);
        assert_eq!(Solution::min_merge_cost(vec_vec![[1, 1, 5], [1, 4, 7, 8]]), 10);
        assert_eq!(Solution::min_merge_cost(vec_vec![[1], [3]]), 4);
        assert_eq!(Solution::min_merge_cost(vec_vec![[1], [1]]), 2);
    }
}
