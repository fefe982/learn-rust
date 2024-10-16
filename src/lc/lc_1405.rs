// https://leetcode.com/problems/longest-happy-string/
// 1405. Longest Happy String
pub struct Solution;
impl Solution {
    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        let mut q = std::collections::BinaryHeap::new();
        if a > 0 {
            q.push((a, 'a'));
        }
        if b > 0 {
            q.push((b, 'b'));
        }
        if c > 0 {
            q.push((c, 'c'));
        }
        let mut last = ' ';
        let mut lastc = 0;
        let mut res = String::new();
        while let Some((count, ch)) = q.pop() {
            if ch == last {
                if lastc < 2 {
                    res.push(ch);
                    if count > 1 {
                        q.push((count - 1, ch));
                    }
                    lastc += 1;
                } else {
                    if let Some((count2, ch2)) = q.pop() {
                        res.push(ch2);
                        if count2 > 1 {
                            q.push((count2 - 1, ch2));
                        }
                        q.push((count, ch));
                        last = ch2;
                        lastc = 1;
                    } else {
                        break;
                    }
                }
            } else {
                res.push(ch);
                if count > 1 {
                    q.push((count - 1, ch));
                }
                last = ch;
                lastc = 1;
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn check(a: i32, b: i32, c: i32, expect: &str) {
        let res = Solution::longest_diverse_string(a, b, c);
        println!("{res:?} {expect:?}");
        assert_eq!(res.len(), expect.len());
        let mut ac = 0;
        let mut bc = 0;
        let mut cc = 0;
        let mut last = ' ';
        let mut lastc = 0;
        for ch in res.chars() {
            match ch {
                'a' => ac += 1,
                'b' => bc += 1,
                'c' => cc += 1,
                _ => assert!(false),
            }
            if ch == last {
                lastc += 1;
            } else {
                last = ch;
                lastc = 1;
            }
            if lastc >= 3 {
                assert!(false);
            }
        }
        assert!(ac <= a && bc <= b && cc <= c);
    }
    #[test]
    fn test_longest_diverse_string() {
        check(1, 1, 7, "ccaccbcc");
        check(7, 1, 0, "aabaa");
    }
}
