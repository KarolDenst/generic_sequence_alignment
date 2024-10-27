use clap::{Parser, ValueEnum};

#[derive(PartialEq, Clone, ValueEnum)]
pub enum Algorithm {
    NeedlemanWunsch,
    SmithWaterman,
}

#[derive(Parser)]
#[command(about = "Needleman-Wunsch and Smith-Waterman algorithms")]
pub struct Cli {
    #[arg(short, long, help = "Maximum number of alignments to show")]
    pub n: u32,

    #[arg(short, long, default_value_t = -2, help="Gap penalty")]
    pub gap: i32,

    #[arg(long, help = "First sequence")]
    pub s1: String,

    #[arg(long, help = "Second sequence")]
    pub s2: String,

    #[arg(
        short,
        long,
        help = "Substitution matrix (4x4) in the format: 1,2,3,4;5,6,7,8;9,10,11,12;13,14,15,16"
    )]
    pub substitution_matrix: String,

    #[arg(
        short,
        long,
        value_enum,
        help = "Algorithm to use (NeedlemanWunsch or SmithWaterman)"
    )]
    pub algorithm: Algorithm,
}

pub fn parse_2d_array(input: &str) -> [[i32; 4]; 4] {
    let mut result = [[0; 4]; 4]; // Default value
    let rows: Vec<&str> = input.split(';').collect();

    if rows.len() != 4 {
        panic!("Expected exactly 4 rows for the 2D array");
    }

    for (i, row) in rows.iter().enumerate() {
        let values: Vec<i32> = row.split(',').map(|v| v.parse().unwrap()).collect();

        if values.len() != 4 {
            panic!("Each row must contain exactly 4 elements");
        }

        for (j, value) in values.iter().enumerate() {
            result[i][j] = *value;
        }
    }

    return result;
}
