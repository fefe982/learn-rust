// https://leetcode.com/problems/kth-missing-positive-number/
// 1539. Kth Missing Positive Number
pub struct Solution;
impl Solution {
    pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
        if arr[arr.len() - 1] - (arr.len() as i32) < k {
            return arr.len() as i32 + k;
        }
        if arr[0] - 1 >= k {
            return k;
        }
        let mut low: usize = 0;
        let mut high: usize = arr.len() - 1;
        while low < high {
            if high - low == 1 {
                return k + high as i32;
            }
            let mid = (low + high) / 2;
            if arr[mid] - mid as i32 - 1 >= k {
                high = mid;
            } else {
                low = mid;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_kth_positive() {
        assert_eq!(Solution::find_kth_positive(vec![2, 3, 4, 7, 11], 5), 9);
        assert_eq!(Solution::find_kth_positive(vec![1, 2, 3, 4], 2), 6);
    }
}
