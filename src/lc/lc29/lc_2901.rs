// https://leetcode.com/problems/longest-unequal-adjacent-groups-subsequence-ii/
// 2901. Longest Unequal Adjacent Groups Subsequence II
pub struct Solution;
impl Solution {
    pub fn get_words_in_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        let mut from = vec![usize::MAX; words.len()];
        let mut max_len = 0;
        let mut max_len_end = 0;
        let mask = 0b11111;
        let mut end = std::collections::HashMap::<u64, (i32, usize)>::new();
        for (i, word) in words.iter().enumerate() {
            let w = word.as_bytes();
            let wl = w.len();
            let mut h = 0u64;
            for (j, &c) in w.iter().enumerate() {
                h |= (c as u64 & mask) << (5 * j);
            }
            let mut this_max = 0;
            for il in 0..wl {
                let wh = h | (mask << (5 * il));
                let &(len, j) = end.get(&wh).unwrap_or(&(0, usize::MAX));
                let g = if j == usize::MAX { 0 } else { groups[j] };
                if groups[i] != g {
                    if len + 1 > this_max {
                        this_max = len + 1;
                        from[i] = j;
                    }
                }
            }
            for il in 0..wl {
                let wh = h | (mask << (5 * il));
                let &(len, _) = end.get(&wh).unwrap_or(&(0, usize::MAX));
                if this_max > len {
                    end.insert(wh, (this_max, i));
                }
            }
            if this_max > max_len {
                max_len = this_max;
                max_len_end = i;
            }
        }
        let mut stk = Vec::with_capacity(max_len as usize);
        while max_len_end != usize::MAX {
            stk.push(max_len_end);
            max_len_end = from[max_len_end];
        }
        let mut ans = Vec::with_capacity(max_len as usize);
        for &i in stk.iter().rev() {
            ans.push(words[i].clone());
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    fn check(input: Vec<String>, groups: Vec<i32>, expect_len: usize) {
        let ans = Solution::get_words_in_longest_subsequence(input.clone(), groups.clone());
        assert_eq!(ans.len(), expect_len);
        let mut i = 0;
        let mut j = 0;
        let mut g = 0;
        while i < input.len() && j < ans.len() {
            if input[i] == ans[j] {
                assert_ne!(g, groups[i]);
                g = groups[i];
                i += 1;
                j += 1;
            } else {
                i += 1;
            }
        }
        assert_eq!(j, ans.len());
    }
    #[test]
    fn get_words_in_longest_subsequence() {
        check(
            vec_str![
                "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t",
                "u", "v", "w", "x", "y", "z", "aa", "ba", "ca", "da", "ea", "fa", "ga", "ha", "ia", "ja", "ka", "la",
                "ma", "na", "oa", "pa", "qa", "ra", "sa", "ta", "ua", "va", "wa", "xa", "ya", "za", "ab", "bb", "cb",
                "db", "eb", "fb", "gb", "hb", "ib", "jb", "kb", "lb", "mb", "nb", "ob", "pb", "qb", "rb", "sb", "tb",
                "ub", "vb", "wb", "xb", "yb", "zb", "ac", "bc", "cc", "dc", "ec", "fc", "gc", "hc", "ic", "jc", "kc",
                "lc", "mc", "nc", "oc", "pc", "qc", "rc", "sc", "tc", "uc", "vc", "wc", "xc", "yc", "zc", "ad", "bd",
                "cd", "dd", "ed", "fd", "gd", "hd", "id", "jd", "kd", "ld", "md", "nd", "od", "pd", "qd", "rd", "sd",
                "td", "ud", "vd", "wd", "xd", "yd", "zd", "ae", "be", "ce", "de", "ee", "fe", "ge", "he", "ie", "je",
                "ke", "le", "me", "ne", "oe", "pe", "qe", "re", "se", "te", "ue", "ve", "we", "xe", "ye", "ze", "af",
                "bf", "cf", "df", "ef", "ff", "gf", "hf", "if", "jf", "kf", "lf", "mf", "nf", "of", "pf", "qf", "rf",
                "sf", "tf", "uf", "vf", "wf", "xf", "yf", "zf", "ag", "bg", "cg", "dg", "eg", "fg", "gg", "hg", "ig",
                "jg", "kg", "lg", "mg", "ng", "og", "pg", "qg", "rg", "sg", "tg", "ug", "vg", "wg", "xg", "yg", "zg",
                "ah", "bh", "ch", "dh", "eh", "fh", "gh", "hh", "ih", "jh", "kh", "lh", "mh", "nh", "oh", "ph", "qh",
                "rh", "sh", "th", "uh", "vh", "wh", "xh", "yh", "zh", "ai", "bi", "ci", "di", "ei", "fi", "gi", "hi",
                "ii", "ji", "ki", "li", "mi", "ni", "oi", "pi", "qi", "ri", "si", "ti", "ui", "vi", "wi", "xi", "yi",
                "zi", "aj", "bj", "cj", "dj", "ej", "fj", "gj", "hj", "ij", "jj", "kj", "lj", "mj", "nj", "oj", "pj",
                "qj", "rj", "sj", "tj", "uj", "vj", "wj", "xj", "yj", "zj", "ak", "bk", "ck", "dk", "ek", "fk", "gk",
                "hk", "ik", "jk", "kk", "lk", "mk", "nk", "ok", "pk", "qk", "rk", "sk", "tk", "uk", "vk", "wk", "xk",
                "yk", "zk", "al", "bl", "cl", "dl", "el", "fl", "gl", "hl", "il", "jl", "kl", "ll", "ml", "nl", "ol",
                "pl", "ql", "rl", "sl", "tl", "ul", "vl", "wl", "xl", "yl", "zl", "am", "bm", "cm", "dm", "em", "fm",
                "gm", "hm", "im", "jm", "km", "lm", "mm", "nm", "om", "pm", "qm", "rm", "sm", "tm", "um", "vm", "wm",
                "xm", "ym", "zm", "an", "bn", "cn", "dn", "en", "fn", "gn", "hn", "in", "jn", "kn", "ln", "mn", "nn",
                "on", "pn", "qn", "rn", "sn", "tn", "un", "vn", "wn", "xn", "yn", "zn", "ao", "bo", "co", "do", "eo",
                "fo", "go", "ho", "io", "jo", "ko", "lo", "mo", "no", "oo", "po", "qo", "ro", "so", "to", "uo", "vo",
                "wo", "xo", "yo", "zo", "ap", "bp", "cp", "dp", "ep", "fp", "gp", "hp", "ip", "jp", "kp", "lp", "mp",
                "np", "op", "pp", "qp", "rp", "sp", "tp", "up", "vp", "wp", "xp", "yp", "zp", "aq", "bq", "cq", "dq",
                "eq", "fq", "gq", "hq", "iq", "jq", "kq", "lq", "mq", "nq", "oq", "pq", "qq", "rq", "sq", "tq", "uq",
                "vq", "wq", "xq", "yq", "zq", "ar", "br", "cr", "dr", "er", "fr", "gr", "hr", "ir", "jr", "kr", "lr",
                "mr", "nr", "or", "pr", "qr", "rr", "sr", "tr", "ur", "vr", "wr", "xr", "yr", "zr", "as", "bs", "cs",
                "ds", "es", "fs", "gs", "hs", "is", "js", "ks", "ls", "ms", "ns", "os", "ps", "qs", "rs", "ss", "ts",
                "us", "vs", "ws", "xs", "ys", "zs", "at", "bt", "ct", "dt", "et", "ft", "gt", "ht", "it", "jt", "kt",
                "lt", "mt", "nt", "ot", "pt", "qt", "rt", "st", "tt", "ut", "vt", "wt", "xt", "yt", "zt", "au", "bu",
                "cu", "du", "eu", "fu", "gu", "hu", "iu", "ju", "ku", "lu", "mu", "nu", "ou", "pu", "qu", "ru", "su",
                "tu", "uu", "vu", "wu", "xu", "yu", "zu", "av", "bv", "cv", "dv", "ev", "fv", "gv", "hv", "iv", "jv",
                "kv", "lv", "mv", "nv", "ov", "pv", "qv", "rv", "sv", "tv", "uv", "vv", "wv", "xv", "yv", "zv", "aw",
                "bw", "cw", "dw", "ew", "fw", "gw", "hw", "iw", "jw", "kw", "lw", "mw", "nw", "ow", "pw", "qw", "rw",
                "sw", "tw", "uw", "vw", "ww", "xw", "yw", "zw", "ax", "bx", "cx", "dx", "ex", "fx", "gx", "hx", "ix",
                "jx", "kx", "lx", "mx", "nx", "ox", "px", "qx", "rx", "sx", "tx", "ux", "vx", "wx", "xx", "yx", "zx",
                "ay", "by", "cy", "dy", "ey", "fy", "gy", "hy", "iy", "jy", "ky", "ly", "my", "ny", "oy", "py", "qy",
                "ry", "sy", "ty", "uy", "vy", "wy", "xy", "yy", "zy", "az", "bz", "cz", "dz", "ez", "fz", "gz", "hz",
                "iz", "jz", "kz", "lz", "mz", "nz", "oz", "pz", "qz", "rz", "sz", "tz", "uz", "vz", "wz", "xz", "yz",
                "zz", "aaa", "baa", "caa", "daa", "eaa", "faa", "gaa", "haa", "iaa", "jaa", "kaa", "laa", "maa", "naa",
                "oaa", "paa", "qaa", "raa", "saa", "taa", "uaa", "vaa", "waa", "xaa", "yaa", "zaa", "aba", "bba",
                "cba", "dba", "eba", "fba", "gba", "hba", "iba", "jba", "kba", "lba", "mba", "nba", "oba", "pba",
                "qba", "rba", "sba", "tba", "uba", "vba", "wba", "xba", "yba", "zba", "aca", "bca", "cca", "dca",
                "eca", "fca", "gca", "hca", "ica", "jca", "kca", "lca", "mca", "nca", "oca", "pca", "qca", "rca",
                "sca", "tca", "uca", "vca", "wca", "xca", "yca", "zca", "ada", "bda", "cda", "dda", "eda", "fda",
                "gda", "hda", "ida", "jda", "kda", "lda", "mda", "nda", "oda", "pda", "qda", "rda", "sda", "tda",
                "uda", "vda", "wda", "xda", "yda", "zda", "aea", "bea", "cea", "dea", "eea", "fea", "gea", "hea",
                "iea", "jea", "kea", "lea", "mea", "nea", "oea", "pea", "qea", "rea", "sea", "tea", "uea", "vea",
                "wea", "xea", "yea", "zea", "afa", "bfa", "cfa", "dfa", "efa", "ffa", "gfa", "hfa", "ifa", "jfa",
                "kfa", "lfa", "mfa", "nfa", "ofa", "pfa", "qfa", "rfa", "sfa", "tfa", "ufa", "vfa", "wfa", "xfa",
                "yfa", "zfa", "aga", "bga", "cga", "dga", "ega", "fga", "gga", "hga", "iga", "jga", "kga", "lga",
                "mga", "nga", "oga", "pga", "qga", "rga", "sga", "tga", "uga", "vga", "wga", "xga", "yga", "zga",
                "aha", "bha", "cha", "dha", "eha", "fha", "gha", "hha", "iha", "jha", "kha", "lha", "mha", "nha",
                "oha", "pha", "qha", "rha", "sha", "tha", "uha", "vha", "wha", "xha", "yha", "zha", "aia", "bia",
                "cia", "dia", "eia", "fia", "gia", "hia", "iia", "jia", "kia", "lia", "mia", "nia", "oia", "pia",
                "qia", "ria", "sia", "tia", "uia", "via", "wia", "xia", "yia", "zia", "aja", "bja", "cja", "dja",
                "eja", "fja", "gja", "hja", "ija", "jja", "kja", "lja", "mja", "nja", "oja", "pja", "qja", "rja",
                "sja", "tja", "uja", "vja", "wja", "xja", "yja", "zja", "aka", "bka", "cka", "dka", "eka", "fka",
                "gka", "hka", "ika", "jka", "kka", "lka", "mka", "nka", "oka", "pka", "qka", "rka", "ska", "tka",
                "uka", "vka", "wka", "xka", "yka", "zka", "ala", "bla", "cla", "dla", "ela", "fla", "gla", "hla",
                "ila", "jla", "kla", "lla"
            ],
            vec![
                2, 1, 2, 2, 2, 3, 2, 2, 1, 1, 2, 1, 3, 1, 1, 3, 2, 2, 3, 2, 2, 1, 2, 1, 2, 3, 3, 2, 3, 2, 3, 3, 2, 1,
                2, 2, 2, 3, 1, 2, 1, 2, 1, 3, 2, 2, 1, 3, 3, 3, 2, 2, 3, 1, 1, 1, 3, 3, 3, 1, 3, 3, 2, 1, 3, 2, 1, 2,
                3, 3, 3, 1, 2, 3, 3, 2, 2, 3, 1, 2, 2, 1, 3, 2, 1, 1, 2, 1, 1, 3, 3, 1, 3, 3, 3, 1, 2, 3, 1, 3, 1, 3,
                3, 3, 3, 3, 2, 3, 1, 3, 1, 3, 2, 2, 2, 2, 1, 3, 1, 3, 2, 1, 2, 2, 3, 1, 3, 2, 2, 3, 1, 2, 3, 1, 3, 2,
                2, 1, 3, 3, 1, 2, 2, 3, 3, 1, 3, 2, 3, 1, 2, 3, 3, 3, 2, 2, 1, 1, 1, 1, 3, 1, 2, 1, 1, 1, 1, 1, 2, 2,
                2, 3, 2, 3, 2, 3, 2, 3, 2, 2, 1, 3, 3, 2, 1, 3, 2, 2, 3, 1, 1, 3, 1, 1, 1, 1, 1, 1, 3, 2, 3, 2, 1, 3,
                1, 1, 2, 2, 2, 1, 2, 3, 2, 3, 2, 1, 1, 3, 1, 2, 3, 1, 2, 1, 2, 3, 2, 1, 3, 2, 2, 3, 2, 1, 3, 2, 3, 3,
                2, 3, 3, 1, 2, 2, 3, 3, 2, 2, 3, 3, 1, 3, 2, 2, 1, 3, 3, 1, 3, 2, 3, 2, 2, 2, 1, 1, 3, 1, 1, 1, 1, 1,
                1, 2, 3, 1, 1, 3, 1, 3, 1, 3, 1, 1, 1, 3, 1, 1, 2, 2, 1, 3, 3, 2, 3, 3, 3, 2, 1, 1, 2, 1, 3, 1, 3, 2,
                1, 1, 2, 3, 2, 2, 3, 3, 2, 3, 2, 2, 2, 1, 3, 2, 2, 3, 3, 2, 2, 3, 2, 2, 3, 1, 3, 2, 3, 1, 3, 2, 1, 1,
                2, 1, 3, 2, 2, 3, 2, 1, 3, 3, 3, 3, 2, 2, 3, 1, 2, 1, 2, 1, 3, 3, 2, 3, 3, 3, 1, 1, 3, 2, 2, 2, 3, 3,
                1, 3, 2, 3, 1, 1, 1, 2, 1, 2, 1, 1, 3, 3, 1, 2, 1, 2, 2, 2, 2, 1, 1, 1, 2, 3, 1, 3, 1, 3, 2, 2, 3, 3,
                2, 1, 2, 2, 1, 2, 3, 3, 1, 3, 1, 1, 1, 3, 3, 2, 2, 1, 3, 2, 2, 1, 2, 3, 2, 3, 1, 1, 2, 3, 1, 3, 2, 2,
                2, 3, 1, 2, 2, 2, 1, 3, 1, 1, 3, 2, 3, 3, 1, 1, 2, 3, 1, 3, 3, 1, 3, 3, 3, 1, 3, 3, 2, 2, 1, 3, 2, 3,
                3, 2, 2, 3, 3, 1, 3, 2, 2, 2, 2, 3, 3, 2, 1, 3, 2, 2, 2, 2, 2, 1, 3, 2, 1, 1, 1, 3, 3, 2, 1, 3, 2, 2,
                1, 2, 1, 1, 3, 1, 2, 3, 3, 2, 3, 3, 2, 3, 2, 3, 2, 2, 2, 1, 1, 1, 1, 3, 3, 3, 3, 1, 2, 3, 2, 1, 2, 1,
                2, 3, 3, 1, 2, 2, 1, 2, 1, 2, 2, 3, 1, 2, 3, 3, 2, 2, 2, 2, 3, 1, 3, 1, 2, 1, 1, 3, 2, 2, 3, 3, 1, 2,
                3, 3, 2, 3, 3, 3, 1, 1, 1, 2, 2, 1, 2, 3, 1, 3, 3, 2, 3, 2, 3, 1, 1, 1, 2, 3, 2, 3, 1, 1, 1, 3, 3, 2,
                3, 2, 2, 2, 1, 3, 1, 3, 2, 1, 2, 1, 1, 2, 3, 1, 3, 3, 1, 3, 2, 1, 3, 2, 1, 1, 2, 1, 1, 3, 2, 1, 1, 2,
                2, 1, 3, 1, 2, 1, 3, 2, 3, 3, 2, 3, 2, 3, 1, 3, 1, 3, 3, 1, 3, 1, 3, 3, 2, 1, 2, 3, 3, 1, 1, 2, 2, 1,
                2, 3, 1, 2, 3, 3, 2, 1, 2, 2, 2, 1, 1, 3, 1, 1, 3, 2, 2, 2, 2, 1, 1, 2, 3, 1, 3, 3, 2, 1, 1, 3, 3, 3,
                2, 2, 2, 2, 2, 3, 1, 1, 3, 3, 3, 1, 1, 2, 2, 3, 2, 3, 2, 2, 1, 1, 1, 1, 3, 2, 2, 1, 2, 2, 3, 2, 3, 1,
                2, 3, 3, 2, 1, 3, 1, 2, 3, 2, 1, 3, 1, 3, 3, 1, 2, 2, 2, 2, 2, 1, 1, 1, 2, 1, 1, 3, 2, 2, 1, 1, 3, 3,
                1, 3, 3, 1, 2, 1, 3, 1, 1, 3, 2, 2, 3, 2, 3, 2, 1, 3, 3, 1, 1, 2, 2, 2, 2, 3, 3, 3, 1, 2, 1, 2, 1, 2,
                1, 1, 3, 3, 1, 3, 3, 2, 2, 1, 3, 1, 2, 2, 3, 1, 3, 2, 3, 2, 1, 1, 2, 3, 3, 2, 1, 1, 1, 3, 2, 2, 2, 1,
                3, 2, 1, 2, 2, 3, 2, 2, 1, 3, 1, 1, 3, 3, 2, 1, 2, 2, 3, 2, 2, 2, 2, 2, 1, 1, 3, 3, 3, 2, 3, 1, 2, 1,
                3, 3, 1, 2, 1, 2, 1, 1, 1, 2, 2, 3, 3, 3, 3, 3, 1, 1, 3, 3, 1, 2, 2, 3, 1, 3, 3, 2, 2, 1, 2, 3, 1, 3,
                3, 2, 2, 3, 1, 2, 3, 2, 1, 2, 3, 1, 1, 1, 2, 3, 3, 1, 2, 3, 2, 1, 1, 3, 3, 2, 3, 1, 2, 3, 2, 2, 3, 2,
                2, 2, 3, 2, 2, 3, 1, 1, 2, 2, 3, 2, 1, 2, 3, 2, 3, 2, 1, 3, 3, 1, 3, 2, 1, 1, 3, 3, 3, 1, 1, 2, 1, 3,
                3, 2, 3, 2, 2, 2, 3, 3, 3, 3, 3, 1, 1, 2,
            ],
            49,
        );
        check(vec_str!["ca", "cb", "bcd", "bb", "ddc"], vec![2, 4, 2, 5, 1], 3);
        check(vec_str!["baa", "ada"], vec![1, 2], 1);
        check(vec_str!["bab", "dab", "cab"], vec![1, 2, 2], 2);
        check(vec_str!["a", "b", "c", "d"], vec![1, 2, 3, 4], 4);
    }
}
