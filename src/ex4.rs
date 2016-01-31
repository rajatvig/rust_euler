// A palindromic number reads the same both ways.
// The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
// Find the largest palindrome made from the product of two 3-digit numbers.

fn main() {
    let mut largest = 0;
    for x in 100..999 {
        for y in 100..999 {
            let m = x*y;
            if palindrome(m) {
                largest = if m < largest {largest} else {m}
            }
        }
    }
    println!("{}", largest);
}

fn palindrome(n: isize) -> bool {
    let mut num = n;
    let mut rev = 0;
    while num > 0 {
        let dig = num % 10;
        rev = rev * 10 + dig;
        num = num / 10;
    }
    rev == n
}

#[test]
fn test_palindrome() {
    assert!(palindrome(9009));
    assert!(!palindrome(99009));
}
