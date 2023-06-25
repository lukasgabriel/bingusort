extern crate clap;
extern crate rand;

use clap::{App, Arg};
use rand::Rng;
use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;

const BINGUS_MILLIS: u64 = 2500;

fn main() {
    let matches = App::new("BINGUS Sort")
        .arg(
            Arg::with_name("verbose")
                .short('v')
                .help("Activates verbose mode"),
        )
        .arg(
            Arg::with_name("length")
                .short('l')
                .help("Specifies length of the list")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("array")
                .short('a')
                .help("Input list to be sorted")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("slow")
                .short('s')
                .help("Activates slow mode, which inserts pauses between verbose outputs of specified length (in ms). Does nothing when used without -v / --verbose")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("bingus")
                .short('b')
                .help("Activates BINGUS mode. Just see for yourself.")
        )
        .get_matches();

    let verbose = matches.is_present("verbose");
    let bingus = matches.is_present("bingus");

    let mut slow_secs: u64 = 0;
    if matches.is_present("slow") {
        slow_secs = matches.value_of("slow").unwrap().parse::<u64>().unwrap();
    }

    let mut length = 10;
    if matches.is_present("length") {
        length = matches
            .value_of("length")
            .unwrap()
            .parse::<usize>()
            .unwrap();
    }

    let mut list = if matches.is_present("array") {
        matches
            .value_of("array")
            .unwrap()
            .split(',')
            .map(|x| x.trim().parse::<i32>().unwrap())
            .collect()
    } else {
        let mut rng = rand::thread_rng();
        (0..length).map(|_| rng.gen_range(1..100)).collect()
    };

    println!("Initial list: {:?}", list);

    let start = Instant::now();
    let mut operations = 0;
    while !is_sorted(&list) {
        bounce(&mut list, verbose, bingus);
        inspect_and_nudge(&mut list, verbose, bingus);
        glimpse_and_upgrade(&mut list, verbose, bingus);
        shuffle(&mut list, verbose, bingus);

        operations += 4;
        if verbose {
            println!("List after {} operations: {:?}", operations, list);
            if matches.is_present("slow") {
                sleep(Duration::from_millis(slow_secs));
            }
        }
        if bingus {
            println!("Bingus has made {} moves so far.", operations);
            println!("Bingus looks at the list and contemplates it: {:?}", list);
            sleep(Duration::from_millis(BINGUS_MILLIS));
        }
    }
    let elapsed = start.elapsed();

    println!("Sorted list: {:?}", list);
    println!("Operations: {}", operations);
    println!("Time taken: {:.2?}", elapsed);
}

fn is_sorted(list: &Vec<i32>) -> bool {
    for i in 1..list.len() {
        if list[i - 1] > list[i] {
            return false;
        }
    }
    true
}

fn bounce(list: &mut Vec<i32>, verbose: bool, bingus: bool) {
    let mut rng = rand::thread_rng();
    let i = rng.gen_range(0..list.len());
    let j = rng.gen_range(0..list.len());

    if list[i] > list[j] {
        list.swap(i, j);
    }

    if verbose {
        println!("  Bounce: swapped elements at positions {} and {}", i, j);
        println!("  {:?}", list);
    }
    if bingus {
        println!(
            "  Bingus picks two random elements ({} and {}) and BOUNCES them on each other.",
            i, j
        );
        println!("  {:?}", list);
        sleep(Duration::from_millis(BINGUS_MILLIS));
    }
}

fn inspect_and_nudge(list: &mut Vec<i32>, verbose: bool, bingus: bool) {
    let mut max_index = 0;
    let mut max_value = list[0];

    for i in 1..list.len() {
        if list[i] > max_value {
            max_index = i;
            max_value = list[i];
        }
    }

    if verbose {
        println!(
            "  Inspect: largest value is {} at index {}",
            max_value, max_index
        );
    }
    if bingus {
        println!(
            "  Bingus INSPECTS the list and finds the largest element {} at position {}.",
            max_value, max_index
        );
    }

    while max_index < list.len() - 1 {
        list.swap(max_index, max_index + 1);
        max_index += 1;
    }

    if verbose {
        println!("  Nudge: moved largest value to the end");
        println!("  {:?}", list);
    }
    if bingus {
        println!("  Bingus NUDGES the largest element to the end of the list.");
        println!("  {:?}", list);
        sleep(Duration::from_millis(BINGUS_MILLIS));
    }
}

fn glimpse_and_upgrade(list: &mut Vec<i32>, verbose: bool, bingus: bool) {
    let mut min_index = 0;
    let mut min_value = list[0];

    for i in 1..list.len() {
        if list[i] < min_value {
            min_index = i;
            min_value = list[i];
        }
    }

    if verbose {
        println!(
            "  Glimpse: smallest value is {} at index {}",
            min_value, min_index
        );
        println!("  {:?}", list);
    }
    if bingus {
        println!("  Bingus takes a close look at the list and catches a GLIMPSE of the smallest element {} at position {}", min_value, min_index);
        println!("  {:?}", list);
        sleep(Duration::from_millis(BINGUS_MILLIS));
    }

    while min_index > 0 {
        list.swap(min_index, min_index - 1);
        min_index -= 1;
    }

    if verbose {
        println!("  Upgrade: moved smallest value to the start");
        println!("  {:?}", list);
    }
    if bingus {
        println!("  Bingus UPGRADES the smallest element and moves it to the start of the list.");
        println!("  {:?}", list);
        sleep(Duration::from_millis(BINGUS_MILLIS));
    }
}

fn shuffle(list: &mut Vec<i32>, verbose: bool, bingus: bool) {
    let bingus_positions = [2, 9, 14, 7, 21, 19]
        .iter()
        .map(|&x| x % list.len())
        .filter(|&x| x != 0)
        .collect::<Vec<usize>>();

    if verbose {
        println!("  Shuffle: the BINGUS positions are {:?}", bingus_positions);
        println!("  {:?}", list);
    }
    if bingus {
        println!("  Bingus picks two letters from his name and performs modular arithmetic on their numerical values: {:?}", bingus_positions);
        println!("  {:?}", list);
        sleep(Duration::from_millis(BINGUS_MILLIS));
    }

    let mut rng = rand::thread_rng();
    for i in 0..bingus_positions.len() {
        let j = rng.gen_range(i..bingus_positions.len());
        list.swap(bingus_positions[i], bingus_positions[j]);
    }

    if verbose {
        println!("  Shuffle: shuffled the elements at the BINGUS positions");
        println!("  {:?}", list);
    }
    if bingus {
        println!("  Bingus gets frustrated and SHUFFLES these elements around.");
        println!("  {:?}", list);
        sleep(Duration::from_millis(BINGUS_MILLIS));
    }
}
