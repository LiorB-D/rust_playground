fn next_collatz(n: i32) -> i32 {
    if n % 2 == 0 { 
        n / 2 
    } else { 
        3 * n + 1 
    }
}

fn collatz_sequence(mut n: i32) -> Vec<i32> {
    let mut sequence = Vec::new();

    sequence.push(n);

    while n != 1 {
        n = next_collatz(n);
        sequence.push(n);
    }
    sequence
}

fn main() {
    let starting_int = 10;
    println!("Sequence {:?}", collatz_sequence(starting_int));
    println!("Starting int was {}", starting_int)
}
