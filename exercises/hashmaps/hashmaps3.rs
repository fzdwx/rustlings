// hashmaps3.rs

// 给出了一场足球比赛的比分列表（每行一个）。每行的形式为：
// <team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>
// 示例：England,France,4,2（英格兰进了 4 个球，法国进了 2 个球）。

// 你必须建立一个包含球队名称、球队进球和球队失球的得分表。
// 建立得分表的一个方法是使用Hashmap。该解决方案部分是为了使用Hashmap而写的，完成它就可以通过测试。
// Make me pass the tests!

// Execute `rustlings hint hashmaps3` or use the `hint` watch subcommand for a hint.

use std::collections::HashMap;

// A structure to store team name and its goal details.
struct Team {
    name: String,
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: String) -> HashMap<String, Team> {
    // The name of the team is the key and its associated struct is the value.
    let mut scores: HashMap<String, Team> = HashMap::new();

    for r in results.lines() {
        let v: Vec<&str> = r.split(',').collect();
        let team_1_name = v[0].to_string();
        let team_1_score: u8 = v[2].parse().unwrap();
        let team_2_name = v[1].to_string();
        let team_2_score: u8 = v[3].parse().unwrap();

        // 请记住，团队_1的进球数将是团队_2的失球数，同样，团队_2的进球数将是团队_1的失球数。
        f(&mut scores, team_1_name, team_1_score, team_2_score);
        f(&mut scores, team_2_name, team_2_score, team_1_score);
    }
    scores
}

fn f(scores: &mut HashMap<String, Team>, team_1_name: String, team_1_score: u8, team_2_score: u8) {
    let team = scores.entry(team_1_name.clone()).or_insert(Team {
        name: team_1_name.clone(),
        goals_scored: 0,
        goals_conceded: 0,
    });

    team.goals_scored += team_1_score;
    team.goals_conceded += team_2_score;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_results() -> String {
        let results = "".to_string()
            + "England,France,4,2\n"
            + "France,Italy,3,1\n"
            + "Poland,Spain,2,0\n"
            + "Germany,England,2,1\n";
        results
    }

    #[test]
    fn build_scores() {
        let scores = build_scores_table(get_results());

        let mut keys: Vec<&String> = scores.keys().collect();
        keys.sort();
        assert_eq!(
            keys,
            vec!["England", "France", "Germany", "Italy", "Poland", "Spain"]
        );
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(get_results());
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 5);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(get_results());
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 2);
    }
}
