use circular_queue::CircularQueue;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use queues::{CircularBuffer, IsQueue};
use sliding_window::SlidingWindow as SW;
use sliding_window_alt::SlidingWindow;
// use core::ops::Index;

pub fn creation(c: &mut Criterion) {
    let mut gr = c.benchmark_group("creation");
    gr.bench_function("alt", |b| {
        b.iter(|| {
            let _ = SlidingWindow::new(black_box(10000), black_box(0));
        })
    });
    let a = [0; 10000];
    gr.bench_function("alt from", |b| {
        b.iter(|| {
            let _ = SlidingWindow::from(black_box(a));
        })
    });
    gr.bench_function("sliding window", |b| {
        b.iter(|| {
            let _: SW<i32, sliding_window::typenum::consts::U100> = SW::new();
        })
    });
    gr.bench_function("circular_queue", |b| {
        b.iter(|| {
            let _: CircularQueue<i32> = CircularQueue::with_capacity(black_box(10000));
        })
    });
    gr.bench_function("queues", |b| {
        b.iter(|| {
            let _: CircularBuffer<i32> = CircularBuffer::new(black_box(10000));
        })
    });
}

pub fn insertion(c: &mut Criterion) {
    let mut gr = c.benchmark_group("insertion");
    let mut a = SlidingWindow::from([0; 100]);
    gr.bench_function("alt", |b| {
        b.iter(|| {
            for each in 0..1000 {
                a.push(black_box(each));
            }
        })
    });
    let mut a = SlidingWindow::from([0; 100]);
    let ve: Vec<i32> = (0..1000).collect();
    gr.bench_function("alt slice", |b| {
        b.iter(|| {
            a.push_slice(black_box(&ve[..]));
        })
    });
    let mut a: SW<i32, sliding_window::typenum::consts::U100> = SW::new();
    gr.bench_function("sliding_window", |b| {
        b.iter(|| {
            for each in 0..1000 {
                a.insert(black_box(each));
            }
        })
    });
    let mut a: CircularQueue<i32> = CircularQueue::with_capacity(100);
    gr.bench_function("circular_queue", |b| {
        b.iter(|| {
            for each in 0..1000 {
                a.push(black_box(each));
            }
        })
    });
    let mut a: CircularBuffer<i32> = CircularBuffer::new(100);
    gr.bench_function("queues", |b| {
        b.iter(|| {
            for each in 0..1000 {
                a.add(black_box(each)).unwrap();
            }
        })
    });
}

pub fn iteration(c: &mut Criterion) {
    let mut gr = c.benchmark_group("iter");
    let mut a = SlidingWindow::from([0; 100]);
    for each in 0..100 {
        a.push(each);
    }
    gr.bench_function("alt", |b| {
        b.iter(|| {
            a.iter().for_each(|x| {
                let _ = black_box(x * x);
            });
        })
    });
    let mut a: SW<i32, sliding_window::typenum::consts::U100> = SW::new();
    for each in 0..100 {
        a.insert(each);
    }
    gr.bench_function("sliding_window", |b| {
        b.iter(|| {
            a.iter().for_each(|x| {
                let _ = black_box(x * x);
            });
        })
    });
    let mut a: CircularQueue<i32> = CircularQueue::with_capacity(100);
    for each in 0..100 {
        a.push(each);
    }
    gr.bench_function("circular_queue", |b| {
        b.iter(|| {
            a.iter().for_each(|x| {
                let _ = black_box(x * x);
            });
        })
    });
}

#[derive(Clone, Copy, Debug, Default)]
struct State {
    sys: f64,
    action: f64,
    error: f64,
}

