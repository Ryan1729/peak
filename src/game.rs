use common::*;

use rand::Rng;

enum Face {
    Up,
    Down,
}

fn draw_hand_ltr(
    framebuffer: &mut Framebuffer,
    hand: &Hand,
    offset: u8,
    (mut x, y): (u8, u8),
    face: Face,
) {
    match face {
        Face::Up => {
            for &card in hand.iter() {
                framebuffer.draw_card(card, x, y);

                x += offset;
            }
        }
        Face::Down => {
            for &_card in hand.iter() {
                framebuffer.draw_card_back(x, y);

                x += offset;
            }
        }
    }
}

fn draw_hand_ttb(
    framebuffer: &mut Framebuffer,
    hand: &Hand,
    offset: u8,
    (x, mut y): (u8, u8),
    face: Face,
) {
    match face {
        Face::Up => {
            for &card in hand.iter() {
                framebuffer.draw_card(card, x, y);

                y += offset;
            }
        }
        Face::Down => {
            for &_card in hand.iter() {
                framebuffer.draw_card_back(x, y);

                y += offset;
            }
        }
    }
}

fn draw_hand(framebuffer: &mut Framebuffer, hand: &Hand, face: Face) {
    let offset = get_card_offset(hand.spread, hand.len());

    match hand.spread {
        Spread::LTR((x, _), y) => {
            draw_hand_ltr(framebuffer, hand, offset, (x, y), face);
        }
        Spread::TTB((y, _), x) => {
            draw_hand_ttb(framebuffer, hand, offset, (x, y), face);
        }
    }
}

fn draw_hand_with_cursor_ltr(
    framebuffer: &mut Framebuffer,
    hand: &Hand,
    offset: u8,
    (mut x, y): (u8, u8),
    index: usize,
) {
    let mut selected_card_and_offset = None;
    for (i, &card) in hand.iter().enumerate() {
        if i == index {
            selected_card_and_offset = Some((card, x));
            x += offset;

            continue;
        }
        framebuffer.draw_card(card, x, y);

        x += offset;
    }

    if let Some((card, cursor_offset)) = selected_card_and_offset {
        framebuffer.draw_highlighted_card(card, cursor_offset, y);
    }
}

fn draw_hand_with_cursor_ttb(
    framebuffer: &mut Framebuffer,
    hand: &Hand,
    offset: u8,
    (x, mut y): (u8, u8),
    index: usize,
) {
    let mut selected_card_and_offset = None;
    for (i, &card) in hand.iter().enumerate() {
        if i == index {
            selected_card_and_offset = Some((card, y));
            y += offset;

            continue;
        }
        framebuffer.draw_card(card, x, y);

        y += offset;
    }

    if let Some((card, cursor_offset)) = selected_card_and_offset {
        framebuffer.draw_highlighted_card(card, x, cursor_offset);
    }
}

fn draw_hand_with_cursor(framebuffer: &mut Framebuffer, hand: &Hand, index: usize) {
    let offset = get_card_offset(hand.spread, hand.len());

    match hand.spread {
        Spread::LTR((x, _), y) => {
            draw_hand_with_cursor_ltr(framebuffer, hand, offset, (x, y), index);
        }
        Spread::TTB((y, _), x) => {
            draw_hand_with_cursor_ttb(framebuffer, hand, offset, (x, y), index);
        }
    }
}

fn move_cursor(state: &mut GameState, input: Input) -> bool {
    if input.pressed_this_frame(Button::Right) {
        if state.hand_index < state.hand.len().saturating_sub(1) {
            state.hand_index = state.hand_index.saturating_add(1);
        }
        true
    } else if input.pressed_this_frame(Button::Left) {
        state.hand_index = state.hand_index.saturating_sub(1);
        true
    } else {
        false
    }
}

fn can_play(state: &GameState, &card: &Card) -> bool {
    if let Some(&top_of_discard) = state.discard.last() {
        get_rank(card) == 8 //TODO player who played an 8 choosing a suit for it
            || get_suit(top_of_discard) == get_suit(card)
            || get_rank(top_of_discard) == get_rank(card)
    } else {
        true
    }
}

//Since this uses rng, callling this in response to repeatable user input allows rng manipulation.
fn cpu_would_play(state: &mut GameState, playerId: PlayerID) -> Option<u8> {
    let playable: Vec<(usize, Card)> = {
        let hand = state.get_hand(playerId);
        hand.iter()
            .cloned()
            .enumerate()
            .filter(|(_, card)| can_play(state, card))
            .collect()
    };

    state.rng.choose(&playable).map(|&(i, _)| i as u8)
}

