// https://leetcode.com/problems/add-binary/
// 67. Add Binary
pub struct Solution;
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut r = Vec::with_capacity(a.len().max(b.len()) + 1);
        let mut carry = 0;
        let a = a.as_bytes();
        let b = b.as_bytes();
        let lmin = a.len().min(b.len());
        for i in 0..lmin {
            let ac = a[a.len() - 1 - i];
            let bc = b[b.len() - 1 - i];
            let sum = carry + (ac - b'0') as i32 + (bc - b'0') as i32;
            r.push(if sum % 2 == 0 { '0' } else { '1' });
            carry = sum / 2;
        }
        for i in lmin..a.len() {
            let ac = a[a.len() - 1 - i];
            let sum = carry + (ac - b'0') as i32;
            r.push(if sum % 2 == 0 { '0' } else { '1' });
            carry = sum / 2;
        }
        for i in lmin..b.len() {
            let bc = b[b.len() - 1 - i];
            let sum = carry + (bc - b'0') as i32;
            r.push(if sum % 2 == 0 { '0' } else { '1' });
            carry = sum / 2;
        }
        if carry > 0 {
            r.push('1');
        }
        r.into_iter().rev().collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_binary() {
        assert_eq!(
            Solution::add_binary("11".to_string(), "1".to_string()),
            "100".to_string()
        );
        assert_eq!(
            Solution::add_binary("1010".to_string(), "1011".to_string()),
            "10101".to_string()
        );
    }
}
