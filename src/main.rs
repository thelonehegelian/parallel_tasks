use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::fmt::Debug;

fn print_ref_ref<T: Debug>(x: &&T) {
    println!("Reference to a Reference: {:?}", x);
}

fn main() {
    /*
     * Mutate the elements of an array in parallel
     */
    let mut arr = [0, 7, 9, 11];

    // mutates the array using rayon's par_iter_mut() function
    arr.par_iter_mut().for_each(|p| *p -= 1);
    println!("{:?}", arr);

    /************************************************/

    /*
     * Search elements in parallel
     */

    let mut v = vec![0; 1024];
    for i in 0..v.len() {
        v[i] = i as i32;
    }

    let f1 = v.par_iter().find_any(|&&x| x == 9);
    let f2 = v.par_iter().find_any(|&&x| x % 2 == 0 && x > 20);
    let f3 = v.par_iter().find_any(|&&x| x > 8);

    // println!("f1: {:?}", f1.unwrap());
    // println!("f2: {:?}", f2.unwrap());
    // println!("f3: {:?}", f3.unwrap());

    // iterate over the array in parallel and print the num that matches the predicate
    let _ = v.par_iter().any(|num| {
        let result = *num % 2 == 0;
        if result {
            // println!("{}", num);
        }
        result
    });
    /****************************************/

    /*
     * create a vector of strings and sort them
     */
    // create a vec of empty string
    let mut vec = vec![String::new(); 100_000];
    vec.par_iter_mut().for_each(|p| {
        // println!("{}", p);
        let mut rng = thread_rng();
        // create a random string
        let random_chars = (0..5).map(|_| rng.sample(&Alphanumeric) as char).collect();
        // set to the vec p
        *p = random_chars;
    });
    vec.par_sort_unstable();
    /**********************************************/

    struct Person {
        age: u32,
    }

    let v: Vec<Person> = vec![
        Person { age: 23 },
        Person { age: 19 },
        Person { age: 42 },
        Person { age: 17 },
        Person { age: 17 },
        Person { age: 31 },
        Person { age: 30 },
    ];

    // get the count of number of people in the Persons vec
    let num_over_30 = v.par_iter().filter(|v| v.age <= 30).count() as f32;
    println!("{}", num_over_30);

    // sum the count using reduce
    let sum_over_30 = v.par_iter()
        .map(|x| x.age)
        .filter(|&x| x > 30)
        .reduce(|| 0, |acc, e| acc + e);

    println!("{}", sum_over_30);

    // same using the sum function
    let alt_sum_30: u32 = v.par_iter()
        .map(|x| x.age)
        .filter(|&x| x > 30)
        .sum();

    println!("{}", alt_sum_30);

    // find average
    let avg_over_30 = sum_over_30 as f32 / num_over_30;
    let alt_avg_over_30 = alt_sum_30 as f32/ num_over_30;

    println!("{}", avg_over_30);
    println!("{}", alt_avg_over_30);

    /****************************************/


}
