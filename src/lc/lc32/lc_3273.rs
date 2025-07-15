// https://leetcode.com/problems/minimum-amount-of-damage-dealt-to-bob/
// 3273. Minimum Amount of Damage Dealt to Bob
pub struct Solution;
impl Solution {
    pub fn min_damage(power: i32, damage: Vec<i32>, health: Vec<i32>) -> i64 {
        let mut dr = damage
            .into_iter()
            .zip(health.into_iter().map(|x| (x + power - 1) / power))
            .collect::<Vec<_>>();
        dr.sort_by(|a, b| (a.1 * b.0).cmp(&(b.1 * a.0)));
        dr.into_iter()
            .fold((0i64, 0i64), |(mut dd, mut rr), (d, r)| {
                rr += r as i64;
                dd += d as i64 * rr as i64;
                (dd, rr)
            })
            .0
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_damage() {
        assert_eq!(Solution::min_damage(4, vec![1, 2, 3, 4], vec![4, 5, 6, 8]), 39);
        assert_eq!(Solution::min_damage(1, vec![1, 1, 1, 1], vec![1, 2, 3, 4]), 20);
        assert_eq!(Solution::min_damage(8, vec![40], vec![59]), 320);
    }
}
