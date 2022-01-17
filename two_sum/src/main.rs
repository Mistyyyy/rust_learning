use std::collections::HashMap;

fn main() {

    assert_eq!(two_sum(vec![1, 2, 3, 7], 5), vec![1, 2])

}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut nums_hashmap = HashMap::new();

    for i in 0..nums.len() {
        if let Some(v) = nums_hashmap.get(&(target - nums[i])) {
            return vec![*v, i as i32]
        }
        nums_hashmap.insert(nums[i], i as i32);
    }
    vec![]
}