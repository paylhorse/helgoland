use godot::prelude::*;
use godot::engine::{ICodeEdit, CodeEdit, InputEvent};
use crate::blackboard::Blackboard;

#[derive(GodotClass)]
#[class(base = CodeEdit)]
pub struct Hagoromo {
    #[base]
    pub node: Base<CodeEdit>,
}

#[godot_api]
impl ICodeEdit for Hagoromo {
    fn init(node: Base<CodeEdit>) -> Self {
        Hagoromo { node }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        let is_return_pressed = Input::singleton().is_action_just_pressed("return".into());
        if is_return_pressed {
            let vbox = self.node.get_parent().unwrap();
            let mut blackboard = vbox.get_node_as::<Blackboard>("PanelContainer/Blackboard");
            blackboard.bind_mut().execute_julia(self.node.get_text().into());
            self.node.set_text("".into());
        }
    }

    fn process(&mut self, delta: f64) {}
}

#[godot_api]
impl Hagoromo {
}
