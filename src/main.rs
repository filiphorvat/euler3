fn main() {
    let n = 600851475143_u64;
    let factor = largest_prime_factor(n);
    println!("{factor}");
}

fn is_prime(n: u64) -> bool {
    (2..isqrt(n)).fold(true, |acc: bool, div: u64| acc & (n % div != 0))
}

fn largest_prime_factor(n: u64) -> u64 {
    (2..isqrt(n))
        .rev()
        .find(|div| n % div == 0 && is_prime(*div))
        .unwrap()
}

fn isqrt(n: u64) -> u64 {
    (n as f64).sqrt() as u64
}
