// https://leetcode.com/problems/two-out-of-three/
// 2032. Two Out of Three
pub struct Solution;
impl Solution {
    pub fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
        let mut count = [0u8; 101];
        for &x in nums1.iter() {
            count[x as usize] |= 1;
        }
        for &x in nums2.iter() {
            count[x as usize] |= 2;
        }
        for &x in nums3.iter() {
            count[x as usize] |= 4;
        }
        let mut ans = Vec::new();
        for (i, &c) in count.iter().enumerate() {
            if c.count_ones() >= 2 {
                ans.push(i as i32);
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn two_out_of_three() {
        assert_sort_eq!(
            Solution::two_out_of_three(vec![1, 1, 3, 2], vec![2, 3], vec![3]),
            [3, 2]
        );
        assert_sort_eq!(
            Solution::two_out_of_three(vec![3, 1], vec![2, 3], vec![1, 2]),
            [2, 3, 1]
        );
        assert_sort_eq!(Solution::two_out_of_three(vec![1, 2, 2], vec![4, 3, 3], vec![5]), []);
    }
}
