// https://leetcode.com/problems/find-array-given-subset-sums/
// 1982. Find Array Given Subset Sums
pub struct Solution;
impl Solution {
    fn recover(sums: Vec<i32>) -> Vec<i32> {
        if sums.len() == 2 {
            if sums[0] == 0 {
                return vec![sums[1]];
            }
            return vec![sums[0]];
        }
        let mut cnt = std::collections::HashMap::<i32, i32>::new();
        let mut d = sums[1] - sums[0];
        let mut reduced = vec![];
        let mut have_zero = false;
        for sum in sums {
            let c = cnt.entry(sum).or_default();
            *c += 1;
            if *c > 0 {
                *c -= 1;
                if sum == 0 {
                    have_zero = true;
                }
                reduced.push(sum);
                *cnt.entry(sum + d).or_default() -= 1;
            }
        }
        if !have_zero {
            reduced.iter_mut().for_each(|x| *x += d);
            d = -d;
        }
        let mut v = vec![d];
        v.append(&mut Self::recover(reduced));
        v
    }
    pub fn recover_array(_n: i32, sums: Vec<i32>) -> Vec<i32> {
        let mut sums = sums;
        sums.sort_unstable();
        Self::recover(sums)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_recover_array() {
        assert_eq!(Solution::recover_array(3, vec![-3, -2, -1, 0, 0, 1, 2, 3]), [1, 2, -3]);
        assert_eq!(Solution::recover_array(2, vec![0, 0, 0, 0]), [0, 0]);
        assert_eq!(
            Solution::recover_array(4, vec![0, 0, 5, 5, 4, -1, 4, 9, 9, -1, 4, 3, 4, 8, 3, 8]),
            [0, -1, 4, 5]
        );
    }
}
