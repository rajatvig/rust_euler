pub fn primefactors(n: isize) -> Vec<isize> {
    let mut number = n;
    let mut factors = Vec::new();
    if number % 2 == 0 {
        factors.push(2);
        number = number / 2;
    }

    let mut factor = 3;
    let upperlimit = (n as f64).sqrt() as isize + 2;
    while factor <= upperlimit {
        if number % factor == 0 {
            factors.push(factor);
            number = number / factor;
        }
        factor += 2;
    }

    factors
}

fn find_root(a: f64, x: f64, epsilon: f64) -> f64 {
    let nextx = (a/x + x) / 2.0;
    if (x-nextx).abs() < epsilon*x {
        nextx
    }
    else {
        find_root(a, nextx, epsilon)
    }
}

pub fn sqrt(n: f64) -> f64 {
    find_root(n, 2.0, 1.0e-40_f64)
}

#[test]
fn test_sqrt() {
    assert!(sqrt(4.0) == 2.0);
}

#[test]
fn test_primefactors() {
    assert!(primefactors(10) == [2, 5]);
    assert!(primefactors(13195) == [5,7,13,29]);
}
