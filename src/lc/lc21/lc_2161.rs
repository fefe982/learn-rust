// https://leetcode.com/problems/partition-array-according-to-given-pivot/
// 2161. Partition Array According to Given Pivot
pub struct Solution;
impl Solution {
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let mut res = vec![0; nums.len()];
        let mut l = 0;
        let mut r = nums.len() - 1;
        for &num in nums.iter() {
            if num < pivot {
                res[l] = num;
                l += 1;
            } else if num > pivot {
                res[r] = num;
                r -= 1;
            }
        }
        for i in l..=r {
            res[i] = pivot;
        }
        let mut i = 1;
        while r + i < nums.len() - i {
            res.swap(r + i, nums.len() - i);
            i += 1;
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pivot_array() {
        assert_eq!(
            Solution::pivot_array(vec![9, 12, 5, 10, 14, 3, 10], 10),
            vec![9, 5, 3, 10, 10, 12, 14]
        );
        assert_eq!(Solution::pivot_array(vec![-3, 4, 3, 2], 2), vec![-3, 2, 4, 3]);
    }
}
