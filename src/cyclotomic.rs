use crate::integers;

// First: standard library
use std::collections::HashMap;
use std::fmt;
use std::ops::Index;
// Second: our own library
use integers::{euler_phi, invertible_mod};

pub struct CyclotomicInteger {
    level: u32,
    vec: Vec<i32>,
}

impl CyclotomicInteger {

    //////////////////
    // CONSTRUCTORS //
    //////////////////

    pub fn from_vec(vec: Vec<i32>, level: u32) -> Self {
        
        if !euler_phi(level) == vec.len() {
            panic!("the vector should contain Euler-phi elements");
        }

        Self { vec, level }

    }

    pub fn from_hashmap(hashmap: HashMap<usize, i32>, level: u32) -> Self {

        // Note that we use the hashmap and not a reference. This seems
        // more idiomatic: our hashmaps are only used to create a
        // CyclotomicObject. This clearly indicates that the function
        // `from_hashmap` takes ownership of the input hashmap. This is
        // based on the implicit assumption that the hashmap will not be
        // used after.

        if level == 0 {
            panic!("no 0th-roots of unity");
        }

        let euler_phi_level = euler_phi(level);
        let mut vec = vec![0 as i32; euler_phi_level];

        for (key, val) in hashmap.into_iter() {
            if key >= euler_phi_level {
                panic!("hashmap keys should be < than Euler-phi of the level")
            } else {
                vec[key] = val;
            }
        }

        Self { vec, level }

    }


    ///////////
    // UTILS //
    ///////////

    fn euler_phi(&self) -> u32 {
        // WARNING: The existence of this function is debatable
        euler_phi(self.vec.len())
    }

    pub fn level(&self) -> u32 {
        self.level
    }

    pub fn support(&self) -> HashMap<usize, i32> {
    
        let mut support = HashMap::new();

        for i in 0..self.euler_phi() {
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

        let level = self.level();
        let euler = self.euler_phi();
        let mut conjugates_vec: Vec<Vec<i32>> = Vec::new();
        // Now, store the Galois group
        let galois = invertible_mod(level as u32);

        // Generate all conjugates, without repetition
        for k in galois {
            // Generate the conjugate for the k-th Galois automorphism:
            let mut conjugate_vec = vec![0 as i32; euler];
            for i in 0..euler {
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
