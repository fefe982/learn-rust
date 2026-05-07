// https://leetcode.com/problems/minimum-operations-to-convert-number/
// 2059. Minimum Operations to Convert Number
pub struct Solution;
impl Solution {
    pub fn minimum_operations(nums: Vec<i32>, start: i32, goal: i32) -> i32 {
        let mut dist = vec![-1; 1001];
        let mut queue = std::collections::VecDeque::new();

        if start >= 0 && start <= 1000 {
            dist[start as usize] = 0;
        }
        queue.push_back((start, 0));

        while let Some((i, step)) = queue.pop_front() {
            let step = step + 1;
            for &v in &nums {
                for &j in &[i + v, i - v, i ^ v] {
                    if j == goal {
                        return step;
                    }
                    if j >= 0 && j < 1001 && dist[j as usize] == -1 {
                        dist[j as usize] = step;
                        queue.push_back((j, step));
                    }
                }
            }
        }
        -1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimum_operations() {
        assert_eq!(Solution::minimum_operations(vec![2, 4, 12], 2, 12), 2);
        assert_eq!(Solution::minimum_operations(vec![3, 5, 7], 0, -4), 2);
        assert_eq!(Solution::minimum_operations(vec![2, 8, 16], 0, 1), -1);
    }
}
