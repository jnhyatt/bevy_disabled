use bevy_ecs::{
    entity::Entity,
    component::Component,
    world::World,
};

#[derive(Component, Debug, Clone, Copy)]
pub struct Disabled<C>(pub C);

pub fn enable<C: Component>(e: Entity, world: &mut World) {
    let mut e = world.entity_mut(e);
    if let Some(Disabled(c)) = e.take::<Disabled<C>>()  {
        e.insert(c);
    }
}

pub fn disable<C: Component>(e: Entity, world: &mut World) {
    let mut e = world.entity_mut(e);
    if let Some(c) = e.take::<C>()  {
        e.insert(Disabled(c));
    }
}

pub fn toggle<C: Component>(e: Entity, world: &mut World) {
    let mut e = world.entity_mut(e);
    if let Some(c) = e.take::<C>() {
        e.insert(Disabled(c));
    } else {
        if let Some(Disabled(c)) = e.take::<Disabled<C>>() {
            e.insert(c);
        }
    }
}
