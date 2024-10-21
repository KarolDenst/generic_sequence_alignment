mod algorithms;
mod reconstruction;
use algorithms::{needleman_wunsch, smith_waterman};
use reconstruction::{get_aligned_sequence, get_smith_waterman_sequence};

#[derive(PartialEq)]
enum Algorithm {
    NeedlemanWunsch,
    SmithWaterman,
}

fn print_tab(s1: &str, s2: &str, tab: &Vec<Vec<i32>>) {
    print!("{:12} ", "");
    for i in 0..s2.len() {
        print!("{:4} ", s2.chars().nth(i).unwrap());
    }
    println!();
    for i in 0..s1.len() + 1 {
        print!(
            "{:4} ",
            if i == 0 {
                " ".chars().nth(0).unwrap()
            } else {
                s1.chars().nth(i - 1).unwrap()
            }
        );
        for j in 0..s2.len() + 1 {
            print!("{:4} ", tab[i][j]);
        }
        println!();
    }
}

fn get_score(c1: char, c2: char) -> i32 {
    if c1 == c2 {
        return 3;
    }
    if c1 == 'T' || c2 == 'T' {
        return -3;
    }
    return -3;
}

fn main() {
    let gap_val = -2;
    let algorithm = Algorithm::SmithWaterman;

    let s2 = "TGTTACGG";
    let s1 = "GGTTGACTA";

    println!("s1: {}", s1);
    println!("s2: {}", s2);
    println!();

    let tab;
    let dir_tab;
    let aligned;
    let score;
    if algorithm == Algorithm::NeedlemanWunsch {
        println!("Needleman-Wunsch");
        (tab, dir_tab) = needleman_wunsch(s1, s2, gap_val, get_score);
        aligned = get_aligned_sequence(s1, s2, &dir_tab);
        score = tab[s1.len()][s2.len()];
    } else {
        println!("Smith-Waterman");
        (tab, dir_tab) = smith_waterman(s1, s2, gap_val, get_score);
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
