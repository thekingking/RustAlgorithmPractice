struct NumArray {
    arr: Vec<i32>,
}

// 前缀和
impl NumArray {

    fn new(nums: Vec<i32>) -> Self {
        let mut arr = vec![0; nums.len() + 1];
        for (i, &x) in nums.iter().enumerate() {
            arr[i + 1] = arr[i] + x;
        }
        NumArray{ arr }
    }
    
    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.arr[(right + 1) as usize] - self.arr[left as usize]
    }
}