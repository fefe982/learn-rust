// https://leetcode.com/problems/sliding-window-median/
// 480. Sliding Window Median
pub struct Solution;
impl Solution {
    fn get_median(
        low: &std::collections::BinaryHeap<(i32, std::cmp::Reverse<usize>)>,
        high: &std::collections::BinaryHeap<(std::cmp::Reverse<i32>, std::cmp::Reverse<usize>)>,
        k: usize,
    ) -> f64 {
        let l = low.peek().unwrap().0 as f64;
        if k % 2 == 0 {
            (l + high.peek().unwrap().0 .0 as f64) / 2.0
        } else {
            l
        }
    }
    pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
        let mut high = std::collections::BinaryHeap::new();
        let mut low = std::collections::BinaryHeap::new();
        let k = k as usize;
        for i in 0..k {
            low.push((nums[i], std::cmp::Reverse(i)));
        }
        for _ in 0..k / 2 {
            let (n, i) = low.pop().unwrap();
            high.push((std::cmp::Reverse(n), i));
        }
        let mut res = vec![Self::get_median(&low, &high, k)];
        for i in k..nums.len() {
            if nums[i] <= low.peek().unwrap().0 {
                low.push((nums[i], std::cmp::Reverse(i)));
                if nums[i - k] > low.peek().unwrap().0
                    || (nums[i - k] == low.peek().unwrap().0 && low.peek().unwrap().1 .0 > i - k)
                {
                    let (n, i) = low.pop().unwrap();
                    high.push((std::cmp::Reverse(n), i));
                }
            } else {
                high.push((std::cmp::Reverse(nums[i]), std::cmp::Reverse(i)));
                if nums[i - k] < low.peek().unwrap().0 || low.peek().unwrap().1 .0 == i - k {
                    let (n, i) = high.pop().unwrap();
                    low.push((n.0, i));
                }
            }
            while let Some(&(_, id)) = low.peek() {
                if id.0 <= i - k {
                    low.pop();
                } else {
                    break;
                }
            }
            while let Some(&(_, id)) = high.peek() {
                if id.0 <= i - k {
                    high.pop();
                } else {
                    break;
                }
            }
            res.push(Self::get_median(&low, &high, k));
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn median_sliding_window() {
        assert_eq!(
            Solution::median_sliding_window(vec![1, 1, 1, 1], 2),
            vec![1.0, 1.0, 1.0]
        );
        assert_eq!(
            Solution::median_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
            vec![1.00000, -1.00000, -1.00000, 3.00000, 5.00000, 6.00000]
        );
        assert_eq!(
            Solution::median_sliding_window(vec![1, 2, 3, 4, 2, 3, 1, 4, 2], 3),
            vec![2.00000, 3.00000, 3.00000, 3.00000, 2.00000, 3.00000, 2.00000]
        );
    }
}
