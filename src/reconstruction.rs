use crate::algorithms::Direction;

pub fn get_smith_waterman_sequence(
    s1: &str,
    s2: &str,
    tab: &Vec<Vec<i32>>,
    dir_tab: &Vec<Vec<Vec<Direction>>>,
    n: u32,
) -> Vec<(String, String)> {
    let mut max = n.clone();
    let mut all_aligned = vec![];
    let indices = find_max_indices(&tab);
    for (i, j) in indices.iter() {
        let mut aligned = get_aligned_sequence(&s1[..*i], &s2[..*j], &dir_tab, &mut max);
        all_aligned.append(&mut aligned);
    }

    return all_aligned;
}

pub fn get_aligned_sequence(
    s1: &str,
    s2: &str,
    dir_tab: &Vec<Vec<Vec<Direction>>>,
    n: &mut u32,
) -> Vec<(String, String)> {
    let mut aligned = Vec::new();
    if *n == 0 {
        return aligned;
    }
    *n -= 1;

    let s1_chars: Vec<_> = s1.chars().collect();
    let s2_chars: Vec<_> = s2.chars().collect();

    let mut stack = Vec::new();
    stack.push((s1.len(), s2.len(), String::new(), String::new()));

    while let Some((i, j, s1_aligned, s2_aligned)) = stack.pop() {
        if i == 0 && j == 0 {
            aligned.push((s1_aligned.clone(), s2_aligned.clone()));
            continue;
        }

        *n += 1;

        let c1 = if i > 0 { s1_chars[i - 1] } else { '-' };
        let c2 = if j > 0 { s2_chars[j - 1] } else { '-' };

        if dir_tab[i][j].contains(&Direction::Start) {
            aligned.push((s1_aligned.clone(), s2_aligned.clone()));
            *n -= 1;
            if *n == 0 {
                continue;
            }
        }
        if dir_tab[i][j].contains(&Direction::Up) {
            let mut s1 = s1_aligned.clone();
            s1.push(c1);
            let mut s2 = s2_aligned.clone();
            s2.push('-');
            stack.push((i - 1, j, s1, s2));
            *n -= 1;
            if *n == 0 {
                continue;
            }
        }
        if dir_tab[i][j].contains(&Direction::Left) {
            let mut s1 = s1_aligned.clone();
            s1.push('-');
            let mut s2 = s2_aligned.clone();
            s2.push(c2);
            stack.push((i, j - 1, s1, s2));
            *n -= 1;
            if *n == 0 {
                continue;
            }
        }
        if dir_tab[i][j].contains(&Direction::Diagonal) {
            let mut s1 = s1_aligned.clone();
            s1.push(c1);
            let mut s2 = s2_aligned.clone();
            s2.push(c2);
            stack.push((i - 1, j - 1, s1, s2));
            *n -= 1;
            if *n == 0 {
                continue;
            }
        }
    }

    return aligned
        .iter()
        .map(|(s1, s2)| (s1.chars().rev().collect(), s2.chars().rev().collect()))
        .collect();
}

fn find_max_indices(matrix: &Vec<Vec<i32>>) -> Vec<(usize, usize)> {
    let mut max_value = i32::MIN;
    let mut max_indices = Vec::new();

    for (i, row) in matrix.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            if value > max_value {
                max_value = value;
                max_indices.clear();
                max_indices.push((i, j));
            } else if value == max_value {
                max_indices.push((i, j));
            }
        }
    }

    return max_indices;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithms::{get_score, get_sequence_table};
    use crate::cli::Algorithm;

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

        let (_, dir_tab) =
            get_sequence_table(s1, s2, gap_val, get_score_fn, &Algorithm::NeedlemanWunsch);
        let mut aligned = get_aligned_sequence(s1, s2, &dir_tab, &mut 100);
        aligned.sort();
        let mut expected = vec![
            (String::from("G-ATTAC-A"), String::from("GCATG-CG-")),
            (String::from("G-ATTACA-"), String::from("GCATG-C-G")),
            (String::from("G-ATTACA"), String::from("GCATG-CG")),
        ];
        expected.sort();

        assert_eq!(aligned, expected);
    }

    #[test]
    fn needleman_wunsch_with_limited_n_works() {
        let s1 = "TTT";
        let s2 = "T";
        let gap_val = -2;
        let scores = [
            [5, -4, -4, -1],
            [-4, 5, -4, -1],
            [-4, -4, 5, -1],
            [-1, -1, -1, 5],
        ];
        let get_score_fn = |c1: char, c2: char| get_score(c1, c2, scores);

        let (_, dir_tab) =
            get_sequence_table(s1, s2, gap_val, get_score_fn, &Algorithm::NeedlemanWunsch);
        let mut aligned = get_aligned_sequence(s1, s2, &dir_tab, &mut 2);
        aligned.sort();
        let mut expected = vec![
            (String::from("TTT"), String::from("--T")),
            (String::from("TTT"), String::from("T--")),
        ];
        expected.sort();

        assert_eq!(aligned, expected);
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
        let (tab, dir_tab) =
            get_sequence_table(s1, s2, gap_val, get_score_fn, &Algorithm::SmithWaterman);

        let aligned = get_smith_waterman_sequence(s1, s2, &tab, &dir_tab, 100);
        let expected = vec![(String::from("G-ATTAC"), String::from("GCATG-C"))];

        assert_eq!(aligned, expected);
    }

    #[test]
    fn smith_waterman_with_limited_n_works() {
        let s1 = "TTT";
        let s2 = "T";
        let gap_val = -2;
        let scores = [
            [5, -4, -4, -1],
            [-4, 5, -4, -1],
            [-4, -4, 5, -1],
            [-1, -1, -1, 5],
        ];
        let get_score_fn = |c1: char, c2: char| get_score(c1, c2, scores);
        let (tab, dir_tab) =
            get_sequence_table(s1, s2, gap_val, get_score_fn, &Algorithm::SmithWaterman);

        let aligned = get_smith_waterman_sequence(s1, s2, &tab, &dir_tab, 2);
        let expected = vec![
            (String::from("T"), String::from("T")),
            (String::from("T"), String::from("T")),
        ];

        assert_eq!(aligned, expected);
    }
}
