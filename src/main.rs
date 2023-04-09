mod largest_prime_factor_of_n;
use largest_prime_factor_of_n::largest_prime_factor_of_n;

fn main() {
    loop {
        println!("Please enter an integer:");
        let mut input: String = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let integer: u64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a valid integer.");
                return;
            }
        };

        let lpf: Option<u64> = largest_prime_factor_of_n(integer);

        match lpf {
            Some(largest_prime_factor) => {
                println!(
                    "Largest prime factor of {} is: {}",
                    integer, largest_prime_factor
                )
            }
            None => println!("No prime factors found."),
        }
    }
}
