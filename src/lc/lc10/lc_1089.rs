// https://leetcode.com/problems/duplicate-zeros/
// 1089. Duplicate Zeros
pub struct Solution;
impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let mut i = 0;
        let mut j = 0;
        while j < arr.len() {
            if arr[i] == 0 {
                j += 1;
            }
            i += 1;
            j += 1;
        }
        i -= 1;
        j -= 1;
        loop {
            if arr[i] == 0 {
                if j < arr.len() {
                    arr[j] = 0;
                }
                j -= 1;
            }
            arr[j] = arr[i];
            if i == 0 {
                break;
            }
            i -= 1;
            j -= 1;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn check(arr: &mut Vec<i32>, expected: &Vec<i32>) {
        Solution::duplicate_zeros(arr);
        assert_eq!(arr, expected);
    }
    #[test]
    fn duplicate_zeros() {
        check(&mut vec![8, 4, 5, 0, 0, 0, 0, 7], &vec![8, 4, 5, 0, 0, 0, 0, 0]);
        check(&mut vec![0, 1, 7, 6, 0, 2, 0, 7], &vec![0, 0, 1, 7, 6, 0, 0, 2]);
        check(&mut vec![1, 0, 2, 3, 0, 4, 5, 0], &vec![1, 0, 0, 2, 3, 0, 0, 4]);
        check(&mut vec![1, 2, 3], &vec![1, 2, 3]);
    }
}
