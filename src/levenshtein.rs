use std::mem::swap;
use std::cmp::min;

// Thanks: https://github.com/Noskcaj19/rust-levenshtein/blob/master/src/lib.rs
pub fn distance(s: &str, t: &str) -> (usize, Vec<bool>) {
    let m = s.len();
    let n = t.len();

    let mut s0_diff    = vec!(false).repeat(m);
    let s0: Vec<_> = s.chars().collect();
    let s1: Vec<_> = t.chars().collect();

    let mut v0: Vec<_> = (0..n + 1).collect();
    let mut v1 = vec![0; n + 1];

    for i in 0..m {
        v1[0] = i + 1;

        for j in 0..n {
            let substitution_cost;
            if s0[i] == s1[j] {
                substitution_cost = 0;
            } else {
                substitution_cost = 1;
                s0_diff[i] = true;
            }
            v1[j + 1] = min(min(v1[j] + 1, v0[j + 1] + 1), v0[j] + substitution_cost);
        }
        swap(&mut v0, &mut v1);
    }
    return (v0[n], s0_diff);
}