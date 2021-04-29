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

pub enum TaskType {
    GoTo(Vec2)
}

enum TaskStatus {
    New,
    InProgress,
    Completed
}

struct Task {
    Task
}

enum Tasks {
    Goto(TaskStatus)
}

fn goto(
    mut query: Query<&Actor>
) {

}

