pub trait Game {
    fn done(&self) -> bool;
    fn take_turn(&mut self);
    fn turns(&self) -> u32;
    fn achieved_victory(&self) -> bool;
}

struct GameResult {
    victory: bool,
    turns: u32,
}

fn run_simulation<G>(mut game: G) -> GameResult
where
    G: Game,
{
    while !game.done() {
        game.take_turn();
    }

    GameResult {
        victory: game.achieved_victory(),
        turns: game.turns(),
    }
}

pub fn monte_carlo<F, G>(ng: F) where G: Game, F: Fn() -> G {
    let mut times = 0;
    let mut victories = 0;
    let mut total_turns: u32 = 0;

    for i in 0..100000 {
        times += 1;
        if i % 10000 == 0 {
            println!("{}", i);
        }
        let result = run_simulation(ng());
        total_turns += result.turns;
        if result.victory {
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
