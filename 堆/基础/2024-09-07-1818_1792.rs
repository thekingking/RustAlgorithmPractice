#[derive(Eq, PartialEq, Debug)]
struct Pair(i64, i64);

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(
            ((self.1 - self.0) * (other.1 + 1) * other.1)
                .cmp(&((other.1 - other.0) * (self.1 + 1) * self.1)),
        )
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

struct Solution;

impl Solution {
    pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
        use std::collections::BinaryHeap;

        let n = classes.len() as f64;
        let mut heap = BinaryHeap::new();
        for class in classes {
            heap.push(Pair(class[0] as i64, class[1] as i64));
        }
        for _ in 0..extra_students {
            let Pair(pass, total) = heap.pop().unwrap();
            heap.push(Pair(pass + 1, total + 1));
        }
        heap.into_iter().map(|pair| pair.0 as f64 / pair.1 as f64).sum::<f64>() / n
    }
}