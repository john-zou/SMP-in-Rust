fn main() {
    test_1();
    test_2();
    test_3();
}

fn test_3(){
    println!("");
    println!("--------------");
    println!("Test 3: 15 men and women with various different preferences, checked with stability checking function");
    println!("");
    let mut men : Vec<&mut Man> = Vec::new();
    let mut women : Vec<&mut Woman> = Vec::new();
    let mut m1 = Man {
        id: 1,
        preferences: vec![9, 12, 3, 8, 14, 7, 13, 15, 6, 4, 5, 10, 2, 11, 1],
        engaged_to: None,
    };
    let mut m2 = Man {
        id: 2,
        preferences: vec![15, 8, 9, 4, 13, 2, 6, 10, 7, 3, 14, 5, 1, 12, 11],
        engaged_to: None,
    };
    let mut m3 = Man {
        id: 3,
        preferences: vec![9, 5, 7, 11, 8, 2, 15, 4, 1, 10, 6, 14, 3, 13, 12],
        engaged_to: None,
    };    
    let mut m4 = Man {
        id: 4,
        preferences: vec![14, 8, 2, 13, 11, 7, 5, 4, 10, 15, 6, 12, 9, 3, 1],
        engaged_to: None,
    };
    let mut m5 = Man {
        id: 5,
        preferences: vec![7, 8, 10, 14, 11, 12, 15, 4, 9, 13, 3, 5, 6, 2, 1],
        engaged_to: None,
    };
    let mut m6 = Man {
        id: 6,
        preferences: vec![8, 14, 3, 15, 2, 7, 1, 9, 10, 6, 13, 4, 11, 5, 12],
        engaged_to: None,
    };
    let mut m7 = Man {
        id: 7,
        preferences: vec![11, 7, 4, 9, 5, 3, 1, 6, 8, 15, 14, 13, 10, 12, 2],
        engaged_to: None,
    };
    let mut m8 = Man {
        id: 8,
        preferences: vec![1, 7, 13, 9, 11, 10, 8, 15, 6, 3, 2, 5, 12, 14, 4],
        engaged_to: None,
    };
    let mut m9 = Man {
        id: 9,
        preferences: vec![7, 1, 11, 12, 10, 4, 14, 3, 6, 2, 8, 5, 15, 13, 9],
        engaged_to: None,
    };   
    let mut m10 = Man {
        id: 10,
        preferences: vec![15, 6, 2, 12, 8, 3, 7, 9, 10, 11, 4, 13, 14, 5, 1],
        engaged_to: None,
    };
    let mut m11 = Man {
        id: 11,
        preferences: vec![6, 15, 4, 14, 8, 10, 3, 2, 9, 13, 1, 12, 7, 5, 11],
        engaged_to: None,
    };
    let mut m12 = Man {
        id: 12,
        preferences: vec![14, 8, 7, 9, 4, 1, 10, 11, 6, 5, 15, 2, 3, 12, 13],
        engaged_to: None,
    };   
    let mut m13 = Man {
        id: 13,
        preferences: vec![1, 13, 9, 8, 2, 12, 3, 15, 14, 10, 6, 11, 7, 5, 4],
        engaged_to: None,
    };
    let mut m14 = Man {
        id: 14,
        preferences: vec![11, 10, 6, 9, 7, 13, 2, 15, 14, 4, 1, 12, 8, 3, 5],
        engaged_to: None,
    };
    let mut m15 = Man {
        id: 15,
        preferences: vec![10, 4, 8, 15, 14, 3, 7, 1, 13, 5, 12, 2, 6, 9, 11],
        engaged_to: None,
    };   
    let mut w1 = Woman {
        id: 1,
        preferences: vec![1, 4, 5, 13, 8, 10, 12, 2, 14, 9, 7, 3, 15, 6, 11],
        engaged_to: None,
    };
    let mut w2 = Woman {
        id: 2,
        preferences: vec![9, 15, 11, 3, 1, 6, 2, 7, 10, 4, 14, 12, 13, 8, 5],
        engaged_to: None,
    };
    let mut w3 = Woman {
        id: 3,
        preferences: vec![3, 14, 8, 12, 9, 5, 11, 1, 13, 7, 2, 10, 15, 4, 6],
        engaged_to: None,
    };    
    let mut w4 = Woman {
        id: 4,
        preferences: vec![3, 12, 4, 15, 9, 10, 5, 1, 13, 8, 2, 11, 7, 6, 14],
        engaged_to: None,
    };
    let mut w5 = Woman {
        id: 5,
        preferences: vec![9, 1, 11, 2, 7, 3, 13, 4, 8, 12, 5, 14, 6, 10, 15],
        engaged_to: None,
    };
    let mut w6 = Woman {
        id: 6,
        preferences: vec![14, 4, 3, 6, 15, 11, 10, 9, 8, 2, 13, 7, 12, 1, 5],
        engaged_to: None,
    };
    let mut w7 = Woman {
        id: 7,
        preferences: vec![5, 14, 4, 10, 7, 3, 1, 6, 15, 8, 11, 13, 12, 2, 9],
        engaged_to: None,
    };
    let mut w8 = Woman {
        id: 8,
        preferences: vec![5, 11, 1, 14, 12, 6, 4, 9, 8, 7, 2, 10, 15, 13, 3],
        engaged_to: None,
    };
    let mut w9 = Woman {
        id: 9,
        preferences: vec![4, 12, 15, 1, 5, 11, 14, 9, 7, 3, 2, 13, 10, 8, 6],
        engaged_to: None,
    };   
    let mut w10 = Woman {
        id: 10,
        preferences: vec![14, 4, 7, 5, 1, 9, 2, 8, 12, 11, 6, 15, 10, 3, 13],
        engaged_to: None,
    };
    let mut w11 = Woman {
        id: 11,
        preferences: vec![7, 3, 12, 10, 13, 11, 14, 8, 6, 5, 9, 1, 2, 4, 15],
        engaged_to: None,
    };
    let mut w12 = Woman {
        id: 12,
        preferences: vec![9, 8, 7, 12, 15, 14, 5, 3, 6, 4, 10, 1, 13, 11, 2],
        engaged_to: None,
    };   
    let mut w13 = Woman {
        id: 13,
        preferences: vec![8, 13, 11, 1, 3, 4, 9, 5, 7, 10, 6, 15, 14, 2, 12],
        engaged_to: None,
    };
    let mut w14 = Woman {
        id: 14,
        preferences: vec![11, 8, 14, 13, 3, 4, 5, 2, 15, 1, 7, 10, 12, 9, 6],
        engaged_to: None,
    };
    let mut w15 = Woman {
        id: 15,
        preferences: vec![6, 12, 10, 9, 1, 8, 2, 13, 11, 15, 5, 7, 4, 14, 3],
        engaged_to: None,
    };   

    men.push(&mut m1);
    men.push(&mut m2);
    men.push(&mut m3);
    men.push(&mut m4);
    men.push(&mut m5);
    men.push(&mut m6);
    men.push(&mut m7);
    men.push(&mut m8);
    men.push(&mut m9);
    men.push(&mut m10);
    men.push(&mut m11);
    men.push(&mut m12);
    men.push(&mut m13);
    men.push(&mut m14);
    men.push(&mut m15);
    women.push(&mut w1);
    women.push(&mut w2);
    women.push(&mut w3);
    women.push(&mut w4);
    women.push(&mut w5);
    women.push(&mut w6);
    women.push(&mut w7);
    women.push(&mut w8);
    women.push(&mut w9);
    women.push(&mut w10);
    women.push(&mut w11);
    women.push(&mut w12);
    women.push(&mut w13);
    women.push(&mut w14);
    women.push(&mut w15);
    stable_marriage(&mut men, &mut women);

    // Stability check
    println!("");
    println!("Stability check:");
    // go through each woman's engaged_to, and for every male with higher position than her engaged_to, see if this woman is OFF that male's preference list
    // if so, then that male prefers her to his currently-engaged. this is the only source of instability
    // Note: relies on every preference list to initially contain every member of opposite sex
    let mut passed = true;
    'outer: for w in women.into_iter(){
        match w.engaged_to {
            None => break,
            Some(man) => {
                let position = w.preferences.iter().position(|&m| m == man).unwrap();
                'inner: for i in position + 1 .. w.preferences.len(){
                    // check every man with position (rating) higher than her final engaged
                    let higher_man = men.iter().find(|&m| m.id == w.preferences[i]).unwrap();
                    match higher_man.preferences.iter().find(|&&woman| woman == w.id){
                        None => {
                            passed = false;
                            break 'outer;
                        }
                        Some(_w) => continue,
                    }
                }
            }
        }
    }
    match passed{
        true => println!("Passed!"),
        false => println!("Failed!")
    }
}

