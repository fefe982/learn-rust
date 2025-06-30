// https://leetcode.com/problems/sorted-gcd-pair-queries/
// 3312. Sorted GCD Pair Queries
pub struct Solution;
impl Solution {
    fn get_divs(n: i32, divs: &mut Vec<Vec<i32>>) {
        if divs[n as usize].len() > 0 {
            return;
        }
        if n != 1 {
            divs[n as usize].push(1);
        }
        for i in 2.. {
            if i * i > n {
                break;
            }
            if n % i == 0 {
                divs[n as usize].push(i as i32);
                if i * i != n {
                    divs[n as usize].push(n / i as i32);
                }
            }
        }
    }
    pub fn gcd_values(nums: Vec<i32>, queries: Vec<i64>) -> Vec<i32> {
        let max = *nums.iter().max().unwrap();
        let mut div = vec![vec![]; max as usize + 1];
        let mut cnt = vec![0; max as usize + 1];
        for n in nums {
            Self::get_divs(n, &mut div);
            for &d in div[n as usize].iter().chain([n].iter()) {
                cnt[d as usize] += 1;
            }
        }
        let mut gcd = vec![0; max as usize + 1];
        for (k, &v) in cnt.iter().enumerate().skip(1).rev() {
            let v = v as i64;
            let nv = v * (v - 1) / 2;
            gcd[k] += nv;
            for j in 2..=max as usize / k {
                gcd[k] -= gcd[k * j];
            }
        }
        let cnt = gcd
            .into_iter()
            .enumerate()
            .filter(|&(_, v)| v != 0)
            .scan(0, |s, (k, v)| {
                *s += v;
                Some((k, *s))
            })
            .collect::<Vec<_>>();
        let mut ans = Vec::with_capacity(queries.len());
        for q in queries {
            let p = cnt.partition_point(|x| x.1 <= q);
            ans.push(cnt[p].0 as i32);
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gcd_values() {
        assert_eq!(Solution::gcd_values(vec![13, 19, 3], vec![1, 0, 2]), [1, 1, 1]);
        assert_eq!(Solution::gcd_values(vec![2, 3, 4], vec![0, 2, 2]), [1, 2, 2]);
        assert_eq!(Solution::gcd_values(vec![4, 4, 2, 1], vec![5, 3, 1, 0]), [4, 2, 1, 1]);
        assert_eq!(Solution::gcd_values(vec![2, 2], vec![0, 0]), [2, 2]);
    }
}
