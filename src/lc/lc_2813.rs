// https://leetcode.com/problems/maximum-elegance-of-a-k-length-subsequence/
// 2813. Maximum Elegance of a K-Length Subsequence
pub struct Solution;
impl Solution {
    pub fn find_maximum_elegance(items: Vec<Vec<i32>>, k: i32) -> i64 {
        let mut items = items;
        items.sort_unstable_by_key(|x| std::cmp::Reverse(x[0]));
        let mut set = std::collections::HashSet::<i32>::new();
        let mut dup = vec![];
        let mut sum = 0;
        let mut res = 0;
        for (i, item) in items.into_iter().enumerate() {
            if i < k as usize {
                sum += item[0] as i64;
                if !set.insert(item[1]) {
                    dup.push(item[0]);
                }
            } else {
                if !dup.is_empty() && set.insert(item[1]) {
                    sum -= dup.pop().unwrap() as i64;
                    sum += item[0] as i64;
                }
            }
            res = res.max(sum + set.len() as i64 * set.len() as i64);
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_find_maximum_elegance() {
        assert_eq!(
            Solution::find_maximum_elegance(vec_vec![[3, 2], [5, 1], [10, 1]], 2),
            17
        );
        assert_eq!(
            Solution::find_maximum_elegance(vec_vec![[3, 1], [3, 1], [2, 2], [5, 3]], 3),
            19
        );
        assert_eq!(Solution::find_maximum_elegance(vec_vec![[1, 1], [2, 1], [3, 1]], 3), 7);
    }
}
