// https://leetcode.com/problems/people-whose-list-of-favorite-companies-is-not-a-subset-of-another-list/
// 1452. People Whose List of Favorite Companies Is Not a Subset of Another List
pub struct Solution;
impl Solution {
    pub fn people_indexes(favorite_companies: Vec<Vec<String>>) -> Vec<i32> {
        let mut cmap = std::collections::HashMap::<String, i32>::new();
        let mut v = (0..favorite_companies.len()).collect::<Vec<_>>();
        let mut cset = vec![std::collections::HashSet::<i32>::new(); favorite_companies.len()];
        for (i, fvec) in favorite_companies.into_iter().enumerate() {
            {
                let set = &mut cset[i];
                for f in fvec {
                    let l = cmap.len() as i32;
                    let val = *cmap.entry(f).or_insert(l);
                    set.insert(val);
                }
            }
            for j in 0..i {
                if v[j] != j {
                    continue;
                }
                if cset[j].is_superset(&cset[i]) {
                    v[i] = j;
                    break;
                }
                if cset[i].is_superset(&cset[j]) {
                    v[j] = i;
                }
            }
        }
        v.into_iter()
            .enumerate()
            .filter_map(|(i, x)| if i == x { Some(i as i32) } else { None })
            .collect::<Vec<_>>()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn people_indexes() {
        assert_eq!(
            Solution::people_indexes(vec_vec_str![
                ["leetcode", "google", "facebook"],
                ["google", "microsoft"],
                ["google", "facebook"],
                ["google"],
                ["amazon"]
            ]),
            [0, 1, 4]
        );
        assert_eq!(
            Solution::people_indexes(vec_vec_str![
                ["leetcode", "google", "facebook"],
                ["leetcode", "amazon"],
                ["facebook", "google"]
            ]),
            [0, 1]
        );
        assert_eq!(
            Solution::people_indexes(vec_vec_str![["leetcode"], ["google"], ["facebook"], ["amazon"]]),
            [0, 1, 2, 3]
        );
    }
}
