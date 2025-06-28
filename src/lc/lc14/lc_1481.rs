// https://leetcode.com/problems/least-number-of-unique-integers-after-k-removals/
// 1481. Least Number of Unique Integers after K Removals
pub struct Solution;
impl Solution {
    pub fn find_least_num_of_unique_ints(arr: Vec<i32>, k: i32) -> i32 {
        let mut cnt = std::collections::HashMap::<i32, i32>::new();
        for x in arr {
            *cnt.entry(x).or_default() += 1
        }
        let mut cnt = cnt.into_values().collect::<Vec<_>>();
        cnt.sort_unstable();
        let mut n = cnt.len() as i32;
        let mut k = k;
        for c in cnt {
            if c <= k {
                n -= 1;
                k -= c;
            } else {
                break;
            }
        }
        n
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_least_num_of_unique_ints() {
        assert_eq!(Solution::find_least_num_of_unique_ints(vec![5, 5, 4], 1), 1);
        assert_eq!(Solution::find_least_num_of_unique_ints(vec![4, 3, 1, 1, 3, 3, 2], 3), 2);
    }
}
