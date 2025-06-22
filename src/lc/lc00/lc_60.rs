// https://leetcode.com/problems/permutation-sequence/
// 60. Permutation Sequence
pub struct Solution;
impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let mut k = k;
        let mut digits: Vec<u8> = (b'1'..=(b'0' + n as u8)).collect();
        let mut result = Vec::with_capacity(n as usize);
        let mut modulo = 1;
        for i in 2..n {
            modulo *= i;
        }
        k -= 1;
        for i in (1..n).rev() {
            let idx = (k / modulo) as usize;
            result.push(digits.remove(idx));
            k %= modulo;
            modulo /= i;
        }
        result.push(digits[0]);
        String::from_utf8(result).unwrap()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_permutation() {
        assert_eq!(Solution::get_permutation(3, 3), String::from("213"));
        assert_eq!(Solution::get_permutation(4, 9), String::from("2314"));
        assert_eq!(Solution::get_permutation(3, 1), String::from("123"));
    }
}
