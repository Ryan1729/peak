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

fn update(state: &mut game::State, input: Input, speaker: &mut Speaker) {
    if input.gamepad != <_>::default() {
        state.add_splat();
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

    const GRID_X_START: i16 = 2;
    const GRID_Y_START: i16 = 14;

    for grid_x in GRID_X_START..(GRID_X_START + 6) {
        for grid_y in GRID_Y_START..(GRID_Y_START + 8) {
            let cube_i = usize::try_from(
                ((grid_x ^ grid_y) & 1) as u16
            ).unwrap();

            let iso_x = (grid_y - grid_x).saturating_sub(6);
            assert!(iso_x >= 0, "grid_x: {grid_x} grid_y: {grid_y}");
            let iso_y = (grid_y + grid_x).saturating_sub(GRID_Y_START + 2);
            assert!(iso_y >= 0, "grid_x: {grid_x} grid_y: {grid_y}");

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

    for &Splat { kind, x, y } in &state.splats {
        commands.draw_card(kind, x, y);

        commands.sspr(
            sprite::XY {
                x: sprite::X(0),
                y: sprite::Y(64),
            },
            command::Rect::from_unscaled(unscaled::Rect {
                x: x.saturating_sub(unscaled::W(16)),
                y: y.saturating_sub(unscaled::H(16)),
                w: unscaled::W(16),
                h: unscaled::H(16),
            })
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
