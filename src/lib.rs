#![doc = include_str!("../README.md")]

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
/// The const generic parameter `N` allows for multiple `WithChild` components of the same bundle type.
///
/// You can add multiple children in this way, if and only if their bundle types are distinct.
/// See [`WithChildren`] for a version that supports adding multiple children of the same type.
///
/// Under the hood, this is done using component lifecycle hooks.
///
/// ```rust
/// use bevy_ecs::prelude::*;
/// use i_cant_believe_its_not_bsn::WithChild;
///
/// #[derive(Component)]
/// struct A;
///
/// #[derive(Component)]
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
pub struct WithChildN<B: Bundle, const N: u8>(pub B);

impl<B: Bundle, const N: u8> Component for WithChildN<B, N> {
    /// This is a sparse set component as it's only ever added and removed, never iterated over.
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(with_child_hook::<B, N>);
    }
}

/// A hook that runs whenever [`WithChild`] is added to an entity.
///
/// Generates a [`WithChildCommand`].
fn with_child_hook<B: Bundle, const N: u8>(
    mut world: DeferredWorld<'_>,
    entity: Entity,
    _component_id: ComponentId,
) {
    // Component hooks can't perform structural changes, so we need to rely on commands.
    world.commands().add(WithChildCommand::<B, N> {
        parent_entity: entity,
        _phantom: PhantomData,
    });
}

struct WithChildCommand<B, const N: u8> {
    parent_entity: Entity,
    _phantom: PhantomData<B>,
}

