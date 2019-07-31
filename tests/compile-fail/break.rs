// error-pattern: A `break` expression is not allowed within the body of a `#[parallel]` loop

#![feature(proc_macro_hygiene)]
#![feature(stmt_expr_attributes)]

extern crate parallel_for_macro;
extern crate rayon;

use rayon::prelude::*;

use parallel_for_macro::parallel;

fn main() {
    #[parallel]
    for x in 0..10 {
        if x % 2 == 0 {
            break;
        }
        println!("{}", x);
    }
}
