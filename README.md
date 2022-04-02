# What

This is a sliding window data structure. Inside it there is a vector that has
at position `0` the newest element and at position `capacity` the oldest.
Whenever you push an item, said item is stored at position `0`, everything
shifts, and the last position is forgotten.

For some speed gain, this is not how it internally works. Internall, it has an
index that moves, always pointing to the oldest element in the set. This way
insertions are O(1).

You can get the information out in three ways, by getting an iterator, by
getting a `Vec<T>`, or by directly indexing

# Why

The following crates are fairly similar:

[Queues](https://crates.io/crates/queues) has the type CircularBuffer. It has
several problems. The first one is that it has O(n) insertions. The way less
worrying one is that it only get the newest item back. On the other hand, it
expresses the operations of an abstract queue as a trait, which is good. It has
no testing. This is most probably not what you were looking for when you
searched for a sliding window.

[circular_queue](https://crates.io/crates/circular-queue) is pretty much the
same as this crate, except that this crate doesn't provide several possible
orders, but provide more possible comparisons and ways of creating and pushing
and a vector return. It doesn't allow indexing, which this create does, mostly
inspired by...

[sliding_window](https://crates.io/crates/sliding_window) has a similar
functionality as `circular_queue`, but is `no_std` and has some unsafe code. It
allows indexing where `0` is the oldest. This is probably the crate to go for
speed and portability.

However, I have published this crate. My reasons are:

1. It is always filled from the creations, always returning same sized iterators
and vectors. This is specially useful for mathematical manipulations. This
would be a breaking change for the other crates, and is the reason there is a
new crate instead of a contribution to those crates.

1. It adds on to `circular_queue` indexing, vector return, and a bunch of
`PartialEqs` implementations.

1. It is 100% safe.

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

## Declaring

There are three ways of creating a sliding window. One with new, and two froms.
You can create from an array, (not Vec!!) or a slice.

## Converting

Both `&SlidingWindow` and `SlidingWindow` `into_iter` methods allow for use in
for loops, however, both of **these methods are O(n)**.

## Pushing

The `push` method pushes one item and the `push_slice` an array of items, being
the item at position **`0` the "youngest"** of the items.

## Iterating

The methods `iter` and `iter_mut` provide iterators that start at the newest
item.

## Indexing

You can access the contents of your sliding window by indexing, it. `0` is the
newest element and `capacity` is the oldest.

## Vector

If you need to perform some analysis for your application, you can obtain a
vector from the `sliding_window`. **This operation is O(n)**, use it when you
genuinely need a vector.