impl<B: Bundle, const N: u8> Command for WithChildCommand<B, N> {
    fn apply(self, world: &mut World) {
        let Some(mut entity_mut) = world.get_entity_mut(self.parent_entity) else {
            #[cfg(debug_assertions)]
            panic!("Parent entity not found");

            #[cfg(not(debug_assertions))]
            return;
        };

        let Some(with_child_component) = entity_mut.take::<WithChildN<B, N>>() else {
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
/// This component will be removed from the entity immediately upon being spawned,
/// and the supplied iterator will be iterated to completion to generate the data needed for each child.
/// See [`WithChild`] for a more convenient API when adding only one child (or multiple children with distinct bundle types).
///
/// The const generic parameter `N` allows for multiple `WithChildren` components of the same bundle type.
///
/// Under the hood, this is done using component lifecycle hooks.
///
/// # Examples
///
/// Just like when using [`Commands::spawn_batch`], any iterator that returns a bundle of the same type can be used.
///
/// Working with vectors, arrays and other collections is straightforward:
///
/// ```rust
/// use bevy_ecs::prelude::*;
/// use i_cant_believe_its_not_bsn::WithChildren;
///
/// #[derive(Component)]
/// struct Name(&'static str);
///
/// fn spawn_hierarchy_with_vector(mut commands: Commands) {
///   commands.spawn(
///    (Name("Zeus"),
///     WithChildren([Name("Athena"), Name("Apollo"), Name("Hermes")])
///   ));
/// }
///```
///
/// However, generator-style iterators can also be used to dynamically vary the number and property of children:
///
/// ```rust
/// use bevy_ecs::prelude::*;
/// use i_cant_believe_its_not_bsn::WithChildren;
///
/// #[derive(Component)]
/// struct A;
///
/// #[derive(Component)]
/// struct ChildNumber(usize);
///
/// fn spawn_hierarchy_with_generator(mut commands: Commands) {
///   commands.spawn(
///    (A, // Component on parent
///      WithChildren((0..3).map(|i| (ChildNumber(i)))) // Each child will have a ChildNumber component
///    ));
/// }
///```
#[derive(Debug, Clone, Default)]
pub struct WithChildrenN<B: Bundle, I: IntoIterator<Item = B>, const N: u8>(pub I);

impl<B: Bundle, I: IntoIterator<Item = B> + Send + Sync + 'static, const N: u8> Component
    for WithChildrenN<B, I, N>
{
    /// This is a sparse set component as it's only ever added and removed, never iterated over.
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(with_children_hook::<B, I, N>);
    }
}

/// A hook that runs whenever [`WithChildren`] is added to an entity.
///
/// Generates a [`WithChildrenCommand`].
fn with_children_hook<B: Bundle, I: IntoIterator<Item = B> + Send + Sync + 'static, const N: u8>(
    mut world: DeferredWorld<'_>,
    entity: Entity,
    _component_id: ComponentId,
) {
    // Component hooks can't perform structural changes, so we need to rely on commands.
    world.commands().add(WithChildrenCommand::<B, I, N> {
        parent_entity: entity,
        _phantom: PhantomData,
    });
}

struct WithChildrenCommand<B, I, const N: u8> {
    parent_entity: Entity,
    _phantom: PhantomData<(B, I)>,
}

impl<B: Bundle, I: IntoIterator<Item = B> + Send + Sync + 'static, const N: u8> Command
    for WithChildrenCommand<B, I, N>
{
    fn apply(self, world: &mut World) {
        let Some(mut entity_mut) = world.get_entity_mut(self.parent_entity) else {
            #[cfg(debug_assertions)]
            panic!("Parent entity not found");

            #[cfg(not(debug_assertions))]
            return;
        };

        let Some(with_children_component) = entity_mut.take::<WithChildrenN<B, I, N>>() else {
            #[cfg(debug_assertions)]
            panic!("WithChildren component not found");

            #[cfg(not(debug_assertions))]
            return;
        };

        for child_bundle in with_children_component.0 {
            let child_entity = world.spawn(child_bundle).id();
            world.entity_mut(self.parent_entity).add_child(child_entity);
        }
    }
}

pub type WithChild<B> = WithChildN<B, 0>;
pub type WithChildren<B, I> = WithChildrenN<B, I, 0>;

pub fn spawn_child<B: Bundle>(bundle: B) -> WithChild<B> {
    WithChildN(bundle)
}

pub fn spawn_children<B: Bundle, I: IntoIterator<Item = B>>(iter: I) -> WithChildren<B, I> {
    WithChildrenN(iter)
}

pub fn spawn_repeated_child<B: Bundle, const N: u8>(bundle: B) -> WithChildN<B, N> {
    WithChildN(bundle)
}

pub fn spawn_repeated_children<B: Bundle, I: IntoIterator<Item = B>, const N: u8>(
    iter: I,
) -> WithChildrenN<B, I, N> {
    WithChildrenN(iter)
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

        let parent = world.spawn(spawn_child((A, B(3)))).id();
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
    fn with_children_vec() {
        let mut world = World::default();

        let parent = world.spawn(spawn_children(vec![B(0), B(1), B(2)])).id();
        // FIXME: this should not be needed!
        world.flush();

        assert!(!world.entity(parent).contains::<WithChildren<B, Vec<B>>>());
        assert!(!world.entity(parent).contains::<B>());

        let children = world.get::<Children>(parent).unwrap();
        assert_eq!(children.len(), 3);

        for (i, child_entity) in children.iter().enumerate() {
            assert_eq!(world.get::<B>(*child_entity), Some(&B(i as u8)));
        }
    }

    #[test]
    fn with_child_closure() {
        let mut world = World::default();

        let parent = world.spawn(spawn_children((0..7).map(|i| B(i as u8)))).id();
        // FIXME: this should not be needed!
        world.flush();

        assert!(!world.entity(parent).contains::<WithChildren<B, Vec<B>>>());
        assert!(!world.entity(parent).contains::<B>());

        let children = world.get::<Children>(parent).unwrap();
        assert_eq!(children.len(), 7);

        for (i, child_entity) in children.iter().enumerate() {
            assert_eq!(world.get::<B>(*child_entity), Some(&B(i as u8)));
        }
    }

    #[test]
    fn with_distinct_children() {
        let mut world = World::default();

        let parent = world.spawn((spawn_child(A), spawn_child(B(1)))).id();
        // FIXME: this should not be needed!
        world.flush();

        let children = world.get::<Children>(parent).unwrap();
        assert_eq!(children.len(), 2);
        assert_eq!(world.get::<A>(children[0]), Some(&A));
        assert_eq!(world.get::<B>(children[1]), Some(&B(1)));

        // Ordering should matter
        let parent = world.spawn((spawn_child(B(1)), spawn_child(A))).id();
        // FIXME: this should not be needed!
        world.flush();

        let children = world.get::<Children>(parent).unwrap();
        assert_eq!(children.len(), 2);
        assert_eq!(world.get::<B>(children[0]), Some(&B(1)));
        assert_eq!(world.get::<A>(children[1]), Some(&A));
    }

    #[test]
    fn grandchildren() {
        let mut world = World::default();

        let parent = world.spawn(spawn_child((A, spawn_child((A, B(3)))))).id();
        // FIXME: this should not be needed!
        world.flush();

        let children = world.get::<Children>(parent).unwrap();
        assert_eq!(children.len(), 1);

        let child_entity = children[0];
        assert_eq!(world.get::<A>(child_entity), Some(&A));

        let grandchildren = world.get::<Children>(child_entity).unwrap();
        assert_eq!(grandchildren.len(), 1);

        let grandchild_entity = grandchildren[0];
        assert_eq!(world.get::<A>(grandchild_entity), Some(&A));
        assert_eq!(world.get::<B>(grandchild_entity), Some(&B(3)));
    }

    #[test]
    fn hierarchical_bundle() {
        let mut world = World::default();

        let parent = world
            .spawn(HierarchicalBundle {
                a: A,
                child: spawn_child(ABBundle { a: A, b: B(17) }),
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
            commands.spawn((A, spawn_child(B(5)))).id()
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
