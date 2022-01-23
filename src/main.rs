use fasteval::{Compiler, Evaler};
// use this trait so we can call eval().
use std::{
    alloc::{self, Layout},
    io::Write,
    time::Instant,
}; // use this trait so we can call compile().

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

fn epsilon_round(n: f64) -> f64 {
    const EPSILON: f64 = 1e-6;
    if (n - n.round()).abs() <= EPSILON {
        return n.round() as f64;
    } else {
        return n;
    };
}

fn possible_fraction(number: f64) -> Option<String> {
    fn iterate_denom(number: f64, iter: impl Iterator<Item = f64>) -> Option<String> {
        for possible_denominator in iter {
            let possible_numerator = epsilon_round(number * (possible_denominator as f64));
            // println!("{possible_numerator}/{possible_denominator}");
            if possible_numerator.fract() == 0.0 {
                if possible_denominator == 1.0 {
                    return Some(possible_numerator.to_string());
                } else {
                    return Some(format!("{possible_numerator}/{possible_denominator}"));
                }
            }
        }
        return None;
    }
    // let number: f64 = epsilon_round(number);

    return iterate_denom(number, (1..=420).map(|x| (x as f64))).or_else(|| {
        iterate_denom(
            number,
            [
                3.0_f64.sqrt(),
                5.0_f64.sqrt(),
                7.0_f64.sqrt(),
                10.0_f64.sqrt(),
                13.0_f64.sqrt(),
                15.0_f64.sqrt(),
            ]
            .into_iter(),
        )
    });
}

fn main() {
    // meval impl
    // let func = loop {
    //     print!("f(x) = ");
    //     std::io::stdout().flush().ok();

    //     let mut input = String::new();
    //     std::io::stdin().read_line(&mut input).unwrap();

    //     let expr = input.parse::<meval::Expr>();
    //     match expr {
    //         Ok(val) => break (val.bind("x").unwrap()),
    //         Err(_) => {
    //             println!("Malformed input");
    //         }
    //     }
    // };

    // let func = loop {
    //     let a = parse_input();
    // };

    unsafe {
        let parser = fasteval::Parser::new();
        let mut slab = fasteval::Slab::new();

        // The Unsafe Variable will use a pointer to read this memory location:
        // You must make sure that this variable stays in-scope as long as the
        // expression is in-use.
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

        let t1 = Instant::now();

        let result = integrate(func, lower_bound, upper_bound, 2.5e7 as u32);
        let fraction = possible_fraction(result).unwrap_or("None".to_string());

        let time = Instant::now() - t1;

        println!("Result: {result}");
        println!("Possible fraction: {fraction}");
        println!("Time: {time:?}");
    }
}
