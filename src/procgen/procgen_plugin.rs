use crate::prelude::*;

pub struct ProcgenPlugin<S: StateNext> {
    pub state_construct: S,
    pub state_running: S,
}

impl<S: StateNext> Plugin for ProcgenPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_enter_system(self.state_construct.clone(), Self::setup);
    }
}

impl<S: StateNext> ProcgenPlugin<S> {
    fn setup(ctx: Res<GameContext>, mut commands: Commands, mut state: ResMut<CurrentState<S>>) {
        /*
        let (start_x, start_y) = random_start_position(&mut ctx.get_rng());
        let chain = BuilderChain::new(0, [80, 50], "New Map")
            .start_with(CellularAutomataArchitect::new())
            .with(RoomMapArchitect::new())
            .with(AreaStartingPosition::new(start_x, start_y))
            .generate(&mut ctx.get_rng());

        let map = chain.get_map();

        #[cfg(feature = "debug")]
        {
            use crate::debug::colorized::Colorized;
            info!("{}", map.to_colorized_string());
        }

        commands.insert_resource(map);
        */
        state.set_next(&mut commands);
    }
}