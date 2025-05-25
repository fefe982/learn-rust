// https://leetcode.com/problems/find-k-closest-elements/
// 658. Find K Closest Elements
pub struct Solution;
impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let mut j = arr.partition_point(|&v| v < x);
        let k = k as usize;
        if j == 0 {
            return arr[0..k].to_vec();
        }
        if j == arr.len() {
            return arr[arr.len() - k..].to_vec();
        }
        let mut i = j - 1;
        loop {
            while (j - i) < k && j < arr.len() && arr[j] - x < x - arr[i] {
                j += 1;
            }
            if j == arr.len() {
                return arr[arr.len() - k..].to_vec();
            }
            while i < arr.len() && j - i < k && x - arr[i] <= arr[j] - x {
                i = i.wrapping_sub(1);
            }
            if i > arr.len() {
                return arr[0..k].to_vec();
            }
            if j - i == k {
                if x - arr[i] <= arr[j] - x {
                    return arr[i..j].to_vec();
                }
                return arr[i + 1..j + 1].to_vec();
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_closest_elements() {
        assert_eq!(Solution::find_closest_elements(vec![1, 1, 10, 10, 10], 1, 9), vec![10]);
        assert_eq!(
            Solution::find_closest_elements(vec![1, 2, 3, 4, 5], 4, 3),
            vec![1, 2, 3, 4]
        );
        assert_eq!(
            Solution::find_closest_elements(vec![1, 1, 2, 3, 4, 5], 4, -1),
            vec![1, 1, 2, 3]
        );
    }
}
