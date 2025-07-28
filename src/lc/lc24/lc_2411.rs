// https://leetcode.com/problems/smallest-subarrays-with-maximum-bitwise-or/
// 2411. Smallest Subarrays With Maximum Bitwise OR
pub struct Solution;
impl Solution {
    pub fn smallest_subarrays(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut idx = vec![Vec::with_capacity(n); 32];

        for (i, n) in nums.into_iter().enumerate() {
            let mut n = n as u32;
            while n != 0 {
                idx[n.trailing_zeros() as usize].push(i);
                n = n & (n - 1);
            }
        }
        let mut ans = Vec::with_capacity(n);
        let mut cidx = vec![0; 32];
        let mut ending = 0;
        for i in 0..n {
            for j in 0..32 {
                if cidx[j] < idx[j].len() && idx[j][cidx[j]] < i {
                    cidx[j] += 1;
                }
                if cidx[j] < idx[j].len() {
                    ending = ending.max(idx[j][cidx[j]]);
                }
            }
            ending = ending.max(i);
            ans.push((ending - i + 1) as i32);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn smallest_subarrays() {
        assert_eq!(Solution::smallest_subarrays(vec![1, 0, 2, 1, 3]), vec![3, 3, 2, 2, 1]);
        assert_eq!(Solution::smallest_subarrays(vec![1, 2]), vec![2, 1]);
    }
}
