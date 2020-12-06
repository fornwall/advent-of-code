#![feature(test, const_fn)]
#![allow(clippy::zero_prefixed_literal)]
extern crate test;

use advent_of_code::solve;
use paste::paste;
#[cfg(feature = "count-allocations")]
use std::alloc::System;
use std::fs::read_to_string;
use test::Bencher;

#[cfg(feature = "count-allocations")]
mod counting_allocator {
    use backtrace::Backtrace;
    use std::alloc::{GlobalAlloc, Layout};
    use std::sync::atomic::{AtomicU64, Ordering};

    /// A memory allocator which wraps another and counts allocations.
    ///
    /// It can optionally fail with a backtrace after a specific number of allocations.
    ///
    /// https://www.reddit.com/r/rust/comments/8z83wc/is_there_any_way_to_benchmark_memory_usage_in_rust/
    /// RUSTFLAGS="-Cdebuginfo=1" cargo +nightly-2020-11-21 bench problem_2020 --features count-allocations -- --nocapture
    pub struct CountingAllocator<A: GlobalAlloc> {
        pub wrapped_allocator: A,
        allocator_count: AtomicU64,
        start_failing_after: AtomicU64,
    }

    unsafe impl<A: GlobalAlloc> GlobalAlloc for CountingAllocator<A> {
        unsafe fn alloc(&self, l: Layout) -> *mut u8 {
            let previous_count = self.allocator_count.fetch_add(1u64, Ordering::SeqCst);
            if previous_count >= self.start_failing_after.load(Ordering::SeqCst) {
                self.start_failing_after.store(u64::MAX, Ordering::SeqCst);
                println!("Allocation back trace: {:?}", Backtrace::new());
                panic!("Aborting after found allocation");
            }

            self.wrapped_allocator.alloc(l)
        }

        unsafe fn dealloc(&self, ptr: *mut u8, l: Layout) {
            self.wrapped_allocator.dealloc(ptr, l);
        }
    }

    impl<A: GlobalAlloc> CountingAllocator<A> {
        pub const fn new(a: A) -> Self {
            Self {
                wrapped_allocator: a,
                allocator_count: AtomicU64::new(0),
                start_failing_after: AtomicU64::new(u64::MAX),
            }
        }

        pub fn reset(&self) {
            self.allocator_count.store(0, Ordering::SeqCst);
            self.start_failing_after.store(u64::MAX, Ordering::SeqCst);
        }

        pub fn reset_and_fail_after(&self, fail_after: u64) {
            self.start_failing_after.store(fail_after, Ordering::SeqCst);
            self.allocator_count.store(0, Ordering::SeqCst);
        }

        pub fn get(&self) -> u64 {
            self.allocator_count.load(Ordering::SeqCst)
        }
    }
}

#[global_allocator]
#[cfg(feature = "count-allocations")]
static GLOBAL: counting_allocator::CountingAllocator<System> =
    counting_allocator::CountingAllocator::new(System);

fn solve_parts(b: &mut Bencher, year: u16, day: u8, part: u8) {
    #![allow(clippy::unwrap_used)]
    let input_path = format!("src/year{}/day{:02}_input.txt", year, day);
    let input = read_to_string(input_path).unwrap();

    b.iter(|| {
        #[cfg(feature = "count-allocations")]
        GLOBAL.reset_and_fail_after(1);

        solve(year, day, part, &input).unwrap();

        #[cfg(feature = "count-allocations")]
        {
            GLOBAL.reset();
        }
    });
}

macro_rules! run_bench {
    ($year: literal, $day: literal) => {
        paste! {
           #[bench]
            fn [<problem_ $year _ $day _ part1>](b: &mut Bencher) {
                solve_parts(b, $year, $day, 1);
            }
           #[bench]
            fn [<problem_ $year _ $day _ part2>](b: &mut Bencher) {
                solve_parts(b, $year, $day, 2);
            }
        }
    };
}

run_bench!(2017, 01);
run_bench!(2017, 02);
run_bench!(2017, 03);
run_bench!(2017, 04);
run_bench!(2017, 05);
run_bench!(2017, 06);
run_bench!(2017, 07);
run_bench!(2017, 08);
run_bench!(2017, 09);
run_bench!(2017, 10);
run_bench!(2017, 11);
run_bench!(2017, 12);
run_bench!(2017, 13);
run_bench!(2017, 14);
run_bench!(2017, 15);
run_bench!(2017, 16);

run_bench!(2018, 01);
run_bench!(2018, 02);
run_bench!(2018, 03);
run_bench!(2018, 04);
run_bench!(2018, 05);
run_bench!(2018, 06);
run_bench!(2018, 07);
run_bench!(2018, 08);
run_bench!(2018, 09);
run_bench!(2018, 10);
run_bench!(2018, 11);
run_bench!(2018, 12);
run_bench!(2018, 13);
run_bench!(2018, 14);
run_bench!(2018, 15);
run_bench!(2018, 16);
run_bench!(2018, 17);
run_bench!(2018, 18);
run_bench!(2018, 19);
run_bench!(2018, 20);
run_bench!(2018, 21);
run_bench!(2018, 22);
run_bench!(2018, 23);
run_bench!(2018, 24);
run_bench!(2018, 25);

run_bench!(2019, 01);
run_bench!(2019, 02);
run_bench!(2019, 03);
run_bench!(2019, 04);
run_bench!(2019, 05);
run_bench!(2019, 06);
run_bench!(2019, 07);
run_bench!(2019, 08);
run_bench!(2019, 09);
run_bench!(2019, 10);
run_bench!(2019, 11);
run_bench!(2019, 12);
run_bench!(2019, 13);
run_bench!(2019, 14);
run_bench!(2019, 15);
run_bench!(2019, 16);
run_bench!(2019, 17);
run_bench!(2019, 18);
run_bench!(2019, 19);
run_bench!(2019, 20);
run_bench!(2019, 21);
run_bench!(2019, 22);
run_bench!(2019, 23);
run_bench!(2019, 24);
run_bench!(2019, 25);

run_bench!(2020, 1);
run_bench!(2020, 2);
run_bench!(2020, 3);
run_bench!(2020, 4);
run_bench!(2020, 5);
run_bench!(2020, 6);
run_bench!(2020, 7);
