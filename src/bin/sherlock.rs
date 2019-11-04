use simulations::riffle_shuffle;

fn main() {
    let deck = (1..53).collect();

    let shuffled = riffle_shuffle(&riffle_shuffle(&deck));
    println!("{:?}", shuffled);
}
