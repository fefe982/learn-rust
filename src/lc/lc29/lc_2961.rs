// https://leetcode.com/problems/double-modular-exponentiation/
// 2961. Double Modular Exponentiation
pub struct Solution;
impl Solution {
    pub fn get_good_indices(variables: Vec<Vec<i32>>, target: i32) -> Vec<i32> {
        let pow = |mut a, mut b, m| -> i32 {
            a %= m;
            if a == 0 || a == 1 {
                return a;
            }
            let mut r = 1;
            while b > 0 {
                if b & 1 == 1 {
                    r = (r * a) % m;
                }
                a = (a * a) % m;
                b >>= 1;
            }
            r
        };
        variables
            .into_iter()
            .enumerate()
            .filter_map(|(i, v)| {
                if pow(pow(v[0], v[1], 10), v[2], v[3]) == target {
                    Some(i as i32)
                } else {
                    None
                }
            })
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_get_good_indices() {
        assert_eq!(
            Solution::get_good_indices(vec_vec![[2, 3, 3, 10], [3, 3, 3, 1], [6, 1, 1, 4]], 2),
            [0, 2]
        );
        assert_eq!(Solution::get_good_indices(vec_vec![[39, 3, 1000, 1000]], 17), []);
    }
}
