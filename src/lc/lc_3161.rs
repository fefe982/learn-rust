// https://leetcode.com/problems/block-placement-queries/
// 3161. Block Placement Queries
pub struct Solution;
impl Solution {
    fn add(v: &mut Vec<(i32, i32, i32, i32)>, idx: usize, l: i32, r: i32, pos: i32) -> (i32, i32, i32) {
        let next1 = idx * 2 + 1;
        let next2 = idx * 2 + 2;
        let mid = (l + r) >> 1;
        if v[next1].2 > pos {
            if v[next1].2 - v[next1].3 < pos {
                let len = pos + v[next1].3 - v[next1].2;
                println!("add1 {idx} {l} {r} {pos} {next1} {:?}", v[next1]);
                (v[next1].2, v[next1].2 - pos, len)
            } else {
                Self::add(v, next1, l, mid, pos)
            }
        } else if v[next2].2 - v[next2].3 < pos {
            let len = pos + v[next2].3 - v[next2].2;
            println!("add2 {idx} {l} {r} {pos} {next2} {:?}", v[next2]);
            (v[next2].2, v[next2].2 - pos, len)
        } else {
            Self::add(v, next2, mid, r, pos)
        }
    }
    fn insert(v: &mut Vec<(i32, i32, i32, i32)>, idx: usize, l: i32, r: i32, pos: i32, len: i32) {
        if l + 1 == r {
            v[idx] = (pos, len, pos, len);
            return;
        }
        let mid = (l + r) >> 1;
        let next1 = idx * 2 + 1;
        let next2 = idx * 2 + 2;
        if pos < mid {
            Self::insert(v, idx * 2 + 1, l, mid, pos, len);
        } else {
            Self::insert(v, idx * 2 + 2, mid, r, pos, len);
        }
        let long_idx = if v[next1].1 > v[next2].1 { next1 } else { next2 };
        let last_idx = if v[next2].3 == 0 { next1 } else { next2 };
        v[idx] = (v[long_idx].0, v[long_idx].1, v[last_idx].2, v[last_idx].3);
    }
    fn find(v: &Vec<(i32, i32, i32, i32)>, idx: usize, l: i32, r: i32, pos: i32) -> (i32, i32) {
        if pos + 1 >= r {
            println!("{idx} {l} {r} {pos} {:?}", v[idx]);
            return (v[idx].1, v[idx].2);
        }
        let mid = (l + r) >> 1;
        let res = Self::find(v, idx * 2 + 1, l, mid, pos);
        if pos >= mid {
            let resr = Self::find(v, idx * 2 + 2, mid, r, pos);
            (res.0.max(resr.0), res.1.max(resr.1))
        } else {
            res
        }
    }
    pub fn get_results(queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut v = vec![(0, 0, 0, 0); 65536 * 2];
        let mut ans = vec![];
        for q in queries {
            if q[0] == 1 {
                if q[1] > v[0].2 {
                    let len = q[1] - v[0].2;
                    Self::insert(&mut v, 0, 0, 65536, q[1], len);
                } else {
                    let (ins, insl, len) = Self::add(&mut v, 0, 0, 65536, q[1]);
                    Self::insert(&mut v, 0, 0, 65536, q[1], len);
                    Self::insert(&mut v, 0, 0, 65536, ins, insl);
                }
            } else {
                if q[2] == 1 {
                    ans.push(true);
                } else if q[2] > q[1] {
                    ans.push(false);
                } else {
                    let (max, last) = Self::find(&v, 0, 0, 65536, q[1]);
                    println!("{max} {last} {:?}", q);
                    ans.push(max.max(q[1] - last) >= q[2]);
                }
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
    fn get_results() {
        assert_eq!(
            Solution::get_results(vec_vec![[1, 14], [1, 9], [1, 8], [1, 3], [2, 15, 8]]),
            vec![false]
        );
        assert_eq!(
            Solution::get_results(vec_vec![[1, 10], [2, 8, 6], [1, 6], [1, 1], [2, 14, 6]]),
            vec![true, false]
        );
        assert_eq!(Solution::get_results(vec_vec![[1, 3], [2, 4, 2]]), vec![true]);
        assert_eq!(Solution::get_results(vec_vec![[2, 1, 2]]), vec![false]);
        assert_eq!(
            Solution::get_results(vec_vec![[1, 2], [2, 3, 3], [2, 3, 1], [2, 2, 2]]),
            vec![false, true, true]
        );
        assert_eq!(
            Solution::get_results(vec_vec![[1, 7], [2, 7, 6], [1, 2], [2, 7, 5], [2, 7, 6]]),
            [true, true, false]
        );
    }
}
