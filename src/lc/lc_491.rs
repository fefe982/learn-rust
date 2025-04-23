// https://leetcode.com/problems/non-decreasing-subsequences/
// 491. Non-decreasing Subsequences
pub struct Solution;
impl Solution {
    fn dfs(nums: &Vec<i32>, s: &mut std::collections::HashSet<Vec<i32>>, v: &mut Vec<i32>, i: usize) {
        if v.len() >= 2 {
            s.insert(v.clone());
        }
        for j in i..nums.len() {
            if v.is_empty() || nums[j] >= v[v.len() - 1] {
                v.push(nums[j]);
                Solution::dfs(nums, s, v, j + 1);
                v.pop();
            }
        }
    }
    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut s = std::collections::HashSet::<Vec<i32>>::new();
        Solution::dfs(&nums, &mut s, &mut vec![], 0);
        s.into_iter().collect::<Vec<_>>()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn find_subsequences() {
        assert_sort_eq!(
            Solution::find_subsequences(vec![4, 6, 7, 7]),
            vec_vec![
                [4, 6],
                [4, 6, 7],
                [4, 6, 7, 7],
                [4, 7],
                [4, 7, 7],
                [6, 7],
                [6, 7, 7],
                [7, 7]
            ]
        );
        assert_sort_eq!(Solution::find_subsequences(vec![4, 4, 3, 2, 1]), vec_vec![[4, 4]]);
    }
}
