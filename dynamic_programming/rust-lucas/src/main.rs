use std::time::{Instant};

fn main() {
    let lr = Instant::now();
    println!("Recurse @ 30: {}", lucas_recurse(20));
    println!("Time taken: {}", lr.elapsed().as_micros());

    let ldp = Instant::now();
    println!("Dynamic prog. @ 30: {}", lucas_dp(20));
    println!("Time taken: {}", ldp.elapsed().as_micros());
}

fn lucas_recurse(n: usize) -> usize {
    match n {
        0 => 2,
        1 => 1,
        n => {
            lucas_recurse(n-2) + lucas_recurse(n-1)
        }
    }
}

fn lucas_dp(n: usize) -> usize {
    let mut lookup_table: Vec<usize> = vec![0; 2]; 
    lookup_table[0] = 2;
    lookup_table[1] = 1;

    for i in 0..(n - 1) {
        let temp = lookup_table[0];
        lookup_table[0] = lookup_table[1];
        lookup_table[1] = lookup_table[1] + temp;
    }
    lookup_table[1]
}