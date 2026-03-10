// https://leetcode.com/problems/decode-xored-array/
// 1720. Decode XORed Array
pub struct Solution;
impl Solution {
    pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
        let mut ans = vec![first];
        let mut first = first;
        for i in encoded {
            first ^= i;
            ans.push(first);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn decode() {
        assert_eq!(Solution::decode(vec![1, 2, 3], 1), vec![1, 0, 2, 1]);
        assert_eq!(Solution::decode(vec![6, 2, 7, 3], 4), vec![4, 2, 0, 7, 4]);
    }
}
