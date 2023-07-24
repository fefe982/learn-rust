// https://leetcode.com/problems/count-the-repetitions/
// 466. Count The Repetitions
pub struct Solution;
impl Solution {
    pub fn get_max_repetitions(s1: String, n1: i32, s2: String, n2: i32) -> i32 {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let mut rec = vec![0];
        let mut endpos = std::collections::HashMap::new();
        let mut cnt = 0;
        let mut start = 0;
        let mut cycle_start;
        loop {
            's2: for &c2 in s2 {
                for _ in 0..2 {
                    for (i, &c1) in s1[start..].iter().enumerate() {
                        if c1 == c2 {
                            start += i + 1;
                            continue 's2;
                        }
                    }
                    cnt += 1;
                    start = 0;
                }
                return 0;
            }
            rec.push(cnt + 1);
            if cnt > n1 {
                return (rec.len() as i32 - 2) / n2;
            }
            if let Some(&rep) = endpos.get(&start) {
                cycle_start = rep as i32;
                break;
            }
            endpos.insert(start, rec.len() - 1);
        }
        let cycle_l2 = rec.len() as i32 - 1 - cycle_start;
        let cycle_l1 = cnt + 1 - rec[cycle_start as usize];
        let cycles = (n1 - rec[cycle_start as usize]) / cycle_l1;
        while rec[cycle_start as usize + 1] + cycles * cycle_l1 <= n1 {
            cycle_start += 1;
        }
        (cycle_start + cycles * cycle_l2) / n2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_max_repetitions() {
        assert_eq!(
            Solution::get_max_repetitions(String::from("bacaba"), 3, String::from("abacab"), 1),
            2
        );
        assert_eq!(
            Solution::get_max_repetitions(String::from("baba"), 11, String::from("baab"), 1),
            7
        );
        assert_eq!(
            Solution::get_max_repetitions(String::from("aaa"), 20, String::from("aaaaa"), 1),
            12
        );
        assert_eq!(
            Solution::get_max_repetitions(String::from("aaa"), 3, String::from("aa"), 1),
            4
        );
        assert_eq!(
            Solution::get_max_repetitions(String::from("acb"), 4, String::from("ab"), 2),
            2
        );
        assert_eq!(
            Solution::get_max_repetitions(String::from("acb"), 1, String::from("acb"), 1),
            1
        );
    }
}
