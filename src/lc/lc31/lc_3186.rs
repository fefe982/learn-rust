// https://leetcode.com/problems/maximum-total-damage-with-spell-casting/
// 3186. Maximum Total Damage with Spell Casting
pub struct Solution;
impl Solution {
    pub fn maximum_total_damage(power: Vec<i32>) -> i64 {
        let mut map = std::collections::BTreeMap::new();
        for &p in &power {
            *map.entry(p).or_insert(0) += 1;
        }
        let mut q = std::collections::VecDeque::<(i32, i64)>::new();
        let mut mx = 0;
        for (p, c) in map.into_iter() {
            while let Some(&(p1, c1)) = q.front() {
                if p1 < p - 2 {
                    q.pop_front();
                    mx = mx.max(c1)
                } else {
                    break;
                }
            }
            q.push_back((p, mx + c as i64 * p as i64));
        }
        while let Some((_, c1)) = q.pop_front() {
            mx = mx.max(c1)
        }
        mx
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maximum_total_damage() {
        assert_eq!(Solution::maximum_total_damage(vec![1, 1, 3, 4]), 6);
        assert_eq!(Solution::maximum_total_damage(vec![7, 1, 6, 6]), 13);
    }
}
