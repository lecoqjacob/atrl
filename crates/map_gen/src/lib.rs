mod builders {
    mod cellular_automata_builder;
    pub use cellular_automata_builder::*;
    mod finalizer_builder;
    pub use finalizer_builder::*;
    mod scatter_builder;
    pub use scatter_builder::*;
    mod set_builder;
    pub use set_builder::*;
}

mod map_architect;
mod map_gen_data;
mod map_generator;

pub mod prelude {
    mod imports {
        pub use bevy::{
            ecs::schedule::StateData,
            prelude::*,
            utils::{HashMap, HashSet},
        };

        pub use iyes_loopless::prelude::*;

        pub use rand::prelude::*;

        pub use std::marker::PhantomData;

        pub use atrl_common::prelude::*;
    }
    pub(crate) use imports::*;

    pub use crate::builders::*;
    pub use crate::map_architect::*;
    pub use crate::map_gen_data::*;
    pub use crate::map_generator::*;
}