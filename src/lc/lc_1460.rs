// https://leetcode.com/problems/make-two-arrays-equal-by-reversing-subarrays/
// 1460. Make Two Arrays Equal by Reversing Subarrays
pub struct Solution;
impl Solution {
    pub fn can_be_equal(target: Vec<i32>, arr: Vec<i32>) -> bool {
        let mut cnt = vec![0; 1001];
        for i in target.into_iter() {
            cnt[i as usize] += 1;
        }
        for i in arr.into_iter() {
            if cnt[i as usize] == 0 {
                return false;
            } else {
                cnt[i as usize] -= 1;
            }
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_can_be_equal() {
        assert_eq!(Solution::can_be_equal(vec![1, 2, 3, 4], vec![2, 4, 1, 3]), true);
        assert_eq!(Solution::can_be_equal(vec![7], vec![7]), true);
        assert_eq!(Solution::can_be_equal(vec![3, 7, 9], vec![3, 7, 11]), false);
    }
}
