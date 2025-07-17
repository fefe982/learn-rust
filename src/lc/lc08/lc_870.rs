// https://leetcode.com/problems/advantage-shuffle/
// 870. Advantage Shuffle
pub struct Solution;
impl Solution {
    pub fn advantage_count(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut nums1 = nums1;
        let mut nums2 = nums2.into_iter().enumerate().collect::<Vec<_>>();
        nums1.sort();
        nums2.sort_by_key(|(_, v)| *v);
        let mut ans = vec![0; nums1.len()];
        let mut i = 0;
        let mut j = nums1.len() - 1;
        for k in 0..nums1.len() {
            if nums1[k] > nums2[i].1 {
                ans[nums2[i].0] = nums1[k];
                i += 1;
            } else {
                ans[nums2[j].0] = nums1[k];
                j -= 1;
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    fn check(nums1: Vec<i32>, nums2: Vec<i32>, expect: Vec<i32>) {
        let adv = nums2.iter().zip(expect.iter()).filter(|&(a, b)| a < b).count();
        let ans = Solution::advantage_count(nums1, nums2.clone());
        let adv_ans = nums2.iter().zip(ans.iter()).filter(|&(a, b)| a < b).count();
        assert_sort_eq!(ans, expect);
        assert_eq!(adv, adv_ans);
    }
    #[test]
    fn advantage_count() {
        check(vec![2, 7, 11, 15], vec![1, 10, 4, 11], vec![2, 11, 7, 15]);
        check(vec![12, 24, 8, 32], vec![13, 25, 32, 11], vec![24, 32, 8, 12]);
    }
}
