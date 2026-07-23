// https://leetcode.com/problems/number-of-unique-xor-triplets-ii/
// 3514. Number of Unique XOR Triples II
pub struct Solution;
impl Solution {
    pub fn unique_xor_triplets(nums: Vec<i32>) -> i32 {
        let max = *nums.iter().max().unwrap();
        let mut n = 1;
        while n <= max {
            n <<= 1;
        }
        let mut one = vec![0; n as usize];
        let mut two = vec![0; n as usize];
        for i in 0..nums.len() {
            let ni = nums[i] as usize;
            if one[ni] == 1 {
                continue;
            }
            one[ni] = 1;
            for j in 0..n {
                let nj = j as usize;
                if one[nj] == 1 {
                    println!("t2 {} ^ {} = {}", ni, nj, ni ^ nj);
                    two[ni ^ nj] = 1;
                }
            }
        }
        let mut three = vec![0; n as usize];
        for i in 0..nums.len() {
            let ni = nums[i] as usize;
            for j in 0..n {
                let nj = j as usize;
                if two[nj] == 1 {
                    println!("t3 {} ^ {} = {}", ni, nj, ni ^ nj);
                    three[ni ^ nj] = 1;
                }
            }
        }
        three.iter().sum()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unique_xor_triplets() {
        assert_eq!(Solution::unique_xor_triplets(vec![420, 1073, 386, 496]), 8);
        assert_eq!(Solution::unique_xor_triplets(vec![1, 3]), 2);
        assert_eq!(Solution::unique_xor_triplets(vec![6, 7, 8, 9]), 4);
    }
}
