// https://leetcode.com/problems/reduce-array-size-to-the-half/
// 1338. Reduce Array Size to The Half
pub struct Solution;
impl Solution {
    pub fn min_set_size(arr: Vec<i32>) -> i32 {
        let mut cnt = vec![0; 100001];
        let len = arr.len() as i32;
        for i in arr.into_iter() {
            cnt[i as usize] += 1;
        }
        cnt.sort_unstable();
        let ic = cnt.len() as i32;
        let mut sum = 0;
        let hlen = len / 2;
        for (i, c) in cnt.into_iter().enumerate().rev() {
            sum += c;
            if sum >= hlen {
                return ic - i as i32;
            }
        }
        0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_set_size() {
        assert_eq!(Solution::min_set_size(vec![3, 3, 3, 3, 5, 5, 5, 2, 2, 7]), 2);
        assert_eq!(Solution::min_set_size(vec![7, 7, 7, 7, 7, 7]), 1);
    }
}
