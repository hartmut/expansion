use crate::core::component::shadow::*;
use crate::core::component::*;
use bevy::prelude::*;

pub fn shadow_clear(mut shadow_query: Query<&mut Shadow>) {
    for mut shadow in shadow_query.iter_mut() {
        shadow.energy = energy::Energy::default();
    }
}

pub fn shadow_update_module(
    mut energy_query: Query<&mut energy::Energy>,
    mut query: Query<(Entity, &Children, &mut Shadow), With<tags::ModuleTag>>,
) {
    for (entity, children, mut shadow) in query.iter_mut() {
        for child in children.iter() {
            let energy = energy_query.get(*child);
            if let Ok(e) = energy {
                shadow.energy.act_storage += e.act_storage;
                shadow.energy.max_storage += e.max_storage;
                shadow.energy.production += e.production;
                shadow.energy.consumption += e.consumption;
            }
        }
        let mut entity_energy = energy_query.get_mut(entity).unwrap();
        entity_energy.act_storage = shadow.energy.act_storage;
        entity_energy.max_storage = shadow.energy.max_storage;
        entity_energy.production = shadow.energy.production;
        entity_energy.consumption = shadow.energy.consumption;
    }
}

pub fn shadow_update_station(
    mut energy_query: Query<&mut energy::Energy>,
    mut query: Query<(Entity, &Children, &mut Shadow), With<tags::StationTag>>,
) {
    for (entity, children, mut shadow) in query.iter_mut() {
        for child in children.iter() {
            let energy = energy_query.get(*child);
            if let Ok(e) = energy {
                shadow.energy.act_storage += e.act_storage;
                shadow.energy.max_storage += e.max_storage;
                shadow.energy.production += e.production;
                shadow.energy.consumption += e.consumption;
            }
        }
        let mut entity_energy = energy_query.get_mut(entity).unwrap();
        entity_energy.act_storage = shadow.energy.act_storage;
        entity_energy.max_storage = shadow.energy.max_storage;
        entity_energy.production = shadow.energy.production;
        entity_energy.consumption = shadow.energy.consumption;
    }
}
