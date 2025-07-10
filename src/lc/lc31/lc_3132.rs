// https://leetcode.com/problems/find-the-integer-added-to-array-ii/
// 3132. Find the Integer Added to Array II
pub struct Solution;
impl Solution {
    pub fn minimum_added_integer(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut nums1 = nums1;
        nums1.sort_unstable();
        let mut nums2 = nums2;
        nums2.sort_unstable();
        for i in (0..3).rev() {
            let d = nums2[0] - nums1[i];
            let mut j1 = i + 1;
            let mut j2 = 1;
            while j2 < nums2.len() {
                if j1 >= nums1.len() {
                    break;
                }
                match (nums2[j2] - nums1[j1]).cmp(&d) {
                    std::cmp::Ordering::Equal => {
                        j1 += 1;
                        j2 += 1;
                    }
                    std::cmp::Ordering::Less => {
                        break;
                    }
                    std::cmp::Ordering::Greater => {
                        j1 += 1;
                    }
                }
            }
            if j2 >= nums2.len() {
                return d;
            }
        }
        0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimum_added_integer() {
        assert_eq!(
            Solution::minimum_added_integer(vec![4, 20, 16, 12, 8], vec![14, 18, 10]),
            -2
        );
        assert_eq!(Solution::minimum_added_integer(vec![3, 5, 5, 3], vec![7, 7]), 2);
    }
}
