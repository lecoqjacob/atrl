use crate::game::prelude::internal::*;
use crate::prelude::*;

// This is the list of "things in the game I want to be able to do based on input"
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum PlayerAction {
    // Movement
    Up,
    Down,
    Left,
    Right,
}

impl PlayerAction {
    // Lists like this can be very useful for quickly matching subsets of actions
    pub const DIRECTIONS: [Self; 4] = [Self::Up, Self::Down, Self::Left, Self::Right];

    pub const fn direction(self) -> Option<GridDirection> {
        match self {
            Self::Up => Some(GridDirection::North),
            Self::Down => Some(GridDirection::South),
            Self::Left => Some(GridDirection::West),
            Self::Right => Some(GridDirection::East),
        }
    }
}

pub struct PlayerPlugin<T> {
    pub state_running: T,
}

impl<T: StateNext> Plugin for PlayerPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<PlayerAction>::default()).add_system_set(
            ConditionSet::new()
                .run_in_state(self.state_running.clone())
                .with_system(player_input)
                .into(),
        );

        // TODO: Remove this once states are working for player / AI
        app.add_system(
            insert_resource!(TurnState::AwaitingInput).run_if_resource_equals(TurnState::Ticking),
        );
    }
}
