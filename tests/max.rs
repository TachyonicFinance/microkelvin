// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use rand::{prelude::SliceRandom, thread_rng};

mod linked_list;
use linked_list::LinkedList;

use canonical_derive::Canon;
use microkelvin::{GetMaxKey, Keyed, MaxKey};

#[derive(PartialEq, Clone, Canon, Debug)]
struct TestLeaf {
    key: u64,
    other: (),
}

impl Keyed<u64> for TestLeaf {
    fn key(&self) -> &u64 {
        &self.key
    }
}

#[test]
fn maximum() {
    let n: u64 = 1024;

    let mut keys = vec![];

    for i in 0..n {
        keys.push(i)
    }

    keys.shuffle(&mut thread_rng());

    let mut list = LinkedList::<_, MaxKey<u64>>::new();

    for key in keys {
        list.insert(TestLeaf { key, other: () });
    }

    let max = list.max_key().unwrap().unwrap();

    assert_eq!(
        *max,
        TestLeaf {
            key: 1023,
            other: ()
        }
    )
}
