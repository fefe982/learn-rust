// https://leetcode.com/problems/choose-k-elements-with-maximum-sum/
// 3478. Choose K Elements With Maximum Sum
pub struct Solution;
impl Solution {
    pub fn find_max_sum(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i64> {
        let n = nums1.len();
        let mut nums = nums1.into_iter().zip(nums2.into_iter()).enumerate().collect::<Vec<_>>();
        nums.sort_unstable_by_key(|a| a.1 .0);
        let mut res = vec![0; n];
        let mut sum = 0;
        let mut heap = std::collections::BinaryHeap::new();
        let mut last = 0;
        let mut save = vec![];
        let k = k as usize;
        for (i, (a, b)) in nums.into_iter() {
            if a > last {
                while let Some(s) = save.pop() {
                    heap.push(std::cmp::Reverse(s));
                    sum += s as i64;
                }
            }
            save.push(b);
            last = a;
            while heap.len() > k {
                sum -= heap.pop().unwrap().0 as i64;
            }
            res[i] = sum;
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_max_sum() {
        assert_eq!(
            Solution::find_max_sum(vec![4, 2, 1, 5, 3], vec![10, 20, 30, 40, 50], 2),
            vec![80, 30, 0, 80, 50]
        );
        assert_eq!(
            Solution::find_max_sum(vec![2, 2, 2, 2], vec![3, 1, 2, 3], 2),
            vec![0, 0, 0, 0]
        );
    }
}
