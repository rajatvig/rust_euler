#[derive(Copy, Clone, Debug)]
struct FibonacciTerm {
    prev1: isize,
    prev2: isize
}

struct FibonacciSeq {
    current: FibonacciTerm
}

impl Iterator for FibonacciSeq {
    type Item = FibonacciTerm;
    fn next(&mut self) -> Option<FibonacciTerm> {
        let current = FibonacciTerm {prev1: self.current.prev1 + self.current.prev2, prev2: self.current.prev1};
        self.current = current;
        Some(self.current)
    }
}

fn initialize() -> FibonacciSeq {
    FibonacciSeq { current: FibonacciTerm {prev1: 1, prev2: 1} }
}

fn main() {
    let n = initialize().into_iter()
        .take_while(|x| x.prev1 < 4000000)
        .filter(|x| x.prev1 % 2 == 0)
        .fold(0, |acc, x| acc + x.prev1);
    println!("{}", n);
}

#[test]
fn test_fibonacci_seq() {
    let mut seq =  initialize();
    assert!(seq.nth(8).unwrap().prev1 == 89);
}
