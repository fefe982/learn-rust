// https://leetcode.com/problems/last-stone-weight-ii/
// 1049. Last Stone Weight II
pub struct Solution;
impl Solution {
    pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
        let mut v = vec![usize::MAX; 1501];
        let mut s = Vec::with_capacity(1501);
        s.push(0);
        for (i, is) in stones.into_iter().enumerate() {
            let mut ns = Vec::with_capacity(1501);
            for j in s {
                let mut t = j + is;
                if t < 1501 && v[t as usize] != i {
                    ns.push(t);
                    v[t as usize] = i;
                }
                t = (j - is).abs();
                if v[t as usize] != i {
                    ns.push(t);
                    v[t as usize] = i;
                }
            }
            s = ns;
        }
        s.into_iter().min().unwrap()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn last_stone_weight_ii() {
        assert_eq!(Solution::last_stone_weight_ii(vec![2, 7, 4, 1, 8, 1]), 1);
        assert_eq!(Solution::last_stone_weight_ii(vec![31, 26, 33, 21, 40]), 5);
    }
}
