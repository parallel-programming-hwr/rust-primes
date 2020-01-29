use may::go;
use may::sync::mpmc::{channel, Sender};
use num_cpus;
use std::env;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufWriter;
use std::time::Instant;

fn main() {
    may::config().set_workers(num_cpus::get());
    let (tx, rx) = channel::<u64>();
    let num_threads: u16 = num_cpus::get() as u16;
    let mut start: u64 = 1;
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        start = args[1].parse::<u64>().unwrap();
    }
    if &start % 2 == 0 {
        start += 1;
    }
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("primes.txt")
        .unwrap();
    let mut buffer = BufWriter::new(file);
    println!("Starting {} threads", num_threads);
    for i in 0u16..num_threads {
        let tx = tx.clone();
        go!(move || {
            get_primes(start + (2 * &i) as u64, (&num_threads * 2) as u64, &tx);
        });
        println!("Started thread {}", i);
    }
    let time_start = Instant::now();
    let mut prime_count = 0;
    loop {
        let prime = rx.recv().unwrap();
        prime_count += 1;
        println!("\r{: <30}", prime);
        print!(
            "{} Primes/s",
            prime_count as f64 / time_start.elapsed().as_secs_f64()
        );
        if let Err(e) = buffer.write(&format!("{}\n", prime).into_bytes()) {
            panic!(e);
        }
    }
}

fn get_primes(start: u64, incr: u64, tx: &Sender<u64>) {
    println!("Hi, I'm a thread.");
    let mut num = start;
    loop {
        let mut is_prime = true;
        if (num < 3) | (&num % 2 == 0) {
            num += incr;
            continue;
        }
        for i in (3u64..&num / 2).step_by(2) {
            if num % i == 0 {
                is_prime = false;
            }
        }
        if is_prime {
            tx.send(num).unwrap();
        }
        num += incr;
    }
}
