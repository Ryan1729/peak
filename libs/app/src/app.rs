use game::{CUBE_H, CUBE_W, GRID_W, GRID_H, HZ, HZ_BOTTOM, Cell, Grid};
use gfx::{Commands};
use platform_types::{command, sprite, unscaled, Button, Input, Speaker, SFX};
pub use platform_types::StateParams;

pub struct State {
    pub game_state: game::State,
    pub commands: Commands,
    pub input: Input,
    pub speaker: Speaker,
}

impl State {
    pub fn new((seed, logger, error_logger): StateParams) -> Self {
        unsafe {
            features::GLOBAL_LOGGER = logger;
            features::GLOBAL_ERROR_LOGGER = error_logger;
        }

        let seed = [250, 32, 206, 198, 29, 107, 217, 65, 131, 103, 255, 37, 147, 36, 4, 62];
        // We always want to log the seed, if there is a logger available, so use the function,
        // not the macro.
        features::log(&format!("{:?}", seed));

        let mut game_state = game::State::new(seed);

        Self {
            game_state,
            commands: Commands::default(),
            input: Input::default(),
            speaker: Speaker::default(),
        }
    }
}

impl platform_types::State for State {
    fn frame(&mut self) -> (&[platform_types::Command], &[SFX]) {
        self.commands.clear();
        self.speaker.clear();
        update_and_render(
            &mut self.commands,
            &mut self.game_state,
            self.input,
            &mut self.speaker,
        );

        self.input.previous_gamepad = self.input.gamepad;

        (self.commands.slice(), self.speaker.slice())
    }

    fn press(&mut self, button: Button) {
        if self.input.previous_gamepad.contains(button) {
            //This is meant to pass along the key repeat, if any.
            //Not sure if rewriting history is the best way to do this.
            self.input.previous_gamepad.remove(button);
        }

        self.input.gamepad.insert(button);
    }

    fn release(&mut self, button: Button) {
        self.input.gamepad.remove(button);
    }
}

const DEBUG_MODE: usize = 14;
const DEBUG_I: usize = 15;
const DEBUG_GRID_X_START: usize = 0;
const DEBUG_GRID_Y_START: usize = 1;
const DEBUG_GRID_X_END: usize = 2;
const DEBUG_GRID_Y_END: usize = 3;
const DEBUG_Z1: usize = 4;
const DEBUG_Z2: usize = 5;

fn update(state: &mut game::State, input: Input, speaker: &mut Speaker) {
    let pressed = input.button_pressed_this_frame();

    match pressed {
        Some(Button::START) => {
            state.debug[DEBUG_MODE] = state.debug[DEBUG_MODE].wrapping_sub(1);
        }
        Some(Button::SELECT) => {
            state.debug[DEBUG_MODE] = state.debug[DEBUG_MODE].wrapping_add(1);
        }
        None | _ => {}
    }

    match state.debug[DEBUG_MODE] {
        1 => {
            match pressed {
                Some(Button::UP) => {
                    state.camera_y = state.camera_y.wrapping_sub(1);
                }
                Some(Button::DOWN) => {
                    state.camera_y = state.camera_y.wrapping_add(1);
                }
                Some(Button::LEFT) => {
                    state.camera_x = state.camera_x.wrapping_sub(1);
                }
                Some(Button::RIGHT) => {
                    state.camera_x = state.camera_x.wrapping_add(1);
                }
                None | _ => {}
            }
        }
        0 | _ => {
            match pressed {
                Some(Button::A) => {
                    state.debug[DEBUG_I] = state.debug[DEBUG_I].wrapping_sub(1);
                }
                Some(Button::B) => {
                    state.debug[DEBUG_I] = state.debug[DEBUG_I].wrapping_add(1);
                }
                Some(Button::UP) => {
                    let i = state.debug[DEBUG_I] as usize;
                    if let Some(byte_ref) = state.debug.get_mut(i) {
                        *byte_ref = byte_ref.wrapping_add(1);
                    }
                }
                Some(Button::DOWN) => {
                    let i = state.debug[DEBUG_I] as usize;
                    if let Some(byte_ref) = state.debug.get_mut(i) {
                        *byte_ref = byte_ref.wrapping_sub(1);
                    }
                }
                Some(Button::RIGHT) => {
                    let i = state.debug[DEBUG_I] as usize;
                    if let Some(byte_ref) = state.debug.get_mut(i) {
                        *byte_ref = byte_ref.wrapping_add(8);
                    }
                }
                Some(Button::LEFT) => {
                    let i = state.debug[DEBUG_I] as usize;
                    if let Some(byte_ref) = state.debug.get_mut(i) {
                        *byte_ref = byte_ref.wrapping_sub(8);
                    }
                }
                None | _ => {}
            }
        }
    }

    if input.gamepad != <_>::default() {
        speaker.request_sfx(SFX::CardPlace);
    }
}

