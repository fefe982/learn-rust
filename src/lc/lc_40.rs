// https://leetcode.com/problems/combination-sum-ii/
// 40. Combination Sum II
pub struct Solution;
impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates;
        candidates.sort_unstable();
        let mut res = Vec::new();
        let mut stk = vec![(candidates[0], 0)];
        let mut sum = candidates[0];
        loop {
            while sum < target {
                let &(_, i) = stk.last().unwrap();
                if i + 1 < candidates.len() {
                    sum += candidates[i + 1];
                    stk.push((candidates[i + 1], i + 1));
                } else {
                    break;
                }
            }
            if sum == target {
                res.push(stk.iter().map(|&(x, _)| x).collect());
            }
            sum -= stk.pop().unwrap().0;
            'stk: while let Some((v, i)) = stk.pop() {
                sum -= v;
                for j in i + 1..candidates.len() {
                    if candidates[j] != v {
                        sum += candidates[j];
                        stk.push((candidates[j], j));
                        break 'stk;
                    }
                }
            }
            if stk.is_empty() {
                break;
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_combination_sum2() {
        assert_eq!(Solution::combination_sum2(vec![1], 2), Vec::<Vec<i32>>::new());
        assert_eq!(
            Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8),
            vec_vec![[1, 1, 6], [1, 2, 5], [1, 7], [2, 6]]
        );
        assert_eq!(
            Solution::combination_sum2(vec![2, 5, 2, 1, 2], 5),
            vec_vec![[1, 2, 2], [5]]
        );
    }
}
