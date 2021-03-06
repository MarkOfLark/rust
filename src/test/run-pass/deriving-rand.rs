// Copyright 2013-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::rand;

#[derive(Rand)]
struct A;

#[derive(Rand)]
struct B(int, int);

#[derive(Rand)]
struct C {
    x: f64,
    y: (u8, u8)
}

#[derive(Rand)]
enum D {
    D0,
    D1(uint),
    D2 { x: (), y: () }
}

pub fn main() {
    // check there's no segfaults
    for _ in 0..20 {
        rand::random::<A>();
        rand::random::<B>();
        rand::random::<C>();
        rand::random::<D>();
    }
}
