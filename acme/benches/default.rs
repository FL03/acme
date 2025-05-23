/*
    Appellation: default <module>
    Contrib: @FL03
*/
use criterion::{BatchSize, BenchmarkId, Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use std::time::Duration;

/// the default number of iterations to benchmark a method for
const N: usize = 20;
/// the default number of seconds a benchmark should complete in
const DEFAULT_DURATION_SECS: u64 = 10;
/// the default batch sizes to use for the benchmarks
const BATCHES: [usize; 5] = [10, 50, 100, 500, 1000];

fn bench_simple_fibonacci(c: &mut Criterion) {
    c.bench_function("fib::fibonacci_nth", |b| {
        b.iter(|| fib::fibonacci_nth(black_box(N)))
    });
}

fn bench_recursive_fibonacci(c: &mut Criterion) {
    c.bench_function("fib::fibonacci_rec", |b| {
        b.iter(|| fib::fibonacci_rec(black_box(N)))
    });
}

fn bench_iter_fibonacci(c: &mut Criterion) {
    // create a benchmark group
    let mut group = c.benchmark_group("Fibonacci");
    // set the measurement time
    group.measurement_time(Duration::from_secs(DEFAULT_DURATION_SECS));
    // configure the sample size
    group.sample_size(50);
    //
    BATCHES.iter().copied().for_each(|n| {
        group.bench_with_input(BenchmarkId::new("Fibonacci", n), &n, |b, &x| {
            b.iter_batched(
                fib::Fibonacci::new,
                |mut fib| {
                    black_box(fib.compute(x));
                },
                BatchSize::SmallInput,
            );
        });
    });
    // finish the benchmark
    group.finish();
}
// initialize the benchmark group
criterion_group! {
    benches,
    bench_simple_fibonacci,
    bench_iter_fibonacci,
    bench_recursive_fibonacci,
}
// This macro expands to a function named `benches`, which uses the given config
criterion_main!(benches);

pub mod fib {
    //! various implementations of the fibonacci sequence
    //!
    //! ##_Definition_:
    //!
    //! $F(0) = F(1) = 1 \text{ and } F(n+1) = F(n) + F(n-1) | \forall: n > 0$

    /// a simple implementation of the fibonacci sequence for benchmarking purposes
    /// **Warning:** This will overflow the 128-bit unsigned integer at n=186
    pub fn fibonacci_nth(n: usize) -> u128 {
        // Use a and b to store the previous two values in the sequence
        let mut a = 0;
        let mut b = 1;
        (0..n).for_each(|_| {
            // As we iterate through, move b's value into a and the new computed
            // value into b.
            let c = a + b;
            a = b;
            b = c;
        });
        b
    }
    /// a recursive implementation of the fibonacci sequence
    pub fn fibonacci_rec(n: usize) -> u128 {
        const fn _inner(n: usize, previous: u128, current: u128) -> u128 {
            match n {
                0 | 1 => current,
                _ => _inner(n - 1, current, current + previous),
            }
        }
        // Call the actual tail recursive implementation, with the extra
        // arguments set up.
        _inner(n, 0, 1)
    }
    /// A structural implementation of the fibonacci sequence that leverages the
    /// [`iter`](core::iter) as its backend
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct Fibonacci {
        curr: u32,
        next: u32,
    }

    impl Fibonacci {
        pub fn new() -> Fibonacci {
            Fibonacci { curr: 0, next: 1 }
        }

        pub fn compute(&mut self, n: usize) -> u32 {
            if let Some(res) = self.nth(n + 1) {
                return res;
            }
            panic!("Unable to compute the nth value of the fibonacci sequence...")
        }

        pub fn set_curr(&mut self, curr: u32) -> &mut Self {
            self.curr = curr;
            self
        }

        pub fn set_next(&mut self, next: u32) -> &mut Self {
            self.next = next;
            self
        }

        pub fn reset(&mut self) -> &mut Self {
            self.set_curr(0).set_next(1)
        }

        fn compute_next(&self) -> u32 {
            self.curr + self.next
        }
    }

    impl Default for Fibonacci {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Iterator for Fibonacci {
        type Item = u32;

        fn next(&mut self) -> Option<u32> {
            use core::mem::replace;
            let new_next = self.compute_next();
            let new_curr = replace(&mut self.next, new_next);

            Some(replace(&mut self.curr, new_curr))
        }
    }
}
