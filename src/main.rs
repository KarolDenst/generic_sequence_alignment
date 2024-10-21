mod algorithms;
mod reconstruction;
mod utils;
use algorithms::{get_score, get_sequence_table, Algorithm};
use reconstruction::{get_aligned_sequence, get_smith_waterman_sequence};
use utils::print_tab;

fn main() {
    let gap_val = -2;
    let scores = [
        [5, -4, -4, -1],
        [-4, 5, -4, -1],
        [-4, -4, 5, -1],
        [-1, -1, -1, 5],
    ];
    let get_score_fn = |c1: char, c2: char| get_score(c1, c2, scores);
    let algorithm = &Algorithm::NeedlemanWunsch;

    let s2 = &"A".repeat(1000);
    let s1 = &"G".repeat(1000);
    // let s1 = "GATTACA";
    // let s2 = "GCATGCG";

    println!("s1: {}", s1);
    println!("s2: {}", s2);
    println!();

    let (tab, dir_tab) = get_sequence_table(s1, s2, gap_val, get_score_fn, algorithm);
    let aligned;
    let score;
    if algorithm == &Algorithm::NeedlemanWunsch {
        aligned = get_aligned_sequence(s1, s2, &dir_tab);
        score = tab[s1.len()][s2.len()];
    } else {
        aligned = get_smith_waterman_sequence(s1, s2, &tab, &dir_tab);
        score = *tab.iter().flat_map(|row| row.iter()).max().unwrap();
    }
    print_tab(s1, s2, &tab);

    println!();
    println!("Score: {}", score);
    for (pos, (s1_aligned, s2_aligned)) in aligned.iter().enumerate() {
        println!();
        println!("Alignment {}", pos + 1);
        println!("s1_aligned: {}", s1_aligned);
        println!("s2_aligned: {}", s2_aligned);
    }
}
