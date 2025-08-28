// https://leetcode.com/problems/defanging-an-ip-address/
// 1108. Defanging an IP Address
pub struct Solution;
impl Solution {
    pub fn defang_i_paddr(address: String) -> String {
        address.replace(".", "[.]")
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn defang_i_paddr() {
        assert_eq!(
            Solution::defang_i_paddr("1.1.1.1".to_string()),
            "1[.]1[.]1[.]1".to_string()
        );
        assert_eq!(
            Solution::defang_i_paddr("255.100.50.0".to_string()),
            "255[.]100[.]50[.]0".to_string()
        );
    }
}
