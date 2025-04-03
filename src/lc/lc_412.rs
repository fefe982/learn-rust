// https://leetcode.com/problems/fizz-buzz/
// 412. Fizz Buzz
pub struct Solution;
impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut vec = Vec::with_capacity(n as usize);
        for i in 1..=n {
            match (i % 3, i % 5) {
                (0, 0) => vec.push("FizzBuzz".to_string()),
                (0, _) => vec.push("Fizz".to_string()),
                (_, 0) => vec.push("Buzz".to_string()),
                _ => vec.push(i.to_string()),
            }
        }
        vec
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fizz_buzz() {
        assert_eq!(Solution::fizz_buzz(3), ["1", "2", "Fizz"]);
        assert_eq!(Solution::fizz_buzz(5), ["1", "2", "Fizz", "4", "Buzz"]);
        assert_eq!(
            Solution::fizz_buzz(15),
            ["1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13", "14", "FizzBuzz"]
        );
    }
}
