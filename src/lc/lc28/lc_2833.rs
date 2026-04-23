// https://leetcode.com/problems/furthest-point-from-origin/
// 2833. Furthest Point From Origin
pub struct Solution;
impl Solution {
    pub fn furthest_distance_from_origin(moves: String) -> i32 {
        let mut lr: i32 = 0;
        let mut mv = 0;
        for c in moves.chars() {
            match c {
                'L' => lr += 1,
                'R' => lr -= 1,
                '_' => mv += 1,
                _ => unreachable!(),
            }
        }
        lr.abs() + mv
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_furthest_distance_from_origin() {
        assert_eq!(Solution::furthest_distance_from_origin("L_RL__R".to_string()), 3);
        assert_eq!(Solution::furthest_distance_from_origin("_R__LL_".to_string()), 5);
        assert_eq!(Solution::furthest_distance_from_origin("_______".to_string()), 7);
    }
}
