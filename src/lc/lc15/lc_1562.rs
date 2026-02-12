// https://leetcode.com/problems/find-latest-group-of-size-m/
// 1562. Find Latest Group of Size M
pub struct Solution;
impl Solution {
    pub fn find_latest_step(arr: Vec<i32>, m: i32) -> i32 {
        let mut v = vec![0; arr.len()];
        let mut g = (0..arr.len()).collect::<Vec<_>>();
        let mut sz = vec![1; arr.len()];
        let mut ans = -1;
        let mut c = 0;
        for (ia, i) in arr.into_iter().enumerate() {
            let idx = (i - 1) as usize;
            v[idx] = 1;
            if m == 1 {
                c += 1;
            }
            if idx > 0 && v[idx - 1] == 1 {
                let mut k = g[idx - 1];
                while g[k] != k {
                    k = g[k];
                }
                let mut kk = idx - 1;
                while g[kk] != k {
                    kk = g[kk];
                    g[kk] = k;
                }
                g[idx] = k;
                if sz[k] == m {
                    c -= 1;
                }
                if m == 1 {
                    c -= 1;
                }
                sz[k] += 1;
                if sz[k] == m {
                    c += 1;
                }
            }
            if idx + 1 < v.len() && v[idx + 1] == 1 {
                if sz[idx + 1] == m {
                    c -= 1;
                }
                let mut k = idx;
                while g[k] != k {
                    k = g[k];
                }
                let mut kk = idx;
                while g[kk] != k {
                    kk = g[kk];
                    g[kk] = k;
                }
                g[idx + 1] = k;
                if sz[k] == m {
                    c -= 1;
                }
                sz[k] += sz[idx + 1];
                if sz[k] == m {
                    c += 1;
                }
            }
            if c > 0 {
                ans = (ia + 1) as i32;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_latest_step() {
        assert_eq!(Solution::find_latest_step(vec![10, 6, 9, 4, 7, 8, 5, 2, 1, 3], 1), 8);
        assert_eq!(Solution::find_latest_step(vec![3, 2, 5, 6, 10, 8, 9, 4, 1, 7], 3), 9);
        assert_eq!(Solution::find_latest_step(vec![3, 5, 1, 2, 4], 1), 4);
        assert_eq!(Solution::find_latest_step(vec![3, 1, 5, 4, 2], 2), -1);
    }
}
