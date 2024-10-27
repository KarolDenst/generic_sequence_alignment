Example of running the program:

``````consoleconsole
cargo run -- --n 2 --s1 ATAT --s2 TATA --substitution-matrix "5,-4,-4,-1;-4,5,-4,-1;-4,-4,5,-1;-1,-1,-1,5" --algorithm "needleman-wunsch"

cargo run -- --s1 TTTT --s2 T --substitution-matrix "5,-4,-4,-1;-4,5,-4,-1;-4,-4,5,-1;-1,-1,-1,5" --algorithm "smith-waterman" --n 0
