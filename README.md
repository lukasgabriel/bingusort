# bingusort
An incredibly inefficient sorting algorithm, badly implemented in Rust.

My goal was to create the worst possible algorithm that
- is more original than bogosort - as in, not just shuffling the array,
- would still *eventually* sort the list,
- while making its implementation as fast as possible.

[Engage Bingus mode](https://knowyourmeme.com/memes/bingus) with `-b` to make execution so slow that you can watch the program work in real time! 😎


## Usage
```
BINGUS Sort 

USAGE:
    bingusort [OPTIONS]

OPTIONS:
    -a <array>         Input list to be sorted
    -b                 Activates BINGUS mode. Just see for yourself.
    -h, --help         Print help information
    -l <length>        Specifies length of the list
    -s <slow>          Activates slow mode, which inserts pauses between verbose outputs of
                       specified length (in ms). Does nothing when used without -v / --verbose
    -v                 Activates verbose mode
```

## Here's what it does

1. **B**ounce: Randomly select and swap elements.
2. **I**nspect: Identify the largest value..
3. **N**udge: ...and laboriously shift it to the end.
4. **G**limpse: Locate the smallest value
5. **U**pgrade: ...and painstakingly push it to the front.
6. **S**cramble: Shuffle elements at two randomly chosen positions based on the BINGUS numerals.


If you can figure out what the time complexity of this algorithm is, let me know.
