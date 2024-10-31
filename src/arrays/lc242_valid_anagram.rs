use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    let mut sash = HashMap::new();
    let mut tash = HashMap::new();

    for i in s.chars() {
        *sash.entry(i).or_insert(0) += 1;
    }
    for i in t.chars() {
        *tash.entry(i).or_insert(0) += 1;
    }
    if sash == tash {
        return true; 
    }
    false
}
    