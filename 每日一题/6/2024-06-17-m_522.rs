struct Solution;

impl Solution {
    /// 给我写烦了，对子序列判断写错了，耽误太久时间了
    pub fn find_lu_slength(mut strs: Vec<String>) -> i32 {
        strs.sort_unstable_by(|a, b| b.len().cmp(&a.len()));
        'next:
        for (i, s) in strs.iter().enumerate() {
            let s = s.as_bytes();
            for (j, t) in strs.iter().enumerate() {
                if j != i && Self::is_subseq(s, t) {
                    continue 'next;
                }
            }
            return s.len() as _;
        }
        -1
    }
    /// 返回 s 是否为 t 的子序列
    fn is_subseq(s: &[u8], t: &str) -> bool {
        let mut i = 0;
        for c in t.bytes() {
            if s[i] == c {
                i += 1;
                if i == s.len() { // 所有字符匹配完毕
                    return true; // s 是 t 的子序列
                }
            }
        }
        false
    }
}