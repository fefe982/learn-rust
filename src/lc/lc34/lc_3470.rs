// https://leetcode.com/problems/permutations-iv/
// 3470. Permutations IV
pub struct Solution;
impl Solution {
    pub fn permute(n: i32, k: i64) -> Vec<i32> {
        let mut l0 = 0;
        let mut l1 = 0;
        let mut cnt = 1;
        for i in (1..=n).rev() {
            if i % 2 == 0 {
                l0 += 1;
                cnt *= l0 as i64;
            } else {
                l1 += 1;
                cnt *= l1 as i64;
            }
            if cnt >= k {
                break;
            }
        }
        if (n % 2 == 1 && cnt < k) || (n % 2 == 0 && cnt * 2 < k) {
            return vec![];
        }

        let mut ans = (1..=(n - l0 - l1)).collect::<Vec<i32>>();
        let mut m0 = (1 << l0) - 1;
        let mut m1 = (1 << l1) - 1;
        let s0 = (n / 2 - l0 + 1) * 2;
        let s1 = ((n - 1) / 2 + 1 - l1) * 2 + 1;
        let mut swap = false;
        let mut k = k;
        if n % 2 == 0 && n == l0 + l1 {
            let pcnt = cnt / l0 as i64;
            let c = (k - 1) / pcnt + 1;
            ans.push(c as i32);
            if c % 2 == 0 {
                swap = true;
                m0 ^= 1 << (c / 2 - 1);
                l0 -= 1;
            } else {
                m1 ^= 1 << (c / 2);
                l1 -= 1;
            }
            k -= (c - 1) * pcnt;
            cnt = pcnt;
        }
        println!("{ans:?}");
        for i in (n - l0 - l1 + 1)..=n {
            let mask;
            let s;
            let l;
            if (i % 2 == 0) ^ swap {
                mask = &mut m0;
                s = s0;
                l = &mut l0;
            } else {
                mask = &mut m1;
                s = s1;
                l = &mut l1;
            }
            let pcnt = cnt / *l as i64;
            *l -= 1;
            let c = (k - 1) / pcnt;
            let mm: i32 = *mask;
            let mut b = mm.trailing_zeros() as i32;
            let mut cc = c;
            while cc > 0 {
                cc -= 1;
                b += 1;
                while mm & (1 << b) == 0 {
                    b += 1;
                }
            }
            ans.push(s + b * 2);
            *mask = mm ^ (1 << b);
            cnt = pcnt;
            k -= pcnt * c;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn permute() {
        assert_eq!(Solution::permute(4, 6), [3, 4, 1, 2]);
        assert_eq!(Solution::permute(3, 2), [3, 2, 1]);
        assert_eq!(Solution::permute(2, 3), []);
    }
}
