use std::collections::HashMap;

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

        if level == 0 {
            panic!("no 0th-roots of unity");
        }

        let mut vec = vec![0 as i32; level];

        for (key, val) in hashmap.into_iter() {
            vec[key as usize] = val;
        }

        Self { vec }

    }


    ///////////
    // UTILS //
    ///////////

    pub fn level(&self) -> usize {
        return self.vec.len()
    }


    //////////////////
    // COMPUTATIONS //
    //////////////////
    
    // pub fn conjugates() -> Vec<CyclotomicInteger> {

    // }

}
