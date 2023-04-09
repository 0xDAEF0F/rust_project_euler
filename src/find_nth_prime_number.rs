#[allow(dead_code)]
pub fn n_prime_number(n: usize) -> u64 {
    let sieve = Sieve {
        primes: Vec::new(),
        candidate: 2,
    };
    sieve.take(n).last().unwrap()
}
struct Sieve {
    primes: Vec<u64>,
    candidate: u64,
}

impl Iterator for Sieve {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        while !is_prime(self.candidate, &self.primes) {
            self.candidate += 1;
        }
        let prime = self.candidate;
        self.primes.push(prime);
        self.candidate += 1;
        Some(prime)
    }
}

fn is_prime(candidate: u64, primes: &[u64]) -> bool {
    for &prime in primes {
        if prime * prime > candidate {
            break;
        }
        if candidate % prime == 0 {
            return false;
        }
    }
    true
}
