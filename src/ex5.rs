// 2520 is the smallest number that can be divided by each of
// the numbers from 1 to 10 without any remainder.
// What is the smallest positive number that is evenly
// divisible by all of the numbers from 1 to 20?
extern crate helpers;

fn main() {
    let r = vec!(1, // 1
                 2, // 2
                 3, // 3
                 2, // 4
                 5, // 5
                 1, // 6
                 7, // 7
                 2, // 8
                 3, // 9
                 1, // 10
                 11,// 11
                 1, // 12
                 13,// 13
                 1, // 14
                 1, // 15
                 2, // 16
                 17,// 17
                 1, // 18
                 19,// 19
                 1  // 20
    ).iter().fold(1, |acc, x| acc * x );
    println!("{}", r);
}
