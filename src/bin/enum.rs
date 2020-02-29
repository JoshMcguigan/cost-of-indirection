use std::{env, time::Instant};

fn main() {
    // I don't intend to ever set this environment variable. The purpose
    // of this is only to ensure the compiler won't be able to know
    // which of the two OddFinder implementations we will use.
    let finder: OddFinders = match env::var("USE_FINDER_TWO") {
        Ok(use_finder_two) if use_finder_two == String::from("True")
            => OddFinders::Two(OddFinderTwo),
        _ => OddFinders::One(OddFinderOne),
    };

    let start_time = Instant::now();
    let result = find_nth_odd(finder, 1_000_000_000);
    let elapsed_time = start_time.elapsed().as_millis();

    println!("{} in {} ms", result, elapsed_time);
}


fn find_nth_odd(odd_finder: OddFinders, n: u64) -> u64 {
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

enum OddFinders {
    One(OddFinderOne),
    Two(OddFinderTwo),
}

impl OddFinder for OddFinders {
    fn is_odd(&self, n: u64) -> bool {
        match self {
            OddFinders::One(finder) => finder.is_odd(n),
            OddFinders::Two(finder) => finder.is_odd(n),
        }
    }
}
