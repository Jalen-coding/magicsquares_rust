use std::io;

fn main() {
    println!("\n Enter the size of the magic square (odd integer 3-11): ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: usize = input.trim().parse().expect("Invalid input");

    let mut square = vec![vec![0; n]; n];

    for (i, row) in square.iter_mut().enumerate() {
        for (j, e) in row.iter_mut().enumerate() {
            *e = n * (((i + 1) + (j + 1) - 1 + (n >> 1)) % n) +
                  (((i + 1) + (2 * (j + 1)) - 2) % n) + 1;
            print!("{:3} ", e);
        }
        println!();
    }

    let sum = n * (((n * n) + 1) / 2);
    println!("The sum of the square is {}", sum);
}
