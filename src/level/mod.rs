use bevy::{prelude::*, state::state::FreelyMutableState};
use bevy_asset_loader::{
    asset_collection::AssetCollection,
    loading_state::{config::ConfigureLoadingState, LoadingState, LoadingStateAppExt},
};
use feature_garden::LevelFeatureGarden;

mod feature_garden;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        register_level::<LevelFeatureGarden, _>(app);
    }
}

#[derive(Event)]
pub struct EventStartLoadingLevel;
#[derive(Event)]
pub struct EventEndLoadingLevel;

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum LevelState {
    #[default]
    LoadFeatureGarden,
    PlayFeatureGarden,
}

pub trait LevelDescription<StateType>
where
    StateType: States + FreelyMutableState,
{
    type LevelAssets: AssetCollection;
    const LOAD_STATE: StateType;
    const LEVEL_STATE: StateType;
    const ADDITIONAL_SETUP: Option<fn(&mut App)>;
}

/// Registers a proper level architecture for easy loading of associated assets
fn register_level<Level, StateType>(app: &mut App)
where
    StateType: States + FreelyMutableState + Default,
    Level: LevelDescription<StateType>,
{
    app.init_state::<StateType>();
    app.enable_state_scoped_entities::<StateType>();
    app.add_loading_state(
        LoadingState::new(Level::LOAD_STATE)
            .continue_to_state(Level::LEVEL_STATE)
            .load_collection::<Level::LevelAssets>(),
    );
    app.add_systems(OnEnter(Level::LOAD_STATE), |mut cmd: Commands| {
        cmd.trigger(EventStartLoadingLevel);
        cmd.spawn((
            // a state scoped label to show we are loading
            Text::new("Loading, please wait..."),
            StateScoped(Level::LOAD_STATE),
        ));
    });
    app.add_systems(OnEnter(Level::LEVEL_STATE), |mut cmd: Commands| {
        cmd.trigger(EventEndLoadingLevel);
    });
    if let Some(setup) = Level::ADDITIONAL_SETUP {
        setup(app);
    }
}
