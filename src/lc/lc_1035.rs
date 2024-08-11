// https://leetcode.com/problems/uncrossed-lines/
// 1035. Uncrossed Lines
pub struct Solution;
impl Solution {
    fn max_uncrossed_lines_slice(nums1: &[i32], nums2: &[i32], cache: &mut Vec<Vec<i32>>) -> i32 {
        if nums1.len() == 0 || nums2.len() == 0 {
            return 0;
        }
        let c = cache[nums1.len()][nums2.len()];
        if c != -1 {
            return c;
        }
        let cnt = if nums1[0] == nums2[0] {
            Self::max_uncrossed_lines_slice(&nums1[1..], &nums2[1..], cache) + 1
        } else {
            std::cmp::max(
                Self::max_uncrossed_lines_slice(&nums1[1..], nums2, cache),
                Self::max_uncrossed_lines_slice(nums1, &nums2[1..], cache),
            )
        };
        cache[nums1.len()][nums2.len()] = cnt;
        cnt
    }
    pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        Self::max_uncrossed_lines_slice(
            &nums1[..],
            &nums2[..],
            &mut vec![vec![-1; nums2.len() + 1]; nums1.len() + 1],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_uncrossed_lines() {
        assert_eq!(Solution::max_uncrossed_lines(vec![1, 4, 2], vec![1, 2, 4]), 2);
        assert_eq!(
            Solution::max_uncrossed_lines(vec![2, 5, 1, 2, 5], vec![10, 5, 2, 1, 5, 2]),
            3
        );
        assert_eq!(
            Solution::max_uncrossed_lines(vec![1, 3, 7, 1, 7, 5], vec![1, 9, 2, 5, 1]),
            2
        );
    }
}
