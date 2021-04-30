use crate::GameState;
use bevy::prelude::*;

pub enum IngredientVariant {
    HotDogBun,
    HotDogLink,
}

pub struct Ingredient(pub IngredientVariant);

pub struct IngredientPlugin;

impl Plugin for IngredientPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup.system()));
    }
}

fn setup(mut commands: Commands) {
    // do nothing so far
}
