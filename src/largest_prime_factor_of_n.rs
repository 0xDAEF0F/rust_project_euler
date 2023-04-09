#[allow(dead_code)]
pub fn largest_prime_factor_of_n(mut n: u64) -> Option<u64> {
    let mut factors = Vec::new();

    // Divide n by 2 until it's odd.
    while n % 2 == 0 {
        factors.push(2);
        n /= 2;
    }

    // Try odd numbers starting from 3.
    let mut divisor = 3;
    while n > 1 {
        while n % divisor == 0 {
            factors.push(divisor);
            n /= divisor;
        }
        divisor += 2;
    }

    factors.into_iter().max()
}
