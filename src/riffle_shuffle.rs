use itertools::Itertools;
use rand::distributions::Uniform;
use rand::Rng;

// A implementation of a riffle shuffle as described in
//
// [1] D. Bayer and P. Diaconis (1992), "Trailing the Dovetail Shuffle to its Lair,"
//     Annals of Applied Probability 2(2) 294-313.
// [2] R. Wicklin, "An improved simulation of card shuffling," The DO Loop, SAS Blog,
//     https://blogs.sas.com/content/iml/2011/04/20/an-improved-simulation-of-card-shuffling.html

// Note that elements are clone()'d into the destination Vec, so prefer slices of
// types that are cheap to clone (or even better, that are 'Copy').
pub fn riffle_shuffle<T>(deck: &[T]) -> Vec<T>
where
    T: Clone,
{
    // NOTE: we can probably make this faster by eliminating one of the sorts and replacing it
    //       with a merge().
    let range = Uniform::from(0.0..1.0);
    let mut rng = rand::thread_rng();

    // Start by generating a vector of random numbers from 0..1.
    let mut b= (0..deck.len())
        .into_iter()
        .map({ |_| rng.sample(range) })
        .collect_vec();

    // unwrap: should be okay, because probabilities are never NaN.
    b.sort_by({ |p1, p2| p1.partial_cmp(p2).unwrap() });

    let mut partitioned = b
        .into_iter()
        .map(|p| (f32::from(2.0) * p).fract())
        .zip((0..deck.len()).into_iter())
        .collect_vec();

    partitioned.sort_by(|(p1, _), (p2, _)| p1.partial_cmp(p2).unwrap());

    partitioned
        .into_iter()
        .map(|(_, i)| deck[i].clone())
        .collect_vec()
}

fn find_break<T>(slice: &[T]) -> Option<usize> where T: PartialOrd {
    slice.iter().tuple_windows()
        .find_position(|(i1, i2)| i1 > i2)
        .map(|(location, _)| location + 1)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_break() {
        assert_eq!(None, find_break(&vec!{ 0, 1, 2, 3, 4, 5, 6 }));
        assert_eq!(Some(4), find_break(&vec!{0, 2, 4, 6, 1, 3, 5 }));
        assert_eq!(Some(5), find_break(&vec!{0, 2, 4, 6, 8, 1}));
        assert_eq!(Some(1), find_break(&vec!{1, 0, 1, 2, 3}));

        // Tests with equal values.
        assert_eq!(None, find_break(&vec!{ 0, 2, 2, 3, 5, 7}));
        assert_eq!(Some(3), find_break(&vec!{ 0, 2, 2, 1, 5, 7}));
        assert_eq!(Some(4), find_break(&vec!{ 0, 2, 4, 6, 2, 4, 6}));
    }
}