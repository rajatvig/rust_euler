// The prime factors of 13195 are 5, 7, 13 and 29.
// What is the largest prime factor of the number 600851475143 ?

fn main() {
    println!("{:?}", primefactors(13195));
    println!("{:?}", primefactors(600851475143));
    println!("{}", primefactors(600851475143).iter().max().unwrap());
}

fn primefactors(n: isize) -> Vec<isize> {
    let mut number = n;
    let mut factors = Vec::new();
    if number % 2 == 0 {
        factors.push(2);
        number = number / 2;
    }

    let mut factor = 3;
    let upperlimit = (number as f64).sqrt() as isize + 1;
    while factor < upperlimit {
        if number % factor == 0 {
            factors.push(factor);
            number = number / factor;
        }
        factor += 2;
    }

    factors
}

#[test]
fn test_primefactors() {
    assert!(primefactors(10) == [2, 5]);
    assert!(primefactors(13195) == [5,7,13,29]);
}