fn test_2(){
    println!("");
    println!("--------------");
    println!("Test 2: Men and women have same preferences, 5 each");
    println!("");
    let mut men : Vec<&mut Man> = Vec::new();
    let mut women : Vec<&mut Woman> = Vec::new();
    let mut m1 = Man {
        id: 1,
        preferences: vec![1,2,3,4,5],
        engaged_to: None,
    };
    let mut m2 = Man {
        id: 2,
        preferences: vec![1,2,3,4,5],
        engaged_to: None,
    };
    let mut m3 = Man {
        id: 3,
        preferences: vec![1,2,3,4,5],
        engaged_to: None,
    };
    let mut m4 = Man {
        id: 4,
        preferences: vec![1,2,3,4,5],
        engaged_to: None,
    };
    let mut m5 = Man {
        id: 5,
        preferences: vec![1,2,3,4,5],
        engaged_to: None,
    };
    let mut w1 = Woman {
        id: 1,
        preferences: vec![1,2,3,4,5],
        engaged_to: None,
    };
    let mut w2 = Woman {
        id: 2,
        preferences: vec![1,2,3,4,5],
        engaged_to: None,
    };
    let mut w3 = Woman {
        id: 3,
        preferences: vec![1,2,3,4,5],
        engaged_to: None,
    };
    let mut w4 = Woman {
        id: 4,
        preferences: vec![1,2,3,4,5],
        engaged_to: None,
    };
    let mut w5 = Woman {
        id: 5,
        preferences: vec![1,2,3,4,5],
        engaged_to: None,
    };

    men.push(&mut m1);
    men.push(&mut m2);
    men.push(&mut m3);
    men.push(&mut m4);
    men.push(&mut m5);
    women.push(&mut w1);
    women.push(&mut w2);
    women.push(&mut w3);
    women.push(&mut w4);
    women.push(&mut w5);
    stable_marriage(&mut men, &mut women);
}

