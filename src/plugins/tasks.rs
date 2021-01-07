use bevy::prelude::*;

use crate::plugins::{Actor, Destination, Dish, DishType, Employee, Patron, Craving};

pub struct TasksPlugin;

impl Plugin for TasksPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(TasksQueue(vec![]))
            .add_system(goto.system())
            .add_system(goto_dish.system())
            .add_system(goto_entity.system())
            .add_system(give_to.system())
            .add_system(assign_tasks.system())
            .add_system(request_dish.system())
            .add_system(wait_for_dish.system())
            .add_system(leave.system());
    }
}

pub struct Task {
    task: Tasks,
    steps: Vec<Step>,
}

pub struct TasksQueue(pub Vec<Task>);

impl Task {
    pub fn new(task_type: Tasks) -> Task {
        match task_type {
            Tasks::DeliverOrder(dish_type, entity) => Task {
                task: task_type,
                steps: vec![
                    Step::new(Steps::PickupDishType(dish_type)),
                    Step::new(Steps::GoToEntity(entity)),
                    Step::new(Steps::GiveTo(entity)),
                ],
            },
            Tasks::RequestOrder(dish_type, register) => Task {
                task: task_type,
                steps: vec![
                    Step::new(Steps::GoToEntity(register)),
                    Step::new(Steps::RequestDish(dish_type)),
                    Step::new(Steps::WaitForDish(dish_type)),
                    Step::new(Steps::Leave)
                ],
            },
        }
    }
}

pub enum Tasks {
    DeliverOrder(DishType, Entity),
    RequestOrder(DishType, Entity),
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
    PickupDishType(DishType),
    RequestDish(DishType),
    WaitForDish(DishType),
    GiveTo(Entity),
    Leave,
}

#[derive(Clone, Copy)]
enum StepStatus {
    New,
    InProgress,
    Completed,
}

// This takes a task off the stack and assigns it to an idle employee actor.
fn assign_tasks(
    commands: &mut Commands,
    mut tasks: ResMut<TasksQueue>,
    query: Query<(Entity, &Actor, &Employee), Without<Task>>,
) {
    for (entity, _actor, _employee) in query.iter() {
        if let Some(task) = tasks.0.pop() {
            commands.insert_one(entity, task);
        }
    }
}

fn leave(
    commands: &mut Commands,
    mut query: Query<(Entity, &Patron, &mut Task)>
) {
    for (entity, patron, mut task) in query.iter_mut() {
        if let Some(step) = task.steps.first_mut() {
            match step.status {
                StepStatus::New => {
                    step.status = StepStatus::InProgress;
                },
                StepStatus::InProgress => {
                    commands.insert_one(entity, Destination(Vec3::new(256.0, 256.0, 0.0)));
                    step.status = StepStatus::Completed;
                },
                StepStatus::Completed => {
                    task.steps.remove(0);
                }
            }
        }
    }
}

fn request_dish(
    mut tasks: ResMut<TasksQueue>,
    mut query: Query<(Entity, &Patron, &Craving, &mut Task)>,
) {
    for (entity, _patron, craving, mut task) in query.iter_mut() {
        if let Some(step) = task.steps.first_mut() {
            match step.status {
                StepStatus::New => {
                    step.status = StepStatus::InProgress;
                },
                StepStatus::InProgress => {
                    tasks.0.push(Task::new(Tasks::DeliverOrder(craving.0, entity)));
                    step.status = StepStatus::Completed;
                },
                StepStatus::Completed => {
                    task.steps.remove(0);
                }
            }
        }
    }
}

fn wait_for_dish(
    mut query: Query<(&Patron, &Craving, &Children, &mut Task)>,
    dish_query: Query<(Entity, &Dish)>,
) {
    for (patron, craving, children, mut task) in query.iter_mut() {
        if let Some(step) = task.steps.first_mut() {
            match step.status {
                StepStatus::New => {
                    step.status = StepStatus::InProgress;
                },
                StepStatus::InProgress => {
                    for child in children.iter() {
                        for (entity, dish) in dish_query.iter() {
                            if *child == entity {
                                if dish.0 == craving.0 {
                                    step.status = StepStatus::Completed;
                                }
                            }
                        }
                    }
                },
                StepStatus::Completed => {
                    task.steps.remove(0);
                }
            }
        }
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

fn give_to(commands: &mut Commands, mut query: Query<(Entity, &Actor, &Children, &mut Task)>) {
    for (_entity, _actor, children, mut task) in query.iter_mut() {
        if let Some(step) = task.steps.first_mut() {
            if let Steps::GiveTo(owner) = step.step {
                match step.status {
                    StepStatus::New => {
                        step.status = StepStatus::InProgress;
                    }
                    StepStatus::InProgress => {
                        let item = children.first().unwrap();
                        commands.push_children(owner, &[*item]);
                        step.status = StepStatus::Completed;
                    }
                    StepStatus::Completed => {
                        task.steps.remove(0);
                    }
                }
            }
        }
    }
}

fn goto_dish(
    commands: &mut Commands,
    mut query: Query<(Entity, &Actor, &mut Task, &Transform)>,
    mut dish_query: Query<(Entity, &Dish, &mut Transform)>,
) {
    for (entity, _actor, mut task, transform) in query.iter_mut() {
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

fn goto_entity(
    commands: &mut Commands,
    mut query: Query<(Entity, &Actor, &mut Task, &Transform)>,
    destination_query: Query<(Entity, &Transform)>,
) {
    for (entity, _actor, mut task, transform) in query.iter_mut() {
        if let Some(step) = task.steps.first_mut() {
            if let Steps::GoToEntity(destination_entity) = step.step {
                match step.status {
                    StepStatus::New => {
                        // Where is the destination entity?
                        // Add a destination to the actor
                        for (dest_entity, dest_transform) in destination_query.iter() {
                            if dest_entity == destination_entity {
                                let destination = dest_transform.translation;
                                commands.insert_one(entity, Destination(destination));
                                step.status = StepStatus::InProgress;
                            }
                        }
                    }
                    StepStatus::InProgress => {
                        // Is the person close enough to the destination?
                        for (_entity, dest_transform) in destination_query.iter() {
                            let distance =
                                (dest_transform.translation - transform.translation).length();
                            dbg!(distance);
                            if distance < 32.0 {
                                step.status = StepStatus::Completed;
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
