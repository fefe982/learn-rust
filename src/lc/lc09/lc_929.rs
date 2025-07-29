// https://leetcode.com/problems/unique-email-addresses/
// 929. Unique Email Addresses
pub struct Solution;
impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        for email in emails {
            let mut local = true;
            let mut ignore = false;
            let filtered = email
                .chars()
                .filter(|&c| {
                    if local {
                        if c == '@' {
                            local = false;
                            return true;
                        }
                        if ignore {
                            return false;
                        }
                        if c == '.' {
                            return false;
                        }
                        if c == '+' {
                            ignore = true;
                            return false;
                        }
                        return true;
                    }
                    true
                })
                .collect::<String>();
            set.insert(filtered);
        }
        set.len() as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn num_unique_emails() {
        assert_eq!(
            Solution::num_unique_emails(vec_str![
                "test.email+alex@leetcode.com",
                "test.e.mail+bob.cathy@leetcode.com",
                "testemail+david@lee.tcode.com"
            ]),
            2
        );
        assert_eq!(
            Solution::num_unique_emails(vec_str!["a@leetcode.com", "b@leetcode.com", "c@leetcode.com"]),
            3
        );
    }
}
