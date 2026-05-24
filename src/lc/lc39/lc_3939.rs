// https://leetcode.com/problems/count-non-adjacent-subsets-in-a-rooted-tree/
// 3939. Count Non-Adjacent Subsets in a Rooted Tree
pub struct Solution;
impl Solution {
    pub fn count_valid_subsets(parent: Vec<i32>, nums: Vec<i32>, k: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;

        let n = parent.len();
        let k = k as usize;

        let mut children = vec![Vec::new(); n];
        for (i, &p) in parent.iter().enumerate().skip(1) {
            children[p as usize].push(i);
        }

        let mut dp0 = vec![vec![0_i64; k]; n];
        let mut dp1 = vec![vec![0_i64; k]; n];

        let convolve = |a: &[i64], b: &[i64]| -> Vec<i64> {
            let mut out = vec![0_i64; k];
            for i in 0..k {
                if a[i] == 0 {
                    continue;
                }
                for j in 0..k {
                    if b[j] == 0 {
                        continue;
                    }
                    let idx = (i + j) % k;
                    out[idx] = (out[idx] + a[i] * b[j]) % MOD;
                }
            }
            out
        };

        for u in (0..n).rev() {
            let mut cur0 = vec![0_i64; k];
            let mut cur1 = vec![0_i64; k];

            cur0[0] = 1;
            cur1[(nums[u] as usize) % k] = 1;

            for &v in &children[u] {
                let mut child_any = vec![0_i64; k];
                for r in 0..k {
                    child_any[r] = (dp0[v][r] + dp1[v][r]) % MOD;
                }

                cur0 = convolve(&cur0, &child_any);
                cur1 = convolve(&cur1, &dp0[v]);
            }

            dp0[u] = cur0;
            dp1[u] = cur1;
        }

        let mut ans = (dp0[0][0] + dp1[0][0] - 1) % MOD;
        if ans < 0 {
            ans += MOD;
        }
        ans as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_valid_subsets() {
        assert_eq!(Solution::count_valid_subsets(vec![-1, 0, 1], vec![1, 2, 3], 3), 1);
        assert_eq!(Solution::count_valid_subsets(vec![-1, 0, 0, 0], vec![2, 1, 2, 1], 3), 2);
        assert_eq!(Solution::count_valid_subsets(vec![-1], vec![5], 5), 1);
        assert_eq!(Solution::count_valid_subsets(vec![-1], vec![5], 2), 0);
    }
}
