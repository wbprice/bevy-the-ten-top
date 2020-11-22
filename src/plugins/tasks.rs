use bevy::prelude::*;

use crate::plugins::{
    Destination
};

pub struct Task {
    task: Tasks,
    steps: Vec<Step>
}

impl Task {
    pub fn new(task_type: Tasks) -> Task {
        match task_type {
            Tasks::GoTo(actor, destination) => {
                Task {
                    task: task_type,
                    steps: vec![
                        Step::new(Steps::GoTo(actor, destination))
                    ]
                }
            },
            Tasks::Pickup(actor, thing) => {
                Task {
                    task: task_type,
                    steps: vec![
                        Step::new(Steps::GoToEntity(actor, thing)),
                        Step::new(Steps::SetEntityOwner(actor, thing))
                    ]
                }
            },
            _ => {
                unimplemented!();
            }
        }
    }
}

pub enum Tasks {
    Pickup(Entity, Entity),
    GoTo(Entity, Destination)
}

struct Step {
    status: StepStatus,
    step: Steps
}

impl Step {
    fn new(step_type: Steps) -> Step {
        Step {
            status: StepStatus::New,
            step: step_type
        }
    }
}

enum Steps {
    GoTo(Entity, Destination),
    GoToEntity(Entity, Entity),
    SetEntityOwner(Entity, Entity),
}

enum StepStatus {
    New,
    InProgress,
    Blocked
}

pub struct TasksPlugin;

impl Plugin for TasksPlugin {
    fn build(&self, app: &mut AppBuilder) {

    }
}