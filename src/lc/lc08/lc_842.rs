// https://leetcode.com/problems/split-array-into-fibonacci-sequence/
// 842. Split Array into Fibonacci Sequence
pub struct Solution;
impl Solution {
    fn is_fibonacci(num: &str, v: &mut Vec<i32>) -> bool {
        if num.is_empty() {
            return true;
        }
        let lv = v.len();
        if let Some(s) = v[lv - 2].checked_add(v[lv - 1]) {
            let ss = s.to_string();
            if num.starts_with(&ss) {
                v.push(s);
                if Self::is_fibonacci(&num[ss.len()..], v) {
                    return true;
                } else {
                    v.pop();
                }
            }
        }
        false
    }
    pub fn split_into_fibonacci(num: String) -> Vec<i32> {
        let nb = num.as_str();
        let mut v = vec![];
        for i in 1..(num.len() - 2) {
            if nb.as_bytes()[0] == b'0' && i > 1 {
                break;
            }
            let n = i64::from_str_radix(&nb[0..i], 10).unwrap();
            if n > i32::MAX as i64 {
                break;
            }
            v.push(n as i32);
            for j in i + 1..(num.len() - 1) {
                if nb.as_bytes()[i] == b'0' && j > i + 1 {
                    break;
                }
                let nj = i64::from_str_radix(&nb[i..j], 10).unwrap();
                if nj > i32::MAX as i64 {
                    break;
                }
                v.push(nj as i32);
                if Self::is_fibonacci(&num[j..], &mut v) {
                    return v;
                }
                v.pop();
            }
            v.pop();
        }
        v
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn check(num: &str, expect: bool) {
        let res = Solution::split_into_fibonacci(num.to_string());
        println!("{num:?}, {res:?}");
        assert_eq!(res.len() > 0, expect);
        if expect {
            assert!(res.len() > 2);
            assert_eq!(res.iter().fold("".to_string(), |acc, &x| acc + &x.to_string()), num);
            for i in 2..res.len() {
                assert_eq!(Some(res[i]), res[i - 1].checked_add(res[i - 2]));
            }
        }
    }
    #[test]
    fn split_into_fibonacci() {
        check("121474836462147483647", true);
        check(
            "98131550821532132082953615085698039313111501114243242539335396365964102995031666546726964970",
            true,
        );
        check("2820022842865610841740282445647388119521934031292", true);
        check("1101111", true);
        check("112358130", false);
        check("0123", false);
    }
}
