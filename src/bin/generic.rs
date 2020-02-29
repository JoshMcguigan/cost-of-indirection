use std::time::Instant;

fn main() {
    let finder = OddFinderOne;

    let start_time = Instant::now();
    let result = find_nth_odd(finder, 1_000_000_000);
    let elapsed_time = start_time.elapsed().as_millis();

    println!("{} in {} ms", result, elapsed_time);
}

fn find_nth_odd<T: OddFinder>(odd_finder: T, n: u64) -> u64 {
    let mut i = 0;
    let mut odd_count = 0;

    while odd_count != n {
        i += 1;
        if odd_finder.is_odd(i) {
            odd_count += 1;
        }
    }

    i
}

trait OddFinder {
    fn is_odd(&self, n: u64) -> bool;
}

struct OddFinderOne;

impl OddFinder for OddFinderOne {
    fn is_odd(&self, n: u64) -> bool {
        n % 2 == 1
    }
}

struct OddFinderTwo;

impl OddFinder for OddFinderTwo {
    fn is_odd(&self, _n: u64) -> bool {
        unimplemented!()
    }
}
