// https://leetcode.com/problems/accounts-merge/
// 721. Accounts Merge
pub struct Solution;
#[derive(Debug)]
struct Name {
    emails: std::collections::HashMap<String, usize>,
    p: Vec<usize>,
}
impl Name {
    fn new() -> Self {
        Self {
            emails: std::collections::HashMap::new(),
            p: vec![],
        }
    }
    fn get_root(&mut self, idx: usize) -> usize {
        let mut i = idx;
        while i != self.p[i] {
            i = self.p[i];
        }
        self.p[idx] = i;
        i
    }
    fn get_rooti(&self, mut idx: usize) -> usize {
        while idx != self.p[idx] {
            idx = self.p[idx];
        }
        idx
    }
    fn merge(&mut self, i1: usize, i2: usize) -> usize {
        let p1 = self.get_root(i1);
        let p2 = self.get_root(i2);
        if p1 != p2 {
            self.p[p2] = p1;
        }
        p1
    }
    fn insert(&mut self, s: String, p: usize) -> usize {
        let idx = if let Some(&idx) = self.emails.get(&s) {
            idx
        } else {
            let idx = self.p.len();
            self.emails.insert(s, idx);
            self.p.push(idx);
            idx
        };
        if p != usize::MAX {
            self.merge(p, idx)
        } else {
            self.get_root(idx)
        }
    }
    fn get_emails(&self) -> Vec<Vec<String>> {
        let mut m = std::collections::HashMap::new();
        let mut ans: Vec<Vec<String>> = vec![];
        for (email, &idx) in &self.emails {
            let p = self.get_rooti(idx);
            if let Some(&idx) = m.get(&p) {
                let v: &mut Vec<String> = &mut ans[idx];
                v.push(email.clone());
            } else {
                m.insert(p, ans.len());
                ans.push(vec![email.clone()]);
            }
        }
        for email in ans.iter_mut() {
            email.sort();
        }
        ans
    }
}
impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut acc = std::collections::HashMap::<String, Name>::new();
        for account in accounts {
            let mut p = usize::MAX;
            let name = account[0].clone();
            for email in account.into_iter().skip(1) {
                p = acc.entry(name.clone()).or_insert_with(Name::new).insert(email, p);
            }
        }
        let mut ans = vec![];
        for (name, name_acc) in acc {
            for emails in name_acc.get_emails() {
                ans.push(vec![name.clone()]);
                ans.last_mut().unwrap().extend(emails.into_iter())
            }
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_accounts_merge() {
        assert_sort_eq!(
            Solution::accounts_merge(vec_vec_str![
                ["John", "johnsmith@mail.com", "john_newyork@mail.com"],
                ["John", "johnsmith@mail.com", "john00@mail.com"],
                ["Mary", "mary@mail.com"],
                ["John", "johnnybravo@mail.com"]
            ]),
            vec_vec_str![
                ["John", "john00@mail.com", "john_newyork@mail.com", "johnsmith@mail.com"],
                ["Mary", "mary@mail.com"],
                ["John", "johnnybravo@mail.com"]
            ]
        );
        assert_sort_eq!(
            Solution::accounts_merge(vec_vec_str![
                ["Gabe", "Gabe0@m.co", "Gabe3@m.co", "Gabe1@m.co"],
                ["Kevin", "Kevin3@m.co", "Kevin5@m.co", "Kevin0@m.co"],
                ["Ethan", "Ethan5@m.co", "Ethan4@m.co", "Ethan0@m.co"],
                ["Hanzo", "Hanzo3@m.co", "Hanzo1@m.co", "Hanzo0@m.co"],
                ["Fern", "Fern5@m.co", "Fern1@m.co", "Fern0@m.co"]
            ]),
            vec_vec_str![
                ["Ethan", "Ethan0@m.co", "Ethan4@m.co", "Ethan5@m.co"],
                ["Gabe", "Gabe0@m.co", "Gabe1@m.co", "Gabe3@m.co"],
                ["Hanzo", "Hanzo0@m.co", "Hanzo1@m.co", "Hanzo3@m.co"],
                ["Kevin", "Kevin0@m.co", "Kevin3@m.co", "Kevin5@m.co"],
                ["Fern", "Fern0@m.co", "Fern1@m.co", "Fern5@m.co"]
            ]
        )
    }
}
