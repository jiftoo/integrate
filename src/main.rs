use fasteval::{Compiler, Evaler};
use std::{
    alloc::{self, Layout},
    io::Write,
    time::Instant,
};

mod cf;

const EPSILON: f64 = 1e-11;

fn integrate(
    mut func: impl FnMut(f64) -> f64,
    lower_bound: f64,
    upper_bound: f64,
    iterations: u32,
) -> f64 {
    let (a, b, k) = if upper_bound > lower_bound {
        (lower_bound, upper_bound, 1.0)
    } else {
        (upper_bound, lower_bound, -1.0)
    };
    let precision = (b - a) / iterations as f64;

    let mut area = 0.0;
    let mut i = lower_bound;

    while i <= upper_bound {
        area += func(i) * precision;
        i += precision;
    }

    if area < EPSILON {
        return 0.0;
    } else {
        return area * k;
    }
}

fn parse_bounds() -> (f64, f64) {
    print!("lower bound = ");
    std::io::stdout().flush().ok();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();

    let lower_bound: f64 = input.parse().unwrap();

    print!("upper bound = ");
    std::io::stdout().flush().ok();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();

    let upper_bound: f64 = input.parse().unwrap();

    return (lower_bound, upper_bound);
}

fn main() {
    unsafe {
        let parser = fasteval::Parser::new();
        let mut slab = fasteval::Slab::new();

        let layout = Layout::new::<f64>();
        let x_ptr = alloc::alloc(layout);
        let x_ptr: *mut f64 = x_ptr as *mut f64;

        //* it's gonna blow up!
        slab.ps.add_unsafe_var("x".to_string(), &*x_ptr);

        print!("f(x) = ");
        std::io::stdout().flush().ok();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();

        // let expr_str = "2 * y^2";
        let expr_str = input.as_str();
        let compiled = parser
            .parse(expr_str, &mut slab.ps)
            .unwrap()
            .from(&slab.ps)
            .compile(&slab.ps, &mut slab.cs);

        let mut ns = fasteval::EmptyNamespace;
        let mut ns1 = fasteval::EmptyNamespace;

        let func = |x: f64| -> f64 {
            *x_ptr = x;
            compiled.eval(&slab, &mut ns).unwrap()
        };

        // let func = |x: f64| -> f64 { x * x };

        // just so it panics if there's something wrong
        {
            if compiled.eval(&slab, &mut ns1).is_err() {
                eprintln!("Bad expression");
                std::process::exit(-1);
            }
        }

        let (lower_bound, upper_bound) = parse_bounds();

        // let func = |x: f64| -> f64 { x * x };
        // let lower_bound = 0.0;
        // let upper_bound = 1.0;

        let t1 = Instant::now();

        let result = integrate(func, lower_bound, upper_bound, 2.6e7 as u32);
        let (p, q): (u64, u64) = cf::dec_to_fraction(result);

        let time = Instant::now() - t1;

        println!("Result: {result}");
        println!("Possible fraction: {p}/{q}");
        println!("Time: {time:?}");
    }
}
