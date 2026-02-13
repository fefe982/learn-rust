// https://leetcode.com/problems/detect-pattern-of-length-m-repeated-k-or-more-times/
// 1566. Detect Pattern of Length M Repeated K or More Times
pub struct Solution;
impl Solution {
    pub fn contains_pattern(arr: Vec<i32>, m: i32, k: i32) -> bool {
        if m as usize * k as usize > arr.len() {
            return false;
        }
        for i in 0..arr.len() - m as usize * k as usize + 1 {
            let mut j = i + m as usize;
            let mut cnt = 1;
            while j + m as usize <= arr.len() && arr[i..i + m as usize] == arr[j..j + m as usize] {
                j += m as usize;
                cnt += 1;
            }
            if cnt >= k {
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
    fn contains_pattern() {
        assert_eq!(Solution::contains_pattern(vec![2, 2, 2, 2], 2, 3), false);
        assert_eq!(Solution::contains_pattern(vec![1, 2, 4, 4, 4, 4], 1, 3), true);
        assert_eq!(Solution::contains_pattern(vec![1, 2, 1, 2, 1, 1, 1, 3], 2, 2), true);
        assert_eq!(Solution::contains_pattern(vec![1, 2, 1, 2, 1, 3], 2, 3), false);
    }
}
