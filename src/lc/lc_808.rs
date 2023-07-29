// https://leetcode.com/problems/soup-servings/description/
// 808. Soup Servings
pub struct Solution;
impl Solution {
    fn f(a: i32, b: i32, m: &mut std::collections::HashMap<(i32, i32), f64>) -> f64 {
        if a <= 0 && b <= 0 {
            0.5
        } else if a <= 0 {
            1.0
        } else if b <= 0 {
            0.0
        } else if let Some(&p) = m.get(&(a, b)) {
            p
        } else {
            let p = (Self::f(a - 4, b, m)
                + Self::f(a - 3, b - 1, m)
                + Self::f(a - 2, b - 2, m)
                + Self::f(a - 1, b - 3, m))
                * 0.25;
            m.insert((a, b), p);
            p
        }
    }
    pub fn soup_servings(n: i32) -> f64 {
        let n = (n + 24) / 25;
        let mut m = std::collections::HashMap::<(i32, i32), f64>::new();
        let mut p = 0.0;
        for i in 0..=n {
            p = Self::f(i, i, &mut m);
            if p > 1.0 - 1e-5 {
                break;
            }
        }
        p
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn soup_servings() {
        assert_eq!(Solution::soup_servings(50), 0.625);
        assert_eq!(Solution::soup_servings(100), 0.71875);
    }
}
