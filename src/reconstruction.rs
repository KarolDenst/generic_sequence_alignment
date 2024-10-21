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

    if dir_tab[i][j].contains(&Direction::Up) {
        let mut prevs = get_aligned_sequence(&s1[..i - 1], &s2[..j], dir_tab);
        for prev in prevs.iter_mut() {
            prev.0.push(s1.chars().nth(i - 1).unwrap());
            prev.1.push('-');
        }
        alligned.append(&mut prevs);
    }
    if dir_tab[i][j].contains(&Direction::Left) {
        let mut prevs = get_aligned_sequence(&s1[..i], &s2[..j - 1], dir_tab);
        for prev in prevs.iter_mut() {
            prev.0.push('-');
            prev.1.push(s2.chars().nth(j - 1).unwrap());
        }
        alligned.append(&mut prevs);
    }
    if dir_tab[i][j].contains(&Direction::Diagonal) {
        let mut prevs = get_aligned_sequence(&s1[..i - 1], &s2[..j - 1], dir_tab);
        for prev in prevs.iter_mut() {
            prev.0.push(s1.chars().nth(i - 1).unwrap());
            prev.1.push(s2.chars().nth(j - 1).unwrap());
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
                max_indices.clear(); // New max found, clear previous indices
                max_indices.push((i, j));
            } else if value == max_value {
                max_indices.push((i, j)); // Same max value found, add indices
            }
        }
    }

    return max_indices;
}
