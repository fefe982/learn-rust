// https://leetcode.com/problems/maximum-performance-of-a-team/
// 1383. Maximum Performance of a Team
pub struct Solution;
impl Solution {
    pub fn max_performance(_n: i32, speed: Vec<i32>, efficiency: Vec<i32>, k: i32) -> i32 {
        let mut speed_efficiency = speed.into_iter().zip(efficiency.into_iter()).collect::<Vec<_>>();
        speed_efficiency.sort_by_key(|x| std::cmp::Reverse(x.1));
        let mut heap = std::collections::BinaryHeap::new();
        let mut sum = 0;
        let mut max = 0;
        for (s, e) in speed_efficiency {
            sum += s as i64;
            max = max.max(sum * e as i64);
            heap.push(std::cmp::Reverse(s));
            if heap.len() >= k as usize {
                sum -= heap.pop().unwrap().0 as i64;
            }
        }
        (max % 1_0000_0000_7) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_performace() {
        assert_eq!(
            Solution::max_performance(6, vec![2, 10, 3, 1, 5, 8], vec![5, 4, 3, 9, 7, 2], 2),
            60
        );
        assert_eq!(
            Solution::max_performance(6, vec![2, 10, 3, 1, 5, 8], vec![5, 4, 3, 9, 7, 2], 3),
            68
        );
        assert_eq!(
            Solution::max_performance(6, vec![2, 10, 3, 1, 5, 8], vec![5, 4, 3, 9, 7, 2], 4),
            72
        );
    }
}
