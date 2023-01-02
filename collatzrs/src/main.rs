use std::env;

#[derive(Debug)]
struct J(u64, u64);
fn main() {
    let args: Vec<String> = env::args().collect();
    let upper_bound: u64 = args[1].parse().unwrap();
    let mut j_max: J = J(0,0);
    let mut ct_max = 0;
    let mut i_max: u64 = 0;
    let mut i = 2;
    while i <= upper_bound {
        //let mut j = J(u64::MAX << 1, 1);
        let mut j = J(i, 0);
        let mut ct = 1;
        //while j.0 > 1 || j.1 > 0 {
        while j.1 > 0 || j.0 >= i {
            ct = ct + 1;
            //println!("{:?} j    {:?}", i, j);
            if 0 == j.0 % 2 {
                j_even(&mut j);        
                //println!("{:?} even {:?}", i, j);
            } else {
                j_odd(&mut j);
                //println!("{:?} odd  {:?}", i, j);
            }
            if j.1 > j_max.1 || (j.1 == j_max.1 && j.0 > j_max.0) {
                j_max.0 = j.0;
                j_max.1 = j.1;
            }
        }
        if ct > ct_max {
            ct_max = ct;
            i_max = i;
        }
        i = i + 1;
    }
    println!("MAX ct={} i={} {:?}", ct_max, i_max, j_max);
}

fn j_add (j: &mut J, v0: u64, v1: u64) {
    if u64::MAX - j.0 >= v0 {
        j.0 = j.0 + v0;
        j.1 = j.1 + v1;
    } else {
        j.0 = v0 - (u64::MAX - j.0) - 1;
        j.1 = j.1 + v1 + 1;
    };
}

fn j_even (j: &mut J) {
    if 0 == j.1 % 2 {
        j.0 = j.0 / 2;
        j.1 = j.1 / 2;
    } else {
        j.0 = j.0 / 2 + (u64::MAX / 2 + 1);
        j.1 = j.1 / 2;
    };
}

fn j_odd (j: &mut J) {
    let v0 = j.0;
    let v1 = j.1;
    j.0 = j.0 << 1 | 1;
    if v0 & (1 << 63) == 0 {
        j.1 = j.1 << 1;
    } else {
        j.1 = j.1 << 1 | 1;
    };
    j_add(j, v0, v1);
}
