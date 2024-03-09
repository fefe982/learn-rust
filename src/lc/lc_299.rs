// https://leetcode.com/problems/bulls-and-cows/
// 299. Bulls and Cows
pub struct Solution;
impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {
        let mut a = 0;
        let mut b = 0;
        let mut m = std::collections::HashMap::<char, Vec<i32>>::new();
        for (s, g) in secret.chars().zip(guess.chars()) {
            if s == g {
                a += 1;
            } else {
                m.entry(s).or_insert(vec![0, 0])[0] += 1;
                m.entry(g).or_insert(vec![0, 0])[1] += 1;
            }
        }
        for (_, v) in m {
            b += std::cmp::min(v[0], v[1]);
        }
        format!("{}A{}B", a, b)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_hint() {
        assert_eq!(
            Solution::get_hint("1807".to_string(), "7810".to_string()),
            "1A3B".to_string()
        );
        assert_eq!(
            Solution::get_hint("1123".to_string(), "0111".to_string()),
            "1A1B".to_string()
        );
    }
}
