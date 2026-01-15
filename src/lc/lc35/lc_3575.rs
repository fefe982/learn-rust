// https://leetcode.com/problems/maximum-good-subtree-score/
// 3575. Maximum Good Subtree Score
pub struct Solution;
impl Solution {
    fn dfs(g: &Vec<Vec<usize>>, mask: &Vec<i32>, vals: &Vec<i32>, u: usize) -> (i64, [i64; 1024]) {
        let mut max = [i32::MIN as i64; 1024];
        max[0] = 0;
        let mut sum = 0;
        for &v in &g[u] {
            let (vs, vm) = Self::dfs(g, mask, vals, v);
            sum = sum + vs;
            for i in (1..1024).rev() {
                let mut j = i;
                while j > 0 {
                    max[i] = max[i].max(max[i ^ j] + vm[j]);
                    j = (j - 1) & i;
                }
            }
        }
        if mask[u] > 0 {
            let mu = mask[u] as usize;
            let comp = 1023 ^ mu;
            let mut cm = comp;
            while cm > 0 {
                let union = (cm ^ mu) as usize;
                max[union] = max[union].max(max[cm] + vals[u] as i64);
                cm = (cm - 1) & comp;
            }
            max[mu] = max[mu].max(vals[u] as i64);
        }
        sum += max.into_iter().max().unwrap();
        (sum, max)
    }
    pub fn good_subtree_sum(vals: Vec<i32>, par: Vec<i32>) -> i32 {
        let n = vals.len();
        let mut g = vec![vec![]; n];
        for i in 1..n {
            g[par[i] as usize].push(i);
        }
        let mut mask = vec![0; n];
        for i in 0..n {
            let mut v = vals[i];
            let mut m = 0;
            while v > 0 {
                let b = 1 << (v % 10);
                v /= 10;
                if m & b > 0 {
                    m = -1;
                    break;
                }
                m |= b;
            }
            mask[i] = m;
        }
        (Self::dfs(&g, &mask, &vals, 0).0 % 1_000_000_007) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn good_subtree_sum() {
        assert_eq!(Solution::good_subtree_sum(vec![2, 3], vec![-1, 0]), 8);
        assert_eq!(Solution::good_subtree_sum(vec![1, 5, 2], vec![-1, 0, 0]), 15);
        assert_eq!(Solution::good_subtree_sum(vec![34, 1, 2], vec![-1, 0, 1]), 42);
        assert_eq!(Solution::good_subtree_sum(vec![3, 22, 5], vec![-1, 0, 1]), 18);
    }
}
