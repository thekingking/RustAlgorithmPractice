struct BookMyShow {
    n: usize,
    m: i32,
    min: Vec<i32>,
    sum: Vec<i64>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BookMyShow {
    // 把下标 i 上的元素值增加 val
    fn update(&mut self, o: usize, l: usize, r: usize, i: usize, val: i32) {
        if l == r {
            self.min[o] += val;
            self.sum[o] += val as i64;
            return;
        }
        let m = (l + r) / 2;
        if i <= m {
            self.update(o * 2, l, m, i, val);
        } else {
            self.update(o * 2 + 1, m + 1, r, i, val);
        }
        self.min[o] = self.min[o * 2].min(self.min[o * 2 + 1]);
        self.sum[o] = self.sum[o * 2] + self.sum[o * 2 + 1];
    }

    // 返回区间 [L,R] 内的元素和
    fn query_sum(&self, o: usize, l: usize, r: usize, L: usize, R: usize) -> i64 {
        if L <= l && r <= R {
            return self.sum[o];
        }
        let mut res = 0;
        let m = (l + r) / 2;
        if L <= m {
            res = self.query_sum(o * 2, l, m, L, R);
        }
        if R > m {
            res += self.query_sum(o * 2 + 1, m + 1, r, L, R);
        }
        res
    }

     // 返回区间 [0,R] 中 <= val 的最靠左的位置，不存在时返回 -1
     fn find_first(&self, o: usize, l: usize, r: usize, R: usize, val: i32) -> i32 {
        if self.min[o] > val {
            return -1; // 整个区间的元素值都大于 val
        }
        if l == r {
            return l as i32;
        }
        let m = (l + r) / 2;
        if self.min[o * 2] <= val {
            return self.find_first(o * 2, l, m, R, val);
        }
        if R > m {
            return self.find_first(o * 2 + 1, m + 1, r, R, val);
        }
        -1
    }

    fn new(n: i32, m: i32) -> Self {
        let size = 2 << (32 - n.leading_zeros()) as usize;
        Self {
            n: n as usize,
            m,
            min: vec![0; size],
            sum: vec![0; size],
        }
    }
    
    fn gather(&mut self, k: i32, max_row: i32) -> Vec<i32> {
        // 找第一个能倒入 k 升水的水桶
        let r = self.find_first(1, 0, self.n - 1, max_row as usize, self.m - k);
        if r < 0 {
            return vec![]; // 没有这样的水桶
        }
        let c = self.query_sum(1, 0, self.n - 1, r as usize, r as usize) as i32;
        self.update(1, 0, self.n - 1, r as usize, k); // 倒水
        vec![r, c]
    }

    fn scatter(&mut self, mut k: i32, max_row: i32) -> bool {
        // [0,maxRow] 的接水量之和
        let s = self.query_sum(1, 0, self.n - 1, 0, max_row as usize);
        if s > (self.m as i64 * (max_row + 1) as i64) - k as i64 {
            return false; // 水桶已经装了太多的水
        }
        // 从第一个没有装满的水桶开始
        let mut i = self.find_first(1, 0, self.n - 1, max_row as usize, self.m - 1) as usize;
        while k > 0 {
            let left = k.min(self.m - self.query_sum(1, 0, self.n - 1, i, i) as i32);
            self.update(1, 0, self.n - 1, i, left); // 倒水
            k -= left;
            i += 1;
        }
        true
    }
}