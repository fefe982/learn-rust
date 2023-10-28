// https://leetcode.com/problems/3sum/
// 15. 3Sum
pub struct Solution;
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut map = std::collections::BTreeMap::<i32, i32>::new();
        for &n in &nums {
            *map.entry(n).or_default() += 1;
        }
        for (&i, &ni) in map.range(..=0) {
            for (&j, &nj) in map.range(i..) {
                if i == j && ni == 1 {
                    continue;
                }
                if -(i + j) < j {
                    break;
                }
                if -(i + j) == j && nj == 1 {
                    break;
                }
                if i == 0 && ni < 3 {
                    break;
                }
                if map.contains_key(&-(i + j)) {
                    res.push(vec![i, j, -(i + j)]);
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
    fn three_sum() {
        assert_eq!(
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec_vec![[-1, -1, 2], [-1, 0, 1]]
        );
        assert_eq!(Solution::three_sum(vec![0, 1, 1]), Vec::<Vec<i32>>::new());
        assert_eq!(Solution::three_sum(vec![0, 0, 0]), vec_vec![[0, 0, 0]]);
    }
}
