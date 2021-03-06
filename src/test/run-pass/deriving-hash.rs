// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


#![feature(hash_default)]

use std::hash::{Hash, SipHasher, Hasher};
use std::mem::size_of;

#[derive(Hash)]
struct Person {
    id: usize,
    name: String,
    phone: usize,
}

// test for hygiene name collisions
#[derive(Hash)] struct __H__H;
#[derive(Hash)] enum Collision<__H> { __H { __H__H: __H } }

#[derive(Hash)]
enum E { A=1, B }

fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = SipHasher::new_with_keys(0, 0);
    t.hash(&mut s);
    s.finish()
}

struct FakeHasher<'a>(&'a mut Vec<u8>);
impl<'a> Hasher for FakeHasher<'a> {
    fn finish(&self) -> u64 {
        unimplemented!()
    }

    fn write(&mut self, bytes: &[u8]) {
        self.0.extend(bytes);
    }
}

fn fake_hash(v: &mut Vec<u8>, e: E) {
    e.hash(&mut FakeHasher(v));
}

fn main() {
    let person1 = Person {
        id: 5,
        name: "Janet".to_string(),
        phone: 555_666_7777
    };
    let person2 = Person {
        id: 5,
        name: "Bob".to_string(),
        phone: 555_666_7777
    };
    assert_eq!(hash(&person1), hash(&person1));
    assert!(hash(&person1) != hash(&person2));

    // test #21714
    let mut va = vec![];
    let mut vb = vec![];
    fake_hash(&mut va, E::A);
    fake_hash(&mut vb, E::B);
    assert!(va != vb);
}
