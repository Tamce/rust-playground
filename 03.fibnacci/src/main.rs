use std::io;
fn main() {
    let mut n = String::new();
    print!("Input n for outputing n fibonacci: ");
    io::stdin().read_line(&mut n).expect("Read line failed!");
    let n = n.trim().parse().expect("must provice number");
    let mut last = 1;
    let mut cur = 1;
    for _i in 1..=n {
        print!("{} ", last);
        cur = last + cur;
        last = cur - last;
    }
}
