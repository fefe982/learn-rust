// https://leetcode.com/problems/maximum-earnings-from-taxi/
// 2008. Maximum Earnings From Taxi
pub struct Solution;
impl Solution {
    pub fn max_taxi_earnings(_: i32, rides: Vec<Vec<i32>>) -> i64 {
        let mut rides = rides;
        rides.sort_by_key(|ride| ride[1]);
        let mut earnings = std::collections::BTreeMap::<i32, i64>::new();
        earnings.insert(0, 0);
        for ride in rides {
            let (_, &c) = earnings.range(..=ride[0]).rev().next().unwrap();
            let earn = (ride[1] - ride[0] + ride[2]) as i64 + c;
            let (_, &c) = earnings.range(..=ride[1]).rev().next().unwrap();
            if earn <= c {
                continue;
            }
            earnings.insert(ride[1], earn);
            let mut remove = vec![];
            for (&pos, &val) in earnings.range(ride[1] + 1..) {
                if val <= earn {
                    remove.push(pos);
                } else {
                    break;
                }
            }
            for r in remove {
                earnings.remove(&r);
            }
        }
        *earnings.iter().last().unwrap().1
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_max_taxi_earnings() {
        assert_eq!(Solution::max_taxi_earnings(5, vec_vec![[2, 5, 4], [1, 5, 1]]), 7);
        assert_eq!(
            Solution::max_taxi_earnings(
                20,
                vec_vec![
                    [1, 6, 1],
                    [3, 10, 2],
                    [10, 12, 3],
                    [11, 12, 2],
                    [12, 15, 2],
                    [13, 18, 1]
                ]
            ),
            20
        );
    }
}
