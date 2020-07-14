use std::collections::HashMap;

#[derive(Debug)]
pub struct Controller {
    actions: HashMap<i32, ControllerAction>
}

impl Controller {
    pub fn set_actions(mut self, actions: HashMap<i32, ControllerAction>) {
        self.actions = actions;
    }
    pub fn get_actions(self) -> HashMap<i32, ControllerAction> {
        self.actions
    }
}

pub trait Input {
    fn input(&self);
}

#[derive(Debug)]
pub struct ControllerAction {

}