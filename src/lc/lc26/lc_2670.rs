// https://leetcode.com/problems/find-the-distinct-difference-array/
// 2670. Find the Distinct Difference Array
pub struct Solution;
impl Solution {
    pub fn distinct_difference_array(nums: Vec<i32>) -> Vec<i32> {
        let mut ele = vec![];
        let mut m = std::collections::HashSet::new();
        for &n in nums.iter() {
            m.insert(n);
            ele.push(m.len() as i32);
        }
        m.clear();
        for (i, n) in nums.into_iter().enumerate().rev() {
            ele[i] -= m.len() as i32;
            m.insert(n);
        }
        ele
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_distinct_difference_array() {
        assert_eq!(
            Solution::distinct_difference_array(vec![1, 2, 3, 4, 5]),
            vec![-3, -1, 1, 3, 5]
        );
        assert_eq!(
            Solution::distinct_difference_array(vec![3, 2, 3, 4, 2]),
            vec![-2, -1, 0, 2, 3]
        );
    }
}
