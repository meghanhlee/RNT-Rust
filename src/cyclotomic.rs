use std::collections::HashMap;
use std::fmt;
use std::ops::Index;

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
        return self.vec.len()
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
    
    // pub fn conjugates() -> Vec<CyclotomicInteger> {

    // }

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
