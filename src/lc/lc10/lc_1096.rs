// https://leetcode.com/problems/brace-expansion-ii/
// 1096. Brace Expansion II
pub struct Solution;
use std::{collections::BTreeSet, str::from_utf8};
impl Solution {
    fn expand_brace(expr: &[u8], idx: &mut usize) -> BTreeSet<String> {
        let mut result = BTreeSet::new();
        while expr[*idx] as char != '}' {
            *idx += 1;
            result.append(&mut Self::expand_concat(expr, idx));
        }
        *idx += 1;
        result
    }
    fn expand_str(expr: &[u8], idx: &mut usize) -> String {
        let beg = *idx;
        while *idx < expr.len()
            && expr[*idx] as char != '{'
            && expr[*idx] as char != '}'
            && expr[*idx] as char != ','
        {
            *idx += 1;
        }
        from_utf8(&expr[beg..*idx]).unwrap().to_string()
    }
    fn expand_concat(expr: &[u8], idx: &mut usize) -> BTreeSet<String> {
        let mut result = BTreeSet::from([String::from("")]);
        while *idx < expr.len() {
            match expr[*idx] as char {
                '{' => {
                    let piece_set = Self::expand_brace(expr, idx);
                    let mut n_set = BTreeSet::new();
                    for a in result.iter() {
                        for b in piece_set.iter() {
                            n_set.insert(a.clone() + b);
                        }
                    }
                    result = n_set
                }
                ',' | '}' => break,
                _ => {
                    let s = Self::expand_str(expr, idx);
                    result = result.into_iter().map(|x| x + &s).collect();
                }
            }
        }
        result
    }
    pub fn brace_expansion_ii(expression: String) -> Vec<String> {
        let expr = expression.as_bytes();
        Self::expand_concat(expr, &mut 0).into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn brace_expansion_ii() {
        assert_eq!(
            Solution::brace_expansion_ii(String::from("{a,b}{c,{d,e}}")),
            vec!["ac", "ad", "ae", "bc", "bd", "be"]
        );
        assert_eq!(
            Solution::brace_expansion_ii(String::from("{{a,z},a{b,c},{ab,z}}")),
            vec!["a", "ab", "ac", "z"]
        );
    }
}
