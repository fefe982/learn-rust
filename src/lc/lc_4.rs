// https://leetcode.com/problems/median-of-two-sorted-arrays/
// 4. Median of Two Sorted Arrays
pub struct Solution;
impl Solution {
    fn get_n(nums1: &Vec<i32>, i1: &usize, nums2: &Vec<i32>, i2: &usize) -> i32 {
        if *i1 == 0 {
            return nums2[*i2 - 1];
        }
        if *i2 == 0 {
            return nums1[*i1 - 1];
        }
        if nums1[*i1 - 1] < nums2[*i2 - 1] {
            return nums2[*i2 - 1];
        }
        nums1[*i1 - 1]
    }
    fn find_kth(
        nums1: &Vec<i32>,
        i1: &mut usize,
        nums2: &Vec<i32>,
        i2: &mut usize,
        mut k: usize,
    ) -> i32 {
        loop {
            if k == 1 {
                if *i1 < nums1.len() && *i2 < nums2.len() {
                    if nums1[*i1] < nums2[*i2] {
                        *i1 += 1;
                        return Self::get_n(nums1, i1, nums2, i2);
                    }
                    *i2 += 1;
                    return Self::get_n(nums1, i1, nums2, i2);
                }
                if *i1 < nums1.len() {
                    *i1 += 1;
                    return Self::get_n(nums1, i1, nums2, i2);
                }
                *i2 += 1;
                return Self::get_n(nums1, i1, nums2, i2);
            }
            let nk = k / 2;
            let n1 = if *i1 + nk < nums1.len() {
                nums1[*i1 + nk]
            } else {
                i32::MAX
            };
            let n2 = if *i2 + nk < nums2.len() {
                nums2[*i2 + nk]
            } else {
                i32::MAX
            };
            if n1 < n2 {
                *i1 += nk;
            } else {
                *i2 += nk;
            }
            k -= nk;
        }
    }
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let len = nums1.len() + nums2.len();
        if len == 1 {
            if nums1.len() > 0 {
                return nums1[0] as f64;
            }
            return nums2[0] as f64;
        }
        let mid = (len + 1) / 2;
        let mut i1: usize = 0;
        let mut i2: usize = 0;
        let sum = Self::find_kth(&nums1, &mut i1, &nums2, &mut i2, mid) as f64;
        if len % 2 == 1 {
            return sum;
        }
        return (sum + Self::find_kth(&nums1, &mut i1, &nums2, &mut i2, 1) as f64) / 2_f64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_median_sorted_arrays() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2.0
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5
        );
    }
}
