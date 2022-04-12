use sliding_window_alt::SlidingWindow;

fn main() {
    let mut sys = SlidingWindow::new(5, 0.0);

    // caracteristical polynomial of the system, it's stable
    let carac_pol = [0.5, -0.4, 0.2, -0.3, 0.05];

    for _ in 1..=100 {
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
