use platform_types::{Button, Command, SFX, State, StateParams, Rect, Kind::*};

struct StateWrapper {
    state: game::BartogState,
    frame_count: u64,
    stashed: Vec<Command>,
}

impl State for StateWrapper {
    fn frame(&mut self) -> (&[Command], &[SFX]) {
        self.frame_count += 1;
        if self.stashed.len() > 0 {
            (&self.stashed, &[])
        } else {
            do_frame(&mut self.state, self.frame_count)
        }
    }

    fn press(&mut self, button: Button) {
        self.state.press(button)
    }

    fn release(&mut self, button: Button) {
        self.state.release(button)
    }
}

impl StateWrapper {
    fn new(params: StateParams) -> Self {
        Self {
            state: game::BartogState::new(params),
            frame_count: 0,
            stashed: Vec::new(),
        }
    }
}

fn do_frame(state: &mut game::BartogState, frame_count: u64) -> (&[Command], &[SFX]) {
    if frame_count == 2 {
        state.press(Button::A);
        state.frame()
    } else if frame_count == 3 {
        state.release(Button::A);
        state.frame()
    } else if frame_count >= 10 {
        (&COMMANDS_10, &[])
    } else {
        state.frame()
    }
}


fn main() {
    let seed = [10, 56, 42, 75, 1, 190, 216, 65, 6, 119, 65, 160, 129, 177, 4, 62];
    let state = StateWrapper::new((
        seed,
        None,
        None,
    ));

    platform::run(state);
}

