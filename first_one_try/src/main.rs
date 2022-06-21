fn fib_loop(n: u8) {
    let mut a = 1;
    let mut b = 1;
    let mut i = 2u8;

    loop {
        let x = fib_math_logic(a, b);
        a = x.0;
        b = x.1;
        i += 1;

        println!("next val is {}", b);

        if i >= n {
            break;
        }
    }
}

fn fib_math_logic(mut a: i32, mut b: i32) -> (i32, i32) {
    let c = a + b;
    a = b;
    b = c;
    (a, b)
}

fn fib_while(n: u8) {
    let (mut a, mut b, mut i) = (1, 1, 2);

    while i < n {
        let x = fib_math_logic(a, b);
        a = x.0;
        b = x.1;
        i += 1;
        println!("next val is {}", b);
    }
}

fn fib_for(n: u8) {
    let (mut a, mut b) = (1, 1);

    for _i in 2..n {
        let x = fib_math_logic(a, b);
        a = x.0;
        b = x.1;
        println!("next val is {}", b);
    }
}

fn main() {
    let n = 10;
    fib_loop(n);
    fib_while(n);
    fib_for(n);
}
