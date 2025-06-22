// https://leetcode.cn/problems/zigzag-conversion/
// 6. ZigZag Conversion
pub struct Solution;
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let s = s.as_bytes();
        let mut res = "".to_string();
        let num_rows = num_rows as usize;
        let block = 2 * num_rows - 2;
        let nblock = s.len() / block;
        for i in 0..(s.len() + block - 1) / block {
            res.push(s[i * block] as char);
        }
        for i in 1..num_rows - 1 {
            for j in 0..nblock {
                res.push(s[j * block + i] as char);
                res.push(s[j * block + block - i] as char);
            }
            if nblock * block + i < s.len() {
                res.push(s[nblock * block + i] as char);
            }
            if nblock * block + block - i < s.len() {
                res.push(s[nblock * block + block - i] as char);
            }
        }
        for i in 0..(s.len() + block - num_rows) / block {
            res.push(s[i * block + num_rows - 1] as char);
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn convert() {
        assert_eq!(Solution::convert("PAYPALISHIRING".to_string(), 3), "PAHNAPLSIIGYIR");
        assert_eq!(Solution::convert("PAYPALISHIRING".to_string(), 4), "PINALSIGYAHRPI");
        assert_eq!(Solution::convert("A".to_string(), 1), "A")
    }
}
