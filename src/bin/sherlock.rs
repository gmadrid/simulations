use simulations::riffle_shuffle;

fn main() {
    let deck: Vec<u8> = (1..53).collect();

    let shuffled = riffle_shuffle(&riffle_shuffle(&deck));
    println!("{:?}", shuffled);
}
