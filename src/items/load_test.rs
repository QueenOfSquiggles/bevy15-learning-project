use bevy::prelude::*;

use crate::{items::Item, toast::ToastEvent};

use super::ItemType;

pub struct LoadTestPlugin;

impl Plugin for LoadTestPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ItemRegistry>();
        app.add_systems(Startup, dispatch_load);
        app.add_systems(Update, events_item_added);
    }
}

#[derive(Resource, Default)]
struct ItemRegistry {
    items: Vec<Handle<ItemType>>,
}

fn dispatch_load(assets: Res<AssetServer>, mut registry: ResMut<ItemRegistry>) {
    registry.items.push(assets.load("item/test_basic.json"));
    registry.items.push(assets.load("item/test_weapon.json"));
}

fn events_item_added(
    mut events: EventReader<AssetEvent<ItemType>>,
    items: Res<Assets<ItemType>>,
    mut cmd: Commands,
) {
    for event in events.read() {
        let AssetEvent::LoadedWithDependencies { id } = event else {
            return;
        };
        let Some(item) = items.get(*id) else {
            return;
        };
        cmd.trigger(ToastEvent(format!("Item loaded: {:?}", item.get_name())));
    }
}
