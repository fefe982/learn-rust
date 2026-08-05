// https://leetcode.com/problems/minimum-number-of-groups-to-create-a-valid-assignment/
// 2910. Minimum Number of Groups to Create a Valid Assignment
pub struct Solution;
impl Solution {
    pub fn min_groups_for_valid_assignment(balls: Vec<i32>) -> i32 {
        let mut map = std::collections::HashMap::new();
        for b in balls {
            map.entry(b).and_modify(|e| *e += 1).or_insert(1);
        }
        let mut min = i32::MAX;
        for (_, &v) in &map {
            min = min.min(v);
        }
        'n: for i in (1..=min).rev() {
            let mut cnt = 0;
            for (_, &v) in &map {
                if v % (i + 1) == 0 {
                    cnt += v / (i + 1);
                } else if v / (i + 1) >= i - v % (i + 1) {
                    cnt += v / (i + 1) + 1;
                } else {
                    continue 'n;
                }
            }
            return cnt;
        }
        unreachable!()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn min_groups_for_valid_assignment() {
        assert_eq!(
            Solution::min_groups_for_valid_assignment(vec![1, 1, 3, 3, 1, 1, 2, 2, 3, 1, 3, 2]),
            5
        );
        assert_eq!(Solution::min_groups_for_valid_assignment(vec![3, 2, 3, 2, 3]), 2);
        assert_eq!(Solution::min_groups_for_valid_assignment(vec![10, 10, 10, 3, 1, 1]), 4);
    }
}
