use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut hashset = HashSet::new();
    for i in nums {
        if hashset.contains(&i) {
            return true
        }
        hashset.insert(i);
    }
    false
}