fn sum_of_multiples_of(n: u64, until: u64) -> u64 {
    let p = until / n;
    n * (p * (p + 1)) / 2
}

fn main() {
    let target : u64 = 1000_000_000;
    let result = sum_of_multiples_of(3, target) + sum_of_multiples_of(5, target) - sum_of_multiples_of(15, target);
    println!("Sum of multiples of 3 or 5 from 0 to {} is {}", target, result);
}