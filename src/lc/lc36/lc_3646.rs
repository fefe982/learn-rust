// https://leetcode.com/problems/next-special-palindrome-number/
// 3646. Next Special Palindrome
pub struct Solution;
impl Solution {
    fn walk_perm(ll: &Vec<i32>, n: &Vec<i32>) -> i64 {
        let mut cnt = vec![0; 10];
        let mut mid = 0;
        let len = n.len();
        for &i in ll {
            cnt[i as usize] = i / 2;
            if i % 2 == 1 {
                mid = i;
            }
        }
        let mut nn = Vec::with_capacity(len / 2);
        let mut g = 0;
        'i: for i in 0..len / 2 {
            if cnt[n[i] as usize] > 0 {
                cnt[n[i] as usize] -= 1;
                nn.push(n[i]);
            } else {
                for j in n[i] + 1..10 {
                    if cnt[j as usize] > 0 {
                        cnt[j as usize] -= 1;
                        nn.push(j);
                        g = 1;
                        for k in 1..10 {
                            for _ in 0..cnt[k] {
                                nn.push(k as i32);
                            }
                        }
                        break 'i;
                    }
                }
                g = -1;
                break;
            }
        }
        if g == 0 {
            let mut s = len / 2;
            if len % 2 == 1 {
                match mid.cmp(&n[s]) {
                    std::cmp::Ordering::Greater => g = 1,
                    std::cmp::Ordering::Less => g = -1,
                    _ => {}
                }
                s += 1;
            }
            for i in s..len {
                if g != 0 {
                    break;
                }
                match nn[len - 1 - i].cmp(&n[i]) {
                    std::cmp::Ordering::Greater => g = 1,
                    std::cmp::Ordering::Less => g = -1,
                    _ => {}
                }
            }
        }
        if g < 1 {
            'd: while let Some(d) = nn.pop() {
                cnt[d as usize] += 1;
                for i in d + 1..10 {
                    if cnt[i as usize] > 0 {
                        cnt[i as usize] -= 1;
                        nn.push(i);
                        g = 1;
                        for k in 1..10 {
                            for _ in 0..cnt[k] {
                                nn.push(k as i32);
                            }
                        }
                        break 'd;
                    }
                }
            }
            if g != 1 {
                return i64::MAX;
            }
        }
        let mut val = 0;
        for &d in &nn {
            val = val * 10 + d as i64;
        }
        if len % 2 == 1 {
            val = val * 10 + mid as i64;
        }
        for d in nn.into_iter().rev() {
            val = val * 10 + d as i64;
        }
        val
    }
    fn walk_palindrome(len: i32, odd: bool, start: i32, ll: &mut Vec<i32>, n: &Vec<i32>) -> i64 {
        let mut r = i64::MAX;
        if odd {
            for i in 0..5i32 {
                let d = i * 2 + 1;
                if d > len {
                    return r;
                }
                ll.push(d);
                r = r.min(Self::walk_palindrome(len - d, false, 2, ll, n));
                ll.pop();
            }
        } else {
            if len == 0 {
                r = r.min(Self::walk_perm(ll, n));
            } else {
                for i in start / 2..5 {
                    let d = i * 2;
                    if d > len {
                        return r;
                    }
                    ll.push(d);
                    r = r.min(Self::walk_palindrome(len - d, false, d + 2, ll, n));
                    ll.pop();
                }
            }
        }
        r
    }
    pub fn special_palindrome(n: i64) -> i64 {
        let mut ns = n
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect::<Vec<i32>>();
        let len = ns.len() as i32;
        let mut val = Self::walk_palindrome(len, len % 2 == 1, 2, &mut vec![], &ns);
        if val == i64::MAX {
            ns.insert(0, 0);
            val = Self::walk_palindrome(len + 1, len % 2 == 0, 2, &mut vec![], &ns);
        }
        val
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn special_palindrome() {
        assert_eq!(Solution::special_palindrome(2441442), 2555552);
        assert_eq!(Solution::special_palindrome(95048245), 234434432);
        assert_eq!(Solution::special_palindrome(2), 22);
        assert_eq!(Solution::special_palindrome(33), 212);
    }
}
