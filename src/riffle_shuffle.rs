use rand::distributions::Uniform;
use rand::Rng;

// A implementation of a riffle shuffle as described in
//
// [1] D. Bayer and P. Diaconis (1992), "Trailing the Dovetail Shuffle to its Lair,"
//     Annals of Applied Probability 2(2) 294-313.
// [2] R. Wicklin, "An improved simulation of card shuffling," The DO Loop, SAS Blog,
//     https://blogs.sas.com/content/iml/2011/04/20/an-improved-simulation-of-card-shuffling.html

// Note that elements are clone()'d into the destination array, so prefer arrays of
// types that are cheap to clone (or even better, that are 'Copy').
pub fn riffle_shuffle<T>(deck: &Vec<T>) -> Vec<T>
    where
        T: Clone,
{
    let range = Uniform::from(0.0..1.0);
    let mut rng = rand::thread_rng();

    // Start by generating a vector of random numbers from 0..1.
    let mut b: Vec<f32> = (0..deck.len())
        .into_iter()
        .map({ |_| rng.sample(range) })
        .collect();

    // unwrap: should be okay, because probabilities are never NaN.
    b.sort_by({ |p1, p2| p1.partial_cmp(p2).unwrap() });

    let mut partitioned: Vec<(f32, usize)> = b
        .into_iter()
        .map(|p| (f32::from(2.0) * p).fract())
        .zip((0..deck.len()).into_iter())
        .collect();

    partitioned.sort_by(|(p1, _), (p2, _)| p1.partial_cmp(p2).unwrap());

    partitioned.into_iter().map(|(_, i)| deck[i].clone()).collect()
}
