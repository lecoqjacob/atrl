use crate::prelude::*;

// This needs to impl FromWorld not derive reflect
#[derive(Component, Clone)]
pub struct Map {
    pub size: UVec2,
    pub random: Random,
    pub world_position: WorldPosition,

    pub update_all: bool,
    pub update_tiles: HashSet<UVec2>,

    pub actors: Grid<Option<Entity>>,

    pub terrain_tileset_id: u8,
    pub feature_tileset_id: u8,
    pub item_tileset_id: u8,

    pub terrain_layer_entity: Entity,
    pub feature_layer_entity: Entity,
    pub item_layer_entity: Entity,

    pub terrain_types: Grid<TerrainType>,
    pub feature_types: Grid<FeatureType>,
    pub item_types: Grid<Vec<ItemType>>,

    pub pathmap: BitGrid,
    pub explored_tiles: HashSet<UVec2>,
}

impl Map {
    pub fn can_move_through(&self, index: impl Point2d, movement_component: &Movement) -> bool {
        let terrain = self
            .terrain_types
            .get(index)
            .map_or(MovementType::None.as_u8(), |t| t.allowed_movement());
        let feature = self
            .feature_types
            .get(index)
            .map_or(MovementType::Any.as_u8(), |f| f.allowed_movement());

        (terrain & feature & **movement_component) != 0
    }

    pub fn can_see_through(&self, index: impl Point2d, vision_type: VisionType) -> bool {
        let vision_type = vision_type.as_u8();

        // Check if the player is blind
        if (vision_type & VisionType::Blind as u8) != 0 {
            return false;
        }

        // Get the vision types that can see through this terrain:
        // None by default (if there's no terrain, there's nothing to see)
        let terrain = self
            .terrain_types
            .get(index)
            .map_or(VisionType::None.as_u8(), |t| t.vision_penetrates());

        // Get the vision types that can see through this feature:
        // Any by default (if there's no feature, there's nothing blocking)
        let feature = self
            .feature_types
            .get(index)
            .map_or(VisionType::Any.as_u8(), |f| f.vision_penetrates());

        (terrain & feature & (vision_type)) != 0
    }

    pub fn can_see_feature(&self, index: impl Point2d, vision_type: VisionType) -> bool {
        let feature =
            self.feature_types.get(index).map_or(VisionType::None.as_u8(), |f| f.allowed_vision());

        (feature & vision_type.as_u8()) != 0
    }

    pub fn set_terrain_at(&mut self, index: impl Point2d, terrain_type: TerrainType) {
        self.terrain_types.set(index, terrain_type);
        self.update_tiles.insert(index.as_uvec2());
    }

    pub fn set_feature_at(&mut self, index: impl Point2d, feature_type: FeatureType) {
        self.feature_types.set(index, feature_type);
        self.update_tiles.insert(index.as_uvec2());
    }

    pub fn has_actor(&mut self, index: impl Point2d) -> bool {
        self.actors.get(index).is_some()
    }

    pub fn add_actor(&mut self, index: impl Point2d, actor: Entity) {
        self.actors.set(index, Some(actor));
    }

    pub fn remove_actor(&mut self, index: impl Point2d) {
        self.actors.set(index, None);
    }

    pub fn get_actor(&self, index: impl Point2d) -> Option<Entity> {
        self.actors.get(index).and_then(|e| e.as_ref().copied())
    }

    pub fn get_actor_position(&self, actor: Entity) -> Option<IVec2> {
        self.actors.enumerate().find_map(
            |(pt, e)| {
                if e.as_ref() == Some(&actor) {
                    Some(pt)
                } else {
                    None
                }
            },
        )
    }

    pub fn move_actor(&mut self, from: impl Point2d, to: impl Point2d) {
        if self.has_actor(to) {
            return;
        }

        if let Some(actor) = self.get_actor(from) {
            self.remove_actor(from);
            self.add_actor(to, actor);
        }
    }
}

pub struct MapPassThroughData {
    pub world_position: WorldPosition,
    pub random: Random,

    pub terrain_tileset_id: u8,
    pub feature_tileset_id: u8,
    pub item_tileset_id: u8,

    pub terrain_layer_entity: Entity,
    pub feature_layer_entity: Entity,
    pub item_layer_entity: Entity,
    // TODO: Explored tiles should be passed from serialized data for the map on loading, or just a new HashSet
    //pub explored_tiles: HashSet<UVec2>
}

impl From<MapGenData<MapPassThroughData>> for Map {
    fn from(data: MapGenData<MapPassThroughData>) -> Self {
        let mut terrain_types = Grid::new_default(data.size);

        for y in 0..data.size.height() {
            for x in 0..data.size.width() {
                let v = *data.grid.get_unchecked((x, y));
                terrain_types.set((x, y), v.into());
            }
        }

        Self {
            size: data.size,
            update_all: true,

            update_tiles: HashSet::new(),
            random: data.user_data.random,
            actors: Grid::new_default(data.size),
            world_position: data.user_data.world_position,

            terrain_types,
            item_types: Grid::new_default(data.size),
            feature_types: Grid::new_default(data.size),

            item_tileset_id: data.user_data.item_tileset_id,
            terrain_tileset_id: data.user_data.terrain_tileset_id,
            feature_tileset_id: data.user_data.feature_tileset_id,

            item_layer_entity: data.user_data.item_layer_entity,
            terrain_layer_entity: data.user_data.terrain_layer_entity,
            feature_layer_entity: data.user_data.feature_layer_entity,

            // TODO: Add explored_tiles HashSet to MapPassThroughData
            explored_tiles: HashSet::new(),
            pathmap: BitGrid::new_default(data.size),
        }
    }
}

impl FovProvider for Map {
    fn is_opaque(&self, position: IVec2, vision_type: VisionType) -> bool {
        if self.size.contains(position) {
            !self.can_see_through(position, vision_type)
        } else {
            true
        }
    }
}

impl PathMap for Map {
    type ExitIterator = arrayvec::IntoIter<(IVec2, OrderedFloat<f32>), 8>;

    fn cost(&self, node: impl Point2d, movement_component: &Movement) -> OrderedFloat<f32> {
        if self.can_move_through(node, movement_component) {
            self.terrain_types.get(node).map_or(0.0, |t| t.get_movement_cost()).into()
        } else {
            0.0.into()
        }
    }

    fn distance(&self, a: impl Point2d, b: impl Point2d) -> OrderedFloat<f32> {
        a.taxi_dist(b).into()
    }

    fn successors(&self, p: impl Point2d, movement_component: &Movement) -> Self::ExitIterator {
        let mut points = arrayvec::ArrayVec::new();

        for adj in p.adj_8() {
            if !self.pathmap.in_bounds(adj / IVec2::new(4, 1)) {
                continue;
            }

            if !self.pathmap.get_unchecked(adj) && self.can_move_through(adj, movement_component) {
                points.push((adj, self.cost(adj, movement_component)));
            }
        }

        points.into_iter()
    }
}
