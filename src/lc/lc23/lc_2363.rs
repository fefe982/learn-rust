// https://leetcode.com/problems/merge-similar-items/
// 2363. Merge Similar Items
pub struct Solution;
impl Solution {
    pub fn merge_similar_items(items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut items1 = items1;
        let mut items2 = items2;
        items1.sort_unstable_by_key(|x| x[0]);
        items2.sort_unstable_by_key(|x| x[0]);
        let mut i = 0;
        let mut j = 0;
        let mut ans = Vec::new();
        while i < items1.len() && j < items2.len() {
            if items1[i][0] == items2[j][0] {
                ans.push(vec![items1[i][0], items1[i][1] + items2[j][1]]);
                i += 1;
                j += 1;
            } else if items1[i][0] < items2[j][0] {
                ans.push(items1[i].clone());
                i += 1;
            } else {
                ans.push(items2[j].clone());
                j += 1;
            }
        }
        while i < items1.len() {
            ans.push(items1[i].clone());
            i += 1;
        }
        while j < items2.len() {
            ans.push(items2[j].clone());
            j += 1;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_merge_similar_items() {
        assert_eq!(
            Solution::merge_similar_items(vec_vec![[1, 1], [4, 5], [3, 8]], vec_vec![[3, 1], [1, 5]]),
            vec_vec![[1, 6], [3, 9], [4, 5]]
        );
        assert_eq!(
            Solution::merge_similar_items(vec_vec![[1, 1], [3, 2], [2, 3]], vec_vec![[2, 1], [3, 2], [1, 3]]),
            vec_vec![[1, 4], [2, 4], [3, 4]]
        );
    }
}
