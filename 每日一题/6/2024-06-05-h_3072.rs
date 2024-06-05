struct Solution;

impl Solution {
    /// 这个hard怎么这么假
    /// 模拟
    pub fn result_array(nums: Vec<i32>) -> Vec<i32> {
      let mut arr1 = Vec::new();
      let mut arr2 = Vec::new();
      arr1.push(nums[0]);
      arr2.push(nums[1]);
      for &x in &nums[2..] {
        let n1 = Self::greaterCount(&arr1, x);
        let n2 = Self::greaterCount(&arr2, x);
        if n1 > n2 {
          arr1.push(x);
        } else if n1 < n2 {
          arr2.push(x);
        } else if arr2.len() < arr1.len() {
          arr2.push(x);
        } else {
          arr1.push(x);
        }
      }
      arr1.extend(arr2);
      arr1
    }

    fn greaterCount(arr: &Vec<i32>, val: i32) -> i32 {
        let mut num = 0;
        for &x in arr {
          if x > val {
            num += 1;
          }
        }
        num
    }
}