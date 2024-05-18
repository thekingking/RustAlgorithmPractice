struct Solution;

impl Solution {
    // 题解里看见的，代码挺简洁的，思路一致，先排序，构建前缀和，然后检索，使用了Rust的库进行检索
    pub fn answer_queries1(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut queries = queries;
        nums.sort_unstable();
        nums.iter_mut().fold(0,|s,x|{*x+=s;*x});
        queries.iter_mut().for_each(|x|*x=nums.partition_point(|y|y<=x) as i32);
        queries
    }

    // 排序，循环检索，官解为二分检索简化过程，我没有构建前缀和是因为循环检索没必要构建了
    pub fn answer_queries2(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort();

        let mut answer = vec![0; queries.len()];

        for i in 0..queries.len() {
            let mut sum = 0;
            let mut index = 0;
            while index < nums.len() {
                if sum + nums[index] > queries[i] {
                    break;
                }
                sum += nums[index];
                index += 1;
            }
            answer[i] = index as i32;
        }
        answer
    }

    // 模仿今天的每日一题，将queries进行排序，由低到高，减少检索子序列数的过程
    // 同样在这里也没必要构建前缀和
    pub fn answer_queries3(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort();

        let ids: Vec<usize> = (0..).take(queries.len()).collect();
        let mut query:Vec<(i32, usize)> = queries.into_iter().zip(ids).collect();
        query.sort_by_key(|x| x.0);

        let mut answer = vec![0; query.len()];
        let mut index = 0;
        let mut sum = 0;

        for q in query {
            while index < nums.len() && sum <= q.0 {
                sum += nums[index];
                index += 1;
            }
            if index == nums.len() && sum <= q.0 {
                answer[q.1] = nums.len() as i32;
            } else {
                answer[q.1] = index as i32 - 1;
            }
        }
        answer
    }
}