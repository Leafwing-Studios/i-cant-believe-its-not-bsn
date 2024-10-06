use std::{any::TypeId, marker::PhantomData};

use bevy_ecs::{
    component::{ComponentHooks, ComponentId, StorageType},
    prelude::*,
    world::{Command, DeferredWorld},
};
use bevy_hierarchy::BuildWorldChildren;

/// A component that, when added to an entity, will add a child entity with the given bundle.
///
/// This component will be removed from the entity, as its data is moved into the child entity.
/// See [`WithChildren`] for a version that supports adding multiple children.
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
#[derive(Debug, Clone, Default)]
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
/// Generates a [`WithChildCommand`].
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

/// A component that, when added to an entity, will add child entities with the given bundles.
///
/// This component will be removed from the entity, as its data is moved into child entities.
/// See [`WithChild`] for a simpler version that only adds a single child entity.
///
/// /// The 8 generic bundle types (`B1` through `B8`) represent the bundles of the children to be spawned,
/// in the order they will be spawned.
/// If a bundle type is `()`, no child will be spawned.
///
/// Because Rust doesn't support variadic generics, this component is limited to 8 children.
/// If you need more than that, please reconsider your choices and/or complain to Rust about the lack of variadic generics.
///
/// Not content with the standard type crimes involved and desperately want even more children?
/// Well, in that case, simply add another [`WithChildren`] component to the parent entity.
/// As long as the types of the [`WithChildren`] component are different (including all 8 generics), you can have as many children as you want.
///
///
/// Under the hood, this dark magic uses component lifecycle hooks.
///
/// # Example
///
/// ```rust
/// todo!();
/// ```
#[derive(Debug, Clone, Default)]
pub struct WithChildren<
    B1: Bundle = (),
    B2: Bundle = (),
    B3: Bundle = (),
    B4: Bundle = (),
    B5: Bundle = (),
    B6: Bundle = (),
    B7: Bundle = (),
    B8: Bundle = (),
> {
    pub b1: B1,
    pub b2: B2,
    pub b3: B3,
    pub b4: B4,
    pub b5: B5,
    pub b6: B6,
    pub b7: B7,
    pub b8: B8,
}

impl<
        B1: Bundle,
        B2: Bundle,
        B3: Bundle,
        B4: Bundle,
        B5: Bundle,
        B6: Bundle,
        B7: Bundle,
        B8: Bundle,
    > Component for WithChildren<B1, B2, B3, B4, B5, B6, B7, B8>
{
    /// This is a sparse set component as it's only ever added and removed, never iterated over.
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(with_children_hook::<B1, B2, B3, B4, B5, B6, B7, B8>);
    }
}

/// A hook that runs whenever [`WithChildren`] is added to an entity.
///
/// Generates a [`WithChildrenCommand`].
fn with_children_hook<
    'w,
    B1: Bundle,
    B2: Bundle,
    B3: Bundle,
    B4: Bundle,
    B5: Bundle,
    B6: Bundle,
    B7: Bundle,
    B8: Bundle,
>(
    mut world: DeferredWorld<'w>,
    entity: Entity,
    _component_id: ComponentId,
) {
    // Component hooks can't perform structural changes, so we need to rely on commands.
    world.commands().add(WithChildrenCommand {
        parent_entity: entity,
        _phantom: PhantomData::<(B1, B2, B3, B4, B5, B6, B7, B8)>,
    });
}

struct WithChildrenCommand<B1, B2, B3, B4, B5, B6, B7, B8> {
    parent_entity: Entity,
    _phantom: PhantomData<(B1, B2, B3, B4, B5, B6, B7, B8)>,
}

impl<
        B1: Bundle,
        B2: Bundle,
        B3: Bundle,
        B4: Bundle,
        B5: Bundle,
        B6: Bundle,
        B7: Bundle,
        B8: Bundle,
    > Command for WithChildrenCommand<B1, B2, B3, B4, B5, B6, B7, B8>
{
    fn apply(self, world: &mut World) {
        let Some(mut entity_mut) = world.get_entity_mut(self.parent_entity) else {
            #[cfg(debug_assertions)]
            panic!("Parent entity not found");

            #[cfg(not(debug_assertions))]
            return;
        };

        let Some(with_children_component) =
            entity_mut.take::<WithChildren<(B1, B2, B3, B4, B5, B6, B7, B8)>>()
        else {
            #[cfg(debug_assertions)]
            panic!("WithChildren component not found");

            #[cfg(not(debug_assertions))]
            return;
        };

        let WithChildren {
            b1,
            b2,
            b3,
            b4,
            b5,
            b6,
            b7,
            b8,
        } = with_children_component;

        if TypeId::of::<B1>() != TypeId::of::<()>() {
            let child_entity1 = world.spawn(b1).id();
            world
                .entity_mut(self.parent_entity)
                .add_child(child_entity1);
        }

        if TypeId::of::<B2>() != TypeId::of::<()>() {
            let child_entity2 = world.spawn(b2).id();
            world
                .entity_mut(self.parent_entity)
                .add_child(child_entity2);
        }

        if TypeId::of::<B3>() != TypeId::of::<()>() {
            let child_entity3 = world.spawn(b3).id();
            world
                .entity_mut(self.parent_entity)
                .add_child(child_entity3);
        }

        if TypeId::of::<B4>() != TypeId::of::<()>() {
            let child_entity4 = world.spawn(b4).id();
            world
                .entity_mut(self.parent_entity)
                .add_child(child_entity4);
        }

        if TypeId::of::<B5>() != TypeId::of::<()>() {
            let child_entity5 = world.spawn(b5).id();
            world
                .entity_mut(self.parent_entity)
                .add_child(child_entity5);
        }

        if TypeId::of::<B6>() != TypeId::of::<()>() {
            let child_entity6 = world.spawn(b6).id();
            world
                .entity_mut(self.parent_entity)
                .add_child(child_entity6);
        }

        if TypeId::of::<B7>() != TypeId::of::<()>() {
            let child_entity7 = world.spawn(b7).id();
            world
                .entity_mut(self.parent_entity)
                .add_child(child_entity7);
        }

        if TypeId::of::<B8>() != TypeId::of::<()>() {
            let child_entity8 = world.spawn(b8).id();
            world
                .entity_mut(self.parent_entity)
                .add_child(child_entity8);
        }
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
    fn with_children() {
        let mut world = World::default();

        let parent = world
            .spawn(WithChildren {
                b1: A,
                b2: B(2),
                b3: (A, B(3)),
                b4: (),
                b5: (),
                b6: (),
                b7: (),
                b8: (),
            })
            .id();
        // FIXME: this should not be needed!
        world.flush();

        let children = world.get::<Children>(parent).unwrap();
        assert_eq!(children.len(), 3);

        let child_entity_0 = children[0];
        assert_eq!(world.get::<A>(child_entity_0), Some(&A));

        let child_entity_1 = children[1];
        assert_eq!(world.get::<B>(child_entity_1), Some(&B(2)));

        let child_entity_2 = children[2];
        assert_eq!(world.get::<A>(child_entity_2), Some(&A));
        assert_eq!(world.get::<B>(child_entity_2), Some(&B(3)));
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
