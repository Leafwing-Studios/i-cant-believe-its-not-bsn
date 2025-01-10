//! Super Sheep-Counter 2000
//!
//! An all-in-one numerical ruminant package.

use i_cant_believe_its_not_bsn::*;

use bevy::color::palettes::css;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(sheep_plugin)
        .run();
}

fn sheep_plugin(app: &mut App) {
    app.add_systems(Startup, setup)
        .add_systems(Update, sheep_system)
        .add_observer(observe_buttons);
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn(UiRoot);
}

#[derive(Component)]
struct UiRoot;

#[derive(Component)]
struct Sheep;

#[derive(Component)]
enum Button {
    Increment,
    Decrement,
}

// A query that pulls data from the ecs and then updates it using a template.
fn sheep_system(mut commands: Commands, sheep: Query<&Sheep>, root: Single<Entity, With<UiRoot>>) {
    let num_sheep = sheep.iter().len();

    let template = template! {
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        } => [
            @{ counter(num_sheep, "sheep", Button::Increment, Button::Decrement) };
        ];
    };

    commands.entity(*root).build(template);
}

// A function that returns an ecs template.
fn counter<T: Component>(num: usize, name: &str, inc: T, dec: T) -> Template {
    template! {
        Text::new("You have ") => [
            TextSpan::new(format!("{num}"));
            TextSpan::new(format!(" {name}!"));
        ];
        ( Button, Text::new("Increase"), TextColor(css::GREEN.into()), inc, visible_if(num < 100) );
        ( Button, Text::new("Decrease"), TextColor(css::RED.into()), dec, visible_if(num > 0) );
    }
}

// A component helper function for computing visibility.
fn visible_if(condition: bool) -> Visibility {
    if condition {
        Visibility::Visible
    } else {
        Visibility::Hidden
    }
}

// A global observer which responds to button clicks.
fn observe_buttons(
    mut trigger: Trigger<Pointer<Up>>,
    buttons: Query<&Button>,
    sheep: Query<Entity, With<Sheep>>,
    mut commands: Commands,
) {
    match buttons.get(trigger.target).ok() {
        Some(Button::Increment) => {
            commands.spawn(Sheep);
        }
        Some(Button::Decrement) => {
            if let Some(sheep) = sheep.iter().next() {
                commands.entity(sheep).despawn_recursive();
            }
        }
        _ => {}
    }
    trigger.propagate(false);
}
