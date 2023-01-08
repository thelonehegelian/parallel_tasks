use rayon::prelude::*;

fn main() {
    let mut arr = [0, 7, 9, 11];
    let mut v = vec![0; 1024];
    v.par_iter_mut()
        .enumerate()
        .for_each(|(i, v)| println!("{}{}", i, v));

    arr.par_iter_mut().for_each(|p| *p -= 1);
    println!("{:?}", arr);
}
