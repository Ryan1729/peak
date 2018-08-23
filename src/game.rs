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

fn move_cursor(state: &mut GameState, input: Input, speaker: &mut Speaker) -> bool {
    if input.pressed_this_frame(Button::Right) {
        if state.hand_index < state.hand.len().saturating_sub(1) {
            state.hand_index = state.hand_index.saturating_add(1);
        }
        speaker.request_sfx(SFX::CardSlide);
        true
    } else if input.pressed_this_frame(Button::Left) {
        state.hand_index = state.hand_index.saturating_sub(1);
        speaker.request_sfx(SFX::CardSlide);
        true
    } else {
        false
    }
}

fn is_wild(card: Card) -> bool {
    get_rank(card) == 8
}

fn can_play(state: &GameState, &card: &Card) -> bool {
    if let Some(&top_of_discard) = state.discard.last() {
        is_wild(card)
            || (is_wild(top_of_discard) && state.top_wild_declared_as == Some(get_suit(card)))
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

fn choose_suit(state: &mut GameState) -> Option<Suit> {
    match state.choice {
        Choice::NoChoice => {
            state.choice = Choice::OfSuit;
            None
        }
        Choice::Already(Chosen::Suit(suit)) => {
            state.choice = Choice::NoChoice;
            Some(suit)
        }
        _ => None,
    }
}
fn choose_play_again(state: &mut GameState) -> Option<bool> {
    match state.choice {
        Choice::NoChoice => {
            state.choice = Choice::OfBool;
            None
        }
        Choice::Already(Chosen::Bool(b)) => {
            state.choice = Choice::NoChoice;
            Some(b)
        }
        _ => None,
    }
}

fn advance_card_animations(state: &mut GameState) {
    // I should really be able to use `Vec::retain` here,
    // but that passes a `&T` insteead of a `&mut T`.

    let mut i = state.card_animations.len() - 1;
    loop {
        let (is_complete, last_pos) = {
            let animation = &mut state.card_animations[i];

            let last_pos = (animation.card.x, animation.card.y);

            animation.approach_target();
            console!(log, &format!("{:#?}", animation));

            (animation.is_complete(), last_pos)
        };

        if is_complete {
            let mut animation = state.card_animations.remove(i);

            let card = animation.card.card;

            match animation.completion_action {
                Action::MoveToDiscard => {
                    state.discard.push(card);
                }
                Action::SelectWild(playerId) => {
                    if is_cpu_player(&state, playerId) {
                        state.top_wild_declared_as = {
                            let hand = state.get_hand(playerId);
                            hand.most_common_suit()
                        };
                        state.discard.push(card);
                    } else {
                        if let Some(suit) = choose_suit(state) {
                            state.top_wild_declared_as = Some(suit);
                            state.discard.push(card);
                            state.get_hand_mut(playerId).push(card);
                        } else {
                            //wait until they choose
                            animation.card.x = last_pos.0;
                            animation.card.y = last_pos.1;
                            state.card_animations.push(animation);
                        }
                    }
                }
                Action::MoveToHand(playerId) => {
                    state.get_hand_mut(playerId).push(card);
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
        .map(|card| {
            if is_wild(card.card) {
                CardAnimation::new(card, DISCARD_X, DISCARD_Y, Action::MoveToDiscard)
            } else {
                CardAnimation::new(card, DISCARD_X, DISCARD_Y, Action::MoveToDiscard)
            }
        })
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

fn is_cpu_player(state: &GameState, playerId: PlayerID) -> bool {
    (playerId as usize) < state.cpu_hands.len()
}

fn take_turn(state: &mut GameState, input: Input, speaker: &mut Speaker) {
    let player = state.current_player;
    match player {
        p if is_cpu_player(&state, p) => {
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
            if move_cursor(state, input, speaker) {
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

fn update(state: &mut GameState, input: Input, speaker: &mut Speaker) {
    if state.card_animations.len() == 0 {
        if state.winners.len() == 0 {
            take_turn(state, input, speaker);
        }
    } else {
        advance_card_animations(state);

        move_cursor(state, input, speaker);
    }
}

#[inline]
pub fn do_suit_choice(
    framebuffer: &mut Framebuffer,
    state: &mut GameState,
    input: Input,
    speaker: &mut Speaker,
) {
    framebuffer.full_window();
    //TODO render 4 buttons and set choice based upon them
}
#[inline]
pub fn do_bool_choice(
    framebuffer: &mut Framebuffer,
    state: &mut GameState,
    input: Input,
    speaker: &mut Speaker,
) {
    framebuffer.full_window();
    //TODO render 2 buttons and set choice based upon them
}

#[inline]
pub fn update_and_render(
    framebuffer: &mut Framebuffer,
    state: &mut GameState,
    input: Input,
    speaker: &mut Speaker,
) {
    update(state, input, speaker);

    invariant_assert_eq!(state.missing_cards(), vec![0; 0]);

    framebuffer.clearTo(GREEN);

    for hand in state.cpu_hands.iter() {
        draw_hand(framebuffer, hand, Face::Down);
    }

    framebuffer.draw_card_back(DECK_X, DECK_Y);

    state
        .discard
        .iter()
        .last()
        .map(|&c| framebuffer.draw_card(c, DISCARD_X, DISCARD_Y));

    draw_hand_with_cursor(framebuffer, &state.hand, state.hand_index as usize);

    for &CardAnimation { card, .. } in state.card_animations.iter() {
        framebuffer.draw_card_back(card.x, card.y);
    }

    let len = state.winners.len();
    if len > 0 {
        framebuffer.text_window(
            reflow(
                &state.get_winner_text(),
                window::MAX_INTERIOR_WIDTH_IN_CHARS as usize,
            ).as_bytes(),
        );

        if let Some(again) = choose_play_again(state) {
            if again {
                state.reset();
            }
        }
    }

    match state.choice {
        Choice::OfSuit => do_suit_choice(framebuffer, state, input, speaker),
        Choice::OfBool => do_bool_choice(framebuffer, state, input, speaker),
        _ => {}
    }

    //For testing
    do_bool_choice(framebuffer, state, input, speaker);
}
