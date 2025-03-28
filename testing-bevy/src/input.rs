use bevy::{input::keyboard::KeyboardInput, prelude::*};

use crate::{Ball, Bar, Motor, Position, STARTING_BALL_POS, STARTING_BAR_POS};

// TODO: organize consts into single file
const MOVE_SPEED: f32 = 0.015;
const MAX_DISTANCE: f32 = 3.0;

fn handle_bar_movement(
    q_motors: &mut Query<(&mut Transform, &Position), (With<Motor>, With<Position>)>,
    kb_code: &KeyCode,
) {
    let mut left_motor = None;
    let mut right_motor = None;

    for (transform, position) in q_motors.iter_mut() {
        match position {
            Position::Left => left_motor = Some((transform, position)),
            Position::Right => right_motor = Some((transform, position)),
        }
    }

    let left_translation_y = left_motor.expect("left exists").0.translation.y;
    let mut left_res: f32 = left_translation_y;
    if matches!(kb_code, KeyCode::KeyW) {
        left_res += MOVE_SPEED;
    }
    if matches!(kb_code, KeyCode::KeyS) {
        left_res -= MOVE_SPEED;
    }

    let right_translation_y = right_motor.expect("right exists").0.translation.y;
    let mut right_res: f32 = right_translation_y;
    if matches!(kb_code, KeyCode::ArrowUp) {
        right_res += MOVE_SPEED;
    }
    if matches!(kb_code, KeyCode::ArrowDown) {
        right_res -= MOVE_SPEED;
    }

    for (mut transform, position) in q_motors.iter_mut() {
        match position {
            Position::Left => {
                transform.translation.y =
                    left_res.clamp(right_res - MAX_DISTANCE, right_res + MAX_DISTANCE);
            }
            Position::Right => {
                transform.translation.y =
                    right_res.clamp(left_res - MAX_DISTANCE, left_res + MAX_DISTANCE);
            }
        }
    }
}

fn handle_bar_ball_reset(
    q_bars: &mut Query<&mut Transform, (With<Bar>, Without<Ball>)>,
    q_balls: &mut Query<&mut Transform, (With<Ball>, Without<Bar>)>,
) {
    let Ok(mut bar) = q_bars.get_single_mut() else {
        return;
    };

    let Ok(mut ball) = q_balls.get_single_mut() else {
        return;
    };

    bar.translation = STARTING_BAR_POS.translation;
    ball.translation = STARTING_BALL_POS.translation;
}

pub fn handle_inputs(
    mut char_input_events: EventReader<KeyboardInput>,
    mut q_motors: Query<(&mut Transform, &Position), (With<Motor>, With<Position>)>,
    mut q_bars: Query<&mut Transform, (With<Bar>, Without<Ball>)>,
    mut q_balls: Query<&mut Transform, (With<Ball>, Without<Bar>)>,
) {
    let motors_movement_keys = vec![
        KeyCode::KeyW,
        KeyCode::KeyS,
        KeyCode::ArrowUp,
        KeyCode::ArrowDown,
    ];

    for event in char_input_events.read() {
        if event.state.is_pressed() {
            if motors_movement_keys.contains(&event.key_code) {
                handle_bar_movement(&mut q_motors, &event.key_code);
            }
            if event.key_code == KeyCode::KeyR {
                handle_bar_ball_reset(&mut q_bars, &mut q_balls);
            }
        }
    }
}
