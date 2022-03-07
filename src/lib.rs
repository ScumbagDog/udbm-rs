#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

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
        unsafe {
            bindings::rs_dbm_init(vec.as_mut_ptr(), dim as u32);
        }
        return DBM {
            data: vec,
            dim: dim,
        };
    }

    pub fn relation(lhs_dbm: &DBM, rhs_dbm: &DBM) -> u32 {
        unsafe {
            return bindings::rs_dbm_relation(
                lhs_dbm.data.as_ptr(),
                rhs_dbm.data.as_ptr(),
                lhs_dbm.dim as u32,
            );
        }
    }
    pub fn satisfies(dbm: &DBM, i: usize, j: usize, constraint: i32) -> bool {
        unsafe {
            return bindings::rs_dbm_satisfies(
                dbm.data.as_ptr(),
                dbm.dim as u32,
                i as u32,
                j as u32,
                constraint,
            );
        }
    }

    pub fn close(dbm: &mut DBM) -> bool {
        unsafe {
            return bindings::rs_dbm_close(dbm.data.as_mut_ptr(), dbm.dim as u32);
        }
    }

    pub fn up(dbm: &mut DBM) -> () {
        unsafe {
            bindings::rs_dbm_up(dbm.data.as_mut_ptr(), dbm.dim as u32);
        }
    }

    pub fn down(dbm: &mut DBM) -> () {
        unsafe {
            bindings::rs_dbm_down(dbm.data.as_mut_ptr(), dbm.dim as u32);
        }
    }

    pub fn and(dbm: &mut DBM, i: usize, j: usize, constraint: i32) -> bool {
        unsafe {
            return bindings::rs_dbm_constrain1(
                dbm.data.as_mut_ptr(),
                dbm.dim as u32,
                i as u32,
                j as u32,
                constraint,
            );
        }
    }

    pub fn free(dbm: &mut DBM, k: usize) {
        unsafe {
            bindings::rs_dbm_freeClock(dbm.data.as_mut_ptr(), dbm.dim as u32, k as u32);
        }
    }

    pub fn assign(dbm: &mut DBM, x: usize, value: i32) -> () {
        unsafe {
            bindings::rs_dbm_updateValue(dbm.data.as_mut_ptr(), dbm.dim as u32, x as u32, value);
        }
    }

    pub fn copy(dbm: &mut DBM, x: usize, y: usize) -> () {
        unsafe {
            bindings::rs_dbm_updateClock(dbm.data.as_mut_ptr(), dbm.dim as u32, x as u32, y as u32);
        }
    }

    pub fn shift(dbm: &mut DBM, x: usize, value: i32) -> () {
        unsafe {
            bindings::rs_dbm_updateIncrement(dbm.data.as_mut_ptr(), dbm.dim as u32, x as u32, value);
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
