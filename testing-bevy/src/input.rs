use bevy::prelude::*;

use crate::{Ball, Bar, INIT_BALL_POS, INIT_BAR_POS, Motor, Position};

// TODO: organize consts into single file?
const MOVE_SPEED: f32 = 0.1;
const MAX_DISTANCE: f32 = 3.0;

pub fn handle_inputs(
    keys: Res<ButtonInput<KeyCode>>,
    mut q_motors: Query<
        (&mut Transform, &Position),
        (With<Motor>, With<Position>, Without<Bar>, Without<Ball>),
    >,
    mut q_bars: Query<&mut Transform, (With<Bar>, Without<Ball>)>,
    mut q_balls: Query<&mut Transform, (With<Ball>, Without<Bar>)>,
) {
    let motors_movement_keys = vec![
        KeyCode::KeyW,
        KeyCode::KeyS,
        KeyCode::ArrowUp,
        KeyCode::ArrowDown,
    ];

    if keys.any_pressed(motors_movement_keys) {
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

        let right_translation_y = right_motor.expect("right exists").0.translation.y;
        let mut right_res: f32 = right_translation_y;

        for kb_code in keys.get_pressed() {
            if matches!(kb_code, KeyCode::KeyW) {
                println!("left move UP {:?}", kb_code);
                left_res += MOVE_SPEED;
            }
            if matches!(kb_code, KeyCode::KeyS) {
                println!("left move DOWN {:?}", kb_code);
                left_res -= MOVE_SPEED;
            }

            if matches!(kb_code, KeyCode::ArrowUp) {
                println!("right move UP {:?}", kb_code);
                right_res += MOVE_SPEED;
            }
            if matches!(kb_code, KeyCode::ArrowDown) {
                println!("right move DOWN {:?}", kb_code);
                right_res -= MOVE_SPEED;
            }
            println!("{} {}", left_res, right_res);
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

    if keys.pressed(KeyCode::KeyR) {
        println!("should reset!");
        let Ok(mut bar) = q_bars.get_single_mut() else {
            return;
        };

        let Ok(mut ball) = q_balls.get_single_mut() else {
            return;
        };

        bar.translation = INIT_BAR_POS.translation;
        ball.translation = INIT_BALL_POS.translation;
    }
}
