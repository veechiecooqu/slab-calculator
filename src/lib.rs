//! Generates slabs given the size of each slabs as an array.
#![warn(missing_docs)]
/// When given the size of each slab, it will return the slabs, with an extra element
/// for the infinite range.
/// The reason it is done like that is because there can be disagreements between whether the slab sizes are inclusive or exclusive and having a delta forces the implementer to address this ambiguity
///
/// Example
/// ```rust
/// # use slab_calculator::slabbed_values;
/// let slab_result = slabbed_values(&[2, 3, 2, 2], 3);
/// ```
pub fn slabbed_values(slabs: &[i32], num: i32) -> Vec<i32> {
    let mut generated_slabs = Vec::new();
    let mut remainder = num;
    for &slab in slabs {
        if remainder <= slab {
            generated_slabs.push(remainder);
            remainder = 0; // Let's just assume the remainders are flushed for the final slab of
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
    fn slab_test() {
        let slab_result = slabbed_values(&[2, 3, 2, 2], 3);
        assert_eq!(slab_result, vec![2, 1, 0, 0, 0])
    }

    #[test]
    fn slab_bigger_than_limit() {
        let slab_result = slabbed_values(&[2, 3, 2, 2], 13);
        assert_eq!(slab_result, vec![2, 3, 2, 2, 4]);
    }
}
