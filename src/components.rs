


use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct KeyboardControlled;