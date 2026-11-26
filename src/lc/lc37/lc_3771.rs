// https://leetcode.com/problems/total-score-of-dungeon-runs/
// 3771. Total Score of Dungeon Runs
pub struct Solution;
impl Solution {
    pub fn total_score(hp: i32, damage: Vec<i32>, requirement: Vec<i32>) -> i64 {
        let mut total = 0;
        let mut s = Vec::with_capacity(damage.len());
        let mut sum = 0;
        let hp = hp as i64;
        for i in 0..damage.len() {
            s.push(sum);
            let j = s.partition_point(|&v| hp + v < sum + damage[i] as i64 + requirement[i] as i64);
            total += (i + 1 - j) as i64;
            sum += damage[i] as i64;
        }
        total
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn total_score() {
        assert_eq!(Solution::total_score(11, vec![3, 6, 7], vec![4, 2, 5]), 3);
        assert_eq!(Solution::total_score(2, vec![10000, 1], vec![1, 1]), 1);
    }
}
