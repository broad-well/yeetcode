struct Solution;
impl Solution {
    // enhancement: prefix sums in place
    pub fn distance_between_bus_stops(mut distance: Vec<i32>, start: i32, destination: i32) -> i32 {
        if start == destination { return 0; }
        
        let mut i = 1;
        while i < distance.len() {
            distance[i] += distance[i - 1];
            i += 1;
        }

        let sum_dist = *distance.last().unwrap() as i32;
        let distance_index = |i: i32| if i == 0 { 0 } else { distance[i as usize - 1] };
        let clockwise_dist: i32 = (distance_index(destination) - distance_index(start)).abs();
        clockwise_dist.min(sum_dist - clockwise_dist)
    }

    #[allow(dead_code, unused)]
    pub fn distance_between_bus_stops_old(distance: Vec<i32>, start: i32, destination: i32) -> i32 {
        if start == destination { return 0; }
        let sum_dist: i32 = distance.iter().sum();
        let ibegin = start.min(destination) as usize;
        let iend = start.max(destination) as usize;
        let clockwise_dist: i32 = distance[ibegin..iend].iter().sum();
        clockwise_dist.min(sum_dist - clockwise_dist)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples() {
        assert_eq!(Solution::distance_between_bus_stops(vec![1, 2, 3, 4], 0, 1), 1);
        assert_eq!(Solution::distance_between_bus_stops(vec![1, 2, 3, 4], 0, 2), 3);
        assert_eq!(Solution::distance_between_bus_stops(vec![1, 2, 3, 4], 3, 0), 4);
        assert_eq!(Solution::distance_between_bus_stops(vec![1, 2, 3, 4], 1, 2), 2);
        assert_eq!(Solution::distance_between_bus_stops(vec![1, 2, 3, 4], 3, 1), 5);
        
        assert_eq!(Solution::distance_between_bus_stops(vec![1], 0, 0), 0);
        assert_eq!(Solution::distance_between_bus_stops(vec![9, 2], 0, 1), 2);
        assert_eq!(Solution::distance_between_bus_stops(vec![2, 2, 2, 2, 2], 0, 4), 2);
        assert_eq!(Solution::distance_between_bus_stops(vec![0, 0, 0], 2, 1), 0);
    }
}