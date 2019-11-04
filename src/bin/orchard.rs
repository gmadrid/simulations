use std::collections::HashMap;

use rand::distributions::Uniform;
use rand::Rng;

struct Variation {
    init_fruits: usize,
    raven_path: usize,
    basket_times: usize,
}

impl Variation {
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

type FruitLeft = HashMap<OrchardDie, usize>;

#[derive(Debug)]
struct Game {
    left: FruitLeft,
    raven_path: usize,
    turns: u16,
    basket_times: usize,
}

impl Game {
    fn new(variation: &Variation) -> Game {
        let mut game = Game {
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

    fn done(&self) -> bool {
        self.raven_path == 0 || self.left.iter().all({ |(_, num)| *num == 0 })
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
}

fn run_simulation() -> (bool, u16) {
    let mut game = Game::new(&Variation::big_game());

    while !game.done() {
        game.take_turn();
    }

    (game.achieved_victory(), game.turns)
}

fn monte_carlo() {
    let mut times = 0;
    let mut victories = 0;
    let mut total_turns: u32 = 0;
    for i in 0..100000 {
        if i % 10000 == 0 {
            println!("{}", i);
        }
        times += 1;
        let (victory, turns) = run_simulation();
        total_turns += u32::from(turns);
        if victory {
            victories += 1;
        }
    }
    println!(
        "\n{}/{} => {}",
        victories,
        times,
        f64::from(victories) / f64::from(times)
    );
    println!("Avg turns: {}", f64::from(total_turns) / f64::from(times));
}

fn main() {
    monte_carlo();
}
