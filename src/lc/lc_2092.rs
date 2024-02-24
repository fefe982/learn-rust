// https://leetcode.com/problems/find-all-people-with-secret/
// 2092. Find All People With Secret
pub struct Solution;
impl Solution {
    fn p(v: &mut Vec<usize>, i: usize) -> usize {
        let mut p = i;
        while v[p] != p {
            p = v[p];
        }
        v[i] = p;
        p
    }
    fn union(v: &mut Vec<usize>, i: usize, j: usize) {
        let pi = Self::p(v, i);
        let pj = Self::p(v, j);
        if pi == pj {
            return;
        }
        v[pi.max(pj)] = pj.min(pi);
    }
    pub fn find_all_people(n: i32, meetings: Vec<Vec<i32>>, first_person: i32) -> Vec<i32> {
        let mut v = (0..n as usize).collect::<Vec<_>>();
        v[first_person as usize] = 0;
        let mut meetings = meetings;
        meetings.sort_unstable_by_key(|x| x[2]);
        let mut set = std::collections::HashSet::new();
        let mut i = 0;
        let mut j;
        while i < meetings.len() {
            j = i + 1;
            while j < meetings.len() && meetings[j][2] == meetings[i][2] {
                j += 1;
            }
            for k in i..j {
                Self::union(&mut v, meetings[k][0] as usize, meetings[k][1] as usize);
                set.insert(meetings[k][0] as usize);
                set.insert(meetings[k][1] as usize);
            }
            for &p in &set {
                if Self::p(&mut v, p) != 0 {
                    v[p] = p;
                }
            }
            set.clear();
            i = j;
        }
        let mut res = vec![];
        for i in 0..n as usize {
            if Self::p(&mut v, i) == 0 {
                res.push(i as i32);
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
    fn test_find_all_people() {
        assert_eq!(
            Solution::find_all_people(6, vec_vec![[1, 2, 5], [2, 3, 8], [1, 5, 10]], 1),
            vec![0, 1, 2, 3, 5]
        );
        assert_eq!(
            Solution::find_all_people(4, vec_vec![[3, 1, 3], [1, 2, 2], [0, 3, 3]], 3),
            vec![0, 1, 3]
        );
        assert_eq!(
            Solution::find_all_people(5, vec_vec![[3, 4, 2], [1, 2, 1], [2, 3, 1]], 1),
            vec![0, 1, 2, 3, 4]
        );
    }
}
