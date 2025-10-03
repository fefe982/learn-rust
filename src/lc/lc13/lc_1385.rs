// https://leetcode.com/problems/find-the-distance-value-between-two-arrays/
// 1385. Find the Distance Value Between Two Arrays
pub struct Solution;
impl Solution {
    pub fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
        let mut arr1 = arr1;
        let mut arr2 = arr2;
        arr1.sort();
        arr2.sort();
        let mut i = 0;
        let mut j = 1;
        let mut res = 0;
        if arr1[0] < arr2[0] {
            while i < arr1.len() && arr1[i] + d < arr2[0] {
                res += 1;
                i += 1;
            }
            while i < arr1.len() && arr1[i] < arr2[0] {
                i += 1;
            }
        } else {
            while j < arr2.len() && arr2[j] < arr1[0] {
                j += 1;
            }
        }
        while i < arr1.len() && j < arr2.len() {
            while i < arr1.len() && arr1[i] < arr2[j] {
                if arr2[j - 1] + d < arr1[i] && arr1[i] < arr2[j] - d {
                    res += 1;
                }
                i += 1;
            }
            j += 1;
        }
        while i < arr1.len() && arr1[i] <= arr2[arr2.len() - 1] + d {
            i += 1;
        }
        res + (arr1.len() - i) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_the_distance_value() {
        assert_eq!(
            Solution::find_the_distance_value(vec![4, 5, 8], vec![10, 9, 1, 8], 2),
            2
        );
        assert_eq!(
            Solution::find_the_distance_value(vec![1, 4, 2, 3], vec![-4, -3, 6, 10, 20, 30], 3),
            2
        );
        assert_eq!(
            Solution::find_the_distance_value(vec![2, 1, 100, 3], vec![-5, -2, 10, -3, 7], 6),
            1
        );
    }
}
