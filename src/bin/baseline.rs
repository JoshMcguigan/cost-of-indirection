use std::time::Instant;

fn main() {
    let start_time = Instant::now();
    let result = find_nth_odd(1_000_000_000);
    let elapsed_time = start_time.elapsed().as_millis();

    println!("{} in {} ms", result, elapsed_time);
}

fn find_nth_odd(n: u64) -> u64 {
    let mut i = 0;
    let mut odd_count = 0;

    while odd_count != n {
        i += 1;
        if is_odd(i) {
            odd_count += 1;
        }
    }

    i
}

#[inline(never)]
fn is_odd(n: u64) -> bool {
    n % 2 == 1
}
