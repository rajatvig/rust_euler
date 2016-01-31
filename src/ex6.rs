// The sum of the squares of the first ten natural numbers is,
//
// 1^2 + 2^2 + ... + 10^2 = 385
//
// The square of the sum of the first ten natural numbers is,
// (1 + 2 + ... + 10)^2 = 55^2 = 3025
//
// Hence the difference between the sum of the squares of the first ten natural numbers
// and the square of the sum is 3025 − 385 = 2640.
//
// Find the difference between the sum of the squares of the first one
// hundred natural numbers and the square of the sum.

fn main() {
    println!("{}", diff(100));
}

fn diff(n: isize) -> isize {
    let sq_sum = (1..n+1).fold(0, |acc, x| acc+(x*x));
    let sum = (1..n+1).fold(0, |acc, x| acc+x);
    sum*sum - sq_sum
}

#[test]
fn test_diff() {
    assert!(diff(10) == 2640);
}
