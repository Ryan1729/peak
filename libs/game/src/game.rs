use models::{Card, gen_card};
use platform_types::{command, unscaled};
use xs::{Xs, Seed};

#[derive(Clone, Default)]
pub struct Splat {
    pub kind: Card,
    pub x: unscaled::X,
    pub y: unscaled::Y,
}

#[derive(Clone, Default)]
pub struct State {
    pub rng: Xs,
    pub debug: [u8; 16],
}

impl State {
    pub fn new(seed: Seed) -> State {
        let mut debug: [u8; 16] = <_>::default();
        debug[2] = 2;
        debug[3] = 2;
        let rng = xs::from_seed(seed);

        State {
            rng,
            debug,
            .. <_>::default()
        }
    }
}
