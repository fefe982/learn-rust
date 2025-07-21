// https://leetcode.com/problems/minimize-deviation-in-array/
// 1675. Minimize Deviation in Array
use std::cmp;
use std::collections::BinaryHeap;
pub struct Solution;
impl Solution {
    pub fn minimum_deviation(nums: Vec<i32>) -> i32 {
        let mut min_dev = i32::MAX;
        let mut min_n = i32::MAX;
        let mut heap = BinaryHeap::with_capacity(nums.len());
        for n in nums.iter() {
            let mut num = *n;
            if num % 2 == 1 {
                num *= 2;
            }
            if num < min_n {
                min_n = num;
            }
            heap.push(num);
        }
        while let Some(max_n) = heap.pop() {
            min_dev = cmp::min(max_n - min_n, min_dev);
            if max_n % 2 == 1 {
                break;
            }
            min_n = cmp::min(min_n, max_n / 2);
            heap.push(max_n / 2);
        }
        min_dev
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimum_deviation() {
        assert_eq!(Solution::minimum_deviation(vec![1, 2, 3, 4]), 1);
        assert_eq!(Solution::minimum_deviation(vec![4, 1, 5, 20, 3]), 3);
        assert_eq!(Solution::minimum_deviation(vec![2, 10, 8]), 3);
    }
}
