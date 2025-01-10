use std::time::Duration;

use bevy::{ecs::component::StorageType, prelude::*, time::common_conditions::on_timer};

use crate::game_states::PauseState;

pub struct RpgStatsPlugin;

impl Plugin for RpgStatsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_modifiers, update_computed_stats)
                .chain()
                .run_if(in_state(PauseState::Running).and(on_timer(Duration::from_secs_f32(0.1)))),
        );
    }
}

fn handle_append_modifier(
    trigger: Trigger<EventAppendStatModifier>,
    mut q_stats: Query<&mut BaseStats>,
) {
    let Ok(mut stats) = q_stats.get_mut(trigger.entity()) else {
        return;
    };
    let container = match trigger.event().target {
        RpgStatTarget::Strength => &mut stats.strength,
        RpgStatTarget::Dexterity => &mut stats.dexterity,
        RpgStatTarget::Consitition => &mut stats.constitution,
        RpgStatTarget::Intelligence => &mut stats.intelligence,
        RpgStatTarget::Wisdom => &mut stats.wisdom,
        RpgStatTarget::Charisma => &mut stats.charisma,
    };
    container.modifiers.push(RpgStatModifier {
        owner: trigger.event().owner_ref,
        modifier: trigger.event().modifier.clone(),
    });
}

fn update_modifiers(mut query: Query<&mut BaseStats>, q_entities: Query<Entity>) {
    for mut stats in query.iter_mut() {
        clear_dead_modifiers(&mut stats.strength, &q_entities);
        clear_dead_modifiers(&mut stats.dexterity, &q_entities);
        clear_dead_modifiers(&mut stats.constitution, &q_entities);
        clear_dead_modifiers(&mut stats.intelligence, &q_entities);
        clear_dead_modifiers(&mut stats.wisdom, &q_entities);
        clear_dead_modifiers(&mut stats.charisma, &q_entities);
    }
}

fn clear_dead_modifiers(container: &mut RpgStatContainer, q: &Query<Entity>) {
    container.modifiers = container
        .modifiers
        .clone()
        .into_iter()
        .filter(|p| q.contains(p.owner))
        .collect();
}

fn update_computed_stats(mut query: Query<(&mut ComputedStats, &BaseStats)>) {
    for (mut computed, base) in query.iter_mut() {
        computed.strength = compute_stat(&base.strength);
        computed.dexterity = compute_stat(&base.dexterity);
        computed.constitution = compute_stat(&base.constitution);
        computed.intelligence = compute_stat(&base.intelligence);
        computed.wisdom = compute_stat(&base.wisdom);
        computed.charisma = compute_stat(&base.charisma);
    }
}

fn compute_stat(container: &RpgStatContainer) -> i32 {
    let mut additive: i32 = 0;
    let mut multiplicative: i32 = 0;
    let mut minimum = i32::MIN;
    let mut maximum = i32::MAX;
    for statmod in &container.modifiers {
        match statmod.modifier {
            ModificationType::Additive(val) => {
                additive += val;
            }
            ModificationType::Multiplicative(val) => {
                // This is intentional
                // Pathfinder2e uses an additive minus one multiplier
                // For example if there are 3 buffs that double (mul by 2) a stat
                // The final value would be 2 + (2-1) + (2-1) = 4
                if multiplicative == 0 {
                    multiplicative += val;
                } else {
                    multiplicative += val - val.signum();
                }
            }
            ModificationType::Minimum(val) => {
                minimum = minimum.max(val);
            }
            ModificationType::Maximum(val) => {
                maximum = maximum.min(val);
            }
        }
    }
    ((container.base_value + additive) * multiplicative).clamp(minimum, maximum)
}

#[derive(Debug, Event)]
pub struct EventAppendStatModifier {
    owner_ref: Entity,
    target: RpgStatTarget,
    modifier: ModificationType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RpgStatTarget {
    Strength,
    Dexterity,
    Consitition,
    Intelligence,
    Wisdom,
    Charisma,
}

#[derive(Debug, /*Component,*/ Clone)]
pub struct BaseStats {
    strength: RpgStatContainer,
    dexterity: RpgStatContainer,
    constitution: RpgStatContainer,
    intelligence: RpgStatContainer,
    wisdom: RpgStatContainer,
    charisma: RpgStatContainer,
}

impl Component for BaseStats {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;

    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_add(|mut world, entity, _| {
            world
                .commands()
                .entity(entity)
                .observe(handle_append_modifier);
        });
    }

    fn register_required_components(
        _component_id: bevy::ecs::component::ComponentId,
        components: &mut bevy::ecs::component::Components,
        storages: &mut bevy::ecs::storage::Storages,
        required_components: &mut bevy::ecs::component::RequiredComponents,
        inheritance_depth: u16,
    ) {
        required_components.register(
            components,
            storages,
            || ComputedStats::new(),
            inheritance_depth,
        );
    }
}

#[derive(Debug, Clone)]
pub struct RpgStatContainer {
    base_value: i32,
    modifiers: Vec<RpgStatModifier>,
}

#[derive(Debug, Clone)]
pub struct RpgStatModifier {
    // owner that determines whether this modifier should be present or not.
    owner: Entity,
    modifier: ModificationType,
}

#[derive(Debug, Clone)]
pub enum ModificationType {
    Additive(i32),
    Multiplicative(i32),
    Minimum(i32),
    Maximum(i32),
}

#[derive(Debug, Component, Clone, Default)]
pub struct ComputedStats {
    strength: i32,
    dexterity: i32,
    constitution: i32,
    intelligence: i32,
    wisdom: i32,
    charisma: i32,
}

impl ComputedStats {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_base(base: &BaseStats) -> Self {
        Self {
            strength: base.strength.base_value,
            dexterity: base.dexterity.base_value,
            constitution: base.constitution.base_value,
            intelligence: base.intelligence.base_value,
            wisdom: base.wisdom.base_value,
            charisma: base.charisma.base_value,
        }
    }
}
