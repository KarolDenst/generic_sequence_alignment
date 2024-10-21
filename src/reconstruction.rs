use crate::algorithms::Direction;

pub fn get_smith_waterman_sequence(
    s1: &str,
    s2: &str,
    tab: &Vec<Vec<i32>>,
    dir_tab: &Vec<Vec<Vec<Direction>>>,
) -> Vec<(String, String)> {
    let mut all_aligned = vec![];
    let indices = find_max_indices(&tab);
    for (i, j) in indices.iter() {
        let mut aligned = get_aligned_sequence(&s1[..*i], &s2[..*j], &dir_tab);
        all_aligned.append(&mut aligned);
    }

    return all_aligned;
}

pub fn get_aligned_sequence(
    s1: &str,
    s2: &str,
    dir_tab: &Vec<Vec<Vec<Direction>>>,
) -> Vec<(String, String)> {
    let i = s1.len();
    let j = s2.len();
    let mut alligned = Vec::new();

    if i == 0 && j == 0 {
        return vec![(String::new(), String::new())];
    }

    let c1 = s1[i - 1..].chars().next().unwrap();
    let c2 = s2[j - 1..].chars().next().unwrap();

    if dir_tab[i][j].contains(&Direction::Up) {
        let mut prevs = get_aligned_sequence(&s1[..i - 1], &s2[..j], dir_tab);
        for prev in prevs.iter_mut() {
            prev.0.push(c1);
            prev.1.push('-');
        }
        alligned.append(&mut prevs);
    }
    if dir_tab[i][j].contains(&Direction::Left) {
        let mut prevs = get_aligned_sequence(&s1[..i], &s2[..j - 1], dir_tab);
        for prev in prevs.iter_mut() {
            prev.0.push('-');
            prev.1.push(c2);
        }
        alligned.append(&mut prevs);
    }
    if dir_tab[i][j].contains(&Direction::Diagonal) {
        let mut prevs = get_aligned_sequence(&s1[..i - 1], &s2[..j - 1], dir_tab);
        for prev in prevs.iter_mut() {
            prev.0.push(c1);
            prev.1.push(c2);
        }
        alligned.append(&mut prevs);
    }
    if dir_tab[i][j].contains(&Direction::Start) {
        let mut prevs = vec![(String::new(), String::new())];
        alligned.append(&mut prevs);
    }

    return alligned;
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
    use crate::algorithms::{get_score, get_sequence_table, Algorithm};

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
        let aligned = get_aligned_sequence(s1, s2, &dir_tab);
        let expected = vec![
            (String::from("G-ATTAC-A"), String::from("GCATG-CG-")),
            (String::from("G-ATTACA-"), String::from("GCATG-C-G")),
            (String::from("G-ATTACA"), String::from("GCATG-CG")),
        ];

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

        let aligned = get_smith_waterman_sequence(s1, s2, &tab, &dir_tab);
        let expected = vec![(String::from("G-ATTAC"), String::from("GCATG-C"))];

        assert_eq!(aligned, expected);
    }
}
