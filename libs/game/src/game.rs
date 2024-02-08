use models::{Card, gen_card};
use platform_types::{sprite, unscaled};
use xs::{Xs, Seed};

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

pub const HZ_BOTTOM: HZ = 32;

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

pub type GridInner = u8;
pub type GridXInner = GridInner;
pub type GridYInner = GridInner;

const GRID_X_MIN: GridXInner = 0;
const GRID_X_MAX: GridXInner = GRID_W - 1;
const GRID_Y_MIN: GridYInner = 0;
const GRID_Y_MAX: GridYInner = GRID_W - 1;

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct GridX(GridXInner);

impl GridX {
    pub const MIN: Self = Self(GRID_X_MIN);
    pub const MAX: Self = Self(GRID_X_MAX);

    pub fn clamped(inner: GridXInner) -> Self {
        Self(
            if inner <= Self::MIN.0 {
                Self::MIN.0
            } else if inner >= Self::MAX.0 {
                Self::MAX.0
            } else {
                inner
            }
        )
    }

    pub fn saturating_sub(self, inner: GridXInner) -> Self {
        Self::clamped(self.0.saturating_sub(inner))
    }

    pub fn saturating_add(self, inner: GridXInner) -> Self {
        Self::clamped(self.0.saturating_add(inner))
    }

