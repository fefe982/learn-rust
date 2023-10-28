// https://leetcode.com/problems/peak-index-in-a-mountain-array/
// 852. Peak Index in a Mountain Array
pub struct Solution;
impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let mut low = 0;
        let mut high = arr.len() - 1;
        loop {
            let mid = (low + high) / 2;
            if arr[mid - 1] < arr[mid] && arr[mid] > arr[mid + 1] {
                return mid as i32;
            }
            if arr[mid - 1] > arr[mid] {
                high = mid;
            } else {
                low = mid;
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn peak_index_in_mountain_array() {
        assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 1, 0]), 1);
        assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 2, 1, 0]), 1);
        assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 10, 5, 2]), 1);
    }
}
