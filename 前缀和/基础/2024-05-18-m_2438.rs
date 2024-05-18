struct Solution;

impl Solution {
    
    // 由于Rust数值类型长度限制，所以分段求余数然后再相乘
    // 本题主要是考察二进制，powers中为n中二进制每一位，将每一位取出即可计算，
    // 可使用前缀和进行优化，考虑到需要进行类型转换太麻烦了就没做了
    // 个人认为本题难点在数值计算中的取余操作，由于有长度限制，不方便通过二进制位数一次操作完成

    pub fn product_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        const MOD:i64 = 1_000_000_007;
        let mut powers = vec![];
        let mut n = n;
        while n != 0 {
            let lowbit = n & (-n);
            powers.push(lowbit);
            n ^= lowbit;
        }
        let len = powers.len() as i32;
        let mut answer = vec![];
        for q in queries {
            let l = q[0].min(len - 1);
            let r = q[1].min(len - 1);
            let mut mul:i64 = 1;
            for i in l..=r {
                mul *= powers[i as usize] as i64;
                mul %= MOD;
            }
            answer.push(mul as i32);
        }
        answer
    }
}