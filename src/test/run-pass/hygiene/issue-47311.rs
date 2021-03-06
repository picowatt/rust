// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// ignore-pretty pretty-printing is unhygienic

#![feature(decl_macro, proc_macro_path_invoc)]
#![allow(unused)]

macro m($S:ident, $x:ident) {
    $S { $x: 0 }
}

mod foo {
    struct S { x: i32 }

    fn f() { ::m!(S, x); }
}

fn main() {}