    pub fn get(self) -> GridXInner {
        self.0
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct GridY(GridYInner);

impl GridY {
    pub const MIN: Self = Self(GRID_Y_MIN);
    pub const MAX: Self = Self(GRID_Y_MAX);

    pub fn clamped(inner: GridYInner) -> Self {
        Self(
            if inner <= Self::MIN.0 {
                Self::MIN.0
            } else if inner >= Self::MAX.0 {
                Self::MAX.0
            } else {
                inner
            }
        )
    }

    pub fn saturating_sub(self, inner: GridYInner) -> Self {
        Self::clamped(self.0.saturating_sub(inner))
    }

    pub fn saturating_add(self, inner: GridYInner) -> Self {
        Self::clamped(self.0.saturating_add(inner))
    }

    pub fn get(self) -> GridYInner {
        self.0
    }
}

pub fn grid_xy_to_i((x, y): (GridX, GridY)) -> usize {
    grid_xy_inner_to_i((x.get(), y.get()))
}

pub fn grid_xy_inner_to_i((x, y): (GridXInner, GridYInner)) -> usize {
    y as usize * GRID_W as usize + x as usize
}

pub fn grid_i_to_xy(i: usize) -> (GridXInner, GridYInner) {
    (
        i as GridXInner % GRID_W as GridXInner,
        i as GridYInner / GRID_W as GridYInner
    )
}

#[test]
fn grid_xy_to_i_to_xy_is_identity_on_these_examples() {
    assert!(GRID_W >= 3);
    assert!(GRID_H >= 2);
    assert_eq!(grid_i_to_xy(grid_xy_to_i((0, 0))), (0, 0));
    assert_eq!(grid_i_to_xy(grid_xy_to_i((1, 0))), (1, 0));
    assert_eq!(grid_i_to_xy(grid_xy_to_i((0, 2))), (0, 2));
    assert_eq!(grid_i_to_xy(grid_xy_to_i((2, 2))), (2, 2));
}

#[test]
fn grid_i_to_xy_to_i_is_identity_on_these_examples() {
    assert_eq!(grid_xy_to_i(grid_i_to_xy(0)), 0);
    assert_eq!(grid_xy_to_i(grid_i_to_xy(2)), 2);
    assert_eq!(grid_xy_to_i(grid_i_to_xy(3 * GRID_W as usize)), 3 * GRID_W as usize);
    assert_eq!(grid_xy_to_i(grid_i_to_xy(4 * GRID_W as usize + 4)), 4 * GRID_W as usize + 4);
}

pub type PlayerX = GridX;
pub type PlayerY = GridY;

#[repr(u8)]
#[derive(Copy, Clone, Debug, Default)]
pub enum SubFace {
    #[default]
    LeftBottom,
    LeftMiddle,
    LeftTop,
    RightBottom,
    RightMiddle,
    RightTop,
    /// The top slash like this `/`
    TopSlashBottom,
    TopSlashMiddle,
    TopSlashTop,
    /// The top slash like this `\`
    TopBackslashBottom,
    TopBackslashMiddle,
    TopBackslashTop,
}

impl SubFace {
    pub fn sprite_xy(self) -> sprite::XY {
        PLAYER_XYS[self as u8 as usize]
    }

    pub fn wrapping_add_1(self) -> Self {
        use SubFace::*;
        match self {
            LeftBottom => LeftMiddle,
            LeftMiddle => LeftTop,
            LeftTop => RightBottom,
            RightBottom => RightMiddle,
            RightMiddle => RightTop,
            RightTop => TopSlashBottom,
            TopSlashBottom => TopSlashMiddle,
            TopSlashMiddle => TopSlashTop,
            TopSlashTop => TopBackslashBottom,
            TopBackslashBottom => TopBackslashMiddle,
            TopBackslashMiddle => TopBackslashTop,
            TopBackslashTop => LeftBottom,
        }
    }

    pub fn wrapping_sub_1(self) -> Self {
        use SubFace::*;
        match self {
            LeftBottom => TopBackslashTop,
            LeftMiddle => LeftBottom,
            LeftTop => LeftMiddle,
            RightBottom => LeftTop,
            RightMiddle => RightBottom,
            RightTop => RightMiddle,
            TopSlashBottom => RightTop,
            TopSlashMiddle => TopSlashBottom,
            TopSlashTop => TopSlashMiddle,
            TopBackslashBottom => TopSlashTop,
            TopBackslashMiddle => TopBackslashBottom,
            TopBackslashTop => TopBackslashMiddle,
        }
    }
}

#[derive(Clone, Default)]
pub struct Player {
    pub x: PlayerX,
    pub y: PlayerY,
    pub sub_face: SubFace,
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum MoveMode {
    #[default]
    A,
    B,
}

#[derive(Clone)]
pub struct State {
    pub rng: Xs,
    pub camera_x: CameraX,
    pub camera_y: CameraY,
    pub grid: Grid,
    pub player: Player,
    pub move_mode: MoveMode,
    pub debug: [u8; 16],
}

impl State {
    pub fn new(seed: Seed) -> State {
        let mut debug: [u8; 16] = <_>::default();
        debug[2] = 2;
        debug[3] = 2;
        debug[14] = 1;
        let mut rng = xs::from_seed(seed);

        let mut grid = [Cell::default(); GRID_LEN as usize];

        let mut x = GridX::MIN.get();
        let mut y = GridY::MIN.get();
        let mut i = grid_xy_inner_to_i((x, y));
        let mut hz = 1;

        macro_rules! break_cond {
            () => { i < grid.len() }
        }

        #[derive(Clone, Copy, Debug)]
        enum DiagonalOrder {
            Forward,
            Reverse
        }
        use DiagonalOrder::*;

        enum GenMode {
            Switchback(DiagonalOrder),
            DiagonalPlateaus,
        }
        use GenMode::*;

        // TODO? experiment with switching modes during generation.
        // TODO randomize starting mode

        //let mut mode = Switchback(Forward);
        // TODO get filling out the entire grid working
        let mut mode = DiagonalPlateaus;

        while break_cond!() {
            if let Some(cell) = grid.get_mut(i) {
                // Assert that we haven't already set this cell
                assert_eq!(cell.hz, 0, "{:?}", (x, y));

                cell.hz = hz;

                let rolled = xs::range(&mut rng, 0..4);
                cell.cube_i = (1 + (rolled & 0b11)) as _;
            }

            match mode {
                Switchback(Forward) => {
                    if x == GridX::MIN.get() {
                        mode = Switchback(Reverse);

                        // setup for reverse
                        x = GridX::MIN.get();
                        y = y.saturating_add(1);
                    } else {
                        x = x.saturating_sub(1);
                        y = y.saturating_add(1);
                    }
                }
                Switchback(Reverse) => {
                    if y == GridY::MIN.get() {
                        mode = Switchback(Forward);

                        // setup for forward
                        x = x.saturating_add(1);
                        y = GridY::MIN.get();
                    } else {
                        x = x.saturating_add(1);
                        y = y.saturating_sub(1);
                    }
                }
                DiagonalPlateaus => {
                    // Each diagonal, forward
                    if x == GridX::MIN.get() {
                        x = y.saturating_add(1);
                        y = GridY::MIN.get();
                    } else {
                        x = x.saturating_sub(1);
                        y = y.saturating_add(1);
                    }
                }
            }
            i = grid_xy_inner_to_i((x, y));

            if x >= GridX::MAX.get()
            || y >= GridY::MAX.get()  {
                if break_cond!() {
                    break
                } else {
                    continue
                }
            }

            match mode {
                Switchback(_) => {
                    hz += 1;
                }
                DiagonalPlateaus => {
                    // If we just reached a new layer
                    if y == GridY::MIN.get() {
                        hz += 1;
                    }
                }
            }
        }

        // Probably only useful for debugging
        for cell in grid.iter_mut() {
            if cell.hz == 0 {
                cell.hz = HZ_BOTTOM;
            }
        }

        State {
            rng,
            debug,
            grid,
            camera_x: 3,
            camera_y: 1,
            player: <_>::default(),
            move_mode: <_>::default(),
        }
    }

    pub fn player_cell(&self) -> Cell {
        self.grid[grid_xy_to_i((self.player.x, self.player.y))]
    }
}
