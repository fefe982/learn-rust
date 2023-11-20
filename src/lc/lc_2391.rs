// https://leetcode.com/problems/minimum-amount-of-time-to-collect-garbage/
// 2391. Minimum Amount of Time to Collect Garbage
pub struct Solution;
impl Solution {
    pub fn garbage_collection(garbage: Vec<String>, travel: Vec<i32>) -> i32 {
        let travel = vec![0]
            .into_iter()
            .chain(travel.into_iter())
            .scan(0, |acc, x| {
                *acc += x;
                Some(*acc)
            })
            .collect::<Vec<_>>();
        let mut last_m = 0;
        let mut last_p = 0;
        let mut last_g = 0;
        let mut sum_m = 0;
        let mut sum_p = 0;
        let mut sum_g = 0;
        for (i, g) in garbage.into_iter().enumerate() {
            for c in g.chars() {
                match c {
                    'M' => {
                        sum_m += 1;
                        last_m = i;
                    }
                    'P' => {
                        sum_p += 1;
                        last_p = i;
                    }
                    'G' => {
                        sum_g += 1;
                        last_g = i;
                    }
                    _ => unreachable!(),
                }
            }
        }
        sum_m + sum_p + sum_g + travel[last_m] + travel[last_p] + travel[last_g]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_garbage_collection() {
        assert_eq!(
            Solution::garbage_collection(vec_str!["G", "P", "GP", "GG"], vec![2, 4, 3]),
            21
        );
        assert_eq!(
            Solution::garbage_collection(vec_str!["MMM", "PGM", "GP"], vec![3, 10]),
            37
        );
    }
}
