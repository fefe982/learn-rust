// https://leetcode.com/problems/subdomain-visit-count/
// 811. Subdomain Visit Count
pub struct Solution;
impl Solution {
    pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
        let mut counts = std::collections::HashMap::new();
        for cpdomain in cpdomains {
            let mut split = cpdomain.split(' ');
            let count = split.next().unwrap().parse::<i32>().unwrap();
            let mut domain = split.next().unwrap();
            loop {
                *counts.entry(domain.to_string()).or_insert(0) += count;
                if let Some(pos) = domain.find('.') {
                    domain = &domain[pos + 1..];
                } else {
                    break;
                }
            }
        }
        counts
            .into_iter()
            .map(|(domain, count)| format!("{} {}", count, domain))
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_subdomain_visits() {
        assert_sort_eq!(
            Solution::subdomain_visits(vec_str!["9001 discuss.leetcode.com"]),
            vec!["9001 leetcode.com", "9001 discuss.leetcode.com", "9001 com"]
        );
        assert_sort_eq!(
            Solution::subdomain_visits(vec_str![
                "900 google.mail.com",
                "50 yahoo.com",
                "1 intel.mail.com",
                "5 wiki.org"
            ]),
            vec![
                "901 mail.com",
                "50 yahoo.com",
                "900 google.mail.com",
                "5 wiki.org",
                "5 org",
                "1 intel.mail.com",
                "951 com"
            ]
        );
    }
}