type GridInner = u16;
type GridX = GridInner;
type GridY = GridInner;

struct LayerDrawIter<'grid> {
    grid: &'grid [Cell],
    x: GridX,
    y: GridY,
}

impl <'grid> LayerDrawIter<'grid> {
    fn of(grid: &'grid [Cell]) -> Self {
        Self {
            grid,
            x: 0,
            y: 0,
        }
    }
}

impl Iterator for LayerDrawIter<'_> {
    type Item = ((i16, i16), Cell);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let mut output;

            // Iterate in such a way that with a naive draw loop body
            // things overlap correctly, with blocks filled in below other
            // ones. That is, bottom, back corner then the three ones adjacent
            // to that, then the next slice and so on.
            // It turns out that to make that pattern, we can create all the
            // coords with a given sum, least to greatest. Maintaining a square
            // can be done by keeping the value of any one coord within the
            // desired range. Probably relies on being based at the origin.
            let x = self.x;
            let y = self.y;

            const MAX: u16 = if GRID_W > GRID_H {
                GRID_W
            } else {
                GRID_H
            } as u16;

            if self.x >= MAX
            && self.y >= MAX {
                // Fallthrough to loop check
                output = None;
            } else {
                loop {
                    if self.x == 0 {
                        self.x = self.y + 1;
                        self.y = 0;
                    } else {
                        self.x -= 1;
                        self.y += 1;
                    }

                    if self.x >= MAX
                    && self.y >= MAX {
                        break
                    }

                    if self.x >= MAX
                    || self.y >= MAX {
                        continue
                    }

                    break
                }
                let index = grid_xy_to_i((x, y));
                output = self.grid.get(index)
                    .cloned()
                    .map(|cell| ((x as i16, y as i16), cell));
            }

            if let Some(o) = output {
                return Some(o)
            } else if more_indexes_are_left(
                (self.x, self.y),
                self.grid.len().saturating_sub(1)
            ) {
                // try again
            } else {
                return None
            }
        }
    }
}

fn grid_xy_to_i((x, y): (GridX, GridY)) -> usize {
    usize::from(y * GRID_W as GridInner + x)
}

fn grid_i_to_xy(i: usize) -> (GridX, GridY) {
    (i as GridX % GRID_W as GridX, i as GridY / GRID_W as GridY)
}

#[test]
fn grid_xy_to_i_to_xy_is_identity_on_these_examples() {
    assert!(GRID_W >= 3);
    assert!(GRID_H >= 2);
    assert_eq!(grid_i_to_xy(grid_xy_to_i((0, 0))), (0, 0));
    assert_eq!(grid_i_to_xy(grid_xy_to_i((1, 0))), (1, 0));
    assert_eq!(grid_i_to_xy(grid_xy_to_i((0, 2))), (0, 2));
    assert_eq!(grid_i_to_xy(dbg!(grid_xy_to_i((2, 2)))), (2, 2));
}

#[test]
fn grid_i_to_xy_to_i_is_identity_on_these_examples() {
    assert_eq!(grid_xy_to_i(grid_i_to_xy(0)), 0);
    assert_eq!(grid_xy_to_i(grid_i_to_xy(2)), 2);
    assert_eq!(grid_xy_to_i(grid_i_to_xy(3 * GRID_W as usize)), 3 * GRID_W as usize);
    assert_eq!(grid_xy_to_i(grid_i_to_xy(4 * GRID_W as usize + 4)), 4 * GRID_W as usize + 4);
}

fn more_indexes_are_left(
    (x, y): (GridX, GridY),
    max_index: usize
) -> bool {
    let sum = x + y;

    let max_x = GridInner::from(GRID_W - 1);
    let max_y = GridInner::from(GRID_H - 1);
    let max_index_sum = max_x + max_y;

    if sum > max_index_sum {
        false
    } else if sum == max_index_sum {
        // For example, if the max xy is (2, 1) then we want (3, 0) to
        // result in true, same with (2, 1), but (1, 2) to result in
        // false.
        // 0x0300 >= 0x0201: true
        // 0x0201 >= 0x0201: true
        // 0x0102 >= 0x0201: false
        u32::from(x) << GridX::BITS
        | u32::from(y)
        >=
        u32::from(max_x) << GridX::BITS
        | u32::from(max_y)
    } else {
        true
    }
}

