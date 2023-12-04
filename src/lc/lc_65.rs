// https://leetcode.com/problems/valid-number/
// 65. Valid Number
pub struct Solution;
enum State {
    P00,
    P11,
    Px2,
    P23,
    Px4,
    P5,
    P6,
    P7,
}
enum CType {
    Sign,
    Dot,
    Digit,
    Exp,
}
impl Solution {
    pub fn is_number(s: String) -> bool {
        let mut state: State = State::P00;
        for c in s.as_bytes() {
            let stype = match c {
                b'0'..=b'9' => CType::Digit,
                b'.' => CType::Dot,
                b'+' | b'-' => CType::Sign,
                b'e' | b'E' => CType::Exp,
                _ => return false,
            };
            match state {
                State::P00 => match stype {
                    CType::Sign => state = State::P11,
                    CType::Digit => state = State::P23,
                    CType::Dot => state = State::Px2,
                    _ => return false,
                },
                State::P11 => match stype {
                    CType::Digit => state = State::P23,
                    CType::Dot => state = State::Px2,
                    _ => return false,
                },
                State::P23 => match stype {
                    CType::Digit => (),
                    CType::Dot => state = State::Px4,
                    CType::Exp => state = State::P5,
                    _ => return false,
                },
                State::Px2 => match stype {
                    CType::Digit => state = State::Px4,
                    _ => return false,
                },
                State::Px4 => match stype {
                    CType::Digit => (),
                    CType::Exp => state = State::P5,
                    _ => return false,
                },
                State::P5 => match stype {
                    CType::Digit => state = State::P7,
                    CType::Sign => state = State::P6,
                    _ => return false,
                },
                State::P6 => match stype {
                    CType::Digit => state = State::P7,
                    _ => return false,
                },
                State::P7 => match stype {
                    CType::Digit => (),
                    _ => return false,
                },
            }
        }
        match state {
            State::P23 | State::Px4 | State::P7 => true,
            _ => false,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_number() {
        assert_eq!(Solution::is_number(String::from("0")), true);
        assert_eq!(Solution::is_number(String::from("e")), false);
        assert_eq!(Solution::is_number(String::from(".")), false);
    }
}
