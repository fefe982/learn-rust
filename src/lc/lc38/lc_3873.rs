// https://leetcode.com/problems/maximum-points-activated-with-one-addition/
// 3873. Maximum Points Activated With One Addition
pub struct Solution;
impl Solution {
    fn find(map: &mut std::collections::HashMap<i32, i32>, p: i32) -> i32 {
        if let Some(&v) = map.get(&p) {
            if p == v {
                return v;
            } else {
                let r = Self::find(map, v);
                map.insert(p, r);
                return r;
            }
        } else {
            map.insert(p, p);
            return p;
        }
    }
    pub fn max_activated(points: Vec<Vec<i32>>) -> i32 {
        let mut map = std::collections::HashMap::new();
        for p in &points {
            let (x, y) = (p[0], p[1]);
            let px = Self::find(&mut map, x - 1_000_000_001);
            let py = Self::find(&mut map, y + 1_000_000_000);
            if px != py {
                map.insert(px, py);
            }
        }
        let mut cnt = std::collections::HashMap::new();
        for p in points {
            let x = p[0];
            let px = Self::find(&mut map, x - 1_000_000_001);
            *cnt.entry(px).or_insert(0) += 1;
        }
        let mut mx1 = 0;
        let mut mx2 = 0;
        for &k in cnt.values() {
            if k > mx1 {
                mx2 = mx1;
                mx1 = k;
            } else if k > mx2 {
                mx2 = k;
            }
        }
        mx1 + mx2 + 1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn max_activated() {
        assert_eq!(Solution::max_activated(vec_vec![[1, 1], [1, 2], [2, 2]]), 4);
        assert_eq!(Solution::max_activated(vec_vec![[2, 2], [1, 1], [3, 3]]), 3);
        assert_eq!(Solution::max_activated(vec_vec![[2, 3], [2, 2], [1, 1], [4, 5]]), 4);
    }
}
