mod algorithms;
mod cli;
mod reconstruction;
mod utils;
use algorithms::{get_score, get_sequence_table};
use clap::Parser;
use cli::{parse_2d_array, Algorithm, Cli};
use reconstruction::{get_aligned_sequence, get_smith_waterman_sequence};
use utils::print_tab;

fn main() {
    let args = Cli::parse();
    let gap_val = args.gap;
    let scores = parse_2d_array(&args.substitution_matrix);
    let s1 = &args.s1;
    let s2 = &args.s2;
    let n = args.n;
    let algorithm = &args.algorithm;

    let get_score_fn = |c1: char, c2: char| get_score(c1, c2, scores);

    println!("s1: {}", s1);
    println!("s2: {}", s2);
    println!();

    let (tab, dir_tab) = get_sequence_table(s1, s2, gap_val, get_score_fn, algorithm);
    print_tab(s1, s2, &tab);

    let aligned;
    let score;
    if algorithm == &Algorithm::NeedlemanWunsch {
        aligned = get_aligned_sequence(s1, s2, &dir_tab, &mut n.clone());
        score = tab[s1.len()][s2.len()];
    } else {
        aligned = get_smith_waterman_sequence(s1, s2, &tab, &dir_tab, n);
        score = *tab.iter().flat_map(|row| row.iter()).max().unwrap();
    }

    println!();
    println!("Score: {}", score);
    for (pos, (s1_aligned, s2_aligned)) in aligned.iter().enumerate() {
        println!();
        println!("Alignment {}", pos + 1);
        println!("s1_aligned: {}", s1_aligned);
        println!("s2_aligned: {}", s2_aligned);
    }
}
