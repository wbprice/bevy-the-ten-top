use bevy::prelude::*;

struct Position {
    x: f32,
    y: f32,
}

struct Person;
struct Name(String);

fn add_people(mut commands: Commands) {
    commands
        .spawn((Person, Name("Elaina Proctor".to_string())))
        .spawn((Person, Name("Renzo Hume".to_string())))
        .spawn((Person, Name("Zayna Nieves".to_string())));
}

struct GreetTimer(Timer);
fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, mut query: Query<(&Person, &Name)>) {
    // update our timer with the time elapsed since the last update
    timer.0.tick(time.delta_seconds);

    // check to see if the timer has finished. if it has, print our message
    if timer.0.finished {
        for (_person, name) in &mut query.iter() {
            println!("hello {}!", name.0);
        }
    }
}

fn print_position_system(position: &Position) {
    println!("position: {} {}", position.x, position.y);
}

pub struct HelloPlugin;
impl Plugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(add_people.system())
            .add_system(greet_people.system());
    }
}

fn main() {
    App::build()
        .add_default_plugins()
        .add_plugin(HelloPlugin)
        .run();
}
