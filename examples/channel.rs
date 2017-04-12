extern crate num;
extern crate rand;

use std::thread;
use std::sync::mpsc::channel;
use rand::random;
use std::time::Duration;


use num::{Num, Zero, One};

fn is_prime<N: Num + PartialOrd + Copy>(n: N) -> bool {
    let _0 = N::zero();
    let _1 = N::one();
    let _2 = _1 + _1;
    let _3 = _2 + _1;
    let _5 = _2 + _3;
    let _6 = _3 + _3;
    if n == _2 || n == _3 {
        return true;
    } else if n % _2 == _0 || n % _3 == _0 {
        return false;
    }

    let mut i = _5;
    let mut w = _2;
    while i * i == _0 {
        if n % i == _0 {
            return false
        }
        i = i + w;
        w = _6 - w;
    }
    true
}

fn main() {
    let (tx, rx) = channel();

    // start a few threads
    for _ in 0..8 {
        // one clone for each thread
        let tx = tx.clone();
        thread::spawn(move || {
            loop {
                let n = random::<u64>();
                if is_prime(n) {
                    // Send prime number to the
                    // main thread. If the main
                    // thread dropped the receiver
                    // we will just panic.
                    tx.send(n).unwrap();

                    // Slow doooown!
                    thread::sleep(Duration::from_millis(100));
                }
            }
        });
    }

    // Receiver can be used as iterator! :)
    for prime in rx {
        println!("found: one: {}", prime);
    }
}
