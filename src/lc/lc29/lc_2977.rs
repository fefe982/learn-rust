// https://leetcode.com/problems/minimum-cost-to-convert-string-ii/
// 2977. Minimum Cost to Convert String II
pub struct Solution;
impl Solution {
    pub fn minimum_cost(
        source: String,
        target: String,
        original: Vec<String>,
        changed: Vec<String>,
        cost: Vec<i32>,
    ) -> i64 {
        let mut seg_to_id = std::collections::HashMap::new();
        let mut len_set = std::collections::HashSet::new();
        let mut dist = vec![vec![i64::MAX; original.len() * 2]; original.len() * 2];
        for i in 0..original.len() * 2 {
            dist[i][i] = 0;
        }
        let mut sz = 0;
        for ((s, t), c) in original.iter().zip(changed.iter()).zip(cost) {
            let sid = *seg_to_id.entry(&s[..]).or_insert(sz);
            if sid == sz {
                sz += 1;
            }
            let tid = *seg_to_id.entry(&t[..]).or_insert(sz);
            if tid == sz {
                sz += 1;
            }
            dist[sid][tid] = dist[sid][tid].min(c as i64);
            len_set.insert(s.len());
        }
        for k in 0..sz {
            for i in 0..sz {
                if dist[i][k] == i64::MAX {
                    continue;
                }
                for j in 0..sz {
                    if dist[k][j] == i64::MAX {
                        continue;
                    }
                    dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
                }
            }
        }
        let mut cost = vec![i64::MAX; source.len() + 1];
        cost[0] = 0;
        for i in 0..source.len() {
            if cost[i] == i64::MAX {
                continue;
            }
            if source.as_bytes()[i] == target.as_bytes()[i] {
                cost[i + 1] = cost[i + 1].min(cost[i]);
            }
            for &len in len_set.iter() {
                if i + len > source.len() {
                    continue;
                }
                if let Some(&sid) = seg_to_id.get(&source[i..i + len]) {
                    if let Some(&tid) = seg_to_id.get(&target[i..i + len]) {
                        if dist[sid][tid] != i64::MAX {
                            cost[i + len] = cost[i + len].min(cost[i] + dist[sid][tid]);
                        }
                    }
                }
            }
        }
        if cost[source.len()] == i64::MAX {
            -1
        } else {
            cost[source.len()]
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_minimum_cost() {
        assert_eq!(
            Solution::minimum_cost(
                "fjybg".to_string(),
                "apyyt".to_string(),
                vec_str![
                    "bg", "xr", "cc", "ip", "vq", "po", "ym", "rh", "vw", "lf", "lo", "ee", "qv", "yr", "f", "w", "i",
                    "u", "g", "a", "e", "f", "s", "r", "p", "j", "o", "g", "i", "u"
                ],
                vec_str![
                    "xr", "cc", "ip", "vq", "po", "ym", "rh", "vw", "lf", "lo", "ee", "qv", "yr", "yt", "w", "i", "u",
                    "g", "a", "e", "f", "s", "r", "p", "a", "o", "g", "i", "u", "p"
                ],
                vec![
                    97733, 90086, 87125, 85361, 75644, 46301, 21616, 79538, 52507, 95884, 79353, 61127, 58665, 96031,
                    95035, 12116, 41158, 91096, 47819, 88522, 25493, 80186, 66981, 87597, 56691, 86820, 89031, 99954,
                    41271, 39699
                ]
            ),
            1628332
        );
        assert_eq!(
            Solution::minimum_cost(
                "abcd".to_string(),
                "acbe".to_string(),
                vec_str!["a", "b", "c", "c", "e", "d"],
                vec_str!["b", "c", "b", "e", "b", "e"],
                vec![2, 5, 5, 1, 2, 20]
            ),
            28
        );
        assert_eq!(
            Solution::minimum_cost(
                "abcdefgh".to_string(),
                "acdeeghh".to_string(),
                vec_str!["bcd", "fgh", "thh"],
                vec_str!["cde", "thh", "ghh"],
                vec![1, 3, 5]
            ),
            9
        );
        assert_eq!(
            Solution::minimum_cost(
                "abcdefgh".to_string(),
                "addddddd".to_string(),
                vec_str!["bcd", "defgh"],
                vec_str!["ddd", "ddddd"],
                vec![100, 1578]
            ),
            -1
        );
    }
}
