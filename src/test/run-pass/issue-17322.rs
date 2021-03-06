// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(unknown_features)]
#![feature(box_syntax)]

use std::old_io;

fn f(wr: &mut Writer) {
    wr.write_str("hello").ok().expect("failed");
}

fn main() {
    let mut wr = box old_io::stdout() as Box<Writer + 'static>;
    f(&mut wr);
}
