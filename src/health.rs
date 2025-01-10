use bevy::{ecs::component::StorageType, prelude::*};

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, process_die_on_health_zero);
    }
}

fn process_die_on_health_zero(
    q_health: Query<(Entity, &Health), With<DieOnHealthZero>>,
    mut cmd: Commands,
) {
    for (e, _) in q_health.iter().filter(|(_, hp)| hp.current == 0) {
        // TODO: replace with proper death callbacks
        cmd.entity(e).despawn_recursive();
    }
}

fn handle_health_affects(trigger: Trigger<HealthAffect>, mut q_health: Query<&mut Health>) {
    let Ok(mut hp) = q_health.get_mut(trigger.entity()) else {
        return;
    };
    hp.current = hp
        .current
        .checked_add_signed(trigger.delta)
        .unwrap_or(0)
        .min(hp.max);
}

#[derive(Debug, Event)]
pub struct HealthAffect {
    pub delta: i32,
}

#[derive(Debug, Default)]
pub struct Health {
    pub current: u32,
    pub max: u32,
}

impl Component for Health {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;
    fn register_component_hooks(_hooks: &mut bevy::ecs::component::ComponentHooks) {
        _hooks.on_add(|mut world, entity, _| {
            world
                .commands()
                .entity(entity)
                .observe(handle_health_affects);
        });
    }
}

#[derive(Debug, Component)]
#[require(Health)]
pub struct DieOnHealthZero;
