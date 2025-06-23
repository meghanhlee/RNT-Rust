use std::collections::HashMap;

struct CyclotomicInteger {
    vec: Vec<u32>,
}

impl CyclotomicInteger {

    //////////////////
    // CONSTRUCTORS //
    //////////////////

    pub fn from_vec(vec: Vec<u32>) -> Self {
        
        Self { vec }

    }

    pub fn from_hashmap(hashmap: HashMap<usize, u32>, level: u32) -> Self {

        if level == 0 {
            panic!("no 0th-roots of unity");
        }

        let mut vec = vec![0 as u32, level];

        for (key, val) in hashmap.into_iter() {
            vec[key as usize] = val;
        }

        Self { vec }

    }


    /////////////////
    // SMALL UTILS //
    /////////////////

    pub fn level(&self) -> usize {
        return self.vec.len()
    }


    //////////////////
    // COMPUTATIONS //
    //////////////////
    
    // pub fn conjugates() -> Vec<CyclotomicInteger> {

    // }

}
