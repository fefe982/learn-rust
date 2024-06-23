// https://leetcode.com/problems/detect-capital/
// 520. Detect Capital
pub struct Solution;
impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        enum State {
            Start,
            FirstCap,
            Cap,
            Lower,
        }
        let mut state = State::Start;
        for c in word.chars() {
            let up = c.is_uppercase();
            match state {
                State::Start => {
                    if up {
                        state = State::FirstCap
                    } else {
                        state = State::Lower;
                    }
                }
                State::FirstCap => {
                    if up {
                        state = State::Cap
                    } else {
                        state = State::Lower;
                    }
                }
                State::Cap => {
                    if !up {
                        return false;
                    }
                }
                State::Lower => {
                    if up {
                        return false;
                    }
                }
            }
        }
        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_detect_capital_use() {
        assert_eq!(Solution::detect_capital_use("USA".to_string()), true);
        assert_eq!(Solution::detect_capital_use("FlaG".to_string()), false);
    }
}
