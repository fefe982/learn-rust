// https://leetcode.com/problems/powerful-integers/
// 970. Powerful Integers
pub struct Solution;
impl Solution {
    pub fn powerful_integers(x: i32, y: i32, bound: i32) -> Vec<i32> {
        let mut res = std::collections::BTreeSet::new();
        let mut xp = 1;
        while xp < bound {
            let mut yp = 1;
            while xp + yp <= bound {
                res.insert(xp + yp);
                yp *= y;
                if y == 1 {
                    break;
                }
            }
            xp *= x;
            if x == 1 {
                break;
            }
        }
        res.into_iter().collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn powerful_integers() {
        assert_eq!(
            Solution::powerful_integers(2, 3, 10),
            vec![2, 3, 4, 5, 7, 9, 10]
        );
        assert_eq!(
            Solution::powerful_integers(3, 5, 15),
            vec![2, 4, 6, 8, 10, 14]
        );
    }
}
