use bevy_ecs::bundle::Bundle;

pub struct Maybe<B: Bundle>(Option<B>);
