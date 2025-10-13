// https://leetcode.com/problems/simplified-fractions/
// 1447. Simplified Fractions
pub struct Solution;
impl Solution {
    pub fn simplified_fractions(n: i32) -> Vec<String> {
        let mut res = vec![];
        for den in 2..=n {
            for num in 1..den {
                let mut a = num;
                let mut b = den;
                loop {
                    if b % a == 0 {
                        break;
                    }
                    let t = b % a;
                    b = a;
                    a = t;
                }
                if a == 1 {
                    res.push(format!("{}/{}", num, den));
                }
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn simplified_fractions() {
        assert_sort_eq!(Solution::simplified_fractions(2), vec_str!["1/2"]);
        assert_sort_eq!(Solution::simplified_fractions(3), vec_str!["1/2", "1/3", "2/3"]);
        assert_sort_eq!(
            Solution::simplified_fractions(4),
            vec_str!["1/2", "1/3", "1/4", "2/3", "3/4"]
        );
    }
}
