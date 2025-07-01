// https://leetcode.com/problems/ambiguous-coordinates/
// 816. Ambiguous Coordinates
pub struct Solution;
impl Solution {
    pub fn ambiguous_coordinates(s: String) -> Vec<String> {
        let s = s.chars().skip(1).take(s.len() - 2).collect::<Vec<_>>();
        let n = s.len();
        let mut comma = 0;
        let get_nums = |l: usize, r: usize| {
            let mut nums = vec![s[l..=r].iter().collect::<String>()];
            if l == r || (s[r] == '0' && s[l] != '0') {
                return nums;
            }
            if s[r] == '0' && s[l] == '0' {
                return vec![];
            }
            if s[l] == '0' {
                return vec!["0.".to_string() + &s[l + 1..=r].iter().collect::<String>()];
            } else {
                for j in l..r {
                    nums.push(s[l..=j].iter().collect::<String>() + "." + &s[j + 1..=r].iter().collect::<String>());
                }
            }
            nums
        };
        let mut res = vec![];
        while comma < n - 1 {
            let vl = get_nums(0, comma);
            if vl.is_empty() {
                comma += 1;
                continue;
            }
            let vr = get_nums(comma + 1, n - 1);
            if vr.is_empty() {
                comma += 1;
                continue;
            }
            for v1 in vl {
                for v2 in vr.iter() {
                    res.push("(".to_string() + &v1 + &", " + &v2 + &")");
                }
            }
            comma += 1;
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn ambiguous_coordinates() {
        assert_sort_eq!(Solution::ambiguous_coordinates("(100)".to_string()), vec!["(10, 0)"]);
        assert_sort_eq!(
            Solution::ambiguous_coordinates("(123)".to_string()),
            vec!["(1, 23)", "(12, 3)", "(1.2, 3)", "(1, 2.3)"]
        );
        assert_sort_eq!(
            Solution::ambiguous_coordinates("(0123)".to_string()),
            vec![
                "(0, 1.23)",
                "(0, 12.3)",
                "(0, 123)",
                "(0.1, 2.3)",
                "(0.1, 23)",
                "(0.12, 3)"
            ]
        );
        assert_sort_eq!(
            Solution::ambiguous_coordinates("(00011)".to_string()),
            vec!["(0, 0.011)", "(0.001, 1)",]
        );
    }
}
