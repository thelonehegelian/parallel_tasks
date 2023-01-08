use rayon::prelude::*;
use std::fmt::Debug;

fn print_ref_ref<T: Debug>(x: &&T) {
    println!("Reference to a Reference: {:?}", x);
}

fn main() {
    let mut arr = [0, 7, 9, 11];
    let mut v = vec![0; 1024];
    for i in 0..v.len() {
        v[i] = i as i32;
    }
    //  search a vector in parallel for an element satisfying the predicate in the given closure
    let f1 = v.par_iter().find_any(|&&x| x == 9);
    let f2 = v.par_iter().find_any(|&&x| x % 2 == 0 && x > 20);
    let f3 = v.par_iter().find_any(|&&x| x > 8);

    println!("f1: {:?}", f1.unwrap());
    println!("f2: {:?}", f2.unwrap());
    println!("f3: {:?}", f3.unwrap());

    // iterate over the array in parallel and print the num that matches the predicate
    let _ = v.par_iter().any(|num| {
        let result = *num % 2 == 0;
        if result {
            println!("{}", num);
        }
        result
    });

    // mutates the array using rayon's par_iter_mut() function
    // arr.par_iter_mut().for_each(|p| *p -= 1);
    // println!("{:?}", arr);
}
