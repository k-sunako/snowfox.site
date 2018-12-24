fn coliatz(mut n: u64) {

    while n != 1 {
        println!("{}", n);
        if n % 2 == 0 {
            n = n / 2;
        } else {
            n = 3 * n + 1;
        }
    }

    println!("{}", n);
}

fn main() {

    coliatz(1000);
}
