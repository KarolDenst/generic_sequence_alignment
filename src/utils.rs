pub fn print_tab(s1: &str, s2: &str, tab: &Vec<Vec<i32>>) {
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
