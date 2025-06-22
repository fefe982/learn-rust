// https://leetcode.com/problems/restore-ip-addresses/
// 93. Restore IP Addresses
pub struct Solution;
impl Solution {
    fn restore(s: &[u8], ip: &mut Vec<char>, split: i32, lastn: i32, res: &mut Vec<String>) {
        if s.is_empty() {
            if split == 3 {
                res.push(ip.iter().collect());
            }
            return;
        }
        if lastn != 0 && (lastn < 25 || (lastn == 25 && s[0] <= b'5')) {
            ip.push(s[0] as char);
            Self::restore(&s[1..], ip, split, lastn * 10 + (s[0] - b'0') as i32, res);
            ip.pop();
        }
        if split == 3 {
            return;
        }
        ip.push('.');
        ip.push(s[0] as char);
        Self::restore(&s[1..], ip, split + 1, (s[0] - b'0') as i32, res);
        ip.pop();
        ip.pop();
    }
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let s = s.as_bytes();
        let mut res = vec![];
        Self::restore(&s[1..], &mut vec![s[0] as char], 0, (s[0] - b'0') as i32, &mut res);
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn restore_ip_addresses() {
        assert_sort_eq!(
            Solution::restore_ip_addresses("25525511135".to_string()),
            ["255.255.11.135", "255.255.111.35"]
        );
        assert_sort_eq!(Solution::restore_ip_addresses("0000".to_string()), ["0.0.0.0"]);
        assert_sort_eq!(
            Solution::restore_ip_addresses("101023".to_string()),
            ["1.0.10.23", "1.0.102.3", "10.1.0.23", "10.10.2.3", "101.0.2.3"]
        );
    }
}
