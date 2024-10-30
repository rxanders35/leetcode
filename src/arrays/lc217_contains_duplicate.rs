use core::hash;
use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut hashset = HashSet::new();
    for i in nums {
        if !hashset.contains(&i){
            hashset.insert(i)
        } else {
            //figure this out
        }
    }
    true
}