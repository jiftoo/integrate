pub fn dec_to_fraction(n: f64) -> (u64, u64) {
    find_convergents(&continued_fraction(n))
}

fn continued_fraction(mut n: f64) -> Vec<u64> {
    const MAX_STEPS: u32 = 16;
    let mut coef: Vec<u64> = Vec::new();
    for _ in 0..MAX_STEPS {
        let i = n.floor() as u64;
        coef.push(i);
        let f = n - i as f64;
        if f <= 1e-6 {
            break;
        } else {
            n = 1.0 / f;
        }
    }
    return coef;
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
