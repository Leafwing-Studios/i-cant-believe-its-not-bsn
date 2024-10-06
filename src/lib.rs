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
///
/// ```rust
/// use bevy_ecs::prelude::*;
/// use i_cant_believe_its_not_bsn::WithChild;
///
/// #[derive(Component, PartialEq, Debug)]
/// struct A;
///
/// #[derive(Component, PartialEq, Debug)]
/// struct B(u8);
///
/// fn spawn_hierarchy(mut commands: Commands) {
///   commands.spawn(
///    (A, // Parent
///     WithChild( // This component is removed on spawn
///       (A, B(3)) // Child
///     )
///   ));
/// }
/// ```
#[derive(Debug, Clone)]
pub struct WithChild<B: Bundle>(pub B);

impl<B: Bundle> Component for WithChild<B> {
    /// This is a sparse set component as it's only ever added and removed, never iterated over.
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
        let Some(mut entity_mut) = world.get_entity_mut(self.parent_entity) else {
            #[cfg(debug_assertions)]
            panic!("Parent entity not found");

            #[cfg(not(debug_assertions))]
            return;
        };

        let Some(with_child_component) = entity_mut.take::<WithChild<B>>() else {
            #[cfg(debug_assertions)]
            panic!("WithChild component not found");

            #[cfg(not(debug_assertions))]
            return;
        };

        let child_entity = world.spawn(with_child_component.0).id();
        world.entity_mut(self.parent_entity).add_child(child_entity);
    }
}

/// A component that, when added to an entity, will add a child entity with the given bundle.
///
/// This component will be removed from the entity, as its data is moved into the child entity.
///
/// Under the hood, this is done using component lifecycle hooks.
///
/// ```rust
/// use bevy_ecs::prelude::*;
/// use i_cant_believe_its_not_bsn::WithChild;
///
/// #[derive(Component, PartialEq, Debug)]
/// struct A;
///
/// #[derive(Component, PartialEq, Debug)]
/// struct B(u8);
///
/// fn spawn_hierarchy(mut commands: Commands) {
///   commands.spawn(
///    (A, // Parent
///     WithChild( // This component is removed on spawn
///       (A, B(3)) // Child
///     )
///   ));
/// }
/// ```
#[derive(Debug, Clone)]
pub struct WithChildren<B: Bundle>(pub B);

impl<B: Bundle> Component for WithChildren<B> {
    /// This is a sparse set component as it's only ever added and removed, never iterated over.
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(with_children_hook::<B>);
    }
}

/// A hook that runs whenever [`WithChildren`] is added to an entity.
///
/// Generates a [`ReplaceWithChildCommand`].
fn with_children_hook<'w, B: Bundle>(
    mut world: DeferredWorld<'w>,
    entity: Entity,
    _component_id: ComponentId,
) {
    // Component hooks can't perform structural changes, so we need to rely on commands.
    world.commands().add(WithChildrenCommand {
        parent_entity: entity,
        _phantom: PhantomData::<B>,
    });
}

struct WithChildrenCommand<B> {
    parent_entity: Entity,
    _phantom: PhantomData<B>,
}

impl<B: Bundle> Command for WithChildrenCommand<B> {
    fn apply(self, world: &mut World) {
        let Some(mut entity_mut) = world.get_entity_mut(self.parent_entity) else {
            #[cfg(debug_assertions)]
            panic!("Parent entity not found");

            #[cfg(not(debug_assertions))]
            return;
        };

        let Some(with_child_component) = entity_mut.take::<WithChild<B>>() else {
            #[cfg(debug_assertions)]
            panic!("WithChild component not found");

            #[cfg(not(debug_assertions))]
            return;
        };

        let child_entity = world.spawn(with_child_component.0).id();
        world.entity_mut(self.parent_entity).add_child(child_entity);
    }
}

#[cfg(test)]
mod tests {
    use bevy_ecs::system::RunSystemOnce;
    use bevy_hierarchy::Children;

    use super::*;

    #[derive(Component, PartialEq, Debug)]
    struct A;

    #[derive(Component, PartialEq, Debug)]
    struct B(u8);

    #[derive(Bundle)]
    struct ABBundle {
        a: A,
        b: B,
    }

    #[derive(Bundle)]
    struct HierarchicalBundle {
        a: A,
        child: WithChild<ABBundle>,
    }

    #[test]
    fn with_child() {
        let mut world = World::default();

        let parent = world.spawn(WithChild((A, B(3)))).id();
        // FIXME: this should not be needed!
        world.flush();

        assert!(!world.entity(parent).contains::<WithChild<(A, B)>>());
        assert!(!world.entity(parent).contains::<A>());
        assert!(!world.entity(parent).contains::<B>());

        let children = world.get::<Children>(parent).unwrap();
        assert_eq!(children.len(), 1);

        let child_entity = children[0];
        assert_eq!(world.get::<A>(child_entity), Some(&A));
        assert_eq!(world.get::<B>(child_entity), Some(&B(3)));
    }

    #[test]
    fn hierarchical_bundle() {
        let mut world = World::default();

        let parent = world
            .spawn(HierarchicalBundle {
                a: A,
                child: WithChild(ABBundle { a: A, b: B(17) }),
            })
            .id();

        // FIXME: this should not be needed!
        world.flush();

        assert!(!world.entity(parent).contains::<WithChild<ABBundle>>());
        assert!(world.entity(parent).contains::<A>());
        assert!(!world.entity(parent).contains::<B>());

        let children = world.get::<Children>(parent).unwrap();
        assert_eq!(children.len(), 1);

        let child_entity = children[0];
        assert_eq!(world.get::<A>(child_entity), Some(&A));
        assert_eq!(world.get::<B>(child_entity), Some(&B(17)));
    }

    #[test]
    fn command_form() {
        fn spawn_with_child(mut commands: Commands) -> Entity {
            commands.spawn((A, WithChild(B(5)))).id()
        }

        let mut world = World::new();
        let parent = world.run_system_once(spawn_with_child);

        assert!(!world.entity(parent).contains::<WithChild<B>>());
        assert!(world.entity(parent).contains::<A>());
        assert!(!world.entity(parent).contains::<B>());

        let children = world.get::<Children>(parent).unwrap();
        assert_eq!(children.len(), 1);

        let child_entity = children[0];
        assert_eq!(world.get::<B>(child_entity), Some(&B(5)));
    }
}
