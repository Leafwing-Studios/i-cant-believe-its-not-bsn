use std::marker::PhantomData;

use bevy_ecs::{
    component::{ComponentHooks, ComponentId, StorageType},
    prelude::*,
    world::{Command, DeferredWorld},
};
use bevy_hierarchy::BuildWorldChildren;

/// A component that, when added to an entity, will add a child entity with the given bundle.
///
/// This component will be removed from the entity, as its data is moved into the child entity.
///
/// Under the hood, this is done using component lifecycle hooks.
#[derive(Debug, Clone)]
pub struct WithChild<B: Bundle>(B);

impl<B: Bundle> Component for WithChild<B> {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(with_child_hook::<B>);
    }
}

/// A hook that runs whenever [`WithChild`] is added to an entity.
///
/// Generates a [`ReplaceWithChildCommand`].
fn with_child_hook<'w, B: Bundle>(
    mut world: DeferredWorld<'w>,
    entity: Entity,
    _component_id: ComponentId,
) {
    // Component hooks can't perform structural changes, so we need to rely on commands.
    world.commands().add(WithChildCommand {
        parent_entity: entity,
        _phantom: PhantomData::<B>,
    });
}

struct WithChildCommand<B> {
    parent_entity: Entity,
    _phantom: PhantomData<B>,
}

impl<B: Bundle> Command for WithChildCommand<B> {
    fn apply(self, world: &mut World) {
        let Some(with_child_component) =
            world.entity_mut(self.parent_entity).take::<WithChild<B>>()
        else {
            return;
        };

        let child_entity = world.spawn(with_child_component.0).id();
        world.entity_mut(self.parent_entity).add_child(child_entity);
    }
}
