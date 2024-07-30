// https://leetcode.com/problems/shortest-impossible-sequence-of-rolls/
// 2350. Shortest Impossible Sequence of Rolls
pub struct Solution;
impl Solution {
    pub fn shortest_sequence(rolls: Vec<i32>, k: i32) -> i32 {
        let mut cnt = vec![0; k as usize];
        let mut target = 1;
        let mut sum = 0;
        for r in rolls {
            if cnt[(r - 1) as usize] < target {
                cnt[(r - 1) as usize] += 1;
                sum += 1;
                if sum == k {
                    target += 1;
                    sum = 0;
                }
            }
        }
        target
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_shortest_sequence() {
        assert_eq!(Solution::shortest_sequence(vec![4, 2, 1, 2, 3, 3, 2, 4, 1], 4), 3);
        assert_eq!(Solution::shortest_sequence(vec![1, 1, 2, 2], 2), 2);
        assert_eq!(Solution::shortest_sequence(vec![1, 1, 3, 2, 2, 2, 3, 3], 4), 1);
    }
}
