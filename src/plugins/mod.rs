mod dish;
mod employee;
mod tasks;

pub use self::{
    dish::{DishPlugin, Dish, DishType},
    employee::{Employee, Destination, EmployeePlugin},
    tasks::{Task, Tasks, TasksPlugin},
};
