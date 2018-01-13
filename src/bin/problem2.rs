#![feature(test)]

extern crate test;

// Goal: Compute the sum of even Fibonacci number whose value is <= 4000000.
//
// this naive solution compute all Fibonacci numbers, test if even and sum
// But we can do better...
fn solution1() -> u64 {
    let mut n = 1;
    let mut m = 2;
    let mut result = 2;
    while m <= 4_000_000 {
        let tmp = n;
        n = m;
        m = tmp + m;
        if (m % 2) == 0 {
            result += m; 
        }
    }
    result
}

// Starting with f(1) = 1, f(2) = 1... we notice that every 3rd number is even, eg
// the sequence generates 1 (O), 1 (O), 2 (E), 3 (O), 5 (O), 8 (E)..., 
// So we can compute the sequence of 2 consecutive odd fibonacci numbers and sum their 
// sum to save a remainder operation:
// ..., x, y, x + y (even), y + (x + y), (x + y) + (y + (x + y)),...
// or 
// ..., x, y, z = x + y (sum+= z), x' = y + z, y' = z + y'
// 
// Using the bench below (cargo +nightly bench) we get:
// running 2 tests
// test tests::test_solution1 ... bench:          28 ns/iter (+/- 3)
// test tests::test_solution2 ... bench:          10 ns/iter (+/- 5)
//
// Almost a 3x speedup !
fn solution2() -> u64 {
    let mut x = 1;
    let mut y = 1;
    let mut sum = 0;
    while sum < 4_000_000 {
        let z = x + y; // even f(3n)
        sum += z;
        x = z + y; // f(3n+1)
        y = x + z; // f(3n+2)
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::test::Bencher;

    #[bench]
    fn test_solution1(b: &mut Bencher) {
        b.iter(|| solution1());
    }

    #[bench]
    fn test_solution2(b: &mut Bencher) {
        b.iter(|| solution2());
    }

}
fn main() {
    println!("Sum of even Fibonacci number <= 4 000 000 is {}", solution1());
    println!("Sum of even Fibonacci number <= 4 000 000 is {}", solution2());

}