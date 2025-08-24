    //! Texas Hold'em hand evaluator for two players.
    //!
    //! This crate exposes a single required entrypoint:
    //!
    //! ```rust
    //! pub fn deal(perm: [u32; 9]) -> Vec<String>;
    //! ```
    //!
    //! It accepts the first 9 values of a deck permutation (1..=52) and returns
    //! the winning 5‑card hand as `["<rank><SUIT>", ...]`, where suits are `C/D/H/S`
    //! and ranks use numeric faces (11=J, 12=Q, 13=K, 1=A).
    //!
    //! Notes:
    //! - No `main()` function is included; this is intended to be linked by a tester.
    //! - The implementation is adapted from the uploaded `Poker.rs` file.
    #![forbid(unsafe_code)]
    #![warn(missing_docs, clippy::all, clippy::pedantic)]

    // #[warn(non_snake_case)]

fn helper_straight(a: u32) -> u32 {
    let ret;
    if a < 14 {
        ret = a;
    } else if a < 27 {
        ret = a - 13;
    } else if a < 40 {
        ret = a - 26;
    } else {
        ret = a - 39;
    };
    ret
}
fn winner_to_vec_string(arr: &Vec<u32>) -> Vec<String> {
    let mut winner: Vec<String> = Vec::new();
    let mut ret=arr.to_vec();
    ret.reverse();
    let mut e=0;
    loop{
        if helper_straight(ret[0])==1{
            ret.push(ret[0]);
            ret.remove(0);
        }
        e=e+1;
        if e==ret.len(){
            break;
        };
    };
    for i in 0..(ret.len()) {
        if ret[i] < 14 {
            winner.push((ret[i]).to_string() + "C");
        } else if ret[i] < 27 {
            winner.push((ret[i] - 13).to_string() + "D");
        } else if ret[i] < 40 {
            winner.push((ret[i] - 26).to_string() + "H");
        } else {
            winner.push((ret[i] - 39).to_string() + "S");
        };
    }
    winner
}

fn sub_list(v: &Vec<u32>, royal: Vec<u32>) -> bool {
    let mut ret = true;
    for i in 0..(royal.len()) {
        if !v.iter().any(|&t| t == royal[i]) {
            ret = false;
        };
    }
    ret
}
fn split_exist(v: &Vec<u32>, v2: &Vec<u32>) -> Vec<u32> {
    let mut ret: Vec<u32> = Vec::new();
    for i in 0..(v.len()) {
        if !v2.iter().any(|&t| t == v[i]) {
            ret.push(v[i]);
        };
    }
    ret
}
fn has_royal(arr: &Vec<u32>) -> Vec<u32> {
    let mut ret: Vec<u32> = Vec::new();
    if sub_list(&arr, vec![1, 10, 11, 12, 13]) {
        ret = vec![1, 10, 11, 12, 13];
    } else if sub_list(&arr, vec![14, 23, 24, 25, 26]) {
        ret = vec![14, 23, 24, 25, 26];
    } else if sub_list(&arr, vec![27, 36, 37, 38, 39]) {
        ret = vec![27, 36, 37, 38, 39];
    } else if sub_list(&arr, vec![40, 49, 50, 51, 52]) {
        ret = vec![40, 49, 50, 51, 52];
    };
    ret.reverse();
    ret
}
fn get_ace(a:u32) -> u32 {
    let ret ;
    if a < 14 {
        ret = 1;
    } else if a < 27 {
        ret = 14;
    } else if a < 40 {
        ret = 27;
    } else {
        ret = 40;
    };
    ret
}

