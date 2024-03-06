// https://leetcode.com/problems/number-of-ways-to-wear-different-hats-to-each-other/
// 1434. Number of Ways to Wear Different Hats to Each Other
pub struct Solution;
impl Solution {
    fn count_ways(
        cap: usize,
        caps: &Vec<usize>,
        person_left: usize,
        p_hat: &Vec<usize>,
        mem: &mut Vec<Vec<i64>>,
    ) -> i64 {
        if person_left == 0 {
            return 1;
        }
        if cap == usize::MAX {
            return 0;
        }
        if mem[cap][person_left] >= 0 {
            return mem[cap][person_left];
        }
        let persons = p_hat[cap];
        let mut count = 0;
        let mut np = persons;
        while np > 0 {
            let p = np.trailing_zeros();
            let pmask = 1 << p;
            np = np - pmask;
            if person_left & pmask == 0 {
                continue;
            }
            count = (count + Self::count_ways(caps[cap], caps, person_left & !(1 << p), p_hat, mem)) % 1_0000_0000_7;
        }
        count = (count + Self::count_ways(caps[cap], caps, person_left, p_hat, mem)) % 1_0000_0000_7;
        mem[cap][person_left] = count;
        count
    }
    pub fn number_ways(hats: Vec<Vec<i32>>) -> i32 {
        let n = hats.len();
        let mut mem = vec![vec![-1; 1 << n]; 41];
        let mut p_hat = vec![0; 41];
        let mut caps = vec![0; 41];
        caps[0] = usize::MAX;
        for (i, nhats) in hats.into_iter().enumerate() {
            for nhat in nhats {
                p_hat[nhat as usize] |= 1 << i;
                if caps[nhat as usize] == 0 {
                    caps[nhat as usize] = caps[0];
                    caps[0] = nhat as usize;
                }
            }
        }
        Self::count_ways(caps[0], &caps, (1 << n) - 1, &p_hat, &mut mem) as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_number_ways() {
        assert_eq!(Solution::number_ways(vec_vec![[3, 4], [4, 5], [5]]), 1);
        assert_eq!(Solution::number_ways(vec_vec![[3, 5, 1], [3, 5]]), 4);
        assert_eq!(
            Solution::number_ways(vec_vec![[1, 2, 3, 4], [1, 2, 3, 4], [1, 2, 3, 4], [1, 2, 3, 4]]),
            24
        );
    }
}
