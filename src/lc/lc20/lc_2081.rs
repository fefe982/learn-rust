// https://leetcode.com/problems/sum-of-k-mirror-numbers/
// 2081. Sum of k-Mirror Numbers
pub struct Solution;
impl Solution {
    fn next(mut v: Vec<i32>, radix: i32) -> Vec<i32> {
        let n = v.len();
        let mid = n / 2;
        for i in mid..n {
            if v[i] + 1 < radix {
                let nd = v[i] + 1;
                v[i] = nd;
                v[n - i - 1] = nd;
                for j in n - i..i {
                    v[j] = 0;
                }
                return v;
            }
        }
        v.fill(0);
        v[0] = 1;
        v.push(1);
        v
    }
    fn val(v: &Vec<i32>, radix: i32) -> i64 {
        v.iter().fold(0i64, |acc, &i| acc * radix as i64 + i as i64)
    }
    fn check(mut val: i64, radix: i32) -> bool {
        let mut v = vec![];
        let radix = radix as i64;
        while val > 0 {
            v.push((val % radix) as i32);
            val /= radix;
        }
        for i in 0..v.len() / 2 {
            if v[i] != v[v.len() - i - 1] {
                return false;
            }
        }
        true
    }
    pub fn k_mirror(k: i32, n: i32) -> i64 {
        let mut v = vec![0];
        let mut sum = 0i64;
        for _ in 0..n {
            loop {
                v = Solution::next(v, 10);
                let val = Solution::val(&v, 10);
                if Solution::check(val, k) {
                    sum += val;
                    break;
                }
            }
        }
        sum
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_k_mirror() {
        assert_eq!(Solution::k_mirror(2, 5), 25);
        assert_eq!(Solution::k_mirror(3, 7), 499);
        assert_eq!(Solution::k_mirror(7, 17), 20379000);
    }
}
