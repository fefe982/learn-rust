// https://leetcode.com/problems/check-if-n-and-its-double-exist/
// 1346. Check If N and Its Double Exist
pub struct Solution;
impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut arr = arr;
        arr.sort_unstable();
        let mut j = 0;
        for i in 0..arr.len() {
            while j < arr.len() && arr[j] * 2 < arr[i] {
                j += 1;
            }
            if j >= arr.len() {
                return false;
            }
            if i != j && arr[j] * 2 == arr[i] {
                return true;
            }
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_check_if_exist() {
        assert_eq!(Solution::check_if_exist(vec![10, 2, 5, 3]), true);
        assert_eq!(Solution::check_if_exist(vec![3, 1, 7, 11]), false);
    }
}
