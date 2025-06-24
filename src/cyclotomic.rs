use crate::integers;

// First: standard library
use std::collections::HashMap;
use std::fmt;
use std::ops::Index;
use std::f64::consts::TAU;
// Second: our own library
use integers::{euler_phi, invertible_mod};

pub struct CyclotomicInteger {
    vec: Vec<i32>,
}

impl CyclotomicInteger {

    //////////////////
    // CONSTRUCTORS //
    //////////////////

    pub fn from_vec(vec: Vec<i32>) -> Self {
        
        Self { vec }

    }

    pub fn from_hashmap(hashmap: HashMap<usize, i32>, level: usize) -> Self {

        // Note that we use the hashmap and not a reference. This seems
        // more idiomatic: our hashmaps are only used to create a
        // CyclotomicObject. This clearly indicates that the function
        // `from_hashmap` takes ownership of the input hashmap. This is
        // based on the implicit assumption that the hashmap will not be
        // used after.

        if level == 0 {
            panic!("no 0th-roots of unity");
        }

        let mut vec = vec![0 as i32; level];

        for (key, val) in hashmap.into_iter() {
            if key >= level {
                panic!("hashmap keys should be < than the level")
            } else {
                vec[key] = val;
            }
        }

        Self { vec }

    }


    ///////////
    // UTILS //
    ///////////

    pub fn level(&self) -> usize {
        self.vec.len()
    }

    pub fn support(&self) -> HashMap<usize, i32> {
    
        let mut support = HashMap::new();

        for i in 0..self.level() {
            if self[i] != 0 as i32 {
                support.insert(i, self.vec[i]);
            }
        }
    
        support

    }


    //////////////////
    // COMPUTATIONS //
    //////////////////

    pub fn conjugates(&self) -> Vec<CyclotomicInteger> {

        // First, create euler_phi(n) vectors of length `level`:
        let level = self.level();
        let euler = euler_phi(level as u32);
        let mut conjugates_vec: Vec<Vec<i32>> = Vec::new();
        // Now, store the Galois group
        let galois = invertible_mod(level as u32);

        // Generate all conjugates, without repetition
        for k in galois {
            // Generate the conjugate for the k-th Galois automorphism:
            let mut conjugate_vec = vec![0 as i32; level];
            for i in 0..level {
                // TODO: Implement `iter` for our struct
                let index = (i * (k as usize)) % level;
                conjugate_vec[index] = self[i];
            }
            // Check if the conjugate already exists:
            // (We acknowledge that an Hash-thing would be more
            // efficient. However, we would loose the order.
            // We're not sure if we might need it, but let's keep
            // it simple for now.)
            if !conjugates_vec.contains(&conjugate_vec) {
                conjugates_vec.push(conjugate_vec)
            }
        }

        // Cast to CyclotomicInteger
        let mut conjugates: Vec<CyclotomicInteger> = Vec::new();
        for conjugate_vec in conjugates_vec {
            let conjugate = CyclotomicInteger::from_vec(conjugate_vec);
            conjugates.push(conjugate);
        }
        
        conjugates
    }
    
}

// June 24 rewrites

// added use std::f64::consts::TAU;

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0{
        a }
    else { gcd(b, a%b) }
}

fn max_house(exponents: &Vec<u64>, level: u64) -> f64 {
    let mut max_house = 0.0;

    for k in 1..level {
        if gcd(k, level) != 1 {
            continue;
        }

    let mut cos_sum = 0.0;
    let mut sin_sum = 0.0;

    for &i in exponents {
        let angle = TAU * (k * i % level) as f64 / level as f64;
        cos_sum += angle.cos();
        sin_sum += angle.sin();
    }

    let house = cos_sum.powi(2) + sin_sum.powi(2);
    if house > max_house {
        max_house = house;
        }
    }
    max_house
}

// this is zeta_4 ^ 2, which is -1. the max norm is 1, which is
// what we expect.
let exponents = vec![2];
let level = 4;
let result = max_house(&exponents, level);
println!("Maximum house of conjugates is {}", result);

let exponents = vec![0, 1, 2];
let level = 7;
let result = max_house(&exponents, level);
println!("Maximum house of conjugates is {}", result);




////////////
// TRAITS //
////////////

// I just used an LLM for this one... One can also simply add
// #[derive(Debug)] before the struct declaration, but this yields a
// cleaner result. Although, I understand the code less. Also, note that
// we implement the `Debug` trait, so the formatter is "{:?}" and not
// "{}".

impl fmt::Debug for CyclotomicInteger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.vec.fmt(f) 
    }
}

// The next one is to define the [ ] access operator. See the doc here:
//     https://doc.rust-lang.org/std/ops/trait.Index.html

impl Index<usize> for CyclotomicInteger {
    type Output = i32;
    fn index(&self, i: usize) -> &Self::Output {
        &self.vec[i]
    }
}
