// https://leetcode.com/problems/minimum-processing-time/
// 2895. Minimum Processing Time
pub struct Solution;
impl Solution {
    pub fn min_processing_time(processor_time: Vec<i32>, tasks: Vec<i32>) -> i32 {
        let mut processor_time = processor_time;
        processor_time.sort_unstable();
        let mut tasks = tasks;
        tasks.sort_unstable();
        let mut ans = 0;
        for i in 0..processor_time.len() {
            ans = ans.max(processor_time[i] + tasks[tasks.len() - 1 - i * 4]);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_processing_time() {
        assert_eq!(
            Solution::min_processing_time(vec![8, 10], vec![2, 2, 3, 1, 8, 7, 4, 5]),
            16
        );
        assert_eq!(
            Solution::min_processing_time(vec![10, 20], vec![2, 3, 1, 2, 5, 8, 4, 3]),
            23
        );
    }
}
