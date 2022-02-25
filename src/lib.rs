#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use libc::*;
use std::ptr;

pub mod udbm {
    mod bindings {
        include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
    }
    pub struct DBM {
        data: Vec<i32>,
        dim: usize,
    }
    pub fn init(dim: usize) -> DBM {
        let mut vec = vec![0; dim * dim];
        unsafe{
            bindings::dbm_init(vec.as_mut_ptr(), dim as u32);
        }
        return DBM {
            data: vec,
            dim: dim,
        };
    }

    pub fn consistent(dbm: &DBM) {

    }

    pub fn relation(lesser_dbm: &DBM, greater_dbm: &DBM) -> u32 {
        unsafe{
            return bindings::dbm_relation(lesser_dbm.data.as_ptr(), greater_dbm.data.as_ptr(), lesser_dbm.dim as u32);
        }
    }
}

#[test]
fn init_test() {
    // Can we init without erroring?
    // Kinda useless, but good for sanity
    let dim = 3;
    let dbm = udbm::init(dim);
}

#[test]
fn relation_test1() {
    let dim = 3;
    let dbm1 = udbm::init(dim);
    let dbm2 = udbm::init(dim);

    assert_eq!(udbm::relation(&dbm1, &dbm2), udbm::relation(&dbm2, &dbm1));
}
