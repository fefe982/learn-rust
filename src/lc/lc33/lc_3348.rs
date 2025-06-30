// https://leetcode.com/problems/smallest-divisible-digit-product-ii/
// 3348. Smallest Divisible Digit Product II
pub struct Solution;
impl Solution {
    pub fn smallest_number(num: String, t: i64) -> String {
        let mut p2 = 0;
        let mut p3 = 0;
        let mut p5 = 0;
        let mut p7 = 0;
        let mut t = t;
        while t > 0 && t % 2 == 0 {
            p2 += 1;
            t /= 2;
        }
        while t > 0 && t % 3 == 0 {
            p3 += 1;
            t /= 3;
        }
        while t > 0 && t % 5 == 0 {
            p5 += 1;
            t /= 5;
        }
        while t > 0 && t % 7 == 0 {
            p7 += 1;
            t /= 7;
        }
        if t != 1 {
            return "-1".to_string();
        }
        fn get_min_len(mut n2: i32, mut n3: i32, n5: i32, n7: i32) -> (i32, i32, i32, i32, i32, i32, i32, i32, i32) {
            let n9 = n3 / 2;
            n3 %= 2;
            let n8 = n2 / 3;
            n2 %= 3;
            let n6 = n2.min(n3);
            n2 -= n6;
            n3 -= n6;
            let n4 = n2 / 2;
            n2 %= 2;
            (n2 + n3 + n4 + n5 + n6 + n7 + n8 + n9, n2, n3, n4, n5, n6, n7, n8, n9)
        }
        let (mlen, m2, m3, m4, m5, m6, m7, n8, n9) = get_min_len(p2, p3, p5, p7);
        let num = num.as_bytes();
        if mlen as usize > num.len() {
            return std::iter::repeat('2')
                .take(m2 as usize)
                .chain(std::iter::repeat('3').take(m3 as usize))
                .chain(std::iter::repeat('4').take(m4 as usize))
                .chain(std::iter::repeat('5').take(m5 as usize))
                .chain(std::iter::repeat('6').take(m6 as usize))
                .chain(std::iter::repeat('7').take(m7 as usize))
                .chain(std::iter::repeat('8').take(n8 as usize))
                .chain(std::iter::repeat('9').take(n9 as usize))
                .collect();
        }
        let mut dnum = num.iter().map(|&c| c - b'0').collect::<Vec<_>>();
        fn fill(i: usize, n2: i32, n3: i32, n5: i32, n7: i32, dnum: &mut Vec<u8>) -> bool {
            if i as usize == dnum.len() {
                return n2 == 0 && n3 == 0 && n5 == 0 && n7 == 0;
            }
            let (mlen, m2, m3, m4, m5, m6, m7, m8, m9) = get_min_len(n2, n3, n5, n7);
            if i + mlen as usize > dnum.len() {
                dnum[i] = 0;
                return false;
            }
            if dnum[i] == 0 {
                let mut j = i;
                while (j + mlen as usize) < dnum.len() {
                    dnum[j] = 1;
                    j += 1;
                }
                for _ in 0..m2 {
                    dnum[j] = 2;
                    j += 1;
                }
                for _ in 0..m3 {
                    dnum[j] = 3;
                    j += 1;
                }
                for _ in 0..m4 {
                    dnum[j] = 4;
                    j += 1;
                }
                for _ in 0..m5 {
                    dnum[j] = 5;
                    j += 1;
                }
                for _ in 0..m6 {
                    dnum[j] = 6;
                    j += 1;
                }
                for _ in 0..m7 {
                    dnum[j] = 7;
                    j += 1;
                }
                for _ in 0..m8 {
                    dnum[j] = 8;
                    j += 1;
                }
                for _ in 0..m9 {
                    dnum[j] = 9;
                    j += 1;
                }
                return true;
            } else {
                for d in dnum[i]..=9 {
                    let (nn2, nn3, nn5, nn7) = match d {
                        1 => (n2, n3, n5, n7),
                        2 => ((n2 - 1).max(0), n3, n5, n7),
                        3 => (n2, (n3 - 1).max(0), n5, n7),
                        4 => ((n2 - 2).max(0), n3, n5, n7),
                        5 => (n2, n3, (n5 - 1).max(0), n7),
                        6 => ((n2 - 1).max(0), (n3 - 1).max(0), n5, n7),
                        7 => (n2, n3, n5, (n7 - 1).max(0)),
                        8 => ((n2 - 3).max(0), n3, n5, n7),
                        9 => (n2, (n3 - 2).max(0), n5, n7),
                        _ => unreachable!(),
                    };
                    if fill(i + 1, nn2, nn3, nn5, nn7, dnum) {
                        dnum[i] = d;
                        return true;
                    }
                }
            }
            dnum[i] = 1;
            false
        }
        if fill(0, p2, p3, p5, p7, &mut dnum) {
            return dnum.into_iter().map(|c| (c + b'0') as char).collect();
        }
        return std::iter::repeat('1')
            .take(num.len() + 1 - mlen as usize)
            .chain(std::iter::repeat('2').take(m2 as usize))
            .chain(std::iter::repeat('3').take(m3 as usize))
            .chain(std::iter::repeat('4').take(m4 as usize))
            .chain(std::iter::repeat('5').take(m5 as usize))
            .chain(std::iter::repeat('6').take(m6 as usize))
            .chain(std::iter::repeat('7').take(m7 as usize))
            .chain(std::iter::repeat('8').take(n8 as usize))
            .chain(std::iter::repeat('9').take(n9 as usize))
            .collect();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn smallest_number() {
        assert_eq!(Solution::smallest_number("1234".to_string(), 256), "1488");
        assert_eq!(Solution::smallest_number("12355".to_string(), 50), "12355");
        assert_eq!(Solution::smallest_number("11111".to_string(), 26), "-1");
    }
}
