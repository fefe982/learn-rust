// https://leetcode.cn/problems/find-occurrences-of-an-element-in-an-array/
// 3159. Find Occurrences of an Element in an Array
pub struct Solution;
impl Solution {
    pub fn occurrences_of_element(nums: Vec<i32>, queries: Vec<i32>, x: i32) -> Vec<i32> {
        let mut idx = vec![];
        for (v, i) in nums.into_iter().zip(0..) {
            if v == x {
                idx.push(i);
            }
        }
        queries
            .into_iter()
            .map(|v| {
                if v > idx.len() as i32 {
                    -1
                } else {
                    idx[(v - 1) as usize]
                }
            })
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_occurrences_of_element() {
        assert_eq!(
            Solution::occurrences_of_element(vec![1, 3, 1, 7], vec![1, 3, 2, 4], 1),
            vec![0, -1, 2, -1]
        );
        assert_eq!(Solution::occurrences_of_element(vec![1, 2, 3], vec![10], 5), [-1]);
    }
}
