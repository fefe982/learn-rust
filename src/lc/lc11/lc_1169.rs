// https://leetcode.com/problems/invalid-transactions/
// 1169. Invalid Transactions
pub struct Solution;
impl Solution {
    pub fn invalid_transactions(transactions: Vec<String>) -> Vec<String> {
        let mut m = std::collections::HashMap::<String, std::collections::BTreeMap<i32, Vec<(String, usize)>>>::new();
        let mut invalid = vec![0; transactions.len()];
        for (i, transaction) in transactions.iter().enumerate() {
            let mut s = transaction.split(',');
            let name = s.next().unwrap().to_string();
            let time = s.next().unwrap().parse::<i32>().unwrap();
            let amount = s.next().unwrap().parse::<i32>().unwrap();
            let city = s.next().unwrap().to_string();
            if amount > 1000 {
                invalid[i] = 1;
            }
            if let Some(pm) = m.get(&name) {
                for (_, t) in pm.range((time - 60)..=(time + 60)) {
                    for (c, j) in t {
                        if *c != city {
                            invalid[i] = 1;
                            invalid[*j] = 1;
                        }
                    }
                }
            }
            m.entry(name).or_default().entry(time).or_default().push((city, i));
        }
        invalid
            .into_iter()
            .enumerate()
            .filter_map(|(i, v)| if v == 1 { Some(transactions[i].clone()) } else { None })
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn invalid_transactions() {
        assert_sort_eq!(
            Solution::invalid_transactions(vec_str![
                "bob,627,1973,amsterdam",
                "alex,387,885,bangkok",
                "alex,355,1029,barcelona",
                "alex,587,402,bangkok",
                "chalicefy,973,830,barcelona",
                "alex,932,86,bangkok",
                "bob,188,989,amsterdam"
            ]),
            vec_str![
                "bob,627,1973,amsterdam",
                "alex,387,885,bangkok",
                "alex,355,1029,barcelona"
            ]
        );
        assert_sort_eq!(
            Solution::invalid_transactions(vec_str!["alice,20,800,mtv", "alice,50,100,beijing"]),
            vec_str!["alice,20,800,mtv", "alice,50,100,beijing"]
        );
        assert_sort_eq!(
            Solution::invalid_transactions(vec_str!["alice,20,800,mtv", "alice,50,1200,mtv"]),
            vec_str!["alice,50,1200,mtv"]
        );
        assert_sort_eq!(
            Solution::invalid_transactions(vec_str!["alice,20,800,mtv", "bob,50,1200,mtv"]),
            vec_str!["bob,50,1200,mtv"]
        );
    }
}
