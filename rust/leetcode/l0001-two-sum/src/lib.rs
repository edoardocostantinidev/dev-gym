use std::collections::HashMap;

pub fn on2_two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (i, n) in nums.iter().enumerate() {
        for (j, m) in nums.iter().enumerate() {
            if i == j {
                continue;
            }
            if n + m == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    panic!("impossible")
}

//run: 77.44 %
//mem:  62.20 %
pub fn on_two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // build a map(value,index) with O(n)
    let mut map: HashMap<i32, i32> = HashMap::new();

    // search for target-n in map and retrieve indices, still O(n)
    for (i, n) in nums.iter().enumerate() {
        if let Some(&j) = map.get(&(target - n)) {
            return vec![i as i32, j];
        } else {
            map.insert(*n, i as i32);
        }
    }
    vec![]
}

pub fn less_mem_two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // build a map(value,index) with O(n)
    let mut map: HashMap<i32, usize> = HashMap::new();

    // search for target-n in map and retrieve indices, still O(n)
    for (i, n) in nums.iter().enumerate() {
        if let Some(j) = map.get(&(target - n)) {
            if i != *j {
                return vec![i as i32, *j as i32];
            }
        } else {
            map.insert(*n, i);
        }
    }

    panic!("impossible")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_on2_two_sum() {
        assert_eq!(on2_two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(on2_two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }

    #[test]
    fn test_on_two_sum() {
        assert_eq!(on_two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(on_two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }
}