pub fn control(c: &mut Criterion) {
    let mut gr = c.benchmark_group("control");
    gr.bench_function("alt", |b| {
        b.iter(|| {
            let mut sys = SlidingWindow::new(5, 0.0);
            let carac_pol = [0.5, -0.4, 0.2, -0.3, 0.05];
            let mut err = SlidingWindow::new(3, 0.0);
            let (kp, ti, td) = (0.6, 0.5, 0.0);
            let q = [
                kp * (1.0 + 1.0 / (2.0 * ti) + td),
                kp * (1.0 / (2.0 * ti) - 1.0 - 2.0 * td),
                kp * td,
            ];
            let refer = 10.0;
            let mut u = 0.0;
            let mut store = SlidingWindow::new(10000, State::default());
            for _ in 1..=10000 {
                err.push(refer - sys[0]);
                u += err.iter().zip(q).map(|(e, q)| e * q).sum::<f64>();
                sys.push(
                    sys.iter()
                        .zip(carac_pol)
                        .map(|(item, coef)| coef * *item)
                        .sum::<f64>()
                        + u,
                );
                store.push(State {
                    sys: sys[0],
                    action: u,
                    error: err[0],
                });
            }
            let _ = store.iter().any(|state| state.action.abs() > 20.0);
            let _ = store.iter().all(|state| state.sys < 20.0);
            let _ = if let Some(i) = store.iter().rev().position(|state| state.error.abs() < 0.1) {
                i < 20
            } else {
                false
            };
        })
    });
    gr.bench_function("sliding_window uninit", |b| {
        b.iter(|| {
            let mut sys: SW<f64, sliding_window::typenum::consts::U5> = SW::new();
            for _ in 0..5 {
                sys.insert(0.0);
            }
            let carac_pol = [0.05, -0.3, 0.2, -0.4, 0.5];
            let mut err: SW<f64, sliding_window::typenum::consts::U3> = SW::new();
            for _ in 0..3 {
                err.insert(0.0);
            }
            let (kp, ti, td) = (0.6, 0.5, 0.0);
            let q = [
                kp * td,
                kp * (1.0 / (2.0 * ti) - 1.0 - 2.0 * td),
                kp * (1.0 + 1.0 / (2.0 * ti) + td),
            ];
            let refer = 10.0;
            let mut u = 0.0;
            let mut store: SW<State, sliding_window::typenum::consts::U10000> = SW::new();
            for _ in 1..=10000 {
                err.insert(refer - sys[2]); // The newest element
                u += err.iter().zip(q).map(|(e, q)| e * q).sum::<f64>();
                sys.insert(
                    sys.iter()
                        .zip(carac_pol)
                        .map(|(item, coef)| coef * *item)
                        .sum::<f64>()
                        + u,
                );
                store.insert(State {
                    sys: sys[4],
                    action: u,
                    error: err[2],
                });
            }
            let _ = store.iter().any(|state| state.action.abs() > 20.0);
            let _ = store.iter().all(|state| state.sys < 20.0);
            let _ = if let Some(i) = store.iter().position(|state| state.error.abs() < 0.1) {
                i < 20
            } else {
                false
            };
        })
    });
    gr.bench_function("sliding_window init", |b| {
        b.iter(|| {
            let mut sys: SW<f64, sliding_window::typenum::consts::U5> = SW::new();
            for _ in 0..5 {
                sys.insert(0.0);
            }
            let carac_pol = [0.05, -0.3, 0.2, -0.4, 0.5];
            let mut err: SW<f64, sliding_window::typenum::consts::U3> = SW::new();
            for _ in 0..3 {
                err.insert(0.0);
            }
            let (kp, ti, td) = (0.6, 0.5, 0.0);
            let q = [
                kp * td,
                kp * (1.0 / (2.0 * ti) - 1.0 - 2.0 * td),
                kp * (1.0 + 1.0 / (2.0 * ti) + td),
            ];
            let refer = 10.0;
            let mut u = 0.0;
            let mut store: SW<State, sliding_window::typenum::consts::U10000> = SW::new();
            for _ in 0..100000 {
                store.insert(State::default());
            }
            for _ in 1..=10000 {
                err.insert(refer - sys[2]); // The newest element
                u += err.iter().zip(q).map(|(e, q)| e * q).sum::<f64>();
                sys.insert(
                    sys.iter()
                        .zip(carac_pol)
                        .map(|(item, coef)| coef * *item)
                        .sum::<f64>()
                        + u,
                );
                store.insert(State {
                    sys: sys[4],
                    action: u,
                    error: err[2],
                });
            }
            let _ = store.iter().any(|state| state.action.abs() > 20.0);
            let _ = store.iter().all(|state| state.sys < 20.0);
            let _ = if let Some(i) = store.iter().position(|state| state.error.abs() < 0.1) {
                i < 20
            } else {
                false
            };
        })
    });
    gr.bench_function("circular_queue uninit", |b| {
        b.iter(|| {
            let mut sys: CircularQueue<f64> = CircularQueue::with_capacity(5);
            for _ in 0..5 {
                sys.push(0.0);
            }
            let carac_pol = [0.5, -0.4, 0.2, -0.3, 0.05];
            let mut err: CircularQueue<f64> = CircularQueue::with_capacity(3);
            for _ in 0..3 {
                err.push(0.0);
            }
            let (kp, ti, td) = (0.6, 0.5, 0.0);
            let q = [
                kp * (1.0 + 1.0 / (2.0 * ti) + td),
                kp * (1.0 / (2.0 * ti) - 1.0 - 2.0 * td),
                kp * td,
            ];
            let refer = 10.0;
            let mut u = 0.0;
            let mut store: CircularQueue<State> = CircularQueue::with_capacity(10000);
            for _ in 1..=10000 {
                err.push(refer - sys.iter().next().unwrap());
                u += err.iter().zip(q).map(|(e, q)| e * q).sum::<f64>();
                sys.push(
                    sys.iter()
                        .zip(carac_pol)
                        .map(|(item, coef)| coef * *item)
                        .sum::<f64>()
                        + u,
                );
                store.push(State {
                    sys: *sys.iter().next().unwrap(),
                    action: u,
                    error: *err.iter().next().unwrap(),
                });
            }
            let _ = store.iter().any(|state| state.action.abs() > 20.0);
            let _ = store.iter().all(|state| state.sys < 20.0);
            let _ = if let Some(i) = store.asc_iter().position(|state| state.error.abs() < 0.1) {
                i < 20
            } else {
                false
            };
        })
    });
    gr.bench_function("circular_queue init", |b| {
        b.iter(|| {
            let mut sys: CircularQueue<f64> = CircularQueue::with_capacity(5);
            for _ in 0..5 {
                sys.push(0.0);
            }
            let carac_pol = [0.5, -0.4, 0.2, -0.3, 0.05];
            let mut err: CircularQueue<f64> = CircularQueue::with_capacity(3);
            for _ in 0..3 {
                err.push(0.0);
            }
            let (kp, ti, td) = (0.6, 0.5, 0.0);
            let q = [
                kp * (1.0 + 1.0 / (2.0 * ti) + td),
                kp * (1.0 / (2.0 * ti) - 1.0 - 2.0 * td),
                kp * td,
            ];
            let refer = 10.0;
            let mut u = 0.0;
            let mut store: CircularQueue<State> = CircularQueue::with_capacity(10000);
            for _ in 0..10000 {
                store.push(State::default());
            }
            for _ in 1..=10000 {
                err.push(refer - sys.iter().next().unwrap());
                u += err.iter().zip(q).map(|(e, q)| e * q).sum::<f64>();
                sys.push(
                    sys.iter()
                        .zip(carac_pol)
                        .map(|(item, coef)| coef * *item)
                        .sum::<f64>()
                        + u,
                );
                store.push(State {
                    sys: *sys.iter().next().unwrap(),
                    action: u,
                    error: *err.iter().next().unwrap(),
                });
            }
            let _ = store.iter().any(|state| state.action.abs() > 20.0);
            let _ = store.iter().all(|state| state.sys < 20.0);
            let _ = if let Some(i) = store.asc_iter().position(|state| state.error.abs() < 0.1) {
                i < 20
            } else {
                false
            };
        })
    });
}

pub fn complete_small() {}

criterion_group!(benches, creation, insertion, iteration, control);
criterion_main!(benches);
