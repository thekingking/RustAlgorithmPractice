struct Solution;

impl Solution {
    
    // 又一个双百
    // 迭代器访问数据比通过下标访问更快
    // 这个题应该是简单题才对
    pub fn get_winner(arr: Vec<i32>, k: i32) -> i32 {
        let mut max = arr[0];
        let mut num = 0;
        for &i in &arr[1..] {
            if max < i {
                num = 0;
                max = i;
            }
            num += 1;
            if num == k {
                break;
            }
        }
        max
    }
}