// https://leetcode.cn/problems/push-dominoes/description
// 838. Push Dominoes
pub struct Solution;
impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let mut d = dominoes.chars().collect::<Vec<_>>();
        let mut l = 0;
        let mut r = usize::MAX;
        for i in 0..d.len() {
            if d[i] == 'L' {
                if r <= i {
                    for j in 0..(i - r) / 2 {
                        d[r + j] = 'R';
                        d[i - j - 1] = 'L';
                    }
                    r = usize::MAX;
                } else {
                    for j in l..i {
                        d[j] = 'L';
                    }
                }
                l = i + 1;
            } else if d[i] == 'R' {
                if r < i {
                    for j in r..i {
                        d[j] = 'R';
                    }
                }
                r = i + 1;
            }
        }
        if r != usize::MAX {
            for i in r..d.len() {
                d[i] = 'R';
            }
        }
        d.into_iter().collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn push_dominoes() {
        assert_eq!(Solution::push_dominoes(String::from("RR.L")), String::from("RR.L"));
        assert_eq!(
            Solution::push_dominoes(String::from(".L.R...LR..L..")),
            String::from("LL.RR.LLRRLL..")
        );
    }
}
