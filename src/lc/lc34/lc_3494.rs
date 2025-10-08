// https://leetcode.com/problems/find-the-minimum-amount-of-time-to-brew-potions/
// 3494. Find the Minimum Amount of Time to Brew Potions
pub struct Solution;
impl Solution {
    pub fn min_time(skill: Vec<i32>, mana: Vec<i32>) -> i64 {
        let mut t = vec![0; skill.len() + 1];
        for m in mana {
            let m = m as i64;
            for (i, &s) in skill.iter().enumerate() {
                t[i + 1] = t[i].max(t[i + 1]) + s as i64 * m;
            }
            for i in (0..t.len()).rev().skip(1) {
                t[i] = t[i + 1] - skill[i] as i64 * m;
            }
        }
        t[t.len() - 1]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_time() {
        assert_eq!(Solution::min_time(vec![1, 5, 2, 4], vec![5, 1, 4, 2]), 110);
        assert_eq!(Solution::min_time(vec![1, 1, 1], vec![1, 1, 1]), 5);
        assert_eq!(Solution::min_time(vec![1, 2, 3, 4], vec![1, 2]), 21);
    }
}
