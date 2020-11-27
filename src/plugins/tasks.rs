use bevy::prelude::*;

use crate::plugins::{Destination, Dish, DishType, Employee};

pub struct Task {
    task: Tasks,
    steps: Vec<Step>,
}

impl Task {
    pub fn new(task_type: Tasks) -> Task {
        match task_type {
            Tasks::GoTo(destination) => Task {
                task: task_type,
                steps: vec![Step::new(Steps::GoTo(destination))],
            },
            Tasks::Pickup(dish_type) => Task {
                task: task_type,
                steps: vec![
                    Step::new(Steps::GoToDishType(dish_type)),
                ],
            },
            Tasks::GoToDish(dish_type) => Task {
                task: task_type,
                steps: vec![
                    Step::new(Steps::GoToDishType(dish_type))
                ]
            }
        }
    }
}

pub enum Tasks {
    Pickup(DishType),
    GoTo(Destination),
    GoToDish(DishType)
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
    GoToEntity(Entity),
    GoToDishType(DishType),
    Take(Entity),
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
        app.add_system(goto.system())
            .add_system(goto_dish.system())
            .add_system(take.system());
    }
}

fn take(
    mut commands: Commands,
    mut query: Query<(Entity, &Employee, &Transform, &mut Task)>,
    mut entity_query: Query<(Entity, &Transform)>,
) {
    for (entity, employee, transform, mut task) in query.iter_mut() {
        if let Some(step) = task.steps.first_mut() {
            if let Steps::Take(entity) = step.step {
                match step.status {
                    StepStatus::New => {
                        step.status = StepStatus::InProgress;
                    }
                    StepStatus::InProgress => {
                        // Is actor close enough to take the entity?
                        for (ent, transf) in entity_query.iter() {
                            if ent == entity {
                                let actor_location = transform.translation;
                                let ent_location = transf.translation;
                                let distance = (actor_location - ent_location).length();
                                if distance < 32.0 {
                                    commands.push_children(entity, &[ent]);
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

fn goto(
    mut commands: Commands,
    mut query: Query<(Entity, &Employee, &mut Task)>,
    mut dest_query: Query<Without<Destination, (Entity, &Employee)>>,
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
                        // employee#move_to_desitination removes the destination component
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
            } else {
                // All done, remove the task from the entity
                commands.remove_one::<Task>(entity);
            }
        }
    }
}

fn goto_dish(
    mut commands: Commands,
    mut query: Query<(Entity, &Employee, &mut Task, &Transform)>,
    mut dish_query: Query<(Entity, &Dish, &Transform)>,
) {
    for (entity, _employee, mut task, transform) in query.iter_mut() {
        if let Some(step) = task.steps.first_mut() {
            if let Steps::GoToDishType(dish_type) = step.step {
                match step.status {
                    StepStatus::New => {
                        // Where is the thing to pick up?
                        // Add a destination to the actor
                        for (dish_entity, dish, dish_transform) in dish_query.iter() {
                            if dish_type == dish.0 {
                                let destination = dish_transform.translation;
                                commands.insert_one(entity, Destination(destination));
                                step.status = StepStatus::InProgress;
                            }
                        }
                    }
                    StepStatus::InProgress => {
                        // Is the person close enough to the dish?
                        for (dish_entity, dish, dish_transform) in dish_query.iter() {
                            if dish_type == dish.0 {
                                let destination = dish_transform.translation;
                                let actor_location = transform.translation;
                                let distance = (destination - actor_location).length();
                                if distance < 32.0 {
                                    commands.push_children(entity, &[dish_entity]);
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