fn straight_flush(arr: &Vec<u32>) -> Vec<u32> {
    let mut ret: Vec<u32> = Vec::new();
    let mut v = arr.to_vec();
    v.sort();
    let mut k = 0;
    let mut a = 1;
    let mut b = 13;
    let mut t = v[0];
    ret.push(v[0]);
    for e in 0..(v.len()) {
        if v[e] >= a && v[e] <= b {
            if v[e] == a {
                ret.clear();
                k = v[e];
                continue;
            };
            if t + 1 == v[e] {
                t = t + 1;
                ret.push(v[e]);
                if v[e] == b && k != 0 {
                    ret.push(k);
                };
            } else {
                if ret.len() < 5 {
                    t = v[e];
                    ret.clear();
                    ret.push(v[e]);
                } else {
                    break;
                }
            };
        } else {
            a =get_ace(v[e]);
            b = a + 13 -1;
            k = 0;
            t = v[e];
            ret.clear();
            ret.push(v[e]);
        };
    }

    if ret.len() < 5 {
        ret.clear();
        ret
    } else {
        while ret.len() > 5 {
            ret.remove(0);
        }
        ret.reverse();
        ret
    }
}
fn same_of_kind(arr: &Vec<u32>, n: usize) -> Vec<u32> {
    let mut ret: Vec<u32> = Vec::new();
    let mut v = arr.to_vec();
    v.sort();
    let mut a ;
    for i in 0..(v.len()) {
        a = v[i];
        ret.push(v[i]);
        for e in 0..(v.len()) {
            if a + 13 == v[e] || a+26==v[e] || a+39==v[e] {
                ret.push(v[e]);
                a = v[e];
            };
        }
        if ret.len() == n {
            break;
        } else {
            ret.clear();
        };
    }
    ret.reverse();
    ret
}

fn four_of_kind(arr: &Vec<u32>) -> Vec<u32> {
    same_of_kind(arr, 4)
}

fn full_house(arr: &Vec<u32>) -> Vec<u32> {
    let mut v = same_of_kind(arr, 3);
    if v.len() != 0 {
        let v2 = split_exist(arr, &v);
        let v3 = same_of_kind(&v2, 2);
        if v3.len() == 2 {
            for i in 0..(v3.len()) {
                v.push(v3[i]);
            }
            v
        } else {
            v.clear();
            v
        }
    } else {
        v
    }
}

fn flush(arr: &Vec<u32>) -> Vec<u32> {
    let mut ret: Vec<u32> = Vec::new();
    let mut v = arr.to_vec();
    v.sort();
    let mut a = 1;
    let mut b = 13;
    for e in 0..(v.len()) {
        if v[e] >= a && v[e] <= b {
            ret.push(v[e]);
        } else {
            if ret.len() >= 5 {
                break;
            } else {
                a = get_ace(v[e]);
                b = a + 13-1;
                ret.clear();
                ret.push(v[e]);
            }
        };
    }
    if ret.len() < 5 {
        ret.clear();
    }
    while ret.len() > 5 {
        ret.remove(0);
    }
    ret.reverse();
    ret
}