fn advance_card_animations(state: &mut GameState) {
    // I should really be able to use `Vec::retain` here,
    // but that passes a `&T` insteead of a `&mut T`.

    let mut i = state.card_animations.len() - 1;
    loop {
        let is_complete = {
            let animation = &mut state.card_animations[i];

            animation.approach_target();
            console!(log, &format!("{:#?}", animation));

            animation.is_complete()
        };

        if is_complete {
            let animation = state.card_animations.remove(i);

            let card = animation.card.card;

            match animation.completion_action {
                Action::MoveToDiscard => {
                    state.discard.push(card);
                }
                Action::MoveToHand(player) => {
                    state.get_hand_mut(player).push(card);
                }
            }
        }

        if i == 0 {
            break;
        }
        i -= 1;
    }
}

fn get_discard_animation(
    state: &mut GameState,
    player: PlayerID,
    card_index: u8,
) -> Option<CardAnimation> {
    state
        .remove_positioned_card(player, card_index)
        .map(|card| CardAnimation::new(card, DISCARD_X, DISCARD_Y, Action::MoveToDiscard))
}

fn get_draw_animation(state: &mut GameState, player: PlayerID) -> Option<CardAnimation> {
    let (spread, len) = {
        let hand = state.get_hand(player);

        (hand.spread, hand.len())
    };
    let card = state.deck.draw()?;

    let (x, y) = get_card_position(spread, len + 1, len);

    Some(CardAnimation::new(
        PositionedCard {
            card,
            x: DECK_X,
            y: DECK_Y,
        },
        x,
        y,
        Action::MoveToHand(player),
    ))
}

#[inline]
fn push_if<T>(vec: &mut Vec<T>, op: Option<T>) {
    if let Some(t) = op {
        vec.push(t);
    }
}

fn take_turn(state: &mut GameState, input: Input) {
    let player = state.current_player;
    match player {
        p if (p as usize) < state.cpu_hands.len() => {
            if let Some(index) = cpu_would_play(state, p) {
                let animation = get_discard_animation(state, player, index);
                push_if(&mut state.card_animations, animation);
            } else {
                let animation = get_draw_animation(state, player);
                push_if(&mut state.card_animations, animation);
            }

            state.current_player += 1;
        }
        _ => {
            if move_cursor(state, input) {
                //Already handled.
            } else if input.pressed_this_frame(Button::A) {
                let index = state.hand_index;

                let can_play_it = {
                    state
                        .hand
                        .get(index)
                        .map(|card| can_play(&state, card))
                        .unwrap_or(false)
                };

                if can_play_it {
                    let animation = get_discard_animation(state, player, index);
                    console!(log, &format!("{:?}", animation));

                    push_if(&mut state.card_animations, animation);
                    console!(log, &format!("{:?}", state.card_animations));

                    state.current_player = 0;
                } else {
                    //TODO good feedback. Tint the card red or shake it or something?
                }
            } else if input.pressed_this_frame(Button::B) {
                let animation = get_draw_animation(state, player);
                push_if(&mut state.card_animations, animation);

                state.current_player = 0;
            }
        }
    }

    let player_ids: Vec<PlayerID> = state.player_ids();

    let winners: Vec<PlayerID> = player_ids
        .iter()
        .filter(|&&player| state.get_hand(player).len() == 0)
        .cloned()
        .collect();

    if winners.len() > 0 {
        state.winners = winners;
    }
}

fn update(state: &mut GameState, input: Input) {
    if state.card_animations.len() == 0 {
        if state.winners.len() == 0 {
            take_turn(state, input);
        }
    } else {
        advance_card_animations(state);

        move_cursor(state, input);
    }
}

#[inline]
pub fn update_and_render(framebuffer: &mut Framebuffer, state: &mut GameState, input: Input) {
    update(state, input);

    invariant_assert_eq!(state.missing_cards(), vec![0; 0]);

    framebuffer.clearTo(GREEN);

    for hand in state.cpu_hands.iter() {
        draw_hand(framebuffer, hand, Face::Down);
    }

    state
        .deck
        .iter()
        .last()
        .map(|&c| framebuffer.draw_card(c, DECK_X, DECK_Y));
    state
        .discard
        .iter()
        .last()
        .map(|&c| framebuffer.draw_card(c, DISCARD_X, DISCARD_Y));

    draw_hand_with_cursor(framebuffer, &state.hand, state.hand_index as usize);

    for &CardAnimation { card, .. } in state.card_animations.iter() {
        framebuffer.draw_pos_card(card);
    }

    let reflowed = reflow(
        "CPU 0, CPU 1, CPU 2, and you all win.",
        window::MAX_INTERIOR_WIDTH_IN_CHARS as usize,
    );

    framebuffer.text_window(reflowed.as_bytes());

    let len = state.winners.len();
    if len > 0 {
        framebuffer.text_window(
            reflow(
                &state.get_winner_text(),
                window::MAX_INTERIOR_WIDTH_IN_CHARS as usize,
            ).as_bytes(),
        );
    }
}
