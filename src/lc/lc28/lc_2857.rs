// https://leetcode.com/problems/count-pairs-of-points-with-distance-k/
// 2857. Count Pairs of Points With Distance K
pub struct Solution;
impl Solution {
    pub fn count_pairs(coordinates: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut cnt = std::collections::HashMap::new();
        let mut ans = 0;
        for v in coordinates {
            let x = v[0];
            let y = v[1];
            for i in 0..=k {
                ans += cnt.get(&(x ^ i, y ^ (k - i))).unwrap_or(&0);
            }
            cnt.entry((x, y)).and_modify(|j| *j += 1).or_insert(1);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn count_pairs() {
        assert_eq!(Solution::count_pairs(vec_vec![[1, 2], [4, 2], [1, 3], [5, 2]], 5), 2);
        assert_eq!(
            Solution::count_pairs(vec_vec![[1, 3], [1, 3], [1, 3], [1, 3], [1, 3]], 0),
            10
        );
    }
}
