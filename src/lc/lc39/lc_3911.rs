// https://leetcode.com/problems/k-th-smallest-remaining-even-integer-in-subarray-queries/
// 3911. K-th Smallest Remaining Even Integer in Subarray Queries
pub struct Solution;
impl Solution {
    fn lower_bound_usize(arr: &[usize], target: usize) -> usize {
        let mut lo = 0usize;
        let mut hi = arr.len();
        while lo < hi {
            let mid = lo + ((hi - lo) >> 1);
            if arr[mid] < target {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        lo
    }

    fn upper_bound_usize(arr: &[usize], target: usize) -> usize {
        let mut lo = 0usize;
        let mut hi = arr.len();
        while lo < hi {
            let mid = lo + ((hi - lo) >> 1);
            if arr[mid] <= target {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        lo
    }

    fn upper_bound_i64(arr: &[i64], target: i64) -> usize {
        let mut lo = 0usize;
        let mut hi = arr.len();
        while lo < hi {
            let mid = lo + ((hi - lo) >> 1);
            if arr[mid] <= target {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        lo
    }

    pub fn kth_remaining_integer(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut even_pos = Vec::new();
        let mut even_half = Vec::new();
        for (idx, &v) in nums.iter().enumerate() {
            if (v & 1) == 0 {
                even_pos.push(idx);
                even_half.push((v / 2) as i64);
            }
        }

        let mut ans = Vec::with_capacity(queries.len());
        for q in queries {
            let l = q[0] as usize;
            let r = q[1] as usize;
            let k = q[2] as i64;

            let left = Self::lower_bound_usize(&even_pos, l);
            let right = Self::upper_bound_usize(&even_pos, r);
            let removed_total = (right - left) as i64;

            let mut lo = 1i64;
            let mut hi = k + removed_total;

            while lo < hi {
                let mid = lo + ((hi - lo) >> 1);

                let up = Self::upper_bound_i64(&even_half, mid);
                let removed_le_mid = up.min(right).saturating_sub(left) as i64;
                let kept_le_mid = mid - removed_le_mid;

                if kept_le_mid >= k {
                    hi = mid;
                } else {
                    lo = mid + 1;
                }
            }

            ans.push((lo * 2) as i32);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn kth_remaining_integer() {
        assert_eq!(
            Solution::kth_remaining_integer(vec![1, 4, 7], vec_vec![[0, 2, 1], [1, 1, 2], [0, 0, 3]]),
            vec![2, 6, 6]
        );
        assert_eq!(
            Solution::kth_remaining_integer(vec![2, 5, 8], vec_vec![[0, 1, 2], [1, 2, 1], [0, 2, 4]]),
            vec![6, 2, 12]
        );
        assert_eq!(
            Solution::kth_remaining_integer(vec![3, 6], vec_vec![[0, 1, 1], [1, 1, 3]]),
            vec![2, 8]
        );
    }
}
