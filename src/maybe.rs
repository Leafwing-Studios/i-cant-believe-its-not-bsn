use bevy_ecs::{bundle::Bundle, component::Component};

/// A component that when added to an entity, will be removed from the entity and replaced with its contents if [`Some`].
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Maybe<B: Bundle>(pub Option<B>);

impl<B: Bundle> Maybe<B> {
    /// Creates a new `Maybe` component of type `B` with no bundle.
    pub const NONE: Self = Self(None);

    /// Creates a new `Maybe` component with the given bundle.
    pub const fn new(bundle: B) -> Self {
        Self(Some(bundle))
    }

    /// Returns the contents of the `Maybe` component, if any.
    pub fn into_inner(self) -> Option<B> {
        self.0
    }
}

impl<B: Bundle> Default for Maybe<B> {
    /// Defaults to [`Maybe::NONE`].
    fn default() -> Self {
        Self::NONE
    }
}
