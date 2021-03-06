// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(dead_code)]
#![deny(unreachable_code)]

fn foo() {
    while {return} {
        println!("Hello, world!");
        //~^ ERROR unreachable
    }
}

fn bar() {
    while {true} {
        return;
    }
    println!("I am not dead.");
}

fn baz() {
    // Here, we cite the `while` loop as dead.
    while {return} {
        println!("I am dead.");
        //~^ ERROR unreachable
    }
    println!("I am, too.");
    //~^ ERROR unreachable
}

fn main() { }
