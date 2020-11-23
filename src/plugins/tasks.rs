use bevy::prelude::*;

use crate::plugins::{Destination, Employee};

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
            Tasks::Pickup(thing) => Task {
                task: task_type,
                steps: vec![
                    Step::new(Steps::GoToEntity(thing)),
                    Step::new(Steps::Take(thing)),
                ],
            },
            _ => {
                unimplemented!();
            }
        }
    }
}

pub enum Tasks {
    Pickup(Entity),
    GoTo(Destination),
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
    Take(Entity),
}

#[derive(Clone, Copy)]
enum StepStatus {
    New,
    InProgress,
    Blocked,
}

pub struct TasksPlugin;

impl Plugin for TasksPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(goto.system());
    }
}

fn goto(mut commands: Commands, mut query: Query<(Entity, &Employee, &mut Task)>) {
    // Find employees with tasks
    for (entity, employee, mut task) in query.iter_mut() {
        if let Some(step) = task.steps.first_mut() {
            // Do the next step of the task
            match step.status {
                StepStatus::New => {
                    // add destination to the actor entity
                    step.status = StepStatus::InProgress;
                }
                StepStatus::InProgress => {}
                StepStatus::Blocked => {}
            }
        } else {
            // All done, remove the task
        }
    }
}
