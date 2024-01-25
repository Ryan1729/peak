use game::{CUBE_H, CUBE_W, GRID_W, HZ, HZ_BOTTOM, Cell, Grid};
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

type GridInner = u8;
type GridX = GridInner;
type GridY = GridInner;

struct LayerDrawIter<'grid> {
    grid: &'grid Grid,
    x: GridX,
    y: GridY,
}

impl <'grid> LayerDrawIter<'grid> {
    fn of(grid: &'grid Grid) -> Self {
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
        // Iterate in such a way that with a naive draw loop body
        // things overlap correctly, with blocks filled in below other
        // ones. That is, bottom, back corner then the three ones adjacent
        // to that, then the next slice and so on.
        // It turns out that to make that pattern, we can create all the
        // coords with a given sum, least to greatest. Maintaining a square
        // can be done by keeping the value of any one coord within the
        // desired range. Probably relies on being based at the origin.

        // Additionally, add more cells to fill below things

        //let i = self.x as usize;
        //self.x += 1;
        //self.hardcoded.get(i).cloned()

        let x = self.x;
        let y = self.y;

        const MAX: u8 = 3;

        if self.x >= MAX
        && self.y >= MAX {
            return None
        }

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

        let index = y * GRID_W as GridInner + x;
        self.grid.get(usize::from(index))
            .cloned()
            .map(|cell| ((x as i16, y as i16), cell))
    }
}

struct DrawIter<'grid> {
    layer_iter: std::iter::Peekable<LayerDrawIter<'grid>>,
    hz: game::HZ,
}

impl <'grid> DrawIter<'grid> {
    fn of(grid: &'grid Grid) -> Self {
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

    // TODO loop over a board and draw in such a way that things overlap
    // correctly, with blocks filled in below other ones. Presumably 
    // something like bottom, back corner then the three ones adjacent
    // to that, then the next slice and so on.
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
