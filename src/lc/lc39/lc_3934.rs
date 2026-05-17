// https://leetcode.com/problems/smallest-unique-subarray/
// 3934. Smallest Unique Subarray
pub struct Solution;
impl Solution {
    pub fn smallest_unique_subarray(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let sa = Self::suffix_array(&nums);
        let lcp = Self::lcp_array(&nums, &sa);

        let mut ans = n;
        for i in 0..n {
            let common = lcp[i].max(if i + 1 < n { lcp[i + 1] } else { 0 });
            let start = sa[i];
            if start + common < n {
                ans = ans.min(common + 1);
            }
        }

        ans as i32
    }

    fn suffix_array(nums: &[i32]) -> Vec<usize> {
        let n = nums.len();
        let mut sa: Vec<usize> = (0..n).collect();
        let mut rank = nums.to_vec();
        let mut next_rank = vec![0; n];
        let mut len = 1;

        while len < n {
            sa.sort_unstable_by(|&a, &b| {
                let left = (rank[a], if a + len < n { rank[a + len] } else { -1 });
                let right = (rank[b], if b + len < n { rank[b + len] } else { -1 });
                left.cmp(&right)
            });

            next_rank[sa[0]] = 0;
            for i in 1..n {
                let prev = sa[i - 1];
                let cur = sa[i];
                let prev_key = (rank[prev], if prev + len < n { rank[prev + len] } else { -1 });
                let cur_key = (rank[cur], if cur + len < n { rank[cur + len] } else { -1 });
                next_rank[cur] = next_rank[prev] + if prev_key == cur_key { 0 } else { 1 };
            }

            rank.clone_from(&next_rank);
            if rank[sa[n - 1]] as usize == n - 1 {
                break;
            }
            len <<= 1;
        }

        sa
    }

    fn lcp_array(nums: &[i32], sa: &[usize]) -> Vec<usize> {
        let n = nums.len();
        let mut rank = vec![0; n];
        for (i, &start) in sa.iter().enumerate() {
            rank[start] = i;
        }

        let mut lcp = vec![0; n];
        let mut len = 0;
        for i in 0..n {
            let r = rank[i];
            if r > 0 {
                let j = sa[r - 1];
                while i + len < n && j + len < n && nums[i + len] == nums[j + len] {
                    len += 1;
                }
                lcp[r] = len;
                len = len.saturating_sub(1);
            }
        }

        lcp
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_smallest_unique_subarray() {
        assert_eq!(Solution::smallest_unique_subarray(vec![3, 3, 3]), 3);
        assert_eq!(Solution::smallest_unique_subarray(vec![2, 1, 2, 3, 3]), 1);
        assert_eq!(Solution::smallest_unique_subarray(vec![1, 1, 2, 2, 1]), 2);
    }
}
