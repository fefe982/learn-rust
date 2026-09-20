// https://leetcode.com/problems/lexicographically-largest-power-array/
// 4059. Lexicographically Largest Power Array
pub struct Solution;
impl Solution {
    pub fn largest_power(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; 15];
        let max = *nums.iter().max().unwrap();
        let mut vg = vec![nums];
        let b = (i32::BITS - max.leading_zeros()) as usize;
        for i in (0..b).rev() {
            let mut nvg = vec![];
            let mut cnt = 0;
            let mut z = false;
            for g in vg {
                if z {
                    nvg.push(g);
                    continue;
                }
                let mut ones = vec![];
                let mut zeros = vec![];
                for x in g {
                    if x & (1 << i) != 0 {
                        ones.push(x);
                    } else {
                        zeros.push(x);
                    }
                }
                if !ones.is_empty() {
                    cnt += ones.len() as i32;
                    nvg.push(ones);
                }
                if !zeros.is_empty() {
                    nvg.push(zeros);
                    z = true;
                }
            }
            res[14 - i] = cnt;
            vg = nvg;
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn largest_power() {
        assert_eq!(
            Solution::largest_power(vec![7, 5]),
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 1, 2]
        );
        assert_eq!(
            Solution::largest_power(vec![3, 1, 7]),
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3]
        );
    }
}
