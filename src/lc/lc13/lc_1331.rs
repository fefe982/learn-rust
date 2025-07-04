// https://leetcode.com/problems/rank-transform-of-an-array/
// 1331. Rank Transform of an Array
pub struct Solution;
impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        let mut m = std::collections::HashMap::<i32, i32>::new();
        let mut arrs = arr.clone();
        arrs.sort_unstable();
        arrs.dedup();
        for (v, i) in arrs.into_iter().zip(1..) {
            m.insert(v, i);
        }
        let mut arr = arr;
        for i in 0..arr.len() {
            arr[i] = *m.get(&arr[i]).unwrap();
        }
        arr
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_array_rank_transform() {
        assert_eq!(Solution::array_rank_transform(vec![40, 10, 20, 30]), [4, 1, 2, 3]);
        assert_eq!(Solution::array_rank_transform(vec![100, 100, 100]), [1, 1, 1]);
        assert_eq!(
            Solution::array_rank_transform(vec![37, 12, 28, 9, 100, 56, 80, 5, 12]),
            [5, 3, 4, 2, 8, 6, 7, 1, 3]
        );
    }
}
