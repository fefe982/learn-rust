// https://leetcode.com/problems/divide-players-into-teams-of-equal-skill/
// 2491. Divide Players Into Teams of Equal Skill
pub struct Solution;
impl Solution {
    pub fn divide_players(skill: Vec<i32>) -> i64 {
        let mut cnt = vec![0; 10001];
        let mut sum = 0;
        let n = skill.len() as i32 / 2;
        for s in skill {
            sum += s;
            cnt[s as usize] += 1;
        }
        if sum % n != 0 {
            return -1;
        }
        let sum = sum / n;
        let mut mul = 0;
        if sum % 2 == 0 {
            if cnt[(sum / 2) as usize] % 2 == 1 {
                return -1;
            }
            mul = sum as i64 * sum as i64 * cnt[sum as usize / 2] as i64 / 8;
        }
        for i in 1..=(sum - 1) / 2 {
            if cnt[i as usize] != cnt[(sum - i) as usize] {
                return -1;
            }
            mul += i as i64 * (sum - i) as i64 * cnt[i as usize] as i64;
        }
        mul
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_divide_players() {
        assert_eq!(Solution::divide_players(vec![3, 2, 5, 1, 3, 4]), 22);
        assert_eq!(Solution::divide_players(vec![3, 4]), 12);
        assert_eq!(Solution::divide_players(vec![1, 1, 2, 3]), -1);
    }
}
