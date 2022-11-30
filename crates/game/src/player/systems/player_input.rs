use std::time::Duration;

use crate::prelude::*;

const PRESSED_DURATION: Duration = Duration::from_millis(500);

pub fn player_input(
    mut action_queue: ResMut<ActionQueue>,
    mut query: Query<&ActionState<PlayerAction>, With<Player>>,
) {
    for action_state in query.iter_mut() {
        // Actions
        if action_state.just_pressed(PlayerAction::Wait) {
            action_queue.add_action(ActionType::Wait);
        }

        // Movement
        for input_direction in PlayerAction::DIRECTIONS {
            if action_state.just_pressed(input_direction) ||
                (action_state.pressed(input_direction) &&
                    action_state.current_duration(input_direction) > PRESSED_DURATION)
            {
                if let Some(direction) = input_direction.direction() {
                    action_queue.add_action(ActionType::MovementDelta(direction.coord()));
                }
            }
        }
    }
}
