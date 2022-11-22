use crate::prelude::*;

const SCALE_F32_TO_U32: f32 = 10.0;
const CARDINAL_COST_F32: f32 = 1.0;
const ORDINAL_COST_F32: f32 = 1.4;
const CARDINAL_COST: u32 = (CARDINAL_COST_F32 * SCALE_F32_TO_U32) as u32;
const ORDINAL_COST: u32 = (ORDINAL_COST_F32 * SCALE_F32_TO_U32) as u32;

pub(super) struct AStarNode {
    is_walkable: bool,
    position: IVec2,
    cost_multiplier: u32,
    from_node: Option<IVec2>,

    cost_from_start: u32,
    cost_from_end: u32,
    cost_total: u32,
}

impl AStarNode {
    pub fn new(origin: IVec2, destination: IVec2) -> AStarNode {
        let from_end = (DistanceAlg::DiagonalWithCosts(CARDINAL_COST_F32, ORDINAL_COST_F32)
            .distance2d(origin, destination)
            * SCALE_F32_TO_U32) as u32;

        Self {
            is_walkable: true,
            position: origin,
            cost_multiplier: 0, // we are already here
            from_node: None,

            cost_from_start: u32::MIN,
            cost_from_end: from_end,
            cost_total: from_end,
        }
    }

    fn create_neighbor(
        &self,
        position: IVec2,
        is_diagonal: bool,
        destination: IVec2,
        provider: &impl PathProvider,
        movement_type: u8,
    ) -> AStarNode {
        let cost_from_end = (DistanceAlg::DiagonalWithCosts(CARDINAL_COST_F32, ORDINAL_COST_F32)
            .distance2d(position, destination)
            * SCALE_F32_TO_U32) as u32;

        let mut s = AStarNode {
            is_walkable: provider.is_walkable(position, movement_type),
            position,
            cost_multiplier: provider.cost(position, movement_type),

            from_node: None,
            cost_from_start: u32::MAX,
            cost_from_end,
            cost_total: u32::MAX,
        };

        if s.is_walkable {
            let new_cost_from_start = self.cost_from_start
                + if is_diagonal { ORDINAL_COST } else { CARDINAL_COST } * s.cost_multiplier;
            s.update_node(self, new_cost_from_start);
        }

        s
    }

    pub fn position(&self) -> IVec2 {
        self.position
    }

    pub fn from_node(&self) -> Option<IVec2> {
        self.from_node
    }

    fn update_total(&mut self) {
        if self.is_walkable {
            self.cost_total = self.cost_from_start + self.cost_from_end;
        }
    }

    /// perform walkable / cost checks before calling this
    /// this function is "unchecked"
    fn update_node(&mut self, other: &AStarNode, new_cost_from_start: u32) {
        self.cost_from_start = new_cost_from_start;
        self.from_node = Some(other.position);
        self.update_total();
    }

    pub fn update_at_position(
        &self,
        position: IVec2,
        is_diagonal: bool,
        destination: IVec2,
        provider: &impl PathProvider,
        movement_type: u8,
        open_nodes: &mut IndexList<AStarNode>,
        closed_nodes: &mut IndexList<AStarNode>,
    ) {
        if let Some(_neighbor_index) = Self::find_node_with_position(closed_nodes, position) {
            // Neighbor Closed Nothing to do
        } else if let Some(neighbor_index) = Self::find_node_with_position(open_nodes, position) {
            // Update Neighbor
            let neighbor = open_nodes.get(neighbor_index).unwrap(); // unwrap is safe because we still have a valid index
            let new_cost_from_start = self.cost_from_start
                + if is_diagonal { ORDINAL_COST } else { CARDINAL_COST } * neighbor.cost_multiplier;
            if neighbor.is_walkable && neighbor.cost_from_start > new_cost_from_start {
                let mut neighbor = open_nodes.remove(neighbor_index).unwrap(); // unwrap is safe because we sill have a valid index
                neighbor.update_node(self, new_cost_from_start);
                Self::insert_ordered(open_nodes, neighbor);
            }
        } else {
            let new_neighbor =
                self.create_neighbor(position, is_diagonal, destination, provider, movement_type);
            if new_neighbor.is_walkable {
                Self::insert_ordered(open_nodes, new_neighbor);
            }
        }
    }

    fn insert_ordered(list: &mut IndexList<AStarNode>, node_to_insert: AStarNode) {
        let mut iter_index = list.first_index();
        let mut found_index = None;

        while iter_index.is_some() {
            if let Some(current_node) = list.get(iter_index) {
                if node_to_insert > *current_node {
                    iter_index = list.next_index(iter_index);
                    continue;
                }

                found_index = Some(iter_index);
                break;
            }

            iter_index = list.next_index(iter_index);
        }

        match found_index {
            Some(next_index) => list.insert_before(next_index, node_to_insert),
            None => list.insert_last(node_to_insert),
        };
    }

    pub fn find_node_with_position(list: &IndexList<AStarNode>, position: IVec2) -> Option<Index> {
        let mut index = list.first_index();

        while index.is_some() {
            if let Some(node) = list.get(index) {
                if position == node.position {
                    return Some(index);
                }
            }
            index = list.next_index(index);
        }

        None
    }
}

impl PartialEq for AStarNode {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

impl PartialOrd for AStarNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.cost_total > other.cost_total {
            Some(std::cmp::Ordering::Greater)
        } else if self.cost_total < other.cost_total {
            Some(std::cmp::Ordering::Less)
        } else {
            if self.cost_from_end < other.cost_from_end {
                Some(std::cmp::Ordering::Less)
            } else if self.cost_from_end > other.cost_from_end {
                Some(std::cmp::Ordering::Greater)
            } else {
                Some(std::cmp::Ordering::Equal)
            }
        }
    }
}