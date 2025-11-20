// https://leetcode.cn/problems/qoQAMX/
// LCP 80. 生物进化录
pub struct Solution;
impl Solution {
    pub fn evolutionary_record(parents: Vec<i32>) -> String {
        let mut children = vec![vec![]; parents.len()];
        for (i, &p) in parents.iter().enumerate() {
            if p != -1 {
                children[p as usize].push(i);
            }
        }
        fn dfs(children: &Vec<Vec<usize>>, i: usize) -> Vec<char> {
            let mut ctree = Vec::with_capacity(children[i].len());
            for &c in children[i].iter() {
                ctree.push(dfs(children, c));
            }
            ctree.sort();
            ['0']
                .into_iter()
                .chain(ctree.into_iter().flatten())
                .chain(['1'].into_iter())
                .collect::<Vec<char>>()
        }
        let vc = dfs(&children, 0);
        let mut l = vc.len() - 1;
        while vc[l] == '1' {
            l -= 1;
        }
        vc[1..=l].iter().collect::<String>()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(Solution::evolutionary_record(vec![-1, 0, 0, 2]), "00110");
        assert_eq!(Solution::evolutionary_record(vec![-1, 0, 0, 1, 2, 2]), "00101100");
    }
}
