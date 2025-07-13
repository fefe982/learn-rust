// https://leetcode.com/problems/replace-elements-with-greatest-element-on-right-side/
// 1299. Replace Elements with Greatest Element on Right Side
pub struct Solution;
impl Solution {
    pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
        let mut max = -1;
        let mut arr = arr;
        for i in (0..arr.len()).rev() {
            let tmp = arr[i];
            arr[i] = max;
            if tmp > max {
                max = tmp;
            }
        }
        arr
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_replace_elements() {
        assert_eq!(
            Solution::replace_elements(vec![17, 18, 5, 4, 6, 1]),
            vec![18, 6, 6, 6, 1, -1]
        );
        assert_eq!(Solution::replace_elements(vec![400]), vec![-1]);
    }
}
