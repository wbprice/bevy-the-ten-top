mod dish;
mod employee;
mod tasks;

pub use self::{
    dish::{Dish, DishPlugin, DishType},
    employee::{Destination, Employee, EmployeePlugin},
    tasks::{Task, Tasks, TasksPlugin},
};
