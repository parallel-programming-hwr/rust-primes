extern crate num_cpus;
use std::thread;
use std::fs::{OpenOptions};
use std::io::prelude::*;
use std::sync::mpsc::{sync_channel, SyncSender};
use std::env;

fn main() {
    let (tx, rx) = sync_channel::<u64>(0);
    let num_threads: u64 = num_cpus::get() as u64;
    let mut start: u64 = 1;
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        start = *(&args[1].parse::<u64>().unwrap());
    }
    if &start % 2 == 0 {
        start += 1;
    }
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("primes.txt")
        .unwrap();
    println!("Starting {} threads", num_threads);
    for i in 0u64..num_threads {
        let tx = tx.clone();
        let _child = thread::spawn(move || {
            get_primes(&start + (2*&i), &num_threads * 2, &tx);
        });
    }
    loop {
        let prime = rx.recv().unwrap();
        println!("{}", prime);
        if let Err(e) = writeln!(file, "{}", prime) {
            panic!(e);
        }
    }
}

fn get_primes(start: u64, incr: u64, tx: &SyncSender<u64>) {
    let mut num = start;
    loop {
        let mut is_prime = true;
        if (num < 3) | (&num % 2 == 0) {
            num += incr;
            continue;
        }
        for i in (3u64..&num/2).step_by(2) {
            if &num % i == 0 {
                is_prime = false;
            }
        }
        if is_prime {
            if let Err(e) = (*tx).send(num) {
                panic!(e);
            }
        }
        num += incr;
    }
}