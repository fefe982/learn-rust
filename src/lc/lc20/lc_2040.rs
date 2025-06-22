// https://leetcode.com/problems/kth-smallest-product-of-two-sorted-arrays/
// 2040. Kth Smallest Product of Two Sorted Arrays
pub struct Solution;
impl Solution {
    fn count(short: &Vec<i32>, long: &Vec<i32>, k: i64) -> i64 {
        let mut cnt = 0;
        for &i in short {
            if i >= 0 {
                cnt += long.partition_point(|&j| j as i64 * i as i64 <= k) as i64;
            } else {
                cnt += long.len() as i64 - long.partition_point(|&j| j as i64 * i as i64 > k) as i64
            }
        }
        cnt
    }
    pub fn kth_smallest_product(nums1: Vec<i32>, nums2: Vec<i32>, k: i64) -> i64 {
        let (short, mut long) = if nums1.len() <= nums2.len() {
            (nums1, nums2)
        } else {
            (nums2, nums1)
        };
        long.sort_unstable();
        let mut low = -100000_00001;
        let mut high = 100000_00001;
        while low + 1 < high {
            let mid = (low + high) / 2;
            if Solution::count(&short, &long, mid) < k {
                low = mid;
            } else {
                high = mid;
            }
        }
        high
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_kth_smallest_product() {
        assert_eq!(Solution::kth_smallest_product(vec![2, 5], vec![3, 4], 2), 8);
        assert_eq!(Solution::kth_smallest_product(vec![-4, -2, 0, 3], vec![2, 4], 6), 0);
        assert_eq!(
            Solution::kth_smallest_product(vec![-2, -1, 0, 1, 2], vec![-3, -1, 2, 4, 5], 3),
            -6
        );
    }
}
