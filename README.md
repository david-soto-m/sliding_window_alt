# What

This is a sliding window data structure. Inside it there is a vector that has
at position `0` the newest element and at position `len()` the oldest. Whenever
you push an item, said item is stored at position `0`, everything shifts, and
the last position is forgotten.

For some speed gain, this is not how it internally works. Internally it has an
index that moves, always pointing to the oldest element in the set. This way
insertions are O(1).

You can get the information out in two ways, by getting an iterator or by
getting a `Vec<T>`.

# Why

The following crates are fairly similar:

[Queues](https://crates.io/crates/queues) has the type CircularBuffer. It has
several problems:

1. O(n) insertions
1. Uninitialized default
1. Only gets one item at a time

[Circular_queue](https://crates.io/crates/circular-queue)

1. Uninitialized default
1. Can't get an ordered vector for mathematical operations that may require them

[sliding_window](https://crates.io/crates/sliding_window)

1. Uninitialized default
1. Can't get an ordered vector for more mathematical operations.

# How
```rust
use sliding_window_alt::SlidingWindow;

fn main(){
    SlidingWindow::new(5, 4);
}

```
