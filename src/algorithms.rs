#[derive(Clone, PartialEq, Debug)]
pub enum Direction {
    Up,
    Left,
    Diagonal,
    Start,
}

pub fn needleman_wunsch(
    s1: &str,
    s2: &str,
    gap_val: i32,
    get_score: fn(char, char) -> i32,
) -> (Vec<Vec<i32>>, Vec<Vec<Vec<Direction>>>) {
    let mut tab = vec![vec![0; s2.len() + 1]; s1.len() + 1];
    let mut dir_tab = vec![vec![vec![]; s2.len() + 1]; s1.len() + 1];

    for i in 1..s1.len() + 1 {
        tab[i][0] = gap_val * i as i32;
        dir_tab[i][0] = vec![Direction::Up];
    }
    for j in 1..s2.len() + 1 {
        tab[0][j] = gap_val * j as i32;
        dir_tab[0][j] = vec![Direction::Left];
    }

    for i in 1..s1.len() + 1 {
        for j in 1..s2.len() + 1 {
            let score = get_score(
                s1.chars().nth(i - 1).unwrap(),
                s2.chars().nth(j - 1).unwrap(),
            );
            let max = std::cmp::max(
                tab[i - 1][j - 1] + score,
                std::cmp::max(tab[i - 1][j], tab[i][j - 1]) + gap_val,
            );
            if tab[i - 1][j - 1] + score == max {
                dir_tab[i][j].push(Direction::Diagonal);
            }
            if tab[i - 1][j] + gap_val == max {
                dir_tab[i][j].push(Direction::Up);
            }
            if tab[i][j - 1] + gap_val == max {
                dir_tab[i][j].push(Direction::Left);
            }
            tab[i][j] = max;
        }
    }

    return (tab, dir_tab);
}

pub fn smith_waterman(
    s1: &str,
    s2: &str,
    gap_val: i32,
    get_score: fn(char, char) -> i32,
) -> (Vec<Vec<i32>>, Vec<Vec<Vec<Direction>>>) {
    let mut tab = vec![vec![0; s2.len() + 1]; s1.len() + 1];
    let mut dir_tab = vec![vec![vec![]; s2.len() + 1]; s1.len() + 1];

    for i in 1..s1.len() + 1 {
        tab[i][0] = 0;
        dir_tab[i][0] = vec![Direction::Start];
    }
    for j in 1..s2.len() + 1 {
        tab[0][j] = 0;
        dir_tab[0][j] = vec![Direction::Start];
    }

    for i in 1..s1.len() + 1 {
        for j in 1..s2.len() + 1 {
            let score = get_score(
                s1.chars().nth(i - 1).unwrap(),
                s2.chars().nth(j - 1).unwrap(),
            );
            let max = std::cmp::max(
                std::cmp::max(tab[i - 1][j - 1] + score, 0),
                std::cmp::max(tab[i - 1][j], tab[i][j - 1]) + gap_val,
            );
            if tab[i - 1][j - 1] + score == max {
                dir_tab[i][j].push(Direction::Diagonal);
            }
            if tab[i - 1][j] + gap_val == max {
                dir_tab[i][j].push(Direction::Up);
            }
            if tab[i][j - 1] + gap_val == max {
                dir_tab[i][j].push(Direction::Left);
            }
            if max == 0 {
                dir_tab[i][j].push(Direction::Start);
            }
            tab[i][j] = max;
        }
    }

    return (tab, dir_tab);
}
