use rand::distributions::Uniform;
use rand::Rng;

fn riffle_shuffle<T>(deck: Vec<T>) -> Vec<T>
where
    T: Copy,
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

    partitioned.into_iter().map(|(_, i)| deck[i]).collect()
}

fn main() {
    let deck = riffle_shuffle(riffle_shuffle((0..52).collect()));
    println!("{:?}", deck);
}
