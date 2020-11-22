use bevy::prelude::*;

use crate::plugins::{Destination, Employee};

enum Tasks {
    Pickup(Entity),
}

enum SubTasks {
    GoTo(Destination),
    GoToEntity(Entity),
    SetEntityParent(Entity, Entity),
}

struct Assignment {
    status: TaskStatus,
    assignee: Employee,
    task: Tasks
}

enum TaskStatus {
    New,
    InProgress,
    Completed,
    Blocked
}

pub struct TasksPlugin;

impl Plugin for TasksPlugin {
    fn build(&self, app: &mut AppBuilder) {}
}

fn pickup(mut commands: Commands, mut query: Query<(&Employee, &Task, &Transform, &Destination)>) {

    // First, find an Employee without a task
     
    // First, tell Employee to walk to the destination

    // Second, tell Employee to 

}
