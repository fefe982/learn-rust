// https://leetcode.com/problems/next-greater-element-i/
// 496. Next Greater Element I
pub struct Solution;
impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut s = [-1; 10001];
        let mut v = Vec::with_capacity(nums2.len());
        for n in nums2 {
            while let Some(&l) = v.last() {
                if l < n {
                    s[l as usize] = n;
                    v.pop();
                } else {
                    break;
                }
            }
            v.push(n);
        }
        v.clear();
        for n in nums1 {
            v.push(s[n as usize]);
        }
        v
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn next_greater_element() {
        assert_eq!(
            Solution::next_greater_element(vec![4, 1, 2], vec![1, 3, 4, 2]),
            vec![-1, 3, -1]
        );
        assert_eq!(
            Solution::next_greater_element(vec![2, 4], vec![1, 2, 3, 4]),
            vec![3, -1]
        );
    }
}
