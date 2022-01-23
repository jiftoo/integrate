pub fn dec_to_fraction(n: f64) -> (u64, u64) {
    find_convergents(&continued_fraction(n).take(16).collect::<Vec<u64>>())
}

fn continued_fraction(mut n: f64) -> impl Iterator<Item = u64> {
    let mut done = false;
    std::iter::from_fn(move || {
        if done {
            return None;
        }
        let i = n.floor();
        let f = n - i;
        if f <= 1e-6 {
            done = true;
        } else {
            n = 1.0 / f;
        }
        Some(i as u64)
    })
}

fn find_convergents(vec: &[u64]) -> (u64, u64) {
    fn p_n(vec: &[u64]) -> u64 {
        let len = vec.len();
        match vec {
            [] => 1,
            [x] => *x,
            [.., x] => x * p_n(&vec[..(len - 1)]) + p_n(&vec[..(len - 2)]),
        }
    }
    fn q_n(vec: &[u64]) -> u64 {
        let len = vec.len();
        match vec {
            [] => 0,
            [_] => 1,
            [.., x] => x * q_n(&vec[..(len - 1)]) + q_n(&vec[..(len - 2)]),
        }
    }
    (p_n(vec), q_n(vec))
}
