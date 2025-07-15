// https://leetcode.com/problems/alternating-groups-iii/
// 3245. Alternating Groups III
pub struct Solution;
impl Solution {
    fn prevrange<T>(mrange: &std::collections::BTreeMap<usize, usize>, rng: T) -> (&usize, &usize)
    where
        T: std::ops::RangeBounds<usize>,
    {
        mrange
            .range(rng)
            .rev()
            .next()
            .unwrap_or_else(|| mrange.last_key_value().unwrap())
    }
    fn nextrange<T>(mrange: &std::collections::BTreeMap<usize, usize>, rng: T) -> (&usize, &usize)
    where
        T: std::ops::RangeBounds<usize>,
    {
        mrange
            .range(rng)
            .next()
            .unwrap_or_else(|| mrange.first_key_value().unwrap())
    }
    pub fn number_of_alternating_groups(colors: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut colors = colors;
        let mut mlen = std::collections::BTreeMap::<usize, i32>::new();
        let mut mrange = std::collections::BTreeMap::<usize, usize>::new();
        let mut full = false;
        let mut start = 0;
        let len = colors.len();
        if colors[len - 1] != colors[0] {
            start = len - 1;
            while start > 0 && colors[start - 1] != colors[start] {
                start -= 1;
            }
            if start == 0 {
                full = true;
            }
        }
        if !full {
            let mut s = start;
            let mut e = 0;
            loop {
                while e + 1 < len && colors[e + 1] != colors[e] {
                    e += 1;
                }
                let l = (e + len - s) % len + 1;
                mrange.insert(s, l);
                *mlen.entry(l).or_default() += 1;
                s = e + 1;
                if s >= len || s == start {
                    break;
                }
                e = s;
            }
        }
        let mut res = vec![];
        let modlen = |mlen: &mut std::collections::BTreeMap<usize, i32>, l: usize, m: i32| {
            if let Some(c) = mlen.get_mut(&l) {
                *c += m;
                if *c == 0 {
                    mlen.remove(&l);
                }
            } else {
                mlen.insert(l, m);
            }
        };
        for q in queries {
            if q[0] == 1 {
                if full {
                    res.push(len as i32);
                } else {
                    let mut cnt = 0;
                    for (&l, &c) in mlen.range(q[1] as usize..) {
                        cnt += (l as i32 - q[1] + 1) * c;
                    }
                    res.push(cnt);
                }
            } else {
                let pos = q[1] as usize;
                let nc = q[2];
                if colors[pos] == nc {
                    continue;
                }
                colors[pos] = nc;
                if full {
                    mrange.insert(pos, 1);
                    mlen.insert(1, 1);
                    mrange.insert((pos + 1) % len, len - 1);
                    mlen.insert(len - 1, 1);
                    full = false;
                } else {
                    let (&cpos, &clen) = Self::prevrange(&mrange, ..=pos);
                    if clen == 1 {
                        if mrange.len() == 2 {
                            full = true;
                            mrange.clear();
                            mlen.clear();
                        } else {
                            let (&ppos, &plen) = Self::prevrange(&mrange, ..cpos);
                            let (&npos, &nlen) = Self::nextrange(&mrange, cpos + 1..);
                            modlen(&mut mlen, plen, -1);
                            modlen(&mut mlen, nlen, -1);
                            modlen(&mut mlen, 1, -1);
                            modlen(&mut mlen, plen + nlen + 1, 1);
                            mrange.insert(ppos, plen + nlen + 1);
                            mrange.remove(&cpos);
                            mrange.remove(&npos);
                        }
                    } else if cpos != pos && pos != (cpos + clen - 1) % len {
                        modlen(&mut mlen, clen, -1);
                        modlen(&mut mlen, 1, 1);
                        modlen(&mut mlen, (pos + len - cpos) % len, 1);
                        modlen(&mut mlen, clen - 1 - (pos + len - cpos) % len, 1);
                        mrange.insert(cpos, (pos + len - cpos) % len);
                        mrange.insert(pos, 1);
                        mrange.insert((pos + 1) % len, clen - 1 - (pos + len - cpos) % len);
                    } else if cpos == pos {
                        if mrange.len() == 1 {
                            mrange.clear();
                            mrange.insert((pos + 1) % len, len);
                        } else {
                            let (&ppos, &plen) = Self::prevrange(&mrange, ..cpos);
                            modlen(&mut mlen, plen, -1);
                            modlen(&mut mlen, plen + 1, 1);
                            modlen(&mut mlen, clen, -1);
                            modlen(&mut mlen, clen - 1, 1);
                            mrange.insert(ppos, plen + 1);
                            mrange.remove(&cpos);
                            mrange.insert((pos + 1) % len, clen - 1);
                        }
                    } else {
                        if mrange.len() == 1 {
                            mrange.clear();
                            mrange.insert(pos, len);
                        } else {
                            let (&npos, &nlen) = Self::nextrange(&mrange, cpos + 1..);
                            modlen(&mut mlen, nlen, -1);
                            modlen(&mut mlen, nlen + 1, 1);
                            modlen(&mut mlen, clen, -1);
                            modlen(&mut mlen, clen - 1, 1);
                            mrange.insert(pos, nlen + 1);
                            mrange.insert(cpos, clen - 1);
                            mrange.remove(&npos);
                        }
                    }
                }
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn number_of_alternating_groups() {
        assert_eq!(
            Solution::number_of_alternating_groups(
                vec![1, 0, 0, 1],
                vec_vec![[2, 2, 0], [2, 2, 1], [2, 2, 0], [2, 3, 0]]
            ),
            []
        );
        assert_eq!(
            Solution::number_of_alternating_groups(vec![0, 1, 0, 1], vec_vec![[1, 3], [2, 2, 1], [1, 3], [1, 3]]),
            [4, 1, 1]
        );
        assert_eq!(
            Solution::number_of_alternating_groups(vec![0, 1, 1, 0, 1], vec_vec![[2, 1, 0], [1, 4]]),
            [2]
        );
        assert_eq!(
            Solution::number_of_alternating_groups(vec![0, 0, 1, 0, 1, 1], vec_vec![[1, 3], [2, 3, 0], [1, 5]]),
            [2, 0]
        );
    }
}
