//! 使用i64实现分数排序
//! 还没理清实现思路

#[derive(Eq, PartialEq, Debug)]
struct Pair(i64, i64);

impl PartialOrd for Pair {
    /// 降序排序
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(
            ((self.1 - self.0) * (other.1 + 1) * other.1)
                .cmp(&((other.1 - other.0) * (self.1 + 1) * self.1)),
        )
    }
}


impl Ord for Pair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
