use sliding_window_alt::SlidingWindow;

#[derive(Clone, Copy, Debug, Default)]
struct State {
    sys: f64,
    action: f64,
    error: f64,
}

fn main() {
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
    let mut store = SlidingWindow::new(100, State::default());
    for _ in 1..=100 {
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

    let violent = store.iter().any(|state| state.action.abs() > 20.0);

    let resp_bound = store.iter().all(|state| state.sys < 20.0);

    let is_fast = if let Some(i) = store.iter().rev().position(|state| state.error.abs() < 0.1) {
        i < 20
    } else {
        false
    };

    println!("has violent actuations: {violent}");
    println!("respects bounds: {resp_bound}");
    println!("is fast: {is_fast}");
}
