fn fib_re(n: isize) -> isize {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    return fib_re(n-1) + fib_re(n-2);
}

fn fib_memory(n: isize) -> isize {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    let mut a = 0;
    let mut b = 1;
    for _i in 2..n+1 {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}

fn fib_matrix(n: isize) -> (isize, isize) {
    if n > 0 {
        let (t0, t1) = fib_matrix(n / 2);
        if n % 2 > 0 {
            return (t0 * t0 + t1 * t1, (2 * t0 + t1) * t1);
        } else {
            return ((2 * t1 - t0) * t0, t0 * t0 + t1 * t1);
        }
    }
    (0, 1)
}

fn main() {
    assert_eq!(8, fib_re(6));
    assert_eq!(8, fib_memory(6));
    assert_eq!(8, fib_matrix(6).0);
}
