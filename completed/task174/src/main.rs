/*
    https://www.codewars.com/kata/5544c7a5cb454edb3c000047/train/rust
 */

fn bouncing_ball(h: f64,  bounce: f64,  window: f64) -> i32 {
    if h <= 0.0 || 0.0 >= bounce || 1.0 <= bounce || window >= h { return -1 }
    let mut ball_pass = 0;
    let mut height = h;
    while height > window {
        ball_pass += 1;
        height *= bounce;
        if height <= window { break; }
        ball_pass += 1;
    }
    ball_pass
}

fn main() {
    println!("{}", bouncing_ball(3.0, 0.66, 1.5));
    println!("{}", bouncing_ball(3.0, 1.0, 1.5));
    println!("{}", bouncing_ball(40.0, 0.4, 10.0));
}
