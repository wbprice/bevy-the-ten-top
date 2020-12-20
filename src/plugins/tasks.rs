use bevy::prelude::*;

use crate::plugins::{Destination, Dish, DishType, Employee};

pub struct Task {
    task: Tasks,
    steps: Vec<Step>,
}

impl Task {
    pub fn new(task_type: Tasks) -> Task {
        match task_type {
            Tasks::FindDish(dish_type) => Task {
                task: task_type,
                steps: vec![
                    Step::new(Steps::PickupDishType(dish_type)),
                    Step::new(Steps::GoTo(Destination(Vec3::new(-100.0, -100.0, 0.0)))),
                ],
            },
        }
    }
}

pub enum Tasks {
    FindDish(DishType),
}

struct Step {
    status: StepStatus,
    step: Steps,
}

impl Step {
    fn new(step_type: Steps) -> Step {
        Step {
            status: StepStatus::New,
            step: step_type,
        }
    }
}

enum Steps {
    GoTo(Destination),
    PickupDishType(DishType),
}

#[derive(Clone, Copy)]
enum StepStatus {
    New,
    InProgress,
    Completed,
}

pub struct TasksPlugin;

impl Plugin for TasksPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(goto.system()).add_system(goto_dish.system());
    }
}

fn goto(
    commands: &mut Commands,
    mut query: Query<(Entity, &Employee, &mut Task)>,
    mut dest_query: Query<(Entity, &Employee), Without<Task>>,
) {
    // Find employees with a GoTo step to take
    for (entity, _employee, mut task) in query.iter_mut() {
        if let Some(step) = task.steps.first_mut() {
            if let Steps::GoTo(destination) = step.step {
                // Do the next step of the task
                match step.status {
                    StepStatus::New => {
                        // add destination to the actor entity
                        commands.insert_one(entity, destination);
                        step.status = StepStatus::InProgress;
                    }
                    StepStatus::InProgress => {
                        // is this actor close enough to the destination?
                        // employee#move_to_destination removes the destination component
                        // So if this entity does not have a destination component, we can consider
                        // this step completed
                        for (ent, _empl) in dest_query.iter_mut() {
                            if ent == entity {
                                step.status = StepStatus::Completed;
                            }
                        }
                    }
                    StepStatus::Completed => {
                        // Remove this step from the list, queueing up the next one.
                        task.steps.remove(0);
                    }
                }
            }
        }
    }
}

fn goto_dish(
    commands: &mut Commands,
    mut query: Query<(Entity, &Employee, &mut Task, &Transform)>,
    mut dish_query: Query<(Entity, &Dish, &mut Transform)>,
) {
    for (entity, _employee, mut task, transform) in query.iter_mut() {
        if let Some(step) = task.steps.first_mut() {
            if let Steps::PickupDishType(dish_type) = step.step {
                match step.status {
                    StepStatus::New => {
                        // Where is the thing to pick up?
                        // Add a destination to the actor
                        for (_dish_ent, dish, dish_transform) in dish_query.iter_mut() {
                            if dish_type == dish.0 {
                                let destination = dish_transform.translation;
                                commands.insert_one(entity, Destination(destination));
                                step.status = StepStatus::InProgress;
                            }
                        }
                    }
                    StepStatus::InProgress => {
                        // Is the person close enough to the dish?
                        for (dish_entity, dish, mut dish_transform) in dish_query.iter_mut() {
                            if dish_type == dish.0 {
                                let distance =
                                    (dish_transform.translation - transform.translation).length();
                                if distance < 32.0 {
                                    commands.push_children(entity, &[dish_entity]);
                                    dish_transform.scale = Vec3::splat(2.0);
                                    dish_transform.translation = Vec3::new(0.0, 0.0, 0.0);
                                    step.status = StepStatus::Completed;
                                }
                            }
                        }
                    }
                    StepStatus::Completed => {
                        task.steps.remove(0);
                    }
                }
            }
        }
    }
}
