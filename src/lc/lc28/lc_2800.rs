// https://leetcode.com/problems/shortest-string-that-contains-three-strings/
// 2800. Shortest String That Contains Three Strings
pub struct Solution;

impl Solution {
    pub fn minimum_string(a: String, b: String, c: String) -> String {
        fn merge(first: &str, second: &str) -> String {
            if first.contains(second) {
                return first.to_string();
            }
            if second.contains(first) {
                return second.to_string();
            }

            let max_overlap = (1..=first.len().min(second.len()))
                .rev()
                .find(|&k| first.ends_with(&second[..k]));

            if let Some(k) = max_overlap {
                let mut merged = first.to_string();
                merged.push_str(&second[k..]);
                merged
            } else {
                let mut merged = first.to_string();
                merged.push_str(second);
                merged
            }
        }

        let strs = [&a, &b, &c];
        let perms = [[0, 1, 2], [0, 2, 1], [1, 0, 2], [1, 2, 0], [2, 0, 1], [2, 1, 0]];

        let mut best: Option<String> = None;
        for perm in perms {
            let merged = merge(&strs[perm[0]], &strs[perm[1]]);
            let merged = merge(&merged, &strs[perm[2]]);
            best = Some(match best {
                None => merged,
                Some(prev) => {
                    if merged.len() < prev.len() || (merged.len() == prev.len() && merged < prev) {
                        merged
                    } else {
                        prev
                    }
                }
            });
        }

        best.unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimum_string() {
        assert_eq!(
            Solution::minimum_string("abc".to_string(), "bca".to_string(), "aaa".to_string()),
            "aaabca".to_string()
        );
        assert_eq!(
            Solution::minimum_string("ab".to_string(), "ba".to_string(), "aba".to_string()),
            "aba".to_string()
        );
    }
}
