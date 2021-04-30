use bevy::prelude::*;

use crate::plugins::{Actor, Destination, Dish, DishType, Employee, Patron, Velocity};
const CLOSE_ENOUGH: f32 = 32.0;

pub struct TasksPlugin;

impl Plugin for TasksPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(OrdersQueue(vec![]))
            .add_system(goto.system())
            .add_system(goto_entity.system())
            .add_system(order_food.system())
            .add_system(assign_tasks.system());
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

pub struct Order {
    dish: DishType,
    patron: Entity,
}

pub struct OrdersQueue(pub Vec<Order>);
pub struct Tasks(pub Vec<Task>);

pub struct Task {
    status: TaskStatus,
    variant: TaskVariants,
}

pub enum TaskVariants {
    GoTo(Vec3),
    GoToEntity(Entity),
    OrderFood(DishType),
}

enum TaskStatus {
    New,
    InProgress,
    Completed,
}

impl Task {
    pub fn new(variant: TaskVariants) -> Task {
        match variant {
            TaskVariants::GoTo(vec3) => {
                return Task {
                    status: TaskStatus::New,
                    variant: TaskVariants::GoTo(vec3),
                }
            }
            TaskVariants::GoToEntity(entity) => {
                return Task {
                    status: TaskStatus::New,
                    variant: TaskVariants::GoToEntity(entity),
                }
            }
            TaskVariants::OrderFood(dish) => {
                return Task {
                    status: TaskStatus::New,
                    variant: TaskVariants::OrderFood(dish),
                }
            }
        }
    }
}

fn goto(
    mut commands: Commands,
    mut query: Query<(Entity, &Actor, &Transform, &mut Velocity, &mut Tasks)>,
) {
    for (entity, _actor, transform, mut velocity, mut tasks) in query.iter_mut() {
        if let Some(task) = tasks.0.first_mut() {
            if let TaskVariants::GoTo(dest) = task.variant {
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
                        tasks.0.remove(0);
                    }
                }
            }
        }
    }
}

fn goto_entity(mut query: Query<&mut Tasks>, entity_query: Query<(Entity, &Transform)>) {
    for mut tasks in query.iter_mut() {
        if let Some(task) = tasks.0.first_mut() {
            if let TaskVariants::GoToEntity(target_entity) = task.variant {
                match task.status {
                    TaskStatus::New => {
                        // Find the entity to go to
                        for (entity, transform) in entity_query.iter() {
                            if entity == target_entity {
                                *task = Task::new(TaskVariants::GoTo(transform.translation));
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
}

fn order_food(mut orders_queue: ResMut<OrdersQueue>, mut query: Query<(Entity, &mut Tasks)>) {
    for (entity, mut tasks) in query.iter_mut() {
        if let Some(task) = tasks.0.first_mut() {
            if let TaskVariants::OrderFood(dish) = task.variant {
                match task.status {
                    TaskStatus::New => {
                        // Ask for the food
                        orders_queue.0.push(Order {
                            dish,
                            patron: entity,
                        });
                        task.status = TaskStatus::InProgress;
                    }
                    TaskStatus::InProgress => {
                        // TODO: Do I have the food?
                    }
                    TaskStatus::Completed => {
                        // TODO: Go away with the food
                        tasks.0.pop().unwrap();
                    }
                }
            }
        }
    }
}

fn assign_tasks(mut orders_queue: ResMut<OrdersQueue>, mut commands: Commands, query: Query<Entity, Without<Tasks>>) {
    if let Some(order) = orders_queue.0.first() {
        for (entity) in query.iter() {

        }
    }
}
