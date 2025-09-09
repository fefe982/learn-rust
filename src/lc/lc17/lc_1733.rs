// https://leetcode.com/problems/minimum-number-of-people-to-teach/
// 1733. Minimum Number of People to Teach
pub struct Solution;
impl Solution {
    pub fn minimum_teachings(n: i32, languages: Vec<Vec<i32>>, friendships: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashSet;
        let languages = languages
            .into_iter()
            .map(|v| v.into_iter().collect::<HashSet<_>>())
            .collect::<Vec<_>>();
        let mut nf = HashSet::new();
        for f in friendships {
            if languages[f[0] as usize - 1]
                .intersection(&languages[f[1] as usize - 1])
                .count()
                == 0
            {
                nf.insert(f[0] as usize - 1);
                nf.insert(f[1] as usize - 1);
            }
        }
        let mut cl = vec![0; n as usize];
        let mut mc = 0;
        for &f in &nf {
            for &i in &languages[f] {
                cl[i as usize - 1] += 1;
                mc = mc.max(cl[i as usize - 1]);
            }
        }
        nf.len() as i32 - mc
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn minimum_teachings() {
        assert_eq!(
            Solution::minimum_teachings(2, vec_vec![[1], [2], [1, 2]], vec_vec![[1, 2], [1, 3], [2, 3]]),
            1
        );
        assert_eq!(
            Solution::minimum_teachings(
                3,
                vec_vec![[2], [1, 3], [1, 2], [3]],
                vec_vec![[1, 4], [1, 2], [3, 4], [2, 3]]
            ),
            2
        );
    }
}
