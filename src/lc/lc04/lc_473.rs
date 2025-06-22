// https://leetcode.com/problems/matchsticks-to-square/
// 473. Matchsticks to Square
pub struct Solution;
impl Solution {
    fn dfs(matchsticks: &mut Vec<i32>, sides: &mut Vec<i32>, side: i32, index: usize) -> bool {
        if index == matchsticks.len() {
            return true;
        }
        for i in 0..4 {
            if i > 0 && sides[i - 1] == 0 {
                break;
            }
            if sides[i] + matchsticks[index] <= side {
                sides[i] += matchsticks[index];
                if Self::dfs(matchsticks, sides, side, index + 1) {
                    return true;
                }
                sides[i] -= matchsticks[index];
            }
        }
        false
    }
    pub fn makesquare(matchsticks: Vec<i32>) -> bool {
        let mut side = matchsticks.iter().sum::<i32>();
        if side % 4 != 0 {
            return false;
        }
        side /= 4;
        let mut matchsticks = matchsticks;
        matchsticks.sort_unstable_by(|a, b| b.cmp(a));
        if matchsticks[0] > side {
            return false;
        }
        let mut sides = vec![0; 4];
        Self::dfs(&mut matchsticks, &mut sides, side, 0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn makesquare() {
        assert_eq!(Solution::makesquare(vec![1, 1, 2, 2, 2]), true);
        assert_eq!(Solution::makesquare(vec![3, 3, 3, 3, 4]), false);
    }
}
