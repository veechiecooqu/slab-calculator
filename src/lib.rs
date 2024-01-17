//! Generates slabs given the size of each slabs as an array.
#![warn(missing_docs)]

/// Add two numbers given their left and rights.
///
/// Example
/// ```rust
/// # use slab_calculator::add;
/// let result = add(2, 2);
/// ```
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

/// When given the size of each slab, as a vec, it will return the slabs, with an extra element
/// for the infinite range.
///
/// Example
/// ```rust
/// # use slab_calculator::slabbed_values;
/// let slab_result = slabbed_values(&vec![2, 3, 2, 2], 3);
/// ```
pub fn slabbed_values(slabs: &Vec<u32>, num: u32) -> Vec<u32> {
    let mut generated_slabs = Vec::new();
    let mut remainder = num;
    for &slab in slabs {
        if remainder <= slab {
            generated_slabs.push(remainder);
            remainder = 0; // let's just assume the remainders are flushed for the final slab of
                           // infinity
        } else {
            generated_slabs.push(slab);
            remainder -= slab;
        }
    }
    generated_slabs.push(remainder); // pushes the values out of slab to the final step
    generated_slabs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn slab_test() {
        let slab_result = slabbed_values(&vec![2, 3, 2, 2], 3);
        assert_eq!(slab_result, vec![2, 1, 0, 0, 0])
    }

    #[test]
    fn slab_bigger_than_limit() {
        let slab_result = slabbed_values(&vec![2, 3, 2, 2], 13);
        assert_eq!(slab_result, vec![2, 3, 2, 2, 4]);
    }
}
