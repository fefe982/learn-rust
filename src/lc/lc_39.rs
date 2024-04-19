// https://leetcode.com/problems/combination-sum/
// 39. Combination Sum
pub struct Solution;
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates;
        candidates.sort();
        let mut ans = vec![];
        let mut cand = vec![];
        let mut sum = 0;
        loop {
            if sum >= target {
                if sum == target {
                    ans.push(cand.iter().map(|x| candidates[*x]).collect());
                }
                let i = cand.pop().unwrap();
                sum -= candidates[i];
                while let Some(&i) = cand.last() {
                    if i == candidates.len() - 1 {
                        sum -= candidates[i];
                        cand.pop();
                    } else {
                        break;
                    }
                }
                if cand.is_empty() {
                    break;
                }
                let i = cand.last_mut().unwrap();
                *i += 1;
                sum += candidates[*i] - candidates[*i - 1];
            } else {
                let ni = if let Some(&i) = cand.last() { i } else { 0 };
                cand.push(ni);
                sum += candidates[ni];
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_combination_sum() {
        assert_eq!(Solution::combination_sum(vec![2, 3, 6, 7], 7), vec_vec![[2, 2, 3], [7]]);
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 5], 8),
            vec_vec![[2, 2, 2, 2], [2, 3, 3], [3, 5]]
        );
        assert_eq!(Solution::combination_sum(vec![2], 1), Vec::<Vec<i32>>::new());
    }
}
