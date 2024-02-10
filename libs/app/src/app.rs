use game::{CUBE_H, CUBE_W, GRID_W, GRID_H, HZ, HZ_BOTTOM, CameraX, CameraY, Cell, Grid, GridX, GridY, grid_xy_to_i, GridInner, GridXInner, GridYInner, MoveMode, X_SCALE, Y_SCALE};
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
        2 => {
            let move_mode = state.move_mode;
            match pressed {
                Some(Button::A) => {
                    state.player.sub_face = state.player.sub_face.wrapping_sub_1();
                }
                Some(Button::B) => {
                    state.player.sub_face = state.player.sub_face.wrapping_add_1();
                }
                Some(Button::UP) if move_mode == MoveMode::B => {
                    state.player.y = state.player.y.saturating_sub(1);
                }
                Some(Button::DOWN) if move_mode == MoveMode::B => {
                    state.player.y = state.player.y.saturating_add(1);
                }
                Some(Button::LEFT) if move_mode == MoveMode::B => {
                    state.player.x = state.player.x.saturating_add(1);
                }
                Some(Button::RIGHT) if move_mode == MoveMode::B => {
                    state.player.x = state.player.x.saturating_sub(1);
                }
                Some(Button::UP) if move_mode == MoveMode::A => {
                    state.player.x = state.player.x.saturating_sub(1);
                }
                Some(Button::DOWN) if move_mode == MoveMode::A => {
                    state.player.x = state.player.x.saturating_add(1);
                }
                Some(Button::LEFT) if move_mode == MoveMode::A => {
                    state.player.y = state.player.y.saturating_sub(1);
                }
                Some(Button::RIGHT) if move_mode == MoveMode::A => {
                    state.player.y = state.player.y.saturating_add(1);
                }
                None | _ => {}
            }
        }
        1 => {
            match pressed {
                Some(Button::UP) => {
                    state.camera_y = state.camera_y.wrapping_sub(Y_SCALE);
                }
                Some(Button::DOWN) => {
                    state.camera_y = state.camera_y.wrapping_add(Y_SCALE);
                }
                Some(Button::LEFT) => {
                    state.camera_x = state.camera_x.wrapping_sub(X_SCALE);
                }
                Some(Button::RIGHT) => {
                    state.camera_x = state.camera_x.wrapping_add(X_SCALE);
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

struct LayerDrawIter<'grid> {
    grid: &'grid [Cell],
    x: GridXInner,
    y: GridYInner,
}

impl <'grid> LayerDrawIter<'grid> {
    fn of(grid: &'grid [Cell]) -> Self {
        Self {
            grid,
            x: <_>::default(),
            y: <_>::default(),
        }
    }
}

impl Iterator for LayerDrawIter<'_> {
    type Item = ((GridX, GridY), Cell);

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

            const MAX: GridInner = if GRID_W > GRID_H {
                GRID_W
            } else {
                GRID_H
            };

            if self.x >= MAX
            && self.y >= MAX {
                // Fallthrough to loop check
                output = None;
            } else {
                loop {
                    if self.x == GridX::MIN.get() {
                        self.x = self.y.saturating_add(1);
                        self.y = GridY::MIN.get();
                    } else {
                        self.x = self.x.saturating_sub(1);
                        self.y = self.y.saturating_add(1);
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

                let xy = (GridX::clamped(x), GridY::clamped(y));

                let index = grid_xy_to_i(xy);
                output = self.grid.get(index)
                    .cloned()
                    .map(|cell| (xy, cell));
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

fn more_indexes_are_left(
    (x, y): (GridXInner, GridYInner),
    max_index: usize
) -> bool {
    let sum = x + y;

    let max_x = GridXInner::from(GRID_W - 1);
    let max_y = GridYInner::from(GRID_H - 1);
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
        u32::from(x) << GridXInner::BITS
        | u32::from(y)
        >=
        u32::from(max_x) << GridXInner::BITS
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
    type Item = ((GridX, GridY), Cell);

    fn next(&mut self) -> Option<Self::Item> {
        // add more cells to fill below things
        if let Some(&((x, y), cell)) = self.layer_iter.peek() {
            let hz = self.hz;

            // Move up 2 so we don't get overlapping cubes
            self.hz = self.hz.saturating_sub(2);

            if hz <= cell.hz {
                self.hz = HZ_BOTTOM;
                self.layer_iter.next()
            } else {
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

    fn to_iso(
        (grid_x, grid_y): (GridX, GridY),
        cell: Cell,
    ) -> (CameraX, CameraY) {
        (
            grid_y.get() as CameraX - grid_x.get() as CameraX,
            grid_y.get() as CameraY + grid_x.get() as CameraY + cell.hz as CameraY,
        )
    }
// Have the camera follow the player directly, so the player is always in 
// the center of the screen
    for ((grid_x, grid_y), cell) in DrawIter::of(&state.grid) {
        let (iso_x, iso_y) = to_iso(
            (grid_x, grid_y),
            cell,
        );

        commands.sspr(
            game::CUBE_XYS[usize::from(cell.cube_i)],
            unscaled::Rect {
                x: BASE_X + unscaled::W(
                    iso_x * X_SCALE
                ) + unscaled::W(
                    state.camera_x
                ),
                y: BASE_Y + unscaled::H(
                    iso_y * Y_SCALE
                ) + unscaled::H(
                    state.camera_y
                ),
                w: CUBE_W,
                h: CUBE_H,
            }
        );
    }

    {
        let (iso_x, iso_y) = to_iso(
            (state.player.x, state.player.y),
            state.player_cell(),
        );
    
        commands.sspr(
            state.player.sub_face.sprite_xy(),
            unscaled::Rect {
                x: BASE_X + unscaled::W(
                    iso_x * X_SCALE
                ) + unscaled::W(
                    state.camera_x
                ),
                y: BASE_Y + unscaled::H(
                    iso_y * Y_SCALE
                ) + unscaled::H(
                    state.camera_y
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

    let mut y = unscaled::Y(0);

    const FITS_ON_SCREEN: usize = 6;
    for grid_slice in state.grid.chunks(FITS_ON_SCREEN) {
        y += unscaled::H(16);

        commands.print_line(
            format!("{grid_slice:?}").as_bytes(),
            unscaled::X(0),
            y,
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
