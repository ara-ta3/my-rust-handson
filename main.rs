fn main() {
    let name = "Hoge";
    println!("hello, {}!", name);
    println!("1 + 1 = {}", add(1, 1));
    println!("2 + 4 = {}", add2(2, 4));
    let a = [1,2,3];
    println!("{}", a.len());
    /* a[0] = 10; 
     * error: cannot assign to immutable indexed content `a[..]`
     * --> ./main.rs:7:5
     *   |
     * 7 |     a[0] = 10;
     *   |     ^^^^^^^^^
     * error: aborting due to previous error
     */
    let mut b = [1,2,3];
    b[0] = 10;
    let middle = &a[1..2];
    println!("middle.len() = {}", middle.len());
    let p = (1,2,3);
    let(x, _, _) = p;
    println!("x = {}", x);
    println!("p.0 = {}", p.0);

    let x = 5;
    if x > 0 {
        println!("x > 0");
    } else if x > -5 {
        println!("x > -5");
    } else {
        println!("x <= -5");
    };
    let y = if x > 0 {
        1
    } else {
        0
    };
    println!("y = {}", y);
    let mut i = 0;
    loop {
        i = i + 1;
        println!("loop {}", i);
        if i > 9 {
            break;
        }
    }

    for(idx, v) in (0..10).enumerate() {
        println!("index = {}, value = {}", idx, v);
    }

    let v = vec![1,2,3,4,5];
    let zeroes = vec![0; 10];
    for i in 0..v.len() {
        println!("v[{}] = {}", i, v[i]);
    }
    for i in 0..zeroes.len() {
        println!("zeroes[{}] = {}", i, zeroes[i]);
    }

    let v = vec![1,2,3,4,5];
    let r = v.iter().filter(|&n|n % 2 != 0).map(|n| n + 1);
    for (i, v) in r.enumerate() {
        println!("r[{}]: {}", i, v);
    }

    println!("fib(26) = {}", fib(26));
    println!("fib(0) = {}", fib(0));
    let v = vec![1, 2, 3];
    println!("v[1] = {}", v[1]);
    let v2 = v;
    println!("v2[1] = {}", v2[1]);
    // println!("v[1] = {}", v[1]);
    /*
     * error[E0382]: use of moved value: `v`
     *   --> ./main.rs:73:27
     *      |
     *   71 |     let v2 = v;
     *      |         -- value moved here
     *   72 |     println!("v2[1] = {}", v2[1]);
     *   73 |     println!("v[1] = {}", v[1]);
     *      |                           ^ value used here after move
     *      |
     *          = note: move occurs because `v` has type `std::vec::Vec<i32>`, which does not implement the `Copy` trait
     */

}

fn add(a: i32, b:i32) -> i32 {
    a + b
}

fn add2(a: i32, b:i32) -> i32 {
    let ans: i32;
    ans = a + b;
    return ans;
}

fn fib(n: i32) -> u32 {
    if n <= 1 {
        n as u32
    } else {
        fib(n - 1) + fib(n - 2) as u32
    }
}
