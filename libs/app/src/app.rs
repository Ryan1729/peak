use game::Splat;
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

const DEBUG_I: usize = 15;
const DEBUG_GRID_X_START: usize = 0;
const DEBUG_GRID_Y_START: usize = 1;
const DEBUG_GRID_X_END: usize = 2;
const DEBUG_GRID_Y_END: usize = 3;

fn update(state: &mut game::State, input: Input, speaker: &mut Speaker) {
    match input.button_pressed_this_frame() {
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

    if input.gamepad != <_>::default() {
        speaker.request_sfx(SFX::CardPlace);
    }
}

#[inline]
fn render(commands: &mut Commands, state: &game::State) {
    const CUBE_W: unscaled::W = unscaled::W(111);
    const CUBE_H: unscaled::H = unscaled::H(128);

    const CUBE_XYS: [sprite::XY; 2] = [
        sprite::XY {
            x: sprite::X(128),
            y: sprite::Y(0),
        },
        sprite::XY {
            x: sprite::X(128),
            y: sprite::Y(128),
        },
    ];

    const BASE_X: unscaled::X = unscaled::X(0);
    const BASE_Y: unscaled::Y = unscaled::Y(0);

    let grid_x_start: i16 = state.debug[DEBUG_GRID_X_START] as i8 as _;
    let grid_y_start: i16 = state.debug[DEBUG_GRID_Y_START] as i8 as _;
    let grid_x_end: i16 = state.debug[DEBUG_GRID_X_END] as i8 as _;
    let grid_y_end: i16 = state.debug[DEBUG_GRID_Y_END] as i8 as _;

    for grid_x in grid_x_start..grid_x_end {
        for grid_y in grid_y_start..grid_y_end {
            let cube_i = usize::try_from(
                ((grid_x ^ grid_y) & 1) as u16
            ).unwrap();

            let iso_x = grid_y - grid_x;
            let iso_y = grid_y + grid_x;

            commands.sspr(
                CUBE_XYS[cube_i],
                command::Rect::from_unscaled(unscaled::Rect {
                    x: BASE_X + unscaled::W(
                        iso_x * CUBE_W.0 / 2
                    ),
                    y: BASE_Y + unscaled::H(
                        iso_y * CUBE_H.0 / 4
                    ),
                    w: CUBE_W,
                    h: CUBE_H,
                })
            );
        }
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
