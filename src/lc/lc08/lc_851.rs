// https://leetcode.com/problems/loud-and-rich/
// 851. Loud and Rich
pub struct Solution;
impl Solution {
    pub fn loud_and_rich(richer: Vec<Vec<i32>>, quiet: Vec<i32>) -> Vec<i32> {
        let mut r = vec![vec![]; quiet.len()];
        for rv in richer {
            r[rv[1] as usize].push(rv[0] as usize);
        }
        let mut ans = vec![-1; quiet.len()];
        fn dfs(ans: &mut Vec<i32>, r: &Vec<Vec<usize>>, quiet: &Vec<i32>, i: usize) -> i32 {
            if ans[i] != -1 {
                return ans[i];
            }
            let mut qi = i;
            for &ri in &r[i] {
                let rqi = dfs(ans, r, quiet, ri) as usize;
                if quiet[rqi] < quiet[qi as usize] {
                    qi = rqi;
                }
            }
            ans[i] = qi as i32;
            ans[i]
        }
        for i in 0..quiet.len() {
            dfs(&mut ans, &r, &quiet, i);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn loud_and_rich() {
        assert_eq!(
            Solution::loud_and_rich(
                vec_vec![[1, 0], [2, 1], [3, 1], [3, 7], [4, 3], [5, 3], [6, 3]],
                vec![3, 2, 5, 4, 6, 1, 7, 0]
            ),
            vec![5, 5, 2, 5, 4, 5, 6, 7]
        );
        assert_eq!(Solution::loud_and_rich(vec![], vec![0]), vec![0]);
    }
}
