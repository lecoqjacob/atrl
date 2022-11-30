use big_brain::actions::ActionState;

use crate::prelude::*;

#[derive(Debug, Default, Component, Clone)]
pub struct ChaseActor {
    path: Option<Vec<IVec2>>,
    last_seen_pt: Option<(IVec3, UVec2)>,
}

pub fn chase_action(
    tilesets: Tilesets,
    mut commands: Commands,
    mut manager: ResMut<MapManager>,
    mut target_q: Query<&mut TargetVisualizer>,
    player_q: Query<(Entity, &WorldPosition, &LocalPosition), With<Player>>,
    mut action_q: Query<(&Actor, &mut ActionState, &mut ChaseActor, &ActionSpan)>,
    mut ai_q: Query<
        (
            &WorldPosition,
            &LocalPosition,
            &FieldOfView,
            &Vision,
            &Movement,
            &Name,
            &mut AIComponent,
        ),
        Without<Player>,
    >,
) {
    use ActionState::*;

    for (Actor(actor), mut action_state, mut chase, span) in action_q.iter_mut() {
        let _guard = span.span().enter();

        let (_player_entity, player_world_position, player_local_position) = player_q.single();
        let Ok((ai_world_position, ai_local_position, fov, vision, movement_component, name, mut ai_component)) =
            ai_q.get_mut(*actor) else {
                error!("Actor must have required components");
                return
            };

        if ai_component.preferred_action.is_some() {
            // already chasing, quick return;
            commands.insert_resource(TurnState::Processing);
            return;
        }

        let ai_pos = ai_local_position.0;
        let player_pos = player_local_position.0;
        let Some(map) = manager.get_current_map_mut() else {
            error!("No map found");
            return
        };

        // This doesnt exist on the original match below because whenever the player comes into view
        // the `can_see_player` scorer sets the output to be 1, causing the wander action to spin
        // down (1 frame). Then, the player moves and its the ai turn again. the chase
        // actions boots up and evaluates (2 frames). After that spin up, when the player,
        // finally moves a 3rd time, the chase action moves into the Executing
        // state and moves the AI towards the player.
        //
        // So this acts like a skip frame, where it sets the action to evaluating, then immediately
        // evaluates
        if *action_state == Requested {
            chase.last_seen_pt = Some((player_world_position.0, player_local_position.0));
            chase.path = Some(generate_chase_path(
                ai_pos,
                player_pos,
                movement_component.0,
                map,
            ));
            *action_state = Executing;
            info!("{} gonna start chasing!", name);
        }

        match *action_state {
            Cancelled => {
                if let Ok(mut target_visualizer) = target_q.get_mut(*actor) {
                    target_visualizer.clear(&mut commands);
                }
                ai_component.preferred_action = None;
                info!("{} cancelled chase!", name);
                *action_state = Failure;
            },
            Executing => {
                info!("{} executing chase!", name);

                let position = if entity_in_fov(map, fov, vision, ai_pos, player_pos) {
                    let player_pos = (player_world_position.0, player_local_position.0);
                    chase.last_seen_pt = Some(player_pos);
                    player_pos
                } else {
                    let Some(last_seen) = chase.last_seen_pt else {
                        error!("Executing chase with no target.");
                        ai_component.preferred_action = Some(ActionType::Wait);
                        return;
                    };
                    
                    last_seen
                };

                ai_component.preferred_action = Some(ActionType::Movement(position));


/*

                // if update_path_target is_some() update the path
                // otherwise we will assume chase.path is valid
                let update_path_target = if entity_in_fov(map, fov, vision, ai_pos, player_pos) {
                    // we saw the player, update the last seen position
                    chase.last_seen_pt = Some(player_pos);

                    if can_attack(player_pos) {
                        // We should attack instead of moving!
                        *action_state = ActionState::Failure;
                        return;
                    }

                    // always update when we can see the player
                    // so treat it as we don't have a valid path
                    Some(player_pos)
                } else {
                    let Some(last_seen_position) = chase.last_seen_pt else {
                        // How did we get here?
                        // Make sure every transfer into chase is accompanied
                        // by chase.last_seen_pt being set!
                        error!("AI is chasing, but it has no last_seen_position.");
                        *action_state = ActionState::Failure;
                        return;
                    };

                    // Do we have a place we are chasing to?
                    if let Some(path) = &chase.path {
                        if path.is_empty() {
                            // we don't have a valid path because:
                            // we are at the end of the chase, and we don't see the player.
                            //
                            // SWITCH TO WANDER_STATE
                            ai_component.preferred_action = Some(ActionType::Wait);
                            *action_state = ActionState::Failure;

                            return;
                        } else if map.can_place_actor(path[path.len() - 1], movement_component.0) {
                            // we have a valid path, and the next step is also valid!
                            // we only check to make sure this is valid to see if we need to
                            // try re-generating a path. this move will be checked again
                            // as we actually try to move there.
                            None
                        } else {
                            // we have a valid path
                            // but something is blocking us.. Actor/New Feature/Etc
                            // update the path to try to get around this thing...
                            Some(last_seen_position)
                        }
                    } else {
                        Some(last_seen_position)
                    }
                };

                // update the path if necessary!
                if let Some(target_position) = update_path_target {
                    chase.path = Some(generate_chase_path(
                        ai_pos,
                        target_position,
                        movement_component.0,
                        map,
                    ));
                }

                let Some(mut chase_path) = std::mem::take(&mut chase.path) else {
                    // previous update path failed...
                    error!("AI could not find a path for chasing.");
                    ai_component.preferred_action = Some(ActionType::Wait);
                    commands.insert_resource(TurnState::Processing);
                    *action_state = ActionState::Failure;
                    return;
                };

                // We have a path > 1 and we are not in range to attack.
                println!("Chase path: {:?}", chase_path);

                let action = chase_path.pop().map_or_else(
                    || {
                        // previous update path failed...
                        error!("AI could not find a path for chasing.");
                        *action_state = ActionState::Failure;
                        ActionType::Wait
                    },
                    |next_pt| {
                        update_target_visual(
                            &mut commands,
                            &tilesets,
                            &mut target_q,
                            &chase_path,
                            actor,
                            &next_pt,
                            Color::RED,
                        );

                        chase.path = Some(chase_path);
                        ActionType::Movement(next_pt)
                    },
                );

                ai_component.preferred_action = Some(action);
                commands.insert_resource(TurnState::Processing);
                */
            },

            // Init | Success | Failure
            _ => {},
        }

        info!("Chase action output: {:?}\n", action_state);
    }
}
fn generate_chase_path(
    ai_pos: UVec2,
    target_pos: UVec2,
    movement_type: u8,
    map_provider: &impl PathProvider,
) -> Vec<IVec2> {
    PathFinder::Astar.compute(ai_pos.as_ivec2(), target_pos.as_ivec2(), movement_type, true, map_provider).unwrap_or_default()
}

const fn can_attack(_position: IVec2) -> bool { false }
