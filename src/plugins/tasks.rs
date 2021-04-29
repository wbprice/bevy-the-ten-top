use bevy::prelude::*;

use crate::plugins::{Actor, Destination, Dish, DishType, Employee, Patron, Velocity};
const CLOSE_ENOUGH: f32 = 32.0;

pub struct TasksPlugin;

impl Plugin for TasksPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(goto.system())
            .add_system(goto_entity.system());
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
    GoTo(Vec3),
    GoToEntity(Entity),
}

enum TaskStatus {
    New,
    InProgress,
    Completed,
}

pub struct Task {
    status: TaskStatus,
    variant: Tasks,
}

impl Task {
    pub fn new(variant: Tasks) -> Task {
        match variant {
            Tasks::GoTo(vec3) => {
                return Task {
                    status: TaskStatus::New,
                    variant: Tasks::GoTo(vec3),
                }
            }
            Tasks::GoToEntity(entity) => {
                return Task {
                    status: TaskStatus::New,
                    variant: Tasks::GoToEntity(entity),
                }
            }
        }
    }
}

fn goto(
    mut commands: Commands,
    mut query: Query<(Entity, &Actor, &Transform, &mut Velocity, &mut Task)>,
) {
    for (entity, _actor, transform, mut velocity, mut task) in query.iter_mut() {
        if let Tasks::GoTo(dest) = task.variant {
            match task.status {
                TaskStatus::New => {
                    task.status = TaskStatus::InProgress;
                }
                TaskStatus::InProgress => {
                    // Is the actor close enough to the destination?
                    let translation = transform.translation;
                    let difference = translation - dest;
                    let distance = difference.length();
                    if distance < CLOSE_ENOUGH {
                        commands.entity(entity).insert(Velocity(0.0, 0.0));
                        task.status = TaskStatus::Completed;
                    } else {
                        let heading = (difference.y.atan2(difference.x)) * 180.0 / 3.14;
                        velocity.0 = 50.0 * heading.cos();
                        velocity.1 = 50.0 * heading.sin();
                    }
                }
                TaskStatus::Completed => {
                    commands.entity(entity).remove::<Task>();
                }
            }
        }
    }
}

fn goto_entity(
    mut commands: Commands,
    query: Query<(Entity, &Task)>,
    entity_query: Query<(Entity, &Transform)>,
) {
    for (entity, task) in query.iter() {
        if let Tasks::GoToEntity(target_entity) = task.variant {
            match task.status {
                TaskStatus::New => {
                    // Find the entity to go to
                    for (ent, transform) in entity_query.iter() {
                        if ent == target_entity {
                            commands
                                .entity(entity)
                                .insert(Task::new(Tasks::GoTo(transform.translation)));
                        }
                    }
                }
                _ => {
                    unreachable!();
                }
            }
        }
    }
}
