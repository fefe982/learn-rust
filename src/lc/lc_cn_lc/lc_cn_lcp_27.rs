// https://leetcode.cn/problems/IQvJ9i/
// LCP 27. 黑盒光线反射
pub struct BlackBox {
    gp: Vec<(usize, usize)>,
    gn: Vec<(usize, usize)>,
    open: Vec<std::collections::BTreeMap<usize, i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BlackBox {
    pub fn new(n: i32, m: i32) -> Self {
        let n = n as usize;
        let m = m as usize;
        let c = (n + m) * 2;
        let mut gp = vec![(usize::MAX, usize::MAX); c];
        let mut gn = vec![(usize::MAX, usize::MAX); c];
        let mut cg = 0;
        let mut make_group = |i: usize, dir: i32| {
            if (dir == 1 && (i == 0 || i == m + n || gp[i].0 != usize::MAX))
                || (dir == -1 && (i == m || i == m * 2 + n || gn[i].0 != usize::MAX))
            {
                return;
            }
            let ig = cg;
            cg += 1;
            let mut dir = dir;
            let mut iw = 0;
            let mut i = i;
            while !((dir == 1 && gp[i].0 != usize::MAX) || (dir == -1 && gn[i].0 != usize::MAX)) {
                if dir == 1 {
                    gp[i] = (ig, iw);
                    i = c - i;
                } else {
                    gn[i] = (ig, iw);
                    if i <= m * 2 {
                        i = m * 2 - i;
                    } else {
                        i = (c + m * 2) - i;
                    }
                }
                if i != 0 && i != m && i != m + n && i != m * 2 + n {
                    dir *= -1;
                }
                iw += 1;
            }
        };
        for i in 0..c {
            make_group(i, 1);
            make_group(i, -1);
        }
        Self {
            gp,
            gn,
            open: vec![std::collections::BTreeMap::new(); cg],
        }
    }

    pub fn open(&mut self, index: i32, direction: i32) -> i32 {
        let index = index as usize;
        if self.gp[index].0 != usize::MAX {
            self.open[self.gp[index].0].insert(self.gp[index].1, index as i32);
        }
        if self.gn[index].0 != usize::MAX {
            self.open[self.gn[index].0].insert(self.gn[index].1, index as i32);
        }
        let (ig, iw) = if direction == 1 { self.gp[index] } else { self.gn[index] };
        let g = &mut self.open[ig];
        g.insert(iw, index as i32);
        if let Some(v) = g.range(iw + 1..).next() {
            *v.1
        } else {
            *g.iter().next().unwrap().1
        }
    }

    pub fn close(&mut self, index: i32) {
        let index = index as usize;
        if self.gp[index].0 != usize::MAX {
            self.open[self.gp[index].0].remove(&self.gp[index].1);
        }
        if self.gn[index].0 != usize::MAX {
            self.open[self.gn[index].0].remove(&self.gn[index].1);
        }
    }
}

/**
 * Your BlackBox object will be instantiated and called as such:
 * let obj = BlackBox::new(n, m);
 * let ret_1: i32 = obj.open(index, direction);
 * obj.close(index);
 */
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test() {
        let null = 0;
        for case in [
            (
                vec!["BlackBox", "open", "open", "open", "close", "open"],
                vec_vec![[2, 3], [6, -1], [4, -1], [0, -1], [6], [0, -1]],
                vec![null, 6, 4, 6, null, 4],
            ),
            (
                vec![
                    "BlackBox", "open", "open", "open", "open", "close", "open", "close", "open",
                ],
                vec_vec![[3, 3], [1, -1], [5, 1], [11, -1], [11, 1], [1], [11, 1], [5], [11, -1]],
                vec![null, 1, 1, 5, 1, null, 5, null, 11],
            ),
        ] {
            let mut blackbox = BlackBox::new(case.1[0][0], case.1[0][1]);
            for ((cmd, args), ret) in case
                .0
                .into_iter()
                .zip(case.1.into_iter())
                .zip(case.2.into_iter())
                .skip(1)
            {
                match cmd {
                    "open" => assert_eq!(blackbox.open(args[0], args[1]), ret),
                    "close" => blackbox.close(args[0]),
                    _ => unreachable!(),
                }
            }
        }
    }
}
