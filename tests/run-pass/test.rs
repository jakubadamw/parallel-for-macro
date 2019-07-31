// check-pass
#![feature(proc_macro_hygiene)]
#![feature(stmt_expr_attributes)]

extern crate parallel_for_macro;
extern crate rayon;

use rayon::prelude::*;

use parallel_for_macro::parallel;

fn main() {
    #[parallel]
    for x in 0..10 {}
}