const COMMANDS_10: [Command; 50] = [
            Command {
                rect: Rect {
                    x: 0,
                    y: 0,
                    w: 128,
                    h: 128,
                },
                kind: Colour(
                    1,
                ),
            },
            Command {
                rect: Rect {
                    x: 2,
                    y: 32,
                    w: 20,
                    h: 30,
                },
                kind: Gfx(
                    (
                        26,
                        1,
                    ),
                ),
            },
            Command {
                rect: Rect {
                    x: 2,
                    y: 39,
                    w: 20,
                    h: 30,
                },
                kind: Gfx(
                    (
                        26,
                        1,
                    ),
                ),
            },
            Command {
                rect: Rect {
                    x: 2,
                    y: 46,
                    w: 20,
                    h: 30,
                },
                kind: Gfx(
                    (
                        26,
                        1,
                    ),
                ),
            },
            Command {
                rect: Rect {
                    x: 2,
                    y: 53,
                    w: 20,
                    h: 30,
                },
                kind: Gfx(
                    (
                        26,
                        1,
                    ),
                ),
            },
            Command {
                rect: Rect {
                    x: 2,
                    y: 60,
                    w: 20,
                    h: 30,
                },
                kind: Gfx(
                    (
                        26,
                        1,
                    ),
                ),
            },
            Command {
                rect: Rect {
                    x: 2,
                    y: 1,
                    w: 20,
                    h: 30,
                },
                kind: Gfx(
                    (
                        26,
                        1,
                    ),
                ),
            },
            Command {
                rect: Rect {
                    x: 22,
                    y: 1,
                    w: 20,
                    h: 30,
                },
                kind: Gfx(
                    (
                        26,
                        1,
                    ),
                ),
            },
            Command {
                rect: Rect {
                    x: 42,
                    y: 1,
                    w: 20,
                    h: 30,
                },
                kind: Gfx(
                    (
                        26,
                        1,
                    ),
                ),
            },
            Command {
                rect: Rect {
                    x: 62,
                    y: 1,
                    w: 20,
                    h: 30,
                },
                kind: Gfx(
                    (
                        26,
                        1,
                    ),
                ),
            },
            Command {
                rect: Rect {
                    x: 82,
                    y: 1,
                    w: 20,
                    h: 30,
                },
                kind: Gfx(
                    (
                        26,
                        1,
                    ),
                ),
            },
            Command {
                rect: Rect {
                    x: 106,
                    y: 32,
                    w: 20,
                    h: 30,
                },
                kind: Gfx(
                    (
                        26,
                        1,
                    ),
                ),
            },
            Command {
                rect: Rect {
                    x: 106,
                    y: 40,
                    w: 20,
                    h: 30,
                },
                kind: Gfx(
                    (
                        26,
                        1,
                    ),
                ),
            },
            Command {
                rect: Rect {
                    x: 106,
                    y: 48,
                    w: 20,
                    h: 30,
                },
                kind: Gfx(
                    (
                        26,
                        1,
                    ),
                ),
            },
            Command {
                rect: Rect {
                    x: 106,
                    y: 56,
                    w: 20,
                    h: 30,
                },
                kind: Gfx(
                    (
                        26,
                        1,
                    ),
                ),
            },
            Command {
                rect: Rect {
                    x: 40,
                    y: 32,
                    w: 20,
                    h: 30,
                },
                kind: Gfx(
                    (
                        26,
                        1,
                    ),
                ),
            },
            Command {
                rect: Rect {
                    x: 46,
                    y: 66,
                    w: 8,
                    h: 8,
                },
                kind: Font(
                    (
                        24,
                        24,
                    ),
                    7,
                ),
            },
            Command {
                rect: Rect {
                    x: 50,
                    y: 66,
                    w: 8,
                    h: 8,
                },
                kind: Font(
                    (
                        16,
                        24,
                    ),
                    7,
                ),
            },
            Command {
                rect: Rect {
                    x: 76,
                    y: 66,
                    w: 8,
                    h: 8,
                },
                kind: Font(
                    (
                        0,
                        24,
                    ),
                    7,
                ),
            },
            Command {
                rect: Rect {
                    x: 22,
                    y: 112,
                    w: 20,
                    h: 30,
                },
                kind: Gfx(
                    (
                        2,
                        1,
                    ),
                ),
            },
            Command {
                rect: Rect {
                    x: 25,
                    y: 115,
                    w: 8,
                    h: 8,
                },
                kind: Font(
                    (
                        88,
                        8,
                    ),
                    7,
                ),
            },
            Command {
                rect: Rect {
                    x: 23,
                    y: 122,
                    w: 8,
                    h: 8,
                },
                kind: Font(
                    (
                        96,
                        8,
                    ),
                    7,
                ),
            },
            Command {
                rect: Rect {
                    x: 31,
                    y: 131,
                    w: 8,
                    h: 8,
                },
                kind: Font(
                    (
                        88,
                        72,
                    ),
                    7,
                ),
            },
            Command {
                rect: Rect {
                    x: 33,
                    y: 124,
                    w: 8,
                    h: 8,
                },
                kind: Font(
                    (
                        96,
                        72,
                    ),
                    7,
                ),
            },
            Command {
                rect: Rect {
                    x: 42,
                    y: 112,
                    w: 20,
                    h: 30,
                },
                kind: Gfx(
                    (
                        2,
                        1,
                    ),
                ),
            },
            Command {
                rect: Rect {
                    x: 45,
                    y: 115,
                    w: 8,
                    h: 8,
                },
                kind: Font(
                    (
                        8,
                        56,
                    ),
                    7,
                ),
            },
            Command {
                rect: Rect {
                    x: 43,
                    y: 122,
                    w: 8,
                    h: 8,
                },
                kind: Font(
                    (
                        120,
                        8,
                    ),
                    7,
                ),
            },
            Command {
                rect: Rect {
                    x: 51,
                    y: 131,
                    w: 8,
                    h: 8,
                },
                kind: Font(
                    (
                        8,
                        120,
                    ),
                    7,
                ),
            },
            Command {
                rect: Rect {
                    x: 53,
                    y: 124,
                    w: 8,
                    h: 8,
                },
                kind: Font(
                    (
                        120,
                        72,
                    ),
                    7,
                ),
            },
            Command {
                rect: Rect {
                    x: 62,
                    y: 112,
                    w: 20,
                    h: 30,
                },
                kind: Gfx(
                    (
                        2,
                        1,
                    ),
                ),
            },
            Command {
                rect: Rect {
                    x: 65,
                    y: 115,
                    w: 8,
                    h: 8,
                },
                kind: Font(
                    (
                        8,
                        56,
                    ),
                    2,
                ),
            },
            Command {
                rect: Rect {
                    x: 63,
                    y: 122,
                    w: 8,
                    h: 8,
                },
                kind: Font(
                    (
                        112,
                        8,
                    ),
                    2,
                ),
            },
            Command {
                rect: Rect {
                    x: 71,
                    y: 131,
                    w: 8,
                    h: 8,
                },
                kind: Font(
                    (
                        8,
                        120,
                    ),
                    2,
                ),
            },
            Command {
                rect: Rect {
                    x: 73,
                    y: 124,
                    w: 8,
                    h: 8,
                },
                kind: Font(
                    (
                        112,
                        72,
                    ),
                    2,
                ),
            },
            Command {
                rect: Rect {
                    x: 82,
                    y: 112,
                    w: 20,
                    h: 30,
                },
                kind: Gfx(
                    (
                        2,
                        1,
                    ),
                ),
            },
            Command {
                rect: Rect {
                    x: 85,
                    y: 115,
                    w: 8,
                    h: 8,
                },
                kind: Font(
                    (
                        72,
                        24,
                    ),
                    7,
                ),
            },
            Command {
                rect: Rect {
                    x: 83,
                    y: 122,
                    w: 8,
                    h: 8,
                },
                kind: Font(
                    (
                        96,
                        8,
                    ),
                    7,
                ),
            },
            Command {
                rect: Rect {
                    x: 91,
                    y: 131,
                    w: 8,
                    h: 8,
                },
                kind: Font(
                    (
                        72,
                        88,
                    ),
                    7,
                ),
            },
            Command {
                rect: Rect {
                    x: 93,
                    y: 124,
                    w: 8,
                    h: 8,
                },
                kind: Font(
                    (
                        96,
                        72,
                    ),
                    7,
                ),
            },
            Command {
                rect: Rect {
                    x: 2,
                    y: 112,
                    w: 20,
                    h: 30,
                },
                kind: Gfx(
                    (
                        2,
                        1,
                    ),
                ),
            },
            Command {
                rect: Rect {
                    x: 5,
                    y: 115,
                    w: 8,
                    h: 8,
                },
                kind: Font(
                    (
                        56,
                        24,
                    ),
                    2,
                ),
            },
            Command {
                rect: Rect {
                    x: 3,
                    y: 122,
                    w: 8,
                    h: 8,
                },
                kind: Font(
                    (
                        112,
                        8,
                    ),
                    2,
                ),
            },
            Command {
                rect: Rect {
                    x: 11,
                    y: 131,
                    w: 8,
                    h: 8,
                },
                kind: Font(
                    (
                        56,
                        88,
                    ),
                    2,
                ),
            },
            Command {
                rect: Rect {
                    x: 13,
                    y: 124,
                    w: 8,
                    h: 8,
                },
                kind: Font(
                    (
                        112,
                        72,
                    ),
                    2,
                ),
            },
            Command {
                rect: Rect {
                    x: 1,
                    y: 111,
                    w: 24,
                    h: 32,
                },
                kind: Gfx(
                    (
                        49,
                        0,
                    ),
                ),
            },
            Command {
                rect: Rect {
                    x: 94,
                    y: 54,
                    w: 20,
                    h: 30,
                },
                kind: Gfx(
                    (
                        2,
                        1,
                    ),
                ),
            },
            Command {
                rect: Rect {
                    x: 97,
                    y: 57,
                    w: 8,
                    h: 8,
                },
                kind: Font(
                    (
                        8,
                        56,
                    ),
                    7,
                ),
            },
            Command {
                rect: Rect {
                    x: 95,
                    y: 64,
                    w: 8,
                    h: 8,
                },
                kind: Font(
                    (
                        96,
                        8,
                    ),
                    7,
                ),
            },
            Command {
                rect: Rect {
                    x: 103,
                    y: 73,
                    w: 8,
                    h: 8,
                },
                kind: Font(
                    (
                        8,
                        120,
                    ),
                    7,
                ),
            },
            Command {
                rect: Rect {
                    x: 105,
                    y: 66,
                    w: 8,
                    h: 8,
                },
                kind: Font(
                    (
                        96,
                        72,
                    ),
                    7,
                ),
            },
        ];