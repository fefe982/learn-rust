// https://leetcode.com/problems/stone-game-iii/
// 1406. Stone Game III
pub struct Solution;
impl Solution {
    fn st_gm(
        stone_value: &[i32],
        cache: &mut std::collections::HashMap<usize, (i32, i32)>,
    ) -> (i32, i32) {
        if stone_value.len() == 0 {
            return (0, 0);
        }
        if let Some(&r) = cache.get(&stone_value.len()) {
            return r;
        }
        let mut max = i32::MIN;
        let mut other = 0;
        let mut sum = 0;
        for i in 0..3 {
            if i >= stone_value.len() {
                break;
            }
            sum += stone_value[i];
            let (m, o) = Self::st_gm(&stone_value[i + 1..], cache);
            if sum + o > max {
                max = sum + o;
                other = m;
            }
        }
        cache.insert(stone_value.len(), (max, other));
        (max, other)
    }
    pub fn stone_game_iii(stone_value: Vec<i32>) -> String {
        let (a, b) = Self::st_gm(&stone_value[..], &mut std::collections::HashMap::new());
        String::from(match a.cmp(&b) {
            std::cmp::Ordering::Greater => "Alice",
            std::cmp::Ordering::Equal => "Tie",
            std::cmp::Ordering::Less => "Bob",
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stone_game_iii() {
        assert_eq!(Solution::stone_game_iii(vec![1, 2, 3, 7]), "Bob");
        assert_eq!(Solution::stone_game_iii(vec![1, 2, 3, -9]), "Alice");
        assert_eq!(Solution::stone_game_iii(vec![1, 2, 3, 6]), "Tie");
    }
}
