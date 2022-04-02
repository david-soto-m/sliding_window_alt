use sliding_window_alt::SlidingWindow;

fn main() {
    let mut st = SlidingWindow::new(5, 0);
    for i in 1..=10 {
        st.push(i);
    }
    assert_eq!(st, [10, 9, 8, 7, 6]);

    let a = vec![1, 2, 3, 4];
    let st = SlidingWindow::from(&a[..]);
    assert!(st == &a[..]);

    let a = [4; 5];
    let st = SlidingWindow::from(a);
    assert!(st == a);

    let _ = SlidingWindow::from_iter(a.iter());

    let a = [4; 5];
    let mut st: SlidingWindow<u8> = a.into();
    st.push_slice(&[1, 2, 3, 4, 5, 6]);
    // the six is ignored
    let a = [1, 2, 3, 4, 5];
    assert_eq!(st, a);
    assert_ne!(st, [1, 2, 3, 4, 5, 6]);

    // Iterators
    st.iter().zip(a).for_each(|(e, z)| {
        assert_eq!(z, *e);
    });

    // modify st
    st.iter_mut().map(|item| *item *= *item).count();
    st.iter()
        .zip(a)
        .for_each(|(it, a_it)| assert_eq!(a_it * a_it, *it));

    let st: SlidingWindow<u8> = a.into();
    assert!(a
        .iter()
        .enumerate()
        .all(|(index, value)| { *value == st[index] }));

    let b = st.to_vec();
    assert_eq!(b, a);

    let mut idx = 0;
    for el in &st {
        // calling into_iter() for &st, non consuming
        // (it consumes the reference) O(n)
        assert_eq!(el, a[idx]);
        idx += 1;
    }
    assert_eq!(5, st.capacity());

    let mut idx = 0;
    for el in st {
        // implicitly calling into_iter() and consuming st O(n)
        assert_eq!(el, a[idx]);
        idx += 1;
    }
    //st is now consumed

    let mut st1 = SlidingWindow::from([0, 1, 2, 3]);
    st1.push(2);
    let st2 = SlidingWindow::from([2, 0, 1, 2]);
    assert_eq!(st2, st1);
}
