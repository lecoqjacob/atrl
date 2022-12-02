use crate::prelude::*;
pub fn load_first_map(
    tilesets: Tilesets,
    mut commands: Commands,
    state: Res<CurrentGameState>,
    mut game_context: ResMut<GameContext>,
    mut map_manager: ResMut<MapManager>,
) {
    info!("load_first_map");
    let length = (TILESET_TERRAIN_IDS_TABLE.len() - 1) as u32;
    let tileset_id = Prng::from_entropy().range(0..length) as usize;
    let tileset_id = TILESET_TERRAIN_IDS_TABLE[tileset_id];
    if let Err(e) = map_manager.get_or_generate(
        &mut commands,
        &mut game_context,
        None,
        Some(tileset_id),
        &tilesets,
        IVec3::ZERO,
    ) {
        error!("{}", e);
    }
    state.set_next(&mut commands)
}
