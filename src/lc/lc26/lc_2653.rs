// https://leetcode.com/problems/sliding-subarray-beauty/
// 2653. Sliding Subarray Beauty
pub struct Solution;
impl Solution {
    pub fn get_subarray_beauty(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let mut cnt = vec![0; 50];
        let k = k as usize;
        let mut ans = Vec::with_capacity(nums.len() + 1 - k);
        for i in 0..nums.len() {
            if nums[i] < 0 {
                cnt[(nums[i] + 50).abs() as usize] += 1;
            }
            if i >= k {
                if nums[i - k] < 0 {
                    cnt[(nums[i - k] + 50).abs() as usize] -= 1;
                }
            }
            if i + 1 >= k {
                let mut s = 0;
                let mut b = 0;
                for j in 0..cnt.len() {
                    s += cnt[j];
                    if s >= x {
                        b = j as i32 - 50;
                        break;
                    }
                }
                ans.push(b);
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_subarray_beauty() {
        assert_eq!(
            Solution::get_subarray_beauty(vec![1, -1, -3, -2, 3], 3, 2),
            vec![-1, -2, -2]
        );
        assert_eq!(
            Solution::get_subarray_beauty(vec![-1, -2, -3, -4, -5], 2, 2),
            [-1, -2, -3, -4]
        );
    }
}
