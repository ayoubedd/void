use std::collections::HashMap;

fn gen_char_feq(str: &String) -> String {
    let mut map = [0; 26];

    for c in str.chars() {
        let index = c as i32 - 'a' as i32;
        map[index as usize] += 1;
    }

    let mut res: String = String::new();

    for ele in map {
        res.push(char::from_u32(ele as u32).unwrap());
    }

    res
}

fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for str in &strs {
        let char_freq = gen_char_feq(str);

        let group = {
            if let Some(group) = map.get_mut(&char_freq) {
                group
            } else {
                let group = vec![];
                map.insert(char_freq.clone(), group);
                map.get_mut(&char_freq).unwrap()
            }
        };

        group.push(str.clone());
    }

    let mut result: Vec<Vec<String>> = Vec::new();

    for ele in &map {
        result.push(ele.1.clone());
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::hash::group_anagrams::group_anagrams;

    #[test]
    fn basic() {
        let anagrams = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ];
        let mut result = group_anagrams(anagrams);
        result.sort();
        assert_eq!(
            result,
            [
                ["bat"].to_vec(),
                ["eat", "tea", "ate"].to_vec(),
                ["tan", "nat"].to_vec()
            ]
            .to_vec()
        );
    }
}
