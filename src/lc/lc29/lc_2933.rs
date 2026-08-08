// https://leetcode.com/problems/high-access-employees/
// 2933. High Access Employees
pub struct Solution;
impl Solution {
    pub fn find_high_access_employees(access_times: Vec<Vec<String>>) -> Vec<String> {
        let mut m = std::collections::HashMap::<String, Vec<i32>>::new();
        for at in access_times {
            let e = m.entry(at[0].clone()).or_insert(vec![]);
            let ts = at[1].as_bytes();
            e.push(ts[0] as i32 * 10 * 60 + ts[1] as i32 * 60 + ts[2] as i32 * 10 + ts[3] as i32);
        }
        let mut res = vec![];
        for (k, mut v) in m {
            if v.len() < 3 {
                continue;
            }
            v.sort();
            for i in 2..v.len() {
                if v[i] - v[i - 2] < 60 {
                    res.push(k.clone());
                    break;
                }
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn find_high_access_employees() {
        assert_sort_eq!(
            Solution::find_high_access_employees(vec_vec_str![
                ["a", "0549"],
                ["b", "0457"],
                ["a", "0532"],
                ["a", "0621"],
                ["b", "0540"]
            ]),
            vec_str!["a"]
        );
        assert_sort_eq!(
            Solution::find_high_access_employees(vec_vec_str![
                ["d", "0002"],
                ["c", "0808"],
                ["c", "0829"],
                ["e", "0215"],
                ["d", "1508"],
                ["d", "1444"],
                ["d", "1410"],
                ["c", "0809"]
            ]),
            vec_str!["c", "d"]
        );
        assert_sort_eq!(
            Solution::find_high_access_employees(vec_vec_str![
                ["cd", "1025"],
                ["ab", "1025"],
                ["cd", "1046"],
                ["cd", "1055"],
                ["ab", "1124"],
                ["ab", "1120"]
            ]),
            vec_str!["ab", "cd"]
        );
    }
}
