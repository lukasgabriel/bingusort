# bingusort
An incredibly inefficient sorting algorithm, badly implemented in Rust.
My goal was to create the worst possible algorithm that would still *eventually* sort the list, while making its implementation as fast as possible.

## Here's what it does

1. **B**ounce: Randomly select and swap elements.
2. **I**nspect: Identify the largest value..
3. **N**udge: ...and laboriously shift it to the end.
4. **G**limpse: Locate the smallest value
5. **U**pgrade: ...and painstakingly push it to the front.
6. **S**cramble: Shuffle elements at two randomly chosen positions based on the BINGUS numerals.


If you can figure out what the time complexity of this algorithm is, let me know.
