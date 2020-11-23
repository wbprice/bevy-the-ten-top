mod dish;
mod employee;
mod tasks;

pub use self::{
    dish::DishPlugin,
    employee::{Destination, Employee, EmployeePlugin},
    tasks::{Task, Tasks, TasksPlugin},
};
