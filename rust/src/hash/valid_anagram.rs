use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut set: HashMap<char, i32> = HashMap::new();

    let pair = {
        if s.len() < t.len() {
            (&s, &t)
        } else {
            (&t, &s)
        }
    };

    for c in pair.0.chars() {
        if let Some(count) = set.get(&c) {
            set.insert(c, count + 1);
        } else {
            set.insert(c, 1);
        }
    }

    for c in pair.1.chars() {
        if let Some(count) = set.get_mut(&c) {
            if *count == 1 {
                set.remove(&c);
            } else {
                *count -= 1;
            }
        } else {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let car = "car".to_string();
        let rat = "rat".to_string();

        assert_eq!(is_anagram(car, rat), false);
    }
}
