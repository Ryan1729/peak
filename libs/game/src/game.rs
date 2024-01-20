use models::{Card, gen_card};
use platform_types::{sprite, unscaled};
use xs::{Xs, Seed};

#[derive(Clone, Default)]
pub struct Splat {
    pub kind: Card,
    pub x: unscaled::X,
    pub y: unscaled::Y,
}

pub const CUBE_W: unscaled::W = unscaled::W(111);
pub const CUBE_H: unscaled::H = unscaled::H(128);

pub const CUBE_XYS: [sprite::XY; 2] = [
    sprite::XY {
        x: sprite::X(128),
        y: sprite::Y(0),
    },
    sprite::XY {
        x: sprite::X(128),
        y: sprite::Y(128),
    },
];

// TODO tighter type?
pub type CubeIndex = u8;

/// Half Z
pub type HZ = u8;

#[derive(Clone, Copy, Default)]
pub struct Cell {
    pub hz: HZ,
    pub cube_i: CubeIndex,
}

pub const GRID_W: u8 = 4;
pub const GRID_H: u8 = 4;
pub const GRID_LEN: u16 = GRID_W as u16 * GRID_H as u16;
pub type Grid = [Cell; GRID_LEN as usize];

#[derive(Clone, Default)]
pub struct State {
    pub rng: Xs,
    pub debug: [u8; 16],
    pub grid: Grid,
}

impl State {
    pub fn new(seed: Seed) -> State {
        let mut debug: [u8; 16] = <_>::default();
        debug[2] = 2;
        debug[3] = 2;
        let rng = xs::from_seed(seed);

        let mut grid = [Cell::default(); GRID_LEN as usize];

        for i in 0..grid.len() {
            grid[i].hz = (i & 0b11) as _;
        }

        State {
            rng,
            debug,
            grid,
            .. <_>::default()
        }
    }
}
