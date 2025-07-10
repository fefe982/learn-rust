// https://leetcode.com/problems/guess-the-word/
// 843. Guess the Word
static mut CNT: i32 = 0;
pub struct Master {
    word: String,
}
impl Master {
    fn guess(&self, word: String) -> i32 {
        let mut c = 0;
        for (c1, c2) in word.as_bytes().iter().zip(self.word.as_bytes().iter()) {
            if c1 == c2 {
                c += 1;
            }
        }
        unsafe {
            CNT += 1;
        }
        c
    }
}
pub struct Solution;
impl Solution {
    pub fn find_secret_word(words: Vec<String>, master: &Master) {
        let n = words.len();
        let mut dist = vec![vec![0; n]; n];
        for i in 0..n {
            dist[i][i] = 6;
            for j in i + 1..n {
                let mut c = 0;
                for (c1, c2) in words[i].as_bytes().iter().zip(words[j].as_bytes().iter()) {
                    if c1 == c2 {
                        c += 1;
                    }
                }
                dist[i][j] = c;
                dist[j][i] = c;
            }
        }
        let mut possible = (0..n).collect::<Vec<_>>();
        loop {
            let mut idx = 0;
            let mut min = n;
            for &pidx in &possible {
                let mut max = 0;
                let mut c = [0; 6];
                for &ppidx in &possible {
                    if ppidx != pidx {
                        c[dist[pidx][ppidx]] += 1;
                        if c[dist[pidx][ppidx]] > max {
                            max = c[dist[pidx][ppidx]];
                        }
                    }
                }
                if max < min {
                    min = max;
                    idx = pidx;
                }
            }
            let word = words[idx].clone();
            let n = master.guess(word.clone());
            if n == 6 {
                return;
            }
            possible = possible.into_iter().filter(|&i| dist[i][idx] == n as usize).collect();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    fn check(words: Vec<String>, target: String, limit: i32) {
        let master = Master { word: target };
        unsafe {
            super::CNT = 0;
        }
        Solution::find_secret_word(words, &master);
        unsafe {
            assert!(super::CNT <= limit);
        }
    }
    #[test]
    fn find_secret_word() {
        check(
            vec_str![
                "oahwep", "eqznzs", "vvmplb", "dqefpt", "kmjmxr", "ihkovg", "trbzyb", "xqulhc", "bcsbfw", "rwzslk",
                "abpjhw", "mpubps", "viyzbc", "kodlta", "ckfzjh", "phuepp", "rokoro", "nxcwmo", "awvqlr", "uooeon",
                "sajxgr", "oxgaix", "fnugyu", "lkxwru", "mhtrvb", "xxonmg", "hhfuzz", "tqxlbr", "euxtzg", "tjwvad",
                "uslult", "rtjosi", "hsygda", "vyuica", "mbnagm", "uinqur", "pikenp", "szgupv", "qpxmsw", "vunxdn",
                "jahhfn", "kmbeok", "biywow", "wichbx", "tpulot", "hwzodo", "loffxk", "xavzqd", "vwzpfe", "uairjw",
                "itufkt", "kaklud", "jjinfa", "kqbttl", "zocgux", "ucwjig", "meesxb", "uysfyc", "kdfvtw", "vizxrv",
                "rpbdjh", "wynohw", "lhqxvx", "kaaaaa", "dxxwut", "vjtskm", "yrdswc", "byzjxm", "jeomdc", "saevda",
                "himevi", "ydltnu", "wrrpoc", "khuopg", "ooxarg", "vcvfry", "thaawc", "bssybb", "ajcwbj", "arwfnl",
                "nafmtm", "xoaumd", "vbejda", "kaefne", "swcrkh", "reeyhj", "vmcwaf", "chxitv", "qkwjna", "vklpkp",
                "xfnayl", "ccoyyo", "eywinm", "ktgmfn", "xrmzzm", "fgtuki", "zcffuv", "srxuus", "pydgmq"
            ],
            "ccoyyo".to_string(),
            10,
        );
        check(
            vec_str!["acckzz", "ccbazz", "eiowzz", "abcczz"],
            "acckzz".to_string(),
            10,
        );
        check(vec_str!["hamada", "khaled"], "hamada".to_string(), 10);
    }
}
