// https://leetcode.cn/problems/count-complete-subarrays-in-an-array
// 2799. Count Complete Subarrays in an Array
pub struct Solution;
impl Solution {
    pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
        let mut cnt = 0;
        let mut v = [0; 2001];
        for &n in nums.iter() {
            let n = n as usize;
            v[n] += 1;
            if v[n] == 1 {
                cnt += 1;
            }
        }
        let mut i = 0;
        let mut c = 0;
        let mut v = [0; 2001];
        let mut res = 0;
        for j in 0..nums.len() {
            let nj = nums[j] as usize;
            v[nj] += 1;
            if v[nj] == 1 {
                c += 1;
            }
            if c == cnt {
                while v[nums[i] as usize] > 1 {
                    v[nums[i] as usize] -= 1;
                    i += 1;
                }
                res += i + 1;
            }
        }
        res as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_complete_subarrays() {
        assert_eq!(Solution::count_complete_subarrays(vec![1, 3, 1, 2, 2]), 4);
        assert_eq!(Solution::count_complete_subarrays(vec![5, 5, 5, 5]), 10);
    }
}
