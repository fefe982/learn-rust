// https://leetcode.com/problems/successful-pairs-of-spells-and-potions/
// 2300. Successful Pairs of Spells and Potions
pub struct Solution;
impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
        let mut res = Vec::with_capacity(spells.len());
        potions.sort();
        let l = potions.len();
        for s in spells {
            let s = s as i64;
            if s * (potions[0] as i64) >= success {
                res.push(l as i32);
                continue;
            }
            if s * (potions[l - 1] as i64) < success {
                res.push(0);
                continue;
            }
            let mut low = 0;
            let mut high = l - 1;
            while low < high {
                if low + 1 == high {
                    res.push((l - high) as i32);
                    break;
                }
                let mid = (low + high) / 2;
                if s * (potions[mid] as i64) >= success {
                    high = mid;
                } else {
                    low = mid;
                }
            }
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
