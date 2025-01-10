# i-cant-believe-its-not-bsn

Ergonomic ways to spawn Bevy entity hierarchies.

Eagerly [waiting for BSN](https://github.com/bevyengine/bevy/discussions/14437)?
Really wish you could spawn hierarchies with less boilerplate?
Just want to define some reusable widget types for `bevy_ui` in code?

This crate is here to help!

# Helper Components

You can use the helper component `WithChild`, and its iterator sibling, `WithChildren`, to embed hierarchy information in normal bundles.

Just add it as a component holding the bundle you want to use to spawn the child, and you're off to the races.
A component hook will see that this component has been added, extract the data from your `WithChild` component, and then move it into a child, cleaning itself up as it goes.

These helper components are extremely useful when you just want to insert a tree of entities declaratively.

# The Template Macro

Alternatively you can use the `template!()` macro, which is very similar to the proposed bsn syntax.
Arbitrary data can be passed into the macro using normal rust blocks.
The macro returns portable `Template` values, which can be spliced into other templates using `@{ ... }`.

Not only is the macro declarative and composable, it also supports basic incrementalization (doing partial updates to the ecs rather than rebuilding from scratch).
Building the same macro multiple times only does the work necessary to bring the `entity` into alignment with the template.
