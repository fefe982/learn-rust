// https://leetcode.com/problems/successful-pairs-of-spells-and-potions/
// 2300. Successful Pairs of Spells and Potions
pub struct Solution;
impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
        let mut potions = potions;
        potions.sort_unstable();
        let mut spells = spells.into_iter().enumerate().map(|(i, s)| (s, i)).collect::<Vec<_>>();
        spells.sort();
        let mut pos = potions.len();
        let mut res = vec![0; spells.len()];
        for (s, i) in spells {
            while pos > 0 && s as i64 * potions[pos - 1] as i64 >= success {
                pos -= 1;
            }
            res[i] = (potions.len() - pos) as i32;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn successful_pairs() {
        assert_eq!(
            Solution::successful_pairs(vec![5, 1, 3], vec![1, 2, 3, 4, 5], 7),
            vec![4, 0, 3]
        );
        assert_eq!(
            Solution::successful_pairs(vec![3, 1, 2], vec![8, 5, 8], 16),
            vec![2, 0, 2]
        );
    }
}
