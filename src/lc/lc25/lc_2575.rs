// https://leetcode.com/problems/find-the-divisibility-array-of-a-string/
// 2575. Find the Divisibility Array of a String
pub struct Solution;
impl Solution {
    pub fn divisibility_array(word: String, m: i32) -> Vec<i32> {
        word.as_bytes()
            .iter()
            .scan(0i64, |state, &c| {
                *state = (*state * 10 + (c - b'0') as i64) % m as i64;
                if *state == 0 {
                    Some(1)
                } else {
                    Some(0)
                }
            })
            .collect::<Vec<_>>()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_divisibility_array() {
        assert_eq!(
            Solution::divisibility_array(String::from("998244353"), 3),
            vec![1, 1, 0, 0, 0, 1, 1, 0, 0]
        );
        assert_eq!(Solution::divisibility_array(String::from("1010"), 10), vec![0, 1, 0, 1]);
    }
}
