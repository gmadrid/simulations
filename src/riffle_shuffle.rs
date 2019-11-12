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
    shuffle_with_samples(deck, random_samples(deck.len()))
}

// For testability, we separate out the random part from the deterministic part.
fn shuffle_with_samples<T>(deck: &[T], mut samples: Vec<f32>) -> Vec<T>
where
    T: Clone,
{
    // unwrap: should be okay, because probabilities are never NaN.
    samples.sort_by({ |p1, p2| p1.partial_cmp(p2).unwrap() });

    // Scale the probabilities from (0, 1] to (0, 2], and make a tuple with (index, prob).
    let scaled = samples
        .into_iter()
        .map(|p| (f32::from(2.0) * p).fract())
        .zip((0..deck.len()).into_iter())
        // NOTE: see if you can eliminate this collect_vec().
        .collect_vec();

    let break_point = find_break(&scaled).unwrap_or(scaled.len());

    // Merge the two partitions together, extract the indices, then copy values from deck.
    scaled[..break_point]
        .iter()
        .merge_by(scaled[break_point..].iter(), |(p1, _), (p2, _)| p1 < p2)
        .map(|(_, i)| deck[*i].clone())
        .collect_vec()
}

// Return a Vec with size elements randomly selected between (0, 1].
fn random_samples(size: usize) -> Vec<f32> {
    let range = Uniform::from(0.0..1.0);
    let mut rng = rand::thread_rng();
    std::iter::repeat_with(|| rng.sample(range))
        .take(size)
        .collect_vec()
}

fn find_break<T>(slice: &[T]) -> Option<usize>
where
    T: PartialOrd,
{
    slice
        .iter()
        .tuple_windows()
        .find_position(|(i1, i2)| i1 > i2)
        .map(|(location, _)| location + 1)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_basic_shuffle() {
        let deck = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let samples = vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.51, 0.61, 0.71, 0.81, 0.91];
        assert_eq!(deck.len(), samples.len());

        assert_eq!(
            shuffle_with_samples(&deck, samples),
            vec! { 1, 6, 2, 7, 3, 8, 4, 9, 5, 10}
        );

        // What if there's no break point?
        assert_eq!(
            shuffle_with_samples(
                &deck,
                vec![0.0, 0.05, 0.06, 0.07, 0.08, 0.09, 0.10, 0.11, 0.12, 0.13]
            ),
            vec! { 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 }
        );
    }

    #[test]
    fn test_find_break() {
        assert_eq!(None, find_break(&vec! { 0, 1, 2, 3, 4, 5, 6 }));
        assert_eq!(Some(4), find_break(&vec! {0, 2, 4, 6, 1, 3, 5 }));
        assert_eq!(Some(5), find_break(&vec! {0, 2, 4, 6, 8, 1}));
        assert_eq!(Some(1), find_break(&vec! {1, 0, 1, 2, 3}));

        // Tests with equal values.
        assert_eq!(None, find_break(&vec! { 0, 2, 2, 3, 5, 7}));
        assert_eq!(Some(3), find_break(&vec! { 0, 2, 2, 1, 5, 7}));
        assert_eq!(Some(4), find_break(&vec! { 0, 2, 4, 6, 2, 4, 6}));
    }
}
