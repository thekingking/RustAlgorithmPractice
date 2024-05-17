

fn main() {
    println!("{}", false as i32);
}

struct Solution;

impl Solution {
    
    pub fn answer_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut queries = queries;
        nums.sort_unstable();
        nums.iter_mut().fold(0,|s,x|{*x+=s;*x});
        queries.iter_mut().for_each(|x|*x=nums.partition_point(|y|y<=x) as i32);
        queries
    }
}