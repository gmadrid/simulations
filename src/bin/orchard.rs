use simulations::{monte_carlo, Game};

use std::collections::HashMap;

use rand::distributions::Uniform;
use rand::Rng;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "orchard")]
struct Opts {
    /// True to use the "big" version
    #[structopt(long)]
    big: bool,

    /// Number of ravens before you lose
    #[structopt(short, long)]
    ravens: Option<u8>,

    /// Number of fruits (of each color) to start with.
    #[structopt(short, long)]
    fruits: Option<u8>,

    /// Number of fruits to remove when rolling a basket.
    #[structopt(long)]
    basket: Option<u8>,

    /// Number of iterations for the monte carlo simulation.
    #[structopt(short="n", long, default_value="100000")]
    iterations: u32,
}

struct Variation {
    init_fruits: u8,
    raven_path: u8,
    basket_times: u8,
}

impl Variation {
    fn from_opts(opts: &Opts) -> Variation {
        let mut result = if opts.big {
            Variation::big_game()
        } else {
            Variation::little_game()
        };

        opts.ravens.map(|v| result.raven_path = v);
        opts.fruits.map(|v| result.init_fruits = v);
        opts.basket.map(|v| result.basket_times = v);
        
        result
    }

    fn little_game() -> Variation {
        Variation {
            init_fruits: 4,
            raven_path: 5,
            basket_times: 1,
        }
    }

    fn big_game() -> Variation {
        Variation {
            init_fruits: 10,
            raven_path: 9,
            basket_times: 2,
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum OrchardDie {
    Yellow,
    Green,
    Purple,
    Red,
    Basket,
    Raven,
}

impl OrchardDie {
    fn roll() -> OrchardDie {
        let range = Uniform::from(0..6);
        let mut rng = rand::thread_rng();

        use OrchardDie::*;
        match rng.sample(range) {
            0 => Yellow,
            1 => Green,
            2 => Purple,
            3 => Red,
            4 => Basket,
            5 => Raven,
            _ => panic!("XXX"),
        }
    }
}

type FruitLeft = HashMap<OrchardDie, u8>;

#[derive(Debug)]
struct Orchard {
    left: FruitLeft,
    raven_path: u8,
    turns: u32,
    basket_times: u8,
}

impl Orchard {
    fn new(variation: &Variation) -> Orchard {
        let mut game = Orchard {
            left: HashMap::new(),
            raven_path: variation.raven_path,
            turns: 0,
            basket_times: variation.basket_times,
        };

        use OrchardDie::*;

        game.left.insert(Yellow, variation.init_fruits);
        game.left.insert(Green, variation.init_fruits);
        game.left.insert(Purple, variation.init_fruits);
        game.left.insert(Red, variation.init_fruits);

        game
    }

    fn decrement(&mut self, die: OrchardDie) {
        let val = self.left[&die];
        if val > 0 {
            self.left.insert(die, val - 1);
        }
    }

    fn find_max_die(&self) -> OrchardDie {
        // unwrap: okay because self.left will never be empty.
        *self
            .left
            .iter()
            .max_by(|(_, v1), (_, v2)| v1.cmp(v2))
            .unwrap()
            .0
    }
}

impl Game for Orchard {
    fn done(&self) -> bool {
        self.raven_path == 0 || self.left.iter().all({ |(_, num)| *num == 0 })
    }

    fn take_turn(&mut self) {
        self.turns += 1;
        use OrchardDie::*;
        let roll = OrchardDie::roll();
        match roll {
            Yellow | Green | Purple | Red => self.decrement(roll),
            Basket => {
                for _ in 0..self.basket_times {
                    self.decrement(self.find_max_die());
                }
            }
            Raven => self.raven_path -= 1,
        }
    }

    fn achieved_victory(&self) -> bool {
        self.done() && self.raven_path > 0
    }

    fn turns(&self) -> u32 {
        self.turns
    }
}

fn main() {
    let opts = Opts::from_args();
    let variation = Variation::from_opts(&opts);

    monte_carlo(opts.iterations, || Orchard::new(&variation));
}
