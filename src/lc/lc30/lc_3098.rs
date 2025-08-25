// https://leetcode.cn/problems/find-the-sum-of-subsequence-powers/
// 3098. Find the Sum of Subsequence Powers
pub struct Solution;
impl Solution {
    pub fn sum_of_powers(nums: Vec<i32>, k: i32) -> i32 {
        let md = 1000000007i64;
        let mut vals = std::collections::BTreeSet::new();
        let mut nums = nums;
        nums.sort_unstable();
        let n = nums.len();
        for i in 0..n {
            for j in 0..i {
                vals.insert(nums[i] - nums[j]);
            }
        }
        let vals = vals.into_iter().chain([i32::MAX]).collect::<Vec<_>>();
        let k = k as usize;
        // number of subsequences ending at (), with length (), with value index (into vals) ()
        let mut cnt = vec![vec![vec![0; vals.len()]; k + 1]; n];
        // at each iteration, ending index less than or equal to nums[i] - nums[j] of the value: ending at (j), length ()
        //   as i increaes, it increases monotonically
        let mut border = vec![vec![0; k + 1]; n];
        // sum of counts of subsequences with values more than some value, ending at (), with length ()
        let mut suf = vec![vec![0; k + 1]; n];
        // sum of counts of subsequences until now; ending at some index before current iteration,
        //    with length (), and values equals to () accounting the added element in the current iteration
        let mut sum = vec![vec![0; vals.len()]; k + 1];
        let mut res = 0i64;
        for i in 0..n {
            for j in 0..i {
                let pos = vals.binary_search(&(nums[i] - nums[j])).unwrap();
                for p in 1..=k {
                    while border[j][p] < pos {
                        suf[j][p] = (suf[j][p] - cnt[j][p][border[j][p]] + md) % md;
                        sum[p][border[j][p]] = (sum[p][border[j][p]] - suf[j][p] + md) % md;
                        border[j][p] += 1;
                        sum[p][border[j][p]] = (sum[p][border[j][p]] + suf[j][p] + md) % md;
                    }
                }
            }
            cnt[i][1][vals.len() - 1] = 1;
            for p in 2..=k {
                for v in 0..vals.len() {
                    cnt[i][p][v] = sum[p - 1][v];
                }
            }
            for p in 1..=k {
                for v in 0..vals.len() {
                    suf[i][p] = (suf[i][p] + cnt[i][p][v]) % md;
                }
                sum[p][0] = (sum[p][0] + suf[i][p]) % md;
            }
        }
        for i in 0..n {
            for v in 0..vals.len() {
                res = (res + vals[v] as i64 * cnt[i][k][v]) % md;
            }
        }
        res as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sum_of_powers() {
        assert_eq!(Solution::sum_of_powers(vec![1, 2, 3, 4], 3), 4);
        assert_eq!(Solution::sum_of_powers(vec![2, 2], 2), 0);
        assert_eq!(Solution::sum_of_powers(vec![4, 3, -1], 2), 10);
    }
}
