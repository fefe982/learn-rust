// https://leetcode.com/problems/minimum-operations-to-make-a-subsequence/
// 1713. Minimum Operations to Make a Subsequence
pub struct Solution;
impl Solution {
    pub fn min_operations(target: Vec<i32>, arr: Vec<i32>) -> i32 {
        let n = target.len();
        let map: std::collections::HashMap<i32, i32> = target.into_iter().zip(0..).collect();
        let mut v = vec![];
        for a in arr {
            if let Some(&i) = map.get(&a) {
                let pos = v.partition_point(|x| *x < i);
                if pos < v.len() {
                    v[pos] = i;
                } else {
                    v.push(i);
                }
            }
        }
        (n - v.len()) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_operations() {
        assert_eq!(
            Solution::min_operations(
                vec![17, 18, 14, 13, 6, 9, 1, 3, 2, 20],
                vec![18, 15, 14, 6, 6, 13, 15, 20, 2, 6]
            ),
            6
        );
        assert_eq!(Solution::min_operations(vec![5, 1, 3], vec![9, 4, 2, 3, 4]), 2);
        assert_eq!(
            Solution::min_operations(vec![6, 4, 8, 1, 3, 2], vec![4, 7, 6, 2, 3, 8, 6, 1]),
            3
        );
    }
}
