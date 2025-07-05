// https://leetcode.com/problems/maximum-element-after-decreasing-and-rearranging/
// 1846. Maximum Element After Decreasing and Rearranging
pub struct Solution;
impl Solution {
    pub fn maximum_element_after_decrementing_and_rearranging(arr: Vec<i32>) -> i32 {
        let mut arr = arr;
        arr.sort();
        arr.iter().fold(0, |acc, &x| if x > acc { acc + 1 } else { acc })
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_element_after_decrementing_and_rearranging() {
        assert_eq!(
            Solution::maximum_element_after_decrementing_and_rearranging(vec![73, 98, 9]),
            3
        );
        assert_eq!(
            Solution::maximum_element_after_decrementing_and_rearranging(vec![2, 2, 1, 2, 1]),
            2
        );
        assert_eq!(
            Solution::maximum_element_after_decrementing_and_rearranging(vec![100, 1, 1000]),
            3
        );
        assert_eq!(
            Solution::maximum_element_after_decrementing_and_rearranging(vec![1, 2, 3]),
            3
        );
    }
}
