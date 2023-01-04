use std::env;

#[derive(Debug)]
struct J(u64, u64, u64);
fn main() {
    let args: Vec<String> = env::args().collect();
    //let upper_bound: J = J(5000,0,0);
    let upper_bound: J = J(args[1].parse().unwrap(), 0, 0);
    let lower_bound: J = J(5,0,0);  // min J(5,0,0)
    let mut j_max: J = J(0,0,0);
    let mut ct_max = 0;
    let mut i: J = J(lower_bound.0,lower_bound.1,lower_bound.2);
    let mut i_max: J = J(i.0,i.1,i.2);

    while j_compare(&i, &upper_bound) != 1 {
        let mut j = J(i.0, i.1, i.2);
        let mut ct = 0;
    
        //while j.0 > 1 || j.1 > 0 || j.2 > 0 {
        while j_compare(&j, &i) >= 0 {
            ct = ct + 1;
            if 0 == j.0 % 2 {
                j_even(&mut j);        
            } else {
                j_odd(&mut j);
            }
            if 1 == j_compare(&j, &j_max) {
                j_max.0 = j.0;
                j_max.1 = j.1;
                j_max.2 = j.2;
            }
        }
    
        if ct > ct_max {
            ct_max = ct;
            i_max.0 = i.0;
            i_max.1 = i.1;
            i_max.2 = i.2;
        }
        j_add1(&mut i);
    }
    
    println!("MAX ct={} i={:?} j={:?}", ct_max, i_max, j_max);
}


fn j_add1_1 (j: &mut J) {
    if u64::MAX != j.1 {
        j.1 = j.1 + 1;
    } else {
        j.1 = 0;
        j.2 = j.2 + 1;
    };
}

fn j_add1 (j: &mut J) {
    if u64::MAX != j.0 {
        j.0 = j.0 + 1;
    } else {
        j.0 = 0;
        //j.1 = j.1 + 1;
        j_add1_1(j);
    };
}

fn j_add_1 (j: &mut J, v1: u64, v2: u64, carry: u64) {
    if carry != 0 {
        if u64::MAX != j.1 {
            j.1 = j.1 + 1;
        } else {
            j.1 = 0;
            j.2 = j.2 + 1;
        };
    }
    if u64::MAX - j.1 >= v1 {
        j.1 = j.1 + v1;
        j.2 = j.2 + v2;
    } else {
        j.1 = v1 - (u64::MAX - j.1) - 1;
        j.2 = j.2 + v2 + 1;
    };
}

fn j_add (j: &mut J, v0: u64, v1: u64, v2: u64) {
    //println!("add_N j {:?} v0={} v1={} v2={}", j, v0, v1, v2);
    if u64::MAX - j.0 >= v0 {
        j.0 = j.0 + v0;
        j_add_1(j, v1, v2, 0);
    } else {
        j.0 = v0 - (u64::MAX - j.0) - 1;
        j_add_1(j, v1, v2, 1);
    };
}

fn j_div2_1 (j: &mut J) {
    if 0 == j.2 % 2 {
        j.1 = j.1 / 2;
    } else {
        j.1 = j.1 / 2 + (u64::MAX / 2 + 1);
    };
    j.2 = j.2 / 2;
}

fn j_mult2_1 (j: &mut J, carry: u64) {
    if j.1 & (1 << 63) == 0 {
        j.2 = j.2 * 2;
    } else {
        j.2 = j.2 * 2 + 1;
    };
    j.1 = j.1 << 1 | carry;
}

fn j_even (j: &mut J) {
    if 0 == j.1 % 2 {
        j.0 = j.0 / 2;
    } else {
        j.0 = j.0 / 2 + (u64::MAX / 2 + 1);
    };
    j_div2_1(j);
}

fn j_odd (j: &mut J) {
    let v0 = j.0;
    let v1 = j.1;
    let v2 = j.2;
    j.0 = j.0 << 1 | 1;
    if v0 & (1 << 63) == 0 {
        j_mult2_1(j, 0);
    } else {
        j_mult2_1(j, 1);
    };
    j_add(j, v0, v1, v2);
}

fn j_compare (j1: &J, j2: &J) -> i8 {
    if j1.2 > j2.2 {return 1;}
    else if j1.2 < j2.2 {return -1;}
    else if j1.1 > j2.1 {return 1;}
    else if j1.1 < j2.1 {return -1;}
    else if j1.0 > j2.0 {return 1;}
    else if j1.0 < j2.0 {return -1;}
    else {return 0;}
}
