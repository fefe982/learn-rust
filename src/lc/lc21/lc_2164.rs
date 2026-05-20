// https://leetcode.com/problems/sort-even-and-odd-indices-independently/
// 2164. Sort Even and Odd Indices Independently
pub struct Solution;
impl Solution {
    pub fn sort_even_odd(nums: Vec<i32>) -> Vec<i32> {
        let mut even = Vec::new();
        let mut odd = Vec::new();
        for (i, &num) in nums.iter().enumerate() {
            if i % 2 == 0 {
                even.push(num);
            } else {
                odd.push(num);
            }
        }
        even.sort_unstable();
        odd.sort_unstable_by(|a, b| b.cmp(a));
        let mut res = Vec::with_capacity(nums.len());
        for i in 0..nums.len() {
            if i % 2 == 0 {
                res.push(even[i / 2]);
            } else {
                res.push(odd[i / 2]);
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sort_even_odd() {
        assert_eq!(Solution::sort_even_odd(vec![4, 1, 2, 3]), vec![2, 3, 4, 1]);
        assert_eq!(Solution::sort_even_odd(vec![2, 1]), vec![2, 1]);
    }
}
