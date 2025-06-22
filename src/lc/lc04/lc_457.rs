// https://leetcode.com/problems/circular-array-loop/
// 457. Circular Array Loop
pub struct Solution;
impl Solution {
    fn check_loop(nums: &Vec<i32>, visited: &mut Vec<i32>, start: usize) -> bool {
        let len = nums.len() as i32;
        let next = (((start as i32 + nums[start]) % len + len) % len) as usize;
        if nums[start] * nums[next] < 0 || next == start {
            visited[start] = -1;
            false
        } else if visited[next] == 0 {
            visited[next] = 1;
            if Self::check_loop(nums, visited, next) {
                true
            } else {
                visited[start] = -1;
                false
            }
        } else if visited[next] == 1 {
            true
        } else {
            // visited[next] == -1
            visited[start] = -1;
            false
        }
    }
    pub fn circular_array_loop(nums: Vec<i32>) -> bool {
        let mut visited = vec![0; nums.len()];
        for i in 0..nums.len() {
            if visited[i] != 0 {
                continue;
            }
            visited[i] = 1;
            if Self::check_loop(&nums, &mut visited, i) {
                return true;
            }
        }
        false
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn circular_array_loop() {
        assert_eq!(Solution::circular_array_loop(vec![2, -1, 1, 2, 2]), true);
        assert_eq!(Solution::circular_array_loop(vec![-1, -2, -3, -4, -5, 6]), false);
        assert_eq!(Solution::circular_array_loop(vec![1, -1, 5, 1, 4]), true);
    }
}
