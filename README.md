# i-cant-believe-its-not-bsn

An ergonomic way to spawn Bevy entity hierarchies using component hooks and the magic of the type system.

Eagerly [waiting for BSN](https://github.com/bevyengine/bevy/discussions/14437)?
Really wish you could spawn hierarchies with less boilerplate?
Just want to define some reusable widget types for `bevy_ui` in code?

Try `WithChild`, or its iterator sibling, `WithChildren`!
Just add it as a component holding the bundle you want to use to spawn the child, and you're off to the races.
A component hook will see that this component has been added, extract the data from your `WithChild` component, and then move it into a child, cleaning itself up as it goes.

Have fun!
