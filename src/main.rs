extern crate fizzbuzz;
extern crate time;

use fizzbuzz::fizzbuzz_number_formatter::FizzbuzzMessageFormatter;

fn main() {

    let formatter = FizzbuzzMessageFormatter::default();
    let iterations = 1_000_000;

    let start_time = time::precise_time_ns();

    for count in 0..iterations {
        formatter.format_number(count);
    }

    let nano_seconds = time::precise_time_ns() - start_time;

    println!("Processed {} fizz buzz numbers in {}ms.", iterations, nano_seconds / 1_000_000);
    println!("{} numbers were formatted per second.", iterations * 1_000_000_000 / nano_seconds as i64 );
}
