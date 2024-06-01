struct Solution;

impl Solution {
    // 我自己一开始想的，简单的遍历每个工人，求每个工人的最大收益，然后再求和
    pub fn max_profit_assignment1(difficulty: Vec<i32>, profit: Vec<i32>, worker: Vec<i32>) -> i32 {
        let mut res = 0;
        for i in 0..worker.len() {
            let mut max = 0;
            for j in 0..difficulty.len() {
                if difficulty[j] <= worker[i] && profit[j] > max {
                    max = profit[j];
                }
            }
            res += max;
        }
        res
    }
    
    // 官方题解，排序 + 双指针
    pub fn max_profit_assignment2(difficulty: Vec<i32>, profit: Vec<i32>, worker: Vec<i32>) -> i32 {
        // 关联difficult和profit，并将其排序
        let mut jobs:Vec<(i32, i32)> = difficulty.into_iter().zip(profit).collect();
        jobs.sort_by_key(|&x| x.0);

        // 将工人工作能力进行排序
        let mut worker = worker;
        worker.sort();

        let mut res = 0;
        let mut i = 0;
        let mut best = 0;
        // 双指针，工人工作能力由低到高进行排序，工作能力较低的工人能完成的任务工作能力较高的工人也能完成
        // 随着工人工作能力增加，能够完成的工作数量也随之增加，通过比较原本工作中的最大收益与新增工作的收益
        // 或得当前所有工作中的最大收益，减少了反复寻找最大收益的过程
        // 时间复杂度受排序影响
        for w in worker {
            while i < jobs.len() && w >= jobs[i].0 {
                best = std::cmp::max(best, jobs[i].1);
                i += 1;
            }
            res += best;
        }
        res
    }
}