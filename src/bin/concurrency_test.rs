use std::time::{Duration, Instant};

use dmf::concurrency::pending::Pending;

// 0 1 1 2 3 5 8 13 21

pub fn fib(n: u64) -> u128 {
    match n {
        0 => 0,
        n => {
            let (mut lhs, mut rhs) = (0_u128, 1_u128);
            for _ in 1..n {
                (lhs, rhs) = (rhs, lhs + rhs);
            }
            rhs
        }
    }
}

pub trait OptDrop {
    fn drop(&mut self);
}

impl<T> OptDrop for Option<T> {
    fn drop(&mut self) {
        drop(self.take())
    }
}

pub fn main() {
    println!("Spawning worker.");
    let start_time = Instant::now();
    let pending = Pending::spawn(|| {
        // let mut work = 0_u128;
        // const ITERATIONS: usize = 1000*10000;
        // for _ in 0..ITERATIONS {
        //     for _ in 0..100 {
        //         work = work.wrapping_add(fib(std::hint::black_box(100)));
        //     }
        // }
        // (work, String::from("Hello, world!"))
        std::thread::sleep(Duration::from_millis(3000));
        String::from("This is the return value from the other thread.")
    });
    let result = loop {
        dmf::break_ok!(pending.try_recv());
        let elapsed = start_time.elapsed();
        println!("Waiting... {elapsed:.3?}");
        std::thread::sleep(Duration::from_millis(50));
    };
    println!("Result: {result:?}");

    let elapsed = dmf::time::time_it(|| {
        std::thread::sleep(Duration::from_millis(500));
    }).elapsed;
    println!("time_it time: {elapsed:?}");
}