use std::collections::HashMap;

fn main() {
    assert_eq!(recover_secret(vec![ 
    ['t','u','p'],
    ['w','h','i'],
    ['t','s','u'],
    ['a','t','s'],
    ['h','a','p'],
    ['t','i','s'],
    ['w','h','s']])
    , "whatisup");
}

#[derive(Clone, Debug)]
struct Singlet {
    before: Vec<char>,
    after: Vec<char>
}

fn recover_secret(triplets: Vec<[char; 3]>) -> String {
    let mut output = String::new();
    let mut singlets = HashMap::<char, Singlet>::new();
    triplets.iter().for_each(|triplet| {
        let [first, second, third] = triplet;
        match singlets.contains_key(first) {
            true => {
                let prev = singlets.get(&first).unwrap();
                let mut before = prev.before.clone();
                if !prev.before.contains(second) {
                    before.push(*second);
                }
                if !prev.before.contains(third) {
                    before.push(*third);
                }
                let after: Vec<char> = prev.after.clone();
                singlets.insert(*first, Singlet {
                    before, after
                })
            },
            false => singlets.insert(*first, Singlet {
                before: vec![*second, *third],
                after: vec![],
            }),
        };
        match singlets.contains_key(second) {
            true => {
                let prev = singlets.get(&second).unwrap();
                let mut before = prev.before.clone();
                if !prev.before.contains(third) {
                    before.push(*third);
                }
                let mut after = prev.after.clone();
                if !prev.after.contains(first) {
                    after.push(*first);
                }
                singlets.insert(*second, Singlet {
                    before, after
                })
            },
            false => singlets.insert(*second, Singlet {
                before: vec![*third],
                after: vec![*first],
            }),
        };
        match singlets.contains_key(third) {
            true => {
                let prev = singlets.get(&third).unwrap();
                let before = prev.before.clone();
                let mut after = prev.after.clone();
                if !prev.after.contains(first) {
                    after.push(*first);
                }
                if !prev.after.contains(second) {
                    after.push(*second);
                }
                singlets.insert(*third, Singlet {
                    before, after
                })
            },
            false => singlets.insert(*third, Singlet {
                before: vec![],
                after: vec![*first, *second],
            }),
        };
    });
    println!("{:?}", singlets);
    for _ in 0..singlets.len() {
        let cloned_singlets = singlets.clone();
        let next = match cloned_singlets.iter().find(|(_, singlet)| {
            singlet.after.capacity() == 0
        }) {
            Some(value) => value,
            None => todo!(),
        };
        singlets.remove(next.0);
        println!("{:?}", singlets);
        output.push(next.0.clone());
        singlets.iter_mut().for_each(|(char, singlet)| {
            let Some(index) = singlet.after.iter().position(|val| val == next.0) else {
                println!("{} not found in {}", next.0, char);
                return;
            };
            println!("{} found at index {} in {}", next.0, index, char);
            singlet.after.swap_remove(index);
            
        });
        println!("{}", output);
    }
    output
}