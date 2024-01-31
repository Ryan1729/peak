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

const CUBE_BASE_X: sprite::X = sprite::X(128);
const CUBE_X_ADV: sprite::W = sprite::W(112);

const CUBE_BASE_Y: sprite::Y = sprite::Y(0);
const CUBE_Y_ADV: sprite::H = sprite::H(128);

macro_rules! compile_time_assert {
    ($assertion: expr) => {{
        #[allow(unknown_lints, eq_op)]
        // Based on the const_assert macro from static_assertions;
        const _: [(); 0 - !{$assertion} as usize] = [];
    }}
}

pub const CUBE_XYS: [sprite::XY; 6] = {
    use sprite::x_const_add_w as add_w;
    use sprite::y_const_add_h as add_h;

    compile_time_assert!{
        sprite::x_const_add_w(CUBE_BASE_X, CUBE_X_ADV).0 < 32768
    }

    const X0: sprite::X = CUBE_BASE_X;
    const X1: sprite::X = add_w(X0, CUBE_X_ADV);
    const X2: sprite::X = add_w(X1, CUBE_X_ADV);

    const Y0: sprite::Y = CUBE_BASE_Y;
    const Y1: sprite::Y = add_h(Y0, CUBE_Y_ADV);

    compile_time_assert!{
        Y0.0 < 32768
    }
    compile_time_assert!{
        Y1.0 < 32768
    }

    [
        sprite::XY {
            x: X0,
            y: Y0,
        },
        sprite::XY {
            x: X1,
            y: Y0,
        },
        sprite::XY {
            x: X2,
            y: Y0,
        },
        sprite::XY {
            x: X0,
            y: Y1,
        },
        sprite::XY {
            x: X1,
            y: Y1,
        },
        sprite::XY {
            x: X2,
            y: Y1,
        },
    ]
};

const PLAYER_BASE_X: sprite::X = sprite::X(128);
const PLAYER_X_ADV: sprite::W = sprite::W(112);

const PLAYER_BASE_Y: sprite::Y = sprite::Y(256);
const PLAYER_Y_ADV: sprite::H = sprite::H(128);

pub const PLAYER_XYS: [sprite::XY; 12] = {
    use sprite::x_const_add_w as add_w;
    use sprite::y_const_add_h as add_h;

    const X0: sprite::X = PLAYER_BASE_X;
    const X1: sprite::X = add_w(X0, PLAYER_X_ADV);
    const X2: sprite::X = add_w(X1, PLAYER_X_ADV);

    const Y0: sprite::Y = PLAYER_BASE_Y;
    const Y1: sprite::Y = add_h(Y0, PLAYER_Y_ADV);
    const Y2: sprite::Y = add_h(Y1, PLAYER_Y_ADV);
    const Y3: sprite::Y = add_h(Y2, PLAYER_Y_ADV);

    [
        sprite::XY {
            x: X0,
            y: Y0,
        },
        sprite::XY {
            x: X1,
            y: Y0,
        },
        sprite::XY {
            x: X2,
            y: Y0,
        },
        sprite::XY {
            x: X0,
            y: Y1,
        },
        sprite::XY {
            x: X1,
            y: Y1,
        },
        sprite::XY {
            x: X2,
            y: Y1,
        },
        sprite::XY {
            x: X0,
            y: Y2,
        },
        sprite::XY {
            x: X1,
            y: Y2,
        },
        sprite::XY {
            x: X2,
            y: Y2,
        },
        sprite::XY {
            x: X0,
            y: Y3,
        },
        sprite::XY {
            x: X1,
            y: Y3,
        },
        sprite::XY {
            x: X2,
            y: Y3,
        },
    ]
};

// TODO tighter type?
pub type CubeIndex = u8;

/// Half Z
pub type HZ = u8;

pub const HZ_BOTTOM: HZ = 16;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct Cell {
    pub hz: HZ,
    pub cube_i: CubeIndex,
}

impl core::fmt::Debug for Cell {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("Cell")
             .field("hz", &self.hz)
             .field("cube_i", &self.cube_i)
             .finish()
        } else {
            write!(f, "z:{};i:{}", self.hz, self.cube_i)
        }
    }
}

pub const GRID_W: u8 = 16;
pub const GRID_H: u8 = 16;
pub const GRID_LEN: u16 = GRID_W as u16 * GRID_H as u16;
pub type Grid<const LEN: usize = {GRID_LEN as usize}> = [Cell; LEN];

pub type CameraX = i16;
pub type CameraY = i16;

#[derive(Clone)]
pub struct State {
    pub rng: Xs,
    pub camera_x: CameraX,
    pub camera_y: CameraY,
    pub debug: [u8; 16],
    pub grid: Grid,
}

impl State {
    pub fn new(seed: Seed) -> State {
        let mut debug: [u8; 16] = <_>::default();
        debug[2] = 2;
        debug[3] = 2;
        debug[14] = 1;
        let mut rng = xs::from_seed(seed);

        let mut grid = [Cell::default(); GRID_LEN as usize];

        for i in 0..grid.len() {
            let rolled = xs::range(&mut rng, 0..4);
            grid[i].cube_i = (1 + (rolled & 0b11)) as _;
            grid[i].hz = (rolled & 0b11) as _;
        }

        State {
            rng,
            debug,
            grid,
            camera_x: 3,
            camera_y: 1,
        }
    }
}
