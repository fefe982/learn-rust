// https://leetcode.com/problems/count-magical-substring-pairs-ii/
// 4091. Count Magical Substring Pairs II
pub struct Solution;
impl Solution {
    pub fn count_magical_pairs(s: String) -> i64 {
        let s = s.as_bytes();
        let n = s.len();
        let mut next = vec![usize::MAX; n];
        let mut first = vec![usize::MAX; 26];
        let mut last = vec![usize::MAX; 26];
        for i in 0..n {
            let c = (s[i] - b'a') as usize;
            if last[c] != usize::MAX {
                next[last[c] as usize] = i;
            }
            last[c] = i;
            if i > 0 && first[c] == usize::MAX {
                first[c] = i;
            }
        }
        last.fill(usize::MAX);
        let mut ans = 0;
        let mut c = (s[0] - b'a') as usize;
        for j in 0..n - 1 {
            last[c] = j;
            let mut li = Vec::with_capacity(26);
            let mut ri = Vec::with_capacity(26);
            for cc in 0..26 {
                if last[cc] != usize::MAX {
                    li.push(last[cc]);
                }
                if first[cc] != usize::MAX {
                    ri.push(first[cc]);
                }
            }
            li.sort();
            ri.sort();
            let mut ll = vec![0; 27];
            let d = li.len();
            ll[d] = li[0] + 1;
            for t in 1..d {
                ll[d - t] = li[t] - li[t - 1];
            }
            let mut rr = vec![0; 27];
            let d = ri.len();
            rr[d] = n - ri[d - 1];
            for t in 1..d {
                rr[t] = ri[t] - ri[t - 1];
            }
            for d in 1..=26 {
                ans += ll[d] as i64 * rr[d] as i64;
            }
            c = (s[j + 1] - b'a') as usize;
            first[c] = next[j + 1];
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_magical_pairs() {
        assert_eq!(Solution::count_magical_pairs("aba".to_string()), 2);
        assert_eq!(Solution::count_magical_pairs("kkk".to_string()), 4);
        assert_eq!(Solution::count_magical_pairs("abcac".to_string()), 7);
    }
}
