struct Solution;

impl Solution {

    // 看别人的思路写的，一开始写的时间复杂度高了，这题要把复杂度降到O(n)去
    // 根据官解的思路，本题是一个数学问题，确实不好想，反正我是没想到
    // 本题是前缀和的思路解题
    // 我的理解：首先用二进制代码表示0~9字符出现是奇数次还是偶数次，奇数为1，偶数为1，用二进制表示（这是本题的第一个关键点）
    // 把字符串中s[0:i]中不同字符出现的奇偶次数用二进制表示
    // 若子字符串s[i:j]为回文字符串，那么s[i:j]中最多有一个字符出现奇数次，其余皆出现偶数次
    // 对于字符串s[0:i-1]和字符串s[0:j]，可以看出两者之间相差一个回文串，即相差一个出现次数为奇数次的字符或者相同
    // 通过以上说明，我们可以得出方法，通过将字符串前缀和以哈希表存储，即当前前缀和prev，下标idx，map.insert(prev, idx)
    // 将s[0:i-1]存储，当我们求出当前s[0:j]的前缀和prev时，将这个前缀和减去与s[0:i-1]相差的字符，即可判断当前是否为回文串
    pub fn longest_awesome(s: String) -> i32 {
        // 回文串长度
        let mut ans = 1;
        // 以各个字符出现奇偶次数为键，前缀和下标为值，存储在hash表中
        let mut map = HashMap::new();
        // 初始化，由于数组下标从0开始，当在第一个时，向前追溯为-1
        map.insert(0, -1);
        // 前缀和
        let mut prev = 0;
        // s[0:j]与s[0:i-1]相差的字符
        let mut lib = [0; 11];
        for i in 0..10 {
            lib[i + 1] = 1 << i;
        }

        for (idx, c) in s.bytes().enumerate() {
            // 求当前前缀和
            prev ^= 1 << (c - 48);

            // 遍历测试s[0:j]和s[0:i-1]相差的字符
            for target in lib {
                let cur = target ^ prev;
                if map.get(&cur).is_some() {
                    ans = ans.max(idx as i32 - map.get(&cur).unwrap());
                }
            }
            if map.get(&prev).is_none() {
                map.insert(prev, idx as i32);
            }
        }
        ans as i32
    }
}