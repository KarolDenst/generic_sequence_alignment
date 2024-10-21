#[derive(Clone, PartialEq, Debug)]
pub enum Direction {
    Up,
    Left,
    Diagonal,
    Start,
}

#[derive(PartialEq)]
pub enum Algorithm {
    NeedlemanWunsch,
    SmithWaterman,
}

pub fn get_sequence_table<F>(
    s1: &str,
    s2: &str,
    gap_val: i32,
    get_score: F,
    algorithm: &Algorithm,
) -> (Vec<Vec<i32>>, Vec<Vec<Vec<Direction>>>)
where
    F: Fn(char, char) -> i32,
{
    let mut tab = vec![vec![0; s2.len() + 1]; s1.len() + 1];
    let mut dir_tab = vec![vec![vec![]; s2.len() + 1]; s1.len() + 1];

    if algorithm == &Algorithm::SmithWaterman {
        for i in 0..s1.len() + 1 {
            tab[i][0] = 0;
            dir_tab[i][0] = vec![Direction::Start];
        }
        for j in 0..s2.len() + 1 {
            tab[0][j] = 0;
            dir_tab[0][j] = vec![Direction::Start];
        }
    } else {
        for i in 1..s1.len() + 1 {
            tab[i][0] = gap_val * i as i32;
            dir_tab[i][0] = vec![Direction::Up];
        }
        for j in 1..s2.len() + 1 {
            tab[0][j] = gap_val * j as i32;
            dir_tab[0][j] = vec![Direction::Left];
        }
    }

    for i in 1..s1.len() + 1 {
        for j in 1..s2.len() + 1 {
            let score = get_score(
                s1[i - 1..].chars().next().unwrap(),
                s2[j - 1..].chars().next().unwrap(),
            );
            let mut max = std::cmp::max(
                tab[i - 1][j - 1] + score,
                std::cmp::max(tab[i - 1][j], tab[i][j - 1]) + gap_val,
            );
            if algorithm == &Algorithm::SmithWaterman {
                max = max.max(0);
                if max == 0 {
                    dir_tab[i][j].push(Direction::Start);
                }
            }
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

pub fn get_score(c1: char, c2: char, scores: [[i32; 4]; 4]) -> i32 {
    let idx1 = get_char_idx(c1);
    let idx2 = get_char_idx(c2);
    return scores[idx1][idx2];
}

fn get_char_idx(c: char) -> usize {
    return match c {
        'A' => 0,
        'C' => 1,
        'G' => 2,
        'T' => 3,
        _ => panic!("Invalid character"),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn needleman_wunsch_works() {
        let s1 = "GATTACA";
        let s2 = "GCATGCG";
        let gap_val = -2;
        let scores = [
            [5, -4, -4, -1],
            [-4, 5, -4, -1],
            [-4, -4, 5, -1],
            [-1, -1, -1, 5],
        ];
        let get_score_fn = |c1: char, c2: char| get_score(c1, c2, scores);
        let expected: Vec<Vec<i32>> = vec![
            vec![0, -2, -4, -6, -8, -10, -12, -14],
            vec![-2, 5, 3, 1, -1, -3, -5, -7],
            vec![-4, 3, 1, 8, 6, 4, 2, 0],
            vec![-6, 1, 2, 6, 13, 11, 9, 7],
            vec![-8, -1, 0, 4, 11, 12, 10, 8],
            vec![-10, -3, -2, 5, 9, 10, 8, 6],
            vec![-12, -5, 2, 3, 7, 8, 15, 13],
            vec![-14, -7, 0, 7, 5, 6, 13, 11],
        ];

        let (tab, _) =
            get_sequence_table(s1, s2, gap_val, get_score_fn, &Algorithm::NeedlemanWunsch);

        assert_eq!(tab, expected);
    }

    #[test]
    fn smith_waterman_works() {
        let s1 = "GATTACA";
        let s2 = "GCATGCG";
        let gap_val = -2;
        let scores = [
            [5, -4, -4, -1],
            [-4, 5, -4, -1],
            [-4, -4, 5, -1],
            [-1, -1, -1, 5],
        ];
        let get_score_fn = |c1: char, c2: char| get_score(c1, c2, scores);
        let expected: Vec<Vec<i32>> = vec![
            vec![0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 5, 3, 1, 0, 5, 3, 5],
            vec![0, 3, 1, 8, 6, 4, 2, 3],
            vec![0, 1, 2, 6, 13, 11, 9, 7],
            vec![0, 0, 0, 4, 11, 12, 10, 8],
            vec![0, 0, 0, 5, 9, 10, 8, 6],
            vec![0, 0, 5, 3, 7, 8, 15, 13],
            vec![0, 0, 3, 10, 8, 6, 13, 11],
        ];

        let (tab, _) = get_sequence_table(s1, s2, gap_val, get_score_fn, &Algorithm::SmithWaterman);

        assert_eq!(tab, expected);
    }
}
