use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct PlayerInputsPlugin;

impl Plugin for PlayerInputsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<Inputs>::default());
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Debug, Reflect, Actionlike)]
#[actionlike(Button)]
pub enum Inputs {
    #[actionlike(DualAxis)]
    Move,
    #[actionlike(DualAxis)]
    Look,
    Jump,
    Dodge,
    Interact,
    Attack,
}

pub fn player_root_bundle() -> InputManagerBundle<Inputs> {
    InputManagerBundle::with_map(
        InputMap::<Inputs>::default()
            .with_dual_axis(Inputs::Move, VirtualDPad::wasd())
            .with_dual_axis(
                Inputs::Look,
                MouseMove::default()
                    .with_processor(DualAxisProcessor::Inverted(DualAxisInverted::ALL)),
            )
            .with(Inputs::Dodge, KeyCode::ShiftLeft)
            .with(Inputs::Interact, KeyCode::KeyE)
            .with(Inputs::Jump, KeyCode::Space)
            .with(Inputs::Attack, MouseButton::Left),
    )
}
