// https://leetcode.com/problems/decode-xored-permutation/
// 1734. Decode XORed Permutation
pub struct Solution;
impl Solution {
    pub fn decode(encoded: Vec<i32>) -> Vec<i32> {
        let n = encoded.len() + 1;
        let mut xor = 0;
        for i in 1..=n as i32 {
            xor ^= i;
        }
        let mut x2 = 0;
        for i in 0..n / 2 {
            x2 ^= encoded[i * 2 + 1];
        }
        let mut ans = Vec::with_capacity(n);
        let mut last = x2 ^ xor;
        ans.push(last);
        for i in 0..encoded.len() {
            last = last ^ encoded[i];
            ans.push(last);
        }
        ans
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn decode() {
        assert_eq!(Solution::decode(vec![3, 1]), [1, 2, 3]);
        assert_eq!(Solution::decode(vec![6, 5, 4, 6]), [2, 4, 1, 5, 3]);
    }
}
