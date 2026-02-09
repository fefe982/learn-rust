// https://leetcode.com/problems/count-subarrays-with-cost-less-than-or-equal-to-k/
// 3835. Count Subarrays With Cost Less Than or Equal to K
pub struct Solution;
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
        let mut minh = std::collections::BinaryHeap::new();
        let mut maxh = std::collections::BinaryHeap::new();
        let mut i = 0;
        let mut cnt = 0;
        for j in 0..nums.len() {
            minh.push(std::cmp::Reverse((nums[j] as i64, j)));
            maxh.push((nums[j] as i64, j));
            loop {
                while let Some(&std::cmp::Reverse((_, ih))) = minh.peek() {
                    if ih < i {
                        minh.pop();
                    } else {
                        break;
                    }
                }
                while let Some(&(_, ih)) = maxh.peek() {
                    if ih < i {
                        maxh.pop();
                    } else {
                        break;
                    }
                }
                let c = (j - i + 1) as i64;
                let val = (maxh.peek().unwrap().0 - minh.peek().unwrap().0 .0) * c;
                if val <= k {
                    cnt += c;
                    break;
                } else {
                    i += 1;
                }
            }
        }
        cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_subarrays() {
        assert_eq!(Solution::count_subarrays(vec![1, 3, 2], 4), 5);
        assert_eq!(Solution::count_subarrays(vec![5, 5, 5, 5], 0), 10);
        assert_eq!(Solution::count_subarrays(vec![1, 2, 3], 0), 3);
    }
}