fn test_1(){
    println!("");
    println!("--------------");
    println!("Test 1: Men and women have same preferences, 2 each");
    println!("");
    let mut men : Vec<&mut Man> = Vec::new();
    let mut women : Vec<&mut Woman> = Vec::new();
    let mut m1 = Man {
        id: 1,
        preferences: vec![1,2],
        engaged_to: None,
    };
    let mut m2 = Man {
        id: 2,
        preferences: vec![1,2],
        engaged_to: None,
    };
    let mut w1 = Woman {
        id: 1,
        preferences: vec![1,2],
        engaged_to: None,
    };
    let mut w2 = Woman {
        id: 2,
        preferences: vec![1,2],
        engaged_to: None,
    };


    men.push(&mut m1);
    men.push(&mut m2);
    women.push(&mut w1);
    women.push(&mut w2);
    stable_marriage(&mut men, &mut women);
}

struct Man {
    id: i32,
    preferences: Vec<i32>, // highest index = most preferable
    engaged_to: Option<i32>,
}

struct Woman {
    id: i32,
    preferences: Vec<i32>, // highest index = most preferable
    engaged_to: Option<i32>,
}

fn stable_marriage(men: &mut Vec<&mut Man>, women: &mut Vec<&mut Woman>){
    // initialize
    for i in 0..men.len(){
        men[i].engaged_to = None;
    }
    for i in 0..women.len(){
        women[i].engaged_to = None;
    }

    loop {
        let next_man = men.iter_mut().find(|m| m.engaged_to.is_none());
        match next_man {
            None => break, // end loop if no such man
            Some(man) => {
                let woman_id = man.preferences.pop().unwrap();
                let woman = women.iter_mut().find(|w| w.id == woman_id).unwrap();
                if woman.engaged_to.is_none(){
                    woman.engaged_to = Some(man.id);
                    man.engaged_to = Some(woman.id);
                } else {
                    let prev_man = woman.engaged_to.unwrap();
                    let prev_rating = woman.preferences.iter().position(|&m| m == prev_man);
                    let next_rating = woman.preferences.iter().position(|&m| m == man.id);
                    if next_rating > prev_rating {
                        // engage to this next man
                        woman.engaged_to = Some(man.id);
                        man.engaged_to = Some(woman.id);
                        // prev man now unengaged
                        let prev_man = men.iter_mut().find(|m| m.id == prev_man).unwrap();
                        prev_man.engaged_to = None;
                    }
                }
            }
        }
    }

    // Report set of engaged pairs as final matching
    println!("Reporting engaged pairs: ");
    for m in men.into_iter(){
        if m.engaged_to.is_some(){
            println!("Man: {0} ~ Woman: {1}", m.id, m.engaged_to.unwrap());
        }
    }
}