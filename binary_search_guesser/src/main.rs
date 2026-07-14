use std::io;

fn get_new_bounds(l_bound:i32, u_bound:i32) -> (i32, i32) {
    let pivot = (l_bound + u_bound) / 2;

    println!("Are you higher(H) or lower(L) than {}", pivot);

    let mut input = String::new();

    io::stdin()
    .read_line(&mut input)
    .unwrap();

    let direction = input.trim().to_uppercase();

    if direction == "H" {
        (pivot, u_bound)
    } else if direction == "L" {
        (l_bound, pivot)
    } else {
        println!("You entered an invalid input");
        get_new_bounds(l_bound, u_bound)
    }
}

fn guess_is_num(n: i32) -> bool {
    println!("Is your number {}? (Y/N)", n);
    let mut input = String::new();

    io::stdin()
    .read_line(&mut input)
    .unwrap();

    let result = input.trim().to_uppercase();

    if result == "Y" {
        true
    } else if result == "N" {
        false
    } else {
        println!("You entered an invalid input");
        guess_is_num(n)
    }

}

fn main() {
    println!("The guessing game is beginning. Pick a number between 1 and 1000");
    let  (mut l_bound, mut u_bound) = (0, 1000);
    loop {
        (l_bound, u_bound) = get_new_bounds(l_bound, u_bound);
        println!("The new bounds are {} - {}", l_bound, u_bound);

        if u_bound - l_bound == 1 {
            if guess_is_num(l_bound) {u_bound = l_bound} else {l_bound = u_bound}
        }
        if l_bound == u_bound {
            println!("Your number is {}", l_bound);
            break;
        }
    }
}
