use bevy::prelude::*;

use crate::plugins::{Actor, Destination, Dish, DishType, Employee, Patron};

pub struct TasksPlugin;

impl Plugin for TasksPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(TasksQueue(vec![]))
            .add_system(goto_entity.system())
            .add_system(give_to.system())
            .add_system(assign_tasks.system())
            .add_system(remove_tasks.system())
            .add_system(request_dish.system())
            .add_system(wait_for_dish.system())
            .add_system(find_dish.system())
            .add_system(set_item_owner.system())
            .add_system(leave.system());
    }
}

#[derive(Debug)]
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
                    Step::new(Steps::FindDish(dish_type)),
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
                    Step::new(Steps::Leave),
                ],
            },
        }
    }
}

#[derive(Debug)]
pub enum Tasks {
    DeliverOrder(DishType, Entity),
    RequestOrder(DishType, Entity),
}
#[derive(Debug)]
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

#[derive(Debug)]
enum Steps {
    GoToEntity(Entity),
    FindDish(DishType),
    RequestDish(DishType),
    WaitForDish(DishType),
    GiveTo(Entity),
    SetItemOwner(Entity, Entity),
    Leave,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum StepStatus {
    New,
    InProgress,
    Completed,
}

// This takes a task off the stack and assigns it to an idle employee actor.
fn assign_tasks(
    mut commands: Commands,
    mut tasks: ResMut<TasksQueue>,
    query: Query<(Entity, &Actor, &Employee), Without<Task>>,
) {
    for (entity, _actor, _employee) in query.iter() {
        if let Some(task) = tasks.0.pop() {
            commands.entity(entity).insert(task);
        }
    }
}

// This removes completed tasks from the actors who have them.
fn remove_tasks(mut commands: Commands, query: Query<(Entity, &Actor, &Task)>) {
    for (entity, _actor, task) in query.iter() {
        // If all subtasks are completed, remove the task from the entity.ms
        if task.steps.len() == 0 {
            commands.entity(entity).remove::<Task>();
        }
    }
}

fn leave(mut commands: Commands, mut query: Query<(Entity, &Patron, &mut Task)>) {
    for (entity, _patron, mut task) in query.iter_mut() {
        if let Some(step) = task.steps.first_mut() {
            if let Steps::Leave = step.step {
                match step.status {
                    StepStatus::New => {
                        step.status = StepStatus::InProgress;
                    }
                    StepStatus::InProgress => {
                        commands
                            .entity(entity)
                            .insert(Destination(Vec3::new(0.0, -256.0, 0.0)));
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

fn request_dish(mut tasks: ResMut<TasksQueue>, mut query: Query<(Entity, &Patron, &mut Task)>) {
    for (entity, _patron, mut task) in query.iter_mut() {
        if let Some(step) = task.steps.first_mut() {
            if let Steps::RequestDish(dish_type) = step.step {
                match step.status {
                    StepStatus::New => {
                        step.status = StepStatus::InProgress;
                    }
                    StepStatus::InProgress => {
                        tasks
                            .0
                            .push(Task::new(Tasks::DeliverOrder(dish_type, entity)));
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

fn wait_for_dish(
    mut query: Query<(&Patron, &Children, &mut Task)>,
    dish_query: Query<(Entity, &Dish)>,
) {
    for (_patron, children, mut task) in query.iter_mut() {
        if let Some(step) = task.steps.first_mut() {
            if let Steps::WaitForDish(dish_type) = step.step {
                match step.status {
                    StepStatus::New => {
                        step.status = StepStatus::InProgress;
                    }
                    StepStatus::InProgress => {
                        for child in children.iter() {
                            for (entity, dish) in dish_query.iter() {
                                if *child == entity {
                                    if dish.0 == dish_type {
                                        step.status = StepStatus::Completed;
                                    }
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

fn give_to(mut commands: Commands, mut query: Query<(Entity, &Actor, &Children, &mut Task)>) {
    for (_entity, _actor, children, mut task) in query.iter_mut() {
        if let Some(step) = task.steps.first_mut() {
            if let Steps::GiveTo(owner) = step.step {
                match step.status {
                    StepStatus::New => {
                        step.status = StepStatus::InProgress;
                    }
                    StepStatus::InProgress => {
                        let item = children.first().unwrap();
                        let mut transform = Transform::from_translation(Vec3::new(6.0, -6.0, 1.0));
                        transform.scale = Vec3::splat(1.0);
                        commands.entity(*item).insert(transform);
                        commands.entity(owner).push_children(&[*item]);
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

fn set_item_owner(mut commands: Commands, mut query: Query<(Entity, &Actor, &mut Task)>) {
    for (_entity, _actor, mut task) in query.iter_mut() {
        if let Some(step) = task.steps.first_mut() {
            if let Steps::SetItemOwner(item, owner) = step.step {
                match step.status {
                    StepStatus::New => {
                        step.status = StepStatus::InProgress;
                    }
                    StepStatus::InProgress => {
                        commands.entity(owner).push_children(&[item]);
                        let mut transform = Transform::from_translation(Vec3::new(6.0, -6.0, 1.0));
                        transform.scale = Vec3::splat(1.0);
                        commands.entity(item).insert(transform);
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

fn find_dish(
    mut query: Query<(Entity, &Actor, &mut Task)>,
    dish_query: Query<(Entity, &Dish), Without<Actor>>,
) {
    for (entity, _actor, mut task) in query.iter_mut() {
        let mut steps_to_insert: Vec<Step> = vec![];
        if let Some(step) = task.steps.first_mut() {
            if let Steps::FindDish(dish_type) = step.step {
                match step.status {
                    StepStatus::New => {
                        step.status = StepStatus::InProgress;
                    }
                    StepStatus::InProgress => {
                        for (dish_entity, dish) in dish_query.iter() {
                            if dish.0 == dish_type {
                                steps_to_insert
                                    .insert(0, Step::new(Steps::GoToEntity(dish_entity)));
                                steps_to_insert
                                    .insert(0, Step::new(Steps::SetItemOwner(dish_entity, entity)));
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
        for step in steps_to_insert {
            task.steps.insert(0, step);
        }
    }
}

fn goto_entity(
    mut commands: Commands,
    mut query: Query<(Entity, &Actor, &mut Task)>,
    destination_query: Query<(Entity, &Transform), Without<Destination>>,
    actor_query: Query<(Entity, &Actor, &Destination, &Transform)>,
) {
    for (entity, _actor, mut task) in query.iter_mut() {
        if let Some(step) = task.steps.first_mut() {
            if let Steps::GoToEntity(goto_entity) = step.step {
                match step.status {
                    StepStatus::New => {
                        // Where is the destination entity?
                        // Add a destination to the actor
                        for (dest_entity, dest_transform) in destination_query.iter() {
                            if goto_entity == dest_entity {
                                commands.entity(entity).insert(dest_transform.translation);
                                step.status = StepStatus::InProgress;
                            }
                        }
                    }
                    StepStatus::InProgress => {
                        // Is the person close enough to the destination?
                        for (this_entity, _actor, destination, transform) in actor_query.iter() {
                            if this_entity == entity {
                                if destination
                                    .0
                                    .truncate()
                                    .distance(transform.translation.truncate())
                                    < 48.0
                                {
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
