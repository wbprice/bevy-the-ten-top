mod dish;
mod employee;
mod tasks;

pub use self::{
    dish::DishPlugin,
    employee::{
        EmployeePlugin,
        Employee,
        Destination,
    },
    tasks::TasksPlugin
};
