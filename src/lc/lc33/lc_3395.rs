// https://leetcode.com/problems/subsequences-with-a-unique-middle-mode-i/
// 3395. Subsequences With a Unique Middle Mode I
pub struct Solution;
const M: i64 = 1_000_000_007;
impl Solution {
    fn cnt2(
        cntl: &std::collections::HashMap<i32, usize>,
        cntr: &std::collections::HashMap<i32, usize>,
        cl: usize,
        ncr: usize,
        n: i32,
    ) -> i64 {
        let mut ans = 0;
        let mut sumrr = 0;
        for (&k, &v) in cntr.iter() {
            if k != n {
                sumrr += (v * v) as i64;
            }
        }
        let rr = ((ncr * ncr) as i64 - sumrr) / 2;
        for (&k, &v) in cntl.iter() {
            if k != n {
                if let Some(&r) = cntr.get(&k) {
                    let ncr = (ncr - r) as i64;
                    ans = (ans + (ncr * ncr - sumrr + (r * r) as i64) / 2 * v as i64) % M;
                } else {
                    ans = (ans + rr * v as i64) % M;
                }
            }
        }
        (ans * cl as i64) % M
    }
    pub fn subsequences_with_middle_mode(nums: Vec<i32>) -> i32 {
        let mut cnt = std::collections::HashMap::<i32, usize>::new();
        for &num in &nums {
            *cnt.entry(num).or_default() += 1;
        }
        let mut cc = vec![1; nums.len()];
        for i in 3..nums.len() {
            cc[i] = cc[i - 1] + i as i64 - 1;
        }
        let mut ans = 0;
        let mut cntl = std::collections::HashMap::new();
        let mut cntr = cnt;
        cntl.insert(nums[0], 1);
        fn remove(cnt: &mut std::collections::HashMap<i32, usize>, n: i32) -> usize {
            let c = cnt.get_mut(&n).unwrap();
            *c -= 1;
            if *c == 0 {
                cnt.remove(&n);
                0
            } else {
                *c
            }
        }
        *cntl.entry(nums[1]).or_default() += 1;
        remove(&mut cntr, nums[0]);
        remove(&mut cntr, nums[1]);
        for i in 2..nums.len() - 1 {
            let cr = remove(&mut cntr, nums[i]);
            let cl = cntl.get(&nums[i]).cloned().unwrap_or(0);
            let ncl = i - cl;
            let ncr = (nums.len() - i - 1) - cr;
            if cl >= 2 && cr >= 2 {
                ans = (ans + cc[cl] * cc[cr]) % M;
            }
            if cl >= 1 && cr >= 2 {
                ans = (ans + (cl * ncl) as i64 * cc[cr]) % M;
            }
            if cl >= 2 && cr >= 1 {
                ans = (ans + cc[cl] * (cr * ncr) as i64) % M;
            }
            if ncl >= 2 && cr >= 2 {
                ans = (ans + cc[ncl] * cc[cr]) % M;
            }
            if cl >= 1 && cr >= 1 {
                ans = (ans + (cr * cl * ncr * ncl) as i64) % M;
            }
            if cl >= 2 && ncr >= 2 {
                ans = (ans + cc[cl] * cc[ncr]) % M;
            }
            if cl >= 1 && ncr >= 2 {
                ans = (ans + Self::cnt2(&cntl, &cntr, cl, ncr, nums[i])) % M;
            }
            if ncl >= 2 && cr >= 1 {
                ans = (ans + Self::cnt2(&cntr, &cntl, cr, ncl, nums[i])) % M;
            }
            *cntl.entry(nums[i]).or_default() += 1;
        }
        ans as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn subsequences_with_middle_mode() {
        assert_eq!(Solution::subsequences_with_middle_mode(vec![1, 1, 1, 1, 1, 1]), 6);
        assert_eq!(Solution::subsequences_with_middle_mode(vec![1, 2, 2, 3, 3, 4]), 4);
        assert_eq!(
            Solution::subsequences_with_middle_mode(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]),
            0
        );
    }
}
