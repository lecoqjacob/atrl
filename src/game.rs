#![allow(clippy::module_inception)]

mod abilities {
    mod ability_plugin;
    pub use ability_plugin::*;
}

mod actors {
    mod actor_bundle;
    pub use actor_bundle::*;
    mod actor_plugin;
    pub use actor_plugin::*;
    mod attributes;
    pub use attributes::*;
    mod class_type;
    pub use class_type::*;
    mod equipment_slots;
    pub use equipment_slots::*;
    mod race_type;
    pub use race_type::*;
    mod timing;
    pub use timing::*;
}

mod camera {
    mod systems {
        mod spawn_main_camera;
        pub use spawn_main_camera::*;
    }
    mod camera_plugin;
    pub use camera_plugin::*;
    mod main_camera;
    pub use main_camera::*;
}

mod items {
    mod item_bundle;
    pub use item_bundle::*;
    mod item_plugin;
    pub use item_plugin::*;
}

mod map {
    mod loader {
        mod change_theme;
        pub use change_theme::*;
        mod current_map;
        pub use current_map::*;
        mod map_loader;
        pub use map_loader::*;
    }
    pub use loader::*;

    mod renderer {
        mod map_renderer;
        pub use map_renderer::*;
        mod render_actions;
        pub use render_actions::*;
        mod render_context;
        pub use render_context::*;
        mod tile_bundle;
        pub use tile_bundle::*;
    }
    pub use renderer::*;

    mod systems {
        mod change_map_theme;
        pub use change_map_theme::*;
        mod create_renderer;
        pub use create_renderer::*;
        mod draw_map_tiles;
        pub use draw_map_tiles::*;
        mod load_first_map;
        pub use load_first_map::*;
        mod update_map_tiles;
        pub use update_map_tiles::*;
    }
    pub use systems::*;

    mod tiles {
        mod loader {
            mod color_definition;
            pub use color_definition::*;
            mod tile_template;
            pub use tile_template::*;
            mod texture_atlas_template;
            pub use texture_atlas_template::*;
            mod tile_loader;
            pub use tile_loader::*;
        }
        pub use loader::*;
        mod theme;
        pub use theme::*;
        mod tile_definition;
        pub use tile_definition::*;
    }
    pub use tiles::*;

    mod feature_type;
    pub use feature_type::*;
    mod item_type;
    pub use item_type::*;
    mod map_layer;
    pub use map_layer::*;
    mod map_plugin;
    pub use map_plugin::*;
    mod map;
    pub use map::*;
    mod terrain_type;
    pub use terrain_type::*;
}

mod player {
    mod player_bundle;
    pub use player_bundle::*;
    mod player_plugin;
    pub use player_plugin::*;
}

mod systems {
    mod apply_damage;
    pub use apply_damage::*;
    mod input;
    pub use input::*;
    mod move_actors;
    pub use move_actors::*;
    mod perform_healing;
    pub use perform_healing::*;
    mod player_input;
    pub use player_input::*;
    mod spawn_player;
    pub use spawn_player::*;
}

mod spawner {
    mod spawner_plugin;
    pub use spawner_plugin::*;
}

mod game_context;
mod game_plugin;

pub mod prelude {
    // Files inside of atrl::game *may*
    // use crate::game::prelude::internal::*;
    // Files outside of atrl::game should only
    // access raws from crate::prelude::*;
    pub mod internal {
        pub use super::super::abilities::*;
        pub use super::super::actors::*;
        pub use super::super::camera::*;
        pub use super::super::items::*;
        pub use super::super::map::*;
        pub use super::super::player::*;
        pub use super::super::spawner::*;
        pub use super::super::systems::*;

        pub use super::super::game_context::*;
        pub use super::super::game_plugin::*;
    }

    pub mod external {
        pub use super::super::actors::*;
        pub use super::super::camera::*;
        pub use super::super::game_plugin::*;
        pub use super::super::map::*;
    }
    // local
}
