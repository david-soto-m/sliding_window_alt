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
several problem. The first one is that it has O(n) insertions. The way less
worring one is that it only gets one item at a time and its the newest. On the
other hand it expreses the operations of an abstract queue as a trait, which is
good. It has no testing. This is most probably not what you where looking for
when you searched for a sliding window.

[circular_queue](https://crates.io/crates/circular-queue) is pretty much the
same as this crate except that I don't provide several possible orders, but
provide more possible comparisons and ways of creating and pushing. It doesn't
allow indexing which I do, mostly inspired by...

[sliding_window](https://crates.io/crates/sliding_window) Has a similar
functionality as circular_queue but is no_std and has some unsafe code. It
allows indexing and `0` is the oldest. This is probably the crate to go for
speed and portability.

However I have published this crate. My reasons are:

1. It is always filled from the creations, always returning same sized iterators
and vectors. This is specially usefull for mathematical manipulations.

1. It adds on to circular_queue indexing, vector return, and a bunch of
PartialEqs implementations.

1. It is 100% safe.

It must be noted that returning a vector is O(n), and so are the into_iter
methods, due to complications with lifetimes I'm not smart enough to solve.

# How

There are several examples in the examples folders. Here is some code to get a
feel for the library.

```rust
use sliding_window_alt::SlidingWindow;

fn main() {
    let mut sys = SlidingWindow::new(5, 0.0);

    // caracteristical polynomial of the system, it's stable
    let carac_pol = [0.5, -0.4, 0.2, -0.3, 0.05];

    for _ in 0..=100 {
        sys.push(
            sys.iter()
                .zip(carac_pol)
                .map(|(item, coef)| coef * *item)
                .sum::<f64>()
                + 1.0,
        );
        println!("{}", sys[0]);
    }
}
```

## declaring

## converting
***WARNING*** this operations are both O(n).

## pushing

## iterating

## indexing

## vector

