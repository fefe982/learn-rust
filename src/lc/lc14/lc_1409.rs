// https://leetcode.com/problems/queries-on-a-permutation-with-key/
// 1409. Queries on a Permutation With Key
pub struct Solution;
impl Solution {
    fn inc(bit: &mut Vec<i32>, mut i: usize, v: i32) {
        while i < bit.len() {
            bit[i] += v;
            i += i & (!i + 1);
        }
    }
    fn get(bit: &Vec<i32>, mut i: usize) -> i32 {
        let mut s = 0;
        while i > 0 {
            s += bit[i];
            i -= i & (!i + 1);
        }
        s
    }
    pub fn process_queries(queries: Vec<i32>, m: i32) -> Vec<i32> {
        let m = m as usize;
        let mut bit = vec![0; m + queries.len() + 1];
        let mut ins = queries.len();
        let mut map = std::collections::HashMap::<i32, usize>::new();
        for i in 0..m {
            Self::inc(&mut bit, queries.len() + i + 1, 1);
            map.insert(i as i32 + 1, queries.len() + i + 1);
        }
        let mut ans = Vec::with_capacity(queries.len());
        for q in queries {
            let i = map.insert(q, ins).unwrap();
            ans.push(Self::get(&bit, i) - 1);
            Self::inc(&mut bit, i, -1);
            Self::inc(&mut bit, ins, 1);
            ins -= 1;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn process_queries() {
        assert_eq!(Solution::process_queries(vec![3, 1, 2, 1], 5), vec![2, 1, 2, 1]);
        assert_eq!(Solution::process_queries(vec![4, 1, 2, 2], 4), vec![3, 1, 2, 0]);
        assert_eq!(Solution::process_queries(vec![7, 5, 5, 8, 3], 8), vec![6, 5, 0, 7, 5]);
    }
}
