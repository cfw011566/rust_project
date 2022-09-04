use std::collections::HashMap;

fn main() {
    /*
    for i in 1..=120 {
        println!("fib({}) = {}", i, fib(i));
    }
    */
    for i in 1..=120 {
        let mut memo: HashMap<u128, u128> = HashMap::new();
        println!("fib({}) = {}", i, fast_fib(i, &mut memo));
    }
}

fn fib(n: u128) -> u128 {
    if n == 0 || n == 1 {
        return 1;
    }
    fib(n - 1) + fib(n - 2)
}

fn fast_fib(n: u128, memo: &mut HashMap<u128, u128>) -> u128 {
    if n == 0 || n == 1 {
        return 1;
    }
    if let Some(val) = memo.get(&n) {
        return *val;
    }
    let n1 = fast_fib(n - 1, memo);
    let n2 = fast_fib(n - 2, memo);
    memo.insert(n, n1 + n2);
    n1 + n2
}