#[test]
fn more_indexes_are_left_works_on_these_examples() {
    let len = 6;
    assert_eq!(more_indexes_are_left((0, 0), len), true, "(0, 0)");
    assert_eq!(more_indexes_are_left((1, 0), len), true, "(1, 0)");
    assert_eq!(more_indexes_are_left((0, 1), len), true, "(0, 1)");
    assert_eq!(more_indexes_are_left((1, 1), len), true, "(1, 1)");
    assert_eq!(more_indexes_are_left((0, 2), len), true, "(0, 2)");
    assert_eq!(more_indexes_are_left((3, 0), len), true, "(3, 0)");
    assert_eq!(more_indexes_are_left((2, 1), len), true, "(2, 1)");
    assert_eq!(more_indexes_are_left((1, 2), len), false);
}

#[test]
fn layer_draw_iter_works_on_these_examples() {
    macro_rules! c {
        ($i: literal) => {
            Cell {
                hz: <_>::default(),
                cube_i: $i,
            }
        };
        ($z: literal $(,)? $i: literal) => {
            Cell {
                hz: $z,
                cube_i: $i,
            }
        };
    }

    let acutal = LayerDrawIter::of(&[
        c!(1),
        c!(2),
        c!(3),
        c!(4),
        c!(1),
        c!(2),
    ]).collect::<Vec<_>>();

    assert_eq!(
        acutal,
        &[
            ((0, 0), c!(1)),
            ((1, 0), c!(2)),
            ((0, 1), c!(4)),
            ((2, 0), c!(3)),
            ((1, 1), c!(1)),
            ((2, 1), c!(2)), // Wasn't seeing this one
        ]
    )
}

struct DrawIter<'grid> {
    layer_iter: std::iter::Peekable<LayerDrawIter<'grid>>,
    hz: game::HZ,
}

impl <'grid> DrawIter<'grid> {
    fn of(grid: &'grid [Cell]) -> Self {
        Self {
            layer_iter: LayerDrawIter::of(grid).peekable(),
            hz: HZ_BOTTOM,
        }
    }
}

impl Iterator for DrawIter<'_> {
    type Item = ((i16, i16), Cell);

    fn next(&mut self) -> Option<Self::Item> {
        // add more cells to fill below things
        if let Some(&((x, y), cell)) = self.layer_iter.peek() {
            if self.hz == 0 || self.hz == cell.hz {
                self.hz = HZ_BOTTOM;
                self.layer_iter.next()
            } else {
                let hz = self.hz;
                self.hz = self.hz.saturating_sub(2);
                Some(((x, y), Cell { hz, cube_i: 0 }))
            }
        } else {
            None
        }
    }
}

#[inline]
fn render(commands: &mut Commands, state: &game::State) {
    const BASE_X: unscaled::X = unscaled::X(0);
    const BASE_Y: unscaled::Y = unscaled::Y(0);

    let grid_x_start: i16 = state.debug[DEBUG_GRID_X_START] as i8 as _;
    let grid_y_start: i16 = state.debug[DEBUG_GRID_Y_START] as i8 as _;
    let grid_x_end: i16 = state.debug[DEBUG_GRID_X_END] as i8 as _;
    let grid_y_end: i16 = state.debug[DEBUG_GRID_Y_END] as i8 as _;

    let z1: i16 = state.debug[DEBUG_Z1] as i8 as _;
    let z2: i16 = state.debug[DEBUG_Z2] as i8 as _;

    for ((grid_x, grid_y), cell) in DrawIter::of(&state.grid) {
        let iso_x = grid_y - grid_x + state.camera_x;
        let iso_y = grid_y + grid_x + cell.hz as i16 + state.camera_y;

        commands.sspr(
            game::CUBE_XYS[usize::from(cell.cube_i)],
            unscaled::Rect {
                x: BASE_X + unscaled::W(
                    iso_x * CUBE_W.0 / 2
                ),
                y: BASE_Y + unscaled::H(
                    iso_y * CUBE_H.0 / 4
                ),
                w: CUBE_W,
                h: CUBE_H,
            }
        );
    }

    commands.print_line(
        format!("{:?}", state.debug).as_bytes(),
        unscaled::X(0),
        unscaled::Y(0),
        6
    );

    if state.grid.len() <= 16 {
        commands.print_line(
            format!("{:?}", state.grid).as_bytes(),
            unscaled::X(0),
            unscaled::Y(16),
            6
        );
    }
}

#[inline]
fn update_and_render(
    commands: &mut Commands,
    state: &mut game::State,
    input: Input,
    speaker: &mut Speaker,
) {
    update(state, input, speaker);
    render(commands, state);
}
