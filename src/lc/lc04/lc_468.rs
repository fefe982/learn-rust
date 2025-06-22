// https://leetcode.com/problems/validate-ip-address/
// 468. Validate IP Address
pub struct Solution;
impl Solution {
    pub fn valid_ip_address(query_ip: String) -> String {
        let ipv4 = regex::Regex::new(
            r"^(?:(?:25[0-5]|2[0-4][0-9]|[0-9]|[1-9][0-9]|1[0-9][0-9])\.){3}(?:25[0-5]|2[0-4][0-9]|[0-9]|[1-9][0-9]|1[0-9][0-9])$",
        )
        .unwrap();
        let ipv6 = regex::Regex::new(r"^(?:(?:[0-9a-fA-F]{1,4}:){7}[0-9a-fA-F]{1,4})$").unwrap();
        if ipv4.is_match(&query_ip) {
            "IPv4".to_string()
        } else if ipv6.is_match(&query_ip) {
            "IPv6".to_string()
        } else {
            "Neither".to_string()
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_valid_ip_address() {
        assert_eq!(Solution::valid_ip_address("172.16.254.1".to_string()), "IPv4");
        assert_eq!(
            Solution::valid_ip_address("2001:0db8:85a3:0:0:8A2E:0370:7334".to_string()),
            "IPv6"
        );
        assert_eq!(Solution::valid_ip_address("256.256.256.256".to_string()), "Neither");
    }
}
