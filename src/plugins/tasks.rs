use bevy::prelude::*;

use crate::plugins::{Actor, Destination, Dish, DishType, Employee, Patron};

pub struct TasksPlugin;

impl Plugin for TasksPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(goto.system());
        // .add_system(goto_entity.system())
        // .add_system(give_to.system())
        // .add_system(assign_tasks.system())
        // .add_system(remove_tasks.system())
        // .add_system(request_dish.system())
        // .add_system(wait_for_dish.system())
        // .add_system(find_dish.system())
        // .add_system(set_item_owner.system())
        // .add_system(leave.system());
    }
}

pub enum Tasks {
    GoTo(Vec2)
}

enum TaskStatus {
    New,
    InProgress,
    Completed
}

struct Task {
    status: TaskStatus,
    variant: Tasks,
}

impl Task {
    fn new(variant: Tasks) -> Task {
        match variant {
            Tasks::GoTo(vec2) => {
                return Task {
                    status: TaskStatus::New,
                    variant: Tasks::GoTo(vec2)
                }
            }
        }
    }
}

fn goto(
    mut commands: Commands,
    mut query: Query<(Entity, &Actor, &Transform, &mut Task)>,
) {
    for (entity, actor, transform, task) in query.iter_mut() {
        if let Tasks::GoTo(dest) = task.variant {
            match task.status {
                TaskStatus::New => {
                    commands.entity(entity).insert(Destination(dest));
                    task.status = TaskStatus::InProgress;
                },
                TaskStatus::InProgress => {
                    // Is the actor close enough to the destination?
                    if (transform.translation.as - 
                }
                TaskStatus::Completed => {

                }
            }
        }
    }
}