fn straight(arr: &Vec<u32>) -> Vec<u32> {
    let mut ret: Vec<u32> = Vec::new();
    let mut v = arr.to_vec();
    v.sort();
    let mut k ;
    let mut a ;
    let mut t ;
    let mut current ;
    let mut i = 0;
    ret.push(v[i]);
    t = helper_straight(v[i]);
    loop {
        a = ret.len();
        k = 0;

        for e in 0..(v.len()) {
            current = helper_straight(v[e]);
            if current == 1 {
                k = v[e];
            };
            if t + 1 == current {
                ret.push(v[e]);
                t = t + 1;
                if current == 14 && k != 0 {
                    ret.push(k);
                    k = 0;
                }
                break;
            };
        }
        if ret.len() == a {
            if ret.len() >= 5 {
                break;
            } else {
                i = i + 1;

                ret.clear();
                if i < v.len() {
                    t = helper_straight(v[i]);
                    ret.push(v[i]);
                } else {
                    break;
                };
            };
        }
    }

    if ret.len() < 5 {
        ret.clear();
        ret
    } else {
        while ret.len() > 5 {
            ret.remove(0);
        }
        ret.reverse();
        ret
    }
}
fn three_of_kind(arr: &Vec<u32>) -> Vec<u32> {
    same_of_kind(arr, 3)
}
fn two_pairs(arr: &Vec<u32>) -> Vec<u32> {
    let mut v = same_of_kind(arr, 2);
    if v.len() != 0 {
        let v2 = split_exist(arr, &v);
        let v3 = same_of_kind(&v2, 2);
        if v3.len() == 2 {
            for i in 0..(v3.len()) {
                v.push(v3[i]);
            }
            v
        } else {
            v.clear();
            v
        }
    } else {
        v
    }
}
fn pair(arr: &Vec<u32>) -> Vec<u32> {
    same_of_kind(arr, 2)
}
fn check_winner(p1: &Vec<u32>, p2: &Vec<u32>) -> Vec<u32> {
    if p1.len() == 0 {
        p2.to_vec()
    } else if p2.len() == 0 {
        p1.to_vec()
    } else {
        let mut l = p2.len();
        let mut val1: Vec<u32> = Vec::new();
        let mut val2: Vec<u32> = Vec::new();
        let mut win = 1;
        let mut t;
        for i in 0..(p1.len()) {
            t = helper_straight(p1[i]);
            if t == 1 {
                t = 15;
            }
            val1.push(t);
        }
        for i in 0..(p2.len()) {
            t = helper_straight(p2[i]);
            if t == 1 {
                t = 15;
            }
            val2.push(t);
        }
        val1.sort();
        val2.sort();

        if p1.len() < p2.len() {
            l = p1.len();
        }
        for i in 0..(l) {
            if val1[i] == val2[i] {
                continue;
            } else if val2[i] > val1[i] {
                win = 2;
                break;
            } else {
                win = 1;
                break;
            }
        }
        if win == 1 {
            p1.to_vec()
        } else {
            p2.to_vec()
        }
    }
}
pub fn deal(perm: [u32; 9]) -> Vec<String> {
    let mut p1: Vec<u32> = Vec::new();
    let mut p2: Vec<u32> = Vec::new();
    p1.push(perm[0]);
    p2.push(perm[1]);
    p1.push(perm[2]);
    p2.push(perm[3]);
    for i in 4..(9) {
        p1.push(perm[i]);
        p2.push(perm[i]);
    }

    if check_winner(&has_royal(&p1), &has_royal(&p2)).len() != 0 {
        // println!("Royal");
        winner_to_vec_string(&check_winner(&has_royal(&p1), &has_royal(&p2)))
    } else if check_winner(&straight_flush(&p1), &straight_flush(&p2)).len() != 0 {
        // println!("st flush");
        winner_to_vec_string(&check_winner(&straight_flush(&p1), &straight_flush(&p2)))
    } else if check_winner(&four_of_kind(&p1), &four_of_kind(&p2)).len() != 0 {
        // println!("four");
        winner_to_vec_string(&check_winner(&four_of_kind(&p1), &four_of_kind(&p2)))
    } else if check_winner(&full_house(&p1), &full_house(&p2)).len() != 0 {
        // println!("full");
        winner_to_vec_string(&check_winner(&full_house(&p1), &full_house(&p2)))
    } else if check_winner(&flush(&p1), &flush(&p2)).len() != 0 {
        // println!("flush");
        winner_to_vec_string(&check_winner(&flush(&p1), &flush(&p2)))
    } else if check_winner(&straight(&p1), &straight(&p2)).len() != 0 {
        // println!("st");
        winner_to_vec_string(&check_winner(&straight(&p1), &straight(&p2)))
    } else if check_winner(&three_of_kind(&p1), &three_of_kind(&p2)).len() != 0 {
        // println!("three");
        winner_to_vec_string(&check_winner(&three_of_kind(&p1), &three_of_kind(&p2)))
    } else if check_winner(&two_pairs(&p1), &two_pairs(&p2)).len() != 0 {
        // println!("two");
        winner_to_vec_string(&check_winner(&two_pairs(&p1), &two_pairs(&p2)))
    } else if check_winner(&pair(&p1), &pair(&p2)).len() != 0 {
        // println!("pair");
        winner_to_vec_string(&check_winner(&pair(&p1), &pair(&p2)))
    } else {
        // println!("big");
        winner_to_vec_string(&check_winner(&p1, &p2))
    }
}

