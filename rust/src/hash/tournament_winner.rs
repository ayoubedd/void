use std::collections::HashMap;

fn inc_team_by_one(map: &mut HashMap<String, i32>, away: &str) {
    let wow = map.get_mut(away);
    let count = match wow {
        Some(count) => count,
        None => {
            map.insert(away.to_string(), 0);
            map.get_mut(away).unwrap()
        }
    };

    *count += 1;
}

fn tourname_winner<'a>(matches: &'a [(&str, &str)], results: &'a [i32]) -> String {
    if matches.len() != results.len() {
        return "".to_string();
    }
    let mut map: HashMap<String, i32> = HashMap::new();

    let mut index: usize = 0;

    for (home, away) in matches {
        let winning_team = if results[index] == 0 {
            // away is the winner
            away
        } else {
            // home is the winner
            home
        };
        inc_team_by_one(&mut map, winning_team);
        index += 1;
    }

    let mut winner_team: (String, i32) = (String::new(), -1);
    for (name, score) in &map {
        if score > &winner_team.1 {
            winner_team = (name.clone(), *score);
        }
    }

    winner_team.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let teams = [("HTML", "C#"), ("C#", "Python"), ("Python", "HTML")];
        let results = [0, 0, 1];

        let result = tourname_winner(&teams, &results);
        assert_eq!(result, "Python");
    }
}
