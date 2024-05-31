struct Solution;

impl Solution {

    // 题解里看到的解法
    // 字符串处理方法1
    // pub fn roman_to_int(s: String) -> i32 {
    //     s.chars().fold((0, ' '), |res, ch| {
    //         match (res.1, ch) {
    //             ('I', 'V') => (res.0 + 3, 'V'),
    //             ('I', 'X') => (res.0 + 8, 'X'),
    //             ('X', 'L') => (res.0 + 30, 'L'),
    //             ('X', 'C') => (res.0 + 80, 'C'),
    //             ('C', 'D') => (res.0 + 300, 'C'),
    //             ('C', 'M') => (res.0 + 800, 'M'),
    //             (_, 'I') => (res.0 + 1, 'I'),
    //             (_, 'V') => (res.0 + 5, 'V'),
    //             (_, 'X') => (res.0 + 10, 'X'),
    //             (_, 'L') => (res.0 + 50, 'L'),
    //             (_, 'C') => (res.0 + 100, 'C'),
    //             (_, 'D') => (res.0 + 500, 'D'),
    //             (_, 'M') => (res.0 + 1000, 'M'),
    //             (_, _) => unreachable!(),
    //         }
    //     }).0
    // }

    // 字符串处理方法2
    // 貌似按bytes切片更快，使用反转字符串简化了判断过程
    pub fn roman_to_int(s: String) -> i32 {
        s.bytes().rev().fold((0, 0), |res, cur| {
            let n = match cur {
                b'I' => 1,
                b'V' => 5,
                b'X' => 10,
                b'L' => 50,
                b'C' => 100,
                b'D' => 500,
                b'M' => 1000,
                _ => 0,
            };
            (if n < res.1 {res.0 - n} else {res.0 + n}, n)
        }).0
    }
}