struct Solution;

impl Solution {
  pub fn distance_between_bus_stops(distance: Vec<i32>, start: i32, destination: i32) -> i32 {
    let (start, destination) = (start.min(destination), start.max(destination));
    let mut a = 0;
    let mut b = 0;
    for i in 0..distance.len() {
      if i as i32 >= start && (i as i32) < destination {
        a += distance[i];
      } else {
        b += distance[i];
      }
    }
    a.min(b)
  }
}