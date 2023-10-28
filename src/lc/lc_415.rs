// https://leetcode.com/problems/add-strings/
// 415. Add Strings
pub struct Solution;
impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let mut res = vec![];
        let mut c = 0;
        for ((i1, n1), (i2, n2)) in num1
            .as_bytes()
            .iter()
            .cloned()
            .rev()
            .enumerate()
            .chain(std::iter::repeat((usize::MAX, b'0')))
            .zip(
                num2.as_bytes()
                    .iter()
                    .cloned()
                    .rev()
                    .enumerate()
                    .chain(std::iter::repeat((usize::MAX, b'0'))),
            )
        {
            if i1 == usize::MAX && i2 == usize::MAX && n1 == b'0' && n2 == b'0' && c == 0 {
                break;
            }
            let d = (n1 - b'0') + (n2 - b'0') + c;
            res.push((d % 10 + b'0') as char);
            c = d / 10;
        }
        res.into_iter().rev().collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_strings() {
        assert_eq!(
            Solution::add_strings(String::from("11"), String::from("123")),
            String::from("134")
        );
        assert_eq!(
            Solution::add_strings(String::from("456"), String::from("77")),
            String::from("533")
        );
        assert_eq!(
            Solution::add_strings(String::from("0"), String::from("0")),
            String::from("0")
        );
    }
}
